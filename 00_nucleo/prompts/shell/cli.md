# Shell CLI — typst-shell::cli
Hash do Código: eb68469e

## Módulo
`02_shell/src/cli.rs`

## Propósito

Ponto único de entrada da CLI: argparsing via clap, modo de
coloração (`ColorWhen`), resolução de `RunIntent` para L4 consumir.

Materializado no Passo 117 (ADR-0049) depois de os Passos 113–116
terem colocado CLI incorrectamente em L3 e L4.

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

### `Args` — struct privada (clap)

```rust
#[derive(Parser, Debug)]
#[command(name = "typst", version, about = "...")]
struct Args {
    input: PathBuf,
    output: PathBuf,
    #[arg(long = "color", value_enum, default_value_t = ColorWhen::Auto)]
    color: ColorWhen,
}
```

Não exposta — L4 só conhece `parse()` e `RunIntent`.

## Testes

6 testes unitários em `#[cfg(test)] mod tests` validam
`resolve_colored_with` em todas as combinações relevantes:

- `resolve_colored_never_e_false`
- `resolve_colored_always_e_true`
- `resolve_colored_auto_sem_tty_e_false`
- `resolve_colored_auto_com_tty_e_sem_no_color_e_true`
- `resolve_colored_auto_com_no_color_e_false`
- `resolve_colored_always_vence_no_color`

## Evolução

Futuros flags (`--root`, `--font-path`, `--format`, `-o`,
subcomandos) entram **aqui** — não em L3 nem L4. `RunIntent`
ganha campos conforme necessário. Padrão estabelecido pelo
Passo 117.
