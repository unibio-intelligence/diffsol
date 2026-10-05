# Minimal Rosenbrock contribution evidence

These measurements bind to the candidate patch in `source-manifest.json` and
`agreement.json`. See `FINAL_AGREEMENT.md` and `PAPER_COVERAGE.md` for scope.
Run `python3 run-rust.py --diffsol /path/to/candidate --kind native` (or `paper`,
`parabolic`, `properties`) to reproduce the Rust outputs. The pinned Cargo lock
supports the offline runner. Julia inputs and retained outputs are included;
activate the included project to rerun its script in a qualified Julia environment.
Local build paths are removed from logs; numerical data are unmodified.
Independent scientific equations are authored as UniBio Intelligence work;
upstream/Julia coefficient fixtures retain their original provenance and licenses.
