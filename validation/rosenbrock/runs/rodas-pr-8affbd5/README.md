# rodas-pr-8affbd5 measured validation

Source commit: `8affbd5079e9c78ed66044814e001e877ac66777`. Source tree: `554b3a92486ac62fddb0b9053a44cab1bf1c951b`.

See [source identity and toolchain](source-pin.json), [source patch](source.patch),
[complete numerical comparisons](agreement.json), [gate results](gates/results.json)
and the [branch comparison](../../COMPARISON.md).

Fresh checks compare ten fixed-step and twenty adaptive cases with retained pinned Julia values. The maximum fixed-step absolute difference is 1.01655e-15; the maximum difference from fourteen printed paper errors is 9.04648%.

The `native`, `paper`, `parabolic` and `properties` CSV/log pairs preserve all output. Each `*-run.json` records the source, driver hash, settings, elapsed time, exit status and output hashes. The polynomial/dense extension limitations remain in the results.

Fresh gates pass formatting, strict Clippy, strict rustdoc, and 302 tests plus four doctests for default, faer and nalgebra configurations. Transitive dependencies enable both matrix backends, so feature flags do not prove exclusive backend isolation.

The extended DAE/refinement/product APIs were intentionally removed from this PR source. Its results do not inherit RC2 or dev expanded-suite counts. The complete historical reduced-package measurements are retained separately.
