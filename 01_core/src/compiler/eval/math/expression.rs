//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/math/expression.md
//! @prompt-hash 8d3f62bf
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Avalia um nó de expressão em modo matemático.
pub(super) fn eval_math_expr(
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    expr: Expr<'_>,
) -> SourceResult<Content> {
    match expr {
        Expr::MathIdent(ident) => {
            let name = ident.get();
            // **P780** — 0. Scope local/utilizador tem prioridade absoluta
            // (paridade `get_in_math`, vanilla — medido: `#let sin = 42;
            // $sin$` mostra "42", não o operador; `#let alpha = [x];
            // $alpha$` mostra "x", não α). Nota: o léxico já garante que
            // `ident` aqui é sempre multi-grapheme — um único grapheme
            // tokeniza como `MathText`, nunca `MathIdent`
            // (`compiler/lexer/math.rs`), logo não há fronteira
            // letra-única/multi-letra a replicar aqui: já vem resolvida
            // pelo lexer.
            if let Some(value) = scopes.get_local(name) {
                return Ok(super::value_to_display_content(value.clone())
                    .unwrap_or(Content::Empty));
            }
            // 1. Símbolo grego ou operador Unicode (alpha → α etc.)
            if let Some(sym) = crate::compiler::math::symbols::ident_to_unicode(name) {
                return Ok(Content::Symbol(sym.into()));
            }
            // 2. P301 — auto-lookup scope `math` (42 operadores P299 via SSoT MathOp).
            if let Some(op) = lookup_math_op(scopes, name) {
                return Ok(op);
            }
            // P795 — auto-lookup no módulo `sym` para símbolos bare (ex.: arrow, dif)
            if let Some(sym) = crate::compiler::stdlib::sym::sym_lookup(name) {
                // P820 — símbolo depreciado (ex.: `join`): resolve com
                // warning verbatim do vanilla (`$join$` medido @1:1).
                if let Some(msg) = crate::compiler::stdlib::sym::sym_deprecation(name) {
                    engine.sink.warn_note(ident.span(), msg, "");
                }
                return Ok(Content::Symbol(sym.value));
            }
            // 3. **P780** — identificador realmente desconhecido: erro com
            // hints (paridade `unknown_variable_math`, vanilla). Substitui
            // o fallback pré-P780 ("manter como MathIdent" — regressão
            // pré-P301 preservada até aqui).
            Err(vec![unknown_variable_math(ident.span(), name, scopes.has_global(name))])
        }
        Expr::MathText(text) => match text.get() {
            MathTextKind::Grapheme(s) => Ok(Content::Symbol(s.into())),
            MathTextKind::Number(s) => Ok(Content::MathText(s.into())),
        },
        Expr::MathPrimes(primes) => Ok(Content::math_primes(primes.count())),
        Expr::MathShorthand(sh) => Ok(Content::Symbol(sh.get().to_string().into())),
        Expr::MathFrac(frac) => {
            let num_deparenthesized =
                matches!(frac.num(), Expr::Math(math) if math.was_deparenthesized());
            let denom_deparenthesized =
                matches!(frac.denom(), Expr::Math(math) if math.was_deparenthesized());
            let num = eval_math_expr(scopes, ctx, engine, frac.num())?;
            let den = eval_math_expr(scopes, ctx, engine, frac.denom())?;
            Ok(Content::math_frac_full(
                num,
                den,
                None,
                num_deparenthesized,
                denom_deparenthesized,
                true,
            ))
        }
        Expr::MathAttach(attach) => {
            let base = eval_math_expr(scopes, ctx, engine, attach.base())?;
            let sub = attach
                .bottom()
                .map(|e| eval_math_expr(scopes, ctx, engine, e))
                .transpose()?;
            let sup = attach
                .top()
                .map(|e| eval_math_expr(scopes, ctx, engine, e))
                .transpose()?;

            let prime_count = attach.primes().map(|p| p.count()).unwrap_or(0);
            let primes = (prime_count > 0).then(|| Content::math_primes(prime_count));

            Ok(Content::math_attach(
                base,
                sup.map_or(MathAttachSlot::Omitted, MathAttachSlot::Present),
                sub.map_or(MathAttachSlot::Omitted, MathAttachSlot::Present),
                MathAttachSlot::Omitted,
                MathAttachSlot::Omitted,
                primes.map_or(MathAttachSlot::Omitted, MathAttachSlot::Present),
                MathAttachSlot::Omitted,
            ))
        }
        Expr::MathRoot(root) => {
            // root.index() retorna Option<u8> — converter para Content::MathText se presente
            let index = root.index().map(|n| Content::MathText(n.to_string().into()));
            let radicand = eval_math_expr(scopes, ctx, engine, root.radicand())?;
            Ok(Content::math_root(index, radicand, false))
        }
        Expr::Math(inner) => eval_math_content(scopes, ctx, engine, inner),

        // MathDelimited: preservar estrutura para layout extensível (Passo 42)
        Expr::MathDelimited(delim) => {
            let body = eval_math_content(scopes, ctx, engine, delim.body())?;
            // Extrair o char delimitador do expr (MathText ou MathIdent com 1 char)
            let open_str = delim.open().to_untyped().text();
            let close_str = delim.close().to_untyped().text();
            let open = open_str.as_str().chars().next().unwrap_or('(');
            let close = close_str.as_str().chars().next().unwrap_or(')');
            // **P996** — `\` é quebra de linha ANTES do emparelhamento lr
            // (documentação oficial + vanilla medido em P995:
            // `expand_multiline_fence`, `ir/multiline.rs:56-107` — os
            // delimitadores dimensionam-se pelo segmento próprio, nunca
            // pela pilha). Corpo com `Linebreak` ao nível do topo NÃO
            // emparelha: os delimitadores ficam como glifos normais
            // (tamanho natural), um por linha, e o caminho de
            // quebra/grelha existente (P991) centra as linhas.
            if let Content::MathSequence(items) = &body {
                if items.iter().any(|c| matches!(c, Content::Linebreak(_))) {
                    let mut new_items: Vec<Content> = Vec::with_capacity(items.len() + 2);
                    new_items.push(Content::MathText(open_str.as_str().into()));
                    new_items.extend(items.iter().cloned());
                    new_items.push(Content::MathText(close_str.as_str().into()));
                    return Ok(Content::MathSequence(std::sync::Arc::from(new_items)));
                }
            }
            Ok(Content::math_delimited(open, body, close))
        }

        // frac() e outras funções nativas de math (Passo 38)
        Expr::FuncCall(call) => {
            super::expression_call::eval_math_func_call(scopes, ctx, engine, call)
        }

        // Ponto de alinhamento (`&`) e quebra de linha (`\\`) em equações
        Expr::MathAlignPoint(_) => Ok(Content::math_align_point()),
        Expr::Linebreak(_) => Ok(Content::linebreak()),

        // **P782** — qualquer outro `Expr` (não especificamente math) chegado
        // a uma sequência math: `#expr` (`SyntaxKind::Hash` →
        // `embedded_code_expr`, `compiler/parse/math.rs:62`) e field access
        // bare (`sym.suit.heart` sem `#` — o lexer já monta um nó
        // `SyntaxKind::FieldAccess` genérico, `compiler/parse/math.rs:63-64`,
        // "The lexer manages creating full FieldAccess nodes if needed")
        // produzem literalmente o **mesmo** `Expr::FieldAccess`/`Expr::Ident`
        // — sem distinção sintáctica entre os dois casos nesta arquitectura,
        // ao contrário do vanilla (`Expr::MathFieldAccess` dedicado vs
        // `Expr::FieldAccess` genérico via Hash). Paridade conceptual com
        // `ExprExt::eval_display` (vanilla) = `self.eval(vm)?.display()`:
        // delega ao avaliador genérico de código, depois converte para
        // `Content` via `value_to_display_content` (P780). Cobre `#hc`
        // (Ident), `#sym.suit.heart`/`sym.suit.heart` bare (FieldAccess),
        // `#let x = 5` (agora executa de facto — antes descartado em
        // silêncio, nunca mutava o scope) e qualquer outro literal
        // (`#5`, `#"x"`, ...). Seguro por construção: `eval_expr` já trata
        // `Expr::MathIdent`/`MathText`/etc. como "fronteira deliberada"
        // (`Ok(Value::None)`, `eval/mod.rs`) — mas esses nunca chegam aqui,
        // são apanhados pelos braços específicos acima. **Excepção**:
        // `Expr::FieldAccess` (`sym.suit.heart`, bare ou `#`) tem de passar
        // por `eval_math_callee`, não `eval_expr` directo — o alvo do
        // access (`sym`) é lexado como `Expr::MathIdent` mesmo dentro de um
        // nó `FieldAccess` (montado pelo lexer math, `compiler/lexer/math.rs`
        // — comentário "The lexer manages creating full FieldAccess nodes
        // if needed"); `eval_expr` genérico trata `Expr::MathIdent` como
        // fronteira deliberada (`Value::None`), o que faria o field access
        // falhar com "field access não suportado em none" (medido, mesma
        // causa que `eval_math_callee` já contorna para o caminho de
        // callee — P772y).
        other @ Expr::FieldAccess(_) => {
            let value = eval_math_callee(scopes, ctx, engine, other)?;
            match value {
                Value::Symbol(s) => Ok(Content::Symbol(s.value)),
                other_val => {
                    Ok(super::value_to_display_content(other_val)
                        .unwrap_or(Content::Empty))
                }
            }
        }
        other => {
            let value = eval_expr(other, scopes, ctx, engine)?;
            Ok(super::value_to_display_content(value).unwrap_or(Content::Empty))
        }
    }
}
