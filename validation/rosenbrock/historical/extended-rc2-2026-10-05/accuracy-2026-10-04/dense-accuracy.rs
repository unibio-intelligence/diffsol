use diffsol::*;
type Mat = NalgebraMat<f64>;
type LS = NalgebraLU<f64>;

// Steinebach's polynomial example has an invertible algebraic derivative
// d(y0-y1)/dy1 = -1, so it is index-1, within the supported equation class.
fn main() {
    let times = [0.13_f64, 0.37, 0.73, 1.11, 1.41, 1.89, 2.0];
    println!("sampling,maximum_step,accepted_steps,maximum_sample_error");
    for endpoints in [false, true] {
        for maximum in [2.0, 1.0, 0.5, 0.25, 0.125, 0.0625] {
            let problem = OdeBuilder::<Mat>::new()
                .rtol(10.0)
                .atol([10.0, 10.0])
                .rhs_implicit(
                    |y, _, t, f| {
                        f[0] = 5.0 * t.powi(4);
                        f[1] = y[0] - y[1];
                    },
                    |_, _, _, v, j| {
                        j[0] = 0.0;
                        j[1] = v[0] - v[1];
                    },
                )
                .mass(|v, _, _, beta, y| {
                    y[0] = v[0] + beta * y[0];
                    y[1] *= beta;
                })
                .init(|_, _, y| y.fill(0.0), 2)
                .build()
                .unwrap();
            let mut solver = problem.rodas5p::<LS>().unwrap();
            *solver.state_mut().h = maximum;
            solver.set_maximum_step(maximum).unwrap();
            if !endpoints {
                solver.set_stop_time(2.0).unwrap();
            }
            let mut error = 0.0_f64;
            for t in times {
                if endpoints {
                    solver.set_stop_time(t).unwrap();
                    while solver.step().unwrap() != OdeSolverStopReason::TstopReached {}
                } else {
                    while solver.state().t < t {
                        solver.step().unwrap();
                    }
                }
                let y = solver.interpolate(t).unwrap();
                for i in 0..2 {
                    error = error.max((y[i] - t.powi(5)).abs());
                }
            }
            let mode = if endpoints {
                "observation_endpoints"
            } else {
                "dense"
            };
            println!(
                "{mode},{maximum},{},{error:.17e}",
                solver.get_statistics().number_of_steps
            );
            if endpoints {
                assert!(error < 1e-10, "endpoint sampling error: {error}");
            }
        }
    }
}
