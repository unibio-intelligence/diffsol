use diffsol::*;
type Mat=NalgebraMat<f64>;type LS=NalgebraLU<f64>;
fn main(){
 for z in [-1.0,-10.0,-1000.0,-1e8,-1e14]{
  let p=OdeBuilder::<Mat>::new().rtol(1e30).atol([1e30])
   .rhs_implicit(move |x,_,_,f|f[0]=z*x[0],move |_,_,_,v,jv|jv[0]=z*v[0])
   .init(|_,_,y|y[0]=1.0,1).build().unwrap();
  let mut s=p.rodas5p::<LS>().unwrap();*s.state_mut().h=1.0;s.step().unwrap();
  println!("stability,{z},{:.17e}",s.state().y[0]);
 }
 for degree in 1i32..=5{
  let p=OdeBuilder::<Mat>::new().rtol(10.0).atol([10.0,10.0])
   .rhs_implicit(move |x,_,t,f|{f[0]=f64::from(degree)*t.powi(degree-1);f[1]=x[0]-x[1];},|_,_,_,v,jv|{jv[0]=0.0;jv[1]=v[0]-v[1];})
   .mass(|v,_,_,b,y|{y[0]=v[0]+b*y[0];y[1]*=b;}).init(|_,_,y|y.fill(0.0),2).build().unwrap();
  let mut s=p.rodas5p::<LS>().unwrap();*s.state_mut().h=2.0;s.step().unwrap();
  let mut dense=0.0f64;let mut endpoint=0.0f64;let mut derivative=0.0f64;
  for t in [0.25f64,0.5,1.0,1.5,1.75]{let y=s.interpolate(t).unwrap();for i in 0..2{dense=dense.max((y[i]-t.powi(degree)).abs());}}
  for i in 0..2 {endpoint=endpoint.max((s.state().y[i]-2.0f64.powi(degree)).abs());derivative=derivative.max((s.state().dy[i]-f64::from(degree)*2.0f64.powi(degree-1)).abs());}
  println!("polynomial,{degree},{dense:.17e},{endpoint:.17e},{derivative:.17e}");
 }
}
