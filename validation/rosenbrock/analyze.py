#!/usr/bin/env python3
"""Compare fresh source-pinned measurements with retained numerical references."""
import csv
import importlib.util
import json
import math
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parent
HISTORICAL = ROOT / 'historical/extended-rc2-2026-10-05/accuracy-2026-10-04'


def dictionary_rows(path):
    with path.open() as handle:
        return list(csv.DictReader(handle))


def raw_rows(path):
    with path.open() as handle:
        return list(csv.reader(handle))


def native_key(row):
    return tuple(row[k] for k in ['method', 'problem', 'mode']) + tuple(
        float(row[k]) for k in ['tolerance', 'h', 't'])


def core(run):
    native = {native_key(r): r for r in dictionary_rows(run / 'native.csv')}
    julia = {native_key(r): r for r in dictionary_rows(HISTORICAL / 'julia-trajectories.csv')}
    comparisons = []
    for key, j in julia.items():
        r = native[key]
        rv, jv = [list(map(float, row['y'].split(';'))) for row in [r, j]]
        assert len(rv) == len(jv)
        comparisons.append(dict(method=r['method'], problem=r['problem'], mode=r['mode'],
                                rtol=float(r['tolerance']),
                                maximum_absolute_difference=max(abs(a-b) for a,b in zip(rv,jv)),
                                rust_steps=int(r['steps']), julia_steps=int(j['steps']),
                                rust_rejections=int(r['rejects']), julia_rejections=int(j['rejects'])))
    fixed = [r for r in comparisons if r['mode'] == 'fixed']
    adaptive = [r for r in comparisons if r['mode'] == 'adaptive']
    assert len(fixed) == 10 and len(adaptive) == 20
    paper = []
    for row in raw_rows(run / 'paper.csv'):
        if row[0] == 'paper' and row[1] in ['B4', 'B5', 'B7']:
            paper.append(dict(table={'B4':5,'B5':6,'B7':8}[row[1]],
                              h=float(row[2]), printed_error=float(row[3]),
                              computed_error=float(row[4])))
    parabolic = []
    for row in raw_rows(run / 'parabolic.csv'):
        item=dict(mesh=int(row[0]), lifted=row[1]=='true', h=float(row[2]),
                  printed_error=float(row[3]), computed_error=float(row[4]))
        parabolic.append(item)
        if item['mesh'] == 1000 and not item['lifted']:
            paper.append(dict(table=7, **{k:item[k] for k in ['h','printed_error','computed_error']}))
    assert len(paper) == 14
    for row in paper:
        row['printed_error_relative_difference_percent'] = 100*abs(row['computed_error']/row['printed_error']-1)
    stability = {float(r['z']):float(r['R']) for r in dictionary_rows(HISTORICAL/'julia-stability.csv')}
    differences=[]; polynomial=[]
    for row in raw_rows(run/'properties.csv'):
        if row[0]=='stability':
            differences.append(abs(float(row[2])-stability[float(row[1])]))
        elif row[0]=='polynomial':
            polynomial.append(dict(degree=int(row[1]), maximum_dense_error=float(row[2]),
                                   endpoint_error=float(row[3]), maximum_derivative_error=float(row[4])))
    assert len(differences)==5 and len(polynomial)==5
    result=dict(source=json.loads((run/'source-pin.json').read_text()),
                julia_measurement_status='retained pinned Julia references; fresh Rust runs',
                fixed_cases=fixed, adaptive_cases=adaptive, paper_rows=paper,
                parabolic_rows=parabolic, polynomial_diagnostics=polynomial,
                fixed_cases_count=len(fixed), matched_adaptive_cases_count=len(adaptive),
                max_fixed_absolute_difference=max(r['maximum_absolute_difference']for r in fixed),
                max_stability_absolute_difference=max(differences),
                max_printed_error_difference_percent=max(r['printed_error_relative_difference_percent']for r in paper),
                caveats=['Adaptive pairwise differences do not establish which solution is more accurate.',
                         'Requested tolerance controls local error, not a guaranteed global solution error.',
                         'The index-2 Table 8 example is an out-of-scope diagnostic.',
                         'Printed-error differences are differences between rounded error magnitudes, not solution percentage errors.',
                         'Original parabolic order reduction and fourth-order dense extension are retained.',
                         'Eleven references: ten papers and one textbook; no full-paper reproduction claim.'])
    if (run/'additional-standard-run.json').exists():
        # Keep these analyzers and independent references unchanged in the archive.
        subprocess.run([sys.executable,str(HISTORICAL/'additional/analyze.py'),str(run/'additional-standard')],check=True,stdout=subprocess.DEVNULL)
        standard=json.loads((run/'additional-standard-agreement.json').read_text())
        completed=[r for r in standard['runs']if r['mode']=='sampled']
        assert all(r['maximum_reference_tolerance_units'] < 1 and r['convergence_units'] <= .25
                   for r in completed), 'a completed run exceeds the reported target'
        result['expanded_standard']=dict(completed=len(completed),total=32,accepted_steps_per_pass=5000000,
                                        maximum_passes=8,runs=completed,failures=standard['failures'])
        if (run/'source-gaps-run.json').exists():
            sys.path.insert(0,str(HISTORICAL/'source-gaps'))
            spec=importlib.util.spec_from_file_location('water_analysis',HISTORICAL/'source-gaps/analyze.py')
            mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
            cases=mod.evaluate(mod.load([run/'source-gaps.csv']),json.loads((HISTORICAL/'source-gaps/water-reference.json').read_text()),json.loads((HISTORICAL/'source-gaps/water-bdf-reference.json').read_text()))
            failures=[l for l in (run/'source-gaps.log').read_text().splitlines()if l.startswith(('FAILED','UNRESOLVED'))]
            result['source_supplement']=dict(completed=len(cases),total=8,runs=cases,failures=failures,
                caveat='Exact author water-tube index-1 adaptation is unavailable; independent reduction only.')
        if (run/'circle-observation-endpoints-run.json').exists():
            subprocess.run([sys.executable,str(HISTORICAL/'additional/analyze.py'),str(run/'circle-observation-endpoints')],check=True,stdout=subprocess.DEVNULL)
            result['optional_observation_endpoints']=json.loads((run/'circle-observation-endpoints-agreement.json').read_text())
        refined=dictionary_rows(run/'adaptive-refinement.csv')
        result['refinement_experiment']=dict(runs=len(refined),maximum_final_reference_units=max(float(r['final_reference_units'])for r in refined),
                                             all_below_one=all(float(r['final_reference_units'])<1 for r in refined))
    (run/'agreement.json').write_text(json.dumps(result,indent=2,allow_nan=False)+'\n')
    return result


def main():
    results={name:core(ROOT/'runs'/name)for name in ['rodas-pr-8affbd5','dev-e553754']}
    pr,dev=results.values()
    comparison=dict(schema='unibio.diffsol-branch-comparison/v1',date='2026-10-05',results=results,
                    equal_core_inputs=True,expanded_suite_equal_branch_work_comparison=False,
                    historical_expanded_results_are_fresh_dev_measurements=False,
                    publication_status='local only; Fable review required before publication')
    (ROOT/'comparison.json').write_text(json.dumps(comparison,indent=2,allow_nan=False)+'\n')
    print(json.dumps({name:{k:r[k]for k in ['max_fixed_absolute_difference','max_stability_absolute_difference','max_printed_error_difference_percent']}for name,r in results.items()},indent=2))


if __name__=='__main__':
    main()
