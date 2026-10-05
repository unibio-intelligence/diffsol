# Completed unresolved-run investigations

2026-10-05, uncommitted UniBio Intelligence candidate, patch SHA-256
`6428eb759a9fd56abcef7a5d2f8d68df1121e133146bbe60a8fde6afc741ed50`.
[agreement.json](agreement.json) contains the current comparisons and complete
failure lists. Earlier `unresolved-followup/` results remain historical.
Fable review and publication are pending. No commits or pushes were made.

## General constant-mass DAE control

Rosenbrock23 now projects embedded state error into the mass matrix's row space
and checks endpoint RHS residuals in its left null space. The diagonal case keeps
its inexpensive coordinate specialization. A scaled, pivoted and twice
reorthogonalized basis is cached; numerical rank uses `8*n*epsilon` after scaling
by the largest entry. Algebraic equation scaling and conditioning remain relevant.
This is local constraint control, not a global error certificate.

The rotated nonlinear DAE regression fails before the correction and passes on
nalgebra and faer after it. The non-diagonal amplifier's former rejection plateau
is removed. At target 1e-8 it passes with a 20M accepted-step limit per pass:
14,895,893 total steps, 0.016339 independent Radau reference units. The original
5M-per-pass budget remains insufficient; product budgets have not increased.

## Roundoff in the tight pendulum

Repeated tiny additions to the state accumulated roundoff over millions of steps.
Compensated state and time accumulation now commits its carry only on acceptance;
state mutation/restart clears it. A small-increment regression fails before the
change and passes for both methods on both CPU backends after it.

The 1e-8 pendulum completes four passes and 125,169,053 accepted steps under its
100M-per-pass diagnostic limit. Successive observations change by 0.027111 units;
error is 0.003505 against a separately evaluated 60-digit Jacobi elliptic analytic
solution. The previous tight Radau reference differs from this oracle by only
2.90e-13 absolute. The retained pinned Julia Float64/256-bit experiment uses binary
steps and an exactly representable endpoint; at h=2^-24 its errors are 2.11e-13 and
1.25e-14. These diagnose roundoff without promising BigFloat support in DiffSol.

## Inexact-Jacobian circle

The prescribed approximate Jacobian is retained. Accepted Rodas5P endpoints were
accurate while raw interpolated algebraic variables dominated the error; fixed-step
endpoint order remains approximately two and agrees with Julia. Replacing the
Jacobian would change the published experiment.

The explicit `interpolate_consistent(t, iterations)` API starts with the original
interpolant and performs bounded constraint Newton corrections in ker(M),
preserving M*y. It uses the supplied Jacobian and fails on singular/non-finite or
unconverged corrections. It leaves stages, accepted state and the published dense
extension unchanged. It is intended for smooth constant-mass index-1 steps;
constraint satisfaction does not certify differential-state accuracy or restore
fifth-order interpolation. Tests cover differential-coordinate preservation and
rotated mass on both CPU backends.

With this observation option, Rodas5P passes both circle targets in three passes:
8,385 / 83,993 accepted steps and 0.006427 / 0.007085 analytic error units.
The raw tight Rodas5P result remains unqualified and is not relabeled a success.
Rosenbrock23 still hits the 20M-step diagnostic cap even with corrected observations.
A trace at target 1e-6 reaches only t=0.060414 after 250,000 accepted steps, with
h=1.21e-7 and just 12 rejected steps. This is expensive accepted-step constraint
control under the approximate Jacobian, not a non-advancing-time or repeated
rejection defect. The exact-Jacobian circle passes both methods/targets.

## Water-tube threshold and source boundary

Both Rosenbrock23 runs stall when pipe 18 (nodes 12 to 10) reaches Re=2300. The
retained published pressure-loss law jumps from 0.126305 Pa to a turbulent-side
limit of 0.431389 Pa at that state. The differentiated index-1 equations inherit
this discontinuity; shrinking a smooth step across it cannot satisfy the endpoint
constraint test. At times 3535.493 / 3535.506 seconds, rejected steps become too
small to advance Float64 time. The shared solver now returns typed
`StepSizeTooSmall`, even if a caller disables its ordinary minimum step. It no
longer falsely counts non-advancing steps to a work cap. See [water diagnosis](water-diagnosis.json).

Analytic Jv and time partials pass independent finite-difference checks in smooth
regimes. Rodas5P still passes both targets against separate null-space Radau/BDF
references, preserving original undifferentiated flow balances. Their mutual
uncertainty reaches 0.351080 requested units at the tight target. Event localization
and an algebraic consistency reset would be needed for a defensible method-specific
crossing fix; smoothing the law would change the source experiment. No such
unverified replacement is introduced.

The original water formulation is recovered. The paper's exact index-1 adaptation
remains unavailable; our independently derived system is labeled accordingly.
The user cancelled author contact. No source request was sent; the gap is recorded
only. The author-corroborated hyperbolic formulation passes all four checks.

## Refinement diagnostics and final coverage

The common refinement helper reports persistent non-improvement (three changes
that shrink by less than 5%) or severe late deterioration (more than 8x after a
previous change below one unit). These are conservative heuristic failure
signals, not physical diagnoses. Error text retains pass, local tolerance factor
and last change. Fail-before/pass-after regressions prevent false convergence of
repeated flat or worsening synthetic sequences. Every successful scientific run
still meets the original 0.25 successive-change and one-unit independent-reference
criteria.

Fresh original-budget coverage is 24/32. Five explicitly larger-budget runs raise
raw coverage to 29/32; the four hyperbolic and two Rodas5P water runs give 35/40.
This mixes stated budgets and is not an equal-work comparison. The maximum
independent-reference discrepancy among those 35 checks is 0.330017 units
(the water comparator has its stated uncertainty). Six of the original nine
unsuccessful targets now complete; three original circle and two water targets
remain unqualified. Optional circle observation results are separate from that
raw denominator. Julia's retained extended coverage is 30 checks under its own
budgets, not a ranking at equal work.

Fresh library gates: 319 tests and four doctests for default/nalgebra/faer,
350 and four with Cranelift, strict Clippy/rustdoc and formatting. PharmFlux:
113 Rust tests, one ignored diagnostic, 42 installed-wheel Python tests, source
rebuild/native-module equality, documentation/doctor and 20 sampled Robertson
CLI runs (maximum 0.095622 units). All six shared source files and 55 fresh core
numerical rows agree byte-for-byte between the two repositories.

The full upstream patch has 3,592 additions and 22 deletions. About 1,963 added
lines are its test module, harness and coefficient fixture; approximately 1,629
remain elsewhere. Both-backend new checks already use a single parametrized
body. Removing all those tests would discard correctness evidence; this is a
size breakdown, not a recommendation to delete them. The source-table reproductions
and fixed-step Julia checks remain retained.

To reproduce, use the parent locked Rust runner and its documented filters, then
run `analyze.py` with the SciPy environment. Regenerate the analytic oracle with
mpmath 1.3.0. Julia precision diagnostics use the parent's pinned Julia project.
The original parabolic order reduction and published fourth-order extension remain.
The bibliography describes selected tests and theory, not eleven full reproductions.
