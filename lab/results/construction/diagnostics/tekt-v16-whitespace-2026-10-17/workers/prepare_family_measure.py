from pathlib import Path
import shutil
out=Path(__file__).resolve().parents[1];s=(out/'workers/measure_test.rs.txt').read_text();s=s.replace('for name in [','for carrier in ["source","located","realized","styled"] {for name in [',1)
s=s.replace('if source{middle=middle.with_source_span(span);}', '''if source{middle=match carrier {
 "source"=>middle.with_source_span(span),
 "located"=>Content::Located(std::sync::Arc::new(crate::entities::content::LocatedContent {body:middle,location:crate::entities::location::Location::from_raw(9)})),
 "realized"=>Content::Realized(std::sync::Arc::new(crate::entities::content::RealizedContent{original:Content::text("must-not-observe"),visual:middle})),
 "styled"=>Content::styled(Box::new(middle),crate::entities::style::Styles::default()),
 _=>panic!("measurement carrier"),};}''',1)
s=s.replace('Content::Empty=>"_".into(),','Content::Located(e)=>view(&e.body),Content::Realized(e)=>view(&e.visual),Content::Styled(e)=>view(&e.body),Content::Empty=>"_".into(),',1)
s=s.replace('println!("{name} source={source}', 'println!("{name} carrier={carrier} source={source}',1)
s=s.replace('\n  }}\n }','\n  }}}\n }',1);(out/'workers/measure_family_test.rs.txt').write_text(s)
w=(out/'workers/measure.py').read_text();w=w.replace("'workers/measure_test.rs.txt'","'workers/measure_family_test.rs.txt'").replace("'source-before.json.gz'","'source-family-before.json.gz'").replace("'measurement-before.log'","'measurement-family-before.log'").replace("'source-before-analysis.json'","'source-family-before-analysis.json'").replace('len(lines)==14','len(lines)==56');(out/'workers/measure_family.py').write_text(w)
