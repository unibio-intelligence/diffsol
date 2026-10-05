# Current Rodas PR candidate versus dev

Measured 2026-10-05. Raw measurements and all case-wise comparisons are in
[comparison.json](comparison.json). The Rust checks are fresh; Julia comparator
values are retained from the pinned Julia 1.12.6 / OrdinaryDiffEqRosenbrock 2.7.1
runs. The common drivers, reference values and numerical settings are identical
for the two Rust sources.

| Check | Rodas PR `8affbd5` | dev `e553754` |
|---|---:|---:|
| Fixed-step cases compared with Julia | 10 | 10 |
| Largest absolute fixed-step difference | 1.01655e-15 | 4.44089e-16 |
| Stability values compared with Julia | 5 | 5 |
| Largest absolute stability difference | 1.8735e-15 | 1.98452e-15 |
| Selected printed paper-error rows compared | 14 | 14 |
| Largest relative difference from printed error | 9.04648% | 0.35047% |
| Matched adaptive endpoint comparisons with Julia | 20 | 20 |
| Expanded standard-budget sampled checks | Not run: extended APIs excluded from PR | 24/32 completed and within reference target |
| Hyperbolic/water supplement | Not run with this extended harness | 6/8 completed and within reference target |
| Optional Rodas5P observation-endpoint checks | Not run with this extended harness | 2/2 pass |
| Twenty-case sampled refinement experiment | Not run with this extended harness | 20/20 below target; worst 0.0590058 original tolerance units |
| DiffSol default library tests and doctests | 302 + 4, fresh | 319 + 4, retained checks on identical committed source |

The improved printed-table agreement in dev is measured. Its numerical
roundoff/time-partial improvements bring the finest parabolic row closer to the
printed result. The PR intentionally contains the smaller stage-formula and
constructor contribution. The missing extended harness runs are **not failures
of the PR's declared solver contract** and are not an equal-work branch comparison.

Both sources retain differences from Julia in adaptive output and step history.
Their largest pairwise absolute endpoint differences in these twenty comparisons
are approximately 9.13e-6. Pairwise differences do not establish which output is
closer to truth; inspect analytic/independent-reference checks for that question.
Local tolerances are not a universal bound on global trajectory error.

The fresh expanded dev runs use five million accepted steps per refinement pass,
up to eight passes. **30/40** raw sampled checks complete under those limits.
Eight original-suite runs hit the work cap; both Rosenbrock23 water targets fail
with typed `StepSizeTooSmall` at the friction-regime discontinuity. The complete
logs and successful rows are retained in the dev report. Failures are counted
in the denominator even though the harness exits successfully after recording
individual case failures.

The [historical investigation](historical/extended-rc2-2026-10-05/accuracy-2026-10-04/unresolved-completion/agreement.json)
records **35/40** raw successes with explicit larger budgets, including five
additional completed targets. This is a historical measurement of the RC2 patch;
those expensive larger-budget runs were not rerun for the new dev commit. It
must not be presented as fresh dev coverage or an equal-work Julia comparison.
The two optional observation-endpoint successes use a different sampling protocol
and are not added to the raw 30/40 or historical 35/40 counts.

Original parabolic order reduction and the published fourth-order dense extension
are retained. The dense polynomial diagnostic remains visible in both reports.
The degree-five diagnostic's approximately 0.417 dense error is an intentionally
coarse interpolation check, not a fifth-order dense-output claim. The exact
published water-tube index-1 adaptation remains a source gap; the independently
derived reduction and its checks are labelled accordingly.

The bibliography comprises ten papers and one textbook. These are selected
published-example reproductions and method/software checks, with background
references listed separately. They do not reproduce every result in all eleven
sources. The Table 8 index-2 example remains an out-of-scope diagnostic, separate
from the supported constant-mass index-1 class.

The full evidence stays in UniBio's fork. The reduced upstream patch contains
neither this archive nor the extended product/evidence APIs. Publication-status
fields in retained run receipts describe the state when those records were
collected.
