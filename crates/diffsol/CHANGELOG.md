# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.18.0](https://github.com/unibio-intelligence/diffsol/compare/diffsol-v0.17.1...diffsol-v0.18.0) - 2026-10-26

### Added

- *(diffsol-la)* generic cuda-oxide kernels ([#381](https://github.com/unibio-intelligence/diffsol/pull/381))
- *(docs.rs)* remove llvm feature on docs build to fix OOM ([#379](https://github.com/unibio-intelligence/diffsol/pull/379))
- *(diffsol-la)* [**breaking**] for_each_batch, for_each_elem + device/host variants ([#375](https://github.com/unibio-intelligence/diffsol/pull/375))
- *(diffsol-la)* new cuda-oxide LA backend ([#374](https://github.com/unibio-intelligence/diffsol/pull/374))
- *(diffsol)* [**breaking**] sensitivity/adjoints calculated by batching ([#373](https://github.com/unibio-intelligence/diffsol/pull/373))
- *(diffsol-la)* [**breaking**] allow grouped batching B -> C * B, owned side has priority, then lhs ([#370](https://github.com/unibio-intelligence/diffsol/pull/370))
- *(diffsol)* [**breaking**] refactor Tableau and Bdf to use new SmallVec and SmallMat ([#369](https://github.com/unibio-intelligence/diffsol/pull/369))
- *(diffsol-la)* add batching for cpu backends ([#361](https://github.com/unibio-intelligence/diffsol/pull/361))
- autodiff closures and examples ([#302](https://github.com/unibio-intelligence/diffsol/pull/302))
- add discrete event performance example and book section ([#338](https://github.com/unibio-intelligence/diffsol/pull/338))
- add pi step size controller ([#330](https://github.com/unibio-intelligence/diffsol/pull/330))
- move crates under crates root dir ([#328](https://github.com/unibio-intelligence/diffsol/pull/328))
- expose diffsl external dynamic feature ([#262](https://github.com/unibio-intelligence/diffsol/pull/262))
- add model index to diffsl, refactor state_mut_op ([#249](https://github.com/unibio-intelligence/diffsol/pull/249))
- add helper sensitivities and adjoint functions to method trait ([#101](https://github.com/unibio-intelligence/diffsol/pull/101))
- add c sundials benchmarks ([#67](https://github.com/unibio-intelligence/diffsol/pull/67))

### Fixed

- publish with cuda-oxide feature ([#378](https://github.com/unibio-intelligence/diffsol/pull/378))
- use black_blox to prevent benchmark over-optimisation ([#359](https://github.com/unibio-intelligence/diffsol/pull/359))
- use parameter-scaled sens_atol for forward sensitivity ([#339](https://github.com/unibio-intelligence/diffsol/pull/339))
- consistent capitalisation of diffsol ([#336](https://github.com/unibio-intelligence/diffsol/pull/336))
- fix readme example

### Other

- release ([#380](https://github.com/unibio-intelligence/diffsol/pull/380))
- release ([#360](https://github.com/unibio-intelligence/diffsol/pull/360))
- examples for cpu + gpu parallel ensembles, example for batching and book sections ([#377](https://github.com/unibio-intelligence/diffsol/pull/377))
- remove unused colog
- *(diffsol-la)* use native faer matmul for sparse gemv ([#367](https://github.com/unibio-intelligence/diffsol/pull/367))
- *(diffsol)* add a fast pow for common exponents in pi controller ([#365](https://github.com/unibio-intelligence/diffsol/pull/365))
- *(diffsol-la)* non-allocating nalgebra lu solver ([#364](https://github.com/unibio-intelligence/diffsol/pull/364))
- *(diffsol-la)* use new rustc algebraic floating point ops for reductions (feature gated for now) ([#363](https://github.com/unibio-intelligence/diffsol/pull/363))
- *(diffsol-la)* replace column_axpy kernel with fused backwards_diff_update ([#362](https://github.com/unibio-intelligence/diffsol/pull/362))
- release ([#349](https://github.com/unibio-intelligence/diffsol/pull/349))
- bdf uses LS param instead of NLS, always uses NewtonNonlinearSolver ([#332](https://github.com/unibio-intelligence/diffsol/pull/332))
- version 0.16.1
- move linear algebra traits to diffsol-la ([#325](https://github.com/unibio-intelligence/diffsol/pull/325))
- update README
- minor fixes ([#293](https://github.com/unibio-intelligence/diffsol/pull/293))
- update version in readme
- coverage ([#256](https://github.com/unibio-intelligence/diffsol/pull/256))
- add CITATION file ([#247](https://github.com/unibio-intelligence/diffsol/pull/247))
- update version num in readme
- update version num in readme
- update readme
- update diffsl to 0.6.1 ([#205](https://github.com/unibio-intelligence/diffsol/pull/205))
- version 0.8.0
- fix LLVM_SYS_XXX_PREFIX in readme
- more detailed README and adds CONTRIBUTING.md ([#200](https://github.com/unibio-intelligence/diffsol/pull/200))
- Update README with logo and badges
- update pydiffsol repo url
- add coverage job ([#157](https://github.com/unibio-intelligence/diffsol/pull/157))
- restructure book, move all code to examples ([#144](https://github.com/unibio-intelligence/diffsol/pull/144))
- update deps - diffsl v0.5.1 faer v0.22.6 ([#143](https://github.com/unibio-intelligence/diffsol/pull/143))
- add wrapper advert to readme ([#130](https://github.com/unibio-intelligence/diffsol/pull/130))
- updates to book and readme ([#117](https://github.com/unibio-intelligence/diffsol/pull/117))
- add forward sensitivity to readme feature list
- put some more examples in the docs, and document using custom structs for OdeSolverEquations
- #30 fix doc test
- #30 update readme
- update Readme.md
- Update README.md
- remove example from readme
- #23 fix readme
- #23 remove warning from readme
- #23 add some tests
- #19 fix readme
- #17 add ci badge
- #5 update readme
- #5 update readme
- more readme updates
- remove star
- add readme

## [0.17.1](https://github.com/martinjrobins/diffsol/compare/diffsol-v0.17.0...diffsol-v0.17.1) - 2026-09-28

### Added

- *(docs.rs)* remove llvm feature on docs build to fix OOM ([#379](https://github.com/martinjrobins/diffsol/pull/379))

## [0.17.0](https://github.com/martinjrobins/diffsol/compare/diffsol-v0.16.2...diffsol-v0.17.0) - 2026-09-21

### Added

- *(diffsol-la)* [**breaking**] for_each_batch, for_each_elem + device/host variants ([#375](https://github.com/martinjrobins/diffsol/pull/375))
- *(diffsol-la)* new cuda-oxide LA backend ([#374](https://github.com/martinjrobins/diffsol/pull/374))
- *(diffsol)* [**breaking**] sensitivity/adjoints calculated by batching ([#373](https://github.com/martinjrobins/diffsol/pull/373))
- *(diffsol-la)* [**breaking**] allow grouped batching B -> C * B, owned side has priority, then lhs ([#370](https://github.com/martinjrobins/diffsol/pull/370))
- *(diffsol)* [**breaking**] refactor Tableau and Bdf to use new SmallVec and SmallMat ([#369](https://github.com/martinjrobins/diffsol/pull/369))
- *(diffsol-la)* add batching for cpu backends ([#361](https://github.com/martinjrobins/diffsol/pull/361))

### Fixed

- use black_blox to prevent benchmark over-optimisation ([#359](https://github.com/martinjrobins/diffsol/pull/359))

### Other

- examples for cpu + gpu parallel ensembles, example for batching and book sections ([#377](https://github.com/martinjrobins/diffsol/pull/377))
- remove unused colog
- *(diffsol-la)* use native faer matmul for sparse gemv ([#367](https://github.com/martinjrobins/diffsol/pull/367))
- *(diffsol)* add a fast pow for common exponents in pi controller ([#365](https://github.com/martinjrobins/diffsol/pull/365))
- *(diffsol-la)* non-allocating nalgebra lu solver ([#364](https://github.com/martinjrobins/diffsol/pull/364))
- *(diffsol-la)* use new rustc algebraic floating point ops for reductions (feature gated for now) ([#363](https://github.com/martinjrobins/diffsol/pull/363))
- *(diffsol-la)* replace column_axpy kernel with fused backwards_diff_update ([#362](https://github.com/martinjrobins/diffsol/pull/362))

## [0.16.2](https://github.com/martinjrobins/diffsol/compare/diffsol-v0.16.1...diffsol-v0.16.2) - 2026-08-18

### Added

- autodiff closures and examples ([#302](https://github.com/martinjrobins/diffsol/pull/302))
- add discrete event performance example and book section ([#338](https://github.com/martinjrobins/diffsol/pull/338))

### Fixed

- use parameter-scaled sens_atol for forward sensitivity ([#339](https://github.com/martinjrobins/diffsol/pull/339))
- consistent capitalisation of diffsol ([#336](https://github.com/martinjrobins/diffsol/pull/336))

### Other

- bdf uses LS param instead of NLS, always uses NewtonNonlinearSolver ([#332](https://github.com/martinjrobins/diffsol/pull/332))
