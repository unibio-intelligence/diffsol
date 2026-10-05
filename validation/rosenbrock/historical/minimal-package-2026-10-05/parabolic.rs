use diffsol::*;
fn advance<'a, E: OdeEquationsImplicit<T=f64>+'a, S: OdeSolverMethod<'a,E>>(s:&mut S,t:f64) { s.set_stop_time(t).unwrap(); while s.step().unwrap()!=OdeSolverStopReason::TstopReached {} }
fn main() {
        for (n, lifted) in [(100, false), (100, true), (1000, false), (1000, true)] {
        let nstates = n;
        let dx = 2.0 / (nstates + 1) as f64;
        let inv_dx2 = 1.0 / (dx * dx);
        let published_errors = [5.97e-9, 4.72e-10, 3.45e-11, 2.36e-12];
        for (h, published) in [0.03125, 0.015625, 0.0078125, 0.00390625]
            .into_iter()
            .zip(published_errors)
        {
            let problem = OdeBuilder::<diffsol::FaerSparseMat<f64>>::new()
                .rtol(10.0)
                .atol(vec![10.0; nstates])
                .rhs_implicit(
                    move |u, _, t, f| {
                        let et = t.exp();
                        for i in 0..nstates {
                            let x = -1.0 + (i + 1) as f64 * dx;
                            let left = if i == 0 { if lifted { 0.0 } else { -et } } else { u[i - 1] };
                            let right = if i + 1 == nstates { if lifted { 0.0 } else { et } } else { u[i + 1] };
                            let forcing = (x.powi(3) - 6.0 * x) * et - x.powi(6) * et * et;
                            let physical = u[i] + if lifted { x*et } else { 0.0 };
                            f[i] = (left - 2.0 * u[i] + right) * inv_dx2 + physical*physical + forcing - if lifted { x*et } else { 0.0 };
                        }
                    },
                    move |u, _, t, v, jv| {
                        for i in 0..nstates {
                            let left = if i == 0 { 0.0 } else { v[i - 1] };
                            let right = if i + 1 == nstates { 0.0 } else { v[i + 1] };
                            jv[i] = (left - 2.0 * v[i] + right) * inv_dx2 + 2.0 * (u[i] + if lifted { (-1.0+(i+1) as f64*dx)*t.exp() } else { 0.0 }) * v[i];
                        }
                    },
                )
                .init(
                    move |_, _, y| {
                        for (i, yi) in y.iter_mut().enumerate() {
                            let x=-1.0 + (i + 1) as f64 * dx;
                            *yi = x.powi(3) - if lifted { x } else { 0.0 };
                        }
                    },
                    nstates,
                )
                .build()
                .unwrap();
            let mut solver = problem.rodas5p::<diffsol::FaerSparseLU<f64>>().unwrap();
            *solver.state_mut().h = h;
            solver.config_mut().minimum_timestep_growth = 1.0;
            solver.config_mut().maximum_timestep_growth = 1.0;
            advance(&mut solver, 1.0);
            let error = (0..nstates)
                .map(|i| {
                    let x = -1.0 + (i + 1) as f64 * dx;
                    (solver.state().y[i] + if lifted { x*std::f64::consts::E } else { 0.0 } - x.powi(3) * std::f64::consts::E).abs()
                })
                .fold(0.0_f64, f64::max);
            println!("{nstates},{lifted},{h},{published},{error}");
        }
    }
}
