using OrdinaryDiffEqRosenbrock, LinearAlgebra, InteractiveUtils
const SMB=OrdinaryDiffEqRosenbrock.SciMLBase
versioninfo()
function rhs!(f,y,kind,t)
 if kind==:hires
  a,b,c,d,e,g,h,z=y;r=280*g*z
  f.=[-1.71*a+.43*b+8.32*c+.0007,1.71*a-8.75*b,-10.03*c+.43*d+.035*e,8.32*b+1.71*c-1.12*d,-1.745*e+.43*g+.43*h,-r+.69*d+1.71*e-.43*g+.69*h,r-1.81*h,-r+1.81*h]
 elseif kind==:oregonator
  a,b,c=y;f.=[77.27*(b+a*(1-8.375e-6*a-b)),(c-(1+a)*b)/77.27,.161*(a-c)]
 elseif kind==:amplifier
  a=1e-6*expm1((y[4]-y[3])/.026);b=1e-6*expm1((y[7]-y[6])/.026)
  f.=[y[1]/9000,(y[2]-6)/9000+.99*a,y[3]/9000-a,(2*y[4]-6)/9000+.01*a,(y[5]-6)/9000+.99*b,y[6]/9000-b,(2*y[7]-6)/9000+.01*b,(y[8]-.1*sin(200*pi*t))/1000]
 elseif kind==:pendulum
  x,vx,z,vz,l=y;f.=[vx,l*x,vz,l*z-9.81,vx^2+vz^2+l*(x^2+z^2)-9.81*z]
 elseif kind in (:circle,:circle_inexact)
  f.=[y[2],y[1]^2+y[2]^2-1]
 elseif kind==:pv
  u0,u1,ic,ipv,ib,ub,q=y;v=u1-u0;s=q/36000;oc=((6.8072*s-10.5555)*s+6.2199)*s+10.2668
  power=sum((isodd(i) ? 1 : -1)*25*(tanh((t-3600*i)*3.8002/60)+1) for i in 1:10)
  f.=[u0,ib+ipv-ic,power-ic*v,-3.1037+1.0015*ipv+.0032*v+1.3984e-9*expm1(.4303*ipv+1.5*.9562*v),v-oc+ub+.2*ib,ib/4000-ub/2000,-ib]
 elseif kind==:pollution
  k=[.35,26.6,12300,.00086,.00082,15000,.00013,24000,16500,9000,.022,12000,1.88,16300,4.8e6,.00035,.0175,1e8,4.44e11,1240,2.1,5.78,.0474,1780,3.12]
  pairs=[(1,0),(2,4),(5,2),(7,0),(7,0),(7,6),(9,0),(9,6),(11,2),(11,1),(13,0),(10,2),(14,0),(1,6),(3,0),(4,0),(4,0),(16,0),(16,0),(17,6),(19,0),(19,0),(1,4),(19,1),(20,0)]
  r=[c*y[a]*(b==0 ? 1 : y[b]) for (c,(a,b)) in zip(k,pairs)]
  f.=[-r[1]-r[10]-r[14]-r[23]-r[24]+r[2]+r[3]+r[9]+r[11]+r[12]+r[22]+r[25],-r[2]-r[3]-r[9]-r[12]+r[1]+r[21],-r[15]+r[1]+r[17]+r[19]+r[22],-r[2]-r[16]-r[17]-r[23]+r[15],-r[3]+2*r[4]+r[6]+r[7]+r[13]+r[20],-r[6]-r[8]-r[14]-r[20]+r[3]+2*r[18],-r[4]-r[5]-r[6]+r[13],r[4]+r[5]+r[6]+r[7],-r[7]-r[8],-r[12]+r[7]+r[9],-r[9]-r[10]+r[8]+r[11],r[9],-r[11]+r[10],-r[13]+r[12],r[14],-r[18]-r[19]+r[16],-r[20],r[20],-r[21]-r[22]-r[24]+r[23]+r[25],-r[25]+r[24]]
 end
 nothing
end
function tgrad!(f,y,kind,t)
 fill!(f,0)
 if kind==:amplifier;f[8]=-.1*200*pi*cos(200*pi*t)/1000
 elseif kind==:pv;f[3]=sum((isodd(i) ? 1 : -1)*25*3.8002/60*(1-tanh((t-3600*i)*3.8002/60)^2) for i in 1:10)
 end
 nothing
end
function circle_jac!(J,y,p,t)
 fill!(J,0);if p==:circle;J[1,2]=1;J[2,1]=2*y[1];end;J[2,2]=2*y[2];nothing
end
cases=[(:hires,[1.,0,0,0,0,0,0,.0057],321.8122),(:oregonator,[1.,2,3],360.),(:amplifier,[0.,6,3,3,6,3,3,0],.05),(:pendulum,[2.,0,0,0,0],10.),(:circle,[0.,1],1.),(:circle_inexact,[0.,1],1.),(:pollution,[0.,.2,0,.04,0,0,.1,.3,.01,0,0,0,0,0,0,0,.007,0,0,0],60.),(:pv,[0.,11.856598910310167,0,2.9409008015416687,-2.940900801550821,0,9000],36000.)]
open(get(ENV,"VALIDATION_CSV",joinpath(@__DIR__,"julia.csv")),"w") do io
 println(io,"problem,method,rtol,mode,passes,total_steps,convergence,time,values")
 for(kind,y0,tend) in cases
  haskey(ENV,"VALIDATION_PROBLEM") && ENV["VALIDATION_PROBLEM"] != string(kind) && continue
  if kind==:pv
   soc=y0[7]/36000;oc=((6.8072*soc-10.5555)*soc+6.2199)*soc+10.2668
   lo=0.;hi=10.
   for iter in 1:80
    current=(lo+hi)/2;voltage=oc+.2*current
    residual=-3.1037+1.0015*current+.0032*voltage+1.3984e-9*expm1(.4303*current+1.5*.9562*voltage)
    if residual>0;hi=current;else;lo=current;end
   end
   y0[4]=(lo+hi)/2;y0[5]=-y0[4];y0[2]=oc+.2*y0[4]
  end
  n=length(y0);M=Matrix{Float64}(I,n,n)
  if kind==:amplifier
   fill!(M,0);for(a,c)in[(1,5e-6),(4,3e-6),(7,1e-6)];M[a,a]=-c;M[a+1,a+1]=-c;M[a,a+1]=c;M[a+1,a]=c;end;M[3,3]=-4e-6;M[6,6]=-2e-6
  elseif kind==:pendulum;M[5,5]=0
  elseif kind in (:circle,:circle_inexact);M[2,2]=0
  elseif kind==:pv;M=diagm([0.,0,0,0,0,1,1])
  end
  ts=kind==:pv ? sort(unique(vcat([1.,100,1000,36000],[3600*i+d for i in 1:10 for d in [-60.,-10.,0.,10.,60.] if 3600*i+d<=tend]))) : collect(range(kind==:amplifier ? .0005 : .001,tend,length=kind==:amplifier ? 100 : 101))
  f=kind in (:circle,:circle_inexact) ? SMB.ODEFunction(rhs!;mass_matrix=M,jac=circle_jac!,tgrad=tgrad!) : SMB.ODEFunction(rhs!;mass_matrix=M,tgrad=tgrad!)
  prob=SMB.ODEProblem(f,y0,(0.,tend),kind)
  for(name,alg)in[("rodas5p",Rodas5P()),("rosenbrock23",Rosenbrock23())], target in [1e-6,1e-8]
   haskey(ENV,"VALIDATION_METHOD") && ENV["VALIDATION_METHOD"] != name && continue
   haskey(ENV,"VALIDATION_TARGET") && parse(Float64,ENV["VALIDATION_TARGET"]) != target && continue
   if kind==:amplifier && name=="rosenbrock23";println("UNSUPPORTED $kind $name: Julia requires diagonal mass matrix");continue;end
   previous=nothing;total=0;factor=1.;passed=false
   for pass in 1:parse(Int,get(ENV,"VALIDATION_PASSES","8"))
    println("START $kind $name $target pass=$pass");flush(stdout)
    sol=solve(prob,alg;reltol=target*factor,abstol=.01*target*factor,saveat=ts,save_everystep=false,maxiters=parse(Int,get(ENV,"VALIDATION_STEP_CAP",name=="rodas5p" ? "500000" : "5000000")),dt=kind==:amplifier ? 1e-6 : nothing,dtmax=kind==:pv ? 60*sqrt(factor) : Inf)
    if !SMB.successful_retcode(sol);println("FAILED $kind $name $target pass=$pass $(sol.retcode)");break;end
    ys=[collect(sol(t)) for t in ts];total+=sol.stats.naccept
    change=previous===nothing ? Inf : maximum(abs(a-b)/(.01*target+target*max(abs(a),abs(b))) for (x,y)in zip(previous,ys) for (a,b)in zip(x,y))
    if pass==1 || change<=.25
     mode=pass==1 ? "local" : "sampled"
     for(t,y)in zip(ts,ys);println(io,"$kind,$name,$target,$mode,$pass,$total,$change,$t,",join(repr.(y),";"));end
    end
    if change<=.25;println("$kind $name $target passes=$pass steps=$total");passed=true;break;end
    previous=ys;factor*=.1
   end
   if !passed;println("UNRESOLVED $kind $name $target");end
  end
 end
end
