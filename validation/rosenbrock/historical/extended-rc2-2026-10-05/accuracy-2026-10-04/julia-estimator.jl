using OrdinaryDiffEqRosenbrock
const SMB=OrdinaryDiffEqRosenbrock.SciMLBase
f=SMB.ODEFunction((du,u,p,t)->(du[1]=t);jac=(J,u,p,t)->fill!(J,0),tgrad=(ft,u,p,t)->fill!(ft,1))
println("h,endpoint,error_estimate,derived_unscaled_error")
gamma=1/(2+sqrt(2))
for h in [0.1,0.05,0.025]
    prob=SMB.ODEProblem(f,[0.5],(1.0,2.0))
    integ=init(prob,Rosenbrock23();dt=h,dtmax=h,reltol=0.0,abstol=1.0,save_everystep=false)
    step!(integ)
    println("$h,$(integ.u[1]),$(OrdinaryDiffEqRosenbrock.OrdinaryDiffEqCore.get_EEst(integ)),$((1-gamma)*h^2/6)")
    @assert abs(integ.u[1]-(1+h)^2/2)<1e-14
    @assert abs(OrdinaryDiffEqRosenbrock.OrdinaryDiffEqCore.get_EEst(integ)-(1-gamma)*h^2/6)<1e-14
end
