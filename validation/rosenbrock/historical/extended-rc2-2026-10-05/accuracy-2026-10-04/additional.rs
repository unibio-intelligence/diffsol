// Independently authored scientific equations; provenance in additional/README.md.
use diffsol::*;
mod source_gaps;
type Mat = NalgebraMat<f64>;
type LS = NalgebraLU<f64>;
const K: [f64; 25] = [
    0.35, 26.6, 12300., 0.00086, 0.00082, 15000., 0.00013, 24000., 16500., 9000., 0.022, 12000.,
    1.88, 16300., 4.8e6, 0.00035, 0.0175, 1e8, 4.44e11, 1240., 2.1, 5.78, 0.0474, 1780., 3.12,
];
const PAIRS: [(usize, usize); 25] = [
    (0, 20),
    (1, 3),
    (4, 1),
    (6, 20),
    (6, 20),
    (6, 5),
    (8, 20),
    (8, 5),
    (10, 1),
    (10, 0),
    (12, 20),
    (9, 1),
    (13, 20),
    (0, 5),
    (2, 20),
    (3, 20),
    (3, 20),
    (15, 20),
    (15, 20),
    (16, 5),
    (18, 20),
    (18, 20),
    (0, 3),
    (18, 0),
    (19, 20),
];
fn chemistry(r: &[f64], f: &mut [f64]) {
    f.copy_from_slice(&[
        -r[0] - r[9] - r[13] - r[22] - r[23] + r[1] + r[2] + r[8] + r[10] + r[11] + r[21] + r[24],
        -r[1] - r[2] - r[8] - r[11] + r[0] + r[20],
        -r[14] + r[0] + r[16] + r[18] + r[21],
        -r[1] - r[15] - r[16] - r[22] + r[14],
        -r[2] + 2. * r[3] + r[5] + r[6] + r[12] + r[19],
        -r[5] - r[7] - r[13] - r[19] + r[2] + 2. * r[17],
        -r[3] - r[4] - r[5] + r[12],
        r[3] + r[4] + r[5] + r[6],
        -r[6] - r[7],
        -r[11] + r[6] + r[8],
        -r[8] - r[9] + r[7] + r[10],
        r[8],
        -r[10] + r[9],
        -r[12] + r[11],
        r[13],
        -r[17] - r[18] + r[15],
        -r[19],
        r[19],
        -r[20] - r[21] - r[23] + r[22] + r[24],
        -r[24] + r[23],
    ]);
}
fn consumer(t: f64) -> f64 {
    (1..=10).map(|i|if i%2==1 {1.}else{-1.} * 25.*(((t-3600.*i as f64)*3.8002/60.).tanh()+1.)).sum()
}
fn rhs(kind: &str, y: &[f64], t: f64, f: &mut [f64]) {
    match kind {
        "hyperbolic" | "water_index1" => source_gaps::rhs(kind, y, t, f),
        "hires" => {
            let r = 280. * y[5] * y[7];
            f.copy_from_slice(&[
                -1.71 * y[0] + 0.43 * y[1] + 8.32 * y[2] + 0.0007,
                1.71 * y[0] - 8.75 * y[1],
                -10.03 * y[2] + 0.43 * y[3] + 0.035 * y[4],
                8.32 * y[1] + 1.71 * y[2] - 1.12 * y[3],
                -1.745 * y[4] + 0.43 * y[5] + 0.43 * y[6],
                -r + 0.69 * y[3] + 1.71 * y[4] - 0.43 * y[5] + 0.69 * y[6],
                r - 1.81 * y[6],
                -r + 1.81 * y[6],
            ]);
        }
        "oregonator" => f.copy_from_slice(&[
            77.27 * (y[1] + y[0] * (1. - 8.375e-6 * y[0] - y[1])),
            (y[2] - (1. + y[0]) * y[1]) / 77.27,
            0.161 * (y[0] - y[2]),
        ]),
        "amplifier" => {
            let a = 1e-6 * ((y[3] - y[2]) / 0.026).exp_m1();
            let b = 1e-6 * ((y[6] - y[5]) / 0.026).exp_m1();
            f.copy_from_slice(&[
                y[0] / 9000.,
                (y[1] - 6.) / 9000. + 0.99 * a,
                y[2] / 9000. - a,
                (2. * y[3] - 6.) / 9000. + 0.01 * a,
                (y[4] - 6.) / 9000. + 0.99 * b,
                y[5] / 9000. - b,
                (2. * y[6] - 6.) / 9000. + 0.01 * b,
                (y[7] - 0.1 * (200. * std::f64::consts::PI * t).sin()) / 1000.,
            ]);
        }
        "pendulum" => f.copy_from_slice(&[
            y[1],
            y[4] * y[0],
            y[3],
            y[4] * y[2] - 9.81,
            y[1] * y[1] + y[3] * y[3] + y[4] * (y[0] * y[0] + y[2] * y[2]) - 9.81 * y[2],
        ]),
        "circle" | "circle_inexact" => f.copy_from_slice(&[y[1], y[0] * y[0] + y[1] * y[1] - 1.]),
        "pollution" => {
            let mut r = [0.; 25];
            for (i, &(a, b)) in PAIRS.iter().enumerate() {
                r[i] = K[i] * y[a] * if b == 20 { 1. } else { y[b] };
            }
            chemistry(&r, f);
        }
        "pv" => {
            let v = y[1] - y[0];
            let s = y[6] / 36000.;
            let oc = ((6.8072 * s - 10.5555) * s + 6.2199) * s + 10.2668;
            f.copy_from_slice(&[
                y[0],
                y[4] + y[3] - y[2],
                consumer(t) - y[2] * v,
                -3.1037
                    + 1.0015 * y[3]
                    + 0.0032 * v
                    + 1.3984e-9 * (0.4303 * y[3] + 1.5 * 0.9562 * v).exp_m1(),
                v - oc + y[5] + 0.2 * y[4],
                y[4] / 4000. - y[5] / 2000.,
                -y[4],
            ]);
        }
        _ => unreachable!(),
    }
}
fn jvp(kind: &str, y: &[f64], v: &[f64], f: &mut [f64]) {
    match kind {
        "hyperbolic" | "water_index1" => source_gaps::jvp(kind, y, v, f),
        "hires" => {
            rhs(kind, v, 0., f);
            f[0] -= 0.0007;
            let r = 280. * (v[5] * y[7] + y[5] * v[7]);
            let wrong = 280. * v[5] * v[7];
            f[5] += wrong - r;
            f[6] += r - wrong;
            f[7] += wrong - r;
        }
        "oregonator" => f.copy_from_slice(&[
            77.27 * ((1. - 2. * 8.375e-6 * y[0] - y[1]) * v[0] + (1. - y[0]) * v[1]),
            (-y[1] * v[0] - (1. + y[0]) * v[1] + v[2]) / 77.27,
            0.161 * (v[0] - v[2]),
        ]),
        "amplifier" => {
            let a = 1e-6 * ((y[3] - y[2]) / 0.026).exp() / 0.026 * (v[3] - v[2]);
            let b = 1e-6 * ((y[6] - y[5]) / 0.026).exp() / 0.026 * (v[6] - v[5]);
            f.copy_from_slice(&[
                v[0] / 9000.,
                v[1] / 9000. + 0.99 * a,
                v[2] / 9000. - a,
                2. * v[3] / 9000. + 0.01 * a,
                v[4] / 9000. + 0.99 * b,
                v[5] / 9000. - b,
                2. * v[6] / 9000. + 0.01 * b,
                v[7] / 1000.,
            ]);
        }
        "pendulum" => f.copy_from_slice(&[
            v[1],
            v[4] * y[0] + y[4] * v[0],
            v[3],
            v[4] * y[2] + y[4] * v[2],
            2. * y[1] * v[1]
                + 2. * y[3] * v[3]
                + v[4] * (y[0] * y[0] + y[2] * y[2])
                + 2. * y[4] * (y[0] * v[0] + y[2] * v[2])
                - 9.81 * v[2],
        ]),
        "circle" => f.copy_from_slice(&[v[1], 2. * y[0] * v[0] + 2. * y[1] * v[1]]),
        "circle_inexact" => f.copy_from_slice(&[0., 2. * y[1] * v[1]]),
        "pollution" => {
            let mut r = [0.; 25];
            for (i, &(a, b)) in PAIRS.iter().enumerate() {
                r[i] = K[i]
                    * if b == 20 {
                        v[a]
                    } else {
                        v[a] * y[b] + y[a] * v[b]
                    };
            }
            chemistry(&r, f);
        }
        "pv" => {
            let z = y[1] - y[0];
            let w = v[1] - v[0];
            let s = y[6] / 36000.;
            let doc = (3. * 6.8072 * s * s - 2. * 10.5555 * s + 6.2199) / 36000.;
            f.copy_from_slice(&[
                v[0],
                v[4] + v[3] - v[2],
                -v[2] * z - y[2] * w,
                1.0015 * v[3]
                    + 0.0032 * w
                    + 1.3984e-9
                        * (0.4303 * y[3] + 1.5 * 0.9562 * z).exp()
                        * (0.4303 * v[3] + 1.5 * 0.9562 * w),
                w - doc * v[6] + v[5] + 0.2 * v[4],
                v[4] / 4000. - v[5] / 2000.,
                -v[4],
            ]);
        }
        _ => unreachable!(),
    }
}
fn initial(kind: &str) -> Vec<f64> {
    match kind {
        "hyperbolic" | "water_index1" => source_gaps::initial(kind),
        "hires" => vec![1., 0., 0., 0., 0., 0., 0., 0.0057],
        "oregonator" => vec![1., 2., 3.],
        "amplifier" => vec![0., 6., 3., 3., 6., 3., 3., 0.],
        "pendulum" => vec![2., 0., 0., 0., 0.],
        "circle" | "circle_inexact" => vec![0., 1.],
        "pollution" => vec![
            0., 0.2, 0., 0.04, 0., 0., 0.1, 0.3, 0.01, 0., 0., 0., 0., 0., 0., 0., 0.007, 0., 0.,
            0.,
        ],
        "pv" => vec![
            0.,
            11.856598910310167,
            0.,
            2.9409008015416687,
            -2.940900801550821,
            0.,
            9000.,
        ],
        _ => unreachable!(),
    }
}
fn mass(kind: &str, x: &[f64], beta: f64, y: &mut [f64]) {
    let mut z = vec![0.; x.len()];
    match kind {
        "water_index1" => {
            for i in (0..18).chain(36..38) {
                z[i] = x[i];
            }
        }
        "amplifier" => {
            for (a, c) in [(0, 5e-6), (3, 3e-6), (6, 1e-6)] {
                z[a] = c * (x[a + 1] - x[a]);
                z[a + 1] = -z[a];
            }
            z[2] = -4e-6 * x[2];
            z[5] = -2e-6 * x[5];
        }
        "pendulum" => {
            for i in 0..4 {
                z[i] = x[i];
            }
        }
        "circle" | "circle_inexact" => z[0] = x[0],
        "pv" => {
            z[5] = x[5];
            z[6] = x[6];
        }
        _ => {
            for i in 0..x.len() {
                z[i] = x[i];
            }
        }
    }
    for i in 0..x.len() {
        y[i] = z[i] + beta * y[i];
    }
}
// Supply exact f_t through the public trait hook for these forced examples.
struct ScientificEqn {
    kind: &'static str,
    ctx: NalgebraContext,
}
impl Op for ScientificEqn {
    type T = f64;
    type V = NalgebraVec<f64>;
    type M = Mat;
    type C = NalgebraContext;
    fn context(&self) -> &Self::C {
        &self.ctx
    }
    fn nstates(&self) -> usize {
        initial(self.kind).len()
    }
    fn nout(&self) -> usize {
        self.nstates()
    }
    fn nparams(&self) -> usize {
        0
    }
}
impl NonLinearOp for ScientificEqn {
    fn call_inplace(&self, x: &Self::V, t: f64, y: &mut Self::V) {
        let x = (0..x.len()).map(|i| x[i]).collect::<Vec<_>>();
        let mut z = vec![0.; x.len()];
        rhs(self.kind, &x, t, &mut z);
        for i in 0..z.len() {
            y[i] = z[i];
        }
    }
    fn time_partial_inplace(&self, _: &Self::V, t: f64, y: &mut Self::V) -> bool {
        y.fill(0.);
        if ["hyperbolic", "water_index1"].contains(&self.kind) {
            let mut f = vec![0.; y.len()];
            source_gaps::time_partial(self.kind, t, &mut f);
            for i in 0..f.len() {
                y[i] = f[i];
            }
        }
        if self.kind == "amplifier" {
            y[7] = -0.1 * 200. * std::f64::consts::PI * (200. * std::f64::consts::PI * t).cos()
                / 1000.;
        }
        if self.kind == "pv" {
            y[2]=(1..=10).map(|i|if i%2==1{1.}else{-1.}*25.*3.8002/60.*(1.-((t-3600.*i as f64)*3.8002/60.).tanh().powi(2))).sum();
        }
        true
    }
}
impl NonLinearOpJacobian for ScientificEqn {
    fn jac_mul_inplace(&self, x: &Self::V, _: f64, v: &Self::V, y: &mut Self::V) {
        let x = (0..x.len()).map(|i| x[i]).collect::<Vec<_>>();
        let v = (0..v.len()).map(|i| v[i]).collect::<Vec<_>>();
        let mut z = vec![0.; x.len()];
        jvp(self.kind, &x, &v, &mut z);
        for i in 0..z.len() {
            y[i] = z[i];
        }
    }
}
impl LinearOp for ScientificEqn {
    fn gemv_inplace(&self, x: &Self::V, _: f64, beta: f64, y: &mut Self::V) {
        let x = (0..x.len()).map(|i| x[i]).collect::<Vec<_>>();
        let mut z = (0..y.len()).map(|i| y[i]).collect::<Vec<_>>();
        mass(self.kind, &x, beta, &mut z);
        for i in 0..z.len() {
            y[i] = z[i];
        }
    }
}
impl ConstantOp for ScientificEqn {
    fn call_inplace(&self, _: f64, y: &mut Self::V) {
        for (i, v) in initial(self.kind).iter().enumerate() {
            y[i] = *v;
        }
    }
}
impl<'a> OdeEquationsRef<'a> for ScientificEqn {
    type Rhs = &'a Self;
    type Mass = &'a Self;
    type Init = &'a Self;
    type Root = &'a Self;
    type Out = &'a Self;
    type Reset = &'a Self;
}
impl OdeEquations for ScientificEqn {
    fn rhs(&self) -> &Self {
        self
    }
    fn mass(&self) -> Option<&Self> {
        if ["hires", "oregonator", "pollution", "hyperbolic"].contains(&self.kind) {
            None
        } else {
            Some(self)
        }
    }
    fn init(&self) -> &Self {
        self
    }
    fn set_params(&mut self, _: &Self::V) {}
    fn get_params(&self, _: &mut Self::V) {}
}
fn problem(kind: &'static str, tol: f64) -> OdeSolverProblem<ScientificEqn> {
    let n = initial(kind).len();
    let p = OdeBuilder::<Mat>::new()
        .rtol(tol)
        .atol(vec![tol * 0.01; n])
        .rhs_implicit(|_, _, _, f| f.fill(0.), |_, _, _, _, f| f.fill(0.))
        .init(|_, _, f| f.fill(0.), n)
        .build()
        .unwrap();
    OdeSolverProblem {
        eqn: ScientificEqn {
            kind,
            ctx: p.eqn.context().clone(),
        },
        rtol: p.rtol,
        atol: p.atol,
        t0: p.t0,
        h0: p.h0,
        integrate_out: p.integrate_out,
        sens_rtol: p.sens_rtol,
        sens_atol: p.sens_atol,
        out_rtol: p.out_rtol,
        out_atol: p.out_atol,
        param_rtol: p.param_rtol,
        param_atol: p.param_atol,
        ic_options: p.ic_options,
        ode_options: p.ode_options,
    }
}

fn times(kind: &str) -> Vec<f64> {
    let end = match kind {
        "hires" => 321.8122,
        "oregonator" => 360.,
        "amplifier" => 0.05,
        "pendulum" => 10.,
        "pollution" => 60.,
        "pv" => 36000.,
        "water_index1" => 61200.,
        _ => 1.,
    };
    if kind == "pv" {
        let mut ts = vec![1., 100., 1000., 36000.];
        for i in 1..=10 {
            for delta in [-60., -10., 0., 10., 60.] {
                let t = 3600. * i as f64 + delta;
                if t <= end {
                    ts.push(t);
                }
            }
        }
        ts.sort_by(f64::total_cmp);
        ts.dedup();
        ts
    } else {
        let start = if kind == "amplifier" { 0.0005 } else { 0.001 };
        let intervals = if kind == "amplifier" { 99 } else { 100 };
        (0..=intervals)
            .map(|i| start + (end - start) * i as f64 / intervals as f64)
            .collect()
    }
}
fn circle_orders() {
    println!("jacobian,h,error,order");
    for kind in ["circle", "circle_inexact"] {
        let mut previous = 0.;
        for n in [8, 16, 32, 64, 128] {
            let h = 1. / n as f64;
            let p = problem(kind, 10.);
            let mut solver = p.rodas5p::<LS>().unwrap();
            solver.config_mut().minimum_timestep = 0.;
            solver.config_mut().maximum_timestep_growth = 1.;
            solver.config_mut().minimum_timestep_growth = 1.;
            *solver.state_mut().h = h;
            solver.set_stop_time(1.).unwrap();
            while solver.step().unwrap() != OdeSolverStopReason::TstopReached {}
            let y = solver.state().y;
            let err = (y[0] - 1f64.sin()).abs().max((y[1] - 1f64.cos()).abs());
            let order = if previous == 0. {
                0.
            } else {
                (previous / err).log2()
            };
            println!("{kind},{h},{err:.17e},{order}");
            previous = err;
        }
    }
}
fn main() {
    let observation_endpoints = std::env::var_os("VALIDATION_OBSERVATION_ENDPOINTS").is_some();
    let consistent_samples = std::env::var_os("VALIDATION_CONSISTENT_SAMPLES").is_some();
    assert!(!(observation_endpoints && consistent_samples),
        "Choose observation endpoints or consistent interpolation separately");
    let sample_prefix = if observation_endpoints { "endpoint-" }
        else if consistent_samples { "consistent-" } else { "" };
    if std::env::var_os("VALIDATION_SOURCE_DERIVATIVES").is_some() {
        source_gaps::verify_derivatives();
        return;
    }
    if std::env::var_os("VALIDATION_CIRCLE_ORDERS").is_some() {
        circle_orders();
        return;
    }
    println!("problem,method,rtol,mode,passes,total_steps,convergence,time,values");
    let kinds: &[&str] = if std::env::var_os("VALIDATION_SOURCE_GAPS").is_some() {
        &["hyperbolic", "water_index1"]
    } else {
        &[
            "hires",
            "oregonator",
            "amplifier",
            "pendulum",
            "circle",
            "circle_inexact",
            "pollution",
            "pv",
        ]
    };
    for &kind in kinds {
        for (name, tableau) in [
            ("rodas5p", Tableau::rodas5p()),
            ("rosenbrock23", Tableau::rosenbrock23()),
        ] {
            for target in [1e-6, 1e-8] {
                if std::env::var("VALIDATION_METHOD").is_ok_and(|v| v != name)
                    || std::env::var("VALIDATION_TARGET")
                        .is_ok_and(|v| v.parse::<f64>().unwrap() != target)
                {
                    continue;
                }
                if std::env::var("VALIDATION_PROBLEM").is_ok_and(|v| v != kind) {
                    continue;
                }
                let ts = times(kind);
                let mut total = 0;
                let result = refine_sampled_trajectory(
                    target,
                    &[target * 0.01],
                    std::env::var("VALIDATION_PASSES").map_or(8, |v| v.parse::<usize>().unwrap()),
                    0.25,
                    |factor| {
                        let p = problem(kind, target * factor);
                        let mut s =
                            p.rosenbrock_solver::<LS, Mat>(p.rodas5p_state::<LS>()?, tableau)?;
                        s.config_mut().minimum_timestep = 0.;
                        if kind == "water_index1" {
                            s.set_maximum_step(120. * factor.sqrt())?;
                        }
                        if kind == "pv" {
                            s.set_maximum_step(60. * factor.sqrt())?;
                        }
                        s.set_stop_time(*ts.last().unwrap())?;
                        let mut ys = Vec::new();
                        for &t in &ts {
                            if observation_endpoints {
                                s.set_stop_time(t)?;
                            }
                            while s.state().t < t {
                                if let Err(e)=s.step() {
                                    if std::env::var_os("VALIDATION_TRACE").is_some() {
                                        eprintln!("TRACE_FAILED_STATE,{kind},{name},{target},{factor},{},{},{}",s.state().t,s.state().h,(0..s.state().y.len()).map(|i|format!("{:0.17e}",s.state().y[i])).collect::<Vec<_>>().join(";"));
                                    }
                                    return Err(e);
                                }
                                if std::env::var_os("VALIDATION_TRACE").is_some()
                                    && s.get_statistics().number_of_steps % 50000 == 0 {
                                    eprintln!("TRACE_STEP,{kind},{name},{target},{factor},{},{},{},{}",s.state().t,s.state().h,s.get_statistics().number_of_steps,s.get_statistics().number_of_error_test_failures);
                                }
                                if s.get_statistics().number_of_steps
                                    > std::env::var("VALIDATION_STEP_CAP")
                                        .map_or(5_000_000, |v| v.parse::<usize>().unwrap())
                                {
                                    return Err(DiffsolError::Other("validation step cap".into()));
                                }
                            }
                            let y = if consistent_samples {
                                s.interpolate_consistent(t,8)?
                            }else{s.interpolate(t)?};
                            if std::env::var_os("VALIDATION_ENDPOINT_TRACE").is_some() {
                                eprintln!("TRACE_ENDPOINT,{kind},{name},{target},{factor},{},{t},{}",s.state().t,(0..s.state().y.len()).map(|i|format!("{:0.17e}",s.state().y[i])).collect::<Vec<_>>().join(";"));
                            }
                            ys.push((0..y.len()).map(|i| y[i]).collect::<Vec<_>>());
                        }
                        total += s.get_statistics().number_of_steps;
                        if factor == 1. || kind == "pv" || std::env::var_os("VALIDATION_RECORD_PASSES").is_some() {
                            let mode = if factor == 1. {
                                format!("{sample_prefix}local")
                            } else {
                                format!("{sample_prefix}pass{factor}")
                            };
                            for (&t, y) in ts.iter().zip(&ys) {
                                println!(
                                    "{kind},{name},{target},{mode},1,{total},NaN,{t},{}",
                                    y.iter()
                                        .map(|x| format!("{x:0.17e}"))
                                        .collect::<Vec<_>>()
                                        .join(";")
                                );
                            }
                        }
                        Ok(ys)
                    },
                    |e| e,
                );
                match result {
                    Ok(r) => {
                        for (&t, y) in ts.iter().zip(&r.samples) {
                            println!(
                                "{kind},{name},{target},{},{},{total},{},{t},{}",
                                format!("{sample_prefix}sampled"),
                                r.passes,
                                r.maximum_scaled_change,
                                y.iter()
                                    .map(|x| format!("{x:0.17e}"))
                                    .collect::<Vec<_>>()
                                    .join(";")
                            );
                        }
                        eprintln!("{kind} {name} {target} passes={} steps={total}", r.passes);
                    }
                    Err(e) => eprintln!("FAILED {kind} {name} {target}: {e}"),
                }
            }
        }
    }
}
