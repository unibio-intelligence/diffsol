use diffsol::*;
type Mat=NalgebraMat<f64>; type LS=NalgebraLU<f64>;
fn problem(kind:&'static str, rtol:f64) -> OdeSolverProblem<impl OdeEquationsImplicit<T=f64,M=Mat,V=NalgebraVec<f64>,C=NalgebraContext>> {
    let n=match kind {"robertson"=>3,"vdp"=>2,_=>1};
    OdeBuilder::<Mat>::new().rtol(rtol).atol(vec![rtol*0.01;n])
        .rhs_implicit(move |u,_,t,f| match kind {
            "decay"=>f[0]=-u[0],
            "robertson"=>{f[0]=-0.04*u[0]+1e4*u[1]*u[2];f[2]=3e7*u[1]*u[1];f[1]=-f[0]-f[2];},
            "vdp"=>{f[0]=u[1];f[1]=1000.0*(1.0-u[0]*u[0])*u[1]-u[0];},
            "cosine"=>f[0]=t.cos(), "pr"=>f[0]=-1000.0*(u[0]-t.sin())+t.cos(), _=>unreachable!()},
            move |u,_,_,v,j| match kind {
                "decay"=>j[0]=-v[0],
                "robertson"=>{j[0]=-0.04*v[0]+1e4*u[2]*v[1]+1e4*u[1]*v[2];j[2]=6e7*u[1]*v[1];j[1]=-j[0]-j[2];},
                "vdp"=>{j[0]=v[1];j[1]=(-2000.0*u[0]*u[1]-1.0)*v[0]+1000.0*(1.0-u[0]*u[0])*v[1];},
                "cosine"=>j[0]=0.0,"pr"=>j[0]=-1000.0*v[0],_=>unreachable!()})
        .init(move |_,_,y|{y.fill(0.0); y[0]=match kind {"decay"|"robertson"=>1.0,"vdp"=>2.0,_=>0.0};},n).build().unwrap()
}
fn advance<'a, E: OdeEquationsImplicit<T=f64>+'a,S:OdeSolverMethod<'a,E>>(s:&mut S,t:f64){ s.set_stop_time(t).unwrap();while s.step().unwrap()!=OdeSolverStopReason::TstopReached{} }
fn endpoint(v:&NalgebraVec<f64>)->String{(0..v.len()).map(|i|format!("{:.17e}",v[i])).collect::<Vec<_>>().join(";")}
fn main(){
 println!("method,problem,mode,tolerance,h,t,steps,rejects,y");
 for (kind,h,t) in [("decay",0.01,0.1),("robertson",0.001,0.01),("vdp",1e-5,1e-4),("cosine",0.01,0.1),("pr",0.001,0.01)] {
  for (name,tableau) in [("rodas5p",Tableau::rodas5p()),("rosenbrock23",Tableau::rosenbrock23())] {
   let p=problem(kind,10.0); let mut s=p.rosenbrock_solver::<LS,Mat>(p.rodas5p_state::<LS>().unwrap(),tableau).unwrap();
   *s.state_mut().h=h;s.config_mut().minimum_timestep_growth=1.0;s.config_mut().maximum_timestep_growth=1.0;advance(&mut s,t);
   println!("{name},{kind},fixed,0,{h},{t},{},{},{}",s.get_statistics().number_of_steps,s.get_statistics().number_of_error_test_failures,endpoint(s.state().y));
   for rtol in [1e-6,1e-9]{
    let tend=match kind {"robertson"=>1e4,"cosine"=>10.0,_=>1.0};
    let p=problem(kind,rtol);let mut s=p.rosenbrock_solver::<LS,Mat>(p.rodas5p_state::<LS>().unwrap(),tableau).unwrap();advance(&mut s,tend);
    println!("{name},{kind},adaptive,{rtol},0,{tend},{},{},{}",s.get_statistics().number_of_steps,s.get_statistics().number_of_error_test_failures,endpoint(s.state().y));
   }
  }
  let tend=match kind {"robertson"=>1e4,"cosine"=>10.0,_=>1.0};
  let p=problem(kind,1e-12);let mut s=p.bdf::<LS>().unwrap();advance(&mut s,tend);
  println!("bdf,{kind},reference,1e-12,0,{tend},{},{},{}",s.get_statistics().number_of_steps,s.get_statistics().number_of_error_test_failures,endpoint(s.state().y));
 }
}
