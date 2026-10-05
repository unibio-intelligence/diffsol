"""Independent null-space elimination of water's original index-2 constraints."""
import argparse, json
from pathlib import Path
import numpy as np
import scipy
from scipy.linalg import null_space
from scipy.integrate import solve_ivp

ROOT = Path(__file__).resolve().parent
EDGES = [(0,1),(1,2),(1,5),(2,3),(2,4),(3,4),(4,9),(5,4),(6,3),
         (6,7),(7,4),(7,9),(8,7),(10,8),(10,11),(11,6),(11,7),(12,10)]
NODES = [4,7,0,1,2,3,5,6,8,9,10,11,12]
B = np.zeros((13,18))
for k,(i,j) in enumerate(EDGES): B[i,k]=-1; B[j,k]=1
C = B[NODES[2:]]
H = B[NODES[:2]]
N = null_space(C)
CCINV = np.linalg.inv(C@C.T)
AREA = np.pi/4
VMASS = 1e6/AREA
CMASS = 200/9800

def boundary(t,order=0):
    h=t/3600; e=np.exp(-h); a=e-1
    if order==0: inflow=1-np.cos(a); out=h*h*(3*h*h-92*h+720)/1e6
    else: inflow=-np.sin(a)*e/3600; out=(12*h**3-276*h*h+1440*h)/(1e6*3600)
    z=np.zeros(13); z[0]=inflow/200; z[12]=inflow/80; z[9]=-out
    return z

def eliminated(t,z):
    e=boundary(t); ep=boundary(t,1)
    q=-C.T@CCINV@e[NODES[2:]]+.001*N@z[:7]
    re=np.maximum(np.abs(q)/(1.31e-6*AREA),2300)
    # Colebrook solved in inverse sqrt(lambda), independently of the DAE.
    x=np.full(18,4.6)
    for _ in range(12):
        a=.0004+18.7*x/re
        residual=x-1.74+2*np.log10(a)
        x-=residual/(1+2/np.log(10)*18.7/re/a)
    lam=1/x**2
    loss=np.where(np.abs(q)/(1.31e-6*AREA)>2300,
                  lam*1e6*q*q/AREA**2,32*1.31*q/AREA)
    pbuf=1000*z[7:]
    palg=CCINV@(VMASS*ep[NODES[2:]]-C@H.T@pbuf-C@loss)
    p=np.zeros(13); p[NODES[:2]]=pbuf; p[NODES[2:]]=palg
    qdot=(-B.T@p-loss)/VMASS
    return q,lam,p,qdot

def rhs(t,z):
    q,_,_,qdot=eliminated(t,z)
    return np.r_[N.T@qdot/.001,H@q/(CMASS*1000)]

def full_state(t,z):
    q,lam,p,_=eliminated(t,z)
    return np.r_[q/.001,lam/.05,p[NODES]/1000]

def main():
    parser=argparse.ArgumentParser();parser.add_argument("--method",choices=["Radau","BDF"],default="Radau");args=parser.parse_args()
    assert np.linalg.matrix_rank(C)==11
    assert np.max(np.abs(C@N))<1e-14
    ts=np.r_[0.,np.linspace(.001,61200,101)]
    ys=[]
    for tol in [1e-12,1e-13]:
        sol=solve_ivp(rhs,(0,61200),np.zeros(9),method=args.method,
                      rtol=tol,atol=tol*.01,dense_output=True,max_step=120)
        assert sol.success,sol.message
        states=np.array([full_state(t,z) for t,z in zip(ts,sol.sol(ts).T)])
        ys.append(states)
        print('water reference',args.method,tol,'nfev',sol.nfev,flush=True)
    scale=1e-10+1e-8*np.abs(ys[1])
    report={'schema':'unibio.source-gap-reference/v1','scipy':scipy.__version__,'method':args.method,'coarse_rtol':1e-12,'fine_rtol':1e-13,'atol_fraction':.01,
            'formulation':'9-coordinate null-space ODE from original index-2 equations',
            'times':ts.tolist(),'values':ys[1].tolist(),
            'maximum_scaled_reference_change_at_target_1e-8':float(np.max(np.abs(ys[0]-ys[1])/scale)),
            'maximum_absolute_reference_change':float(np.max(np.abs(ys[0]-ys[1])))}
    (ROOT/('water-reference.json' if args.method=='Radau' else 'water-bdf-reference.json')).write_text(json.dumps(report,indent=2)+'\n')
    print('water reference change',report['maximum_scaled_reference_change_at_target_1e-8'],flush=True)
if __name__=='__main__': main()
