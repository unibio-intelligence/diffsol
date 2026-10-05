# UniBio DiffSol numerical validation

This directory retains the full validation evidence in UniBio's own DiffSol
fork. It is separate from the reduced upstream Rodas contribution and its unit
suite. This collection includes successful, failed and incomplete measurements.
Publication-status fields in retained receipts describe their collection-time
state; the measurements and historical records are preserved.

Start with [the branch comparison](COMPARISON.md). Each measurement records the
source commit and tree, input driver, settings, CSV output, diagnostic log and
checksums. Failed and incomplete runs remain in the collection.

| Collection | Source | Contents |
|---|---|---|
| [Current Rodas PR candidate](runs/rodas-pr-8affbd5/README.md) | `8affbd5079e9c78ed66044814e001e877ac66777` | Fresh common numerical checks and current default/backend gates |
| [Current dev implementation](runs/dev-e553754/README.md) | `e553754e5b984529864c044611f714f312c37329` | Fresh common checks, expanded standard-budget checks, dense sampling, refinement and observation endpoints; retained exact-source library gates |
| [Full historical extended evidence](historical/extended-rc2-2026-10-05/accuracy-2026-10-04/FINAL_AGREEMENT.md) | RC2 patch `6428eb759a9fd56abcef7a5d2f8d68df1121e133146bbe60a8fde6afc741ed50` | Complete investigations, larger-budget runs, Julia/independent references, product integration evidence and failures |
| [Historical minimal package](historical/minimal-package-2026-10-05/FINAL_AGREEMENT.md) | Minimal patch `f1fd8f0f5b3fa6be763532a9f919e5c1ad6f240e8e34b58d395ab8a0a8299dd5` | Earlier reduced-candidate measurements and source pin |

The historical folders preserve their original bytes and manifests. Statements
about source state, test counts and numerical results inside them describe
those earlier measurements. They are not fresh measurements of either current
source. The current comparison and source-specific reports take precedence.

The eleven-reference bibliography comprises ten papers and one textbook.
[Reference coverage](historical/extended-rc2-2026-10-05/accuracy-2026-10-04/PAPER_COVERAGE.md)
identifies selected reproductions, method/software checks and background sources;
it does not claim full reproduction of all eleven references. The exact published
water-tube index-1 adaptation remains unavailable. No author request was sent.

## Inspection and reproduction

The PR source is retained locally as the tag `validation/rodas-pr-2026-10-05`;
the dev source is an ancestor of the evidence commit on `dev`. This preserves
both revisions while keeping only `main` and `dev` as active branches. Each run
also includes a patch against the recorded upstream base for independent review.
The locally imported history begins at that base as an explicit shallow boundary.

Check file integrity first:

```sh
python3 validation/rosenbrock/verify.py
```

Use separate checkouts at the recorded source commits. For an independent run,
write into a fresh output directory; retained evidence is never overwritten:

```sh
python3 validation/rosenbrock/run.py --source /path/to/pr-checkout \
  --snapshot rodas-pr-8affbd5 --target-dir /tmp/pr-target --output-dir /tmp/pr-results
python3 validation/rosenbrock/run.py --source /path/to/dev-checkout \
  --snapshot dev-e553754 --target-dir /tmp/dev-target --extended --output-dir /tmp/dev-results
```

Rust drivers use the retained Cargo lockfile and offline dependency resolution.
Prepopulate a Cargo cache before running in an offline environment. Julia
reference CSVs were retained from Julia 1.12.6 / OrdinaryDiffEqRosenbrock 2.7.1;
Julia was not rerun for these fresh Rust measurements. Its drivers and pinned
Project/Manifest files remain in the historical archive. Expanded analyses use
NumPy; reference generation additionally uses SciPy and mpmath. To regenerate
the retained comparison reports with those dependencies installed, run
`python3 validation/rosenbrock/analyze.py`. The archive retains the original
per-experiment analyzers and reproduction instructions for larger-budget work.

Keep this directory out of the upstream patch. The upstream candidate is the
recorded PR source, not the complete fork's `dev` diff. After approved publication
in the UniBio fork, the maintainer can inspect this index, both source reports,
raw files and historical investigations without adding them to the upstream PR.

Ubi-authored validation contributions retain Apache-2.0 licensing; DiffSol and
SciML material retain their respective terms. See the retained
[NOTICE](historical/extended-rc2-2026-10-05/NOTICE) and
[validation license](historical/extended-rc2-2026-10-05/LICENSE-APACHE).
