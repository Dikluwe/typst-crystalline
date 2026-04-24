# Shell CLI — typst-shell::cli
Hash do Código: 22d2d690

## Módulo
`02_shell/src/cli.rs`

## Propósito

Ponto único de entrada da CLI: argparsing via clap, modo de
coloração (`ColorWhen`), resolução de `RunIntent` para L4 consumir.

Materializado no Passo 117 (ADR-0049) depois de os Passos 113–116
terem colocado CLI incorrectamente em L3 e L4.
Passo 120 (ADR-0051) adicionou flag `-o/--output` com default
derivado — primeira flag funcional.
Passo 121 (ADR-0051) adicionou flag `--root DIR` com fallback
`input.parent() → "."` — segunda flag funcional aplicando o mesmo
pattern.
Passo 122 (ADR-0051) adicionou flag `--font-path DIR` (repetível
via `ArgAction::Append`) com passagem directa `Vec<PathBuf>` para
L4 — terceira flag; fecha o preview original da ADR-0051.

## Contrato

### `ColorWhen` — enum público

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum ColorWhen { Auto, Always, Never }
```

- Variantes com docstrings — viram descrições no `--help` de clap.
- `Copy` + `PartialEq` + `Eq` — valor pequeno, barato de passar.

### `RunIntent` — struct pública

```rust
#[derive(Debug)]
pub struct RunIntent {
    pub input: PathBuf,
    pub output: PathBuf,
    pub root: PathBuf,
    pub font_paths: Vec<PathBuf>,
    pub colored: bool,
}
```

Output puro de `parse()`. L4 consome sem conhecer clap ou env
vars. À medida que flags forem adicionadas em passos futuros,
`RunIntent` ganha campos (sempre como dados crus, nunca
estruturas de clap).

### `parse() -> RunIntent` — API pública

```rust
pub fn parse() -> RunIntent;
```

- Usa `Args::parse()` de clap — em erro de argumentos, clap
  imprime mensagem em stderr e termina o processo (exit 2).
- Em sucesso, traduz `Args` → `RunIntent` e resolve `colored`
  via `resolve_colored`.

### `resolve_colored_with(choice, no_color, is_tty) -> bool` — API pública

Função **pura** (sem I/O, sem env). Testável directamente.

Ordem de precedência (ADR-0048):
1. Flag explícita (`Always` / `Never`) vence tudo.
2. Em `Auto`, `NO_COLOR` desactiva.
3. Em `Auto` sem `NO_COLOR`, decide `is_tty`.

### `resolve_output_with(input, output, output_flag) -> PathBuf` — API pública

Função **pura** (sem I/O). Testável directamente.

Ordem de precedência (ADR-0051):
1. `output_flag` (via `-o/--output`) vence.
2. `output` positional (se presente).
3. Default derivado: `input.with_extension("pdf")`.

### `resolve_root_with(root, input) -> PathBuf` — API pública

Função **pura** (sem I/O; não verifica existência). Testável
directamente.

Ordem de precedência (Passo 121, ADR-0051, alinhada com vanilla
typst-cli):
1. Flag `--root` explícita vence.
2. `input.parent()` se não vazio.
3. Default `"."` (cwd).

### `Args` — struct privada (clap)

```rust
#[derive(Parser, Debug)]
#[command(name = "typst", version, about = "...")]
struct Args {
    input: PathBuf,
    output: Option<PathBuf>,           // positional opcional
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    output_flag: Option<PathBuf>,      // sinónimo via flag
    #[arg(long = "root", value_name = "DIR")]
    root: Option<PathBuf>,             // project root (Passo 121)
    #[arg(long = "font-path", value_name = "DIR",
          action = clap::ArgAction::Append)]
    font_paths: Vec<PathBuf>,          // repetível (Passo 122)
    #[arg(long = "color", value_enum, default_value_t = ColorWhen::Auto)]
    color: ColorWhen,
}
```

Não exposta — L4 só conhece `parse()` e `RunIntent`.

**Nota sobre `output_flag`**: nome interno divergente do clap
`--output` para evitar colisão com campo positional `output`.
Help mostra `-o, --output`.

## Testes

15 testes unitários em `#[cfg(test)] mod tests`:

**`resolve_colored_with`** (6 testes):
- `resolve_colored_never_e_false`
- `resolve_colored_always_e_true`
- `resolve_colored_auto_sem_tty_e_false`
- `resolve_colored_auto_com_tty_e_sem_no_color_e_true`
- `resolve_colored_auto_com_no_color_e_false`
- `resolve_colored_always_vence_no_color`

**`resolve_output_with`** (6 testes):
- `resolve_output_flag_vence_positional`
- `resolve_output_positional_usa_quando_sem_flag`
- `resolve_output_flag_usa_sem_positional`
- `resolve_output_ambos_omitidos_usa_default_derivado`
- `resolve_output_default_com_path_completo`
- `resolve_output_default_sem_extensao_adiciona_pdf`

**`resolve_root_with`** (3 testes):
- `resolve_root_flag_vence_parent`
- `resolve_root_sem_flag_usa_parent_do_input`
- `resolve_root_sem_flag_e_sem_parent_usa_dot`

## Evolução

O preview original de ADR-0051 fica fechado no Passo 122 (-o,
--root, --font-path). Futuros flags (`--format`, `--ignore-system-fonts`,
env vars, subcomandos) entram **aqui** — não em L3 nem L4.
`RunIntent` ganha campos conforme necessário. Padrão estabelecido
pelos Passos 117, 120, 121 e 122 (ADR-0051).

## Nota sobre `font_paths` (Passo 122)

- **`ArgAction::Append`** (não `value_delimiter`): divergência
  deliberada vs vanilla (que usa `:`/`;` separador). UX moderna:
  `--font-path /a --font-path /b`. Documentado em `--help`.
- **Passagem directa** para L4 sem helper `resolve_font_paths_with`:
  lógica é `args.font_paths` move. P6 de ADR-0051 é sobre
  testabilidade; passagem directa não precisa de helper.
- **I/O em L3**: L2 não descobre fontes. `discover_fonts(&paths)`
  vive em `typst_infra::fonts`; L4 compõe.
