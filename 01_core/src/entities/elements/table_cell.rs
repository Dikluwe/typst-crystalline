//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/elements/table_cell.md
//! @prompt-hash 5f630f31
//! @layer L1
//! @updated 2026-10-10
//!
//! `TableCellElem` — Lote 12 P327 (bloco grid/table cell). `table_cell(body, …)`.
//! Contentor: recurse no body em map_content E map_text. Não-locatável.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::geometry::Stroke;
use crate::entities::layout_types::Length;
use crate::entities::layout_types::{Align2D, Color};
use crate::entities::sides::Sides;
use crate::entities::source_result::SourceResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TableHeaderScope {
    Both,
    Column,
    Row,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TableCellKind {
    #[default]
    Auto,
    Header {
        level: u32,
        scope: TableHeaderScope,
    },
    Data,
}

/// Presença lexical; defaults derivados nunca assentam um campo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CellFieldPresence {
    #[default]
    Omitted,
    Auto,
    Explicit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TableCellPresence {
    pub x: CellFieldPresence,
    pub y: CellFieldPresence,
    pub colspan: bool,
    pub rowspan: bool,
    pub fill: CellFieldPresence,
    pub stroke: bool,
    pub align: CellFieldPresence,
    pub inset: CellFieldPresence,
    pub breakable: CellFieldPresence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TableCellStage {
    Prepared,
    RecipeApplied,
    Complete,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableCellResolved {
    pub x: usize,
    pub y: usize,
    pub colspan: usize,
    pub rowspan: usize,
    pub fill: Option<Color>,
    pub stroke: Option<Stroke>,
    pub align: Option<Align2D>,
    pub inset: Sides<Length>,
    pub breakable: bool,
    pub visual: Content,
    pub(crate) stage: TableCellStage,
    pub(crate) guards: Vec<crate::entities::show::RuleId>,
}

/// Propriedades conhecidas da ocorrência, sem alterar o construtor lexical.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TableCellProperties {
    pub(crate) x: Option<usize>,
    pub(crate) y: Option<usize>,
    pub(crate) colspan: Option<usize>,
    pub(crate) rowspan: Option<usize>,
    pub(crate) fill: Option<Color>,
    pub(crate) stroke: Option<Stroke>,
    pub(crate) stroke_inherited: Option<Stroke>,
    pub(crate) align: Option<Align2D>,
    pub(crate) inset: Option<Sides<Option<Length>>>,
    pub(crate) breakable: Option<bool>,
    pub(crate) presence: TableCellPresence,
}

impl TableCellProperties {
    pub(crate) fn chain_styles(
        base: &crate::entities::style_chain::StyleChain,
        styles: &crate::entities::style::Styles,
    ) -> crate::entities::style_chain::StyleChain {
        let mut delta = styles.delta().clone();
        for (key, value) in &mut delta.custom {
            if key.as_str() == "cell.stroke" {
                if let crate::entities::value::Value::Stroke(stroke) = value {
                    let inherited = match base.custom("cell.stroke") {
                        Some(crate::entities::value::Value::Stroke(s)) => {
                            Some(s.as_ref())
                        }
                        _ => None,
                    };
                    *stroke = std::sync::Arc::new(Self::fold_stroke(
                        stroke.as_ref(),
                        inherited,
                    ));
                }
            }
        }
        base.push(delta)
    }
    pub(crate) fn fold_stroke(cell: &Stroke, inherited: Option<&Stroke>) -> Stroke {
        let mut out = cell.clone();
        if let Some(base) = inherited {
            if !cell.specified.paint {
                out.paint = base.paint.clone();
                out.specified.paint = base.specified.paint;
            }
            if !cell.specified.thickness {
                out.thickness = base.thickness;
                out.specified.thickness = base.specified.thickness;
            }
            if !cell.specified.cap {
                out.cap = base.cap;
                out.specified.cap = base.specified.cap;
            }
            if !cell.specified.join {
                out.join = base.join;
                out.specified.join = base.specified.join;
            }
            if !cell.specified.dash {
                out.dash = base.dash.clone();
                out.specified.dash = base.specified.dash;
            }
            if !cell.specified.miter_limit {
                out.miter_limit = base.miter_limit;
                out.specified.miter_limit = base.specified.miter_limit;
            }
        }
        out
    }

    pub(crate) fn style_inset(
        value: &crate::entities::value::Value,
    ) -> Option<Sides<Option<Length>>> {
        use crate::entities::value::Value;
        match value {
            Value::Length(l) => Some(Sides::uniform(Some(*l))),
            Value::Dict(d) => {
                if d.iter().any(|(k, v)| {
                    !matches!(
                        k.as_str(),
                        "left" | "top" | "right" | "bottom" | "x" | "y" | "rest"
                    ) || !matches!(v, Value::Length(_))
                }) {
                    return None;
                }
                let get = |side: &str, axis: &str| {
                    d.get(side)
                        .or_else(|| d.get(axis))
                        .or_else(|| d.get("rest"))
                        .and_then(
                            |v| if let Value::Length(l) = v { Some(*l) } else { None },
                        )
                };
                Some(Sides::new(
                    get("left", "x"),
                    get("top", "y"),
                    get("right", "x"),
                    get("bottom", "y"),
                ))
            }
            _ => None,
        }
    }
}

/// Célula de `Table`. `body` + 9 cosméticos (posição/span/estilo).
#[derive(Debug, Clone, PartialEq)]
pub struct TableCellStandaloneResolved {
    pub(crate) properties: Option<TableCellProperties>,
    pub visual: Content,
    pub(crate) stage: TableCellStage,
    pub(crate) guards: Vec<crate::entities::show::RuleId>,
}

impl TableCellElem {
    pub(crate) fn properties(&self) -> TableCellProperties {
        if let Some(p) = self.standalone.as_ref().and_then(|r| r.properties.as_ref()) {
            return p.clone();
        }
        if let Some(r) = &self.resolved {
            return TableCellProperties {
                x: Some(r.x),
                y: Some(r.y),
                colspan: Some(r.colspan),
                rowspan: Some(r.rowspan),
                fill: r.fill.clone(),
                stroke: r.stroke.clone(),
                stroke_inherited: None,
                align: r.align,
                inset: Some(Sides::new(
                    Some(r.inset.left),
                    Some(r.inset.top),
                    Some(r.inset.right),
                    Some(r.inset.bottom),
                )),
                breakable: Some(r.breakable),
                presence: TableCellPresence {
                    x: CellFieldPresence::Explicit,
                    y: CellFieldPresence::Explicit,
                    colspan: true,
                    rowspan: true,
                    fill: CellFieldPresence::Explicit,
                    stroke: true,
                    align: if r.align.is_some() {
                        CellFieldPresence::Explicit
                    } else {
                        CellFieldPresence::Auto
                    },
                    inset: CellFieldPresence::Explicit,
                    breakable: CellFieldPresence::Explicit,
                },
            };
        }
        TableCellProperties {
            x: self.x,
            y: self.y,
            colspan: self.colspan,
            rowspan: self.rowspan,
            fill: self.fill.clone(),
            stroke: self.stroke.clone(),
            stroke_inherited: None,
            align: self.align,
            inset: self.inset.map(|s| {
                Sides::new(Some(s.left), Some(s.top), Some(s.right), Some(s.bottom))
            }),
            breakable: self.breakable,
            presence: self.presence,
        }
    }

    pub(crate) fn placement(
        &self,
    ) -> (Option<usize>, Option<usize>, Option<usize>, Option<usize>) {
        let p = self.properties();
        (p.x, p.y, p.colspan, p.rowspan)
    }

    pub(crate) fn visual(&self) -> &Content {
        if let Some(r) = &self.resolved {
            &r.visual
        } else if let Some(r) = &self.standalone {
            &r.visual
        } else {
            &self.body
        }
    }

    pub(crate) fn stage(&self) -> Option<TableCellStage> {
        self.resolved
            .as_ref()
            .map(|r| r.stage)
            .or_else(|| self.standalone.as_ref().map(|r| r.stage))
    }

    pub(crate) fn guards(&self) -> &[crate::entities::show::RuleId] {
        if let Some(r) = &self.resolved {
            &r.guards
        } else if let Some(r) = &self.standalone {
            &r.guards
        } else {
            &[]
        }
    }

    pub(crate) fn guarded(&self, id: crate::entities::show::RuleId) -> Self {
        let mut cell = self.clone();
        if let Some(r) = &self.resolved {
            let mut r = r.as_ref().clone();
            r.guards.push(id);
            r.stage = TableCellStage::RecipeApplied;
            cell.resolved = Some(Arc::new(r));
        } else if let Some(r) = &self.standalone {
            let mut r = r.as_ref().clone();
            r.guards.push(id);
            r.stage = TableCellStage::RecipeApplied;
            cell.standalone = Some(Arc::new(r));
        }
        cell
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableCellElem {
    pub source_span: crate::entities::span::Span,
    pub resolved: Option<Arc<TableCellResolved>>,
    pub standalone: Option<Arc<TableCellStandaloneResolved>>,
    pub body: Content,
    pub x: Option<usize>,
    pub y: Option<usize>,
    pub colspan: Option<usize>,
    pub rowspan: Option<usize>,
    pub stroke: Option<Stroke>,
    pub fill: Option<Color>,
    pub align: Option<Align2D>,
    pub inset: Option<Sides<Length>>,
    pub breakable: Option<bool>,
    pub kind: TableCellKind,
    pub presence: TableCellPresence,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `Stroke`/`Color`/
// `Align2D`/`Sides<Length>` carregam `f64`. Ressalva `-0.0` vs `0.0` aceitável.
impl std::hash::Hash for TableCellElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl TableCellElem {
    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        if let Some(standalone) = &self.standalone {
            let Some(visual) = standalone.visual.map_content_if_changed(transform)?
            else {
                return Ok(None);
            };
            let mut cell = self.clone();
            let mut plan = standalone.as_ref().clone();
            plan.visual = visual;
            cell.standalone = Some(Arc::new(plan));
            return Ok(Some(Content::TableCell(Arc::new(cell))));
        }
        if let Some(resolved) = &self.resolved {
            let Some(visual) = resolved.visual.map_content_if_changed(transform)? else {
                return Ok(None);
            };
            let mut cell = self.clone();
            let mut plan = resolved.as_ref().clone();
            plan.visual = visual;
            cell.resolved = Some(Arc::new(plan));
            return Ok(Some(Content::TableCell(Arc::new(cell))));
        }
        let Some(body) = self.body.map_content_if_changed(transform)? else {
            return Ok(None);
        };
        let mut cell = self.clone();
        cell.body = body;
        Ok(Some(Content::TableCell(Arc::new(cell))))
    }
}

impl Element for TableCellElem {
    fn plain_text(&self) -> String {
        self.visual().plain_text()
    }

    fn is_empty(&self) -> bool {
        self.visual().is_empty()
    }

    fn get_field(&self, field: &str) -> Option<crate::entities::value::Value> {
        use crate::entities::value::Value;
        use CellFieldPresence::{Auto, Explicit, Omitted};
        if field == "body" {
            return Some(Value::Content(
                if matches!(self.body.source_body(), Content::Empty) {
                    Content::Sequence(Arc::from([]))
                } else {
                    self.body.clone()
                },
            ));
        }
        if let Some(r) = &self.resolved {
            return match field {
                "x" => Some(Value::Int(r.x as i64)),
                "y" => Some(Value::Int(r.y as i64)),
                "colspan" => Some(Value::Int(
                    self.properties().colspan.unwrap_or(r.colspan) as i64,
                )),
                "rowspan" => Some(Value::Int(
                    self.properties().rowspan.unwrap_or(r.rowspan) as i64,
                )),
                "fill" => Some(r.fill.clone().map(Value::Color).unwrap_or(Value::None)),
                "stroke" => {
                    Some(r.stroke.clone().map(Value::from).unwrap_or(Value::None))
                }
                "align" => Some(r.align.map(Value::Align).unwrap_or(Value::Auto)),
                "breakable" => Some(Value::Bool(r.breakable)),
                "inset" => Some(inset_value(r.inset)),
                _ => None,
            };
        }
        if let Some(p) = self.standalone.as_ref().and_then(|r| r.properties.as_ref()) {
            let mut c = self.clone();
            c.standalone = None;
            c.x = p.x;
            c.y = p.y;
            c.colspan = p.colspan;
            c.rowspan = p.rowspan;
            c.fill = p.fill.clone();
            c.stroke = p.stroke.clone();
            c.align = p.align;
            c.breakable = p.breakable;
            c.presence = p.presence;
            if field == "inset" && p.presence.inset == Explicit {
                return p.inset.map(|s| {
                    if s.left.is_some()
                        && s.left == s.top
                        && s.left == s.right
                        && s.left == s.bottom
                    {
                        return inset_value(Sides::uniform(s.left.unwrap()));
                    }
                    Value::Dict(
                        [
                            ("left", s.left),
                            ("top", s.top),
                            ("right", s.right),
                            ("bottom", s.bottom),
                        ]
                        .into_iter()
                        .filter_map(|(k, v)| {
                            v.map(|l| {
                                (
                                    k.into(),
                                    Value::Relative(crate::entities::rel::Rel {
                                        rel: 0.0,
                                        abs: l,
                                    }),
                                )
                            })
                        })
                        .collect(),
                    )
                });
            }
            let present = match field {
                "x" => p.presence.x != Omitted,
                "y" => p.presence.y != Omitted,
                "fill" => p.presence.fill != Omitted,
                "align" => p.presence.align != Omitted,
                "breakable" => p.presence.breakable != Omitted,
                "stroke" => p.presence.stroke,
                "colspan" => p.presence.colspan,
                "rowspan" => p.presence.rowspan,
                "inset" => p.presence.inset != Omitted,
                _ => false,
            };
            return if present {
                c.get_field(field)
            } else {
                Some(match field {
                    "colspan" | "rowspan" => Value::Int(1),
                    "stroke" => Value::Dict(Default::default()),
                    "x" | "y" | "fill" | "align" | "inset" | "breakable" => Value::Auto,
                    _ => return None,
                })
            };
        }
        let presence = match field {
            "x" => self.presence.x,
            "y" => self.presence.y,
            "colspan" => {
                if self.presence.colspan {
                    Explicit
                } else {
                    Omitted
                }
            }
            "rowspan" => {
                if self.presence.rowspan {
                    Explicit
                } else {
                    Omitted
                }
            }
            "fill" => self.presence.fill,
            "stroke" => {
                if self.presence.stroke {
                    Explicit
                } else {
                    Omitted
                }
            }
            "align" => self.presence.align,
            "inset" => self.presence.inset,
            "breakable" => self.presence.breakable,
            _ => return None,
        };
        match presence {
            Omitted => {
                return if self.standalone.is_some() {
                    Some(match field {
                        "colspan" | "rowspan" => Value::Int(1),
                        "stroke" => Value::Dict(Default::default()),
                        _ => Value::Auto,
                    })
                } else {
                    None
                }
            }
            Auto => return Some(Value::Auto),
            Explicit => {}
        }
        match field {
            "body" => Some(Value::Content(self.body.clone())),
            "x" => self.x.map(|v| Value::Int(v as i64)),
            "y" => self.y.map(|v| Value::Int(v as i64)),
            "colspan" => self.colspan.map(|v| Value::Int(v as i64)),
            "rowspan" => self.rowspan.map(|v| Value::Int(v as i64)),
            "stroke" => Some(self.stroke.clone().map(Value::from).unwrap_or(Value::None)),
            "fill" => Some(self.fill.clone().map(Value::Color).unwrap_or(Value::None)),
            "align" => self.align.map(Value::Align),
            "breakable" => self.breakable.map(Value::Bool),
            "inset" => self.inset.map(|s| {
                let relative =
                    |abs| Value::Relative(crate::entities::rel::Rel { rel: 0.0, abs });
                if s.left == s.top && s.left == s.right && s.left == s.bottom {
                    relative(s.left)
                } else {
                    Value::Dict(
                        [
                            ("left".into(), relative(s.left)),
                            ("top".into(), relative(s.top)),
                            ("right".into(), relative(s.right)),
                            ("bottom".into(), relative(s.bottom)),
                        ]
                        .into_iter()
                        .collect(),
                    )
                }
            }),
            _ => None,
        }
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::TableCell(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        if let Some(r) = &self.standalone {
            let mut cell = self.clone();
            let mut r = r.as_ref().clone();
            r.visual = r.visual.map_text(transform);
            cell.standalone = Some(Arc::new(r));
            return Content::TableCell(Arc::new(cell));
        }
        if let Some(r) = &self.resolved {
            let mut cell = self.clone();
            let mut r = r.as_ref().clone();
            r.visual = r.visual.map_text(transform);
            cell.resolved = Some(Arc::new(r));
            return Content::TableCell(Arc::new(cell));
        }
        Content::TableCell(Arc::new(TableCellElem {
            source_span: self.source_span,
            resolved: self.resolved.clone(),
            standalone: self.standalone.clone(),
            presence: self.presence,
            body: self.body.map_text(transform),
            x: self.x,
            y: self.y,
            colspan: self.colspan,
            rowspan: self.rowspan,
            stroke: self.stroke.clone(),
            fill: self.fill.clone(),
            align: self.align,
            inset: self.inset,
            breakable: self.breakable,
            kind: self.kind,
        }))
    }
}

fn inset_value(s: Sides<Length>) -> crate::entities::value::Value {
    use crate::entities::value::Value;
    let rel = |abs| Value::Relative(crate::entities::rel::Rel { rel: 0.0, abs });
    if s.left == s.top && s.left == s.right && s.left == s.bottom {
        rel(s.left)
    } else {
        Value::Dict(
            [
                ("left".into(), rel(s.left)),
                ("top".into(), rel(s.top)),
                ("right".into(), rel(s.right)),
                ("bottom".into(), rel(s.bottom)),
            ]
            .into_iter()
            .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> TableCellElem {
        TableCellElem {
            source_span: crate::entities::span::Span::detached(),
            resolved: None,
            standalone: None,
            presence: Default::default(),
            body: Content::text("a"),
            x: None,
            y: None,
            colspan: None,
            rowspan: None,
            stroke: None,
            fill: None,
            align: None,
            inset: None,
            breakable: None,
            kind: TableCellKind::Auto,
        }
    }

    #[test]
    fn cell_projection_preserves_known_fields_without_defaults() {
        use crate::entities::value::Value;
        let mut cell = ex();
        let content = Content::TableCell(Arc::new(cell.clone()));
        assert_eq!(content.get_field("body"), Some(Value::Content(Content::text("a"))));
        assert_eq!(content.elem_name(), "cell");
        assert_eq!(content.get_field("x"), None);
        cell.x = Some(0);
        cell.presence.x = CellFieldPresence::Explicit;
        cell.breakable = Some(false);
        cell.presence.breakable = CellFieldPresence::Explicit;
        assert_eq!(cell.get_field("x"), Some(Value::Int(0)));
        assert_eq!(cell.get_field("breakable"), Some(Value::Bool(false)));
        assert_eq!(cell.get_field("kind"), None);
    }

    #[test]
    fn cell_presence_empty_body_has_sequence_morphology() {
        use crate::entities::value::Value;
        let mut cell = ex();
        cell.body = Content::Empty;
        let Some(Value::Content(body)) = cell.get_field("body") else {
            panic!("known body")
        };
        assert_eq!(body.elem_name(), "sequence");
        assert!(body.is_empty());
        assert!(matches!(cell.body, Content::Empty));
    }

    #[test]
    fn plain_text_delega_ao_body() {
        assert_eq!(ex().plain_text(), "a");
    }

    #[test]
    fn is_empty_delega_ao_body() {
        assert!(!ex().is_empty());
        let mut v = ex();
        v.body = Content::Empty;
        assert!(v.is_empty());
    }

    #[test]
    fn map_content_recurse_body_preserva_cosmeticos() {
        let mut c = ex();
        c.colspan = Some(2);
        c.presence.colspan = true;
        let mut f = |x: &Content| -> SourceResult<Option<Content>> {
            match x {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match c.map_content(&mut f).unwrap() {
            Content::TableCell(e) => {
                assert_eq!(e.colspan, Some(2));
                assert!(matches!(&e.body, Content::Text(s) if s.as_str() == "Z"));
            }
            _ => panic!("esperado TableCell"),
        }
    }

    fn h(e: &TableCellElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let mut v = ex();
        v.colspan = Some(3);
        v.presence.colspan = true;
        assert_ne!(h(&ex()), h(&v));
    }
}
