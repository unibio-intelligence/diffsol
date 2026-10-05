#!/usr/bin/env python3
"""Run pinned external evidence without adding tests to DiffSol's unit suite."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent
HARNESS = ROOT / 'historical/extended-rc2-2026-10-05/accuracy-2026-10-04'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', required=True, type=Path)
    parser.add_argument('--snapshot', required=True, choices=['rodas-pr-8affbd5', 'dev-e553754'])
    parser.add_argument('--target-dir', required=True, type=Path)
    parser.add_argument('--output-dir', type=Path, help='use a fresh directory for an independent rerun')
    parser.add_argument('--extended', action='store_true')
    args = parser.parse_args()
    source = args.source.resolve()
    pin = json.loads((ROOT / 'runs' / args.snapshot / 'source-pin.json').read_text())
    output = args.output_dir.resolve() if args.output_dir else ROOT / 'runs' / args.snapshot
    output.mkdir(parents=True, exist_ok=True)
    # A later evidence-only commit may contain the same numerical source.
    subprocess.run(['git', '-C', str(source), 'diff', '--exit-code',
                    pin['source_commit'], '--', '.', ':(exclude)validation'], check=True,
                       stdout=subprocess.DEVNULL)
    untracked = subprocess.check_output(['git', '-C', str(source), 'ls-files',
                                        '--others', '--exclude-standard'], text=True).splitlines()
    if any(not path.startswith('validation/') for path in untracked):
        raise RuntimeError('source has unrecorded files outside validation/')
    jobs = [(kind, kind, {}) for kind in ['native', 'paper', 'parabolic', 'properties']]
    if args.extended:
        if args.snapshot != 'dev-e553754':
            parser.error('the PR source does not expose the extended evidence APIs')
        jobs += [('dense-accuracy', 'dense-accuracy', {}),
                 ('adaptive-refinement', 'adaptive-refinement', {}),
                 ('additional-standard', 'additional', {}),
                 ('source-gaps', 'additional', {'VALIDATION_SOURCE_GAPS': '1'}),
                 ('circle-observation-endpoints', 'additional', {
                     'VALIDATION_PROBLEM': 'circle_inexact', 'VALIDATION_METHOD': 'rodas5p',
                     'VALIDATION_OBSERVATION_ENDPOINTS': '1'})]
    for name, kind, extra in jobs:
        csv_path = output / (name + '.csv')
        log_path = output / (name + '.log')
        receipt_path = output / (name + '-run.json')
        if receipt_path.exists():
            raise RuntimeError('refusing to overwrite measured evidence: ' + name)
        env = {k: v for k, v in os.environ.items() if not k.startswith('VALIDATION_')}
        env.update(extra)
        cmd = [sys.executable, str(HARNESS / 'run-rust.py'), '--diffsol', str(source),
               '--kind', kind, '--target-dir', str(args.target_dir.resolve())]
        start = time.monotonic()
        with csv_path.open('wb') as stdout, log_path.open('wb') as stderr:
            result = subprocess.run(cmd, env=env, stdout=stdout, stderr=stderr)
        # Retain diagnostic text while removing machine-specific source/cache locations.
        log = log_path.read_text()
        for path, label in [(str(source), '<diffsol-source>'), (str(ROOT), '<validation>'),
                            (str(args.target_dir.resolve()), '<cargo-target>')]:
            log = log.replace(path, label)
        import re
        log = re.sub(r'/private/tmp/diffsol-reference-[^/\s]+', '<driver-build>', log)
        log = re.sub(r'/var/folders/[^\s)]+?/diffsol-reference-[^/\s]+', '<driver-build>', log)
        log_path.write_text(log)
        receipt = dict(source_commit=pin['source_commit'], source_tree=pin['source_tree'],
                       kind=kind, environment=extra, exit_code=result.returncode,
                       elapsed_seconds=time.monotonic() - start,
                       driver_sha256=hashlib.sha256((HARNESS / (kind + '.rs')).read_bytes()).hexdigest(),
                       command=f'python3 validation/rosenbrock/run.py --source SOURCE --snapshot {args.snapshot} --target-dir TARGET' + (' --extended' if args.extended else ''),
                       csv_sha256=hashlib.sha256(csv_path.read_bytes()).hexdigest(),
                       diagnostic_sha256=hashlib.sha256(log_path.read_bytes()).hexdigest())
        receipt_path.write_text(json.dumps(receipt, indent=2) + '\n')
        print(f'{args.snapshot} {name}: exit={result.returncode}, {receipt["elapsed_seconds"]:.1f}s', flush=True)
        if result.returncode:
            raise RuntimeError('driver failed; inspect retained log: ' + name)


if __name__ == '__main__':
    main()
