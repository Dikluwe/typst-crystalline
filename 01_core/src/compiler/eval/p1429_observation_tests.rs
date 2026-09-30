//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/p1429-observation-tests.md
//! @prompt-hash f1aa6dd5
//! @layer L1
#![cfg(test)]

use super::*;
use crate::compiler::stdlib::native_accent;
use crate::entities::args::Args;
use crate::entities::font_book::FontBook;
use crate::entities::layout_types::Length;
use crate::entities::rel::Rel;
use crate::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library,
};
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
    if let Some(value) = size {
        args.named.insert("size".into(), Value::Relative(value));
    }
    if let Some(value) = dotless {
        args.named.insert("dotless".into(), Value::Bool(value));
    }
    let Value::Content(content) =
        native_accent(&mut EvalContext::new(), &args, &NullWorld::default(), id())
            .unwrap()
    else {
        unreachable!()
    };
    content
}

#[test]
fn p1429_observation_relation_discrimina_cada_causa_simetricamente() {
    let base = carrier('^', None, None);
    assert_eq!(base.observation_relation(&base), ObservationRelation::Same);
    assert_eq!(base.observation_relation(&base.clone()), ObservationRelation::Same);

    for (label, changed) in [
        ("accent", carrier('~', None, None)),
        ("size", carrier('^', Some(Rel::from_percent(150.0)), None)),
        ("dotless", carrier('^', None, Some(false))),
        ("size-bit", carrier('^', Some(Rel::from_percent(100.0)), None)),
        ("dotless-bit", carrier('^', None, Some(true))),
    ] {
        assert_eq!(
            base.observation_relation(&changed),
            ObservationRelation::Different,
            "forward relation collapsed {label}"
        );
        assert_eq!(
            changed.observation_relation(&base),
            ObservationRelation::Different,
            "reverse relation collapsed {label}"
        );
    }
}
