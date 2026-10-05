"""Recompute the current standard and extended-work status without hiding failures."""
import json, subprocess, sys, csv
from pathlib import Path
ROOT=Path(__file__).resolve().parent
source_patch='87e83dd29470af54ce0c3785e3c2a0562ca225913f9802bd25efc3d7c6ac62cd'
def analyze(name):
    subprocess.run([sys.executable,str(ROOT.parent/'additional/analyze.py'),'../unresolved-followup/'+name],check=True)
    return json.loads((ROOT/(name+'-agreement.json')).read_text())
names=['current-standard','oregonator-work','amplifier-work','circle-rodas-work','circle-rodas-extended','pv-dae-1e-6','pv-dae-1e-8','pendulum-dae-1e-6','pendulum-dae-1e-8','pendulum-tight-extended','circle_inexact-dae-1e-6','circle_inexact-dae-1e-8','pv-vendor','julia-pv-tight','julia-oregonator-tight','julia-pendulum-tight']
reports={name:analyze(name) for name in names}
standard=[r for r in reports['current-standard']['runs'] if r['mode']=='sampled']
key=lambda r:(r['problem'],r['method'],r['rtol'])
qualified={key(r):dict(r,artifact='current-standard.csv',protocol='5000000 accepted steps per pass; 8 passes') for r in standard}
for name in ['oregonator-work','pv-dae-1e-8','pendulum-dae-1e-6']:
    for r in reports[name]['runs']:
        if r['mode']=='sampled':qualified[key(r)]=dict(r,artifact=name+'.csv',protocol='20000000 accepted steps per pass; 8 passes')
historical=json.loads((ROOT.parent/'additional/comparison.json').read_text())
old_failures=historical['diffsol_failures']
resolved=[];remaining=[]
for failure in old_failures:
    words=failure.split();case=(words[1],words[2],float(words[3].rstrip(':')))
    if case in qualified:resolved.append(qualified[case])
    else:remaining.append(dict(problem=case[0],method=case[1],rtol=case[2]))
current_failures=reports['current-standard']['failures']
pv_rows=sum(1 for _ in csv.DictReader((ROOT/'pv-vendor.csv').open()))
assert (ROOT/'pv-vendor.csv').read_bytes()==(ROOT/'pv-dae-1e-6.csv').read_bytes()
assert len(standard)==24 and len(resolved)==4 and len(remaining)==5
reference_residuals=[r for r in reports['pendulum-tight-extended']['runs'] if r['mode'].startswith('pass')]
result={'schema':'unibio.unresolved-numerical-followup/v1','date':'2026-10-05','source_patch_sha256':source_patch,'source_status':'shared uncommitted candidate; Fable review pending','standard_budget_completed':len(standard),'standard_budget_requested':32,'standard_failures':current_failures,'larger_budget_completed_total':len(qualified),'resolved_of_original_nine':resolved,'remaining_of_original_nine':remaining,'completed_checks':list(qualified.values()),'extended_completed_with_source_gap_supplement':len(qualified)+6,'extended_requested_with_source_gap_supplement':40,'water_additional_unresolved':2,'water_current_failures':[l for l in (ROOT/'water-dae.log').read_text().splitlines() if l.startswith('FAILED')],'new_shared_PV_rows_bit_identical':pv_rows,'Julia_followup':{n:reports[n] for n in names if n.startswith('julia-')},'pendulum_nonmonotonic_refinement':reference_residuals,'limits':['Larger-budget completion does not qualify the original budget or increase product limits.','No guarantee that a local tolerance bounds global error.','Hyperbolic formulation corroborated by author 2025 preprint; no complete timing-curve reproduction.','Original water system independently reduced; exact author index-1 adaptation unverified.','General non-diagonal Rosenbrock23 mass-matrix DAE high-accuracy qualification remains limited.','Inexact-Jacobian stress tests retain the published Jacobian; restoration to an exact Jacobian would change the test.','Tighter pendulum diagnostic interrupted during fifth pass after measured loss of accuracy; not counted complete or as a solver return-code failure.']}
(ROOT/'agreement.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'standard_completed':len(standard),'larger_budget_completed':len(qualified),'original_resolved':len(resolved),'original_remaining':len(remaining),'extended_completed':len(qualified)+6,'PV_shared_rows':pv_rows}))
