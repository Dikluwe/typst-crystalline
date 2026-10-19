//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/call_dispatch/call_spans.md
//! @prompt-hash 3a14deee
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) struct FloatIsNanCallSpans {
    call: Span,
    positional: Vec<Span>,
    named: IndexMap<EcoString, Span, FxBuildHasher>,
    has_spread: bool,
}

impl FloatIsNanCallSpans {
    pub(super) fn capture(call: FuncCallNode<'_>) -> Self {
        let mut positional = Vec::new();
        let mut named = IndexMap::default();
        let mut has_spread = false;

        for arg in call.args().items() {
            match arg {
                Arg::Pos(expr) => positional.push(expr.span()),
                Arg::Named(named_arg) => {
                    named.insert(named_arg.name().as_str().into(), named_arg.span());
                }
                Arg::Spread(_) => has_spread = true,
            }
        }

        Self { call: call.span(), positional, named, has_spread }
    }

    pub(super) fn anchor(&self, args: &Args, bound: bool) -> Span {
        if let Some((name, _)) = args.named.first() {
            return self.named.get(name.as_str()).copied().unwrap_or(args.span);
        }

        if bound {
            if args.items.is_empty() || self.has_spread {
                args.span
            } else {
                self.positional.first().copied().unwrap_or(args.span)
            }
        } else {
            match args.items.as_slice() {
                [] => self.call,
                [_] if !self.has_spread => {
                    self.positional.first().copied().unwrap_or(args.span)
                }
                [_, _, ..] if !self.has_spread => {
                    self.positional.get(1).copied().unwrap_or(args.span)
                }
                _ => args.span,
            }
        }
    }
}

/// Identidades nativas P1293-B que exigem âncoras por argumento. A seleção é
/// feita sobre o `FuncRepr` já resolvido, nunca sobre texto do callee.
#[derive(Clone, Copy)]
pub(super) enum P1293MathNative {
    Attach,
    Binom,
    Mono,
    Script,
}

impl P1293MathNative {
    pub(super) fn from_func(func: &Func) -> Option<Self> {
        let FuncRepr::Native(native) = func.repr() else {
            return None;
        };
        match native.name {
            "attach" => Some(Self::Attach),
            "binom" => Some(Self::Binom),
            "mono" => Some(Self::Mono),
            "script" => Some(Self::Script),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct P1293NamedSpan {
    pub(super) full: Span,
    pub(super) value: Span,
}

/// Metadados sintáticos privados e transitórios para os quatro constructors
/// math P1293-B. Não atravessam `Args` nem sobrevivem à chamada.
pub(super) struct P1293MathCallSpans {
    call: Span,
    positional: Vec<Span>,
    named: IndexMap<EcoString, P1293NamedSpan, FxBuildHasher>,
    has_spread: bool,
}

impl P1293MathCallSpans {
    pub(super) fn capture(call: FuncCallNode<'_>) -> Self {
        let mut positional = Vec::new();
        let mut named = IndexMap::default();
        let mut has_spread = false;

        for arg in call.args().items() {
            match arg {
                Arg::Pos(expr) => positional.push(expr.span()),
                Arg::Named(named_arg) => {
                    named.insert(
                        named_arg.name().as_str().into(),
                        P1293NamedSpan {
                            full: named_arg.span(),
                            value: named_arg.expr().span(),
                        },
                    );
                }
                Arg::Spread(_) => has_spread = true,
            }
        }

        Self { call: call.span(), positional, named, has_spread }
    }

    pub(super) fn positional(&self, index: usize, fallback: Span) -> Span {
        self.positional.get(index).copied().unwrap_or(fallback)
    }

    pub(super) fn named_full(&self, name: &str, fallback: Span) -> Span {
        self.named.get(name).map(|span| span.full).unwrap_or(fallback)
    }

    pub(super) fn named_value(&self, name: &str, fallback: Span) -> Span {
        self.named.get(name).map(|span| span.value).unwrap_or(fallback)
    }

    pub(super) fn structural_content(value: &Value) -> bool {
        matches!(
            value,
            Value::Content(_)
                | Value::LocatedContent(_, _)
                | Value::Str(_)
                | Value::Symbol(_)
        )
    }

    pub(super) fn style_content(value: &Value) -> bool {
        matches!(value, Value::Content(_) | Value::Str(_))
    }

    pub(super) fn anchor(&self, identity: P1293MathNative, args: &Args) -> Span {
        if self.has_spread {
            return args.span;
        }

        match identity {
            P1293MathNative::Attach => {
                match args.items.as_slice() {
                    [] => return self.call,
                    [_, _, ..] => return self.positional(1, args.span),
                    [_] => {}
                }
                if !Self::structural_content(&args.items[0]) {
                    return self.positional(0, args.span);
                }
                for (name, value) in &args.named {
                    if matches!(name.as_str(), "t" | "b" | "tl" | "bl" | "tr" | "br") {
                        if !matches!(value, Value::None)
                            && !Self::structural_content(value)
                        {
                            return self.named_value(name, args.span);
                        }
                    } else {
                        return self.named_full(name, args.span);
                    }
                }
            }
            P1293MathNative::Binom => {
                match args.items.len() {
                    0 => {
                        return args
                            .named
                            .get_key_value("upper")
                            .map(|(name, _)| self.named_full(name, args.span))
                            .unwrap_or(self.call);
                    }
                    1 => return self.call,
                    _ => {}
                }
                if let Some((name, _)) = args.named.first() {
                    return self.named_full(name, args.span);
                }
                for (index, value) in args.items.iter().enumerate() {
                    if !Self::structural_content(value) {
                        return self.positional(index, args.span);
                    }
                }
            }
            P1293MathNative::Mono => {
                if let Some((name, _)) = args.named.first() {
                    return self.named_full(name, args.span);
                }
                match args.items.as_slice() {
                    [] => return self.call,
                    [_, _, ..] => return self.positional(1, args.span),
                    [_] => {}
                }
                if !Self::style_content(&args.items[0]) {
                    return self.positional(0, args.span);
                }
            }
            P1293MathNative::Script => {
                for (name, _) in &args.named {
                    if name.as_str() != "cramped" {
                        return self.named_full(name, args.span);
                    }
                }
                if let Some(value) = args.named.get("cramped") {
                    if !matches!(value, Value::Bool(_)) {
                        return self.named_value("cramped", args.span);
                    }
                }
                match args.items.as_slice() {
                    [] => return self.call,
                    [_, _, ..] => return self.positional(1, args.span),
                    [_] => {}
                }
                if !Self::style_content(&args.items[0]) {
                    return self.positional(0, args.span);
                }
            }
        }

        args.span
    }
}

#[derive(Clone, Copy)]
pub(super) enum P1293HtmlNative {
    Normal,
    Void,
}

impl P1293HtmlNative {
    pub(super) fn from_func(func: &Func) -> Option<Self> {
        let name = match func.repr() {
            FuncRepr::Native(native) => native.name,
            _ => return None,
        };
        let kind = match name {
            "button" | "iframe" | "select" | "template" | "video" => Self::Normal,
            "col" | "wbr" => Self::Void,
            _ => return None,
        };
        let module = crate::compiler::stdlib::make_html_module();
        let Value::Func(root) = &module else {
            return None;
        };
        let Value::Func(reference) = root.namespace()?.get(name)? else {
            return None;
        };
        let (Some(actual), Some(expected)) =
            (func.native_fn_addr(), reference.native_fn_addr())
        else {
            return None;
        };
        std::ptr::fn_addr_eq(actual, expected).then_some(kind)
    }
}

pub(super) struct P1293HtmlCallSpans {
    pub(super) kind: P1293HtmlNative,
    pub(super) positional: Vec<Span>,
    pub(super) named: IndexMap<EcoString, P1293NamedSpan, FxBuildHasher>,
    pub(super) has_spread: bool,
}

impl P1293HtmlCallSpans {
    pub(super) fn capture(call: FuncCallNode<'_>, kind: P1293HtmlNative) -> Self {
        let mut positional = Vec::new();
        let mut named = IndexMap::default();
        let mut has_spread = false;
        for arg in call.args().items() {
            match arg {
                Arg::Pos(expr) => positional.push(expr.span()),
                Arg::Named(named_arg) => {
                    named.insert(
                        named_arg.name().as_str().into(),
                        P1293NamedSpan {
                            full: named_arg.span(),
                            value: named_arg.expr().span(),
                        },
                    );
                }
                Arg::Spread(_) => has_spread = true,
            }
        }
        Self { kind, positional, named, has_spread }
    }

    pub(super) fn anchor(&self, args: &Args, message: &str) -> Span {
        if self.has_spread {
            return args.span;
        }
        match self.kind {
            P1293HtmlNative::Void if !args.items.is_empty() => {
                return self.positional.first().copied().unwrap_or(args.span);
            }
            P1293HtmlNative::Normal if args.items.len() > 1 => {
                return self.positional.get(1).copied().unwrap_or(args.span);
            }
            P1293HtmlNative::Normal
                if args
                    .items
                    .first()
                    .is_some_and(|value| !matches!(value, Value::Content(_))) =>
            {
                return self.positional.first().copied().unwrap_or(args.span);
            }
            _ => {}
        }
        if let Some(name) = message.strip_prefix("unexpected argument: ") {
            return self.named.get(name).map(|span| span.full).unwrap_or(args.span);
        }
        self.named.first().map(|(_, span)| span.value).unwrap_or(args.span)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn v18_float_anchors_cardinality_and_precedence() {
        let capture = |has_spread| FloatIsNanCallSpans {
            call: span(1),
            positional: vec![span(2), span(3), span(4), span(5)],
            named: IndexMap::default(),
            has_spread,
        };
        for (count, bound, spread, expected) in [
            (0, false, false, 1),
            (0, false, true, 1),
            (0, true, false, 9),
            (0, true, true, 9),
            (1, false, false, 2),
            (1, false, true, 9),
            (1, true, false, 2),
            (1, true, true, 9),
            (2, false, false, 3),
            (2, false, true, 9),
            (2, true, false, 2),
            (2, true, true, 9),
            (4, false, false, 3),
            (4, false, true, 9),
            (4, true, false, 2),
            (4, true, true, 9),
        ] {
            let args = Args::from_parts(
                vec![Value::Int(0); count],
                IndexMap::default(),
                span(9),
            );
            assert_eq!(capture(spread).anchor(&args, bound), span(expected));
        }
        for bound in [false, true] {
            for spread in [false, true] {
                let mut captured = capture(spread);
                captured.named.insert("value".into(), span(6));
                let mut named = IndexMap::default();
                named.insert("value".into(), Value::Int(0));
                let args = Args::from_parts(vec![], named, span(9));
                assert_eq!(captured.anchor(&args, bound), span(6));
            }
        }
        let mut missing = capture(false);
        missing.positional.clear();
        for count in [1, 2] {
            let args = Args::from_parts(
                vec![Value::Int(0); count],
                IndexMap::default(),
                span(9),
            );
            assert_eq!(missing.anchor(&args, false), span(9));
        }
    }

    #[test]
    fn v18_math_anchors_cardinality_and_precedence() {
        let capture = |has_spread| P1293MathCallSpans {
            call: span(1),
            positional: vec![span(2), span(3), span(4), span(5)],
            named: IndexMap::default(),
            has_spread,
        };
        for identity in
            [P1293MathNative::Attach, P1293MathNative::Mono, P1293MathNative::Script]
        {
            for (count, expected) in [(0, 1), (1, 9), (2, 3), (4, 3)] {
                let args = Args::from_parts(
                    vec![Value::Str("x".into()); count],
                    IndexMap::default(),
                    span(9),
                );
                assert_eq!(capture(false).anchor(identity, &args), span(expected));
                assert_eq!(capture(true).anchor(identity, &args), span(9));
            }
            let args =
                Args::from_parts(vec![Value::Int(1)], IndexMap::default(), span(9));
            assert_eq!(capture(false).anchor(identity, &args), span(2));
        }
        for (identity, count, name, value, expected) in [
            (P1293MathNative::Attach, 0, "unknown", Value::Int(1), 1),
            (P1293MathNative::Attach, 2, "unknown", Value::Int(1), 3),
            (P1293MathNative::Attach, 1, "unknown", Value::Int(1), 6),
            (P1293MathNative::Attach, 1, "t", Value::Int(1), 7),
            (P1293MathNative::Mono, 0, "unknown", Value::Int(1), 6),
            (P1293MathNative::Mono, 2, "unknown", Value::Int(1), 6),
            (P1293MathNative::Script, 0, "unknown", Value::Int(1), 6),
            (P1293MathNative::Script, 2, "unknown", Value::Int(1), 6),
            (P1293MathNative::Script, 0, "cramped", Value::Int(1), 7),
            (P1293MathNative::Script, 2, "cramped", Value::Int(1), 7),
            (P1293MathNative::Script, 0, "cramped", Value::Bool(true), 1),
            (P1293MathNative::Script, 2, "cramped", Value::Bool(true), 3),
        ] {
            for spread in [false, true] {
                let mut captured = capture(spread);
                captured.named.insert(
                    name.into(),
                    P1293NamedSpan { full: span(6), value: span(7) },
                );
                let mut named = IndexMap::default();
                named.insert(name.into(), value.clone());
                let args =
                    Args::from_parts(vec![Value::Str("x".into()); count], named, span(9));
                assert_eq!(
                    captured.anchor(identity, &args),
                    span(if spread { 9 } else { expected })
                );
            }
        }
    }

    use super::*;
    use std::num::NonZeroU16;

    fn span(start: usize) -> Span {
        let id = crate::entities::file_id::FileId::from_raw(NonZeroU16::new(9).unwrap());
        Span::from_range(id, start..start + 1)
    }

    #[test]
    fn float_anchor_selects_call_or_first_argument() {
        let spans = FloatIsNanCallSpans {
            call: span(1),
            positional: vec![span(2)],
            named: IndexMap::default(),
            has_spread: false,
        };
        assert_eq!(spans.anchor(&Args::positional(vec![]), false), span(1));
        assert_eq!(spans.anchor(&Args::positional(vec![Value::Int(3)]), false), span(2));
    }
}
