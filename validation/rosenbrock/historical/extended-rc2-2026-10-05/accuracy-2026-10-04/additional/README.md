> These original measurements use the pre-correction candidate. See the
> [current unresolved-run follow-up](../unresolved-followup/README.md) for the
> corrected DAE estimator, fresh standard-budget runs and larger-budget results.

# Additional published-problem validation

Executed 2026-10-04 against the uncommitted DiffSol candidate. These independently
written drivers exercise scientific equations; they do not copy book drivers or
publisher figures. Evidence publication and Fable review remain pending.

| Example | Specification and reference | Current result |
|---|---|---|
| HIRES, 8 states, end 321.8122 | [Hairer–Wanner test set](https://www.unige.ch/~hairer/testset/testset.html); independently refined SciPy Radau ODE | Both methods converge at requested rtol 1e-6 and 1e-8; final two-state sum checked |
| Oregonator, 3 states, end 360 | [Book test set](https://www.unige.ch/~hairer/testset/testset.html); independently refined Radau | Rodas5P both targets, Rosenbrock23 1e-6 pass; 1e-8 Rosenbrock23 reaches the step cap in both implementations |
| Transistor amplifier, 8 states, end .05 | [Author's index-1 equations](https://www.unige.ch/~hairer/prog/stiff/dr2_radau5.f); reference analytically eliminates capacitor constraints with Wright omega | Rodas5P both targets and DiffSol Rosenbrock23 1e-6 pass; tighter Rosenbrock23 unresolved; Julia Rosenbrock23 explicitly excludes non-diagonal mass |
| Plane pendulum, 5 states, end 10 | [Steinebach §4](https://link.springer.com/article/10.1007/s10543-023-00967-x), index-1 formulation; independently integrated angular ODE | Rodas5P both targets pass; DiffSol Rosenbrock23 refinement reaches step cap. Position/velocity drift retained |
| Circle, exact/inexact Jacobian, end 1 | Steinebach Problem 5; analytic sin/cos | Ten fixed-step error diagnostics match Julia; inexact Jacobian reduces order to about two. Tight adaptive inexact checks are limited and not claimed qualified |
| Pollution, 20 states, end 60 | [SciML benchmark equations](https://github.com/SciML/SciMLBenchmarks.jl/blob/2ac54a8200771e74bab9144851885cb12a8aa8dd/benchmarks/StiffODE/Pollution.jmd); independently refined Radau | Both methods pass both targets |
| PV network, 7 states, end 36000 | Steinebach equations/Appendix constants; independently eliminated high-voltage electrical constraints with scalar root finding plus Radau | Rodas5P both targets pass with progressively refined forcing step cap; DiffSol Rosenbrock23 encounters repeated error failures |
| Hyperbolic discretization | Original cited specification recovered from author-hosted 1986 paper | Both methods pass both targets against the analytic solution; [new evidence](../source-gaps/README.md) |
| Water tubes | Complete original system recovered; independently derived index-1 adaptation | Rodas5P passes both targets against independently eliminated Radau/BDF references; Rosenbrock23 unresolved. Exact 2023 adaptation remains unverified; [new evidence](../source-gaps/README.md) |

`reference.py` repeats Radau at rtol 1e-11 and 1e-13 (atol 0.001*rtol).
`references.json` includes the refinement delta, every time and component.
The numerical references are converged approximations, not exact solutions.
The amplifier uses exact pi rather than the original Fortran's rounded pi;
the pendulum declares g=9.81, L=2. PV uses c5=.4303 and c6=1.5*.9562, as printed
in the Appendix, with uB(0)=0 and qB(0)=9000. Julia's PV algebraic initial values
are recomputed to remove the published starting vector's rounding residual.

The full constant singular mass matrix is integrated for the amplifier; it is
not replaced by an ODE in DiffSol or Julia. Its independent reference uses five
capacitor coordinates. The pendulum reference uses two angular coordinates;
DiffSol/Julia integrate the five-state index-1 DAE. The electrical reference
eliminates five algebraic variables and integrates the battery's two states.

Ordinary local runs are retained as baselines. Sampled refinement uses the shared
API, eight passes, and a 0.25-original-scale coarse/fine agreement criterion.
An independent check asserts that each completed sampled run is within one
original tolerance unit. Each DiffSol pass has a 5,000,000 accepted-step cap;
Julia uses a 500,000 attempted-step cap for Rodas5P and 5,000,000 for
Rosenbrock23. These are diagnostic work limits, not equal-cost benchmarks.
For PV, the maximum step is `60*sqrt(factor)` seconds; exact f_t is supplied for
forced examples. No paper CPU/work-precision curves are reproduced.

[comparison.json](comparison.json) records all denominators, completed adaptive
comparisons, ten circle fixed diagnostics, source hash, failures and source gaps.
[agreement.json](agreement.json) and [julia-agreement.json](julia-agreement.json)
retain baseline error, convergence, work and available constraint measurements.
All 23 completed DiffSol sampled checks and all 23 completed Julia sampled checks
are below one original tolerance unit. Completion counts deliberately exclude
failed checks; they do not support a universal correctness claim.

Reproduce from the parent evidence directory with `run-rust.py --kind additional`
and the pinned Julia Project/Manifest. Use `VALIDATION_CIRCLE_ORDERS=1` for the
fixed-step diagnostic or `VALIDATION_PROBLEM=pv` to isolate a problem. Run
`additional/reference.py`, `additional/julia.jl`, `additional/circle-julia.jl`,
then `additional/analyze.py` (and `additional/analyze.py julia`). Run from a
writable Julia depot with the pinned packages already installed.
