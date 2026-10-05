import mpmath as mp,json
from pathlib import Path
mp.mp.dps=60;m=mp.mpf('0.5');k=mp.sqrt(m);g=mp.mpf(9.81);om=mp.sqrt(g/2);K=mp.ellipk(m)
root=Path(__file__).resolve().parent.parent; d=json.loads((root/'additional/references.json').read_text())['references']['pendulum'];values=[]
for t in d['times']:
 u=K-om*mp.mpf(t);sn=mp.ellipfun('sn',u,m);cn=mp.ellipfun('cn',u,m);a=2*mp.asin(k*sn);w=-2*k*om*cn;x=2*mp.sin(a);z=-2*mp.cos(a);vx=2*mp.cos(a)*w;vz=2*mp.sin(a)*w;lam=(g*z-vx*vx-vz*vz)/4;values.append([float(v) for v in [x,vx,z,vz,lam]])
err=max(abs(a-b)for x,y in zip(values,d['values'])for a,b in zip(x,y));print('Analytic elliptic / Radau maximum absolute discrepancy',err)
Path(__file__).with_suffix('.json').write_text(json.dumps({'method':'Jacobi elliptic analytic solution','mpmath':mp.__version__,'decimal_precision':60,'gravity':'exact binary64 value of 9.81','times':d['times'],'values':values,'maximum_Radau_discrepancy':err},indent=2)+'\n')
