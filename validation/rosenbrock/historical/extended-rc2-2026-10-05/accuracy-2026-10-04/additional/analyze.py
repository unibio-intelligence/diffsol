import csv,json,sys,math
from collections import defaultdict
from pathlib import Path
import numpy as np
import reference
root=Path(__file__).resolve().parent
refs=json.loads((root/'references.json').read_text())['references']
input_name = sys.argv[1] if len(sys.argv)>1 else 'native'
groups=defaultdict(list)
for row in csv.DictReader((root/(input_name+'.csv')).open()):groups[(row['problem'],row['method'],float(row['rtol']),row['mode'])].append(row)
results=[]
for (name,method,tol,mode),rows in groups.items():
 ts=np.array([float(r['time']) for r in rows]); ys=np.array([[float(v) for v in r['values'].split(';')] for r in rows])
 if name.startswith('circle'):expected=np.array([np.sin(ts),np.cos(ts)]).T
 else:
  r=refs[name];assert np.max(np.abs(ts-r['times']))<1e-10,(name,ts,r['times']);expected=np.array(r['values'])
 scale=tol*.01+tol*np.abs(expected);error=float(np.max(np.abs(ys-expected)/scale))
 item=dict(problem=name,method=method,rtol=tol,mode=mode,passes=int(rows[0]['passes']),total_steps=int(rows[0]['total_steps']),maximum_reference_tolerance_units=error,convergence_units=float(rows[0]['convergence']) if math.isfinite(float(rows[0]['convergence'])) else None,maximum_absolute_error=float(np.max(np.abs(ys-expected))))
 if name=='hires':item['mass_drift']=float(np.max(np.abs(ys[:,6]+ys[:,7]-.0057)))
 if name.startswith('circle'):item['algebraic_residual']=float(np.max(np.abs(ys[:,0]**2+ys[:,1]**2-1)))
 if name=='pendulum':item['position_constraint_drift']=float(np.max(np.abs(ys[:,0]**2+ys[:,2]**2-4)));item['velocity_constraint_drift']=float(np.max(np.abs(ys[:,0]*ys[:,1]+ys[:,2]*ys[:,3])))
 if name=='amplifier':
  residual=[]
  for t,y in zip(ts,ys):
   a=1e-6*np.expm1((y[3]-y[2])/.026);b=1e-6*np.expm1((y[6]-y[5])/.026)
   f=[y[0]/9000,(y[1]-6)/9000+.99*a,y[2]/9000-a,(2*y[3]-6)/9000+.01*a,(y[4]-6)/9000+.99*b,y[5]/9000-b,(2*y[6]-6)/9000+.01*b,(y[7]-.1*np.sin(200*np.pi*t))/1000]
   residual.append(max(abs(f[0]+f[1]),abs(f[3]+f[4]),abs(f[6]+f[7])))
  item['algebraic_residual']=float(max(residual))
 if name=='pv':
  residual=[]
  for t,y in zip(ts,ys):
   v=y[1]-y[0];soc=y[6]/36000;oc=((6.8072*soc-10.5555)*soc+6.2199)*soc+10.2668
   residual.append(max(abs(y[0]),abs(y[4]+y[3]-y[2]),abs(reference.consumer(t)-y[2]*v),abs(-3.1037+1.0015*y[3]+.0032*v+1.3984e-9*np.expm1(.4303*y[3]+1.5*.9562*v)),abs(v-oc+y[5]+.2*y[4])))
  item['algebraic_residual']=float(max(residual))
 results.append(item)
(root/('agreement.json' if input_name=='native' else input_name+'-agreement.json')).write_text(json.dumps({'schema':'unibio.additional-sampled-agreement/v1','runs':results,'failures':[l.strip() for l in (root/(input_name+'.log')).read_text().splitlines() if l.startswith(('FAILED','UNRESOLVED','UNSUPPORTED'))]},indent=2)+'\n')
for r in results:
 if r['mode'] in {'sampled','endpoint-sampled','consistent-sampled'}:print(r['problem'],r['method'],r['rtol'],'error',r['maximum_reference_tolerance_units'],'passes',r['passes'])
