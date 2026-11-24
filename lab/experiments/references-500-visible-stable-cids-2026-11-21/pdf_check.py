"""Audit PDF semantics while allowing a different CID allocation."""
import base64
import hashlib
import json
import re
import subprocess


FONT_REFS = ("/DescendantFonts", "/FontDescriptor", "/ToUnicode",
             "/FontFile", "/FontFile2", "/FontFile3", "/CIDSet")
XMP_IDS = rb"(<xmpMM:(?:Instance|Document)ID>)[A-Za-z0-9+/=]+(</xmpMM:(?:Instance|Document)ID>)"


def digest(data):
    return hashlib.sha256(data).hexdigest()


def qpdf(pdf, *args):
    return subprocess.check_output(["qpdf", *args, str(pdf)], timeout=35)


def objects(pdf, raw=False):
    options = ["--json", "--json-key=qpdf", "--json-key=pages"]
    if raw:
        options.extend(["--decode-level=none", "--json-stream-data=inline"])
    else:
        options.append("--json-stream-data=inline")
    return json.loads(qpdf(pdf, *options))


def structure(pdf):
    data = objects(pdf)
    contents = data["qpdf"][-1]
    special = {ref for page in data["pages"] for ref in page["contents"]}
    for ref, obj in contents.items():
        values = obj.get("value")
        if isinstance(values, dict) and values.get("/Type") in ("/Font", "/FontDescriptor"):
            special.add(ref[4:])
            for key in FONT_REFS:
                field = values.get(key)
                if isinstance(field, str):
                    special.add(field)
                elif isinstance(field, list):
                    special.update(field)
    assert special, pdf

    for ref in special:
        contents.pop("obj:" + ref)
    contents["trailer"]["value"].pop("/ID", None)
    for obj in contents.values():
        stream = obj.get("stream", {})
        if stream.get("dict", {}).get("/Type") == "/Metadata":
            xml = base64.b64decode(stream["data"])
            xml, count = re.subn(XMP_IDS, rb"\1normalized\2", xml)
            assert count == 2, (pdf, count)
            stream["data"] = base64.b64encode(xml).decode()
    return digest(json.dumps(data, sort_keys=True).encode())


def text_layout(pdf):
    output = subprocess.check_output(["pdftotext", "-bbox-layout", str(pdf), "-"], timeout=20)
    assert b"<word " in output, pdf
    return digest(output)


def render(pdf, directory, tag):
    subprocess.run(["qpdf", "--check", str(pdf)], check=True, capture_output=True, timeout=20)
    subprocess.run(["pdftoppm", "-gray", "-r", "72", str(pdf), str(directory/tag)],
                   check=True, timeout=30)
    pages = sorted(directory.glob(tag + "-*.pgm"))
    assert pages
    return [digest(page.read_bytes()) for page in pages]


def compressed_pages(pdf):
    data = objects(pdf, raw=True)
    contents = data["qpdf"][-1]
    out = []
    for page in data["pages"]:
        refs = page["contents"]
        assert len(refs) == 1
        stream = contents["obj:" + refs[0]]["stream"]
        assert stream["dict"]["/Filter"] == "/FlateDecode"
        out.append(digest(base64.b64decode(stream["data"])))
    return out
