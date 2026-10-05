"""Independent SciPy references; amplifier constraints are eliminated analytically."""
import json
from pathlib import Path
import numpy as np
import scipy
from scipy.integrate import solve_ivp
from scipy.special import wrightomega
from scipy.optimize import brentq
ROOT=Path(__file__).resolve().parent

def hires(t,y):
 a,b,c,d,e,f,g,h=y; reaction=280*f*h
 return [-1.71*a+.43*b+8.32*c+.0007,1.71*a-8.75*b,-10.03*c+.43*d+.035*e,8.32*b+1.71*c-1.12*d,-1.745*e+.43*f+.43*g,-reaction+.69*d+1.71*e-.43*f+.69*g,reaction-1.81*g,-reaction+1.81*g]
def oregonator(t,y):
 a,b,c=y
 return [77.27*(b+a*(1-8.375e-6*a-b)),(c-(1+a)*b)/77.27,.161*(a-c)]
def pendulum(t,y):return [y[1],-9.81/2*np.sin(y[0])]
def pendulum_states(t,y):
 a,w=y; x=2*np.sin(a); z=-2*np.cos(a); vx=2*np.cos(a)*w; vz=2*np.sin(a)*w
 return np.array([x,vx,z,vz,(9.81*z-vx*vx-vz*vz)/4])
# Hairer-Wanner order: [y0,y1,y2,y3,y4,y5,y6,y7]. Reduced
# coordinates are the three capacitor voltage differences and two grounded ones.
def diode_root(A,B,C,other):
 return -C/A-.026*float(wrightomega(np.log(B/(A*.026))+(-C/A-other)/.026))
def amplifier_states(t,z):
 d0,d1,d2,y2,y5=z; source=.1*np.sin(2*np.pi*100*t); beta=1e-6; alpha=.99
 y6=diode_root(2/9000+1/1000,(1-alpha)*beta,d2/1000-6/9000-source/1000-(1-alpha)*beta,y5)
 i2=beta*np.expm1((y6-y5)/.026)
 y3=diode_root(3/9000,(1-alpha)*beta,(d1-12)/9000+alpha*i2-(1-alpha)*beta,y2)
 i1=beta*np.expm1((y3-y2)/.026)
 y0=(6/9000-d0/9000-alpha*i1)/(2/9000)
 return np.array([y0,y0+d0,y2,y3,y3+d1,y5,y6,y6+d2])
def amplifier(t,z):
 y=amplifier_states(t,z); i1=1e-6*np.expm1((y[3]-y[2])/.026); i2=1e-6*np.expm1((y[6]-y[5])/.026)
 return [y[0]/9000/5e-6,(y[3]/9000+(y[3]-6)/9000+.01*i1)/3e-6,(y[6]/9000+(y[6]-6)/9000+.01*i2)/1e-6,-(y[2]/9000-i1)/4e-6,-(y[5]/9000-i2)/2e-6]

def pollution(t,y):
 k=np.array([.35,26.6,12300,.00086,.00082,15000,.00013,24000,16500,9000,.022,12000,1.88,16300,4.8e6,.00035,.0175,1e8,4.44e11,1240,2.1,5.78,.0474,1780,3.12])
 pairs=[(0,), (1,3),(4,1),(6,),(6,),(6,5),(8,),(8,5),(10,1),(10,0),(12,),(9,1),(13,),(0,5),(2,),(3,),(3,),(15,),(15,),(16,5),(18,),(18,),(0,3),(18,0),(19,)]
 r=[c*np.prod(y[list(p)]) for c,p in zip(k,pairs)]
 a,b,c,d,e,f,g,h,i,j,l,m,n,o,p,q,s,u,v,w,xx,yy,zz,aa,bb=r
 return [-a-j-o-zz-aa+b+c+i+l+m+yy+bb,-b-c-i-m+a+xx,-p+a+s+v+yy,-b-q-s-zz+p,-c+2*d+f+g+n+w,-f-h-o-w+c+2*u,-d-e-f+n,d+e+f+g,-g-h,-m+g+i,-i-j+h+l,i,-l+j,-n+m,o,-u-v+q,-w,w,-xx-yy-aa+zz+bb,-bb+aa]

def consumer(t):
 return sum((-1 if i%2==0 else 1)*25*(np.tanh((t-3600*i)*3.8002/60)+1) for i in range(1,11))
def pv_states(t,y):
 ub,q=y; soc=q/36000; oc=((6.8072*soc-10.5555)*soc+6.2199)*soc+10.2668; power=consumer(t)
 def residual(v):
  ib=(oc-ub-v)/.2; ipv=power/v-ib
  return -3.1037+1.0015*ipv+.0032*v+1.3984e-9*np.expm1(np.clip(.4303*ipv+1.5*.9562*v,-700,700))
 # Physical, high-voltage constant-power branch; lower branch is excluded.
 voltage=brentq(residual,5,25,xtol=1e-13)
 ib=(oc-ub-voltage)/.2; ic=power/voltage
 return np.array([0,voltage,ic,ic-ib,ib,ub,q])
def pv(t,y):
 state=pv_states(t,y);return [state[4]/4000-y[0]/2000,-state[4]]

def main():
 cases=[('hires',hires,[1,0,0,0,0,0,0,.0057],321.8122,None),('oregonator',oregonator,[1,2,3],360,None),('amplifier',amplifier,[6,3,-3,3,3],.05,amplifier_states),('pendulum',pendulum,[np.pi/2,0],10,pendulum_states),('pollution',pollution,[0,.2,0,.04,0,0,.1,.3,.01,0,0,0,0,0,0,0,.007,0,0,0],60,None),('pv',pv,[0,9000],36000,pv_states)]
 result={'schema':'unibio.independent-additional-examples/v1','scipy':scipy.__version__,'references':{}}
 for name,fn,y0,end,transform in cases:
  times=(np.linspace(.001,end,101) if name!='amplifier' else np.linspace(.0005,end,100))
  if name=='pv':times=np.array(sorted(set([1.,100.,1000.,36000.]+[3600.*i+d for i in range(1,11) for d in [-60.,-10.,0.,10.,60.] if 3600.*i+d<=36000])))
  sols=[]
  for tol in [1e-11,1e-13]:
   sol=solve_ivp(fn,(0,end),y0,method='Radau',rtol=tol,atol=tol*.001,dense_output=True)
   assert sol.success,(name,sol.message)
   ys=sol.sol(times).T
   if transform:ys=np.array([transform(t,y) for t,y in zip(times,ys)])
   sols.append(ys)
  delta=np.max(np.abs(sols[0]-sols[1])); result['references'][name]={'times':times.tolist(),'values':sols[1].tolist(),'maximum_reference_refinement_change':float(delta),'method':'SciPy Radau','rtol':1e-13,'atol':1e-16,'formulation':'independently eliminated capacitor constraints' if name=='amplifier' else 'independent angular ODE' if name=='pendulum' else 'independently eliminated high-voltage electrical constraints' if name=='pv' else 'published ODE'}
  print(name,'reference delta',delta,flush=True)
 (ROOT/'references.json').write_text(json.dumps(result,indent=2)+'\n')
if __name__=='__main__':main()
