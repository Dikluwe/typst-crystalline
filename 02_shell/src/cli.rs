//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/cli.md
//! @prompt-hash d35e4dc4
//! @layer L2
//! @updated 2026-07-21
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

use std::io::IsTerminal;
use std::path::{Path, PathBuf};

use clap::Parser;

/// Separador de paths em env vars estilo `PATH` (Passo 123).
///
/// Alinhado com vanilla typst-cli: `':'` em Unix, `';'` em Windows.
/// Usado por `--font-path` para suportar `TYPST_FONT_PATHS=/a:/b`.
const ENV_PATH_SEP: char = if cfg!(windows) { ';' } else { ':' };

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
// §"Decisão — número de versão do CLI"). Formato final: `typst 0.15.0
// (⟨commit curto⟩)`, mesmo formato do vanilla (`typst 0.15.0 (969087ec)`).
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
    /// Input .typ file.
    input: PathBuf,

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
        action = clap::ArgAction::Append,
    )]
    font_paths: Vec<PathBuf>,

    /// When to use coloured diagnostics.
    #[arg(long = "color", value_enum, default_value_t = ColorWhen::Auto)]
    color: ColorWhen,

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
}

/// Intenção de execução — output puro de L2 para L4 (ADR-0049).
///
/// L2 traduz argumentos + env vars + isatty para este struct.
/// L4 consome directamente sem conhecer clap ou env vars.
#[derive(Debug)]
pub struct RunIntent {
    pub input: PathBuf,
    pub output: PathBuf,
    /// P866 — formato de saída resolvido pela extensão ou `--format`.
    pub output_format: OutputFormat,
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
}

/// Ponto de entrada público da CLI.
///
/// Em erro de argumentos, `Args::parse()` (clap) imprime mensagem
/// em stderr e termina o processo com exit 2. Em sucesso, devolve
/// `RunIntent` com `output`, `root` e `colored` já resolvidos.
pub fn parse() -> RunIntent {
    let args = Args::parse();
    let colored = resolve_colored(&args.color);
    let output =
        resolve_output_with(&args.input, args.output.as_ref(), args.output_flag.as_ref());
    // P866 — resolver formato antes de validações dependentes de caminho.
    let output_format = resolve_output_format_with(
        args.format.as_ref(),
        args.output_flag.as_ref().or(args.output.as_ref()),
        OutputFormat::Pdf,
    );
    let root = resolve_root_with(args.root.as_ref(), &args.input);

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

    RunIntent {
        input: args.input,
        output,
        output_format,
        root,
        font_paths: args.font_paths,
        colored,
        // P350c → P428 (DEBT-59): a flag CLI é parseada e fiada a L4/L1.
        // Default `false` preserva comportamento byte-idêntico ao vanilla.
        full_error: args.full_error,
        timings_json: args.timings_json,
        document_id,
        inputs,
        compact: args.compact,
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
///    `pdf`, `png` ou `svg` (case-insensitive).
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

    #[test]
    fn resolve_colored_never_e_false() {
        assert_eq!(resolve_colored_with(&ColorWhen::Never, false, false), false);
        assert_eq!(resolve_colored_with(&ColorWhen::Never, true, false), false);
        assert_eq!(resolve_colored_with(&ColorWhen::Never, false, true), false);
        assert_eq!(resolve_colored_with(&ColorWhen::Never, true, true), false);
    }

    #[test]
    fn resolve_colored_always_e_true() {
        assert_eq!(resolve_colored_with(&ColorWhen::Always, false, false), true);
        assert_eq!(resolve_colored_with(&ColorWhen::Always, true, false), true);
        assert_eq!(resolve_colored_with(&ColorWhen::Always, false, true), true);
        assert_eq!(resolve_colored_with(&ColorWhen::Always, true, true), true);
    }

    #[test]
    fn resolve_colored_auto_sem_tty_e_false() {
        assert_eq!(resolve_colored_with(&ColorWhen::Auto, false, false), false);
    }

    #[test]
    fn resolve_colored_auto_com_tty_e_sem_no_color_e_true() {
        assert_eq!(resolve_colored_with(&ColorWhen::Auto, false, true), true);
    }

    #[test]
    fn resolve_colored_auto_com_no_color_e_false() {
        assert_eq!(resolve_colored_with(&ColorWhen::Auto, true, true), false);
        assert_eq!(resolve_colored_with(&ColorWhen::Auto, true, false), false);
    }

    #[test]
    fn resolve_colored_always_vence_no_color() {
        assert_eq!(resolve_colored_with(&ColorWhen::Always, true, false), true);
        assert_eq!(resolve_colored_with(&ColorWhen::Always, true, true), true);
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
        let fmt = resolve_output_format_with(Some(&flag), Some(&output), OutputFormat::Pdf);
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
}
