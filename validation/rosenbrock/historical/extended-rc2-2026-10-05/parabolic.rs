use diffsol::*;
fn advance<'a, E: OdeEquationsImplicit<T=f64>+'a, S: OdeSolverMethod<'a,E>>(s:&mut S,t:f64) { s.set_stop_time(t).unwrap(); while s.step().unwrap()!=OdeSolverStopReason::TstopReached {} }
fn main() {
        const N: usize = 1000;
        let dx = 2.0 / (N + 1) as f64;
        let inv_dx2 = 1.0 / (dx * dx);
        let published_errors = [5.97e-9, 4.72e-10, 3.45e-11, 2.36e-12];
        for (h, published) in [0.03125, 0.015625, 0.0078125, 0.00390625]
            .into_iter()
            .zip(published_errors)
        {
            let problem = OdeBuilder::<diffsol::FaerSparseMat<f64>>::new()
                .rtol(10.0)
                .atol(vec![10.0; N])
                .rhs_implicit(
                    move |u, _, t, f| {
                        let et = t.exp();
                        for i in 0..N {
                            let x = -1.0 + (i + 1) as f64 * dx;
                            let left = if i == 0 { -et } else { u[i - 1] };
                            let right = if i + 1 == N { et } else { u[i + 1] };
                            let forcing = (x.powi(3) - 6.0 * x) * et - x.powi(6) * et * et;
                            f[i] = (left - 2.0 * u[i] + right) * inv_dx2 + u[i] * u[i] + forcing;
                        }
                    },
                    move |u, _, _, v, jv| {
                        for i in 0..N {
                            let left = if i == 0 { 0.0 } else { v[i - 1] };
                            let right = if i + 1 == N { 0.0 } else { v[i + 1] };
                            jv[i] = (left - 2.0 * v[i] + right) * inv_dx2 + 2.0 * u[i] * v[i];
                        }
                    },
                )
                .init(
                    move |_, _, y| {
                        for (i, yi) in y.iter_mut().enumerate() {
                            *yi = (-1.0 + (i + 1) as f64 * dx).powi(3);
                        }
                    },
                    N,
                )
                .build()
                .unwrap();
            let mut solver = problem.rodas5p::<diffsol::FaerSparseLU<f64>>().unwrap();
            *solver.state_mut().h = h;
            solver.config_mut().minimum_timestep_growth = 1.0;
            solver.config_mut().maximum_timestep_growth = 1.0;
            advance(&mut solver, 1.0);
            let error = (0..N)
                .map(|i| {
                    let x = -1.0 + (i + 1) as f64 * dx;
                    (solver.state().y[i] - x.powi(3) * std::f64::consts::E).abs()
                })
                .fold(0.0_f64, f64::max);
            println!("{h},{published},{error}");
        }
    }