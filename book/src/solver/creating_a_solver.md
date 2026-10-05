# Creating a solver

Once you have defined the problem, you need to create a solver to solve the problem. The available solvers are:
- [`diffsol::Bdf`](https://docs.rs/diffsol/latest/diffsol/ode_solver/bdf/struct.Bdf.html): A Backwards Difference Formulae solver, suitable for stiff problems and singular mass matrices.
- [`diffsol::Sdirk`](https://docs.rs/diffsol/latest/diffsol/ode_solver/sdirk/struct.Sdirk.html) A Singly Diagonally Implicit Runge-Kutta (SDIRK or ESDIRK) solver. You can define your own butcher tableau using [`Tableau`](https://docs.rs/diffsol/latest/diffsol/ode_solver/tableau/struct.Tableau.html) or use one of the pre-defined tableaues.
- [`diffsol::ExplicitRk`](https://docs.rs/diffsol/latest/diffsol/ode_solver/explicit_rk/struct.ExplicitRk.html): An explicit Runge-Kutta solver. You can define your own butcher tableau using [`Tableau`](https://docs.rs/diffsol/latest/diffsol/ode_solver/tableau/struct.Tableau.html) or use one of the pre-defined tableaues.
- `diffsol::Rosenbrock23`: A compatibility alias for the shared `Rosenbrock` solver. Construct it with `problem.rosenbrock23::<LS>()?` to select the second-order method with a third-order embedded estimate. It supports stiff ODEs, constant-mass index-1 DAEs, and integrated outputs (global order two).
- `diffsol::Rodas5P`: A compatibility alias for the shared `Rosenbrock` solver. Construct it with `problem.rodas5p::<LS>()?` to select the fifth-order Rodas5P tableau. It supports stiff ODEs, constant-mass index-1 DAEs, and integrated outputs (global order four). Both aliases describe the same Rust type; the selected tableau determines the method.

For each solver, you will need to specify the linear solver type to use. The available linear solvers are:
- [`diffsol::NalgebraLU`](https://docs.rs/diffsol/latest/diffsol/linear_solver/nalgebra_lu/struct.NalgebraLU.html): A LU decomposition solver using the [nalgebra](https://nalgebra.org) crate.
- [`diffsol::FaerLU`](https://docs.rs/diffsol/latest/diffsol/linear_solver/faer_lu/struct.FaerLU.html): A LU decomposition solver using the [faer](https://github.com/sarah-ek/faer-rs) crate.
- [`diffsol::FaerSparseLU`](https://docs.rs/diffsol/latest/diffsol/linear_solver/faer_sparse_lu/struct.FaerSparseLU.html): A sparse LU decomposition solver using the `faer` crate.

Each solver can be created directly, but it generally easier to use the methods on the [`OdeSolverProblem`](https://docs.rs/diffsol/latest/diffsol/ode_solver/problem/struct.OdeSolverProblem.html) struct to create the solver.
For an implicit ODE, construct Rosenbrock23 with `problem.rosenbrock23::<NalgebraLU<f64>>()?` after building the problem. Use `solver.set_discontinuity_stop_time(t)?` before a known forcing jump; ordinary output times use `solver.set_stop_time(t)?`.

Both Rosenbrock methods require a right-hand-side Jacobian action. Mass matrices must be constant, and DAEs must be index one. Mass-matrix problems and integrated outputs require a tableau with a continuous extension, as supplied by both built-in methods. Forward and adjoint sensitivities are not implemented for the Rosenbrock solver class. For Rosenbrock23 DAEs, scale algebraic RHS rows consistently with the absolute tolerances; endpoint residual control does not project interpolated states onto the algebraic constraints.

Use `set_time_derivative_within_step(true)` to keep numerical time-derivative probes inside each attempted step for piecewise forcing. An analytic time partial supplied through `NonLinearOp::time_partial_inplace` takes precedence. `set_maximum_step(h)?` bounds the absolute step size; it supplements local tolerances rather than guaranteeing global trajectory accuracy.

The optional `refine_sampled_trajectory` helper compares repeated solves at caller-selected times. Agreement is sampled convergence evidence, not a certified true-error bound. The caller must restart identical intervals and share its work budget across refinement passes.

For example:

```rust,ignore
{{#include ../../../examples/intro-logistic-closures/src/create_solvers.rs}}
```
