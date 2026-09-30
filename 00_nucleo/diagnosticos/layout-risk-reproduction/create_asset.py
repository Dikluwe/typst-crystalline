"""Generate the deterministic two-color diagnostic image (not product code)."""
from pathlib import Path
from PIL import Image
im = Image.new("RGB", (80, 40), "red")
im.paste("blue", (40, 0, 80, 40))
im.save(Path(__file__).parent / "fixtures" / "wide.png")
