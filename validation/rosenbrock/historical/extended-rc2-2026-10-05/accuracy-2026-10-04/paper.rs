// Standalone literature evidence; unit tests retain assertions without emitting tables.
use diffsol::*;
type Mat = NalgebraMat<f64>;
type LS = NalgebraLU<f64>;
fn advance<'a, E: OdeEquationsImplicit<T = f64> + 'a, S: OdeSolverMethod<'a, E>>(
    s: &mut S,
    t: f64,
) {
    s.set_stop_time(t).unwrap();
    while s.step().unwrap() != OdeSolverStopReason::TstopReached {}
}
fn main() {
    paper_prothero_robinson_problem_two();
    paper_table_six_fixed_step_errors_and_order();
    paper_table_eight_index_two_order_reduction();
    paper_index_one_dae_problem_one();
    paper_table_five_fixed_step_dae_errors_and_order();
    analytic_scalar_oracles();
}

// Independent scalar tableau arithmetic with analytic f_t; this is an evidence oracle,
// not the production step() path. Production fixed steps are emitted by native.rs.
fn analytic_scalar_oracles() {
    for (name, tableau, lambda, h, expected) in [
        (
            "rodas5p",
            Tableau::<f64>::rodas5p(),
            0.0,
            0.01,
            0.09983341664682904,
        ),
        (
            "rodas5p",
            Tableau::<f64>::rodas5p(),
            -1000.0,
            0.001,
            0.009999833334166675,
        ),
        (
            "rosenbrock23",
            Tableau::<f64>::rosenbrock23(),
            0.0,
            0.01,
            0.09983383262061077,
        ),
        (
            "rosenbrock23",
            Tableau::<f64>::rosenbrock23(),
            -1000.0,
            0.001,
            0.009999834105815892,
        ),
    ] {
        let row = tableau.rosenbrock().unwrap();
        let gamma = row.gamma();
        let mut t: f64 = 0.0;
        let mut y = 0.0;
        for _ in 0..10 {
            let ft = -lambda * t.cos() - t.sin();
            let mut stages = [0.0; 8];
            for i in 0..tableau.s() {
                let mut yi = y;
                let mut coupling = 0.0;
                for (j, &u) in stages.iter().enumerate().take(i) {
                    yi += tableau.a(i, j) * u;
                    coupling += row.coupling()[(i, j)] * u / h;
                }
                let ti = t + tableau.c()[i] * h;
                let f = lambda * (yi - ti.sin()) + ti.cos();
                stages[i] = gamma * h * (f + coupling + h * row.time_weights()[i] * ft)
                    / (1.0 - gamma * h * lambda);
            }
            for (i, &u) in stages.iter().enumerate().take(tableau.s()) {
                y += tableau.b()[i] * u;
            }
            t += h;
        }
        let error = (y - expected).abs();
        println!(
            "analytic_scalar_oracle,{name},{lambda},{h},{expected:.17e},{y:.17e},{error:.17e}"
        );
        assert!(error < 1e-13, "{name}, lambda={lambda}, {error}");
    }
}
fn paper_prothero_robinson_problem_two() {
    const LAMBDA: f64 = 1e5;
    let g = |t: f64| 10.0 - (10.0 + t) * (-t).exp();
    let problem = OdeBuilder::<Mat>::new()
        .rtol(1e-8)
        .atol([1e-10])
        .rhs_implicit(
            |x, _, t, f| {
                let g = 10.0 - (10.0 + t) * (-t).exp();
                let dg = (9.0 + t) * (-t).exp();
                f[0] = -LAMBDA * (x[0] - g) + dg;
            },
            |_, _, _, v, jv| jv[0] = -LAMBDA * v[0],
        )
        .init(|_, _, y| y[0] = 0.0, 1)
        .build()
        .unwrap();
    let mut solver = problem.rodas5p::<LS>().unwrap();
    advance(&mut solver, 2.0);
    let error = (solver.state().y[0] - g(2.0)).abs();
    println!(
        "paper,B3,{error:.17e},{}",
        solver.get_statistics().number_of_steps
    );
    assert!(error < 1e-7, "paper problem 2 error={error}");
}
fn paper_table_six_fixed_step_errors_and_order() {
    let published_errors = [1.26e-9, 1.47e-10, 1.78e-11, 2.17e-12];
    let mut errors = Vec::new();
    for (h, published) in [0.25, 0.125, 0.0625, 0.03125]
        .into_iter()
        .zip(published_errors)
    {
        let problem = OdeBuilder::<Mat>::new()
            .rtol(10.0)
            .atol([10.0])
            .rhs_implicit(
                |x, _, t, f| {
                    let g = 10.0 - (10.0 + t) * (-t).exp();
                    let dg = (9.0 + t) * (-t).exp();
                    f[0] = -1e5 * (x[0] - g) + dg;
                },
                |_, _, _, v, jv| jv[0] = -1e5 * v[0],
            )
            .init(|_, _, y| y[0] = 0.0, 1)
            .build()
            .unwrap();
        let mut solver = problem.rodas5p::<LS>().unwrap();
        *solver.state_mut().h = h;
        solver.config_mut().minimum_timestep_growth = 1.0;
        solver.config_mut().maximum_timestep_growth = 1.0;
        advance(&mut solver, 2.0);
        let exact = 10.0 - 12.0 * (-2.0_f64).exp();
        let error = (solver.state().y[0] - exact).abs();
        println!("paper,B5,{h},{published:.17e},{error:.17e}");
        assert!(
            (error - published).abs() < 0.2 * published,
            "h={h}, error={error}, published={published}"
        );
        errors.push(error);
    }
    for pair in errors.windows(2) {
        let order = (pair[0] / pair[1]).log2();
        assert!((order - 3.0).abs() < 0.2, "observed order={order}");
    }
}
fn paper_table_eight_index_two_order_reduction() {
    let published_errors = [9.00e-5, 2.33e-5, 5.94e-6];
    let mut errors = Vec::new();
    for (h, published) in [0.03125, 0.015625, 0.0078125]
        .into_iter()
        .zip(published_errors)
    {
        let problem = OdeBuilder::<Mat>::new()
            .t0(1.0)
            .rtol(10.0)
            .atol([10.0, 10.0])
            .rhs_implicit(
                |x, _, t, f| {
                    f[0] = x[1];
                    f[1] = x[0] * x[0] - 1.0 / (t * t);
                },
                |x, _, _, v, jv| {
                    jv[0] = v[1];
                    jv[1] = 2.0 * x[0] * v[0];
                },
            )
            .mass(|v, _, _, beta, y| {
                y[0] = v[0] + beta * y[0];
                y[1] *= beta;
            })
            .init(
                |_, _, y| {
                    y[0] = -1.0;
                    y[1] = 1.0;
                },
                2,
            )
            .build()
            .unwrap();
        let mut solver = problem
            .rodas5p_solver::<LS>(RkState::new(&problem, 5).unwrap())
            .unwrap();
        *solver.state_mut().h = h;
        solver.config_mut().minimum_timestep_growth = 1.0;
        solver.config_mut().maximum_timestep_growth = 1.0;
        advance(&mut solver, 2.0);
        let y = solver.state().y;
        let error = (y[0] + 0.5).abs().max((y[1] - 0.25).abs());
        println!("paper,B7,{h},{published:.17e},{error:.17e}");
        assert!(
            (error - published).abs() < 0.2 * published,
            "h={h}, error={error}, published={published}"
        );
        errors.push(error);
    }
    for pair in errors.windows(2) {
        let order = (pair[0] / pair[1]).log2();
        assert!((order - 2.0).abs() < 0.2, "observed order={order}");
    }
}
fn paper_index_one_dae_problem_one() {
    let problem = OdeBuilder::<Mat>::new()
        .t0(2.0)
        .rtol(1e-7)
        .atol([1e-10, 1e-10])
        .rhs_implicit(
            |x, _, t, f| {
                f[0] = x[1] / x[0];
                f[1] = x[0] / x[1] - t;
            },
            |x, _, _, v, jv| {
                jv[0] = v[1] / x[0] - x[1] * v[0] / x[0].powi(2);
                jv[1] = v[0] / x[1] - x[0] * v[1] / x[1].powi(2);
            },
        )
        .mass(|v, _, _, beta, y| {
            y[0] = v[0] + beta * y[0];
            y[1] *= beta;
        })
        .init(
            |_, _, y| {
                y[0] = 2.0_f64.ln();
                y[1] = 2.0_f64.ln() / 2.0;
            },
            2,
        )
        .build()
        .unwrap();
    let mut solver = problem.rodas5p::<LS>().unwrap();
    advance(&mut solver, 4.0);
    let y = solver.state().y;
    println!(
        "paper,B2,{:.17e},{:.17e},{}",
        (y[0] - 4.0_f64.ln()).abs(),
        (y[1] - 4.0_f64.ln() / 4.0).abs(),
        solver.get_statistics().number_of_steps
    );
    assert!((y[0] - 4.0_f64.ln()).abs() < 1e-7);
    assert!((y[1] - 4.0_f64.ln() / 4.0).abs() < 1e-7);
    assert!((y[0] / y[1] - 4.0).abs() < 1e-7);
}
fn paper_table_five_fixed_step_dae_errors_and_order() {
    let published_errors = [2.93e-8, 8.56e-10, 2.59e-11];
    let mut errors = Vec::new();
    for (h, published) in [0.125, 0.0625, 0.03125].into_iter().zip(published_errors) {
        let problem = OdeBuilder::<Mat>::new()
            .t0(2.0)
            .rtol(10.0)
            .atol([10.0, 10.0])
            .rhs_implicit(
                |x, _, t, f| {
                    f[0] = x[1] / x[0];
                    f[1] = x[0] / x[1] - t;
                },
                |x, _, _, v, jv| {
                    jv[0] = v[1] / x[0] - x[1] * v[0] / x[0].powi(2);
                    jv[1] = v[0] / x[1] - x[0] * v[1] / x[1].powi(2);
                },
            )
            .mass(|v, _, _, beta, y| {
                y[0] = v[0] + beta * y[0];
                y[1] *= beta;
            })
            .init(
                |_, _, y| {
                    y[0] = 2.0_f64.ln();
                    y[1] = 2.0_f64.ln() / 2.0;
                },
                2,
            )
            .build()
            .unwrap();
        let mut solver = problem.rodas5p::<LS>().unwrap();
        *solver.state_mut().h = h;
        solver.config_mut().minimum_timestep_growth = 1.0;
        solver.config_mut().maximum_timestep_growth = 1.0;
        advance(&mut solver, 4.0);
        let y = solver.state().y;
        let error = (y[0] - 4.0_f64.ln())
            .abs()
            .max((y[1] - 4.0_f64.ln() / 4.0).abs());
        println!("paper,B4,{h},{published:.17e},{error:.17e}");
        assert!(
            (error - published).abs() < 0.2 * published,
            "h={h}, error={error}, published={published}"
        );
        errors.push(error);
    }
    for pair in errors.windows(2) {
        let order = (pair[0] / pair[1]).log2();
        assert!((order - 5.0).abs() < 0.25, "observed order={order}");
    }
}
