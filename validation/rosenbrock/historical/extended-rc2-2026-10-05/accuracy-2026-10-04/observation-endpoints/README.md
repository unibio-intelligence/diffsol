# Accepted endpoints at requested observations

This separately labeled sampling option uses the existing shared Rosenbrock
`set_stop_time(t)` API to end an accepted step at each requested observation.
It retains the paper's deliberately approximate circle Jacobian and original
method coefficients. It does not use `interpolate_consistent` or alter its
accepted states through a constraint correction.

Within each refinement pass, set the next observation as the stop time, advance
until it is reached, and read the endpoint through the existing sampling API:

```rust
for &t in &observation_times {
    solver.set_stop_time(t)?;
    while solver.state().t < t {
        solver.step()?;
    }
    samples.push(solver.interpolate(t)?);
}
```

Observation times here are strictly increasing and later than the initial time.
The same solver instance and controller continue within each pass. This removes
interior interpolation error at these times; time integration, roundoff and
other errors remain. Extra stops change the adaptive mesh and may increase work.
For smooth constant-mass index-1 problems with known observation times, this is
a demonstrated alternative to dense sampling. Unknown discontinuities still
require explicit event handling.

## Reproduction

From the evidence root, with a local candidate checkout:

```sh
VALIDATION_PROBLEM=circle_inexact VALIDATION_METHOD=rodas5p \
VALIDATION_OBSERVATION_ENDPOINTS=1 VALIDATION_RECORD_PASSES=1 \
python3 run-rust.py --diffsol /path/to/diffsol --kind additional \
  > observation-endpoints/native.csv 2> observation-endpoints/native.log
python3 additional/analyze.py "$(pwd)/observation-endpoints/native"
```

The analyzer uses the independent sin(t), cos(t) oracle. The original maximum
of 5 million accepted steps per pass and eight refinement passes is unchanged.
Whole-trajectory refinement uses the shared API's 0.25-unit convergence gate.
Endpoint and consistent-interpolation flags are mutually exclusive in this
comparison harness so the protocols cannot be accidentally combined.

| Requested rtol | Refinement passes | Total accepted steps | Maximum analytic error in requested tolerance units |
|---|---:|---:|---:|
| 1e-6 | 3 | 8,900 | 0.004264024 |
| 1e-8 | 3 | 84,526 | 0.004180982 |

Both checks pass the one-unit analytic gate. CSV modes use the `endpoint-`
prefix; [native-agreement.json](native-agreement.json) retains every pass and
algebraic residual. These results reproduce the earlier private diagnostic.

The retained raw dense-output benchmark remains 35/40. Its tight Rodas5P circle
failure remains visible. This protocol has not been applied to the entire
40-case suite and is not substituted into that denominator. The two expensive
Rosenbrock23 approximate-Jacobian circle targets and the two water discontinuity
targets remain unqualified. The generic sampling option does not promise a
universal global error bound or a fifth-order dense extension.
