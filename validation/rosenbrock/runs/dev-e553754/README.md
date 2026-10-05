# dev-e553754 measured validation

Source commit: `e553754e5b984529864c044611f714f312c37329`. Source tree: `1656884009fb87ed1015fe4ad4617c8d076f09aa`.

See [source identity and toolchain](source-pin.json), [source patch](source.patch),
[complete numerical comparisons](agreement.json), [gate results](gates/results.json)
and the [branch comparison](../../COMPARISON.md).

Fresh checks compare ten fixed-step and twenty adaptive cases with retained pinned Julia values. The maximum fixed-step absolute difference is 4.44089e-16; the maximum difference from fourteen printed paper errors is 0.35047%.

The `native`, `paper`, `parabolic` and `properties` CSV/log pairs preserve all output. Each `*-run.json` records the source, driver hash, settings, elapsed time, exit status and output hashes. The polynomial/dense extension limitations remain in the results.

The `additional-standard` suite completes 24/32 checks under five million accepted steps per pass and eight passes. The `source-gaps` suite completes 6/8; both Rosenbrock23 water targets return `StepSizeTooSmall`. Individual failures are in the logs even when the driver process exits zero.

The twenty-case `adaptive-refinement` experiment finishes below one original tolerance unit in all cases; the worst error is approximately 0.059006. The dense sampling study and optional circle observation endpoints are separate experiments. The endpoint-sampled Rodas5P circle targets pass with worst analytic error approximately 0.004264 tolerance units; the endpoint-local rows remain visible as their unrefined baseline.

Full larger-budget investigations and references are retained under `../../historical/extended-rc2-2026-10-05/`. Their 35/40 outcome is historical, not a fresh measurement of this commit.

Library, Clippy and formatting logs are retained from the canonical consolidation on this exact source; product solver checks are labelled separately and excluded from numerical solver denominators.
