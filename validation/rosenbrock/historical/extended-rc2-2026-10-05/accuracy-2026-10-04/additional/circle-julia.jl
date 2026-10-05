using OrdinaryDiffEqRosenbrock, LinearAlgebra
const SMB=OrdinaryDiffEqRosenbrock.SciMLBase
function f!(f,y,p,t);f[1]=y[2];f[2]=y[1]^2+y[2]^2-1;nothing;end
function j!(J,y,p,t);fill!(J,0);if p==:exact;J[1,2]=1;J[2,1]=2y[1];end;J[2,2]=2y[2];nothing;end
open(joinpath(@__DIR__,"circle-julia.csv"),"w") do io
 println(io,"jacobian,h,error,order")
 for kind in [:exact,:inexact]
  previous=0.
  for n in [8,16,32,64,128]
   h=1/n;prob=SMB.ODEProblem(SMB.ODEFunction(f!;jac=j!,tgrad=(f,y,p,t)->fill!(f,0),mass_matrix=diagm([1.,0.])),[0.,1.],(0.,1.),kind)
   sol=solve(prob,Rodas5P();adaptive=false,dt=h,save_everystep=false)
   @assert SMB.successful_retcode(sol)
   error=maximum(abs.(sol.u[end]-[sin(1.),cos(1.)]));order=previous==0 ? 0 : log2(previous/error)
   println(io,"$kind,$h,$error,$order");previous=error
  end
 end
end
