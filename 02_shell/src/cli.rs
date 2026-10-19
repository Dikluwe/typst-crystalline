//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/cli.md
//! @prompt-hash c74ad6bd
//! @layer L2
//! @updated 2026-10-17
//!
//! CLI do compilador cristalino (Passo 117, ADR-0049).
//!
//! Migrado de L3 (`ColorWhen`, `resolve_colored_with`) e L4 (`Args`,
//! `resolve_colored`) para respeitar a definição fundacional de L2
//! como camada "CLI — interface com utilizador".
//!
//! Exposições públicas:
//! - `ColorWhen` — enum do modo de coloração (clap `ValueEnum`).
//! - `RunIntent` — struct pura com `input`, `output`, `root`,
//!   `font_paths`, `colored`; produto de `parse()` consumido por L4.
//! - `parse() -> RunIntent` — ponto de entrada da CLI.
//! - `resolve_colored_with(choice, no_color, is_tty) -> bool` —
//!   função pura (decisão de precedência flag > NO_COLOR > isatty).

use std::ffi::OsString;
use std::io::IsTerminal;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::builder::TypedValueParser;
use clap::{CommandFactory, Parser, Subcommand};
use typst_core::compiler::lang::heading_supplement_for_lang;
use typst_core::entities::document_format_options::{PageRange, PageRanges, PdfStandard};
use typst_core::entities::elements::math_attach::MathAttachSlot;
use typst_core::entities::value::Value;

/// Separador de paths em env vars estilo `PATH` (Passo 123).
///
/// Alinhado com vanilla typst-cli: `':'` em Unix, `';'` em Windows.
/// Usado por `--font-path` para suportar `TYPST_FONT_PATHS=/a:/b`.
const ENV_PATH_SEP: char = if cfg!(windows) { ';' } else { ':' };

/// P1500: aceita valores vazios sem os transformar em diretórios de fontes.
fn font_path_value_parser() -> impl TypedValueParser<Value = Option<PathBuf>> {
    clap::builder::OsStringValueParser::new()
        .map(|value| (!value.is_empty()).then(|| value.into()))
}

/// Trunca o hash do commit a 8 chars — mirror de `typst_utils::display_commit`
/// do vanilla (P796, `shell/cli.md` §"Decisão — número de versão do CLI").
/// `None` (sem `.git`, ex. build a partir de tarball) → `"unknown commit"`,
/// mesmo fallback do vanilla.
fn display_commit(commit: Option<&'static str>) -> &'static str {
    const LENGTH: usize = 8;
    match commit {
        Some(s) => &s[..s.len().min(LENGTH)],
        None => "unknown commit",
    }
}

#[cfg(test)]
mod p1360_ab_tests {
    use super::*;

    fn parsed_pretty(command: &str, spelling: Option<&str>) -> Option<bool> {
        let mut argv = vec!["typst", command, "p1360.typ"];
        if let Some(spelling) = spelling {
            argv.push(spelling);
        }
        let parsed =
            Args::try_parse_from(argv).expect("P1360 accepted spelling must parse");
        match parsed.command {
            Command::Compile(args) => args.compile.pretty,
            Command::Watch(args) => args.compile.pretty,
            _ => panic!("P1360 test selected an unexpected command"),
        }
    }

    #[test]
    fn p1360_ab_parser_tri_state_compile_watch_and_w() {
        for command in ["compile", "watch", "w"] {
            assert_eq!(parsed_pretty(command, None), None, "{command}: absent");
            assert_eq!(
                parsed_pretty(command, Some("--pretty")),
                Some(true),
                "{command}: bare"
            );
            assert_eq!(
                parsed_pretty(command, Some("--pretty=true")),
                Some(true),
                "{command}: true"
            );
            assert_eq!(
                parsed_pretty(command, Some("--pretty=false")),
                Some(false),
                "{command}: false"
            );
        }
    }

    #[test]
    fn p1360_ab_parser_separates_positional_and_rejects_invalid_booleans() {
        for command in ["compile", "watch", "w"] {
            let separate = Args::try_parse_from([
                "typst",
                command,
                "p1360.typ",
                "--pretty",
                "false",
            ])
            .expect(
                "the token after bare --pretty remains available to positional parsing",
            );
            let compile = match separate.command {
                Command::Compile(args) => args.compile,
                Command::Watch(args) => args.compile,
                _ => panic!("P1360 test selected an unexpected command"),
            };
            assert_eq!(compile.pretty, Some(true), "{command}: bare --pretty");
            assert_eq!(
                compile.output,
                Some(PathBuf::from("false")),
                "{command}: the separated token must be the output positional"
            );

            for invalid in ["--pretty=maybe", "--pretty="] {
                let error =
                    Args::try_parse_from(["typst", command, "p1360.typ", invalid])
                        .expect_err("an invalid explicit boolean must be rejected");
                assert_eq!(error.exit_code(), 2);
                let message = error.to_string();
                assert!(message.contains("--pretty"));
                if invalid.ends_with("maybe") {
                    assert!(message.contains("maybe"));
                }
                assert!(
                    message.contains("true") && message.contains("false"),
                    "diagnostic must expose the boolean domain: {message}"
                );
            }
        }
    }
}

#[cfg(test)]
mod p1500_font_path_tests {
    use super::*;

    #[test]
    fn p1500_empty_slots_preserve_nonempty_order_and_duplicates() {
        let first = "p1500-first-font-dir";
        let second = "p1500-second-font-dir";
        let delimited = format!("{first}{ENV_PATH_SEP}{second}{ENV_PATH_SEP}");
        let parsed = Args::try_parse_from([
            "typst",
            "compile",
            "p1500.typ",
            "--font-path",
            "",
            "--font-path",
            delimited.as_str(),
            "--font-path",
            first,
            "--font-path=",
        ])
        .expect("P1500: tokens vazios devem parsear");
        let Command::Compile(args) = parsed.command else {
            panic!("P1500: compile esperado");
        };
        let intent =
            compile_intent(args.compile, HtmlSerialization::Crystalline, false, None);
        assert_eq!(
            intent.font_paths,
            vec![PathBuf::from(first), PathBuf::from(second), PathBuf::from(first)]
        );
    }
}

#[cfg(test)]
mod p1501_completion_shell_tests {
    use super::*;

    #[test]
    fn p1501_nushell_entra_no_dominio_sem_perder_shells_existentes() {
        for shell in ["bash", "elvish", "fish", "nushell", "powershell", "zsh"] {
            let parsed = Args::try_parse_from(["typst", "completions", shell])
                .unwrap_or_else(|error| panic!("{shell}: {error}"));
            assert!(matches!(parsed.command, Command::Completions(_)));
        }

        let invalid = Args::try_parse_from(["typst", "completions", "invalid-shell"])
            .expect_err("shell desconhecido deve continuar inválido");
        assert_eq!(invalid.exit_code(), 2);
    }
}

/// Modo de coloração para diagnostics (ADR-0048).
///
/// Enum dedicada a *diagnóstico do compilador*; não confundir com
/// `clap::ColorChoice` (que controla o output do próprio clap).
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum ColorWhen {
    /// Cores activas se stderr é terminal e `NO_COLOR` ausente.
    Auto,
    /// Cores sempre activas, mesmo em pipe.
    Always,
    /// Cores sempre desactivadas.
    Never,
}

/// Formato de saída do compilador (P866).
///
/// Alinhado com o vanilla (`typst-cli/src/args.rs::OutputFormat`) mas
/// restrito aos formatos paginados suportados pelo subset actual do
/// cristalino. `Html` e `Bundle` ficam fora de escopo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum OutputFormat {
    /// PDF — formato principal, implementado.
    Pdf,
    /// PNG — requer rasterização (não implementada no cristalino).
    Png,
    /// SVG — requer exporter SVG (não implementado no cristalino).
    Svg,
    /// HTML semântico — backend experimental, sem layout paginado.
    Html,
    /// Directory containing bundle documents and assets.
    Bundle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum FeatureArg {
    Html,
    A11yExtras,
    Bundle,
}

fn resolve_features(
    values: &[FeatureArg],
) -> typst_core::entities::compiler_features::Features {
    use typst_core::entities::compiler_features::{Feature, Features};
    let mut features = Features::default();
    for value in values {
        match value {
            FeatureArg::Html => features.enable(Feature::Html),
            FeatureArg::A11yExtras => features.enable(Feature::A11yExtras),
            FeatureArg::Bundle => features.enable(Feature::Bundle),
        }
    }
    features
}

// Passo 115 escopo (a): positional `input output`.
// Passo 116 (ADR-0048): + `--color=auto|always|never`.
// Passo 117 (ADR-0049): `Args` vive em L2.
// Passo 120 (ADR-0051): `output` opcional + `-o/--output` sinónimo
// + default derivado (`input.with_extension("pdf")`).
// Passo 121 (ADR-0051): + `--root DIR` (fallback para input.parent()).
// Passo 122 (ADR-0051): + `--font-path DIR` (repetível; raw para L3).
// Passo 123 (ADR-0051): env vars TYPST_ROOT + TYPST_FONT_PATHS;
// `--font-path` ganha `value_delimiter = ENV_PATH_SEP`.
// P796 — `version` deixa de ser o atributo implícito do clap (que lia
// `CARGO_PKG_VERSION` do crate `typst-shell`, "0.1.0", inconsistente com
// `sys.version`). Mostra a versão de **paridade** com a linguagem Typst
// (mesma constante que `sys.version`, `entities/version.md` §9) + hash do
// commit HEAD do próprio repositório cristalino (capturado em build.rs,
// mecânica copiada directamente do vanilla — decisão em `shell/cli.md`
// §"Decisão — número de versão do CLI"). Formato final: `typst 0.15.1
// (⟨commit curto⟩)`, mesmo formato do vanilla ratificado.
#[derive(Parser, Debug)]
#[command(
    name = "typst",
    version = format!(
        "{}.{}.{} ({})",
        typst_core::entities::version::PARITY_VERSION.0,
        typst_core::entities::version::PARITY_VERSION.1,
        typst_core::entities::version::PARITY_VERSION.2,
        display_commit(option_env!("TYPST_COMMIT_SHA")),
    ),
    about = "Typst compiler (crystalline)"
)]
struct Args {
    /// Command to run.
    #[command(subcommand)]
    command: Command,

    /// When to use coloured diagnostics.
    #[arg(long = "color", value_enum, default_value_t = ColorWhen::Auto, global = true)]
    color: ColorWhen,

    /// Add a custom certificate authority for HTTPS downloads.
    #[arg(long = "cert", value_name = "PATH", global = true)]
    cert_path: Option<PathBuf>,
}

/// **P1468** — newtype de L2 que hospeda `impl FromStr` para `--pages`,
/// reproduzindo exatamente o algoritmo `split('-').map(trim)` do vanilla
/// ratificado (`typst-cli/src/args.rs:779-813`), incluindo
/// `parse_page_number`. Reexporta `PageRange` de L1 (`entities::
/// document_format_options`) só como o carrier tipado do intervalo já
/// parseado.
#[derive(Debug, Clone, PartialEq, Eq)]
struct PageRangeArg(PageRange);

impl FromStr for PageRangeArg {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.split('-').map(str::trim).collect::<Vec<_>>().as_slice() {
            [] | [""] => Err("page range must not be empty"),
            [single_page] => {
                let page_number = pages_arg_parse_page_number(single_page)?;
                Ok(PageRangeArg(PageRange::at(page_number)))
            }
            ["", ""] => Err("page range must have start or end"),
            [start, ""] => Ok(PageRangeArg(PageRange {
                start: Some(pages_arg_parse_page_number(start)?),
                end: None,
            })),
            ["", end] => Ok(PageRangeArg(PageRange {
                start: None,
                end: Some(pages_arg_parse_page_number(end)?),
            })),
            [start, end] => {
                let start = pages_arg_parse_page_number(start)?;
                let end = pages_arg_parse_page_number(end)?;
                if start > end {
                    Err("page range must end at a page after the start")
                } else {
                    Ok(PageRangeArg(PageRange { start: Some(start), end: Some(end) }))
                }
            }
            [_, _, _, ..] => Err("page range must have a single hyphen"),
        }
    }
}

/// Espelha `parse_page_number` do vanilla (`typst-cli/src/args.rs:806-813`).
fn pages_arg_parse_page_number(value: &str) -> Result<NonZeroUsize, &'static str> {
    if value == "0" {
        Err("page numbers start at one")
    } else {
        NonZeroUsize::from_str(value).map_err(|_| "not a valid page number")
    }
}

/// **P1469** (escopo reduzido — contrato de linguagem) — os 17 nomes brutos
/// aceites por `--pdf-standard`, mesma ordem/grafia do `ValueEnum` clap do
/// vanilla ratificado (medido em `shell/cli.md` P1469: "possible values:
/// 1.4, 1.5, 1.6, 1.7, 2.0, a-1b, a-1a, a-2b, a-2u, a-2a, a-3b, a-3u, a-3a,
/// a-4, a-4f, a-4e, ua-1"). Newtype fino de L2 que só faz parsing sintático
/// (mensagem nativa do clap, incluindo `tip:` de sugestão por proximidade);
/// a validação de conflitos entre nomes pertence a `entities::
/// document_format_options::PdfStandards::new` (L1), consumida por
/// `infra/pipeline.md`/`wiring.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum PdfStandardArg {
    #[value(name = "1.4")]
    V1_4,
    #[value(name = "1.5")]
    V1_5,
    #[value(name = "1.6")]
    V1_6,
    #[value(name = "1.7")]
    V1_7,
    #[value(name = "2.0")]
    V2_0,
    #[value(name = "a-1b")]
    A1b,
    #[value(name = "a-1a")]
    A1a,
    #[value(name = "a-2b")]
    A2b,
    #[value(name = "a-2u")]
    A2u,
    #[value(name = "a-2a")]
    A2a,
    #[value(name = "a-3b")]
    A3b,
    #[value(name = "a-3u")]
    A3u,
    #[value(name = "a-3a")]
    A3a,
    #[value(name = "a-4")]
    A4,
    #[value(name = "a-4f")]
    A4f,
    #[value(name = "a-4e")]
    A4e,
    #[value(name = "ua-1")]
    Ua1,
}

impl From<PdfStandardArg> for PdfStandard {
    fn from(value: PdfStandardArg) -> Self {
        match value {
            PdfStandardArg::V1_4 => PdfStandard::V1_4,
            PdfStandardArg::V1_5 => PdfStandard::V1_5,
            PdfStandardArg::V1_6 => PdfStandard::V1_6,
            PdfStandardArg::V1_7 => PdfStandard::V1_7,
            PdfStandardArg::V2_0 => PdfStandard::V2_0,
            PdfStandardArg::A1b => PdfStandard::A1b,
            PdfStandardArg::A1a => PdfStandard::A1a,
            PdfStandardArg::A2b => PdfStandard::A2b,
            PdfStandardArg::A2u => PdfStandard::A2u,
            PdfStandardArg::A2a => PdfStandard::A2a,
            PdfStandardArg::A3b => PdfStandard::A3b,
            PdfStandardArg::A3u => PdfStandard::A3u,
            PdfStandardArg::A3a => PdfStandard::A3a,
            PdfStandardArg::A4 => PdfStandard::A4,
            PdfStandardArg::A4f => PdfStandard::A4f,
            PdfStandardArg::A4e => PdfStandard::A4e,
            PdfStandardArg::Ua1 => PdfStandard::Ua1,
        }
    }
}

#[derive(Debug, clap::Args)]
struct CompileArgs {
    /// Enables in-development compiler features.
    #[arg(long = "features", value_enum, value_delimiter = ',', action = clap::ArgAction::Append)]
    features: Vec<FeatureArg>,
    /// Input .typ file.
    input: Option<PathBuf>,

    /// Output PDF file (positional). Defaults to input with `.pdf`
    /// extension if omitted. `-o/--output` flag takes precedence.
    output: Option<PathBuf>,

    /// Output PDF file. Alternative to the positional argument;
    /// wins if both are provided.
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    output_flag: Option<PathBuf>,

    /// Project root directory. Used to locate the main file and
    /// (in future) virtualize imports. Defaults to the parent
    /// directory of `input`, or `.` if input has no parent.
    #[arg(long = "root", env = "TYPST_ROOT", value_name = "DIR")]
    root: Option<PathBuf>,

    /// Additional directories to search for fonts. May be repeated.
    /// Also accepts a single value with `:` (Unix) / `;` (Windows)
    /// as separator, e.g. via `TYPST_FONT_PATHS=/a:/b`.
    /// Invalid paths are silently skipped by the font discoverer.
    #[arg(
        long = "font-path",
        env = "TYPST_FONT_PATHS",
        value_name = "DIR",
        value_delimiter = ENV_PATH_SEP,
        value_parser = font_path_value_parser(),
        action = clap::ArgAction::Append,
    )]
    font_paths: Vec<Option<PathBuf>>,

    /// Show an extended third hint for show-rule recursion errors.
    #[arg(long = "full-error", action = clap::ArgAction::SetTrue)]
    full_error: bool,

    /// P507 — escreve tempos das fases do pipeline em JSON.
    #[arg(long = "timings-json", value_name = "FILE")]
    timings_json: Option<PathBuf>,

    /// P617 — UUID externo para fixar o `DocumentID` do pacote XMP entre
    /// compilações. `InstanceID` continua aleatório.
    #[arg(long = "document-id", env = "CRYSTALLINE_DOCUMENT_ID", value_name = "UUID")]
    document_id: Option<String>,

    /// P694 — par `chave=valor` exposto em `sys.inputs`. Repetível. Os valores
    /// são sempre strings (paridade vanilla: `--input n=42` → `sys.inputs.n == "42"`).
    #[arg(long = "input", value_name = "chave=valor", action = clap::ArgAction::Append)]
    inputs: Vec<String>,

    /// P866 — formato de saída explícito. Vence a detecção por extensão.
    #[arg(long = "format", short = 'f', value_enum, value_name = "FORMAT")]
    format: Option<OutputFormat>,

    /// P956 — Emitir content streams no formato compacto (operadores mínimos,
    /// PDF menor). Por omissão emite-se o modo verbose, com a semântica de
    /// operadores do vanilla. Sem efeito em `--format png|svg`.
    #[arg(long = "compact", action = clap::ArgAction::SetTrue)]
    compact: bool,

    /// Pretty-print the selected output format when supported.
    /// An omitted value is equivalent to `--pretty=true`; an explicit value
    /// must use `=` so the absent state remains distinguishable in L4.
    #[arg(
        long = "pretty",
        num_args = 0..=1,
        default_missing_value = "true",
        require_equals = true
    )]
    pretty: Option<bool>,

    /// The PPI (pixels per inch) to use for PNG export.
    #[arg(long = "ppi")]
    ppi: Option<f64>,

    /// Define explicitamente se PDFs devem conter estrutura lógica tagueada.
    /// Sem valor, `--pdf-tagged` equivale a `--pdf-tagged=true`.
    #[arg(
        long = "pdf-tagged",
        num_args = 0..=1,
        default_missing_value = "true",
        require_equals = true
    )]
    pdf_tagged: Option<bool>,

    /// Desativar a estrutura lógica PDF. Por omissão, PDFs são tagueados.
    #[arg(long = "no-pdf-tags", action = clap::ArgAction::SetTrue)]
    no_pdf_tags: bool,

    /// **P1469** (escopo reduzido — contrato de linguagem; retificação de
    /// P1286, que a mantinha oculta/descartada) — conjunto de standards
    /// PDF. Ausente = delega ao documento/default. Lista crua, **não**
    /// validada como `PdfStandards` em L2 (a validação de conflitos entre
    /// nomes pertence a `entities::document_format_options::PdfStandards`,
    /// consumida por `infra/pipeline.md`/`wiring.md`).
    #[arg(long = "pdf-standard", value_enum, value_delimiter = ',')]
    pdf_standard: Vec<PdfStandardArg>,

    /// P980 — FERRAMENTA DE DIAGNÓSTICO: emite o PDF do oráculo de
    /// paridade de operador (transformações `Tj`/`TJ` e futuras checks de
    /// paridade) em vez do PDF normal. Não é um formato de produção.
    #[arg(long = "oracle-pdf", action = clap::ArgAction::SetTrue)]
    oracle_pdf: bool,

    /// **P1468** — quais páginas exportar no PDF. Ausente = todas. Tokens
    /// separados por vírgula, cada um `N`, `A-B`, `A-` ou `-B` (números
    /// físicos, 1-indexed, não afetados pelo contador de página da
    /// linguagem). Só afeta o export PDF (ver scope-out medido em
    /// `shell/cli.md` P1468: PNG/SVG multi-ficheiro não existe ainda).
    #[arg(long = "pages", value_delimiter = ',')]
    pages: Option<Vec<PageRangeArg>>,
}

#[derive(Debug, clap::Args)]
struct CompileCommandArgs {
    #[command(flatten)]
    compile: CompileArgs,

    /// Select the HTML escaping profile. This only affects HTML output.
    #[arg(long = "html-serialization", value_enum, default_value_t)]
    html_serialization: HtmlSerialization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum, Default)]
pub enum HtmlSerialization {
    #[default]
    Crystalline,
    Vanilla,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Compile an input file into a supported output format.
    #[command(alias = "c")]
    Compile(CompileCommandArgs),
    /// Compile continuously when the input or one of its dependencies changes.
    #[command(visible_alias = "w")]
    Watch(WatchArgs),
    /// Evaluate a piece of Typst code.
    Eval(EvalArgs),
    /// List all discovered fonts in system and custom font paths.
    Fonts(FontsArgs),
    /// Generate shell completion scripts.
    Completions(CompletionsArgs),
    /// Display debugging information about Typst.
    Info(InfoArgs),
    /// Create a new project from a template package.
    Init(InitArgs),
    /// Process an input file to extract metadata (deprecated).
    #[command(hide = true)]
    Query(QueryArgs),
}

#[derive(Debug, clap::Args)]
struct WatchArgs {
    #[command(flatten)]
    compile: CompileArgs,
    #[arg(long = "no-serve")]
    no_serve: bool,
    #[arg(long = "no-reload")]
    no_reload: bool,
    #[arg(long = "port")]
    port: Option<u16>,
}

/// Shells aceites pelo subcomando público `completions` (P1501).
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
#[clap(rename_all = "lowercase")]
pub enum CompletionShell {
    Bash,
    Elvish,
    Fish,
    Nushell,
    PowerShell,
    Zsh,
}

#[derive(Debug, clap::Args)]
struct CompletionsArgs {
    /// Shell to generate completions for.
    #[arg(value_enum)]
    shell: CompletionShell,
}

#[derive(Debug, clap::Args)]
struct InfoArgs {
    /// Machine-readable output format.
    #[arg(long, short = 'f', value_enum)]
    format: Option<InfoFormat>,
    /// Pretty-print JSON output.
    #[arg(long, requires = "format")]
    pretty: bool,
}

#[derive(Debug, clap::Args)]
struct InitArgs {
    /// Template package, with an optional explicit version.
    template: String,
    /// Directory to create. Defaults to the package name.
    directory: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum InfoFormat {
    Json,
}

#[derive(Debug, clap::Args)]
struct FontsArgs {
    /// Additional directories to search recursively for fonts.
    #[arg(
        long = "font-path",
        env = "TYPST_FONT_PATHS",
        value_name = "DIR",
        value_delimiter = ENV_PATH_SEP,
        value_parser = font_path_value_parser(),
        action = clap::ArgAction::Append,
    )]
    font_paths: Vec<Option<PathBuf>>,
    /// Do not discover fonts installed in the operating system.
    #[arg(long)]
    ignore_system_fonts: bool,
    /// Also list style variants of each font family.
    #[arg(long)]
    variants: bool,
}

#[derive(Debug, clap::Args)]
struct QueryArgs {
    input: PathBuf,
    selector: String,
    #[arg(long)]
    field: Option<String>,
    #[arg(long)]
    one: bool,
    #[arg(long, default_value = "json")]
    format: QueryFormat,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum, Default)]
pub enum QueryFormat {
    #[default]
    Json,
    Yaml,
}

#[derive(Debug, clap::Args)]
struct EvalArgs {
    /// The piece of Typst code to evaluate.
    expression: String,
    /// Evaluate with context from this input document (`-` reads stdin).
    #[arg(long = "in", value_name = "INPUT")]
    input: Option<PathBuf>,
    /// Output serialization format.
    #[arg(long, default_value = "json")]
    format: EvalFormat,
    /// Pretty-print JSON output.
    #[arg(long)]
    pretty: bool,
    /// Enables in-development compiler features.
    #[arg(long = "features", value_enum, value_delimiter = ',', action = clap::ArgAction::Append)]
    features: Vec<FeatureArg>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum, Default)]
pub enum EvalFormat {
    #[default]
    Json,
    Yaml,
    Raw,
}

/// Intenção de execução — output puro de L2 para L4 (ADR-0049).
///
/// L2 traduz argumentos + env vars + isatty para este struct.
/// L4 consome directamente sem conhecer clap ou env vars.
#[derive(Debug, Clone)]
pub struct CompileIntent {
    pub features: typst_core::entities::compiler_features::Features,
    pub input: PathBuf,
    pub output: PathBuf,
    /// P866 — formato de saída resolvido pela extensão ou `--format`.
    pub output_format: OutputFormat,
    /// Raw L2 choice; L4 maps it to the L3 serializer mode.
    pub html_serialization: HtmlSerialization,
    pub root: PathBuf,
    pub font_paths: Vec<PathBuf>,
    pub colored: bool,
    /// **P350c — flag de "erro completo"** (capacidade interna; origem desta flag).
    /// Quando ligada, o erro de recursão de `#show` ganha um 3º hint classificando
    /// cíclico/não-convergente (a capacidade vive em L1, `EvalContext::full_error`).
    /// **Débito P350c**: (1) o parsing CLI (`--full-error` em `Args` → aqui — hoje
    /// `parse()` fixa `false`); (2) o fio desta flag até L1 pelo caminho **interno**
    /// de L3 (a assinatura pública de `compile_to_pdf_bytes` **não** muda). Até o
    /// débito, este campo é a **casa** da origem (ao lado de `colored`), default `false`.
    pub full_error: bool,
    /// P507 — path opcional para JSON com tempos das fases do pipeline.
    pub timings_json: Option<PathBuf>,
    /// **P617** — `DocumentID` externo de 16 bytes (UUID validado), ou `None`.
    pub document_id: Option<[u8; 16]>,
    /// **P694** — pares `--input chave=valor` validados (raw strings). L3
    /// converte para `SysInputs` em `SystemWorld::with_inputs`. Vazio por
    /// omissão → `sys.inputs == (:)`.
    pub inputs: Vec<(String, String)>,
    /// **P956** — dado cru da flag `--compact` (L2 não importa `StreamMode`
    /// de L3; a tradução bool → modo é em L4, `wiring.md` §P956). Ausente →
    /// verbose (padrão); presente → compacto. Sem efeito em PNG/SVG.
    pub compact: bool,
    /// Raw tri-state selected by `compile --pretty`; L4 resolves `None` to
    /// `false` and forwards the result only to the selected encoder.
    pub pretty: Option<bool>,
    /// Raw PNG override. Absence remains distinguishable for document options.
    pub ppi: Option<f64>,
    /// Escolha moderna tri-state; `None` delega ao legado `no_pdf_tags`.
    pub pdf_tagged: Option<bool>,
    /// Polaridade pública do CLI; L4 traduz para `PdfTags`.
    pub no_pdf_tags: bool,
    /// **P980** — dado cru da flag `--oracle-pdf` (diagnóstico). L4 chama
    /// `compile_to_pdf_bytes_oracle` quando presente (só PDF).
    pub oracle_pdf: bool,
    pub cert_path: Option<PathBuf>,
    /// **P1468** — seleção de páginas de `--pages`, já convertida ao
    /// carrier tipado de L1. `None` = todas as páginas. L4 decide a
    /// precedência face a `#set format.pdf(pages: ...)` e a interação com
    /// `tagged` (`wiring.md`/`infra/pipeline.md` P1468).
    pub pages: Option<PageRanges>,
    /// **P1469** (escopo reduzido — contrato de linguagem) — lista **crua**
    /// de `--pdf-standard`, não validada como `PdfStandards` em L2 (a
    /// validação de conflitos entre nomes pertence a `entities::
    /// document_format_options::PdfStandards::new`, consumida por
    /// `wiring.md`/`infra/pipeline.md` P1469). Vazio = ausente, delega ao
    /// documento/default.
    pub pdf_standard: Vec<PdfStandard>,
}

#[derive(Debug)]
pub struct EvalIntent {
    pub expression: String,
    pub input: Option<PathBuf>,
    pub format: EvalFormat,
    pub pretty: bool,
    pub colored: bool,
    pub cert_path: Option<PathBuf>,
    pub features: typst_core::entities::compiler_features::Features,
}

#[derive(Debug)]
pub struct QueryIntent {
    pub input: PathBuf,
    pub selector: String,
    pub field: Option<String>,
    pub one: bool,
    pub pretty: bool,
    pub format: QueryFormat,
    pub colored: bool,
    pub cert_path: Option<PathBuf>,
}

/// Specific, pure migration hint for the deprecated query command.
pub fn query_deprecation_hint(intent: &QueryIntent) -> String {
    let mut expression = format!("query({})", intent.selector);
    let access = intent.field.as_deref().map(|field| {
        if typst_core::compiler::lexer::is_ident(field) {
            format!(".{field}")
        } else {
            let repr = typst_core::compiler::eval::repr_value_for_serialization(
                &typst_core::entities::value::Value::Str(field.into()),
            );
            format!(".at({repr})")
        }
    });
    if intent.one {
        expression.push_str(".first()");
        if let Some(access) = access {
            expression.push_str(&access);
        }
    } else if let Some(access) = access {
        expression.push_str(&format!(".map(it => it{access})"));
    }
    #[cfg(unix)]
    let escaped = shell_escape::unix::escape(expression.into());
    #[cfg(not(unix))]
    let escaped = shell_escape::windows::escape(expression.into());
    let input = if intent.input == std::path::Path::new("-") {
        String::new()
    } else {
        format!(" --in {}", intent.input.display())
    };
    format!("use `typst eval {escaped}{input}` instead")
}

#[derive(Debug)]
pub struct FontsIntent {
    pub font_paths: Vec<PathBuf>,
    pub include_system: bool,
    pub variants: bool,
}

#[derive(Debug)]
pub struct CompletionsIntent {
    pub shell: CompletionShell,
}

#[derive(Debug)]
pub struct InfoIntent {
    pub format: Option<InfoFormat>,
    pub pretty: bool,
    pub cert_path: Option<PathBuf>,
}

#[derive(Debug)]
pub struct InitIntent {
    pub template: String,
    pub directory: Option<PathBuf>,
    pub cert_path: Option<PathBuf>,
}

#[derive(Debug)]
pub struct WatchIntent {
    pub compile: CompileIntent,
    pub no_serve: bool,
    pub no_reload: bool,
    pub port: Option<u16>,
}

#[derive(Debug)]
pub enum RunIntent {
    Compile(CompileIntent),
    Watch(WatchIntent),
    Eval(EvalIntent),
    Query(QueryIntent),
    Fonts(FontsIntent),
    Completions(CompletionsIntent),
    Info(InfoIntent),
    Init(InitIntent),
}

/// Ponto de entrada público da CLI.
///
/// Em erro de argumentos, `Args::parse()` (clap) imprime mensagem
/// em stderr e termina o processo com exit 2. Em sucesso, devolve
/// `RunIntent` com `output`, `root` e `colored` já resolvidos.
pub fn parse() -> RunIntent {
    let args = Args::parse_from(normalize_legacy_args(std::env::args_os()));
    let colored = resolve_colored(&args.color);
    let cert_path = resolve_cert_path(args.cert_path, std::env::var_os("TYPST_CERT"));
    match args.command {
        Command::Compile(compile) => RunIntent::Compile(compile_intent(
            compile.compile,
            compile.html_serialization,
            colored,
            cert_path,
        )),
        Command::Watch(watch) => RunIntent::Watch(WatchIntent {
            compile: compile_intent(
                watch.compile,
                HtmlSerialization::Crystalline,
                colored,
                cert_path,
            ),
            no_serve: watch.no_serve,
            no_reload: watch.no_reload,
            port: watch.port,
        }),
        Command::Eval(eval) => RunIntent::Eval(EvalIntent {
            features: resolve_features(&eval.features),
            expression: eval.expression,
            input: eval.input,
            format: eval.format,
            pretty: eval.pretty,
            colored,
            cert_path,
        }),
        Command::Fonts(fonts) => RunIntent::Fonts(FontsIntent {
            font_paths: fonts.font_paths.into_iter().flatten().collect(),
            include_system: !fonts.ignore_system_fonts,
            variants: fonts.variants,
        }),
        Command::Completions(args) => {
            RunIntent::Completions(CompletionsIntent { shell: args.shell })
        }
        Command::Info(args) => RunIntent::Info(InfoIntent {
            format: args.format,
            pretty: args.pretty,
            cert_path,
        }),
        Command::Init(args) => RunIntent::Init(InitIntent {
            template: args.template,
            directory: args.directory,
            cert_path,
        }),
        Command::Query(query) => RunIntent::Query(QueryIntent {
            input: query.input,
            selector: query.selector,
            field: query.field,
            one: query.one,
            pretty: query.pretty,
            format: query.format,
            colored,
            cert_path,
        }),
    }
}

/// Árvore clap pública usada pela geração de completions. É a mesma fonte de
/// verdade do parser, incluindo comandos ocultos.
pub fn command() -> clap::Command {
    Args::command()
}

/// Commit completo capturado no build de L2. A saída humana decide se o
/// abrevia; formatos estruturados recebem sempre o valor integral.
pub fn build_commit() -> Option<&'static str> {
    option_env!("TYPST_COMMIT_SHA")
}

fn compile_intent(
    args: CompileArgs,
    html_serialization: HtmlSerialization,
    colored: bool,
    cert_path: Option<PathBuf>,
) -> CompileIntent {
    let input = args.input.unwrap_or_else(|| {
        Args::command()
            .error(
                clap::error::ErrorKind::MissingRequiredArgument,
                "the following required argument was not provided: <INPUT>",
            )
            .exit()
    });
    if input == Path::new("-") && args.output.is_none() && args.output_flag.is_none() {
        Args::command()
            .find_subcommand_mut("compile")
            .expect("compile belongs to the CLI command tree")
            .clone()
            .bin_name("typst compile")
            .mut_arg("input", |arg| arg.required(true))
            .mut_arg("output", |arg| arg.required(true))
            .error(
                clap::error::ErrorKind::MissingRequiredArgument,
                "the following required arguments were not provided:\n  <OUTPUT>",
            )
            .exit();
    }
    // P866 — resolver formato antes de validações dependentes de caminho.
    let output_format = resolve_output_format_with(
        args.format.as_ref(),
        args.output_flag.as_ref().or(args.output.as_ref()),
        OutputFormat::Pdf,
    );
    let output = if output_format == OutputFormat::Bundle
        && args.output.is_none()
        && args.output_flag.is_none()
    {
        input.with_extension("")
    } else {
        resolve_output_with(&input, args.output.as_ref(), args.output_flag.as_ref())
    };
    let root = resolve_root_with(args.root.as_ref(), &input);

    // P617 — validar UUID antes de converter para bytes; erro claro em L2.
    let document_id = args.document_id.as_deref().and_then(parse_uuid_bytes);
    if args.document_id.is_some() && document_id.is_none() {
        eprintln!("error: invalid document ID: expected a UUID (e.g., f81d4fae-7dec-11d0-a765-00a0c91e6bf6)");
        std::process::exit(2);
    }

    // P694 — validar `--input chave=valor` (split no primeiro `=`; chave não
    // vazia). Erro claro em L2 (exit 2) em entrada mal formada.
    let mut inputs: Vec<(String, String)> = Vec::with_capacity(args.inputs.len());
    for raw in &args.inputs {
        match parse_input_entry(raw) {
            Some(pair) => inputs.push(pair),
            None => {
                eprintln!("error: invalid input '{raw}': expected the form key=value");
                std::process::exit(2);
            }
        }
    }

    CompileIntent {
        features: resolve_features(&args.features),
        input,
        output,
        output_format,
        html_serialization,
        root,
        font_paths: args.font_paths.into_iter().flatten().collect(),
        colored,
        // P350c → P428 (DEBT-59): a flag CLI é parseada e fiada a L4/L1.
        // Default `false` preserva comportamento byte-idêntico ao vanilla.
        full_error: args.full_error,
        timings_json: args.timings_json,
        document_id,
        inputs,
        compact: args.compact,
        pretty: args.pretty,
        ppi: args.ppi,
        pdf_tagged: args.pdf_tagged,
        no_pdf_tags: args.no_pdf_tags,
        oracle_pdf: args.oracle_pdf,
        cert_path,
        pages: args
            .pages
            .map(|ranges| PageRanges::new(ranges.into_iter().map(|r| r.0).collect())),
        pdf_standard: args.pdf_standard.into_iter().map(PdfStandard::from).collect(),
    }
}

fn resolve_cert_path(flag: Option<PathBuf>, env: Option<OsString>) -> Option<PathBuf> {
    flag.or_else(|| env.map(PathBuf::from))
}

/// Insere o subcomando canónico `compile` para a grafia histórica
/// `typst INPUT [OUTPUT] ...`. Help/version e subcomandos explícitos não são
/// alterados. A compatibilidade ocorre antes do parsing e não aparece no help.
fn normalize_legacy_args(args: impl IntoIterator<Item = OsString>) -> Vec<OsString> {
    let mut args: Vec<OsString> = args.into_iter().collect();
    if args.len() <= 1 {
        return args;
    }
    let tokens: Vec<&str> = args[1..].iter().filter_map(|arg| arg.to_str()).collect();
    let explicit_command = tokens.iter().any(|token| {
        matches!(
            *token,
            "compile"
                | "c"
                | "eval"
                | "fonts"
                | "completions"
                | "info"
                | "init"
                | "watch"
                | "w"
                | "query"
        )
    });
    let meta_only = tokens
        .iter()
        .any(|token| matches!(*token, "-h" | "--help" | "-V" | "--version"));
    if !explicit_command && !meta_only {
        args.insert(1, OsString::from("compile"));
    }
    args
}

/// Serializa o resultado público de `typst eval`.
pub fn serialize_eval(
    value: &Value,
    format: EvalFormat,
    pretty: bool,
) -> Result<Vec<u8>, String> {
    match format {
        EvalFormat::Raw => match value {
            Value::Str(s) => Ok(s.as_bytes().to_vec()),
            Value::Bytes(b) => Ok(b.as_slice().to_vec()),
            other => Err(format!(
                "cannot print {} in raw format\nhint: `--format=raw` only supports strings and bytes",
                eval_type_name(other)
            )),
        },
        EvalFormat::Json => serialize_semantic(&value_to_semantic(value), true, pretty),
        EvalFormat::Yaml => serialize_semantic(&value_to_semantic(value), false, false),
    }
}

/// Serializa resultados do subcomando deprecated `query`.
pub fn serialize_query(
    elements: &[Value],
    field: Option<&str>,
    one: bool,
    pretty: bool,
) -> Result<Vec<u8>, String> {
    serialize_query_with_format(elements, field, one, pretty, QueryFormat::Json)
}

/// Serializa query no formato já resolvido pelo parser L2.
pub fn serialize_query_with_format(
    elements: &[Value],
    field: Option<&str>,
    one: bool,
    pretty: bool,
    format: QueryFormat,
) -> Result<Vec<u8>, String> {
    let mut values = elements.iter().map(value_to_semantic).collect::<Vec<_>>();
    let output = if one {
        if values.len() != 1 {
            return Err(format!("expected exactly one element, found {}", values.len()));
        }
        let value = values.remove(0);
        match field {
            Some(field) => semantic_field(&value, field)
                .cloned()
                .ok_or_else(|| "no such field found for element".to_string())?,
            None => value,
        }
    } else {
        if let Some(field) = field {
            values = values
                .into_iter()
                .filter_map(|value| semantic_field(&value, field).cloned())
                .collect();
        }
        SemanticValue::Array(values)
    };
    serialize_semantic(
        &output,
        matches!(format, QueryFormat::Json),
        pretty && matches!(format, QueryFormat::Json),
    )
}

#[derive(Clone)]
enum SemanticValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<SemanticValue>),
    Object(Vec<(String, SemanticValue)>),
}

fn semantic_field<'a>(
    value: &'a SemanticValue,
    field: &str,
) -> Option<&'a SemanticValue> {
    let SemanticValue::Object(fields) = value else { return None };
    fields.iter().find(|(name, _)| name == field).map(|(_, value)| value)
}

fn content_to_semantic(
    content: &typst_core::entities::content::Content,
) -> SemanticValue {
    use typst_core::entities::content::Content;
    match content {
        Content::Source(r) => return content_to_semantic(&r.body),
        Content::ShowRecipe(r) => return content_to_semantic(&r.body),
        Content::Located(r) => return content_to_semantic(&r.body),
        Content::Realized(r) => return content_to_semantic(&r.visual),
        _ => {}
    }
    if let Content::Label(label) = content {
        let mut value = content_to_semantic(&label.body);
        if let SemanticValue::Object(fields) = &mut value {
            fields.push((
                "label".into(),
                SemanticValue::String(format!("<{}>", label.name)),
            ));
        }
        return value;
    }

    let mut fields = vec![(
        "func".into(),
        SemanticValue::String(content_semantic_func_name(content).into()),
    )];
    match content {
        Content::Empty => fields.push(("children".into(), SemanticValue::Array(vec![]))),
        Content::FigureCaption(_) => {
            for name in ["position", "separator", "body"] {
                if let Some(value) = content.get_field(name) {
                    fields.push((name.into(), value_to_semantic(&value)));
                }
            }
        }
        Content::Raw(_) => {
            for name in ["text", "block", "lang"] {
                if let Some(value) = content.get_field(name) {
                    fields.push((name.into(), value_to_semantic(&value)));
                }
            }
        }
        Content::MathMatrix(_) => {
            for name in ["delim", "align", "augment", "row-gap", "column-gap", "rows"] {
                if let Some(value) = content.get_field(name) {
                    fields.push((name.into(), value_to_semantic(&value)));
                }
            }
        }
        Content::Text(text) => {
            fields.push(("text".into(), SemanticValue::String(text.to_string())))
        }
        Content::Sequence(children) | Content::MathSequence(children) => fields.push((
            "children".into(),
            SemanticValue::Array(children.iter().map(content_to_semantic).collect()),
        )),
        Content::Par { body } | Content::SmallCaps { body } => {
            fields.push(("body".into(), content_to_semantic(body)))
        }
        Content::Heading(heading) => {
            fields.extend([
                ("level".into(), SemanticValue::Int(heading.level as i64)),
                ("depth".into(), SemanticValue::Int(heading.level as i64)),
                ("offset".into(), SemanticValue::Int(0)),
                ("numbering".into(), SemanticValue::Null),
                (
                    "supplement".into(),
                    content_to_semantic(&Content::text(heading_supplement_for_lang(
                        None,
                    ))),
                ),
                ("outlined".into(), SemanticValue::Bool(heading.outlined)),
                (
                    "bookmarked".into(),
                    heading
                        .bookmarked
                        .map(SemanticValue::Bool)
                        .unwrap_or_else(|| SemanticValue::String("auto".into())),
                ),
                ("hanging-indent".into(), SemanticValue::String("auto".into())),
                ("body".into(), content_to_semantic(&heading.body)),
            ]);
        }
        Content::OutlineEntry(entry) => {
            fields.push(("level".into(), SemanticValue::Int(entry.level as i64)));
            fields.push(("element".into(), content_to_semantic(&entry.element)));
            fields.push((
                "fill".into(),
                entry
                    .fill
                    .as_ref()
                    .map(content_to_semantic)
                    .unwrap_or(SemanticValue::Null),
            ));
        }
        Content::Title(title) => {
            fields.push(("body".into(), content_to_semantic(&title.body)))
        }
        Content::Strong(elem) => {
            fields.push(("body".into(), content_to_semantic(&elem.body)))
        }
        Content::Emph(elem) => {
            fields.push(("body".into(), content_to_semantic(&elem.body)))
        }
        Content::Figure(figure) => {
            fields.push(("body".into(), content_to_semantic(&figure.body)));
            fields.push(("alt".into(), SemanticValue::Null));
            fields.push(("placement".into(), SemanticValue::Null));
            fields.push(("scope".into(), SemanticValue::String("column".into())));
            let kind = figure.kind.as_deref().unwrap_or("image");
            let supplement = Content::text(figure_supplement(kind));
            let counter = format!("counter(figure.where(kind: {kind}))");
            fields.push((
                "caption".into(),
                figure
                    .caption
                    .as_ref()
                    .map(|caption| {
                        figure_caption_to_semantic(caption, kind, &supplement, &counter)
                    })
                    .unwrap_or(SemanticValue::Null),
            ));
            fields.push(("kind".into(), SemanticValue::String(kind.into())));
            fields.push(("supplement".into(), content_to_semantic(&supplement)));
            fields.push(("numbering".into(), SemanticValue::String("1".into())));
            fields.push(("gap".into(), SemanticValue::String("0.65em".into())));
            fields.push(("outlined".into(), SemanticValue::Bool(true)));
            fields.push(("counter".into(), SemanticValue::String(counter)));
        }
        Content::TableCell(_) => {
            for name in [
                "body",
                "x",
                "y",
                "colspan",
                "rowspan",
                "inset",
                "align",
                "fill",
                "stroke",
                "breakable",
            ] {
                if let Some(value) = content.get_field(name) {
                    fields.push((name.into(), value_to_semantic(&value)));
                }
            }
        }
        Content::Equation(equation) => fields.extend([
            ("block".into(), SemanticValue::Bool(equation.block)),
            ("numbering".into(), SemanticValue::Null),
            ("number-align".into(), SemanticValue::String("end + horizon".into())),
            ("supplement".into(), content_to_semantic(&Content::text("Equation"))),
            ("alt".into(), SemanticValue::Null),
            ("body".into(), content_to_semantic(&equation.body)),
        ]),
        Content::MathIdent(text) | Content::MathText(text) => {
            fields.push(("text".into(), SemanticValue::String(text.to_string())))
        }
        Content::MathAttach(attach) => {
            fields.push(("base".into(), content_to_semantic(&attach.base)));
            // The crystalline layout model separates centred limits (`t`/`b`)
            // from right scripts (`tr`/`br`). The public content surface uses
            // `t`/`b` for both; normalize that mechanical distinction here.
            fn first_supplied<'a>(
                primary: &'a MathAttachSlot,
                fallback: &'a MathAttachSlot,
            ) -> &'a MathAttachSlot {
                match primary {
                    MathAttachSlot::Omitted => fallback,
                    supplied => supplied,
                }
            }
            for (name, slot) in [
                ("t", first_supplied(&attach.t, &attach.tr)),
                ("b", first_supplied(&attach.b, &attach.br)),
                ("tl", &attach.tl),
                ("bl", &attach.bl),
            ] {
                match slot {
                    MathAttachSlot::Omitted => {}
                    MathAttachSlot::ExplicitNone => {
                        fields.push((name.into(), SemanticValue::Null));
                    }
                    MathAttachSlot::Present(child) => {
                        fields.push((name.into(), content_to_semantic(child)));
                    }
                }
            }
        }
        Content::Shape(shape) => {
            fields.extend([
                (
                    "width".into(),
                    shape
                        .width
                        .as_deref()
                        .map(shape_dimension_to_semantic)
                        .unwrap_or_else(|| SemanticValue::String("auto".into())),
                ),
                (
                    "height".into(),
                    shape
                        .height
                        .as_deref()
                        .map(shape_dimension_to_semantic)
                        .unwrap_or_else(|| SemanticValue::String("auto".into())),
                ),
                (
                    "fill".into(),
                    shape
                        .fill
                        .as_ref()
                        .map(paint_to_semantic)
                        .unwrap_or(SemanticValue::Null),
                ),
            ]);
            if let Some(stroke) = &shape.stroke {
                fields.push((
                    "stroke".into(),
                    value_to_semantic(&Value::from(stroke.clone())),
                ));
            }
        }
        Content::Metadata(metadata) => {
            fields.push(("value".into(), value_to_semantic(metadata.value.as_ref())))
        }
        Content::SmartQuote(e) => {
            for (name, present) in [
                ("double", e.double.is_some()),
                ("enabled", e.enabled.is_some()),
                ("alternative", e.alternative.is_some()),
                ("quotes", e.quotes.is_some()),
            ] {
                if present {
                    fields.push((
                        name.into(),
                        value_to_semantic(
                            &content.get_field(name).expect("known quote field"),
                        ),
                    ));
                }
            }
        }
        Content::Quote(quote) => {
            fields.push(("block".into(), SemanticValue::Bool(quote.block)));
            fields.push(("quotes".into(), SemanticValue::Bool(quote.quotes)));
            fields.push(("body".into(), content_to_semantic(&quote.body)));
            fields.push((
                "attribution".into(),
                quote
                    .attribution
                    .as_ref()
                    .map(content_to_semantic)
                    .unwrap_or(SemanticValue::Null),
            ));
        }
        Content::Styled(styled) => {
            fields.push(("child".into(), content_to_semantic(&styled.body)));
            fields.push(("styles".into(), SemanticValue::String("styles(..)".into())))
        }
        _ => {
            if let Some(body) = content.get_field("body") {
                fields.push(("body".into(), value_to_semantic(&body)));
            }
        }
    }
    SemanticValue::Object(fields)
}

fn content_semantic_func_name(
    content: &typst_core::entities::content::Content,
) -> &'static str {
    use typst_core::entities::content::Content;
    use typst_core::entities::geometry::ShapeKind;
    match content {
        Content::Source(source) => content_semantic_func_name(&source.body),
        Content::ShowRecipe(recipe) => content_semantic_func_name(&recipe.body),
        Content::Located(located) => content_semantic_func_name(&located.body),
        Content::Realized(realized) => content_semantic_func_name(&realized.visual),
        Content::Empty => "sequence",
        Content::Shape(shape) => match shape.kind {
            ShapeKind::Rect | ShapeKind::RoundedRect { .. } => "rect",
            ShapeKind::Ellipse => "ellipse",
            ShapeKind::Line { .. } => "line",
            ShapeKind::Path(_) => "polygon",
        },
        // Crystalline keeps math graphemes in a dedicated variant, while
        // their public content shape is still text-like.
        Content::FigureCaption(_) => "caption",
        Content::SmartQuote(_) => "smartquote",
        Content::MathAttach(_) => "attach",
        Content::MathText(_) => "text",
        Content::MathIdent(_) => "symbol",
        _ => content.elem_name(),
    }
}

fn figure_caption_to_semantic(
    body: &typst_core::entities::content::Content,
    kind: &str,
    supplement: &typst_core::entities::content::Content,
    counter: &str,
) -> SemanticValue {
    SemanticValue::Object(vec![
        ("func".into(), SemanticValue::String("caption".into())),
        (
            "separator".into(),
            content_to_semantic(&typst_core::entities::content::Content::text(": ")),
        ),
        ("body".into(), content_to_semantic(body)),
        ("kind".into(), SemanticValue::String(kind.into())),
        ("supplement".into(), content_to_semantic(supplement)),
        ("numbering".into(), SemanticValue::String("1".into())),
        ("counter".into(), SemanticValue::String(counter.into())),
    ])
}

fn figure_supplement(kind: &str) -> &'static str {
    match kind {
        "table" => "Table",
        "raw" => "Listing",
        _ => "Figure",
    }
}

fn paint_to_semantic(paint: &typst_core::entities::paint::Paint) -> SemanticValue {
    use std::sync::Arc;
    use typst_core::entities::paint::Paint;
    match paint {
        Paint::Solid(color) => value_to_semantic(&Value::Color(color.clone())),
        Paint::Gradient(gradient) => {
            value_to_semantic(&Value::Gradient(gradient.clone()))
        }
        Paint::Tiling(tiling) => {
            value_to_semantic(&Value::Tiling(Arc::new(tiling.clone())))
        }
    }
}

fn shape_dimension_to_semantic(value: &Value) -> SemanticValue {
    use typst_core::entities::rel::Rel;

    match value {
        Value::Length(length) => {
            value_to_semantic(&Value::Relative(Rel { rel: 0.0, abs: *length }))
        }
        _ => value_to_semantic(value),
    }
}

fn value_to_semantic(value: &Value) -> SemanticValue {
    match value {
        Value::None => SemanticValue::Null,
        Value::Bool(v) => SemanticValue::Bool(*v),
        Value::Int(v) => SemanticValue::Int(*v),
        Value::Float(v) => SemanticValue::Float(*v),
        Value::Str(v) => SemanticValue::String(v.to_string()),
        Value::Bytes(v) => {
            SemanticValue::String(format!("bytes({})", v.as_slice().len()))
        }
        Value::Symbol(v) => SemanticValue::String(v.value.to_string()),
        Value::Content(content) => content_to_semantic(content),
        Value::LocatedContent(content, _) => match content.fields() {
            Some(fields) => {
                let mut entries = vec![(
                    "func".to_string(),
                    SemanticValue::String(
                        content_semantic_func_name(content.content()).to_string(),
                    ),
                )];
                entries.extend(
                    fields
                        .iter()
                        .map(|(key, value)| (key.to_string(), value_to_semantic(value))),
                );
                SemanticValue::Object(entries)
            }
            None => content_to_semantic(content.content()),
        },
        Value::Array(values) => {
            SemanticValue::Array(values.iter().map(value_to_semantic).collect())
        }
        Value::Dict(values) => SemanticValue::Object(
            values
                .iter()
                .map(|(key, value)| (key.to_string(), value_to_semantic(value)))
                .collect(),
        ),
        other => SemanticValue::String(
            typst_core::compiler::eval::repr_value_for_serialization(other),
        ),
    }
}

fn eval_type_name(value: &Value) -> &'static str {
    value.type_name()
}

fn serialize_semantic(
    value: &SemanticValue,
    json: bool,
    pretty: bool,
) -> Result<Vec<u8>, String> {
    if json {
        let json = semantic_to_json(value);
        let mut bytes = if pretty {
            serde_json::to_vec_pretty(&json)
        } else {
            serde_json::to_vec(&json)
        }
        .map_err(|e| format!("failed to serialize result: {e}"))?;
        bytes.push(b'\n');
        Ok(bytes)
    } else {
        let mut yaml = String::new();
        write_yaml(value, 0, &mut yaml);
        if !yaml.ends_with('\n') {
            yaml.push('\n');
        }
        Ok(yaml.into_bytes())
    }
}

fn semantic_to_json(value: &SemanticValue) -> serde_json::Value {
    match value {
        SemanticValue::Null => serde_json::Value::Null,
        SemanticValue::Bool(v) => (*v).into(),
        SemanticValue::Int(v) => (*v).into(),
        SemanticValue::Float(v) => serde_json::Number::from_f64(*v)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        SemanticValue::String(v) => v.as_str().into(),
        SemanticValue::Array(values) => {
            serde_json::Value::Array(values.iter().map(semantic_to_json).collect())
        }
        SemanticValue::Object(fields) => serde_json::Value::Object(
            fields
                .iter()
                .map(|(key, value)| (key.clone(), semantic_to_json(value)))
                .collect(),
        ),
    }
}

fn write_yaml(value: &SemanticValue, indent: usize, out: &mut String) {
    match value {
        SemanticValue::Array(values) if values.is_empty() => out.push_str("[]"),
        SemanticValue::Object(fields) if fields.is_empty() => out.push_str("{}"),
        SemanticValue::Array(values) => {
            for value in values {
                out.push_str(&" ".repeat(indent));
                out.push('-');
                if yaml_is_scalar(value) {
                    out.push(' ');
                    write_yaml_scalar(value, out);
                    out.push('\n');
                } else {
                    out.push('\n');
                    write_yaml(value, indent + 2, out);
                }
            }
        }
        SemanticValue::Object(fields) => {
            for (key, value) in fields {
                out.push_str(&" ".repeat(indent));
                write_yaml_string(key, out);
                out.push(':');
                if yaml_is_scalar(value) {
                    out.push(' ');
                    write_yaml_scalar(value, out);
                    out.push('\n');
                } else {
                    out.push('\n');
                    write_yaml(value, indent + 2, out);
                }
            }
        }
        scalar => write_yaml_scalar(scalar, out),
    }
}

fn yaml_is_scalar(value: &SemanticValue) -> bool {
    !matches!(value, SemanticValue::Array(values) if !values.is_empty())
        && !matches!(value, SemanticValue::Object(fields) if !fields.is_empty())
}

fn write_yaml_scalar(value: &SemanticValue, out: &mut String) {
    match value {
        SemanticValue::Null => out.push_str("null"),
        SemanticValue::Bool(v) => out.push_str(if *v { "true" } else { "false" }),
        SemanticValue::Int(v) => out.push_str(&v.to_string()),
        SemanticValue::Float(v) if v.is_nan() => out.push_str(".nan"),
        SemanticValue::Float(v) if *v == f64::INFINITY => out.push_str(".inf"),
        SemanticValue::Float(v) if *v == f64::NEG_INFINITY => out.push_str("-.inf"),
        SemanticValue::Float(v) => out.push_str(&v.to_string()),
        SemanticValue::String(v) => write_yaml_string(v, out),
        SemanticValue::Array(_) => out.push_str("[]"),
        SemanticValue::Object(_) => out.push_str("{}"),
    }
}

fn write_yaml_string(value: &str, out: &mut String) {
    let reserved = matches!(
        value,
        "null"
            | "Null"
            | "NULL"
            | "true"
            | "True"
            | "TRUE"
            | "false"
            | "False"
            | "FALSE"
            | "~"
    );
    let plain = !value.is_empty()
        && !reserved
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-'))
        && !value.starts_with(|c: char| c.is_ascii_digit() || c == '-');
    if plain {
        out.push_str(value);
    } else {
        out.push_str(&serde_json::to_string(value).expect("string JSON infallible"));
    }
}

/// **P694** — valida e divide uma entrada `--input` no primeiro `=`. Aceita
/// `=` no valor (`k=a=b` → `("k", "a=b")`). Rejeita ausência de `=` e chave
/// vazia (`"=v"`). O valor pode ser vazio (`"k="` → `("k", "")`).
fn parse_input_entry(s: &str) -> Option<(String, String)> {
    let eq = s.find('=')?;
    if eq == 0 {
        return None;
    }
    let key = s[..eq].to_string();
    let value = s[eq + 1..].to_string();
    Some((key, value))
}

/// **P617** — converte uma string UUID textual nos 16 bytes correspondentes.
/// Aceita o formato canónico `8-4-4-4-12` ou 32 hex sem hífenes.
/// Devolve `None` se o formato ou os dígitos hex forem inválidos.
fn parse_uuid_bytes(s: &str) -> Option<[u8; 16]> {
    let hex: String = s.chars().filter(|&c| c != '-').collect();
    if hex.len() != 32 {
        return None;
    }
    let mut bytes = [0u8; 16];
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).ok()?;
    }
    Some(bytes)
}

/// Decisão pura de resolução do path de output (Passo 120, ADR-0051).
///
/// Ordem de precedência:
/// 1. `output_flag` (passada via `-o/--output`) vence.
/// 2. `output` positional (se presente).
/// 3. Default derivado: `input.with_extension("pdf")`.
pub fn resolve_output_with(
    input: &std::path::Path,
    output: Option<&PathBuf>,
    output_flag: Option<&PathBuf>,
) -> PathBuf {
    output_flag
        .cloned()
        .or_else(|| output.cloned())
        .unwrap_or_else(|| input.with_extension("pdf"))
}

/// Decisão pura de resolução do formato de saída (P866).
///
/// Ordem de precedência (alinhada com vanilla `CompileConfig::new_impl`):
/// 1. `format_flag` (via `--format`) vence se presente.
/// 2. Extensão do path de `output` (flag `-o` ou positional) se for
///    `pdf`, `png`, `svg` ou `html` (case-insensitive).
/// 3. `default` (tipicamente `OutputFormat::Pdf`).
///
/// Função pura — não valida se o formato é suportado pelo backend;
/// essa decisão fica para L4.
pub fn resolve_output_format_with(
    format_flag: Option<&OutputFormat>,
    output: Option<&PathBuf>,
    default: OutputFormat,
) -> OutputFormat {
    if let Some(format) = format_flag {
        return *format;
    }
    output
        .and_then(|p| p.extension())
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .and_then(|ext| match ext.as_str() {
            "pdf" => Some(OutputFormat::Pdf),
            "png" => Some(OutputFormat::Png),
            "svg" => Some(OutputFormat::Svg),
            "html" => Some(OutputFormat::Html),
            _ => None,
        })
        .unwrap_or(default)
}

/// Decisão pura de resolução do root directory (Passo 121, ADR-0051).
///
/// Ordem de precedência (alinhada com vanilla typst-cli):
/// 1. `--root` flag explícita vence.
/// 2. `input.parent()` se não vazio.
/// 3. Default `"."` (cwd).
///
/// Função pura — não verifica se o path existe (I/O é L3/L4).
pub fn resolve_root_with(root: Option<&PathBuf>, input: &Path) -> PathBuf {
    root.cloned()
        .or_else(|| {
            input
                .parent()
                .map(Path::to_path_buf)
                .filter(|p| !p.as_os_str().is_empty())
        })
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Wrapper em torno de `resolve_colored_with` que lê env e tty.
fn resolve_colored(choice: &ColorWhen) -> bool {
    resolve_colored_with(
        choice,
        std::env::var_os("NO_COLOR").is_some(),
        std::io::stderr().is_terminal(),
    )
}

/// Decisão pura de coloração — testável sem env mutation (ADR-0048).
///
/// Ordem de precedência:
/// 1. Flag explícita (`Always` / `Never`) vence tudo.
/// 2. Em `Auto`, `NO_COLOR` desactiva.
/// 3. Em `Auto` sem `NO_COLOR`, decide `is_tty`.
pub fn resolve_colored_with(
    choice: &ColorWhen,
    no_color_present: bool,
    is_tty: bool,
) -> bool {
    match choice {
        ColorWhen::Never => false,
        ColorWhen::Always => true,
        ColorWhen::Auto => !no_color_present && is_tty,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn os_args(items: &[&str]) -> Vec<OsString> {
        items.iter().map(OsString::from).collect()
    }

    #[test]
    fn p1293_c_html_serialization_compile_only_e_default_crystalline() {
        let omitted = Args::try_parse_from(["typst", "compile", "in.typ"])
            .expect("default deve parsear");
        let explicit = Args::try_parse_from([
            "typst",
            "compile",
            "in.typ",
            "--html-serialization",
            "vanilla",
        ])
        .expect("vanilla deve parsear");

        let Command::Compile(omitted) = omitted.command else { panic!() };
        let Command::Compile(explicit) = explicit.command else { panic!() };
        assert_eq!(omitted.html_serialization, HtmlSerialization::Crystalline);
        assert_eq!(explicit.html_serialization, HtmlSerialization::Vanilla);
        assert!(Args::try_parse_from([
            "typst",
            "compile",
            "in.typ",
            "--html-serialization",
            "other",
        ])
        .is_err());
        assert!(Args::try_parse_from([
            "typst",
            "watch",
            "in.typ",
            "--html-serialization",
            "vanilla",
        ])
        .is_err());
        assert!(Args::try_parse_from([
            "typst",
            "eval",
            "1",
            "--html-serialization",
            "vanilla",
        ])
        .is_err());
    }

    #[test]
    fn p1137_legacy_recebe_compile_implicito() {
        assert_eq!(
            normalize_legacy_args(os_args(&["typst", "main.typ", "out.pdf"])),
            os_args(&["typst", "compile", "main.typ", "out.pdf"]),
        );
    }

    #[test]
    fn p1137_subcomando_e_meta_nao_sao_reescritos() {
        for args in [
            os_args(&["typst", "compile", "main.typ"]),
            os_args(&["typst", "c", "main.typ"]),
            os_args(&["typst", "eval", "1 + 1"]),
            os_args(&["typst", "--help"]),
        ] {
            assert_eq!(normalize_legacy_args(args.clone()), args);
        }
    }

    #[test]
    fn p1137_cert_flag_vence_env_e_env_e_fallback() {
        assert_eq!(
            resolve_cert_path(
                Some(PathBuf::from("flag.pem")),
                Some(OsString::from("env.pem")),
            ),
            Some(PathBuf::from("flag.pem")),
        );
        assert_eq!(
            resolve_cert_path(None, Some(OsString::from("env.pem"))),
            Some(PathBuf::from("env.pem")),
        );
    }

    #[test]
    fn resolve_colored_never_e_false() {
        assert!(!resolve_colored_with(&ColorWhen::Never, false, false));
        assert!(!resolve_colored_with(&ColorWhen::Never, true, false));
        assert!(!resolve_colored_with(&ColorWhen::Never, false, true));
        assert!(!resolve_colored_with(&ColorWhen::Never, true, true));
    }

    #[test]
    fn resolve_colored_always_e_true() {
        assert!(resolve_colored_with(&ColorWhen::Always, false, false));
        assert!(resolve_colored_with(&ColorWhen::Always, true, false));
        assert!(resolve_colored_with(&ColorWhen::Always, false, true));
        assert!(resolve_colored_with(&ColorWhen::Always, true, true));
    }

    #[test]
    fn resolve_colored_auto_sem_tty_e_false() {
        assert!(!resolve_colored_with(&ColorWhen::Auto, false, false));
    }

    #[test]
    fn resolve_colored_auto_com_tty_e_sem_no_color_e_true() {
        assert!(resolve_colored_with(&ColorWhen::Auto, false, true));
    }

    #[test]
    fn resolve_colored_auto_com_no_color_e_false() {
        assert!(!resolve_colored_with(&ColorWhen::Auto, true, true));
        assert!(!resolve_colored_with(&ColorWhen::Auto, true, false));
    }

    #[test]
    fn resolve_colored_always_vence_no_color() {
        assert!(resolve_colored_with(&ColorWhen::Always, true, false));
        assert!(resolve_colored_with(&ColorWhen::Always, true, true));
    }

    // ── resolve_output_with (pura — Passo 120, ADR-0051) ───────────────

    #[test]
    fn resolve_output_flag_vence_positional() {
        let input = PathBuf::from("in.typ");
        let positional = PathBuf::from("pos.pdf");
        let flag = PathBuf::from("flag.pdf");
        let out = resolve_output_with(&input, Some(&positional), Some(&flag));
        assert_eq!(out, PathBuf::from("flag.pdf"));
    }

    #[test]
    fn resolve_output_positional_usa_quando_sem_flag() {
        let input = PathBuf::from("in.typ");
        let positional = PathBuf::from("pos.pdf");
        let out = resolve_output_with(&input, Some(&positional), None);
        assert_eq!(out, PathBuf::from("pos.pdf"));
    }

    #[test]
    fn resolve_output_flag_usa_sem_positional() {
        let input = PathBuf::from("in.typ");
        let flag = PathBuf::from("flag.pdf");
        let out = resolve_output_with(&input, None, Some(&flag));
        assert_eq!(out, PathBuf::from("flag.pdf"));
    }

    #[test]
    fn resolve_output_ambos_omitidos_usa_default_derivado() {
        let input = PathBuf::from("in.typ");
        let out = resolve_output_with(&input, None, None);
        assert_eq!(out, PathBuf::from("in.pdf"));
    }

    #[test]
    fn resolve_output_default_com_path_completo() {
        // Path com directório preservado no default derivado.
        let input = PathBuf::from("/tmp/sub/file.typ");
        let out = resolve_output_with(&input, None, None);
        assert_eq!(out, PathBuf::from("/tmp/sub/file.pdf"));
    }

    #[test]
    fn resolve_output_default_sem_extensao_adiciona_pdf() {
        // Se input não tiver extensão, with_extension adiciona.
        let input = PathBuf::from("noext");
        let out = resolve_output_with(&input, None, None);
        assert_eq!(out, PathBuf::from("noext.pdf"));
    }

    // ── resolve_output_format_with (pura — P866) ───────────────────────

    #[test]
    fn resolve_output_format_flag_vence_extensao() {
        let flag = OutputFormat::Svg;
        let output = PathBuf::from("out.png");
        let fmt =
            resolve_output_format_with(Some(&flag), Some(&output), OutputFormat::Pdf);
        assert_eq!(fmt, OutputFormat::Svg);
    }

    #[test]
    fn resolve_output_format_detecta_png() {
        let output = PathBuf::from("simple.png");
        let fmt = resolve_output_format_with(None, Some(&output), OutputFormat::Pdf);
        assert_eq!(fmt, OutputFormat::Png);
    }

    #[test]
    fn resolve_output_format_detecta_svg() {
        let output = PathBuf::from("simple.svg");
        let fmt = resolve_output_format_with(None, Some(&output), OutputFormat::Pdf);
        assert_eq!(fmt, OutputFormat::Svg);
    }

    #[test]
    fn resolve_output_format_detecta_html() {
        let output = PathBuf::from("simple.html");
        let fmt = resolve_output_format_with(None, Some(&output), OutputFormat::Pdf);
        assert_eq!(fmt, OutputFormat::Html);
    }

    #[test]
    fn resolve_output_format_pdf_default() {
        // Sem flag e sem output, default é Pdf.
        let fmt = resolve_output_format_with(None, None, OutputFormat::Pdf);
        assert_eq!(fmt, OutputFormat::Pdf);

        // Output sem extensão reconhecida cai no default.
        let output = PathBuf::from("simple");
        let fmt = resolve_output_format_with(None, Some(&output), OutputFormat::Pdf);
        assert_eq!(fmt, OutputFormat::Pdf);
    }

    #[test]
    fn resolve_output_format_case_insensitive() {
        let output = PathBuf::from("simple.PNG");
        let fmt = resolve_output_format_with(None, Some(&output), OutputFormat::Pdf);
        assert_eq!(fmt, OutputFormat::Png);
    }

    // ── resolve_root_with (pura — Passo 121, ADR-0051) ─────────────────

    #[test]
    fn resolve_root_flag_vence_parent() {
        let input = PathBuf::from("/tmp/sub/file.typ");
        let flag = PathBuf::from("/custom/root");
        let out = resolve_root_with(Some(&flag), &input);
        assert_eq!(out, PathBuf::from("/custom/root"));
    }

    #[test]
    fn resolve_root_sem_flag_usa_parent_do_input() {
        let input = PathBuf::from("/tmp/sub/file.typ");
        let out = resolve_root_with(None, &input);
        assert_eq!(out, PathBuf::from("/tmp/sub"));
    }

    #[test]
    fn resolve_root_sem_flag_e_sem_parent_usa_dot() {
        // Input sem directório — `Path::parent()` devolve `Some("")`.
        // Filter rejeita vazio; fallback é `.`.
        let input = PathBuf::from("file.typ");
        let out = resolve_root_with(None, &input);
        assert_eq!(out, PathBuf::from("."));
    }

    // ── P617 — parsing de UUID para DocumentID ───────────────────────────

    #[test]
    fn p617_parse_uuid_canonico() {
        let bytes = parse_uuid_bytes("f81d4fae-7dec-11d0-a765-00a0c91e6bf6").unwrap();
        assert_eq!(
            bytes,
            [
                0xf8, 0x1d, 0x4f, 0xae, 0x7d, 0xec, 0x11, 0xd0, 0xa7, 0x65, 0x00, 0xa0,
                0xc9, 0x1e, 0x6b, 0xf6,
            ]
        );
    }

    #[test]
    fn p617_parse_uuid_sem_hifens() {
        let bytes = parse_uuid_bytes("f81d4fae7dec11d0a76500a0c91e6bf6").unwrap();
        assert_eq!(
            bytes,
            [
                0xf8, 0x1d, 0x4f, 0xae, 0x7d, 0xec, 0x11, 0xd0, 0xa7, 0x65, 0x00, 0xa0,
                0xc9, 0x1e, 0x6b, 0xf6,
            ]
        );
    }

    #[test]
    fn p617_parse_uuid_maiusculas() {
        let bytes = parse_uuid_bytes("F81D4FAE-7DEC-11D0-A765-00A0C91E6BF6").unwrap();
        assert_eq!(
            bytes,
            [
                0xf8, 0x1d, 0x4f, 0xae, 0x7d, 0xec, 0x11, 0xd0, 0xa7, 0x65, 0x00, 0xa0,
                0xc9, 0x1e, 0x6b, 0xf6,
            ]
        );
    }

    #[test]
    fn p617_parse_uuid_invalido_devolve_none() {
        assert!(parse_uuid_bytes("not-a-uuid").is_none());
        assert!(parse_uuid_bytes("f81d4fae-7dec-11d0-a765").is_none());
        assert!(parse_uuid_bytes("f81d4fae-7dec-11d0-a765-00a0c91e6bf6-EXTRA").is_none());
        assert!(parse_uuid_bytes("f81d4fae-7dec-11d0-a765-00a0c91e6bg6").is_none());
    }

    // ── P694 — parsing de `--input chave=valor` ────────────────────────────

    #[test]
    fn p694_parse_input_simples() {
        assert_eq!(
            parse_input_entry("chave=valor"),
            Some(("chave".to_string(), "valor".to_string()))
        );
    }

    #[test]
    fn p694_parse_input_valor_numerico_e_string() {
        // Paridade vanilla: `--input n=42` → valor é a string "42".
        assert_eq!(parse_input_entry("n=42"), Some(("n".to_string(), "42".to_string())));
    }

    #[test]
    fn p694_parse_input_permite_igual_no_valor() {
        // Split no primeiro `=`; o resto (incluindo `=`) é o valor.
        assert_eq!(
            parse_input_entry("k=a=b"),
            Some(("k".to_string(), "a=b".to_string()))
        );
    }

    #[test]
    fn p694_parse_input_valor_vazio_e_valido() {
        assert_eq!(parse_input_entry("k="), Some(("k".to_string(), String::new())));
    }

    #[test]
    fn p694_parse_input_rejeita_sem_igual_ou_chave_vazia() {
        assert!(parse_input_entry("semigual").is_none());
        assert!(parse_input_entry("=valor").is_none());
        assert!(parse_input_entry("").is_none());
    }

    #[test]
    fn p1137_eval_json_inteiro_tem_newline() {
        assert_eq!(
            serialize_eval(&Value::Int(6), EvalFormat::Json, false).unwrap(),
            b"6\n"
        );
    }

    #[test]
    fn p1137_eval_json_array_e_estrutural() {
        let value = Value::Array(vec![Value::Int(2), Value::Int(3)]);
        assert_eq!(serialize_eval(&value, EvalFormat::Json, false).unwrap(), b"[2,3]\n");
    }

    #[test]
    fn p1137_eval_raw_string_nao_tem_newline() {
        assert_eq!(
            serialize_eval(&Value::from("abc"), EvalFormat::Raw, false).unwrap(),
            b"abc"
        );
    }

    #[test]
    fn p1137_eval_raw_inteiro_falha() {
        let error = serialize_eval(&Value::Int(3), EvalFormat::Raw, false).unwrap_err();
        assert!(error.contains("only supports strings and bytes"));
    }

    #[test]
    fn p1163_eval_json_symbol_serializa_grapheme_integral() {
        use typst_core::entities::symbol::Symbol;

        for grapheme in ["❤️", "👩‍💻", "❣️"] {
            let value = Value::Symbol(Symbol::new(grapheme, "test"));
            let expected = format!("{}\n", serde_json::to_string(grapheme).unwrap());
            assert_eq!(
                serialize_eval(&value, EvalFormat::Json, false).unwrap(),
                expected.as_bytes()
            );
            assert_eq!(
                serialize_eval(&value, EvalFormat::Json, true).unwrap(),
                expected.as_bytes()
            );
        }
    }

    #[test]
    fn p1163_eval_raw_symbol_nomeia_tipo_publico() {
        use typst_core::entities::symbol::Symbol;

        let value = Value::Symbol(Symbol::new("❤️", "heart"));
        let error = serialize_eval(&value, EvalFormat::Raw, false).unwrap_err();
        assert_eq!(
            error,
            "cannot print symbol in raw format\nhint: `--format=raw` only supports strings and bytes"
        );
    }

    #[test]
    fn p1225_eval_json_stroke_e_nominal_e_raw_permanece_proibido() {
        use typst_core::entities::geometry::{
            DashLength, DashPattern, LineCap, LineJoin, Stroke, StrokeFields,
        };
        let value = Value::from(Stroke {
            cap: LineCap::Round,
            join: LineJoin::Bevel,
            dash: Some(DashPattern {
                array: vec![DashLength::Length(2.0), DashLength::LineWidth],
                phase: -0.5,
            }),
            miter_limit: 2.0,
            specified: StrokeFields {
                cap: true,
                join: true,
                dash: true,
                miter_limit: true,
                ..StrokeFields::default()
            },
            ..Stroke::default()
        });
        let json =
            String::from_utf8(serialize_eval(&value, EvalFormat::Json, false).unwrap())
                .unwrap();
        assert!(json.contains("cap: \\\"round\\\""));
        assert!(json.contains("join: \\\"bevel\\\""));
        assert!(json.contains("array: (2pt, \\\"dot\\\")"));
        assert!(json.contains("phase: -0.5pt"));
        assert!(json.contains("miter-limit: 2"));
        assert!(serialize_eval(&value, EvalFormat::Raw, false).is_err());
        assert_eq!(
            serialize_eval(
                &Value::Color(typst_core::entities::layout_types::Color::rgb(0, 0, 0)),
                EvalFormat::Json,
                false,
            )
            .unwrap(),
            b"\"rgb(\\\"#000000\\\")\"\n"
        );
    }

    #[test]
    fn p1137_query_heading_json_vanilla() {
        let heading = typst_core::entities::content::Content::heading(
            1,
            typst_core::entities::content::Content::text("First"),
        );
        let json = String::from_utf8(
            serialize_query(&[Value::Content(heading)], None, false, false).unwrap(),
        )
        .unwrap();
        assert!(json.starts_with("[{\"func\":\"heading\",\"level\":1"));
        assert!(json.contains("\"body\":{\"func\":\"text\",\"text\":\"First\"}"));
    }

    #[test]
    fn p1137_query_field_e_one() {
        let heading = typst_core::entities::content::Content::heading(
            1,
            typst_core::entities::content::Content::text("First"),
        );
        assert_eq!(
            serialize_query(
                &[Value::Content(heading.clone())],
                Some("level"),
                false,
                false
            )
            .unwrap(),
            b"[1]\n"
        );
        let one = serialize_query(&[Value::Content(heading)], None, true, false).unwrap();
        assert!(one.starts_with(b"{\"func\":\"heading\""));
    }

    #[test]
    fn p1460_raw_heading_delega_owner_e_preserva_transcripts() {
        use std::str::FromStr;
        use typst_core::compiler::lang::heading_supplement_for_lang;
        use typst_core::entities::content::Content;
        use typst_core::entities::lang::Lang;

        for (lang, expected) in [
            (None, "Section"),
            (Some("de"), "Abschnitt"),
            (Some("fr"), "Section"),
            (Some("pt"), "Seção"),
            (Some("af"), "Section"),
        ] {
            let parsed = lang.map(|code| Lang::from_str(code).unwrap());
            assert_eq!(heading_supplement_for_lang(parsed.as_ref()), expected);
        }

        let heading = Content::heading(1, Content::text("Probe"));
        let value = Value::Content(heading.clone());
        assert_eq!(
            serialize_eval(&value, EvalFormat::Json, false).unwrap(),
            concat!(
                "{\"func\":\"heading\",\"level\":1,\"depth\":1,\"offset\":0,",
                "\"numbering\":null,\"supplement\":{\"func\":\"text\",",
                "\"text\":\"Section\"},\"outlined\":true,\"bookmarked\":\"auto\",",
                "\"hanging-indent\":\"auto\",\"body\":{\"func\":\"text\",",
                "\"text\":\"Probe\"}}\n"
            )
            .as_bytes()
        );
        assert_eq!(
            serialize_eval(&value, EvalFormat::Json, true).unwrap(),
            concat!(
                "{\n",
                "  \"func\": \"heading\",\n",
                "  \"level\": 1,\n",
                "  \"depth\": 1,\n",
                "  \"offset\": 0,\n",
                "  \"numbering\": null,\n",
                "  \"supplement\": {\n",
                "    \"func\": \"text\",\n",
                "    \"text\": \"Section\"\n",
                "  },\n",
                "  \"outlined\": true,\n",
                "  \"bookmarked\": \"auto\",\n",
                "  \"hanging-indent\": \"auto\",\n",
                "  \"body\": {\n",
                "    \"func\": \"text\",\n",
                "    \"text\": \"Probe\"\n",
                "  }\n",
                "}\n"
            )
            .as_bytes()
        );
        let yaml = concat!(
            "func: heading\n",
            "level: 1\n",
            "depth: 1\n",
            "offset: 0\n",
            "numbering: null\n",
            "supplement:\n",
            "  func: text\n",
            "  text: Section\n",
            "outlined: true\n",
            "bookmarked: auto\n",
            "hanging-indent: auto\n",
            "body:\n",
            "  func: text\n",
            "  text: Probe\n"
        )
        .as_bytes();
        assert_eq!(serialize_eval(&value, EvalFormat::Yaml, false).unwrap(), yaml);
        assert_eq!(serialize_eval(&value, EvalFormat::Yaml, true).unwrap(), yaml);
        assert_eq!(
            serialize_eval(&value, EvalFormat::Raw, false).unwrap_err(),
            "cannot print content in raw format\nhint: `--format=raw` only supports strings and bytes"
        );
        assert_eq!(
            serialize_query(&[Value::Content(heading)], Some("supplement"), true, false)
                .unwrap(),
            b"{\"func\":\"text\",\"text\":\"Section\"}\n"
        );
    }

    #[test]
    fn p1460_snapshot_causal_continua_autoritativo() {
        use typst_core::entities::content::Content;
        use typst_core::entities::locator::Locator;
        use typst_core::entities::value::IntrospectedContent;

        let heading = Content::heading(1, Content::text("Probe"));
        let mut locator = Locator::new();
        let located = Value::LocatedContent(
            IntrospectedContent::new(
                heading,
                Some(
                    std::iter::once((
                        "supplement".into(),
                        Value::Content(Content::text("Abschnitt")),
                    ))
                    .collect(),
                ),
            ),
            locator.next(),
        );

        assert_eq!(
            serialize_query(&[located], Some("supplement"), true, false).unwrap(),
            b"{\"func\":\"text\",\"text\":\"Abschnitt\"}\n"
        );
    }

    #[test]
    fn p1140_6_no_pdf_tags_pertence_a_compile_e_watch() {
        for command in ["compile", "watch", "w"] {
            let args =
                Args::try_parse_from(["typst", command, "in.typ", "--no-pdf-tags"])
                    .expect("flag deve ser aceite");
            let compile = match args.command {
                Command::Compile(args) => args.compile,
                Command::Watch(args) => args.compile,
                _ => panic!("comando inesperado"),
            };
            assert!(compile.no_pdf_tags);
            assert!(!compile.compact, "eixos devem permanecer independentes");
        }
    }

    #[test]
    fn p1359_pdf_tagged_tristate_parseia_e_chega_ao_compile_intent() {
        let cases = [
            (vec!["typst", "compile", "in.typ", "--pdf-tagged"], Some(true)),
            (vec!["typst", "compile", "in.typ", "--pdf-tagged=true"], Some(true)),
            (vec!["typst", "compile", "in.typ", "--pdf-tagged=false"], Some(false)),
            (vec!["typst", "compile", "in.typ"], None),
            (vec!["typst", "watch", "in.typ", "--pdf-tagged"], Some(true)),
            (vec!["typst", "watch", "in.typ", "--pdf-tagged=false"], Some(false)),
            (vec!["typst", "w", "in.typ", "--pdf-tagged=true"], Some(true)),
        ];

        for (argv, expected) in cases {
            let parsed = Args::try_parse_from(argv).expect("forma P1359 deve parsear");
            let compile = match parsed.command {
                Command::Compile(args) => args.compile,
                Command::Watch(args) => args.compile,
                _ => panic!("comando compile/watch esperado"),
            };
            assert_eq!(compile.pdf_tagged, expected);
            let intent =
                compile_intent(compile, HtmlSerialization::Crystalline, false, None);
            assert_eq!(intent.pdf_tagged, expected);
        }
    }

    #[test]
    fn p1359_pdf_tagged_exige_equals_e_bool_valido() {
        let cases = [
            (
                vec!["typst", "compile", "in.typ", "out.pdf", "--pdf-tagged", "false"],
                "unexpected argument 'false'",
            ),
            (
                vec!["typst", "compile", "in.typ", "--pdf-tagged=maybe"],
                "invalid value 'maybe'",
            ),
            (
                vec!["typst", "compile", "in.typ", "out.pdf", "--pdf-tagged="],
                "a value is required",
            ),
        ];

        for (argv, expected) in cases {
            let error =
                Args::try_parse_from(argv).expect_err("forma inválida deve falhar");
            assert!(
                error.to_string().contains(expected),
                "esperava {expected:?}; erro: {error}"
            );
        }
    }

    fn p1285_metadata_value() -> Value {
        let mut value = Value::Dict(Default::default());
        let Value::Dict(fields) = &mut value else {
            unreachable!();
        };
        fields.insert("name".into(), Value::from("x"));
        fields.insert("n".into(), Value::Int(2));
        value
    }

    #[test]
    fn p1285_eval_e_query_aceitam_yaml_com_variantes_distintas() {
        let eval =
            Args::try_parse_from(["typst", "eval", "sys.version", "--format", "yaml"])
                .expect("eval --format yaml deve ser aceite");
        let Command::Eval(eval) = eval.command else {
            panic!("comando eval esperado");
        };
        assert!(matches!(eval.format, EvalFormat::Yaml));

        let query =
            Args::try_parse_from(["typst", "query", "-", "metadata", "--format", "yaml"])
                .expect("query --format yaml deve ser aceite");
        let Command::Query(query) = query.command else {
            panic!("comando query esperado");
        };
        assert!(matches!(query.format, QueryFormat::Yaml));
    }

    #[test]
    fn p1285_eval_version_e_fallbacks_preservam_repr_publica() {
        use std::sync::Arc;
        use typst_core::entities::color::Color;
        use typst_core::entities::func::Func;
        use typst_core::entities::gradient::{Gradient, GradientStop};
        use typst_core::entities::layout_types::{Angle, Length, Pt, Size};
        use typst_core::entities::tiling::{Tiling, TilingBody};
        use typst_core::entities::version::Version;

        let version = Value::from(Version::new(0, 15, 1));
        assert_eq!(
            serialize_eval(&version, EvalFormat::Json, false).unwrap(),
            b"\"version(0, 15, 1)\"\n"
        );

        let fallbacks =
            Value::Array(vec![Value::Auto, Value::Length(Length::pt(12.0)), version]);
        assert_eq!(
            serialize_eval(&fallbacks, EvalFormat::Json, false).unwrap(),
            b"[\"auto\",\"12pt\",\"version(0, 15, 1)\"]\n"
        );

        let gcd = Value::Func(Func::native("calc.gcd", |_ctx, _args, _world, _cf| {
            Ok(Value::None)
        }));
        assert_eq!(serialize_eval(&gcd, EvalFormat::Json, false).unwrap(), b"\"gcd\"\n");

        let gradient = Value::Gradient(Gradient::linear(
            vec![
                GradientStop::unspaced(Color::rgb(255, 65, 54)),
                GradientStop::unspaced(Color::rgb(0, 116, 217)),
            ],
            Angle::deg(180.0),
        ));
        assert_eq!(
            serialize_eval(&gradient, EvalFormat::Json, false).unwrap(),
            b"\"gradient.linear((oklab(65.95%, 0.2, 0.108), 0%), (oklab(56.22%, -0.05, -0.17), 100%))\"\n"
        );

        let mut tiling = Tiling::new(TilingBody::Color(Color::rgb(255, 0, 0)));
        tiling.size = Some(Size { width: Pt(10.0), height: Pt(10.0) });
        assert_eq!(
            serialize_eval(&Value::Tiling(Arc::new(tiling)), EvalFormat::Json, false,)
                .unwrap(),
            b"\"tiling((10pt, 10pt), ..)\"\n"
        );
    }

    #[test]
    fn p1285_eval_content_sequence_e_strong_sao_estruturais() {
        use typst_core::entities::content::Content;

        let content = Content::sequence(vec![
            Content::strong(Content::text("Hi")),
            Content::Space,
            Content::text("there"),
        ]);
        assert_eq!(
            serialize_eval(&Value::Content(content), EvalFormat::Json, false).unwrap(),
            br#"{"func":"sequence","children":[{"func":"strong","body":{"func":"text","text":"Hi"}},{"func":"space"},{"func":"text","text":"there"}]}"#
                .iter()
                .copied()
                .chain(std::iter::once(b'\n'))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn p1285_query_metadata_figure_e_label_preservam_morfologia() {
        use typst_core::entities::color::Color;
        use typst_core::entities::content::Content;
        use typst_core::entities::geometry::ShapeKind;
        use typst_core::entities::layout_types::Length;
        use typst_core::entities::paint::Paint;

        let metadata = Content::metadata(p1285_metadata_value());
        let metadata_json =
            serialize_query(&[Value::Content(metadata)], None, false, false).unwrap();
        let metadata_tree: serde_json::Value =
            serde_json::from_slice(&metadata_json).unwrap();
        assert_eq!(metadata_tree[0]["func"], "metadata");
        assert_eq!(metadata_tree[0]["value"], serde_json::json!({"name": "x", "n": 2}));

        let figure = Content::label(
            "fig-one",
            Content::figure(Content::text("one"), None, None, None),
        );
        let figure_json =
            serialize_query(&[Value::Content(figure)], None, false, false).unwrap();
        let figure_tree: serde_json::Value =
            serde_json::from_slice(&figure_json).unwrap();
        assert_eq!(figure_tree.as_array().unwrap().len(), 1);
        assert_eq!(figure_tree[0]["func"], "figure");
        assert_eq!(figure_tree[0]["label"], "<fig-one>");
        assert_eq!(figure_tree[0]["body"]["func"], "text");
        assert_ne!(figure_tree[0]["func"], "label");

        let rect = Content::shape(
            ShapeKind::Rect,
            Some(Box::new(Value::Length(Length::pt(10.0)))),
            Some(Box::new(Value::Length(Length::pt(20.0)))),
            Some(Paint::solid(Color::rgb(255, 0, 0))),
            None,
        );
        let figure = Content::label(
            "fig",
            Content::figure(rect, Some(Content::text("Cap")), Some("image".into()), None),
        );
        let equation = Content::equation(
            Content::math_attach(
                Content::MathIdent("x".into()),
                MathAttachSlot::Present(Content::MathText("2".into())),
                MathAttachSlot::Omitted,
                MathAttachSlot::Omitted,
                MathAttachSlot::Omitted,
                MathAttachSlot::Omitted,
                MathAttachSlot::Omitted,
            ),
            false,
        );
        let tree: serde_json::Value = serde_json::from_slice(
            &serialize_query(
                &[Value::Content(figure), Value::Content(equation)],
                None,
                false,
                false,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            tree,
            serde_json::json!([
                {
                    "func": "figure",
                    "body": {
                        "func": "rect",
                        "width": "0% + 10pt",
                        "height": "0% + 20pt",
                        "fill": "rgb(\"#ff0000\")"
                    },
                    "alt": null,
                    "placement": null,
                    "scope": "column",
                    "caption": {
                        "func": "caption",
                        "separator": {"func": "text", "text": ": "},
                        "body": {"func": "text", "text": "Cap"},
                        "kind": "image",
                        "supplement": {"func": "text", "text": "Figure"},
                        "numbering": "1",
                        "counter": "counter(figure.where(kind: image))"
                    },
                    "kind": "image",
                    "supplement": {"func": "text", "text": "Figure"},
                    "numbering": "1",
                    "gap": "0.65em",
                    "outlined": true,
                    "counter": "counter(figure.where(kind: image))",
                    "label": "<fig>"
                },
                {
                    "func": "equation",
                    "block": false,
                    "numbering": null,
                    "number-align": "end + horizon",
                    "supplement": {"func": "text", "text": "Equation"},
                    "alt": null,
                    "body": {
                        "func": "attach",
                        "base": {"func": "symbol", "text": "x"},
                        "t": {"func": "text", "text": "2"}
                    }
                }
            ])
        );
    }

    #[test]
    fn p1285_query_field_e_one_seguem_ordem_e_fallbacks() {
        use typst_core::entities::content::Content;

        let metadata = Content::metadata(p1285_metadata_value());
        assert_eq!(
            serialize_query(&[Value::Content(metadata)], Some("value"), true, false)
                .unwrap(),
            b"{\"name\":\"x\",\"n\":2}\n"
        );

        let figure = Content::figure(Content::text("one"), None, None, None);
        assert_eq!(
            serialize_query(
                &[Value::Content(figure.clone())],
                Some("does-not-exist"),
                false,
                false
            )
            .unwrap(),
            b"[]\n"
        );
        assert_eq!(
            serialize_query(
                &[Value::Content(figure)],
                Some("does-not-exist"),
                true,
                false
            )
            .unwrap_err(),
            "no such field found for element"
        );
        assert_eq!(
            serialize_query(&[], None, true, false).unwrap_err(),
            "expected exactly one element, found 0"
        );
    }

    #[test]
    fn p1401_eval_in_transporta_path_e_hifen() {
        for input in ["doc.typ", "-"] {
            let parsed = Args::try_parse_from([
                "typst",
                "eval",
                "query(heading).len()",
                "--in",
                input,
            ])
            .expect("eval --in deve parsear");
            let Command::Eval(eval) = parsed.command else {
                panic!("eval esperado");
            };
            assert_eq!(eval.input.as_deref(), Some(Path::new(input)));
        }
    }

    // ── P1468 (RED — Agente B) — flag `compile --pages`. `CompileArgs`
    // ainda não tem o campo `pages` neste baseline; este módulo falha por
    // erro de compilação até `PageRangeArg`/`CompileArgs.pages`/
    // `CompileIntent.pages` serem materializados (Agente A, em paralelo),
    // conforme `shell/cli.md` §P1468.
    use typst_core::entities::document_format_options::{PageRange, PageRanges};

    fn single_page_ranges(n: usize) -> PageRanges {
        let p = std::num::NonZeroUsize::new(n).unwrap();
        PageRanges::new(vec![PageRange { start: Some(p), end: Some(p) }])
    }

    fn compile_args_for(argv: &[&str]) -> CompileArgs {
        let parsed = Args::try_parse_from(argv).expect("forma P1468 deve parsear");
        match parsed.command {
            Command::Compile(args) => args.compile,
            Command::Watch(args) => args.compile,
            _ => panic!("comando compile/watch esperado"),
        }
    }

    #[test]
    fn p1468_pages_absent_is_none_in_args_and_intent() {
        let compile = compile_args_for(&["typst", "compile", "in.typ"]);
        assert_eq!(compile.pages, None);
        let intent = compile_intent(compile, HtmlSerialization::Crystalline, false, None);
        assert_eq!(intent.pages, None);
    }

    #[test]
    fn p1468_pages_single_token_parses_single_page_range() {
        let compile = compile_args_for(&["typst", "compile", "in.typ", "--pages=3"]);
        let intent = compile_intent(compile, HtmlSerialization::Crystalline, false, None);
        assert_eq!(intent.pages, Some(single_page_ranges(3)));
    }

    #[test]
    fn p1468_pages_closed_range_parses_start_and_end() {
        let compile = compile_args_for(&["typst", "compile", "in.typ", "--pages=2-5"]);
        let intent = compile_intent(compile, HtmlSerialization::Crystalline, false, None);
        assert_eq!(
            intent.pages,
            Some(PageRanges::new(vec![PageRange {
                start: Some(std::num::NonZeroUsize::new(2).unwrap()),
                end: Some(std::num::NonZeroUsize::new(5).unwrap()),
            }]))
        );
    }

    #[test]
    fn p1468_pages_open_at_end_parses_start_only() {
        let compile = compile_args_for(&["typst", "compile", "in.typ", "--pages=3-"]);
        let intent = compile_intent(compile, HtmlSerialization::Crystalline, false, None);
        assert_eq!(
            intent.pages,
            Some(PageRanges::new(vec![PageRange {
                start: Some(std::num::NonZeroUsize::new(3).unwrap()),
                end: None,
            }]))
        );
    }

    #[test]
    fn p1468_pages_open_at_start_parses_end_only() {
        let compile = compile_args_for(&["typst", "compile", "in.typ", "--pages=-2"]);
        let intent = compile_intent(compile, HtmlSerialization::Crystalline, false, None);
        assert_eq!(
            intent.pages,
            Some(PageRanges::new(vec![PageRange {
                start: None,
                end: Some(std::num::NonZeroUsize::new(2).unwrap()),
            }]))
        );
    }

    #[test]
    fn p1468_pages_multiple_tokens_with_space_are_trimmed() {
        let compile = compile_args_for(&["typst", "compile", "in.typ", "--pages=1, 2"]);
        let intent = compile_intent(compile, HtmlSerialization::Crystalline, false, None);
        assert_eq!(
            intent.pages,
            Some(PageRanges::new(vec![
                PageRange {
                    start: Some(std::num::NonZeroUsize::new(1).unwrap()),
                    end: Some(std::num::NonZeroUsize::new(1).unwrap()),
                },
                PageRange {
                    start: Some(std::num::NonZeroUsize::new(2).unwrap()),
                    end: Some(std::num::NonZeroUsize::new(2).unwrap()),
                },
            ]))
        );
    }

    #[test]
    fn p1468_pages_compile_watch_and_alias_all_accept_the_flag() {
        for command in ["compile", "watch", "w"] {
            let compile = compile_args_for(&["typst", command, "in.typ", "--pages=2"]);
            assert!(compile.pages.is_some(), "{command}");
            let intent =
                compile_intent(compile, HtmlSerialization::Crystalline, false, None);
            assert_eq!(intent.pages, Some(single_page_ranges(2)), "{command}");
        }
    }

    #[test]
    fn p1468_pages_seven_schema_negatives_use_cli_specific_messages() {
        let cases = [
            ("", "page range must not be empty"),
            ("1--3", "page range must have a single hyphen"),
            ("-", "page range must have start or end"),
            ("abc", "not a valid page number"),
            ("0", "page numbers start at one"),
            // Mensagem CLI-only, DIFERENTE da versão documental
            // ("start of page range cannot be larger than end") —
            // `shell/cli.md` P1468 confirma que são dois pontos de
            // validação distintos, não o mesmo texto reaproveitado.
            ("3-1", "page range must end at a page after the start"),
        ];

        for (token, expected) in cases {
            let argv = ["typst", "compile", "in.typ", &format!("--pages={token}")];
            let error = Args::try_parse_from(argv)
                .expect_err(&format!("token {token:?} deveria falhar"));
            assert!(
                error.to_string().contains(expected),
                "token {token:?}: esperava {expected:?}; erro: {error}"
            );
        }
    }

    // ── P1469 (RED — Agente B) — flag `compile --pdf-standard` pública
    // (contrato de linguagem, escopo reduzido — ver `shell/cli.md` P1469,
    // e a retificação da secção `## P1286` no mesmo L0). `CompileArgs`
    // ainda tem só `_pdf_standard: Option<String>` (oculto, descartado)
    // neste baseline (`65e68ad45dcbfd8db00aeda78d0d2fbaaadefb27`); este
    // módulo falha por erro de compilação até `PdfStandardArg`/
    // `CompileArgs.pdf_standard`/`CompileIntent.pdf_standard` serem
    // materializados (Agente A, em paralelo).
    use typst_core::entities::document_format_options::PdfStandard;

    #[test]
    fn p1469_pdf_standard_absent_is_empty_in_args_and_intent() {
        let compile = compile_args_for(&["typst", "compile", "in.typ"]);
        assert!(compile.pdf_standard.is_empty());
        let intent = compile_intent(compile, HtmlSerialization::Crystalline, false, None);
        assert!(intent.pdf_standard.is_empty());
    }

    #[test]
    fn p1469_pdf_standard_single_name_parses_to_one_element() {
        let compile =
            compile_args_for(&["typst", "compile", "in.typ", "--pdf-standard=a-2b"]);
        assert_eq!(compile.pdf_standard.len(), 1);
        let intent = compile_intent(compile, HtmlSerialization::Crystalline, false, None);
        assert_eq!(intent.pdf_standard, vec![PdfStandard::A2b]);
    }

    #[test]
    fn p1469_pdf_standard_multiple_names_comma_separated_parses_in_order() {
        let compile =
            compile_args_for(&["typst", "compile", "in.typ", "--pdf-standard=a-2b,ua-1"]);
        let intent = compile_intent(compile, HtmlSerialization::Crystalline, false, None);
        assert_eq!(intent.pdf_standard, vec![PdfStandard::A2b, PdfStandard::Ua1]);
    }

    #[test]
    fn p1469_pdf_standard_all_seventeen_names_are_individually_valid_clap_values() {
        for name in [
            "1.4", "1.5", "1.6", "1.7", "2.0", "a-1b", "a-1a", "a-2b", "a-2u", "a-2a",
            "a-3b", "a-3u", "a-3a", "a-4", "a-4f", "a-4e", "ua-1",
        ] {
            let argv = ["typst", "compile", "in.typ", &format!("--pdf-standard={name}")];
            Args::try_parse_from(argv)
                .unwrap_or_else(|e| panic!("nome {name:?} deveria parsear: {e}"));
        }
    }

    #[test]
    fn p1469_pdf_standard_invalid_name_uses_clap_native_message_with_possible_values() {
        let error =
            Args::try_parse_from(["typst", "compile", "in.typ", "--pdf-standard=3.0"])
                .expect_err("nome inválido deve falhar");
        let rendered = error.to_string();
        assert!(
            rendered.contains("invalid value '3.0'"),
            "erro deve nomear o valor inválido (mensagem nativa do clap): {rendered}"
        );
        assert!(
            rendered.contains("possible values"),
            "erro deve listar os valores possíveis (comportamento nativo do clap, \
             não replicado manualmente): {rendered}"
        );
    }

    #[test]
    fn p1469_pdf_standard_compile_watch_and_alias_all_accept_the_flag() {
        for command in ["compile", "watch", "w"] {
            let compile =
                compile_args_for(&["typst", command, "in.typ", "--pdf-standard=a-4"]);
            assert_eq!(compile.pdf_standard.len(), 1, "{command}");
            let intent =
                compile_intent(compile, HtmlSerialization::Crystalline, false, None);
            assert_eq!(intent.pdf_standard, vec![PdfStandard::A4], "{command}");
        }
    }
    #[test]
    fn p1484_bundle_cli_compile_watch_eval_e_combinacoes() {
        use typst_core::entities::compiler_features::Feature;
        for command in ["compile", "watch"] {
            let parsed = Args::try_parse_from([
                "typst",
                command,
                "in.typ",
                "--features",
                "html,bundle,a11y-extras,bundle",
            ])
            .expect("features combinadas devem parsear");
            let args = match parsed.command {
                Command::Compile(args) => args.compile,
                Command::Watch(args) => args.compile,
                _ => panic!("compile/watch esperado"),
            };
            let intent =
                compile_intent(args, HtmlSerialization::Crystalline, false, None);
            assert!(intent.features.contains(Feature::Bundle), "{command}");
            assert!(intent.features.contains(Feature::Html), "{command}");
            assert!(intent.features.contains(Feature::A11yExtras), "{command}");
        }
        let parsed = Args::try_parse_from([
            "typst",
            "eval",
            "type(asset)",
            "--features",
            "bundle",
        ])
        .expect("eval --features bundle deve parsear");
        let Command::Eval(args) = parsed.command else { panic!("eval esperado") };
        let features = resolve_features(&args.features);
        assert!(features.contains(Feature::Bundle));
        assert!(!features.contains(Feature::Html));
        assert!(!features.contains(Feature::A11yExtras));
        assert!(!resolve_features(&[]).contains(Feature::Bundle));
        assert!(Args::try_parse_from([
            "typst",
            "eval",
            "type(asset)",
            "--features",
            "unknown",
        ])
        .is_err());
    }
}

#[cfg(all(test, unix))]
mod warnings_cli_hint_tests {
    use super::*;
    fn intent() -> QueryIntent {
        QueryIntent {
            input: PathBuf::from("-"),
            selector: "heading".into(),
            field: None,
            one: false,
            pretty: false,
            format: QueryFormat::Json,
            colored: false,
            cert_path: None,
        }
    }
    #[test]
    fn specific_hint_covers_cardinality_field_repr_and_shell_quotes() {
        let mut query = intent();
        assert_eq!(
            query_deprecation_hint(&query),
            "use `typst eval 'query(heading)'` instead"
        );
        query.one = true;
        assert_eq!(
            query_deprecation_hint(&query),
            "use `typst eval 'query(heading).first()'` instead"
        );
        query.field = Some("body".into());
        assert_eq!(
            query_deprecation_hint(&query),
            "use `typst eval 'query(heading).first().body'` instead"
        );
        query.one = false;
        assert_eq!(
            query_deprecation_hint(&query),
            "use `typst eval 'query(heading).map(it => it.body)'` instead"
        );
        query.field = Some("a b".into());
        assert_eq!(
            query_deprecation_hint(&query),
            "use `typst eval 'query(heading).map(it => it.at(\"a b\"))'` instead"
        );
        query.selector = "heading.where(body: [it's Ω])".into();
        query.field = None;
        query.input = PathBuf::from("pasta Ω/a b.typ");
        assert_eq!(query_deprecation_hint(&query), "use `typst eval 'query(heading.where(body: [it'\\''s Ω]))' --in pasta Ω/a b.typ` instead");
    }
}

#[cfg(test)]
mod smartquote_public_name_tests {
    use super::*;
    #[test]
    fn smartquote_public_name_is_canonical() {
        let value =
            Value::Content(typst_core::entities::content::Content::smartquote(false));
        let encoded = serialize_eval(&value, EvalFormat::Json, false).unwrap();
        let json: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(json["func"], "smartquote");
    }
}

#[cfg(test)]
mod matrix_morphology_tests {
    use super::*;
    #[test]
    fn matrix_morphology_json_keeps_rows_cells_and_empty_padding() {
        use typst_core::entities::content::Content;
        let matrix = Content::math_matrix(
            vec![
                vec![Content::text("a"), Content::text("b")],
                vec![Content::text("c"), Content::Empty],
            ],
            ('(', ')'),
        );
        let json = semantic_to_json(&content_to_semantic(&matrix));
        assert_eq!(
            json,
            serde_json::json!({"func":"mat","delim":["(",")"],"rows":[[{"func":"text","text":"a"},{"func":"text","text":"b"}],[{"func":"text","text":"c"},{"func":"sequence","children":[]}]]})
        );
    }
}

#[cfg(test)]
mod content_serialization_regressions {
    use super::*;
    use typst_core::entities::content::Content;
    use typst_core::entities::file_id::FileId;
    use typst_core::entities::span::Span;
    use typst_core::entities::style::Styles;
    use typst_core::entities::value::IntrospectedContent;

    fn sourced(content: Content) -> Content {
        let file = FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
        Content::Source(std::sync::Arc::new(
            typst_core::entities::content::SourceContent {
                body: content,
                span: Span::from_range(file, 1..2),
            },
        ))
    }

    #[test]
    fn content_serialization_empty_is_sequence_in_eval_and_query() {
        for content in [Content::Empty, sourced(Content::Empty)] {
            let value = Value::Content(content);
            for pretty in [false, true] {
                let bytes = serialize_eval(&value, EvalFormat::Json, pretty).unwrap();
                assert_eq!(
                    serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(),
                    serde_json::json!({"func":"sequence","children":[]})
                );
            }
            let bytes = serialize_query(&[value], None, true, false).unwrap();
            assert_eq!(
                serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(),
                serde_json::json!({"func":"sequence","children":[]})
            );
        }
    }

    #[test]
    fn content_serialization_nested_empty_keeps_structural_child() {
        let value =
            Value::Content(Content::styled(sourced(Content::Empty), Styles::new()));
        let bytes = serialize_eval(&value, EvalFormat::Json, false).unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(),
            serde_json::json!({"func":"styled","child":{"func":"sequence","children":[]},"styles":"styles(..)"})
        );
        let yaml =
            String::from_utf8(serialize_eval(&value, EvalFormat::Yaml, false).unwrap())
                .unwrap();
        assert!(yaml.contains("  func: sequence\n  children: []\n"));
    }

    #[test]
    fn content_serialization_raw_keeps_available_text_and_language() {
        let raw = Content::raw("é\n\"x\"", Some("typc".into()), false);
        for content in [raw.clone(), sourced(raw)] {
            let bytes = serialize_eval(&Value::Content(content), EvalFormat::Json, false)
                .unwrap();
            let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(json["func"], "raw");
            assert_eq!(json["text"], "é\n\"x\"");
            assert_eq!(json["lang"], "typc");
        }
    }

    #[test]
    fn content_serialization_snapshot_empty_remains_authoritative() {
        let mut locator = typst_core::entities::locator::Locator::new();
        for (content, name) in [
            (sourced(Content::Empty), "sequence"),
            (Content::raw("hidden", Some("typc".into()), false), "raw"),
        ] {
            let value = Value::LocatedContent(
                IntrospectedContent::new(content, Some(Default::default())),
                locator.next(),
            );
            let bytes = serialize_eval(&value, EvalFormat::Json, false).unwrap();
            assert_eq!(
                serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(),
                serde_json::json!({"func":name})
            );
        }
    }

    #[test]
    fn raw_presence_cli_constructor_block_and_snapshot() {
        for snapshot in [None, Some(Default::default())] {
            let present = snapshot.is_some();
            let raw = sourced(Content::raw("x", None, false));
            let value = Value::LocatedContent(
                IntrospectedContent::new(raw, snapshot),
                typst_core::entities::locator::Locator::new().next(),
            );
            let bytes = serialize_eval(&value, EvalFormat::Json, false).unwrap();
            let expected = if present {
                serde_json::json!({"func":"raw"})
            } else {
                serde_json::json!({"func":"raw","text":"x","block":false})
            };
            assert_eq!(
                serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(),
                expected
            );
            let bytes = serialize_query(&[value], None, true, false).unwrap();
            assert_eq!(
                serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(),
                expected
            );
        }
    }
}

#[cfg(test)]
mod v16_walk_entry_serialization_tests {
    use super::*;
    use std::sync::Arc;
    use typst_core::entities::{
        content::{Content, LocatedContent, SourceContent},
        file_id::FileId,
        locator::Locator,
        span::Span,
        value::IntrospectedContent,
    };

    fn entry() -> Content {
        Content::outline_entry(
            2,
            Content::heading(2, Content::strong(Content::text("semantic"))),
            None,
        )
    }
    fn located_value() -> Value {
        Value::LocatedContent(
            IntrospectedContent::new(Content::label_auto("direct", entry()), None),
            Locator::new().next(),
        )
    }

    #[test]
    fn v16_walk_cli_entry_fields_survive_legacy_wrappers() {
        let sourced = Content::Source(Arc::new(SourceContent {
            body: entry(),
            span: Span::from_range(
                FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
                2..5,
            ),
        }));
        let located = Content::Located(Arc::new(LocatedContent {
            location: Locator::new().next(),
            body: sourced,
        }));
        for content in [entry(), located, Content::label_auto("direct", entry())] {
            let bytes = serialize_eval(&Value::Content(content), EvalFormat::Json, false)
                .unwrap();
            let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(json["func"], "entry");
            assert_eq!(json["level"], 2);
            assert_eq!(json["element"]["func"], "heading");
            assert_eq!(json["element"]["body"]["func"], "strong");
            assert_eq!(json["element"]["body"]["body"]["text"], "semantic");
            assert!(json.get("fill").is_some());
            assert!(json["fill"].is_null());
        }
    }

    #[test]
    fn v16_walk_cli_entry_query_field_none_snapshot_keeps_json_yaml_and_one() {
        let value = located_value();
        for (format, expected) in
            [(QueryFormat::Json, "[2]\n"), (QueryFormat::Yaml, "- 2\n")]
        {
            let bytes = serialize_query_with_format(
                std::slice::from_ref(&value),
                Some("level"),
                false,
                false,
                format,
            )
            .unwrap();
            assert_eq!(String::from_utf8(bytes).unwrap(), expected);
            let bytes = serialize_query_with_format(
                std::slice::from_ref(&value),
                Some("level"),
                true,
                false,
                format,
            )
            .unwrap();
            assert_eq!(String::from_utf8(bytes).unwrap(), "2\n");
        }
    }

    #[test]
    fn v16_walk_cli_entry_present_snapshots_remain_authoritative() {
        let location = Locator::new().next();
        let empty = Value::LocatedContent(
            IntrospectedContent::new(entry(), Some(Default::default())),
            location,
        );
        assert_eq!(
            serialize_query(&[empty], Some("level"), false, false).unwrap(),
            b"[]\n"
        );
        let value = Value::LocatedContent(
            IntrospectedContent::new(
                entry(),
                Some([("level".into(), Value::Int(7))].into_iter().collect()),
            ),
            location,
        );
        assert_eq!(
            serialize_query(&[value], Some("level"), false, false).unwrap(),
            b"[7]\n"
        );
    }
}

#[cfg(test)]
mod caption_shape_tests {
    use super::*;
    #[test]
    fn caption_shape_cli_json_uses_public_name_and_known_payload() {
        use typst_core::entities::content::Content;
        use typst_core::entities::elements::figure_caption::CaptionPosition;
        let value = Value::Content(Content::figure_caption(
            Content::text("C"),
            Some(CaptionPosition::Top),
            Some(Content::text("S")),
        ));
        let encoded = serialize_eval(&value, EvalFormat::Json, false).unwrap();
        let actual: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(
            actual,
            serde_json::json!({"func":"caption","position":"top","separator":{"func":"text","text":"S"},"body":{"func":"text","text":"C"}})
        );
    }
}
