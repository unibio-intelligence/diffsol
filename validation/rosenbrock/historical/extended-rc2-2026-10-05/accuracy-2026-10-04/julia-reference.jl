using OrdinaryDiffEqRosenbrock, InteractiveUtils, LinearAlgebra
import Pkg
versioninfo(); Pkg.status()
const SMB = OrdinaryDiffEqRosenbrock.SciMLBase
const RT = OrdinaryDiffEqRosenbrock.OrdinaryDiffEqRosenbrockTableaus
println("REFERENCE_SOURCE=",pathof(OrdinaryDiffEqRosenbrock))
println("TABLEAU_SOURCE=",pathof(RT))
# Row-major fixture; independent Julia coefficients, not copied from the Rust tableau.
tab = OrdinaryDiffEqRosenbrock.Rodas5PTableau(Float64, Float64)
open(joinpath(@__DIR__, "julia-rodas5p-coefficients.txt"), "w") do io
    for (name, value) in [("gamma",tab.gamma),("A",tab.A),("C",tab.C),("c",tab.c),("d",tab.d),("H",tab.H)]
        println(io, name)
        for v in (value isa Number ? [value] : value isa Matrix ? vec(permutedims(value)) : value)
            println(io, repr(v))
        end
    end
end
function rhs!(du,u,p,t)
    if p==:decay; du[1]=-u[1]
    elseif p==:robertson
        du[1]=-0.04*u[1]+1e4*u[2]*u[3]; du[3]=3e7*u[2]^2; du[2]=-du[1]-du[3]
    elseif p==:vdp
        du[1]=u[2]; du[2]=1000*(1-u[1]^2)*u[2]-u[1]
    elseif p==:cosine; du[1]=cos(t)
    elseif p==:pr; du[1]=-1000*(u[1]-sin(t))+cos(t)
    end
end
function jac!(J,u,p,t)
    fill!(J,0)
    if p==:decay; J[1,1]=-1
    elseif p==:robertson
        J[1,1]=-0.04; J[1,2]=1e4*u[3]; J[1,3]=1e4*u[2]
        J[2,1]=0.04; J[2,2]=-1e4*u[3]-6e7*u[2]; J[2,3]=-1e4*u[2]; J[3,2]=6e7*u[2]
    elseif p==:vdp; J[1,2]=1; J[2,1]=-2000*u[1]*u[2]-1; J[2,2]=1000*(1-u[1]^2)
    elseif p==:pr; J[1,1]=-1000
    end
end
function tgrad!(ft,u,p,t)
    fill!(ft,0)
    if p==:cosine; ft[1]=-sin(t)
    elseif p==:pr; ft[1]=1000*cos(t)-sin(t)
    end
end
open(joinpath(@__DIR__, "julia-trajectories.csv"),"w") do io
    println(io,"method,problem,mode,tolerance,h,t,steps,rejects,y")
    for (name,alg) in [("rodas5p",Rodas5P()),("rosenbrock23",Rosenbrock23())]
        for (p,y0,h,T) in [(:decay,[1.0],0.01,0.1),(:robertson,[1.0,0,0],0.001,0.01),(:vdp,[2.0,0],0.00001,0.0001),(:cosine,[0.0],0.01,0.1),(:pr,[0.0],0.001,0.01)]
            f=SMB.ODEFunction(rhs!;jac=jac!,tgrad=tgrad!)
            prob=SMB.ODEProblem(f,y0,(0.0,T),p)
            sol=solve(prob,alg;adaptive=false,dt=h,save_everystep=false)
            println(io,"$name,$p,fixed,0,$h,$T,$(sol.stats.naccept),$(sol.stats.nreject),",join(repr.(sol.u[end]),";"))
            @assert SMB.successful_retcode(sol)
            for rtol in [1e-6,1e-9]
                tend = p==:robertson ? 1e4 : p==:vdp ? 1.0 : p==:decay ? 1.0 : p==:cosine ? 10.0 : 1.0
                sol=solve(SMB.remake(prob;tspan=(0.0,tend)),alg;reltol=rtol,abstol=rtol*0.01,save_everystep=false,maxiters=10^7)
                @assert SMB.successful_retcode(sol)
                println(io,"$name,$p,adaptive,$rtol,0,$tend,$(sol.stats.naccept),$(sol.stats.nreject),",join(repr.(sol.u[end]),";"))
            end
        end
    end
end
open(joinpath(@__DIR__,"julia-stability.csv"),"w") do io
    println(io,"z,R")
    for z in [-1.0,-10.0,-1000.0,-1e8,-1e14]
        f=SMB.ODEFunction((du,u,p,t)->(du[1]=z*u[1]);jac=(J,u,p,t)->(J[1,1]=z),tgrad=(ft,u,p,t)->fill!(ft,0))
        sol=solve(SMB.ODEProblem(f,[1.0],(0.0,1.0)),Rodas5P();adaptive=false,dt=1.0)
        println(io,"$z,$(repr(sol.u[end][1]))")
    end
end
println("Reference run complete")
