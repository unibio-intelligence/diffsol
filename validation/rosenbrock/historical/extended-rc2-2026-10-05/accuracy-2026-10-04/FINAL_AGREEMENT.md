# Final agreement with Julia and published sources

Updated 2026-10-05 for the complete uncommitted shared DiffSol/PharmFlux candidate.
Patch SHA-256: `6428eb759a9fd56abcef7a5d2f8d68df1121e133146bbe60a8fde6afc741ed50`.
The [completed investigation](unresolved-completion/README.md) describes each fix,
work limit and failure. [final-agreement.json](final-agreement.json) contains the
comparisons; scripts, CSVs, logs and locked environments remain retained.
Fable review and publication are pending. No commits or pushes were made.

## Fresh Julia agreement

Julia 1.12.6 / OrdinaryDiffEqRosenbrock 2.7.1 remains pinned. Fresh Rust runs are
compared with the retained pinned Julia results at identical fixed steps and inputs.
Ten cases cover decay, Robertson, stiff Van der Pol, cosine forcing and
Prothero–Robinson. The two source paths produce 55 byte-identical numerical rows.

| Method | Maximum fixed-step endpoint difference across five cases |
|---|---:|
| Rodas5P | 3.331e-16 |
| Rosenbrock23 | 4.441e-16 |

Five fresh Rodas5P stability values differ from Julia by at most 1.985e-15.
Ten fresh exact/inexact circle fixed-step error comparisons remain at Float64
roundoff agreement. The deliberately approximate Jacobian retains about
second-order convergence; it is not changed to restore a fifth-order claim.

Adaptive meshes and controllers differ. At matching requested tolerances, the
maximum endpoint differences over the five original problems are:

| Method | Requested rtol | Maximum absolute difference from Julia |
|---|---:|---:|
| Rodas5P | 1e-6 | 6.747e-8 |
| Rodas5P | 1e-9 | 1.592e-10 |
| Rosenbrock23 | 1e-6 | 9.126e-6 |
| Rosenbrock23 | 1e-9 | 7.039e-8 |

These are comparisons, not a ranking or true-error certificates. Analytic or
independently refined numerical references decide the separate accuracy gates.
Our retained linear-forcing discriminator checks the published third-stage
`gamma*h*f_t` formula; the pinned Julia estimator differs on that specific test.
This does not establish a blanket verdict about either implementation.

## Fresh selected paper examples

[Steinebach (2023)](https://doi.org/10.1007/s10543-023-00967-x) supplies the selected
Rodas5P table examples. The following percentages compare computed maximum
errors with rounded printed errors, not relative errors in the trajectories.

| Published table | Example | Rows | Maximum difference in printed error |
|---|---|---:|---:|
| 5 | Index-1 DAE | 3 | 0.083% |
| 6 | Stiff Prothero–Robinson | 4 | 0.351% |
| 7 | 1000-point parabolic | 4 | 0.083% |
| 8 | Index-2 diagnostic | 3 | 0.176% |

All fourteen freshly rerun rows agree within 0.351%. Independent scalar stage
oracles differ by at most 4.58e-16. Table 8 remains outside the declared index-1
scope. Table 6's order reduction to roughly three and the parabolic example's
mild reduction remain preserved as requested. Equivalent boundary lifting is
retained as a separately labeled experiment, not substituted for the source test.

The degree-five polynomial index-1 DAE still has approximately 0.417168 coarse
interior error despite a 1.92e-13 endpoint error. The published Rodas5P continuous
extension has order four. Fresh maximum-step and observation-endpoint tests
reduce seven-sample error from 0.461267 to 1.40844e-8 at maximum step 0.0625;
stepping to observations gives errors below 1.5e-13. These analytic-example
checks do not prove an arbitrary-DAE global bound or a fifth-order extension.

## Adaptive accuracy improvements and completed investigations

The common `refine_sampled_trajectory` API is implemented once in DiffSol and
used by PharmFlux. It tightens complete interval solves until successive
observations agree within 0.25 of the original `atol + rtol*abs(value)` scale.
All passes share the caller's budget. Persistent non-improvement, severe late
deterioration and non-convergence return explicit errors with pass/change details.
Those diagnostics are heuristics, not a mathematical true-error bound.

All twenty freshly rerun original analytic/tight-BDF refinement cases have
independent sampled errors below 0.06 units. All twenty actual PharmFlux CLI
Robertson combinations (five stiff solvers, rtol 1e-3 through 1e-6) pass the
one-unit independent Radau gate; worst error is **0.095622 units**. Transient
non-finite domain trials are recoverable rejections. TR-BDF2's previously measured
25.442-unit overrun is controlled through the same bounded interval mechanism.
The earlier ordinary local-control results remain historical comparisons;
requested local tolerances alone do not promise a global error bound.

General constant-mass Rosenbrock23 endpoint constraint control removes the
amplifier plateau. Compensated state/time accumulation removes tight-pendulum
roundoff drift; its final analytic error is **0.003505 units**. The independent
60-digit elliptic solution confirms the previous pendulum Radau reference within
2.90e-13 absolute. Product work budgets have not been enlarged.

| Current expanded coverage | Completed / total | Budget interpretation |
|---|---:|---|
| Original raw observations | 24 / 32 | Original 5M accepted steps/pass, eight passes |
| Original raw observations plus successful diagnostic runs | 29 / 32 | Five explicitly larger-budget runs |
| Including recovered hyperbolic and independently reduced water cases | 35 / 40 | Mixed stated budgets; not equal-work comparison |

The maximum independent-reference disagreement among those 35 raw checks is
0.330017 units. The water references themselves differ by up to 0.351080 units
at the tight target; this uncertainty prevents a finer true-error certificate.
The retained Julia extended results complete 30 checks under different stated
budgets. Neither completed-only denominator hides the failure lists.

A separate opt-in `interpolate_consistent` observation correction preserves M*y
while solving algebraic constraints in ker(M). It leaves the accepted trajectory,
stages, approximate Jacobian and original dense extension unchanged. Rodas5P's
inexact-circle runs then pass both targets in three passes, with worst analytic
error **0.007085 units**. The raw tight circle failure remains in the raw denominator.
This API is for smooth constant-mass index-1 steps; residual satisfaction is not
a differential accuracy certificate or an interpolation-order upgrade.

A separate [observation-endpoint option](observation-endpoints/README.md) uses
existing `set_stop_time(t)` calls at each requested time, with raw sampling and
no constraint correction. Fresh inexact-circle Rodas5P checks pass both targets
in three refinement passes under the original 5M-step/pass budget. Maximum
analytic error is **0.004264 units** (8,900 and 84,526 total accepted steps).
It removes interior interpolation error at the observations by changing the
adaptive mesh. These endpoint results are reported separately; the original
raw count remains 35/40 and the full suite has not been rerun with this policy.
The optional constraint-consistent sampling API remains available for smooth
index-1 steps when interior samples are required.

Three raw circle checks remain unqualified: tight Rodas5P and both Rosenbrock23
targets with the approximate Jacobian. Rosenbrock23's trace shows expensive
accepted-step constraint control rather than non-advancing time. Both water
Rosenbrock23 targets also remain unqualified: at Re=2300 the published pressure-loss
law jumps and smooth step refinement reaches Float64's time resolution. The
solver now reports typed `StepSizeTooSmall` instead of counting non-advancing
steps until a work cap. The law is not smoothed or replaced to make the test pass.

## Source coverage and correctness scope

The [eleven-reference matrix](PAPER_COVERAGE.md) covers ten papers and one
textbook. None is claimed reproduced in full. Selected Steinebach tables/examples,
Shampine–Reichelt formulas and pinned Julia comparisons have executable evidence;
other citations supply theory or deferred sensitivity context. The full
Lang–Verwer, Rang–Angermann and KPP suites have not been reproduced. Hosea–Shampine
TR-BDF2 is an additional citation with coefficient/Robertson checks, not a full
paper-table reproduction.

Nine named examples now have executable specifications, including the
[author-corroborated hyperbolic formulation](source-gaps/README.md). The original
water equations are recovered, but its exact published index-1 adaptation remains
unavailable. The independently derived reduction is labeled and separately
validated against null-space Radau/BDF references and original flow balances.
**Author contact was cancelled by the user; no source request was sent.**

A defensible claim is: the shared Rosenbrock implementation has verified behavior
on selected ODE and constant-mass index-1 DAE cases, supported by independent
stage/analytic oracles, published numerical values, pinned Julia checks and
fail-before/pass-after regressions. PharmFlux uses the same six numerical source
files. This supports the tested scope; it does not prove all adaptive trajectories,
all product workloads, universal error bounds or every cited paper.

Fresh gates pass 319 tests plus four doctests in default/nalgebra/faer, 350 plus
four with Cranelift, strict Clippy/rustdoc and formatting. PharmFlux passes 113 Rust
tests (one intentionally ignored diagnostic), 42 installed-wheel Python tests,
independent source rebuild/native-module equality, documentation/doctor and all
20 sampled Robertson CLI checks. Historical Liu/Rajwade library receipts are
separate archive-equation evidence, not fresh biological or clinical qualification.
Native upstream sensitivities need maintainer scope agreement. Cross-platform CI,
full R CMD and exact Docker/managed-operation qualification were not freshly run.
