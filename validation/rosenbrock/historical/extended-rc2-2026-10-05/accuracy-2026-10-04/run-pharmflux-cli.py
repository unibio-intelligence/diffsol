#!/usr/bin/env python3
"""Exercise sparse Robertson samples through the actual public PharmFlux CLI."""
import argparse, json, subprocess, tempfile
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--pharmflux',type=Path,required=True);p.add_argument('--cli',type=Path,required=True);p.add_argument('--results',type=Path,required=True);p.add_argument('--accuracy-control',choices=['local','sampled'],default='local');a=p.parse_args()
reference=json.loads((a.pharmflux/'conformance/references/robertson-radau.json').read_text());times=reference['times'];a.results.mkdir(parents=True,exist_ok=True)
records=[]
print('method,rtol,maximum_requested_tolerance_units')
with tempfile.TemporaryDirectory(prefix='pharmflux-cli-reference-') as tmp:
 for rtol in [1e-3,1e-4,1e-5,1e-6]:
  for method in ['bdf','esdirk34','tr_bdf2','rosenbrock23','rodas5p']:
   request={'schema':'pharmflux.run/v0.1','solver':'diffsol_'+method,'budgets':{'solver_callbacks':10000000 if a.accuracy_control=='sampled' else 1000000,'output_values':10000,'events':100},'regimen':{'end':{'value':10000,'unit':'s'},'samples':[{'value':t,'unit':'s'} for t in times],'administrations':[],'rtol':rtol,'atol':1e-12}}
   if a.accuracy_control!='local':request['accuracy_control']=a.accuracy_control
   path=Path(tmp)/'request.json';path.write_text(json.dumps(request));result=subprocess.run([str(a.cli),'run',str(a.pharmflux/'conformance/models/synthetic-robertson.pfx'),str(path)],capture_output=True,text=True,timeout=60);assert result.returncode==0,result.stderr
   value=json.loads(result.stdout);maximum=0.0
   for row,t in enumerate(value['times']):
    expected=reference['states'][times.index(t)]
    actual=[output['values'][row] for output in value['outputs']]
    assert abs(sum(actual)-1)<1e-8
    maximum=max(maximum,*[abs(x-y)/(1e-12+rtol*abs(y)) for x,y in zip(actual,expected)])
   assert maximum<(1 if a.accuracy_control=='sampled' or method=='tr_bdf2' else 10),(method,rtol,maximum)
   name=f'{method}-{rtol}.json';(a.results/name).write_text(json.dumps(value,indent=2)+'\n')
   records.append({'method':method,'rtol':rtol,'maximum_requested_tolerance_units':maximum,'result':name})
   print(f'{method},{rtol},{maximum:.16g}')
(a.results/'receipt.json').write_text(json.dumps({'source':'classic Robertson benchmark, independently generated Radau comparator','accuracy_control':a.accuracy_control,'runs':records},indent=2)+'\n')
