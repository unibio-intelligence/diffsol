# Shared DiffSol Rosenbrock implementation and accuracy evidence

Updated 2026-10-05. The [completed investigation](unresolved-completion/README.md) supersedes earlier unresolved-run counts and affected numerical measurements. The [final agreement report](FINAL_AGREEMENT.md) and
[full numerical comparison](final-agreement.json) summarize the current results.
[Coverage of all eleven references](PAPER_COVERAGE.md) separates reproduced
examples, formula/software checks and background citations.
This supplements the earlier Rosenbrock-class validation.
The candidate is uncommitted and has not been pushed or submitted. Fable review
is pending. These checks qualify the declared numerical API and equation class;
they do not qualify all Ubi product workloads, biological models or deployments.

## One implementation

The upstream contribution checkout is `/path/to/diffsol`.
PharmFlux's working source is `/path/to/pharmflux`. Its former
`Rodas5P` and `Rosenbrock23` implementation files now contain type aliases to
`Rosenbrock`. The generic solver, complete Runge–Kutta machinery, tableaus,
time-partial fallback, numerical harness and pinned Julia coefficient fixture
are byte-identical. PharmFlux calls the shared problem factories. It selects
bounded time probes and incoming-side stops for its known forcing/dosing jumps.
Its augmented-equation sensitivity and structural-sparsity infrastructure remain
product integration outside the shared solver files.

`vendor/diffsol/ROSENBROCK_SOURCE.json` records the upstream base, candidate patch
hash and shared source hashes. `scripts/check_diffsol_rosenbrock_sync.py` verifies
those hashes; with `--upstream PATH`, it also compares actual upstream bytes.
The public installation workflow checks this receipt. After upstream acceptance,
replace the candidate identity with the reviewed upstream commit and re-vendor
these files together. The receipt is not a claim of an accepted upstream commit.

`accuracy-results.json` records 55 identical native-result CSV rows from both
implementations under identical settings, including fixed and adaptive runs.
Source equality provides the stronger assurance against accidental duplication.

## Accuracy fixes and controls

- Numerical central time derivatives now use epsilon^(1/3), appropriate to a
  second-order difference, and the actual representable probe spacing. For
  f(t)=10000+sin(t), the derivative error at t=1 falls from about 1.14e-5 to
  3.90e-8. Both the old failing regression and passing new regression are retained
  in private evidence. This is a generic change to the default time-partial trait.
- `NonLinearOp::time_partial_inplace` supplies an optional analytic derivative.
  Its default returns false and selects finite differences; reference operators
  forward the hook. A regression proves that the analytic path avoids RHS probes.
- Accepted ODE endpoint derivatives are evaluated from the RHS. A quintic
  regression previously returned approximately 79.866 instead of 80 at t=2.
  Mass-matrix DAE derivatives remain extension derivatives: an RHS alone is not
  the derivative when M is singular. This adds an RHS evaluation per attempt.
- The common solver provides `set_time_derivative_within_step(true)` and
  `set_discontinuity_stop_time(t)`. Second-order one-sided probes stay within the
  attempted interval; endpoint stages use the incoming side of a marked jump.
  Subsequent intervals use the outgoing forcing. A 1-to-100 forcing-jump test
  passes for both methods. Unknown discontinuities still require caller handling.
- `set_maximum_step(h)` bounds step size even when endpoint error permits a large
  step. Its test covers validation, clone preservation and improved quintic dense
  values. This supplements tolerances; it does not guarantee a global error bound.

## Published parabolic example and equivalent lifting

The original manufactured parabolic equations and their time-dependent boundary
conditions remain the reproduction. A separate equivalent formulation substitutes
u=v+x exp(t). The lifted variable has homogeneous boundary conditions; the
forcing includes both the lifted nonlinear term and -x exp(t). Recover u after
integration. Independently authored drivers evaluate both against the exact
solution on meshes of 100 and 1000 interior points.

On the 1000-point mesh, maximum errors are:

| Step | Paper rounded error | Original formulation | Lifted formulation | Improvement |
|---|---:|---:|---:|---:|
| 1/32 | 5.97e-9 | 5.97040e-9 | 1.38294e-9 | 4.32x |
| 1/64 | 4.72e-10 | 4.72386e-10 | 7.39531e-11 | 6.39x |
| 1/128 | 3.45e-11 | 3.44795e-11 | 3.41971e-12 | 10.08x |
| 1/256 | 2.36e-12 | 2.36167e-12 | 1.44995e-13 | 16.29x |

Fresh Julia Rodas5P errors for the first three steps are 5.970386e-9,
4.724734e-10 and 3.448442e-11. The original source example retains mild order
reduction: errors decrease at observed orders about 3.66, 3.78 and 3.87, rather
than uniformly five. Lifting improves observed orders to about 4.23, 4.43 and
4.56 over this finite range. This is a demonstrated accuracy improvement, not
proof of restored fifth order for every parabolic problem. No coefficients were
changed to make the original reproduction appear fifth order.

The other retained paper table rows reproduce their printed rounded errors
within rounding-level relative differences. Scalar analytic stage oracles agree
within 4.6e-16. Exact DAE examples and stability diagnostics are retained.

## Dense extension: degree five

Rodas5P has fifth-order endpoint integration and a fourth-order continuous
extension. Degree-five interpolation need not be exact, even when the endpoint
is nearly exact. The deliberately coarse polynomial DAE diagnostic still
reports maximum dense error 0.4171677 and endpoint error 1.92e-13; its derivative
error is 0.1338501. This DAE is index-1: its algebraic constraint y0-y1=0 has
derivative -1 with respect to y1. It is within the supported constant-mass
index-1 class. Earlier higher-index wording was incorrect. The separate Table 8
index-2 example remains outside that class.

A separate supported quintic ODE test shows interior error shrinking by about
32x when a single step is halved, consistent with local h^5 interpolation error.
Reducing step size or stepping to the observation improves accuracy without
inventing a new extension. A fresh index-1 DAE check samples seven unaligned
observation times: reducing maximum step 2 to 0.0625 lowers maximum error from
0.4612673 to 1.40844e-8. Ending steps at observations gives errors below
1.5e-13. The upstream and vendored drivers produce identical rows; these are
polynomial-example results, not a bound for arbitrary DAEs. See
[dense-accuracy.json](dense-accuracy.json). A fifth-order dense extension would require a new
method derivation and independent validation; this candidate retains the
published extension and documents its actual limits. ODE endpoint derivatives
are fixed; interpolated and singular-DAE derivatives retain extension limits.

## Adaptive runs and the Julia discrepancy

The same adaptive step count is not an accuracy requirement. DiffSol retains
its shared starting-state scaling and PI controller. Julia's controller and
error scaling differ. Local tolerances do not guarantee a global endpoint error.
At rtol=1e-9, Rosenbrock23 still exceeds a strict absolute 1e-9 endpoint diagnostic
on some examples; that remains visible rather than silently changing defaults.

The additional rtol=1e-13 grid puts all ten tested method/problem endpoints below
2.23e-10 against analytic solutions or a tight BDF comparator. This establishes
an effective setting for these cases, not a universal recommended tolerance.
BDF comparisons are numerical agreement, not an independent exact oracle.

For the pinned OrdinaryDiffEqRosenbrock 2.7.1, `julia-estimator.jl` isolates y'=t,
J=0 and analytic f_t=1. The accepted endpoint is exact, but the reported local
estimate is (1-gamma)h^2/6, where gamma=1/(2+sqrt(2)). Substitution into the
published Rosenbrock23 stages gives gamma*h*f_t in the third stage and zero
error for this linear forcing; using h*f_t produces Julia's nonzero quadratic
estimate. The retained Julia script asserts both quantities at three step sizes.
Existing Rust direct-stage and cubic-estimator regressions validate the shared
implementation independently. This identifies a discrepancy in the pinned
Julia estimator, not a claim that all Julia results or later versions are wrong.

The separate [adaptive refinement experiment](adaptive-refinement.json)
reuses the shared methods and tightens whole sampled trajectories until
successive runs agree. All twenty tested runs finish below 0.06 original
tolerance units against analytic solutions or tight BDF comparators. It is
a standalone experiment, not an integrated production accuracy mode.
Accepted-step work increases up to 8.71x for Rodas5P and 615.91x for
Rosenbrock23; production integration needs explicit options, shared budgets
and convergence/failure reporting. See [reference coverage](PAPER_COVERAGE.md).

## Observation endpoints as an explicit sampling option

For known observation times, the existing shared `set_stop_time(t)` API can end
steps at those times. The [reproducible endpoint option](observation-endpoints/README.md)
retains the deliberately approximate circle Jacobian, uses raw sampling and
passes both Rodas5P targets in three refinement passes; worst analytic error is
0.004264 tolerance units. It changes the mesh and is reported separately from
the raw 35/40 results. No separate solver, new coefficients or default sampling
policy is introduced. This complements optional constraint-consistent interior
samples without claiming a higher-order continuous extension.

## Validation scope and reproduction

Default, nalgebra and faer configurations each pass 312 tests and four doctests.
Cranelift passes 343 tests and four doctests. Formatting, strict all-target Clippy
and strict rustdoc pass. Upstream dependencies also enable both matrix backends,
so feature flags are not evidence of exclusive backend isolation.

The full PharmFlux workspace, installed Python wheel and source archive are
checked separately; see the private release review evidence for final outcomes and hashes.
Cross-platform CI, R CMD check, current runtime-image/product qualification and
public publication remain outside these local numerical checks.

To rerun the numerical drivers:

```sh
python run-rust.py --diffsol /path/to/diffsol --kind native --target-dir /tmp/native-target
python run-rust.py --diffsol /path/to/diffsol --kind paper --target-dir /tmp/native-target
python run-rust.py --diffsol /path/to/diffsol --kind parabolic --target-dir /tmp/native-target
python run-rust.py --diffsol /path/to/diffsol --kind properties --target-dir /tmp/native-target
python run-rust.py --diffsol /path/to/diffsol --kind dense-accuracy --target-dir /tmp/native-target
python run-rust.py --diffsol /path/to/diffsol --kind adaptive-refinement --target-dir /tmp/native-target
julia --project=. julia-reference.jl
julia --project=. julia-parabolic.jl
julia --project=. julia-estimator.jl
```

Julia 1.12.6 and OrdinaryDiffEqRosenbrock 2.7.1 are pinned in Project/Manifest.
The Rust driver uses its retained lockfile. Native logs are not formatted as an
accuracy certificate: inspect the result rows and references for each claim.

Sources: [Steinebach (2023)](https://doi.org/10.1007/s10543-023-00967-x),
[SciML Rosenbrock method documentation](https://docs.sciml.ai/OrdinaryDiffEq/stable/massmatrixdae/Rosenbrock/),
[FiniteDiff step-size guidance](https://docs.sciml.ai/FiniteDiff/dev/epsilons/).

## Reported export defects and current fixes

The public main head was rechecked read-only as
496e9e6f41efeb32011329454d6a4219e7841d49. The candidate/publishing policy use that
base and retain the later public guides and their closure. The earlier stale
accepted head is superseded in the private working policy. Public main remains
unprotected; its settings and remote branches have not been changed.

The two former concrete Rosenbrock implementations are removed. The shared
qualified extension now drives actual PharmFlux sampling. The new Robertson
regression exercises BDF, ESDIRK34, TR-BDF2, Rosenbrock23 and Rodas5P at 13 sparse
sample times from 0 through 10000 seconds and rtol=1e-3, 1e-4, 1e-5 and 1e-6.
Its independent comparator is generated with SciPy Radau, rtol=2.3e-14 and
atol=1e-16; the generator, version and values are retained in public fixtures.
Rosenbrock23's worst discrepancy over this grid is approximately 1.005 tolerance
units and Rodas5P's is 0.946; both complete every case. These are benchmark
results, not reproduction of an unavailable private attached request.

Non-finite trials are now checked before the floating-point max reduction can
suppress NaNs. The shared Rosenbrock engine shrinks and retries, leaving the
accepted state intact. PharmFlux preserves transient Domain errors during
Rosenbrock step attempts and initial-step probes as non-finite trial signals.
Errors at the interval origin, non-Domain errors, budgets and persistent failures
still fail. Both shared red regressions failed before the fix and pass afterward.
Product tests verify successful retry, persistent Domain failure and WorkBudget
precedence. This behavior is common to the contribution and product engine.

TR-BDF2's additional overrun was at the accepted final endpoint. Its stage and
embedded coefficients agree with the independent reference; local error control
alone permitted accumulated global error. PharmFlux now reruns each complete
sampled interval with local tolerances tightened by a factor of ten, requiring
successive state trajectories to agree within the original requested error scale.
It returns the finer pass, permits at most six passes, shares the original
callback budget, and does not replay events or substitute another solver.
If agreement is unattainable it returns a structured solver error. This is a
convergence safeguard using the same DiffSol implementation, not a new TR-BDF2
stage algorithm or a universal mathematical global-error certificate. Repeated
solves cost more; fitted/sensitivity solver availability is unchanged.

| Requested rtol | TR-BDF2 previous maximum tolerance units | Current maximum tolerance units |
|---|---:|---:|
| 1e-3 | 1.67260 | 0.09728 |
| 1e-4 | 3.37665 | 0.21638 |
| 1e-5 | 9.71796 | 0.07990 |
| 1e-6 | 25.44203 | 0.20532 |

The regression now requires TR-BDF2 agreement with Radau within one requested
tolerance unit; the earlier relaxed 30-unit bound was removed. Bolus, infusion,
stiff-dose and solver-selection checks also pass with refinement.

The synthetic fixture license and current engine/license references now say
Apache-2.0. Root LICENSE is the Apache text; duplicate LICENSE-APACHE and the
unreferenced conformance worker are removed, with build references updated.
The pandas-only helper test uses pytest.importorskip so minimal optional-analysis
installs can skip the DataFrame case. Vendored upstream sources are retained;
trimming examples/assets is deferred to an explicit, provenance-preserving update.

Only main and dev remain in the active local PharmFlux and DiffSol contribution
checkouts, and work is on dev. Old branch history is preserved in private Git
bundles. The canonical private remote is unibio-intelligence/pharmflux-internal;
its main already contains the current source head 86f7ebd. Remote branch migration
and protection remain a concrete pending plan under the no-commit/no-push rule.


The latest [final report](FINAL_AGREEMENT.md) includes the shared
`refine_sampled_trajectory` production API, PharmFlux's explicit sampled control,
20 CLI sampled-accuracy checks and [seven additional problems](additional/README.md).
See those current results before using the earlier local-only summaries.

Current 2026-10-05 results and changed-candidate gates are in the
[unresolved-run follow-up](unresolved-followup/README.md). This supersedes earlier
counts and records the diagonal-DAE estimator correction and all remaining failures.
