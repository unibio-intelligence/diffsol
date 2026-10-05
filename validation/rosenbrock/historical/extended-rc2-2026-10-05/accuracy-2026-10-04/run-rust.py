#!/usr/bin/env python3
"""Reproduce native, literature, or parabolic evidence against a local Diffsol candidate."""
import argparse, json, os, shutil, subprocess, tempfile
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--diffsol',type=Path,required=True);p.add_argument('--kind',choices=['native','paper','parabolic','properties','dense-accuracy','adaptive-refinement','additional'],required=True);p.add_argument('--target-dir',type=Path);a=p.parse_args()
source=Path(__file__).resolve().parent
env=os.environ.copy()
if a.target_dir: env['CARGO_TARGET_DIR']=str(a.target_dir.resolve())
with tempfile.TemporaryDirectory(prefix='diffsol-reference-') as tmp:
 root=Path(tmp);(root/'src').mkdir();shutil.copyfile(source/(a.kind+'.rs'),root/'src/main.rs');shutil.copyfile(source/'source_gaps.rs',root/'src/source_gaps.rs');shutil.copyfile(source/'Cargo.lock',root/'Cargo.lock')
 (root/'Cargo.toml').write_text('[package]\nname="diffsol-pr-a-scientific"\nversion="0.0.0"\nedition="2021"\n[dependencies]\ndiffsol={path='+json.dumps(str(a.diffsol.resolve()/'crates/diffsol'))+'}\n')
 subprocess.run(['cargo','run','--release','--offline','--locked','--manifest-path',str(root/'Cargo.toml')],env=env,check=True)
