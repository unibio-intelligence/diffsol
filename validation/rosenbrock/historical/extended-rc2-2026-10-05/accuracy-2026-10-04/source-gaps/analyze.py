"""Compare recovered examples with analytic/null-space references and Julia."""
import csv,json,hashlib
from pathlib import Path
import numpy as np
from reference import B,C,NODES,boundary,AREA
ROOT=Path(__file__).resolve().parent

def load(paths):
    groups={}
    for path in paths:
        for row in csv.DictReader(path.open()):
            if row['mode']!='sampled': continue
            key=(row['problem'],row['method'],float(row['rtol']))
            groups.setdefault(key,[]).append(row)
    return groups

def evaluate(groups,radau,bdf):
    output=[]
    for (kind,method,target),rows in groups.items():
        assert len(rows)==101,(kind,method,target,len(rows))
        t=np.array([float(r['time'])for r in rows])
        y=np.array([[float(v)for v in r['values'].split(';')]for r in rows])
        exact=kind=='hyperbolic'
        truth=(1+np.arange(1,251)/250)[None,:]/(1+t[:,None]) if exact else np.array(radau['values'])[1:]
        if not exact: assert np.max(np.abs(t-np.array(radau['times'])[1:]))<1e-8
        scale=.01*target+target*abs(truth)
        error=abs(y-truth)/scale
        item={'problem':kind,'method':method,'rtol':target,
              'passes':int(rows[0]['passes']),'accepted_steps':int(rows[0]['total_steps']),
              'coarse_fine_agreement_units':float(rows[0]['convergence']),
              'maximum_reference_error_units':float(error.max()),
              'reference':'analytic transport solution' if exact else 'null-space ODE / Radau 1e-13'}
        if not exact:
            other=np.array(bdf['values'])[1:]
            item['maximum_BDF_reference_error_units']=float(np.max(abs(y-other)/(.01*target+target*abs(other))))
            q=.001*y[:,:18]
            balances=np.array([C@qi+boundary(ti)[NODES[2:]]for ti,qi in zip(t,q)])
            item['maximum_original_flow_balance_m3_per_s']=float(abs(balances).max())
            re=np.maximum(abs(q)/(1.31e-6*AREA),2300)
            lam=.05*y[:,18:36]
            colebrook=1/np.sqrt(lam)-1.74+2*np.log10(.0004+18.7/(re*np.sqrt(lam)))
            item['maximum_Colebrook_residual']=float(abs(colebrook).max())
            item['minimum_friction_factor']=float(lam.min())
        assert error.max()<1,item
        if not exact: assert item['maximum_BDF_reference_error_units']<1,item
        output.append(item)
    return output

def main():
    radau=json.loads((ROOT/'water-reference.json').read_text())
    bdf=json.loads((ROOT/'water-bdf-reference.json').read_text())
    rust=load([ROOT/'hyperbolic.csv',ROOT/'water.csv',ROOT/'water-rosenbrock23.csv'])
    julia=load([ROOT/'julia.csv'])
    results={'schema':'unibio.recovered-source-validation/v1','date':'2026-10-05',
             'diffsol':evaluate(rust,radau,bdf),'julia':evaluate(julia,radau,bdf),
             'water_reference_uncertainty':{
                 'Radau_refinement_change_units_at_1e-8':radau['maximum_scaled_reference_change_at_target_1e-8'],
                 'BDF_refinement_change_units_at_1e-8':bdf['maximum_scaled_reference_change_at_target_1e-8'],
                 'Radau_BDF_difference_units_at_1e-8':float(np.max(abs(np.array(radau['values'])-np.array(bdf['values']))/(1e-10+1e-8*abs(np.array(radau['values'])))))},
             'unresolved_runs':[],
             'exact_source_limitations':['2023 water index-1 adaptation is unspecified; independent reduction validated instead',
                                         '2020 hyperbolic variant and full 2023 timing curves not reproduced'],
             'no_commits_or_pushes':True}
    comparisons=[]
    for key,rows in rust.items():
        if key not in julia: continue
        yy=lambda data:np.array([[float(v)for v in r['values'].split(';')]for r in data])
        a,b=yy(rows),yy(julia[key]);target=key[2]
        comparisons.append({'problem':key[0],'method':key[1],'rtol':target,
                            'maximum_scaled_DiffSol_Julia_difference':float(np.max(abs(a-b)/(.01*target+target*np.maximum(abs(a),abs(b)))) )})
    results['adaptive_Julia_comparisons']=comparisons
    if (ROOT/'vendor.csv').exists():
        vendor=load([ROOT/'vendor.csv'])
        assert set(vendor)==set(rust)
        assert vendor==rust,'upstream/vendor completed sampled rows differ'
        results['upstream_vendor_bit_identical_completed_sampled_rows']=sum(map(len,rust.values()))
    for name in ['water-rosenbrock23.log','julia.log']:
        for line in (ROOT/name).read_text().splitlines():
            if line.startswith(('FAILED','UNRESOLVED')):
                results['unresolved_runs'].append({'log':name,'message':line})
    results['interrupted_diagnostic']={'log':'water.log','reason':'After both Rodas5P targets completed, the longer Rosenbrock23 attempt was interrupted and rerun with an explicit 250000 accepted-step cap. No failed numerical outcome is inferred from the interruption.'}
    results['files']=[{'path':f.name,'input_sha256_before_public_path_sanitization':hashlib.sha256(f.read_bytes()).hexdigest()}for f in sorted(ROOT.iterdir())if f.is_file() and f.name!='agreement.json']
    (ROOT/'agreement.json').write_text(json.dumps(results,indent=2,allow_nan=False)+'\n')
    print(json.dumps({k:results[k] for k in ['diffsol','julia','water_reference_uncertainty','unresolved_runs']},indent=2))
if __name__=='__main__':main()
