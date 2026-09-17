//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/p1429-math-accent-tests.md
//! @prompt-hash ad756cbd
//! @layer L1
#![cfg(test)]

use super::Element;
use crate::compiler::eval::{repr::repr_content, EvalContext};
use crate::compiler::stdlib::native_accent;
use crate::contracts::world::World;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontBook;
use crate::entities::layout_types::Length;
use crate::entities::rel::Rel;
use crate::entities::source::Source;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;
use crate::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::num::NonZeroU16;

#[derive(Default)]
struct NullWorld {
    library: Library,
    book: FontBook,
}
impl World for NullWorld {
    fn library(&self) -> &Library {
        &self.library
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        id()
    }
    fn source(&self, _: FileId) -> FileResult<Source> {
        Err(FileError::NotFound)
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound)
    }
    fn font(&self, _: usize) -> Option<Font> {
        None
    }
    fn today(&self, _: Option<crate::entities::duration::Duration>) -> Option<Datetime> {
        None
    }
}
fn id() -> FileId {
    FileId::from_raw(NonZeroU16::new(1429).unwrap())
}

fn carrier(mark: char, size: Option<Rel<Length>>, dotless: Option<bool>) -> Content {
    let mut args = Args::positional(vec![
        Value::Content(Content::text("x")),
        Value::Str(mark.to_string().into()),
    ]);
    if let Some(v) = size {
        args.named.insert("size".into(), Value::Relative(v));
    }
    if let Some(v) = dotless {
        args.named.insert("dotless".into(), Value::Bool(v));
    }
    let Value::Content(content) =
        native_accent(&mut EvalContext::new(), &args, &NullWorld::default(), id())
            .unwrap()
    else {
        unreachable!()
    };
    content
}

fn hash(value: &Content) -> u64 {
    let mut h = DefaultHasher::new();
    value.hash(&mut h);
    h.finish()
}

#[test]
fn p1429_carrier_discrimina_valores_e_presenca_independentes() {
    let absent = carrier('^', None, None);
    let accent = carrier('~', None, None);
    let size = carrier('^', Some(Rel::from_percent(150.0)), None);
    let dotless = carrier('^', None, Some(false));
    let size_bit = carrier('^', Some(Rel::from_percent(100.0)), None);
    let dotless_bit = carrier('^', None, Some(true));
    let explicit_defaults = carrier('^', Some(Rel::from_percent(100.0)), Some(true));

    assert_eq!(absent.clone(), absent, "clone changed complete carrier");
    for (label, changed) in [
        ("accent", &accent),
        ("size", &size),
        ("dotless", &dotless),
        ("size presence", &size_bit),
        ("dotless presence", &dotless_bit),
    ] {
        assert_ne!(&absent, changed, "{label} absent from equality");
        assert_ne!(hash(&absent), hash(changed), "{label} absent from hash");
    }
    assert_eq!(repr_content(&absent), "accent(base: [x], accent: \"\\u{302}\")");
    assert_eq!(
        repr_content(&explicit_defaults),
        "accent(base: [x], accent: \"\\u{302}\", size: 100% + 0pt, dotless: true)"
    );
}

#[test]
fn p1429_walkers_preservam_carrier_e_descem_so_na_base() {
    let input = carrier('^', Some(Rel::from_percent(150.0)), Some(false));
    let Content::MathAccent(elem) = &input else {
        panic!("exactly one MathAccent expected")
    };
    assert_eq!(elem.plain_text(), "x\u{302}");
    let mut transform = |child: &Content| -> SourceResult<Option<Content>> {
        Ok((child.plain_text() == "x").then(|| Content::text("z")))
    };
    let mapped = elem.map_content(&mut transform).unwrap();
    assert_eq!(
        repr_content(&mapped),
        "accent(base: [z], accent: \"\\u{302}\", size: 150% + 0pt, dotless: false)"
    );
    let mut terminal = |_: &str| "mutated".to_string();
    assert_eq!(
        elem.map_text(&mut terminal),
        input,
        "terminal map changed structural accent"
    );
}
