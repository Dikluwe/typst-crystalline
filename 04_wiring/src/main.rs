#[path = "../../04_wiring/app_wiring.rs"]
pub mod app_wiring;

#[path = "../../02_shell/args_cli.rs"]
pub mod args;

#[path = "../../20_lab/crates/typst-cli/src/compile.rs"]
pub mod compile;

#[path = "../../20_lab/crates/typst-cli/src/completions.rs"]
pub mod completions;

#[path = "../../20_lab/crates/typst-cli/src/deps.rs"]
pub mod deps;

#[path = "../../20_lab/crates/typst-cli/src/download.rs"]
pub mod download;

#[path = "../../20_lab/crates/typst-cli/src/eval.rs"]
pub mod eval;

#[path = "../../20_lab/crates/typst-cli/src/fonts.rs"]
pub mod fonts;

#[path = "../../20_lab/crates/typst-cli/src/greet.rs"]
pub mod greet;

#[path = "../../20_lab/crates/typst-cli/src/info.rs"]
pub mod info;

#[path = "../../20_lab/crates/typst-cli/src/init.rs"]
pub mod init;

#[path = "../../20_lab/crates/typst-cli/src/packages.rs"]
pub mod packages;

#[path = "../../20_lab/crates/typst-cli/src/query.rs"]
pub mod query;

#[path = "../../20_lab/crates/typst-cli/src/terminal.rs"]
pub mod terminal;

#[path = "../../20_lab/crates/typst-cli/src/timings.rs"]
pub mod timings;

#[cfg(feature = "self-update")]
#[path = "../../20_lab/crates/typst-cli/src/update.rs"]
pub mod update;

#[path = "../../20_lab/crates/typst-cli/src/watch.rs"]
pub mod watch;

#[path = "../../20_lab/crates/typst-cli/src/world.rs"]
pub mod world;

use std::cell::Cell;
use std::io::{self, Write};
use std::process::ExitCode;
use std::sync::LazyLock;

use clap::Parser;
use clap::error::ErrorKind;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::WriteColor;
use ecow::eco_format;
use serde::Serialize;
use typst::diag::{HintedStrResult, StrResult};

use crate::args::{CliArguments, Command, SerializationFormat};
use crate::timings::Timer;

thread_local! {
    /// The CLI's exit code.
    static EXIT: Cell<ExitCode> = const { Cell::new(ExitCode::SUCCESS) };
}

/// The parsed command line arguments.
static ARGS: LazyLock<CliArguments> = LazyLock::new(|| {
    CliArguments::try_parse().unwrap_or_else(|error| {
        if error.kind() == ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand {
            crate::greet::greet();
        }
        error.exit();
    })
});

/// Entry point.
fn main() -> ExitCode {
    // Inicia e carrega a nova estrutura de injeção L4 da App Inteira, mantendo compatibilidade
    let _app = app_wiring::TypstApp::default();
    println!("Wiring crystal architecture loaded: IO/Env injected na raiz.");

    // Handle SIGPIPE
    sigpipe::reset();

    let res = dispatch();

    if let Err(msg) = res {
        set_failed();
        print_error(msg.message()).expect("failed to print error");
        for hint in msg.hints() {
            print_hint(hint).expect("failed to print hint");
        }
    }

    EXIT.with(|cell| cell.get())
}

/// Execute the requested command.
fn dispatch() -> HintedStrResult<()> {
    let mut timer = Timer::new(&ARGS);

    match &ARGS.command {
        Command::Compile(command) => crate::compile::compile(&mut timer, command)?,
        Command::Watch(command) => crate::watch::watch(&mut timer, command)?,
        Command::Init(command) => crate::init::init(command)?,
        Command::Query(command) => crate::query::query(command)?,
        Command::Eval(command) => crate::eval::eval(command)?,
        Command::Fonts(command) => crate::fonts::fonts(command),
        Command::Update(command) => crate::update::update(command)?,
        Command::Completions(command) => crate::completions::completions(command),
        Command::Info(command) => crate::info::info(command)?,
    }

    Ok(())
}

/// Ensure a failure exit code.
fn set_failed() {
    EXIT.with(|cell| cell.set(ExitCode::FAILURE));
}

/// Print an application-level error (independent from a source file).
fn print_error(msg: &str) -> io::Result<()> {
    let styles = term::Styles::default();

    let mut output = terminal::out();
    output.set_color(&styles.header_error)?;
    write!(output, "error")?;

    output.reset()?;
    writeln!(output, ": {msg}")
}

/// Print an application-level hint (independent from a source file).
fn print_hint(msg: &str) -> io::Result<()> {
    let styles = term::Styles::default();

    let mut output = terminal::out();
    output.set_color(&styles.header_help)?;
    write!(output, "hint")?;

    output.reset()?;
    writeln!(output, ": {msg}")
}

/// Serialize data to the output format and convert the error to an
/// [`EcoString`].
fn serialize(
    data: &impl Serialize,
    format: SerializationFormat,
    pretty: bool,
) -> StrResult<String> {
    match format {
        SerializationFormat::Json => {
            if pretty {
                serde_json::to_string_pretty(data).map_err(|e| eco_format!("{e}"))
            } else {
                serde_json::to_string(data).map_err(|e| eco_format!("{e}"))
            }
        }
        SerializationFormat::Yaml => {
            serde_yaml::to_string(data).map_err(|e| eco_format!("{e}"))
        }
    }
}

#[cfg(not(feature = "self-update"))]
mod update {
    use typst::diag::{StrResult, bail};

    use crate::args::UpdateCommand;

    pub fn update(_: &UpdateCommand) -> StrResult<()> {
        bail!(
            "self-updating is not enabled for this executable, \
             please update with the package manager or mechanism \
             used for initial installation",
        )
    }
}
