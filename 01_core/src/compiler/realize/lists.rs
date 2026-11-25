//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/realize/lists.md
//! @prompt-hash e4ba7388
//! @layer L1
//! @updated 2026-11-24
//!
//! Formação dos conjuntos de itens e preparo dos seus rótulos.

use super::*;
use crate::compiler::eval::selector_matching;
use crate::compiler::stdlib::list_like;
use crate::entities::elements::terms::{ListKind, TermsElem};
use crate::entities::show::Transformation;
use crate::entities::style::Styles;
use crate::entities::value::Value;

/// Espécie de um item solto.
pub(crate) fn item_kind(content: &Content) -> Option<ListKind> {
    let mut content = content;
    loop {
        if let Content::Source(e) = content {
            content = &e.body;
        } else if let Content::ShowRecipe(e) = content {
            // Uma regra que substitui o item decide antes do conjunto.
            let replaced = e.rules.iter().any(|rule| {
                !matches!(rule.transform, Transformation::Style(_))
                    && !e.guards.contains(&rule.id)
                    && loose(innermost(&e.body))
                    && selector_matching::selector_matches(
                        innermost(&e.body),
                        &rule.selector,
                    )
            });
            if replaced {
                return None;
            }
            content = &e.body;
        } else if let Content::Realized(e) = content {
            content = &e.visual;
        } else if let Content::Styled(e) = content {
            content = &e.body;
        } else if let Content::Label(e) = content {
            content = &e.body;
        } else if let Content::ListItem(_) = content {
            return Some(ListKind::List);
        } else if let Content::EnumItem(_) = content {
            return Some(ListKind::Enum);
        } else if let Content::TermItem(_) = content {
            return Some(ListKind::Terms);
        } else {
            return None;
        }
    }
}

fn has_item(content: &Content) -> bool {
    if let Content::Sequence(parts) = content.source_body() {
        parts.iter().any(has_item)
    } else {
        item_kind(content).is_some()
    }
}

fn flatten(parts: &[Content], out: &mut Vec<Content>) {
    for part in parts {
        match part.source_body() {
            Content::Sequence(inner) if has_item(part) => flatten(inner, out),
            _ => out.push(part.clone()), // neutro: N16[β] — só sequência com item é aberta
        }
    }
}

/// Junta itens consecutivos; `made` recebe cada conjunto novo.
pub(crate) fn group_parts<E>(
    parts: &[Content],
    made: &mut dyn FnMut(Content) -> Result<Content, E>,
) -> Result<Option<Vec<Content>>, E> {
    if !parts.iter().any(has_item) {
        return Ok(None);
    }
    let mut flat = Vec::new();
    flatten(parts, &mut flat);
    let mut out = Vec::new();
    let mut index = 0;
    while index < flat.len() {
        let Some(kind) = item_kind(&flat[index]) else {
            out.push(flat[index].clone());
            index += 1;
            continue;
        };
        let mut items = vec![flat[index].clone()];
        let mut tight = true;
        let mut end = index + 1;
        let mut wide = false;
        for (next, part) in flat.iter().enumerate().skip(index + 1) {
            if item_kind(part) == Some(kind) {
                items.push(part.clone());
                tight &= !wide;
                wide = false;
                end = next + 1;
            } else if matches!(part.source_body(), Content::Parbreak) {
                wide = true;
            } else if !matches!(
                part.source_body(),
                Content::Space | Content::SpaceFromNewline | Content::Empty
            ) {
                break;
            }
        }
        let mut named = list_like::Named::default();
        named.insert("tight".into(), Value::Bool(tight));
        let lifted = lift(&mut items);
        let container = made(Content::list_like(kind, items, Some(Arc::new(named))))?;
        out.push(
            lifted
                .into_iter()
                .rev()
                .fold(container, |content, styles| Content::styled(content, styles)),
        );
        index = end;
    }
    Ok(Some(out))
}

fn walk(
    content: &Content,
    enter_completed: bool,
    made: &mut dyn FnMut(Content) -> SourceResult<Content>,
) -> SourceResult<Content> {
    content.map_content_pre(&mut |node| {
        if let Content::Sequence(parts) = node {
            let mut flat = Vec::new();
            flatten(parts, &mut flat);
            let mapped = flat
                .iter()
                .map(|part| {
                    if item_kind(part).is_some() {
                        walk_item(part, enter_completed, made)
                    } else {
                        walk(part, enter_completed, made)
                    }
                })
                .collect::<SourceResult<Vec<_>>>()?;
            let grouped = group_parts(&mapped, made)?;
            return Ok(Some(Content::Sequence(grouped.unwrap_or(mapped).into())));
        }
        if let Content::Terms(e) = node {
            let items = e
                .items
                .iter()
                .map(|item| walk_item(item, enter_completed, made))
                .collect::<SourceResult<Vec<_>>>()?;
            return Ok(Some(Content::Terms(Arc::new(e.with_items(items)))));
        }
        if loose(node) {
            let item = walk_item(node, enter_completed, made)?;
            let grouped = group_parts(std::slice::from_ref(&item), made)?;
            return Ok(Some(match grouped {
                Some(mut grouped) if grouped.len() == 1 => grouped.remove(0),
                Some(grouped) => Content::sequence(grouped),
                None => item,
            }));
        }
        if !enter_completed && matches!(node, Content::Realized(_)) {
            return Ok(Some(node.clone()));
        }
        Ok(None)
    })
}

fn loose(content: &Content) -> bool {
    matches!(
        content,
        Content::ListItem(_) | Content::EnumItem(_) | Content::TermItem(_)
    )
}

/// Percorre o interior de um item que já tem conjunto.
fn walk_item(
    item: &Content,
    enter_completed: bool,
    made: &mut dyn FnMut(Content) -> SourceResult<Content>,
) -> SourceResult<Content> {
    let mut entered = item_kind(item).is_none();
    item.map_content_pre(&mut |node| {
        if entered {
            return walk(node, enter_completed, made).map(Some);
        }
        entered = loose(node);
        Ok(None)
    })
}

fn any_loose(content: &Content) -> bool {
    let mut found = false;
    content.visit_content_static(&mut |node| {
        found |= loose(node);
        !found
    });
    found
}

/// Agrupa em toda a árvore; idempotente.
pub(crate) fn group(content: &Content) -> Content {
    if !any_loose(content) {
        return content.clone();
    }
    walk(content, true, &mut |made| Ok(made))
        .unwrap_or_else(|_| content.clone())
}

/// Conjunto com as declarações em vigor fixadas para a regra de exibição.
fn capture(content: Content, styles: &StyleChain) -> Content {
    let Content::Terms(e) = &content else { return content };
    let mut named = e.named.as_deref().cloned().unwrap_or_default();
    for name in list_like::names(e.kind) {
        if let Some(value) = styles.custom(&format!("{}.{name}", e.kind.name())) {
            named.entry((*name).into()).or_insert_with(|| value.clone());
        }
    }
    Content::list_like(e.kind, e.items.clone(), Some(Arc::new(named)))
}

/// Aplica as regras em vigor aos conjuntos ainda não formados de um trecho.
pub(crate) fn intercept(
    content: Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if !ctx.apply_show_rules || engine.show_rules.is_empty() || !any_loose(&content) {
        return Ok(content);
    }
    let matters = [ListKind::List, ListKind::Enum, ListKind::Terms]
        .into_iter()
        .any(|kind| matches_rule(&Content::list_like(kind, Vec::new(), None), engine));
    if !matters {
        return Ok(content);
    }
    walk(&content, false, &mut |made| {
        if matches_rule(&made, engine) {
            let made = capture(made, engine.styles);
            crate::compiler::eval::rules::intercept_content(made, ctx, engine)
        } else {
            Ok(made)
        }
    })
}

pub(crate) fn matches_rule(content: &Content, engine: &Engine<'_>) -> bool {
    engine
        .show_rules
        .iter()
        .any(|rule| selector_matching::selector_matches(content, &rule.selector))
}

/// Números dos itens, e os dos conjuntos que os contêm.
pub(crate) fn numbers(e: &TermsElem, chain: &StyleChain) -> Vec<u32> {
    let reversed = matches!(list_like::property(e, "reversed", chain), Value::Bool(true));
    let children = e.children();
    let mut number = if let Value::Int(start) = list_like::property(e, "start", chain) {
        u32::try_from(start).unwrap_or(u32::MAX)
    } else if reversed {
        u32::try_from(children.len()).unwrap_or(u32::MAX)
    } else {
        1
    };
    children
        .iter()
        .map(|item| {
            let given = item_number(item).unwrap_or(number);
            number =
                if reversed { given.saturating_sub(1) } else { given.saturating_add(1) };
            given
        })
        .collect()
}

fn item_number(content: &Content) -> Option<u32> {
    if let Content::EnumItem(e) = innermost(content) {
        e.number
    } else {
        None
    }
}

/// O conteúdo sob os envelopes transparentes.
fn innermost(content: &Content) -> &Content {
    let mut content = content;
    loop {
        content = if let Content::Source(e) = content {
            &e.body
        } else if let Content::ShowRecipe(e) = content {
            &e.body
        } else if let Content::Realized(e) = content {
            &e.visual
        } else if let Content::Styled(e) = content {
            &e.body
        } else if let Content::Label(e) = content {
            &e.body
        } else {
            return content;
        };
    }
}

/// Estilos comuns a todos os itens ficam fora do conjunto.
fn lift(items: &mut Vec<Content>) -> Vec<Styles> {
    let mut lifted = Vec::new();
    loop {
        let Some(Content::Styled(first)) = items.first().cloned() else { return lifted };
        let same = |item: &Content| {
            matches!(item, Content::Styled(e) if e.styles == first.styles)
        };
        if !items.iter().all(same) {
            return lifted;
        }
        for item in items.iter_mut() {
            if let Content::Styled(e) = item {
                *item = e.body.clone();
            }
        }
        lifted.push(first.styles.clone());
    }
}

pub(crate) fn depth(chain: &StyleChain) -> usize {
    match chain.custom("list.depth") {
        Some(Value::Int(depth)) => *depth as usize,
        _ => 0, // neutro: N16[α] — profundidade ausente é o nível exterior
    }
}

pub(crate) fn parents(chain: &StyleChain) -> Vec<u32> {
    match chain.custom("enum.parents") {
        Some(Value::Array(numbers)) => numbers
            .iter()
            .filter_map(|n| if let Value::Int(n) = n { Some(*n as u32) } else { None })
            .collect(),
        _ => Vec::new(), // neutro: N16[α] — sem enumeração em volta
    }
}

/// Estilos internos do corpo do item `number`.
pub(crate) fn body_styles(kind: ListKind, number: u32, chain: &StyleChain) -> Styles {
    let styles = Styles::new();
    match kind {
        ListKind::Terms => styles,
        ListKind::List => {
            styles.push_custom("list.depth", Value::Int(depth(chain) as i64 + 1))
        }
        ListKind::Enum => {
            let mut numbers = parents(chain);
            numbers.push(number);
            styles.push_custom(
                "enum.parents",
                Value::Array(numbers.into_iter().map(|n| Value::Int(n.into())).collect()),
            )
        }
    }
}

fn plain(value: &Value) -> bool {
    if let Value::Content(content) = value {
        matches!(
            content.source_body(),
            Content::Text(_) | Content::Symbol(_) | Content::Empty
        )
    } else if let Value::Array(values) = value {
        values.iter().all(plain)
    } else {
        !matches!(value, Value::Func(_))
    }
}

pub(crate) fn needs_preparation(content: &Content) -> bool {
    let mut needs = false;
    content.visit_content_static(&mut |node| {
        if let Content::Terms(e) = node {
            needs |= e.labels == 0
                && ["marker", "numbering"]
                    .into_iter()
                    .filter_map(|name| e.lexical(name))
                    .any(|value| !plain(value));
        } else if let Content::Styled(e) = node {
            needs |= e.styles.delta().custom.iter().any(|(key, value)| {
                matches!(key.as_str(), "list.marker" | "enum.numbering") && !plain(value)
            });
        }
        !needs
    });
    needs
}

/// Produz os rótulos que o layout não pode produzir.
pub(crate) fn prepare(
    content: &Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if !needs_preparation(content) {
        return Ok(content.clone());
    }
    prepare_node(content, ctx, engine)
}

fn prepare_node(
    content: &Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    content.map_content_pre(&mut |node| {
        if let Content::Styled(e) = node {
            let body = under(&e.styles, &e.body, ctx, engine)?;
            return Ok(Some(Content::styled(body, e.styles.clone())));
        }
        if let Content::Terms(e) = node {
            return prepare_container(e, ctx, engine).map(Some);
        }
        Ok(None)
    })
}

fn under(
    styles: &Styles,
    body: &Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let pushed = engine.styles.push_styles(styles);
    let previous = std::mem::replace(engine.styles, pushed);
    let result = prepare_node(body, ctx, engine);
    *engine.styles = previous;
    result
}

fn prepare_container(
    e: &TermsElem,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let chain = engine.styles.clone();
    let numbers = numbers(e, &chain);
    let mut items = Vec::with_capacity(e.items.len());
    for (item, number) in e.children().iter().zip(&numbers) {
        let styles = body_styles(e.kind, *number, &chain);
        items.push(under(&styles, item, ctx, engine)?);
    }
    let mut labels = Vec::new();
    if e.labels == 0 {
        match e.kind {
            ListKind::Terms => {}
            ListKind::List => {
                let marker = list_like::property(e, "marker", &chain);
                if !plain(&marker) {
                    let level = depth(&chain);
                    let label = if let Value::Func(func) = &marker {
                        call(func, vec![Value::Int(level as i64)], ctx, engine)?
                    } else {
                        marker_at(&marker, level)
                    };
                    labels = vec![label; items.len()];
                }
            }
            ListKind::Enum => {
                if let Value::Func(func) = list_like::property(e, "numbering", &chain) {
                    let full =
                        matches!(list_like::property(e, "full", &chain), Value::Bool(true));
                    for number in &numbers {
                        let mut all = if full { parents(&chain) } else { Vec::new() };
                        all.push(*number);
                        let args =
                            all.into_iter().map(|n| Value::Int(n.into())).collect();
                        labels.push(call(&func, args, ctx, engine)?);
                    }
                }
            }
        }
    } else {
        labels = e.items[e.children().len()..].to_vec();
    }
    let mut prepared = e.with_items(items);
    prepared.labels = labels.len();
    prepared.items.extend(labels);
    Ok(Content::Terms(Arc::new(prepared)))
}

/// O marcador de um nível, na lista cíclica.
pub(crate) fn marker_at(marker: &Value, level: usize) -> Content {
    if let Value::Array(markers) = marker {
        if markers.is_empty() {
            Content::Empty
        } else {
            marker_at(&markers[level % markers.len()], 0)
        }
    } else if let Value::Content(content) = marker {
        content.clone()
    } else {
        Content::Empty
    }
}

fn call(
    func: &crate::entities::func::Func,
    args: Vec<Value>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let mut scopes = crate::compiler::scopes::Scopes::new(None);
    let value = crate::compiler::eval::call_dispatch::apply_func(
        func.clone(),
        crate::entities::args::Args::positional(args),
        &mut scopes,
        ctx,
        engine,
    )?;
    let content =
        crate::compiler::eval::value_to_display_content(value).unwrap_or(Content::Empty);
    super::recipes::realize(&content, ctx, engine)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(text: &str) -> Content {
        Content::list_item(Content::text(text))
    }

    fn containers(content: &Content) -> Vec<(ListKind, usize, Option<Value>)> {
        let mut found = Vec::new();
        content.visit_content_static(&mut |node| {
            if let Content::Terms(e) = node {
                found.push((e.kind, e.children().len(), e.lexical("tight").cloned()));
            }
            true
        });
        found
    }

    #[test]
    fn groups_once_and_keeps_order() {
        let content = Content::Sequence(
            vec![
                Content::text("x"),
                Content::Sequence(vec![item("a"), Content::Space].into()),
                item("b"),
                Content::Parbreak,
                Content::enum_item(None, Content::text("c")),
                Content::Parbreak,
                Content::enum_item(None, Content::text("d")),
                Content::Parbreak,
                Content::text("y"),
            ]
            .into(),
        );
        let once = group(&content);
        assert_eq!(
            containers(&once),
            vec![
                (ListKind::List, 2, Some(Value::Bool(true))),
                (ListKind::Enum, 2, Some(Value::Bool(false))),
            ]
        );
        assert_eq!(group(&once), once);
        assert!(matches!(&once, Content::Sequence(parts)
            if matches!(parts.last(), Some(Content::Text(_)))
                && parts.iter().any(|part| matches!(part, Content::Parbreak))));
    }

    #[test]
    fn numbers_follow_start_direction_and_given_values() {
        let make = |named: &[(&str, Value)], items: Vec<Content>| {
            let mut map = list_like::Named::default();
            for (name, value) in named {
                map.insert((*name).into(), value.clone());
            }
            let Content::Terms(e) =
                Content::list_like(ListKind::Enum, items, Some(Arc::new(map)))
            else {
                unreachable!()
            };
            numbers(&e, &StyleChain::default_chain())
        };
        let plain = || Content::enum_item(None, Content::text("a"));
        let seven = || Content::enum_item(Some(7), Content::text("b"));
        assert_eq!(make(&[], vec![plain(), seven(), plain()]), [1, 7, 8]);
        let reversed = [("reversed", Value::Bool(true))];
        assert_eq!(make(&reversed, vec![plain(), seven(), plain()]), [3, 7, 6]);
        let zero = [("reversed", Value::Bool(true)), ("start", Value::Int(0))];
        assert_eq!(make(&zero, vec![plain(), plain()]), [0, 0]);
    }
}
