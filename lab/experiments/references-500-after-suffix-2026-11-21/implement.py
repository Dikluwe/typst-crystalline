subprocess.run([
    sys.executable, str(exp.parent/'references-500-context-index-retention-2026-11-21'/'prototype.py'),
    str(source),
], check=True)
