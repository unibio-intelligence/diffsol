using OrdinaryDiffEqRosenbrock, LinearAlgebra
const SMB=OrdinaryDiffEqRosenbrock.SciMLBase
setprecision(256)
function rhs!(f,y,g,t)
 x,vx,z,vz,l=y;f.=[vx,l*x,vz,l*z-g,vx^2+vz^2+l*(x*x+z*z)-g*z];nothing
end
function jac!(J,y,g,t)
 x,vx,z,vz,l=y;fill!(J,0);J[1,2]=1;J[2,1]=l;J[2,5]=x;J[3,4]=1;J[4,3]=l;J[4,5]=z;
 J[5,:].=[2l*x,2vx,2l*z-g,2vz,x*x+z*z];nothing
end
truth=BigFloat[parse(BigFloat,"1.9999082233665820772041538973124222223257764712931615048827699920444722322"),parse(BigFloat,"-0.00587362367801053321614156793282412897609083494558872100195981365522825574616"),parse(BigFloat,"-0.0191598045585347674652631159481225710586642379665463822905190194616512295483"),parse(BigFloat,"-0.613091237894769843972891198416940317932494870046362163331218132791425825504"),parse(BigFloat,"-0.140968262039419558772949848104631260179847544492308223892495756220043843017")]
println("precision,h,steps,max_endpoint_error,max_constraint,retcode")
for T in [Float64,BigFloat], h in [2.0^-12,2.0^-16,2.0^-20,2.0^-24]
 y0=T[2,0,0,0,0];M=diagm(T[1,1,1,1,0]);g=T(9.81)
 f=SMB.ODEFunction(rhs!;jac=jac!,tgrad=(f,y,g,t)->fill!(f,0),mass_matrix=M)
 p=SMB.ODEProblem(f,y0,(T(0),T(1)/16),g)
 sol=solve(p,Rosenbrock23();adaptive=false,dt=T(h),save_everystep=false,initializealg=SMB.NoInit(),maxiters=2000000)
 err=maximum(abs.(BigFloat.(sol.u[end]).-truth));u=sol.u[end];res=abs(u[2]^2+u[4]^2+u[5]*(u[1]^2+u[3]^2)-g*u[3]);
 println("$T,$h,$(sol.stats.naccept),$err,$res,$(sol.retcode)");flush(stdout)
end
