use comemo::Track;
use ecow::eco_format;
use fusion::diag::{HintedStrResult, SourceResult, Warned};
use fusion::foundations::{Context, Scope, StyleChain, Value};
use fusion::syntax::{Span, SyntaxMode};
use fusion::{World, engine::Sink, introspection::Introspector, layout::PagedDocument};
use semantics::eval_string;
use html::HtmlDocument;

use crate::args::{EvalCommand, Target};
use crate::compile::print_diagnostics;
use crate::set_failed;
use crate::world::SystemWorld;

/// Execute a query command.
pub fn eval(command: &'static EvalCommand) -> HintedStrResult<()> {
    let mut world =
        SystemWorld::new(command.r#in.as_ref(), &command.world, &command.process)?;

    // Reset everything and ensure that the main file is present.
    world.reset();
    world.source(world.main()).map_err(|err| err.to_string())?;

    // Compile the main file and get the introspector.
    let Warned { output, mut warnings } = match command.target {
        Target::Paged => fusion::compile::<PagedDocument>(&world)
            .map(|output| output.map(|document| document.introspector)),
        Target::Html => fusion::compile::<HtmlDocument>(&world)
            .map(|output| output.map(|document| document.introspector)),
    };

    match output {
        // Retrieve and print evaluation results.
        Ok(introspector) => {
            let mut sink = Sink::new();
            let eval_result = evaluate_expression(
                command.expression.clone(),
                &mut sink,
                &world,
                &introspector,
            );
            let errors = match &eval_result {
                Err(errors) => errors.as_slice(),
                Ok(value) => {
                    let serialized =
                        crate::serialize(value, command.format, command.pretty)?;
                    println!("{serialized}");
                    &[]
                }
            };
            // Collect additional warnings from evaluating the expression.
            warnings.extend(sink.warnings());

            print_diagnostics(
                &world,
                errors,
                &warnings,
                command.process.diagnostic_format,
            )
            .map_err(|err| eco_format!("failed to print diagnostics ({err})"))?;
        }

        // Print diagnostics.
        Err(errors) => {
            set_failed();
            print_diagnostics(
                &world,
                &errors,
                &warnings,
                command.process.diagnostic_format,
            )
            .map_err(|err| eco_format!("failed to print diagnostics ({err})"))?;
        }
    }

    Ok(())
}

/// Evaluates the expression with code syntax mode and no scope.
fn evaluate_expression(
    expression: String,
    sink: &mut Sink,
    world: &dyn World,
    introspector: &Introspector,
) -> SourceResult<Value> {
    eval_string(
        &fusion::ROUTINES,
        world.track(),
        sink.track_mut(),
        introspector.track(),
        Context::new(None, Some(StyleChain::new(&world.library().styles))).track(),
        &expression,
        Span::detached(),
        SyntaxMode::Code,
        Scope::default(),
    )
}
