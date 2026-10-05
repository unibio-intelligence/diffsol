"""Recompute final failures, independent errors and optional observation checks."""
from pathlib import Path
import csv,hashlib,importlib.util,json,subprocess,sys
ROOT=Path(__file__).resolve().parent
PARENT=ROOT.parent
names=['final-standard-current','final-amplifier','final-pendulum-tight','final-pv-extended','final-pendulum-extended','final-oregonator-extended','circle-all-consistent']
for name in names:
 subprocess.run([sys.executable,str(PARENT/'additional/analyze.py'),str(ROOT/name)],check=True,stdout=subprocess.DEVNULL)
read=lambda name:json.loads((ROOT/name).read_text())
standard=read('final-standard-current-agreement.json');runs=[r for r in standard['runs']if r['mode']=='sampled'];extended=[]
for name in names[1:-1]:extended.extend(r for r in read(name+'-agreement.json')['runs']if r['mode']=='sampled')
key=lambda r:(r['problem'],r['method'],r['rtol'])
assert len(runs)==24 and len({key(r)for r in runs+extended})==29
assert all(r['maximum_reference_tolerance_units']<1 and r['convergence_units']<=.25 for r in runs+extended)
sys.path.insert(0,str(PARENT/'source-gaps'))
spec=importlib.util.spec_from_file_location('water_analysis',PARENT/'source-gaps/analyze.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
water=module.evaluate(module.load([ROOT/'final-source-gaps.csv']),json.loads((PARENT/'source-gaps/water-reference.json').read_text()),json.loads((PARENT/'source-gaps/water-bdf-reference.json').read_text()))
assert len(water)==6
optional=[r for r in read('circle-all-consistent-agreement.json')['runs']if r['mode']=='consistent-sampled'];assert len(optional)==2
pendulum=read('pendulum-analytic.json');values=pendulum['values'];analytic=[]
for mode in ['pass0.0010000000000000002','sampled']:
 rows=[r for r in csv.DictReader((ROOT/'final-pendulum-tight.csv').open())if r['mode']==mode]
 if not rows:continue
 errors=[]
 for row in rows:
  index=min(range(len(values)),key=lambda i:abs(pendulum['times'][i]-float(row['time'])));assert abs(pendulum['times'][index]-float(row['time']))<1e-10
  y=list(map(float,row['values'].split(';')));truth=values[index];errors.extend(abs(a-b)/(1e-10+1e-8*abs(b))for a,b in zip(y,truth))
 analytic.append({'mode':mode,'maximum_analytic_error_units':max(errors)})
failures=[l.strip()for l in (ROOT/'final-source-gaps.log').read_text().splitlines()if l.startswith('FAILED')]
result={'schema':'unibio.completed-unresolved-investigation/v1','date':'2026-10-05','source_patch_sha256':'6428eb759a9fd56abcef7a5d2f8d68df1121e133146bbe60a8fde6afc741ed50','status':'uncommitted; Fable review/publication pending','standard_budget':{'accepted_steps_per_pass':5000000,'maximum_passes':8,'completed':24,'total':32,'runs':runs,'failures':standard['failures']},'larger_budget_runs':extended,'mixed_explicit_budgets_raw_coverage':{'original':29,'original_total':32,'including_source_supplement':35,'total':40,'equal_work_comparison':False},'source_supplement':water,'source_supplement_failures':failures,'opt_in_consistent_observations':optional,'opt_in_consistent_observation_failures':read('circle-all-consistent-agreement.json')['failures'],'pendulum_analytic_check':analytic,'water_diagnosis':read('water-diagnosis.json'),'paper_source_gap':'Exact author water index-1 adaptation unavailable; independently derived system only','author_contact':'Cancelled by user; no request sent','public_claim':'Selected equations/examples and declared constant-mass index-1 scope; no universal error guarantee or full-bibliography reproduction','no_commits_or_pushes':True}
(ROOT/'agreement.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'coverage':result['mixed_explicit_budgets_raw_coverage'],'maximum_raw_reference_units':max([r['maximum_reference_tolerance_units']for r in runs+extended]+[r['maximum_reference_error_units']for r in water]),'pendulum_analytic':analytic,'optional_circle':optional},indent=2))
