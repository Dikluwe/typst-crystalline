from pathlib import Path
import json,datetime
out=Path(__file__).resolve().parents[1];assert json.load(open(out/'red-caption-resources.json'))['exit']==101;assert '0 passed; 1 failed;' in (out/'red-caption.log').read_text()
p=Path('01_core/src/entities/elements/figure.rs');s=p.read_text();pos=s.index('    fn element_kind(&self)');s=s[:pos]+'''    fn get_field(&self, field: &str) -> Option<crate::entities::value::Value> {
        use crate::entities::value::Value;
        match field {
            "body" => Some(Value::Content(self.body.clone())),
            "caption" => self.caption.as_ref().map(|body| {
                Value::Content(Content::figure_caption(body.clone(), None, None))
            }),
            _ => None,
        }
    }

'''+s[pos:];p.write_text(s)
p=Path('01_core/src/entities/content/fields.rs');s=p.read_text();assert '(Content::Figure(e), "body") => Some(Value::Content(e.body.clone())),' in s;s=s.replace('(Content::Figure(e), "body") => Some(Value::Content(e.body.clone())),','(Content::Figure(e), f) => e.get_field(f),');p.write_text(s)
(out/'caption-projection-implementation.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),public_api_changes=0,remaining='FG1: omitted vs explicit none, caption metadata, kind identity/synthesis; SY1 symbol morphology'),indent=2)+'\n')
