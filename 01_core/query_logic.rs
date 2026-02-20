//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L1 (Core / Lógica Pura)
//! Módulo: Query
//! Responsabilidade: Lógica analítica pura para avaliação de seletores, mapeamento de resultados e geração de aviso de depreciação.

use std::fmt::Write;

use comemo::Track;
use ecow::{EcoString, eco_format};
use typst::World;
use typst::diag::{HintedStrResult, SourceDiagnostic, StrResult, bail};
use typst::engine::Sink;
use typst::foundations::{Content, Context, IntoValue, LocatableSelector, Repr, Scope};
use typst::introspection::Introspector;
use typst::syntax::{Span, SyntaxMode};
use typst_eval::eval_string;

use super::args_cli::{Input, QueryCommand};

// Reutilizamos a lógica de serialização limpa de eval logic
// Como ambas pertencem ao L1 (Core), essa dependência horizontal é aceita.
use super::eval_logic::serialize_value;

/// Realiza a busca no documento utilizando o seletor.
pub fn retrieve_matches(
    world: &dyn World,
    command: &QueryCommand,
    introspector: &Introspector,
) -> HintedStrResult<Vec<Content>> {
    let selector = eval_string(
        &typst::ROUTINES,
        world.track(),
        // TODO: propagate warnings
        Sink::new().track_mut(),
        Introspector::default().track(),
        Context::none().track(),
        &command.selector,
        Span::detached(),
        SyntaxMode::Code,
        Scope::default(),
    )
    .map_err(|errors| {
        let mut message = EcoString::from("failed to evaluate selector");
        for (i, error) in errors.into_iter().enumerate() {
            message.push_str(if i == 0 { ": " } else { ", " });
            message.push_str(&error.message);
        }
        message
    })?
    .cast::<LocatableSelector>()?;

    Ok(introspector.query(&selector.0).into_iter().collect::<Vec<_>>())
}

/// Formata/Mapeia o resultado e gera a string final serializada.
pub fn format_query_result(
    elements: Vec<Content>,
    command: &QueryCommand,
) -> StrResult<String> {
    if command.one && elements.len() != 1 {
        bail!("expected exactly one element, found {}", elements.len());
    }

    let mapped: Vec<_> = elements
        .into_iter()
        .filter_map(|c| match &command.field {
            Some(field) => c.get_by_name(field).ok(),
            _ => Some(c.into_value()),
        })
        .collect();

    if command.one {
        let Some(value) = mapped.first() else {
            bail!("no such field found for element");
        };
        serialize_value(value, command.format, command.pretty)
            .map_err(|e| eco_format!("{e}"))
    } else {
        serialize_value(&mapped, command.format, command.pretty)
            .map_err(|e| eco_format!("{e}"))
    }
}

/// Formata o aviso de depreciação indicando a sintaxe equivalente no `typst eval`.
pub fn deprecation_warning(command: &QueryCommand) -> SourceDiagnostic {
    let query = {
        let mut buf = format!("query({})", command.selector);
        let access = |field: &str| {
            if typst::syntax::is_ident(field) {
                eco_format!(".{field}")
            } else {
                eco_format!(".at({})", field.repr())
            }
        };
        match (command.one, &command.field) {
            (false, None) => {}
            (false, Some(field)) => {
                write!(buf, ".map(it => it{})", access(field)).unwrap()
            }
            (true, None) => write!(buf, ".first()").unwrap(),
            (true, Some(field)) => write!(buf, ".first(){}", access(field)).unwrap(),
        }
        shell_escape::escape(buf.into())
    };

    let eval_command = match &command.input {
        Input::Path(path) => {
            eco_format!("typst eval {query} --in {}", path.display())
        }
        Input::Stdin => eco_format!("typst eval {query}"),
    };

    SourceDiagnostic::warning(
        Span::detached(),
        "the `typst query` subcommand is deprecated",
    )
    .with_hint(eco_format!("use `{}` instead", eval_command))
}
