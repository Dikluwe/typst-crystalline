"""Check that archived table benchmark receipts still match their fixtures."""

import gzip
import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parent
FIXTURES = ROOT / "fixtures"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    receipts = 0
    checks = 0
    errors = []
    for receipt in sorted(ROOT.glob("*.json.gz")):
        try:
            with gzip.open(receipt, "rt", encoding="utf-8") as stream:
                data = json.load(stream)
        except (OSError, ValueError) as error:
            errors.append(f"{receipt.name}: cannot read receipt: {error}")
            continue
        receipts += 1
        fixtures = data.get("fixtures", {})
        if not isinstance(fixtures, dict):
            continue
        for name, entry in fixtures.items():
            if not isinstance(entry, dict) or "sha256" not in entry:
                continue
            filename = Path(name).name
            path = FIXTURES / (filename if filename.endswith(".typ") else f"{filename}.typ")
            checks += 1
            if not path.is_file():
                errors.append(f"{receipt.name}: missing fixture {path.name}")
            elif sha256(path) != entry["sha256"]:
                errors.append(f"{receipt.name}: changed fixture {path.name}")
    print(f"receipts={receipts} fixture_hash_checks={checks} errors={len(errors)}")
    for error in errors:
        print(error)
    return bool(errors)


if __name__ == "__main__":
    raise SystemExit(main())
