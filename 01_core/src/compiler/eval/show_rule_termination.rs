//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/show_rule_termination.md
//! @prompt-hash 7bde14db
//! @layer L1
//! @updated 2026-08-12
//!
//! Mecanismo de paragem do loop α de show rules — detecção de ponto-fixo e
//! ciclos por formas canónicas (`morph_canon`). Extraído de
//! `compiler/eval/rules.rs` no Passo 1009.

use crate::entities::content::Content;
use crate::entities::show::RuleId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;

/// Executa o loop α de reescrita de um único nó até terminação.
///
/// A estratégia é reescrita de termos: cada passo produz uma forma canónica
/// (`morph_canon`). Se a forma se repetir, há um ciclo; se nenhuma regra
/// casar, atingiu-se um ponto-fixo. O limite de profundidade funciona como
/// backstop de segurança.
///
/// `apply_one_step` recebe o conteúdo actual e devolve:
/// - `Ok(Some((new_content, rule_id)))` se uma regra foi aplicada;
/// - `Ok(None)` se nenhuma regra casa (ponto-fixo local).
/// Resultado do loop α: conteúdo final e flag que indica se pelo menos uma
/// regra foi aplicada.
pub(crate) type ShowRuleLoopResult = (Content, bool);

pub(crate) fn run_show_rule_loop<F>(
    initial: Content,
    mut apply_one_step: F,
    max_depth: usize,
    full_error: bool,
) -> SourceResult<ShowRuleLoopResult>
where
    F: FnMut(&Content) -> SourceResult<Option<(Content, RuleId)>>,
{
    let mut seen: Vec<(Content, usize)> = Vec::new();
    let mut transitions: Vec<Vec<RuleId>> = Vec::new();
    let mut cycle = false;
    let mut history: Vec<Content> = Vec::new();
    let mut any_applied = false;

    let mut work = initial.clone();
    let mut canon = work.morph_canon();
    seen.push((canon.clone(), 0));

    for step in 1..=max_depth {
        let applied = match apply_one_step(&work)? {
            Some((new_work, id)) => {
                any_applied = true;
                work = new_work;
                vec![id]
            }
            None => {
                // Desfecho 1 — ponto-fixo local: nenhuma regra casa.
                return Ok((work, any_applied));
            }
        };
        transitions.push(applied);

        let new_canon = work.morph_canon();

        // Desfecho 1b — ponto-fixo morfológico (output re-escreve para a mesma forma).
        if new_canon == canon {
            return Ok((work, any_applied));
        }

        // P350c (sob flag): registo para classificação do hint extra.
        if full_error {
            if history.iter().any(|h| *h == new_canon) {
                cycle = true;
            }
            history.push(new_canon.clone());
        }

        // Desfecho 2 — ciclo detectado por forma canónica repetida.
        if let Some(&start) = seen.iter().find_map(|(c, s)| {
            if *c == new_canon {
                Some(s)
            } else {
                None
            }
        }) {
            let cycle_rules: Vec<RuleId> = transitions[start..step]
                .iter()
                .flatten()
                .copied()
                .collect();
            return Err(vec![show_rule_cycle_error(start, step, cycle_rules)]);

        }

        seen.push((new_canon.clone(), step));
        canon = new_canon;
    }

    // Desfecho 3 — limite de profundidade (backstop).
    Err(vec![show_rule_depth_exceeded_error(cycle, full_error)])
}

/// Erro para ciclo de show rules (capacidade nova, sem equivalente vanilla).
fn show_rule_cycle_error(start: usize, end: usize, rules: Vec<RuleId>) -> SourceDiagnostic {
    let rules_str = rules
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    SourceDiagnostic::error(Span::detached(), "show rule cycle detected")
        .with_hint(format!(
            "the loop repeated a content form between steps {start} and {end} \
             (rules involved: {rules_str})"
        ))
}

/// Erro para limite de profundidade de show rules — mensagem primária
/// byte-idêntica ao vanilla (ADR-0033).
fn show_rule_depth_exceeded_error(cycle: bool, full_error: bool) -> SourceDiagnostic {
    let mut diag = SourceDiagnostic::error(
        Span::detached(),
        "maximum show rule depth exceeded",
    )
    .with_hint("maybe a show rule matches its own output")
    .with_hint("maybe there are too deeply nested elements");

    if full_error {
        diag = diag.with_hint(if cycle {
            "erro completo: recursão CÍCLICA — uma forma de conteúdo \
             repetiu-se no caminho de revisitação (a regra de #show \
             reescreve para algo que reaparece)"
        } else {
            "erro completo: recursão NÃO-CONVERGENTE — passou do limite \
             sem repetir nem estabilizar (verifique se a regra termina, \
             ou se é recursão legítima profunda)"
        });
    }

    diag
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::show::RuleId;

    #[test]
    fn fixed_point_immediate() {
        let content = Content::text("x");
        let result = run_show_rule_loop(
            content.clone(),
            |_c| Ok(None),
            64,
            false,
        );
        assert_eq!(result.unwrap().0.plain_text(), "x");
    }

    #[test]
    fn fixed_point_after_one_step() {
        let content = Content::text("a");
        let mut step = 0;
        let result = run_show_rule_loop(
            content,
            |c| {
                step += 1;
                if step == 1 && c.plain_text() == "a" {
                    Ok(Some((Content::text("b"), 1 as RuleId)))
                } else {
                    Ok(None)
                }
            },
            64,
            false,
        );
        assert_eq!(result.unwrap().0.plain_text(), "b");
    }

    #[test]
    fn cycle_detected() {
        let content = Content::text("a");
        let mut step = 0;
        let result = run_show_rule_loop(
            content,
            |c| {
                step += 1;
                if c.plain_text() == "a" {
                    Ok(Some((Content::text("b"), 1 as RuleId)))
                } else {
                    Ok(Some((Content::text("a"), 2 as RuleId)))
                }
            },
            64,
            false,
        );
        let err = result.unwrap_err();
        assert!(err.iter().any(|d| d.message.contains("cycle")));
    }

    #[test]
    fn depth_exceeded() {
        let content = Content::text("a");
        let mut step = 0;
        let result = run_show_rule_loop(
            content,
            |c| {
                step += 1;
                Ok(Some((Content::text(format!("{}/{}", c.plain_text(), step)), 1 as RuleId)))
            },
            2,
            false,
        );
        let err = result.unwrap_err();
        assert!(err
            .iter()
            .any(|d| d.message == "maximum show rule depth exceeded"));
    }
}
