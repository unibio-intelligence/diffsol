# Coverage of the eleven cited references

Prepared 2026-10-04. The bibliography has **ten papers and one textbook**.
**No complete paper or textbook is claimed to be reproduced.** A citation may
identify method provenance, mathematical context, software, or future work.
Those roles do not imply that its experiments were rerun.

| # | Reference | Current evidence | Reproduction limit |
|---|---|---|---|
| 1 | [Rosenbrock (1963)](https://doi.org/10.1093/comjnl/5.4.329) | Historical foundation of the solver family | Original method and paper experiments not reproduced |
| 2 | [Kaps & Rentrop (1979)](https://doi.org/10.1007/BF01396495) | Background for transformed stages and embedded methods | GRK4A/GRK4T and their benchmarks not implemented or reproduced in this contribution |
| 3 | [Hairer & Wanner (1996), *Solving Ordinary Differential Equations II*](https://doi.org/10.1007/978-3-642-05221-7) — textbook | Stage/DAE framework; independent stage, stability and constant-mass index-1 tests | No book-wide reproduction or claim to have verified every order condition |
| 4 | [Steihaug & Wolfbrandt (1979)](https://doi.org/10.1090/S0025-5718-1979-0521273-8) | W-method/approximate-Jacobian context | No dedicated reproduction of its methods or approximate-Jacobian experiments |
| 5 | [Steinebach (2023), Rodas5P](https://doi.org/10.1007/s10543-023-00967-x) | **Selected numerical reproduction:** Rodas5P rows from Tables 5–8, polynomial dense-output diagnostic from Problem 6/Table 9, coefficients and selected stability values | Not the whole paper: no full multi-method tables, eight-problem work-precision suite, timing curves or complete coefficient derivation |
| 6 | [Lang & Verwer (2001), ROS3P](https://doi.org/10.1023/A:1021900219772) | Parabolic order-reduction context | ROS3P is not implemented; its paper's separate experiments are not reproduced |
| 7 | [Rang & Angermann (2005)](https://doi.org/10.1007/s10543-005-0035-y) | Index-1 partial differential-algebraic method context | Its methods and full numerical suite are not reproduced |
| 8 | [Shampine & Reichelt (1997), MATLAB ODE Suite](https://doi.org/10.1137/S1064827594276424) | **Method checks:** Rosenbrock23 coefficients, independent direct-stage arithmetic, cubic estimator and linear-forcing discriminator | No full MATLAB solver-suite reproduction, paper tables or timing results |
| 9 | [Rackauckas & Nie (2017), DifferentialEquations.jl](https://doi.org/10.5334/jors.151) | **Software comparison:** pinned Julia 1.12.6 / OrdinaryDiffEqRosenbrock 2.7.1 fixed/adaptive, coefficient and stability checks | Current software checks do not reproduce the 2017 paper's experiments or historical release |
| 10 | [Sandu, Daescu & Carmichael (2003), KPP Part I](https://doi.org/10.1016/j.atmosenv.2003.08.019) | Sensitivity theory and deferred design context | No KPP sensitivity-method reproduction; native upstream Rosenbrock sensitivities are outside this candidate's scope |
| 11 | [Daescu, Sandu & Carmichael (2003), KPP Part II](https://doi.org/10.1016/j.atmosenv.2003.08.020) | Sensitivity validation/application context for deferred work | Chemical applications, forward/adjoint comparisons and paper results not reproduced |

The repeated parabolic example in our evidence is the one specified by
Steinebach. It does not qualify the separate Lang–Verwer or Rang–Angermann
papers. PharmFlux's augmented-equation sensitivity integration does not qualify
KPP's direct/discrete-adjoint methods. TR-BDF2's Hosea–Shampine citation is an
additional reference outside this eleven-item bibliography.

## Evidence that can be rerun

- [Final numerical agreement](FINAL_AGREEMENT.md) and
  [all comparisons](final-agreement.json): ten Julia fixed-step cases agree
  within 3.47e-16 absolute; fourteen selected printed paper errors agree within
  0.351%. These are rounded-error comparisons, not percentages of solution error.
- [Paper driver](paper.rs), [parabolic driver](parabolic.rs),
  [polynomial/stability driver](properties.rs), [Julia checks](julia-reference.jl)
  and pinned lockfiles retain the reproduction inputs.
- [Adaptive refinement experiment](adaptive-refinement.json): twenty tested
  runs converge below 0.06 original tolerance units against analytic solutions
  or tight BDF comparators. The shared routine is now reused by explicit sampled production control; it is
  not a universal true-global-error bound.
- [Index-1 DAE sampling experiment](dense-accuracy.json): maximum-step reduction
  and observation endpoints improve polynomial sampled accuracy using the
  existing shared solver. This does not change its formal extension order.

## What the remaining limitations mean

**Adaptive Julia differences.** Matching fixed steps isolates the stage formula;
matching adaptive outputs also depends on initialization, scaling and controller
policy. Identical histories are not required for correctness. Refined sampled
accuracy is attainable, but the validated experiment costs up to 616 times the
original accepted-step work for Rosenbrock23. The explicit sampled production mode now includes shared work budgets,
convergence checking and failure tests.

**Parabolic order reduction.** The original example reproduces the source's
reduced order. Equivalent boundary lifting improves errors 4.32–16.29 times and
observed order from 3.66–3.87 to 4.23–4.56. This is a problem formulation change
demonstrated in the driver, not a generic solver transformation or proof of
uniform fifth order.

**Fourth-order dense output.** Rodas5P's published extension has order four;
[SciML documents the same order](https://docs.sciml.ai/OrdinaryDiffEq/stable/massmatrixdae/Rosenbrock/).
Smaller maximum steps and observation endpoints control sampled error now. A
fifth-order extension needs new ODE/index-1 DAE order conditions and independent
validation; increasing a polynomial's degree alone does not establish its order.

**Scope correction.** The polynomial DAE is **index-1**, not higher-index:
its algebraic constraint is y0-y1=0 and its derivative with respect to y1 is -1.
It is within the supported equation class; the interpolation limitation must
remain in that class's accuracy statement. Table 8's different index-2 DAE is
the out-of-scope diagnostic. Earlier contrary wording is corrected in the
current report; historical measurements are unchanged.

Use the claim **“selected published-example reproduction and independent
numerical validation”**, not “all eleven papers reproduced.” Further paper
reproduction would need an explicit result inventory and executable comparator
for each targeted table/figure; adding citations does not supply that evidence.

The [additional example report](additional/README.md) records seven named
problems, completed Julia/Radau comparisons and failed targets. The
[recovered-source supplement](source-gaps/README.md) adds the cited 250-point
hyperbolic example and an independently reduced 49-variable water-tube example.
The paper’s exact water adaptation remains unverified. Expanded problem coverage
does not imply full paper reproduction.

The [2026-10-05 unresolved-run investigation](unresolved-followup/README.md)
adds separate diagonal-DAE error control for Rosenbrock23 and larger-budget
checks. The author's [2025 preprint, equation (18)](https://arxiv.org/html/2511.21252v1#S4)
corroborates the hyperbolic specification. It is an additional scientific source,
not a reproduction of that preprint's new methods. The exact published water
adaptation remains unverified.
