#!/usr/bin/env python3
"""Verify archive completeness, measured outputs and source-patch identities."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    manifest = json.loads((ROOT / 'manifest.json').read_text())
    expected = {r['path'] for r in manifest['files']}
    observed = {p.relative_to(ROOT).as_posix() for p in ROOT.rglob('*') if p.is_file()
                and '__pycache__' not in p.parts and p.suffix != '.pyc'
                and p.name not in ['manifest.json', 'SHA256SUMS']}
    # Nested historical manifests are evidence too.
    observed.update(p.relative_to(ROOT).as_posix() for p in (ROOT/'historical').rglob('manifest.json'))
    assert observed == expected, (observed - expected, expected - observed)
    for row in manifest['files']:
        p = ROOT / row['path']
        assert not p.is_symlink(), p
        assert p.stat().st_size == row['bytes'], p
        assert digest(p) == row['sha256'], p
    for name in ['extended-rc2-2026-10-05', 'minimal-package-2026-10-05']:
        d = ROOT/'historical'/name
        historical = json.loads((d/'manifest.json').read_text())
        for row in historical['files']:
            assert digest(d/row['path']) == row['sha256'], row['path']
    for run in (ROOT/'runs').iterdir():
        pin = json.loads((run/'source-pin.json').read_text())
        assert digest(run/'source.patch') == pin['source_patch_sha256'], run
        agreement = json.loads((run/'agreement.json').read_text())
        assert agreement['source']['source_commit'] == pin['source_commit'], run
        for p in run.glob('*-run.json'):
            receipt = json.loads(p.read_text())
            assert receipt['source_commit'] == pin['source_commit'], p
            assert receipt['source_tree'] == pin['source_tree'], p
            assert receipt['exit_code'] == 0, p
            stem = p.name.removesuffix('-run.json')
            assert digest(run/(stem+'.csv')) == receipt['csv_sha256'], p
            assert digest(run/(stem+'.log')) == receipt['diagnostic_sha256'], p
    print(f'PASS: {len(expected)} evidence files, both source pins, all measured outputs and historical manifests')


if __name__ == '__main__':
    main()
