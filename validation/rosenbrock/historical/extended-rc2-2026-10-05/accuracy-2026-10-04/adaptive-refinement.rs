use diffsol::*;
type Mat = NalgebraMat<f64>;
type LS = NalgebraLU<f64>;
fn problem(
    kind: &'static str,
    rtol: f64,
) -> OdeSolverProblem<
    impl OdeEquationsImplicit<T = f64, M = Mat, V = NalgebraVec<f64>, C = NalgebraContext>,
> {
    let n = match kind {
        "robertson" => 3,
        "vdp" => 2,
        _ => 1,
    };
    OdeBuilder::<Mat>::new()
        .rtol(rtol)
        .atol(vec![rtol * 0.01; n])
        .rhs_implicit(
            move |u, _, t, f| match kind {
                "decay" => f[0] = -u[0],
                "robertson" => {
                    f[0] = -0.04 * u[0] + 1e4 * u[1] * u[2];
                    f[2] = 3e7 * u[1] * u[1];
                    f[1] = -f[0] - f[2];
                }
                "vdp" => {
                    f[0] = u[1];
                    f[1] = 1000.0 * (1.0 - u[0] * u[0]) * u[1] - u[0];
                }
                "cosine" => f[0] = t.cos(),
                "pr" => f[0] = -1000.0 * (u[0] - t.sin()) + t.cos(),
                _ => unreachable!(),
            },
            move |u, _, _, v, j| match kind {
                "decay" => j[0] = -v[0],
                "robertson" => {
                    j[0] = -0.04 * v[0] + 1e4 * u[2] * v[1] + 1e4 * u[1] * v[2];
                    j[2] = 6e7 * u[1] * v[1];
                    j[1] = -j[0] - j[2];
                }
                "vdp" => {
                    j[0] = v[1];
                    j[1] =
                        (-2000.0 * u[0] * u[1] - 1.0) * v[0] + 1000.0 * (1.0 - u[0] * u[0]) * v[1];
                }
                "cosine" => j[0] = 0.0,
                "pr" => j[0] = -1000.0 * v[0],
                _ => unreachable!(),
            },
        )
        .init(
            move |_, _, y| {
                y.fill(0.0);
                y[0] = match kind {
                    "decay" | "robertson" => 1.0,
                    "vdp" => 2.0,
                    _ => 0.0,
                };
            },
            n,
        )
        .build()
        .unwrap()
}
fn advance<'a, E: OdeEquationsImplicit<T = f64> + 'a, S: OdeSolverMethod<'a, E>>(
    s: &mut S,
    t: f64,
) {
    s.set_stop_time(t).unwrap();
    while s.step().unwrap() != OdeSolverStopReason::TstopReached {}
}
fn endpoint(v: &NalgebraVec<f64>) -> String {
    (0..v.len())
        .map(|i| format!("{:.17e}", v[i]))
        .collect::<Vec<_>>()
        .join(";")
}

fn trajectory<
    'a,
    E: OdeEquationsImplicit<T = f64, V = NalgebraVec<f64>> + 'a,
    S: OdeSolverMethod<'a, E>,
>(
    s: &mut S,
    times: &[f64],
) -> Vec<Vec<f64>> {
    s.set_stop_time(*times.last().unwrap()).unwrap();
    times
        .iter()
        .map(|&t| {
            while s.state().t < t {
                s.step().unwrap();
            }
            let y = s.interpolate(t).unwrap();
            (0..y.len()).map(|i| y[i]).collect()
        })
        .collect()
}
fn norm(a: &[Vec<f64>], b: &[Vec<f64>], rtol: f64) -> f64 {
    a.iter()
        .zip(b)
        .flat_map(|(x, y)| x.iter().zip(y))
        .map(|(x, y)| (x - y).abs() / (rtol * 0.01 + rtol * x.abs().max(y.abs())))
        .fold(0.0, f64::max)
}
fn main() {
    println!("method,problem,requested_rtol,passes,final_local_rtol,initial_steps,total_steps,final_steps,initial_reference_units,final_reference_units,convergence_units,endpoint_absolute_error,seconds");
    for kind in ["decay", "robertson", "vdp", "cosine", "pr"] {
        let end = match kind {
            "robertson" => 1e4,
            "cosine" => 10.0,
            _ => 1.0,
        };
        let times = if kind == "robertson" {
            vec![
                0.01, 0.02, 0.04, 0.1, 0.4, 1.0, 4.0, 10.0, 40.0, 100.0, 1000.0, 10000.0,
            ]
        } else {
            (1..=20).map(|i| end * i as f64 / 20.0).collect()
        };
        let reference = if ["decay", "cosine", "pr"].contains(&kind) {
            times
                .iter()
                .map(|t| vec![if kind == "decay" { (-t).exp() } else { t.sin() }])
                .collect::<Vec<_>>()
        } else {
            let p = problem(kind, 1e-14);
            let mut s = p.bdf::<LS>().unwrap();
            s.config_mut().minimum_timestep = 0.0;
            trajectory(&mut s, &times)
        };
        for (name, tableau) in [
            ("rodas5p", Tableau::rodas5p()),
            ("rosenbrock23", Tableau::rosenbrock23()),
        ] {
            for target in [1e-6, 1e-9] {
                let clock = std::time::Instant::now();
                let mut total = 0;
                let mut initial_steps = 0;
                let mut initial_error = 0.0;
                let mut final_steps = 0;
                let result = refine_sampled_trajectory(
                    target,
                    &[target * 0.01],
                    8,
                    0.25,
                    |factor| {
                        let p = problem(kind, target * factor);
                        let mut solver =
                            p.rosenbrock_solver::<LS, Mat>(p.rodas5p_state::<LS>()?, tableau)?;
                        solver.config_mut().minimum_timestep = 0.0;
                        let current = trajectory(&mut solver, &times);
                        final_steps = solver.get_statistics().number_of_steps;
                        total += final_steps;
                        if factor == 1.0 {
                            initial_steps = final_steps;
                            initial_error = norm(&current, &reference, target);
                        }
                        Ok::<_, DiffsolError>(current)
                    },
                    |e| e,
                )
                .unwrap();
                let error = norm(&result.samples, &reference, target);
                let absolute = result
                    .samples
                    .last()
                    .unwrap()
                    .iter()
                    .zip(reference.last().unwrap())
                    .map(|(a, b)| (a - b).abs())
                    .fold(0.0, f64::max);
                println!("{name},{kind},{target},{},{},{initial_steps},{total},{final_steps},{initial_error:.17e},{error:.17e},{:.17e},{absolute:.17e},{}", result.passes,target * result.local_tolerance_factor, result.maximum_scaled_change,clock.elapsed().as_secs_f64());
                assert!(
                    error < 1.0,
                    "independent-reference error: {name} {kind} {target} {error}"
                );
            }
        }
    }
}
