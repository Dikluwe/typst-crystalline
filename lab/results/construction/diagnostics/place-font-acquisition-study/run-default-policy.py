from pathlib import Path
import run
run.R=Path(__file__).resolve().parent/'default-policy'
run.main()
