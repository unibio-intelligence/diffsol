# Historical unresolved-run follow-up

Superseded by [the completed investigation](../unresolved-completion/README.md). Measurements below identify the earlier source candidate. Updated 2026-10-05. Candidate source patch SHA-256:
`87e83dd29470af54ce0c3785e3c2a0562ca225913f9802bd25efc3d7c6ac62cd`.
This is uncommitted UniBio Intelligence work. Fable review and publication are pending.

## Source recovery

The author’s [2025 preprint, §4, equation (18)](https://arxiv.org/html/2511.21252v1#S4)
explicitly reuses the 2023 benchmarks and confirms the hyperbolic equation,
analytic solution, interval and 250-point grid in our recovered specification.
This closes that formulation gap. The 2023 timing curves and complete 2020
chapter implementation remain outside the reproduction claim.

The original water-tube equations, constants, topology and starting values are
recovered. The [independent index reduction and its validation](../source-gaps/README.md)
remain usable evidence: Rodas5P passes both targets against separate Radau/BDF
null-space ODE references, with the original undifferentiated balances checked.
The paper does not specify its particular index-1 adaptation. No exact-author
adaptation claim is made; author contact was subsequently cancelled by the user; no request was sent.

## Corrected Rosenbrock23 diagonal-DAE error control

A PV rejection trace showed its error staying near 7.500185 squared units as
step size shrank to 1.47e-6 s. The accepted algebraic endpoint was inconsistent;
its constant residual prevented the ODE embedded formula from shrinking to zero.

For Rosenbrock23 with a diagonal constant mass matrix, the shared implementation
now excludes algebraic components from the differential embedded estimate and
separately controls the algebraic RHS at the trial endpoint, normalized by its
absolute tolerance. It follows the approach in
[OrdinaryDiffEqRosenbrock’s implementation](https://github.com/SciML/OrdinaryDiffEq.jl/blob/master/lib/OrdinaryDiffEqRosenbrock/src/rosenbrock_perform_step.jl),
while retaining the independently verified published stage/time coefficients.
The PI controller and ODE/Rodas5P stage formulas are unchanged. General,
non-diagonal mass matrices retain the previous estimator; this correction does
not qualify every Rosenbrock23 DAE or project its dense output.
Algebraic equation scaling matters: the residual must have the meaning of its
specified absolute tolerance. This is an accuracy check, not a global-error bound.

A new nonlinear DAE regression with analytic solutions fails before the
correction and passes after it on both CPU backends. The fresh full gates pass:
314 tests and four doctests in default/nalgebra/faer configurations; 345 and four
with Cranelift; strict Clippy/rustdoc and formatting. PharmFlux passes 113 Rust
and 42 installed-wheel Python tests, docs/doctor and a fresh source-distribution
rebuild. Twenty sampled Robertson CLI cases pass again. All six shared numerical
files are byte-identical. The 55 core rows and 153 new PV rows are upstream/vendor
bit-identical; the 55 core rows also match the previous candidate.

## Four original failures resolved

A unit is the original requested `atol + rtol*abs(reference)` scale.
A completed check must pass the unchanged 0.25 successive-sample convergence
criterion and the independent one-unit reference gate. A larger-budget result
does not qualify the original budget. Product work limits have not increased.

| Original failure | Current status | Total accepted steps across passes | Maximum reference error units |
|---|---|---:|---:|
| PV, Rosenbrock23, 1e-6 | Corrected estimator; completes within original limit | 1,403,747 | 0.017769 |
| PV, Rosenbrock23, 1e-8 | Corrected estimator; 20M steps/pass diagnostic | 14,032,305 | 0.017882 |
| Pendulum, Rosenbrock23, 1e-6 | Corrected estimator; 20M steps/pass diagnostic | 12,529,442 | 0.003399 |
| Oregonator, Rosenbrock23, 1e-8 | Unchanged ODE arithmetic; 20M steps/pass diagnostic | 21,870,206 | 0.011393 |

Fresh standard-budget execution completes **24/32** original extended checks.
Larger-budget evidence raises that to **27/32**. Adding the four hyperbolic and
two Rodas5P water checks gives **33/40**, with seven unqualified targets including
the two water Rosenbrock23 cases. These counts mix explicitly stated budgets;
they are not an equal-work benchmark or a guarantee under product limits.

Julia also completes the PV 1e-8 case with its larger budget: 14,383,304 accepted
steps and 0.060388 reference units. Its Oregonator case remains unresolved after
eight passes with a 20M attempted-step cap; its tight pendulum becomes `Unstable`
on pass five despite a 100M attempted-step cap. Adaptive step counts, controller
policies and initial consistency handling differ; independent references, rather
than one implementation’s adaptive trajectory, decide accuracy.

## Five original failures still unqualified

- Amplifier, Rosenbrock23, 1e-8: extending to 20M accepted steps/pass reveals
  repeated error-test failures at t=0.0209779713. Its mass matrix is non-diagonal;
  Julia Rosenbrock23 does not support that matrix. This needs a general-mass DAE
  estimator/constraint treatment or a narrower method qualification, not just
  more work. Rodas5P passes this example at both targets.
- Pendulum, Rosenbrock23, 1e-8: the third completed pass reaches **0.066201**
  reference units, but coarse/fine change is **0.397172**, above the 0.25 gate.
  The fourth pass worsens to **3.191169** units after 125,139,516 total accepted
  steps. The fifth was deliberately interrupted once that loss of accuracy was
  measured; this is not reported as a solver failure return code or completion.
  Further blind tightening is not supported by the evidence. Rodas5P passes.
- Inexact circle, Rodas5P, 1e-8: 20M steps/pass and eight passes do not converge.
  A 100M steps/pass, ten-pass diagnostic reaches the cap. Its eighth and ninth
  completed passes have errors **0.350212** and **0.496218** units: extra
  refinement is no longer consistently improving accuracy. Neither meets the
  prescribed convergence qualification. The published inexact Jacobian is
  retained; replacing it with the exact Jacobian would change the experiment.
- Inexact circle, Rosenbrock23, 1e-6 and 1e-8: the corrected diagonal estimator
  still reaches a 20M accepted-step cap. This deliberately inexact-Jacobian DAE
  remains unqualified. The exact-Jacobian circle passes both targets.

The two additional water Rosenbrock23 targets still hit the explicitly limited
250,000 accepted-step diagnostic cap after the correction. They remain unqualified;
Julia’s earlier `Unstable` outcomes are retained. Rodas5P completes both targets.
The published parabolic order reduction and fourth-order dense extension are
unchanged. None of these observations establishes reproduction of eleven entire
papers or universal global accuracy at a requested local tolerance.

## Reproduction and retained evidence

[agreement.json](agreement.json) records all completed checks, failures,
constraints, independent-reference errors and current counts.
[dae-work-protocol.json](dae-work-protocol.json) records isolated corrected runs.
The original experiments in `additional/` retain their prior source digest.
The Oregonator, amplifier and inexact-Rodas larger-budget probes initially used
that prior candidate; those ODE/Rodas5P formulas are unchanged by this correction.
Run [analyze.py](analyze.py) with the retained SciPy environment to recompute status.

Use the parent locked Rust and pinned Julia runners. `VALIDATION_PROBLEM`,
`VALIDATION_METHOD`, `VALIDATION_TARGET`, `VALIDATION_STEP_CAP`,
`VALIDATION_PASSES` and `VALIDATION_RECORD_PASSES` isolate and document probes.
The standard Rust limits remain 5M accepted steps/pass and eight passes. Julia
counts attempted steps and can write separate CSVs via `VALIDATION_CSV`.
No commits, pushes, public review requests or author messages were made.

The final PharmFlux export is reconciled to its current 720-file public policy,
including concurrent fitting/interface edits. The earlier 735-file snapshot
and its 114-test count are superseded for publication; the final snapshot
passes 113 Rust tests, one ignored diagnostic, and 42 Python tests. An invalid
synthetic MCP fixture start is corrected and its recovery test passes.
