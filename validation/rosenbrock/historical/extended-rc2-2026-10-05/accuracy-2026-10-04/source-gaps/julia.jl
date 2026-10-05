using OrdinaryDiffEqRosenbrock, LinearAlgebra
const SMB=OrdinaryDiffEqRosenbrock.SciMLBase
const EDGES=[(1,2),(2,3),(2,6),(3,4),(3,5),(4,5),(5,10),(6,5),(7,4),
             (7,8),(8,5),(8,10),(9,8),(11,9),(11,12),(12,7),(12,8),(13,11)]
const NODES=[5,8,1,2,3,4,6,7,9,10,11,12,13]
const IDX=invperm(NODES)
function boundary(t,order)
 h=t/3600; e=exp(-h); a=e-1
 if order==0; input=1-cos(a);output=h*h*(3*h*h-92*h+720)/1e6
 elseif order==1;input=-sin(a)*e/3600;output=(12*h^3-276*h*h+1440*h)/(1e6*3600)
 else;input=(cos(a)*e*e+sin(a)*e)/3600^2;output=(36*h*h-552*h+1440)/(1e6*3600^2)
 end
 b=zeros(13);b[1]=input/200;b[13]=input/80;b[10]=-output;b
end
function rhs!(f,y,kind,t)
 if kind==:hyperbolic
  for j in 1:250
   left=j==1 ? 1/(1+t) : y[j-1]
   f[j]=-250*(y[j]-left)+(t-j/250)/(1+t)^2
  end
 else
  area=pi/4;vmass=1e6/area;cmass=200/9800
  net=eltype(y).(boundary(t,0));accel=eltype(y).(boundary(t,1))
  for(k,(i,j))in enumerate(EDGES)
   q=.001*y[k];l=.05*y[18+k];s=sqrt(l);r=abs(q/(1.31e-6*area));re=max(r,2300)
   loss=r>2300 ? l*1e6*q*q/area^2 : 32*1.31*q/area
   f[k]=(1000*(y[36+IDX[i]]-y[36+IDX[j]])-loss)/(vmass*.001)
   f[18+k]=1/s-1.74+2*log10(.0004+18.7/(re*s))
   net[i]-=q;net[j]+=q;accel[i]-=.001*f[k];accel[j]+=.001*f[k]
  end
  f[37]=net[5]/(cmass*1000);f[38]=net[8]/(cmass*1000)
  for(k,node)in enumerate(NODES[3:end]);f[38+k]=accel[node]/.001;end
 end
 nothing
end
function tgrad!(f,y,kind,t)
 fill!(f,0)
 if kind==:hyperbolic
  for j in 1:250;f[j]=1/(1+t)^2-2*(t-j/250)/(1+t)^3;end
  f[1]-=250/(1+t)^2
 else
  b=boundary(t,2)
  for(k,node)in enumerate(NODES[3:end]);f[38+k]=b[node]/.001;end
 end
 nothing
end
open(joinpath(@__DIR__,"julia.csv"),"w")do io
 println(io,"problem,method,rtol,mode,passes,total_steps,convergence,time,values")
 for kind in [:hyperbolic,:water_index1]
  y0=kind==:hyperbolic ? [1+j/250 for j in 1:250] : vcat(zeros(18),fill(.047519404529185289807/.05,18),zeros(13))
  tend=kind==:hyperbolic ? 1. : 61200.
  ts=collect(range(.001,tend,length=101))
  M=kind==:hyperbolic ? I : Diagonal(vcat(ones(18),zeros(18),ones(2),zeros(11)))
  f=SMB.ODEFunction(rhs!;mass_matrix=M,tgrad=tgrad!)
  prob=SMB.ODEProblem(f,y0,(0.,tend),kind)
  for(name,alg)in [("rodas5p",Rodas5P()),("rosenbrock23",Rosenbrock23())], target in [1e-6,1e-8]
   previous=nothing;total=0;factor=1.;passed=false
   for pass in 1:8
    println("START $kind $name $target pass=$pass");flush(stdout)
    sol=solve(prob,alg;reltol=target*factor,abstol=.01*target*factor,
       saveat=ts,save_everystep=false,maxiters=5_000_000,
       dtmax=kind==:water_index1 ? 120*sqrt(factor) : Inf)
    if !SMB.successful_retcode(sol);println("FAILED $kind $name $target pass=$pass $(sol.retcode)");break;end
    ys=[collect(sol(t)) for t in ts];total+=sol.stats.naccept
    change=previous===nothing ? Inf : maximum(abs(a-b)/(.01*target+target*max(abs(a),abs(b))) for (x,y) in zip(previous,ys) for (a,b) in zip(x,y))
    if pass==1 || change<=.25
     mode=pass==1 ? "local" : "sampled"
     for(t,y)in zip(ts,ys);println(io,"$kind,$name,$target,$mode,$pass,$total,$change,$t,",join(repr.(y),";"));end
    end
    if change<=.25;passed=true;println("PASS $kind $name $target passes=$pass steps=$total");break;end
    previous=ys;factor*=.1
   end
   if !passed;println("UNRESOLVED $kind $name $target");end
  end
 end
end
