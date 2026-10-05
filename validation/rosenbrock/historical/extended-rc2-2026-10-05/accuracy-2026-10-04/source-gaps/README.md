# Recovered source specifications and independently reduced water tubes

Prepared 2026-10-05. These are additional validation examples, outside the
upstream source patch. No solver implementation or published parabolic test was
changed. Publication and Fable review remain pending.

## Hyperbolic source recovered

The [author-hosted Sanz-Serna, Verwer and Hundsdorfer paper](https://beta.sanzserna.org/wp-content/uploads/1986/01/473_numer_willem.pdf),
Examples 3.1 and 5.1, gives the previously missing equations:

\[
u_t=-u_x+(t-x)/(1+t)^2,\quad 0<x\le1,\;0\le t\le1,
\qquad u(0,t)=1/(1+t),\quad u(x,0)=1+x.
\]

Its analytic solution is \(u(x,t)=(1+x)/(1+t)\). With 250 unknowns at
\(x_j=j/250\), backward first differences are exact on that solution.
The driver supplies the exact Jacobian and partial time derivative; the boundary
contribution is included in the first row's time derivative.

[Steinebach (2023), §4](https://link.springer.com/article/10.1007/s10543-023-00967-x)
cites that paper and describes 250 points and a solution linear in space.
The author's later [Rodas6P/Tsit5DA preprint (2025), §4, equation (18)](https://arxiv.org/html/2511.21252v1#S4), explicitly reuses the 2023 benchmarks and specifies this same PDE, analytic solution, interval and 250-point grid. This closes the hyperbolic equation/specification gap with direct author corroboration. Only scientific facts are used from that corroborating preprint; its protected text, tables and code are not included.
The 2020 chapter's particular implementation and the 2023 multi-method timing
curves have not been independently recovered or reproduced. We claim execution
of the cited original example on the stated grid, not reproduction of those curves.

## Water: original equations recovered; independent index reduction

The original 49-variable water test is documented by the
[IVP Test Set](https://pitagora.dm.uniba.it/~testset/problems/water.php).
The complete equations, constants, topology and initial conditions are available
in the [SciML benchmark at the pinned source](https://github.com/SciML/SciMLBenchmarks.jl/blob/2ac54a8200771e74bab9144851885cb12a8aa8dd/benchmarks/DAE/water_tube.jmd).
They were cross-checked against the Test Set revision `water.F,v 1.2`, retained
in [deTestSet's source](https://github.com/cran/deTestSet/blob/d5c4cbe821920ad0a2ac36ec03c6175531316213/src/Ex_tube.f).
No Fortran code or its reference solution arrays are included in this evidence.
The drivers independently express the scientific equations.

Let \(B\) be the node/pipe incidence matrix (inflow positive), \(C\) its eleven
non-storage rows, \(H\) its two storage rows, \(q\) the pipe flows, \(p\) node
pressures, \(\ell(q,\lambda)\) the original pressure loss and \(e(t)\) external
node inflows. The original equations are

\[
Vq'=-B^Tp-\ell(q,\lambda),\quad
c p_S'=Hq+e_S(t),\quad Cq+e_N(t)=0,
\]

together with eighteen algebraic Colebrook friction equations. All pipe mass
coefficients equal \(V=\rho L/A\), and \(c=b/(\rho g)\).
We replace only the eleven non-storage balances by their time derivatives:

\[
0=C[-B^Tp-\ell(q,\lambda)]/V+e_N'(t).
\]

The eleven unknown pressures now enter directly through the invertible
\(-CC^T/V\). The original flow constraints remain invariants because their
derivatives vanish. Consistent starting values are essential, and numerical
drift in those original constraints is measured separately.
This is an independently derived index-1 system, **not a verified copy of
Steinebach's unspecified adaptation**. Differentiation does not authorize
applying an index-1 solver directly to the original index-2 system.

All original physical constants and piecewise laminar/turbulent laws are retained,
including the original quadratic turbulent loss. Pressure is represented by its
offset from 109800 Pa; scales are 0.001 m³/s for flow, 0.05 for friction, and
1000 Pa for pressure offset. Differential rows are divided by their mass and
state scales; algebraic balance derivatives by the flow scale. Tolerances apply
to these declared scaled variables, not directly to the original physical units.
The published starting friction coefficient is retained. The simulation spans
the full 61200 s. A maximum step of `120*sqrt(factor)` s accompanies local
tolerance refinement. This explicit policy is the same in Rust and Julia.

The independent reference eliminates all algebraic variables from the **original
index-2 equations**. Set \(q=-C^T(CC^T)^{-1}e_N+Nz\), where the seven columns of
\(N\) form an orthonormal null space of \(C\). Solve Colebrook in
\(1/\sqrt\lambda\); solve the eleven pressures through \(CC^T\); integrate
seven flow coordinates and two storage pressures. This nine-coordinate ODE
preserves the undifferentiated balances by construction and provides a separate
formulation and integration algorithm.

## Measured agreement

Both methods complete the hyperbolic example at rtol 1e-6 and 1e-8 in DiffSol and
pinned Julia. DiffSol's largest sampled error is **0.019427** original tolerance
units; Julia's is **0.021992**. Exact analytic values are the accuracy oracle.
Adaptive meshes and refinement pass counts still differ. All 606 completed
sampled rows from the six new checks are bit-identical between upstream DiffSol
and PharmFlux’s vendored shared implementation under identical driver settings.

Rodas5P completes both water targets in each implementation. Across these two
targets, DiffSol's largest disagreement from the refined Radau null-space
reference is **0.332328** tolerance units, and from the refined BDF null-space
reference **0.065968**. Julia's corresponding maxima are **0.327698** and
**0.065599**. These are agreement measurements against numerical references.
At the tighter target, Radau/BDF differ by up to **0.351080** tolerance units;
their own 1e-12-to-1e-13 refinements change by up to **0.303752** and **0.315953**.
Consequently this example does not support a true-error bound of 0.07 units.
Both references and both solver trajectories remain within the original
one-unit acceptance gate on the sampled comparison.

Original undifferentiated node-balance residuals stay below **5.22e-15 m³/s** in
DiffSol and **5.12e-15 m³/s** in Julia. Colebrook residuals stay below
**4.71e-13** and **3.95e-13**, respectively. Friction factors remain positive.
These checks independently verify that the differentiation did not introduce
appreciable drift from the original conservation constraints on this run.

Water Rosenbrock23 remains unresolved at both targets: DiffSol reaches a
250000 accepted-step diagnostic cap; Julia returns `Unstable` in its first
pass. An earlier longer DiffSol diagnostic was interrupted after both Rodas5P
targets completed, then rerun with that explicit cap; interruption is not
reported as numerical failure. These failures are retained, not qualified.
[agreement.json](agreement.json) contains all completed cases, uncertainty,
constraint diagnostics and unsuccessful attempts.

## Reproduction

From the parent evidence directory, use the retained locked environment:

```sh
VALIDATION_SOURCE_GAPS=1 VALIDATION_PROBLEM=hyperbolic python run-rust.py --kind additional --diffsol /path/to/diffsol
VALIDATION_SOURCE_GAPS=1 VALIDATION_PROBLEM=water_index1 python run-rust.py --kind additional --diffsol /path/to/diffsol
VALIDATION_SOURCE_DERIVATIVES=1 python run-rust.py --kind additional --diffsol /path/to/diffsol
python source-gaps/reference.py
python source-gaps/reference.py --method BDF
julia --project=. source-gaps/julia.jl
python source-gaps/analyze.py
```

Use an installed SciPy environment and a writable Julia depot containing the
pinned packages. `VALIDATION_METHOD` and `VALIDATION_TARGET` isolate diagnostic
runs. Rust limits each refinement pass to five million accepted steps; Julia
limits attempted steps. These are work limits, not equal-cost benchmarks.
The short water Rosenbrock23 diagnostic additionally sets
`VALIDATION_METHOD=rosenbrock23 VALIDATION_STEP_CAP=250000`.
Local baselines, completed sampled runs, failures, reference uncertainty and
original-constraint drift are retained in the CSV/logs and analysis report.
