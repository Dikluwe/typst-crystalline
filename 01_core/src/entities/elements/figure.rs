//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/elements/figure.md
//! @prompt-hash 5b63362a
//! @layer L1
//! @updated 2026-06-11
//!
//! `FigureElem` — Lote 13 P328 (o último element-shaped). `figure(body, …)`.
//! **Locatável** (M1) — absorve `extract_payload`. Contentor: recurse em
//! `body` E `caption` (simétrico em map_content/map_text).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::counter_update::CounterUpdate;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// A nominal element identity or a custom Figure counter name.
#[derive(Debug, Clone)]
pub enum FigureKind {
    Element(crate::entities::func::Func),
    Name(ecow::EcoString),
}

impl PartialEq for FigureKind {
    fn eq(&self, other: &Self) -> bool {
        use crate::entities::func::FuncRepr;
        match self {
            Self::Name(a) => match other {
                Self::Name(b) => a == b,
                Self::Element(_) => false,
            },
            Self::Element(a) => match other {
                Self::Name(_) => false,
                Self::Element(b) => match a.repr() {
                    FuncRepr::Native(a) => match b.repr() {
                        FuncRepr::Native(b) => std::ptr::fn_addr_eq(a.call, b.call),
                        FuncRepr::Format(_, _)
                        | FuncRepr::NativeWithEngine(_)
                        | FuncRepr::Element(_)
                        | FuncRepr::Closure(_)
                        | FuncRepr::Plugin(_)
                        | FuncRepr::With(_) => false,
                    },
                    FuncRepr::Format(ak, a) => match b.repr() {
                        FuncRepr::Format(bk, b) => {
                            ak == bk && std::ptr::fn_addr_eq(a.call, b.call)
                        }
                        FuncRepr::Native(_)
                        | FuncRepr::NativeWithEngine(_)
                        | FuncRepr::Element(_)
                        | FuncRepr::Closure(_)
                        | FuncRepr::Plugin(_)
                        | FuncRepr::With(_) => false,
                    },
                    FuncRepr::NativeWithEngine(a) => match b.repr() {
                        FuncRepr::NativeWithEngine(b) => {
                            std::ptr::fn_addr_eq(a.call, b.call)
                        }
                        FuncRepr::Native(_)
                        | FuncRepr::Format(_, _)
                        | FuncRepr::Element(_)
                        | FuncRepr::Closure(_)
                        | FuncRepr::Plugin(_)
                        | FuncRepr::With(_) => false,
                    },
                    FuncRepr::Element(a) => match b.repr() {
                        FuncRepr::Element(b) => Arc::ptr_eq(&a.ctor, &b.ctor),
                        FuncRepr::Native(_)
                        | FuncRepr::Format(_, _)
                        | FuncRepr::NativeWithEngine(_)
                        | FuncRepr::Closure(_)
                        | FuncRepr::Plugin(_)
                        | FuncRepr::With(_) => false,
                    },
                    FuncRepr::Closure(_) => match b.repr() {
                        FuncRepr::Closure(_) => Arc::ptr_eq(&a.0, &b.0),
                        FuncRepr::Native(_)
                        | FuncRepr::Format(_, _)
                        | FuncRepr::NativeWithEngine(_)
                        | FuncRepr::Element(_)
                        | FuncRepr::Plugin(_)
                        | FuncRepr::With(_) => false,
                    },
                    FuncRepr::Plugin(_) => match b.repr() {
                        FuncRepr::Plugin(_) => Arc::ptr_eq(&a.0, &b.0),
                        FuncRepr::Native(_)
                        | FuncRepr::Format(_, _)
                        | FuncRepr::NativeWithEngine(_)
                        | FuncRepr::Element(_)
                        | FuncRepr::Closure(_)
                        | FuncRepr::With(_) => false,
                    },
                    FuncRepr::With(_) => match b.repr() {
                        FuncRepr::With(_) => Arc::ptr_eq(&a.0, &b.0),
                        FuncRepr::Native(_)
                        | FuncRepr::Format(_, _)
                        | FuncRepr::NativeWithEngine(_)
                        | FuncRepr::Element(_)
                        | FuncRepr::Closure(_)
                        | FuncRepr::Plugin(_) => false,
                    },
                },
            },
        }
    }
}
impl Eq for FigureKind {}
impl std::hash::Hash for FigureKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        use crate::entities::func::FuncRepr;
        match self {
            Self::Name(name) => {
                0u8.hash(state);
                name.hash(state);
            }
            Self::Element(function) => {
                1u8.hash(state);
                std::mem::discriminant(function.repr()).hash(state);
                match function.repr() {
                    FuncRepr::Native(native) => (native.call as usize).hash(state),
                    FuncRepr::Format(kind, native) => {
                        std::mem::discriminant(kind).hash(state);
                        (native.call as usize).hash(state);
                    }
                    FuncRepr::NativeWithEngine(native) => {
                        (native.call as usize).hash(state)
                    }
                    FuncRepr::Element(element) => Arc::as_ptr(&element.ctor).hash(state),
                    FuncRepr::Closure(_) | FuncRepr::Plugin(_) | FuncRepr::With(_) => {
                        Arc::as_ptr(&function.0).hash(state)
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum FigureSupplement {
    Auto,
    None,
    Content(Content),
    Func { function: crate::entities::func::Func, span: crate::entities::span::Span },
}

/// Lexical arguments and one occurrence's prepared Figure values.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct FigureElem {
    pub body: Content,
    pub caption: Option<Content>,
    pub kind: Option<Option<FigureKind>>,
    pub resolved_kind: Option<FigureKind>,
    pub supplement: Option<FigureSupplement>,
    pub resolved_supplement: Option<Option<Content>>,
}

impl Element for FigureElem {
    fn plain_text(&self) -> String {
        // Paridade content.rs:1706: body + caption com espaço; vazios omitidos.
        let body_text = self.body.plain_text();
        let cap_text = self.caption.as_ref().map(|c| c.plain_text()).unwrap_or_default();
        match (body_text.is_empty(), cap_text.is_empty()) {
            (false, false) => format!("{} {}", body_text, cap_text),
            (false, true) => body_text,
            (true, false) => cap_text,
            (true, true) => String::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.body.is_empty() && self.caption.as_ref().is_none_or(|c| c.is_empty())
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Figure(Arc::new(FigureElem {
            body: self.body.map_content(transform)?,
            caption: self
                .caption
                .as_ref()
                .map(|c| c.map_content(transform))
                .transpose()?,
            kind: self.kind.clone(),
            resolved_kind: self.resolved_kind.clone(),
            supplement: if self.resolved_supplement.is_some() {
                self.supplement.clone()
            } else {
                match &self.supplement {
                    Some(FigureSupplement::Content(content)) => {
                        Some(FigureSupplement::Content(content.map_content(transform)?))
                    }
                    Some(
                        FigureSupplement::Auto
                        | FigureSupplement::None
                        | FigureSupplement::Func { .. },
                    )
                    | None => self.supplement.clone(),
                }
            },
            resolved_supplement: self
                .resolved_supplement
                .as_ref()
                .map(|content| {
                    content.as_ref().map(|c| c.map_content(transform)).transpose()
                })
                .transpose()?,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Figure(Arc::new(FigureElem {
            body: self.body.map_text(transform),
            caption: self.caption.as_ref().map(|c| c.map_text(transform)),
            kind: self.kind.clone(),
            resolved_kind: self.resolved_kind.clone(),
            supplement: if self.resolved_supplement.is_some() {
                self.supplement.clone()
            } else {
                match &self.supplement {
                    Some(FigureSupplement::Content(content)) => {
                        Some(FigureSupplement::Content(content.map_text(transform)))
                    }
                    Some(
                        FigureSupplement::Auto
                        | FigureSupplement::None
                        | FigureSupplement::Func { .. },
                    )
                    | None => self.supplement.clone(),
                }
            },
            resolved_supplement: self
                .resolved_supplement
                .as_ref()
                .map(|content| content.as_ref().map(|c| c.map_text(transform))),
        }))
    }

    fn get_field(&self, field: &str) -> Option<crate::entities::value::Value> {
        use crate::entities::value::Value;
        match field {
            "body" => Some(Value::Content(self.body.clone())),
            "kind" => {
                let kind = self
                    .resolved_kind
                    .as_ref()
                    .map(Some)
                    .or(self.kind.as_ref().map(|kind| kind.as_ref()))?;
                Some(match kind {
                    None => Value::Auto,
                    Some(FigureKind::Element(function)) => Value::Func(function.clone()),
                    Some(FigureKind::Name(name)) => Value::Str(name.clone()),
                })
            }
            "supplement" => {
                if let Some(resolved) = &self.resolved_supplement {
                    Some(
                        resolved.as_ref().map_or(Value::None, |content| {
                            Value::Content(content.clone())
                        }),
                    )
                } else {
                    self.supplement.as_ref().map(|supplement| match supplement {
                        FigureSupplement::Auto => Value::Auto,
                        FigureSupplement::None => Value::None,
                        FigureSupplement::Content(content) => {
                            Value::Content(content.clone())
                        }
                        FigureSupplement::Func { function, span: _ } => {
                            Value::Func(function.clone())
                        }
                    })
                }
            }

            "caption" => self.caption.as_ref().map(|body| {
                Value::Content(Content::figure_caption(body.clone(), None, None))
            }),
            _ => None,
        }
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::Figure)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        // The occurrence walk applies effective numbering, independently of caption.
        Some(ElementPayload::Figure {
            kind: self.resolved_kind.clone().or_else(|| self.kind.clone().flatten()),
            supplement: self.resolved_supplement.clone(),
            counter_update: CounterUpdate::step(),
            is_counted: true,
            caption_text: self.caption.as_ref().map(|c| c.plain_text()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> FigureElem {
        FigureElem {
            body: Content::text("img"),
            caption: Some(Content::text("Fig 1")),
            kind: Some(Some(FigureKind::Name("image".into()))),
            resolved_kind: None,
            supplement: None,
            resolved_supplement: None,
        }
    }

    #[test]
    fn plain_text_body_e_caption() {
        assert_eq!(ex().plain_text(), "img Fig 1");
    }

    #[test]
    fn plain_text_sem_caption() {
        let f = FigureElem {
            body: Content::text("img"),
            caption: None,
            kind: None,
            resolved_kind: None,
            supplement: None,
            resolved_supplement: None,
        };
        assert_eq!(f.plain_text(), "img");
    }

    #[test]
    fn is_empty_body_e_caption_vazios() {
        assert!(!ex().is_empty());
        let vazia = FigureElem {
            body: Content::Empty,
            caption: None,
            kind: None,
            resolved_kind: None,
            supplement: None,
            resolved_supplement: None,
        };
        assert!(vazia.is_empty());
    }

    #[test]
    fn map_content_recurse_body_e_caption_preserva_kind() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "img" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::Figure(e) => {
                assert_eq!(e.kind, Some(Some(FigureKind::Name("image".into()))));
                assert!(matches!(&e.body, Content::Text(s) if s.as_str() == "Z"));
            }
            _ => panic!("esperado Figure"),
        }
    }

    #[test]
    fn locatavel_kind_e_payload() {
        assert_eq!(ex().element_kind(), Some(ElementKind::Figure));
        match ex().to_payload() {
            Some(ElementPayload::Figure { kind, counter_update, is_counted, .. }) => {
                assert_eq!(kind, Some(FigureKind::Name("image".into())));
                assert_eq!(counter_update, CounterUpdate::step());
                // Effective numbering is applied by the occurrence walk.
                assert!(is_counted);
            }
            other => panic!("esperado Figure payload, obtido {other:?}"),
        }
    }

    #[test]
    fn is_counted_placeholder_does_not_depend_on_caption() {
        // Caption presence does not decide automatic count eligibility.
        let f = FigureElem {
            body: Content::text("x"),
            caption: None,
            kind: None,
            resolved_kind: None,
            supplement: None,
            resolved_supplement: None,
        };
        match f.to_payload() {
            Some(ElementPayload::Figure { is_counted, .. }) => assert!(is_counted),
            _ => panic!("esperado Figure payload"),
        }
    }

    fn h(e: &FigureElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let mut v = ex();
        v.kind = Some(Some(FigureKind::Name("table".into())));
        assert_ne!(h(&ex()), h(&v));
    }
    #[test]
    fn figure_caption_projection_retains_body_and_omits_unresolved_defaults() {
        use crate::entities::value::Value;
        let element = ex();
        let original_caption = element.caption.clone().unwrap();
        let content = Content::Figure(Arc::new(element.clone()));
        let Some(Value::Content(Content::FigureCaption(caption))) =
            content.get_field("caption")
        else {
            panic!("present caption was lost")
        };
        assert_eq!(caption.body, original_caption);
        assert!(caption.position.is_none());
        assert!(caption.separator.is_none());
        assert_eq!(content.get_field("body"), Some(Value::Content(element.body)));
        assert!(content.get_field("nonexistent").is_none());
    }
}

#[cfg(test)]
mod figure_kind_contract_tests {
    use super::*;

    #[test]
    fn figure_v16_no_caption_payload_keeps_automatic_count_eligibility() {
        let element = FigureElem {
            body: Content::text("body"),
            caption: None,
            kind: None,
            resolved_kind: None,
            supplement: None,
            resolved_supplement: None,
        };
        let Some(ElementPayload::Figure { is_counted, caption_text, .. }) =
            element.to_payload()
        else {
            panic!("Figure must remain locatable");
        };
        assert!(is_counted, "the effective numbering gate belongs to occurrence styles, not caption presence");
        assert_eq!(caption_text, None);
    }

    fn lexical() -> FigureElem {
        FigureElem {
            body: Content::text("body"),
            caption: Some(Content::text("caption")),
            kind: Some(None),
            resolved_kind: None,
            supplement: Some(FigureSupplement::Content(Content::text("supplement"))),
            resolved_supplement: None,
        }
    }

    fn hash_kind(kind: &FigureKind) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        kind.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn figure_v16_kind_identity_distinguishes_homonyms_and_preserves_alias_hashes() {
        use crate::entities::func::Func;
        let original = FigureKind::Element(Func::native(
            "table",
            crate::compiler::stdlib::native_table,
        ));
        let alias = FigureKind::Element(Func::native(
            "alias",
            crate::compiler::stdlib::native_table,
        ));
        let homonym = FigureKind::Element(Func::native(
            "table",
            crate::compiler::stdlib::native_image,
        ));
        assert_eq!(original, alias);
        assert_eq!(hash_kind(&original), hash_kind(&alias));
        assert_ne!(original, homonym);
        assert_ne!(original, FigureKind::Name("table".into()));
        let constructor: crate::entities::element_registry::ElementCtor =
            Arc::new(|_| Ok(Content::Empty));
        let dynamic = FigureKind::Element(Func::element("one", constructor.clone()));
        let dynamic_alias = FigureKind::Element(Func::element("two", constructor));
        let independent =
            FigureKind::Element(Func::element("one", Arc::new(|_| Ok(Content::Empty))));
        assert_eq!(dynamic, dynamic_alias);
        assert_eq!(hash_kind(&dynamic), hash_kind(&dynamic_alias));
        assert_ne!(dynamic, independent);
    }

    #[test]
    fn figure_v16_format_and_engine_kind_identity_keep_nominal_abis() {
        use crate::entities::func::{FormatElementKind, Func, NativeFunc};
        let format = |kind| {
            FigureKind::Element(Func::native_format(
                kind,
                NativeFunc {
                    name: "format",
                    call: crate::compiler::stdlib::native_table,
                    namespace: None,
                },
            ))
        };
        assert_eq!(format(FormatElementKind::Svg), format(FormatElementKind::Svg));
        assert_ne!(format(FormatElementKind::Svg), format(FormatElementKind::Png));
        assert_ne!(
            format(FormatElementKind::Svg),
            FigureKind::Element(Func::native(
                "format",
                crate::compiler::stdlib::native_table
            ))
        );
        let page = FigureKind::Element(Func::native_with_engine(
            "page",
            crate::compiler::stdlib::native_page,
        ));
        let alias = FigureKind::Element(Func::native_with_engine(
            "alias",
            crate::compiler::stdlib::native_page,
        ));
        let distinct = FigureKind::Element(Func::native_with_engine(
            "page",
            crate::compiler::stdlib::native_op,
        ));
        assert_eq!(page, alias);
        assert_eq!(hash_kind(&page), hash_kind(&alias));
        assert_ne!(page, distinct);
    }

    #[test]
    fn figure_v16_lexical_and_prepared_fields_preserve_presence_and_nominal_values() {
        use crate::entities::value::Value;
        let mut element = lexical();
        assert_eq!(element.get_field("kind"), Some(Value::Auto));
        element.kind = None;
        assert_eq!(element.get_field("kind"), None);
        element.kind = Some(Some(FigureKind::Name("table".into())));
        assert_eq!(element.get_field("kind"), Some(Value::Str("table".into())));
        let function = crate::entities::func::Func::native(
            "table",
            crate::compiler::stdlib::native_table,
        );
        element.resolved_kind = Some(FigureKind::Element(function.clone()));
        assert_eq!(element.get_field("kind"), Some(Value::Func(function)));
        element.supplement = None;
        assert_eq!(element.get_field("supplement"), None);
        element.supplement = Some(FigureSupplement::Auto);
        assert_eq!(element.get_field("supplement"), Some(Value::Auto));
        element.resolved_supplement = Some(None);
        assert_eq!(element.get_field("supplement"), Some(Value::None));
        assert_eq!(element.clone(), element);
        let Some(ElementPayload::Figure { kind, supplement, is_counted, .. }) =
            element.to_payload()
        else {
            panic!("missing payload")
        };
        assert_eq!(kind, element.resolved_kind);
        assert_eq!(supplement, Some(None));
        assert!(is_counted);
    }

    #[test]
    fn figure_v16_maps_transform_one_supplement_and_preserve_lexical_origin() {
        let raw = lexical();
        let mut raw_seen = Vec::new();
        let Content::Figure(raw_mapped) = raw
            .map_content(&mut |content| {
                if let Content::Text(text) = content {
                    raw_seen.push(text.to_string());
                }
                Ok(None)
            })
            .unwrap()
        else {
            panic!("missing Figure")
        };
        assert_eq!(raw_seen, ["body", "caption", "supplement"]);
        assert_eq!(*raw_mapped, raw);
        let mut prepared = raw.clone();
        prepared.resolved_kind = Some(FigureKind::Name("custom".into()));
        prepared.resolved_supplement = Some(Some(Content::text("supplement")));
        let mut seen = Vec::new();
        let Content::Figure(mapped) = prepared
            .map_content(&mut |content| {
                if let Content::Text(text) = content {
                    seen.push(text.to_string());
                    return Ok(Some(Content::text(text.to_uppercase())));
                }
                Ok(None)
            })
            .unwrap()
        else {
            panic!("missing Figure")
        };
        assert_eq!(seen, ["body", "caption", "supplement"]);
        assert_eq!(mapped.supplement, raw.supplement);
        assert_eq!(mapped.resolved_supplement, Some(Some(Content::text("SUPPLEMENT"))));
        assert_eq!(mapped.kind, prepared.kind);
        assert_eq!(mapped.resolved_kind, prepared.resolved_kind);
        let mut text_seen = Vec::new();
        let Content::Figure(mapped_text) = prepared.map_text(&mut |text| {
            text_seen.push(text.to_string());
            text.to_uppercase()
        }) else {
            panic!("missing Figure")
        };
        assert_eq!(text_seen, ["body", "caption", "supplement"]);
        assert_eq!(mapped_text.supplement, prepared.supplement);
        assert_eq!(mapped_text.resolved_supplement, mapped.resolved_supplement);
    }

    #[test]
    fn figure_v16_kind_synthetic_invalid_functions_remain_nominal_and_nonexecuting() {
        use crate::contracts::plugin_host::{PluginError, PluginHost, PluginModuleId};
        use crate::entities::bytes::Bytes;
        use crate::entities::func::{ClosureRepr, Func};
        struct NeverCalled;
        impl PluginHost for NeverCalled {
            fn load(&self, _: &[u8]) -> Result<PluginModuleId, PluginError> {
                panic!("identity must not load plugins")
            }
            fn exports(
                &self,
                _: PluginModuleId,
            ) -> Result<Vec<ecow::EcoString>, PluginError> {
                panic!("identity must not enumerate plugins")
            }
            fn call(
                &self,
                _: PluginModuleId,
                _: &str,
                _: &[Bytes],
            ) -> Result<Bytes, PluginError> {
                panic!("identity must not call plugins")
            }
            fn transition(
                &self,
                _: PluginModuleId,
                _: &str,
                _: &[Bytes],
            ) -> Result<PluginModuleId, PluginError> {
                panic!("identity must not mutate plugins")
            }
        }
        let source = crate::entities::source::Source::detached("none");
        let closure = || {
            FigureKind::Element(Func::closure(ClosureRepr {
                name: Some("table".into()),
                params: vec![],
                sink_name: None,
                body: source.root().clone(),
                captured: Arc::new(crate::entities::scope::Scope::new()),
                capturer: crate::entities::scope::Capturer::Function,
            }))
        };
        let plugin = || {
            FigureKind::Element(Func::plugin(crate::entities::plugin_func::PluginFunc {
                host: Arc::new(NeverCalled),
                module: PluginModuleId(1),
                name: "table".into(),
            }))
        };
        let with = || {
            FigureKind::Element(
                Func::native("table", crate::compiler::stdlib::native_table)
                    .with(crate::entities::args::Args::positional(vec![])),
            )
        };
        let values = [
            closure(),
            plugin(),
            with(),
            FigureKind::Element(Func::native(
                "table",
                crate::compiler::stdlib::native_table,
            )),
            FigureKind::Name("table".into()),
        ];
        for (index, value) in values.iter().enumerate() {
            assert_eq!(*value, value.clone());
            assert_eq!(hash_kind(value), hash_kind(&value.clone()));
            for (other_index, other) in values.iter().enumerate() {
                assert_eq!(value == other, index == other_index);
                assert_eq!(value == other, other == value);
            }
        }
        assert_ne!(values[0], closure());
        assert_ne!(values[1], plugin());
        assert_ne!(values[2], with());
    }
}
