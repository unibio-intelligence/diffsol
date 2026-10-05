// Independently authored equations. Source and index reduction: source-gaps/README.md.
pub const PIPES: [(usize, usize); 18] = [
    (0, 1),
    (1, 2),
    (1, 5),
    (2, 3),
    (2, 4),
    (3, 4),
    (4, 9),
    (5, 4),
    (6, 3),
    (6, 7),
    (7, 4),
    (7, 9),
    (8, 7),
    (10, 8),
    (10, 11),
    (11, 6),
    (11, 7),
    (12, 10),
];
const NODES: [usize; 13] = [4, 7, 0, 1, 2, 3, 5, 6, 8, 9, 10, 11, 12];
const QS: f64 = 0.001;
const LS: f64 = 0.05;
const PS: f64 = 1000.;
fn pressure(y: &[f64], node: usize) -> f64 {
    PS * y[36 + NODES.iter().position(|&n| n == node).unwrap()]
}
pub fn boundary(t: f64, order: usize) -> [f64; 13] {
    let h = t / 3600.;
    let e = (-h).exp();
    let a = e - 1.;
    let (input, output) = match order {
        0 => (1. - a.cos(), h * h * (3. * h * h - 92. * h + 720.) / 1e6),
        1 => (
            -a.sin() * e / 3600.,
            (12. * h * h * h - 276. * h * h + 1440. * h) / (1e6 * 3600.),
        ),
        2 => (
            (a.cos() * e * e + a.sin() * e) / 3600_f64.powi(2),
            (36. * h * h - 552. * h + 1440.) / (1e6 * 3600_f64.powi(2)),
        ),
        _ => unreachable!(),
    };
    let mut b = [0.; 13];
    b[0] = input / 200.;
    b[12] = input / 80.;
    b[9] = -output;
    b
}
pub fn initial(kind: &str) -> Vec<f64> {
    if kind == "hyperbolic" {
        (1..=250).map(|j| 1. + j as f64 / 250.).collect()
    } else {
        let mut y = vec![0.; 49];
        y[18..36].fill(0.047519404529185289807 / LS);
        y
    }
}
pub fn rhs(kind: &str, y: &[f64], t: f64, f: &mut [f64]) {
    if kind == "hyperbolic" {
        for j in 0..250 {
            let left = if j == 0 { 1. / (1. + t) } else { y[j - 1] };
            let x = (j + 1) as f64 / 250.;
            f[j] = -250. * (y[j] - left) + (t - x) / (1. + t).powi(2);
        }
        return;
    }
    let area = std::f64::consts::PI / 4.;
    let vmass = 1e6 / area;
    let cmass = 200. / 9800.;
    let mut net = boundary(t, 0);
    let mut acceleration = boundary(t, 1);
    for (k, &(i, j)) in PIPES.iter().enumerate() {
        let q = QS * y[k];
        let lambda = LS * y[18 + k];
        let s = lambda.sqrt();
        let r = (q / (1.31e-6 * area)).abs();
        let re = r.max(2300.);
        let loss = if r > 2300. {
            lambda * 1e6 * q * q / (area * area)
        } else {
            32. * 1.31 * q / area
        };
        f[k] = (pressure(y, i) - pressure(y, j) - loss) / (vmass * QS);
        f[18 + k] = 1. / s - 1.74 + 2. * (0.0004 + 18.7 / (re * s)).log10();
        net[i] -= q;
        net[j] += q;
        acceleration[i] -= f[k] * QS;
        acceleration[j] += f[k] * QS;
    }
    f[36] = net[4] / (cmass * PS);
    f[37] = net[7] / (cmass * PS);
    for (k, &node) in NODES[2..].iter().enumerate() {
        f[38 + k] = acceleration[node] / QS;
    }
}
pub fn jvp(kind: &str, y: &[f64], v: &[f64], f: &mut [f64]) {
    if kind == "hyperbolic" {
        for j in 0..250 {
            f[j] = -250. * (v[j] - if j == 0 { 0. } else { v[j - 1] });
        }
        return;
    }
    let area = std::f64::consts::PI / 4.;
    let vmass = 1e6 / area;
    let cmass = 200. / 9800.;
    let mut net = [0.; 13];
    let mut acceleration = [0.; 13];
    for (k, &(i, j)) in PIPES.iter().enumerate() {
        let q = QS * y[k];
        let dq = QS * v[k];
        let l = LS * y[18 + k];
        let dl = LS * v[18 + k];
        let s = l.sqrt();
        let r = (q / (1.31e-6 * area)).abs();
        let re = r.max(2300.);
        let dr = if r > 2300. {
            q.signum() * dq / (1.31e-6 * area)
        } else {
            0.
        };
        let loss = if r > 2300. {
            1e6 / (area * area) * (dl * q * q + 2. * l * q * dq)
        } else {
            32. * 1.31 * dq / area
        };
        f[k] = (pressure(v, i) - pressure(v, j) - loss) / (vmass * QS);
        let ds = dl / (2. * s);
        let z = 0.0004 + 18.7 / (re * s);
        f[18 + k] = -ds / (s * s)
            - 2. / std::f64::consts::LN_10 * 18.7 / z * (dr / (re * re * s) + ds / (re * s * s));
        net[i] -= dq;
        net[j] += dq;
        acceleration[i] -= f[k] * QS;
        acceleration[j] += f[k] * QS;
    }
    f[36] = net[4] / (cmass * PS);
    f[37] = net[7] / (cmass * PS);
    for (k, &node) in NODES[2..].iter().enumerate() {
        f[38 + k] = acceleration[node] / QS;
    }
}
pub fn time_partial(kind: &str, t: f64, f: &mut [f64]) {
    if kind == "hyperbolic" {
        for (j, z) in f.iter_mut().enumerate() {
            let x = (j + 1) as f64 / 250.;
            *z = 1. / (1. + t).powi(2) - 2. * (t - x) / (1. + t).powi(3);
        }
        f[0] -= 250. / (1. + t).powi(2);
    } else {
        let b = boundary(t, 2);
        for (k, &node) in NODES[2..].iter().enumerate() {
            f[38 + k] = b[node] / QS;
        }
    }
}

pub fn verify_derivatives() {
    for kind in ["hyperbolic", "water_index1"] {
        let mut y = initial(kind);
        if kind == "water_index1" {
            for k in 0..18 {
                y[k] = 0.2 + 0.31 * k as f64;
            }
            for k in 36..49 {
                y[k] = 0.03 * (k - 36) as f64;
            }
        }
        let v = (0..y.len())
            .map(|k| (k as f64 + 1.).sin())
            .collect::<Vec<_>>();
        let mut jac = vec![0.; y.len()];
        jvp(kind, &y, &v, &mut jac);
        let step = 1e-6;
        let yp = y
            .iter()
            .zip(&v)
            .map(|(a, b)| a + step * b)
            .collect::<Vec<_>>();
        let ym = y
            .iter()
            .zip(&v)
            .map(|(a, b)| a - step * b)
            .collect::<Vec<_>>();
        let t = if kind == "hyperbolic" { 0.4 } else { 10000. };
        let mut fp = vec![0.; y.len()];
        let mut fm = fp.clone();
        rhs(kind, &yp, t, &mut fp);
        rhs(kind, &ym, t, &mut fm);
        for i in 0..y.len() {
            assert!(
                ((fp[i] - fm[i]) / (2. * step) - jac[i]).abs() < 1e-7 * (1. + jac[i].abs()),
                "{kind} Jv row {i}"
            );
        }
        let step = if kind == "hyperbolic" { 1e-5 } else { 0.1 };
        rhs(kind, &y, t + step, &mut fp);
        rhs(kind, &y, t - step, &mut fm);
        let mut ft = vec![0.; y.len()];
        time_partial(kind, t, &mut ft);
        for i in 0..y.len() {
            assert!(
                ((fp[i] - fm[i]) / (2. * step) - ft[i]).abs() < 1e-7 * (1. + ft[i].abs()),
                "{kind} f_t row {i}"
            );
        }
        eprintln!("verified {kind} analytic Jv and f_t");
    }
}
