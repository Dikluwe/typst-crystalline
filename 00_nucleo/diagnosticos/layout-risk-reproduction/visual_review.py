"""Render diagnostic PDFs without modifying them; keep renderer receipts."""
import run
cases = ["raw-spaces", "footnote-container-deferred", "footnote-container-flow-control", "footnote-page-last-deferred"]
receipts = []
for case in cases:
    for binary in ["crystalline", "vanilla"]:
        pdf = run.BASE / "outputs" / case / (binary+".pdf")
        if not pdf.exists(): continue
        output = pdf.with_name(binary+"-review-%d.png")
        receipts.append(run.call(["mutool", "draw", "-q", "-r", "144", "-o", output, pdf]))
run.save(run.BASE / "receipts" / "visual-review.json", receipts)
