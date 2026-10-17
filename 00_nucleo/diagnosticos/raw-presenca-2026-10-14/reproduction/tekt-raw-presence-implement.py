from pathlib import Path

def edit(n,old,new):
 p=Path(n);s=p.read_text();assert old in s,n;s=s.replace(old,new,1);p.write_text(s)
raw='01_core/src/entities/elements/raw.rs'
edit(raw,'/// Código raw.', '''/// Presença lexical de `block`, inclusive quando falso.
pub const RAW_SET_BLOCK: u8 = 1;
/// Presença lexical de `lang`, inclusive quando `none`.
pub const RAW_SET_LANG: u8 = 2;

/// Código raw.''')
edit(raw,'    pub block: bool,','    pub block: bool,\n    /// Máscara dos campos opcionais conhecidos; demais bits são reservados.\n    pub set_fields: u8,')
edit(raw,'impl Element for RawElem {','''impl Element for RawElem {
    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "text" => Some(Value::Str(self.text.clone())),
            "block" if self.set_fields & RAW_SET_BLOCK != 0 => Some(Value::Bool(self.block)),
            "lang" if self.set_fields & RAW_SET_LANG != 0 => Some(self.lang.clone().map(Value::Str).unwrap_or(Value::None)),
            _ => None,
        }
    }
''')
edit(raw,'            block: true,','            block: true,\n            set_fields: RAW_SET_BLOCK | RAW_SET_LANG,')
edit('01_core/src/entities/content/construction.rs','        Self::Raw(Arc::new(RawElem { text: text.into(), lang, block }))','''        use crate::entities::elements::raw::{RAW_SET_BLOCK, RAW_SET_LANG};
        let set_fields = RAW_SET_BLOCK | if lang.is_some() { RAW_SET_LANG } else { 0 };
        Self::Raw(Arc::new(RawElem { text: text.into(), lang, block, set_fields }))''')
edit('01_core/src/entities/content/fields.rs','            (Content::Heading(h), f) => h.get_field(f),','            (Content::Heading(h), f) => h.get_field(f),\n            (Content::Raw(e), f) => e.get_field(f),')
edit('01_core/src/compiler/stdlib/structural/markup.rs','    Ok(Value::Content(Content::raw(text, lang, block)))','''    use crate::entities::elements::raw::{RawElem, RAW_SET_BLOCK, RAW_SET_LANG};
    let mut set_fields = 0;
    if matches!(args.named.get("block"), Some(Value::Bool(_))) {
        set_fields |= RAW_SET_BLOCK;
    }
    if args.named.contains_key("lang") {
        set_fields |= RAW_SET_LANG;
    }
    Ok(Value::Content(Content::Raw(std::sync::Arc::new(RawElem {
        text, lang, block, set_fields,
    }))))''')
edit('01_core/src/compiler/eval/observation/elements.rs','        let Self { text: _, lang: _, block: _ } = self;','        let Self { text: _, lang: _, block: _, set_fields: _ } = self;')
edit('01_core/src/compiler/eval/observation/elements.rs','            .and(self.lang.observation_relation(&other.lang))\n            .and(self.block.observation_relation(&other.block))','            .and(self.lang.observation_relation(&other.lang))\n            .and(self.block.observation_relation(&other.block))\n            .and(self.set_fields.observation_relation(&other.set_fields))')
edit('01_core/src/compiler/eval/bindings/field_access/content_methods.rs','        Content::MathMatrix(_) => match field {','''        Content::Raw(_) => match field {
            "text" | "block" | "lang" => c.get_field(field).map(ContentField::Set).unwrap_or(ContentField::Unset),
            _ => ContentField::Undeclared,
        },
        Content::MathMatrix(_) => match field {''')
edit('01_core/src/compiler/eval/bindings/field_access/content_methods.rs','        Content::Text(_) => &["text"],','        Content::Text(_) => &["text"],\n        Content::Raw(_) => &["text", "block", "lang"],')
p=Path('01_core/src/compiler/stdlib/loading/encoding.rs');s=p.read_text();a=s.index('            Content::Raw(raw_elem) => {');b=s.index('            Content::Link(',a);s=s[:a]+s[b:];p.write_text(s)
p=Path('02_shell/src/cli.rs');s=p.read_text();a=s.index('        Content::Raw(raw_elem) => {');b=s.index('        Content::MathMatrix',a);s=s[:a]+'''        Content::Raw(_) => {
            for name in ["text", "block", "lang"] {
                if let Some(value) = content.get_field(name) {
                    fields.push((name.into(), value_to_semantic(&value)));
                }
            }
        }
'''+s[b:];p.write_text(s)
