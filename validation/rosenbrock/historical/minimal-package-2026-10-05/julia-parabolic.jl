using OrdinaryDiffEqRosenbrock, SparseArrays, InteractiveUtils
import Pkg
versioninfo(); Pkg.status()
const SMB=OrdinaryDiffEqRosenbrock.SciMLBase
const N=1000; const dx=2/(N+1); const dxx=1/dx^2
function rhs!(f,u,p,t)
 et=exp(t)
 for i in 1:N
  x=-1+i*dx; left=i==1 ? -et : u[i-1]; right=i==N ? et : u[i+1]
  f[i]=(left-2u[i]+right)*dxx+u[i]^2+(x^3-6x)*et-x^6*et^2
 end
end
function jac!(J,u,p,t)
 for i in 1:N
  J[i,i]=-2dxx+2u[i]
  if i>1; J[i,i-1]=dxx; end
  if i<N; J[i,i+1]=dxx; end
 end
end
function tgrad!(ft,u,p,t)
 et=exp(t)
 for i in 1:N
  x=-1+i*dx; ft[i]=(x^3-6x)*et-2x^6*et^2
 end
 ft[1]-=et*dxx;ft[N]+=et*dxx
end
proto=spdiagm(-1=>fill(dxx,N-1),0=>fill(-2dxx,N),1=>fill(dxx,N-1))
f=SMB.ODEFunction(rhs!;jac=jac!,tgrad=tgrad!,jac_prototype=proto)
y0=[(-1+i*dx)^3 for i in 1:N]
prob=SMB.ODEProblem(f,y0,(0.0,1.0))
println("method,h,error,steps")
for (name,alg) in [("rodas5p",Rodas5P()),("rodas5",Rodas5())]
 for h in [1/32,1/64,1/128]
  sol=solve(prob,alg;adaptive=false,dt=h,save_everystep=false)
  @assert SMB.successful_retcode(sol)
  err=maximum(abs.(sol.u[end].-exp(1)*y0))
  println("$name,$h,$err,$(sol.stats.naccept)")
 end
end
