from pathlib import Path
import hashlib, subprocess, sys
product=Path('lab/target/product/debug/typst')
raise SystemExit(subprocess.call(['python3','lab/results/v16-corrections-2026-10-19/batch-c/agents/effects/replay-candidate.py','--product-sha256',hashlib.sha256(product.read_bytes()).hexdigest(),'--product-build-receipt',sys.argv[1],'--output',sys.argv[2]]))
