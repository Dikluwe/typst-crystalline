from pathlib import Path
import gzip,json,subprocess
out=Path(__file__).resolve().parents[1]
def expected():
 b=json.load(gzip.open(out/'before.json.gz'));s=b['files']['03_infra/src/font_metrics/math.rs'];old='_ => None, // neutro: N16[β] — MathSize não-Script/ScriptScript retorna None para ssty level';assert s.count(old)==1;s=s.replace(old,'MathSize::Display | MathSize::Text => None,',1);t=b['files']['03_infra/src/shaper/shaping.rs'];assert t.count('layout_types::{FrameItem, Point, Pt, ShapedGlyph, TextStyle}')==1;t=t.replace('layout_types::{FrameItem, Point, Pt, ShapedGlyph, TextStyle}','layout_types::{FrameItem, MathSize, Point, Pt, ShapedGlyph, TextStyle}',1);a=t.index('pub(super) fn ssty_level_for_subrun');z=t.index('\n}\n',a)+2;part=t[a:z];assert part.count('_ => None,')==1;part=part.replace('typst_core::entities::layout_types::MathSize::','MathSize::').replace('_ => None,','MathSize::Display | MathSize::Text => None,');t=t[:a]+part+t[z:]
 return {p:subprocess.run(['rustfmt','--edition','2021','--emit','stdout'],input=text,capture_output=True,text=True,check=True).stdout for p,text in [('03_infra/src/font_metrics/math.rs',s),('03_infra/src/shaper/shaping.rs',t)]}
