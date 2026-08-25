# Shell CLI — typst-shell::cli
Hash do Código: 3b2c4f4f

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
Passo 123 (ADR-0051) adiciona env vars `TYPST_ROOT` e
`TYPST_FONT_PATHS` + `value_delimiter = ENV_PATH_SEP` em
`--font-path` (feature `env` do clap). Precedência: flag > env >
default.

**P617** — adiciona `--document-id <UUID>` (e env var
`CRYSTALLINE_DOCUMENT_ID`) para fornecer um `DocumentID` externo e
estável entre compilações. `InstanceID` continua aleatório. A flag é
validada em L2; se o valor não for um UUID bem formado, o processo
termina com erro claro (exit 2). O campo `RunIntent.document_id`
transporta os 16 bytes para L4/L3.

**P796** — corrige `--version`: ver secção dedicada "Decisão — número de
versão do CLI" abaixo. Formato passa de `typst 0.1.0` (versão do crate
Cargo, sem hash) para `typst 0.15.1 (⟨commit curto⟩)`, consistente com
`sys.version` (`entities/version.md` §9) e com o mecanismo do vanilla
(mesmo formato; o número concreto acompanha `PARITY_VERSION`).

## Decisão — número de versão do CLI (P796)

**Achado (P786/P796)**: `typst --version` mostrava `typst 0.1.0` — a versão
do crate Cargo (`[workspace.package] version = "0.1.0"`, identidade própria
do projecto cristalino), sem hash de commit, e inconsistente com
`sys.version` (que reporta `0.15.1`, a versão de paridade com a linguagem
Typst — ver `compiler/stdlib/sys.md`).

**Decisão explícita** (instrução directa do dono do projecto em P796,
2026-07-21): copiar o **mecanismo** do vanilla directamente por agora —
`typst_utils::version()` + `build.rs` que captura `git rev-parse HEAD` em
tempo de build (`lab/typst-original/crates/typst-utils/{src/version.rs,
build.rs}`). A identidade de versão própria do cristalino (divergindo do
vanilla por design) fica para decisão futura — **não** decidida agora, e
**não** deve ser assumida como "copiar o vanilla é definitivo".

Isto **não** significa adoptar `0.15.1` como versão do *crate* Cargo — as
duas coisas são conceptualmente distintas e já divergiam antes de P796:

- Versão do **crate** (`Cargo.toml`, `[workspace.package] version`): `0.1.0`,
  identidade própria do projecto. **Inalterada por P796.**
- Versão de **paridade** (`PARITY_VERSION`, `entities/version.md` §9):
  `0.15.1`, "que versão da linguagem Typst este compilador implementa".
  É esta que `--version` passa a mostrar, pela mesma razão que
  `sys.version` já a mostra (ADR-0107 — paridade é com a linguagem).

`--version` mostra a versão de **paridade** (não a do crate) porque é o
número que responde à pergunta que um utilizador faz ao correr
`typst --version`: "que Typst é este". O hash de commit é o do **próprio
repositório cristalino** (não o do vanilla — `969087ec` é do vanilla e nunca
aparecerá no nosso binário), capturado da mesma forma que o vanilla captura
o seu.

**Correção P1137:** `0.15.0` era esquecimento, não modo de compatibilidade.
O valor passa a `0.15.1`, exatamente o `sys.version` do vanilla ratificado
`a51e02804`. O CLI não escolhe a versão independentemente: continua a consumir
a fonte única `PARITY_VERSION`. Um re-sync futuro só muda o número por passo
explícito que substitua o hash pinado, conforme `entities/version.md` §9a.

### Mecanismo (mirror directo do vanilla)

`02_shell/build.rs` (novo, mesma lógica de
`typst-utils/build.rs`, sem a parte de `TYPST_VERSION` — a versão vem da
constante Rust `PARITY_VERSION`, não precisa de env var):

```rust
fn main() {
    println!("cargo:rerun-if-env-changed=TYPST_COMMIT_SHA");
    if option_env!("TYPST_COMMIT_SHA").is_none() {
        if let Some(sha) = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output().ok()
            .filter(|o| o.status.success())
            .and_then(|o| String::from_utf8(o.stdout).ok())
        {
            println!("cargo:rustc-env=TYPST_COMMIT_SHA={}", sha.trim());
        }
    }
}
```

`cli.rs` — `Args` troca o atributo `version` implícito do clap (que lia
`CARGO_PKG_VERSION` do crate `typst-shell`) por um valor explícito:

```rust
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
struct Args { /* ... */ }

/// Trunca o hash a 8 chars — mirror de `typst_utils::display_commit`.
fn display_commit(commit: Option<&'static str>) -> &'static str {
    const LENGTH: usize = 8;
    match commit {
        Some(s) => &s[..s.len().min(LENGTH)],
        None => "unknown commit",
    }
}
```

- `option_env!("TYPST_COMMIT_SHA")` só resolve o valor que o **build.rs deste
  crate** (`02_shell`) definiu via `cargo:rustc-env` — mecanismo compile-time,
  zero I/O em runtime; não viola nenhuma restrição de camada (build scripts
  não são código L1/L2 em execução, são tooling de compilação, mesmo
  tratamento que o vanilla dá ao seu).
- Sem repositório git (ex.: tarball sem `.git`), `git rev-parse HEAD` falha
  silenciosamente (`.ok()`), `TYPST_COMMIT_SHA` fica por definir,
  `display_commit` devolve `"unknown commit"` — mesmo fallback do vanilla.
- `02_shell/Cargo.toml` activa `clap = { workspace = true, features =
  ["string"] }`: `version = format!(...)` produz `String`, não `&'static
  str`, e `clap::builder::Str: From<String>` só existe com a feature
  `"string"` do clap (`clap_builder-*/src/builder/str.rs`, `#[cfg(feature =
  "string")]`). Mesma feature que o vanilla activa em
  `typst-cli/Cargo.toml:33` para o mesmo padrão — achado durante a
  implementação de P796 (erro de compilação `E0277` sem a feature).

## Contrato

### `ColorWhen` — enum público

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum ColorWhen { Auto, Always, Never }
```

- Variantes com docstrings — viram descrições no `--help` de clap.
- `Copy` + `PartialEq` + `Eq` — valor pequeno, barato de passar.

### `OutputFormat` — enum público (P866)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum OutputFormat { Pdf, Png, Svg }
```

Formato de saída do compilador. Alinhado com o vanilla
(`typst-cli/src/args.rs::OutputFormat`) mas restrito aos formatos
paginados (`Pdf`, `Png`, `Svg`). `Html` e `Bundle` ficam fora do
escopo do cristalino até decisão futura.

- `Copy` + `PartialEq` + `Eq` — valor pequeno, barato de passar.
- `clap::ValueEnum` para suporte a `--format pdf|png|svg`.

### `RunIntent` — struct pública

```rust
#[derive(Debug)]
pub struct RunIntent {
    pub input: PathBuf,
    pub output: PathBuf,
    pub output_format: OutputFormat, // P866 — formato resolvido pela extensão ou --format
    pub root: PathBuf,
    pub font_paths: Vec<PathBuf>,
    pub colored: bool,
    pub full_error: bool, // P350c — origem da flag de erro completo
    pub document_id: Option<[u8; 16]>, // P617 — DocumentID externo (UUID)
    pub inputs: Vec<(String, String)>, // P694 — pares `--input chave=valor` (raw; L3 converte para SysInputs)
}
```

Output puro de `parse()`. L4 consome sem conhecer clap ou env
vars. À medida que flags forem adicionadas em passos futuros,
`RunIntent` ganha campos (sempre como dados crus, nunca
estruturas de clap).

**`full_error` (P350c)** — origem (casa) da flag de "erro completo"
(capacidade interna em L1, `EvalContext::full_error`: classifica o erro de
recursão de `#show` em cíclico/não-convergente num 3º hint). **Débito P350c**:
(1) o `Arg --full-error` em `Args` (hoje `parse()` fixa `false`); (2) o fio
`RunIntent.full_error` → L1 pelo caminho **interno** de L3 (`eval_with_full_error`)
— a assinatura **pública** de `compile_to_pdf_bytes` **não** muda.

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

### `resolve_output_format_with(format_flag, output, default) -> OutputFormat` — API pública (P866)

Função **pura** (sem I/O). Testável directamente.

Ordem de precedência (alinhada com vanilla `CompileConfig::new_impl`):
1. `format_flag` (via `--format`) vence se presente.
2. Extensão do path de `output` (flag `-o` ou positional) se for
   `pdf`, `png` ou `svg` (case-insensitive).
3. `default` (tipicamente `OutputFormat::Pdf`).

Esta função **não** valida se o formato é suportado pelo backend
(L3/L4) — isso é decisão de L4, que pode recusar formatos não
implementados com mensagem clara.

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
#[command(name = "typst", version = format!("{}.{}.{} ({})", ...), about = "...")]
// P796 — version deixa de ser o atributo implícito (CARGO_PKG_VERSION de
// typst-shell, "0.1.0"); ver secção "Decisão — número de versão do CLI".
struct Args {
    input: PathBuf,
    output: Option<PathBuf>,           // positional opcional
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    output_flag: Option<PathBuf>,      // sinónimo via flag
    #[arg(long = "root", env = "TYPST_ROOT", value_name = "DIR")]
    root: Option<PathBuf>,             // project root (121); env em 123
    #[arg(long = "font-path", env = "TYPST_FONT_PATHS", value_name = "DIR",
          value_delimiter = ENV_PATH_SEP,
          action = clap::ArgAction::Append)]
    font_paths: Vec<PathBuf>,          // repetível (122); env+delim em 123
    #[arg(long = "document-id", env = "CRYSTALLINE_DOCUMENT_ID", value_name = "UUID")]
    document_id: Option<String>,       // P617 — UUID externo para DocumentID
    #[arg(long = "input", value_name = "chave=valor",
          action = clap::ArgAction::Append)]
    inputs: Vec<String>,               // P694 — popula sys.inputs (repetível)
    #[arg(long = "format", short = 'f', value_enum, value_name = "FORMAT")]
    format: Option<OutputFormat>,      // P866 — formato explícito (vence extensão)
    #[arg(long = "color", value_enum, default_value_t = ColorWhen::Auto)]
    color: ColorWhen,
}
```

Não exposta — L4 só conhece `parse()` e `RunIntent`.

**Nota sobre `output_flag`**: nome interno divergente do clap
`--output` para evitar colisão com campo positional `output`.
Help mostra `-o, --output`.

### Validação de `--input` (P694)

Cada `--input` tem de ter a forma `chave=valor` (split no **primeiro** `=`,
para permitir `=` no valor). Entradas sem `=` ou com chave vazia são erro
claro em L2 (`eprintln!` + exit 2), à semelhança de `--document-id`. Os pares
validados entram em `RunIntent.inputs` como `Vec<(String, String)>` crus; a
conversão para `SysInputs` (IndexMap<EcoString, EcoString, _>) acontece em L3
(`SystemWorld::with_inputs`), mantendo L2 livre de `ecow`/`indexmap`. Os
valores ficam strings (paridade vanilla: `--input n=42` → `"42"`).

### Validação de `--document-id` (P617)

- Formato aceite: UUID textual (`8-4-4-4-12` hex) ou 32 hex sem
  hífenes.
- Conversão para `[u8; 16]` em L2; valor inválido → mensagem de erro
  em stderr e exit 2 (sem passar para L4).
- Precedência: flag `--document-id` > `CRYSTALLINE_DOCUMENT_ID` >
  `None` (comportamento por defeito de P615).

## Testes

20 testes unitários em `#[cfg(test)] mod tests`:

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

**`resolve_output_format_with`** (5 testes — P866):
- `resolve_output_format_flag_vence_extensao`
- `resolve_output_format_detecta_png`
- `resolve_output_format_detecta_svg`
- `resolve_output_format_pdf_default`
- `resolve_output_format_case_insensitive`

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

## P1137-B-001 — subcomando `eval` (gate ADR-0127)

### Medição anterior à decisão

Medição em 2026-08-23 contra o vanilla ratificado `upstream/main a51e02804`:

- `lab/typst-original/crates/typst-cli/src/args.rs:81-99` declara `Command::Eval`;
- `lab/typst-original/crates/typst-cli/src/args.rs:195-224` recebe uma expressão,
  `--in`, `--target`, `--format` e `--pretty`;
- `lab/typst-original/crates/typst-cli/src/eval.rs:102-159` avalia a expressão em
  `SyntaxMode::Code` com scope fresco e serializa `json|yaml|raw`;
- `typst eval 'calc.gcd(12, 18)' --format json` produz `6\n`, exit 0;
- `typst eval '(a: 1, b: (2, 3))' --format json` produz
  `{"a":1,"b":[2,3]}\n`, exit 0;
- `typst eval '"abc"' --format raw` produz exatamente `abc`, sem newline;
- raw sobre inteiro falha com exit 1 e informa que raw só aceita strings e bytes;
- a CLI cristalina vigente interpreta `eval` como o input legado e rejeita `json`
  como formato de exportação. Logo `P1137-B-001` é `ABSENT`, não defeito do
  comparador.

Classificação: comando, argumentos, stdout e exit code são `PUBLIC_CLI`; o valor
avaliado é `LANGUAGE_SEMANTICS` (ADR-0107). Inferência: o motor L1 já contém
`calc.gcd` e a ausência está na exposição CLI. Refutação: o RED de integração
continuar ausente depois do fio L2→L4→L1, ou o valor direto L1 divergir de `6`.

### Decisão e escopo desta primeira entrega

`RunIntent` deixa de ser struct única e torna-se enum público fechado:

```rust
pub enum RunIntent {
    Compile(CompileIntent),
    Eval(EvalIntent),
}

pub struct EvalIntent {
    pub expression: String,
    pub format: EvalFormat,
    pub pretty: bool,
    pub colored: bool,
}

pub enum EvalFormat { Json, Raw }
```

`CompileIntent` contém, sem mudança semântica, todos os campos do antigo
`RunIntent`. A invocação legada `typst INPUT [OUTPUT] ...` permanece aceite;
`typst eval EXPRESSION [--format json|raw] [--pretty]` é reconhecido antes do
positional legado. `--color` continua global e alimenta ambos os intents.

Esta entrega é deliberadamente incompleta face ao comando vanilla completo:

- `--in`, `--target` e os argumentos comuns de world/process ficam para o passo
  que ligar avaliação contextual;
- YAML fica para o passo de serialização YAML;
- `query` não é alias nem efeito colateral desta mudança: completa
  `P1137-I-001` num passo próprio;
- JSON cobre os valores representáveis pelo modelo JSON (`none`, bool, int,
  float finito, string, array e dict recursivos). Tipo não representável produz
  diagnóstico e exit 1; não cai silenciosamente em `repr`;
- raw aceita apenas `Value::Str` e `Value::Bytes`, preserva bytes e não acrescenta
  newline. Nos demais tipos, exit 1.

Critérios RED→GREEN:

- o parser L2 distingue `EvalIntent` de `CompileIntent` e preserva o comando
  legado de compilação;
- `eval calc.gcd(12, 18) --format json` produz `6\n`, exit 0;
- dict/array serializa como JSON estrutural, não como `repr`;
- raw string não acrescenta newline; raw inteiro falha com exit 1;
- expressão inválida produz diagnóstico em stderr e exit 1;
- `P1137-B-001` passa de `ABSENT` para `MATCH` sem normalização no runner;
- `query` permanece explicitamente `ABSENT` nesta entrega.

### P1163 — serialização de `Symbol` dentro do escopo vigente

Medição em 2026-08-25 contra o vanilla ratificado `a51e02804` e sua fonte
`typst-cli/src/eval.rs:102-159`, `foundations/symbol.rs:339-400`:

```text
typst eval 'emoji.heart' --format json       -> "❤️" + newline, exit 0
typst eval 'symbol("👩‍💻")' --format json     -> "👩‍💻" + newline, exit 0
typst eval 'emoji.heart' --format raw        -> erro "cannot print symbol ...",
                                                hint de string/bytes, exit 1
```

Decisão de paridade: JSON serializa `Value::Symbol` como o grapheme efetivo,
preservando todos os scalar values; `--pretty` não muda a forma de uma string.
Raw continua restrito a string/bytes e o diagnóstico usa o nome público
`symbol`, não o fallback `value`. O `Serialize` genérico do vanilla é mecânica:
o cristalino pode mapear diretamente no serializer L2.

YAML foi re-medido e continua ausente por decisão explícita desta primeira
entrega. Adicionar `EvalFormat::Yaml` ampliaria enum/contrato público e requer
passo com gate ADR-0127; P1163 não o introduz.

## P1137-I-001 — subcomando deprecated `query` (gate ADR-0127)

### Medição anterior à decisão

No vanilla ratificado `a51e02804`, `query INPUT heading --format json` sobre
`= First` produz um array com o objeto heading completo e warning de deprecação;
`--field level` produz `[1]`; `--one` produz o objeto único. A definição está em
`typst-cli/src/args.rs:156-193`. A CLI cristalina ainda rejeita o comando.

### Decisão

Adicionar `Command::Query(QueryArgs)` e `RunIntent::Query(QueryIntent)`:

```rust
pub struct QueryIntent {
    pub input: PathBuf,
    pub selector: String,
    pub field: Option<String>,
    pub one: bool,
    pub pretty: bool,
    pub colored: bool,
}
```

O único formato desta entrega é JSON (`--format json`, default). O formatter L2
serializa `Content::Heading` na forma pública vanilla medida, incluindo
`func`, `level`, `depth`, `offset`, `numbering`, `supplement`, `outlined`,
`bookmarked`, `hanging-indent` e `body`. `--field` aceita inicialmente campos
desse objeto; campo ausente falha. `--one` exige exatamente um resultado.
Outras variantes de `Content` falham explicitamente, sem `Debug` ou `repr` como
substituto. O warning deprecated é emitido em stderr.

Critérios: sentinela P1137-I-001 passa a `MATCH`; `--field level` e `--one`
igualam o stdout vanilla; zero/múltiplos com `--one` falham; compile legado e
`eval` permanecem verdes.

**P866** — detecção de formato de saída pela extensão (`-o simple.png`
→ `OutputFormat::Png`) e flag explícita `--format`. O formato é
resolvido em L2 e transportado em `RunIntent.output_format`; L4
valida se o backend consegue satisfazê-lo (PDF implementado;
PNG/SVG dependem de rasterização que ainda não existe no
repositório cristalino e são recusados com erro claro).

## Nota sobre `font_paths` (Passo 122 + 123)

- **`ArgAction::Append` + `value_delimiter = ENV_PATH_SEP`**
  combinados: `--font-path /a --font-path /b` e
  `TYPST_FONT_PATHS=/a:/b` ambos produzem `[/a, /b]`.
- **Passagem directa** para L4 sem helper `resolve_font_paths_with`:
  lógica é `args.font_paths` move. P6 de ADR-0051 é sobre
  testabilidade; passagem directa não precisa de helper.
- **I/O em L3**: L2 não descobre fontes. `discover_fonts(&paths)`
  vive em `typst_infra::fonts`; L4 compõe.

## Env vars (Passo 123)

| Flag | Env var | Precedência |
|------|---------|-------------|
| `--root` | `TYPST_ROOT` | flag > env > default (`input.parent()`) |
| `--font-path` | `TYPST_FONT_PATHS` | flag > env > default (`Vec::new()`) |

- Feature `env` do clap activa em `Cargo.toml`.
- `--help` mostra `[env: TYPST_ROOT=]` e `[env: TYPST_FONT_PATHS=]`
  automaticamente.
- `ENV_PATH_SEP: char = if cfg!(windows) { ';' } else { ':' }`
  (const privado em `cli.rs`, vanilla-style).
- Flag explícita substitui env **inteiramente** (não concatena);
  comportamento clap standard.
- `resolve_root_with` e passagem directa de `font_paths`
  **não mudam** — clap preenche `args.root` / `args.font_paths`
  transparentemente, quer da flag quer do env.


## P956 — flag `--compact`

ADR-0126 (emendada P956): o modo verbose (vanilla-espelhado) é o **padrão de
produção**; o formato Passo 20 fica atrás de uma flag.

- `Args` ganha `#[arg(long)] compact: bool` — docstring: "Emitir content
  streams no formato compacto (operadores mínimos, PDF menor). Por omissão
  emite-se o modo verbose, com a semântica de operadores do vanilla."
- `RunIntent` ganha `pub compact: bool` (dado cru, como os outros campos —
  L2 não importa o `StreamMode` de L3; a tradução é em L4, `wiring.md` §P956).
- **Ausente → verbose** (o padrão); **presente → compacto**.
- Sem efeito em `--format png|svg` (não há content stream PDF); o help diz
  isso explicitamente.
- **Mudança de comportamento por defeito**: compilar sem flag passa a emitir
  o modo verbose. É interface pública — documentada também no relatório de
  P956 e na secção de consequências da ADR-0126 emendada.

## P1137-X-002 — formato HTML (ADR-0128; gate ADR-0127)

**Medição:** `OutputFormat` possui somente Pdf/Png/Svg e `.html` não resolve.
Acrescentar `OutputFormat::Html`; extensão `.html` e `--format html` resolvem o
valor. O help identifica HTML como experimental. `Bundle` continua fora do
escopo. O intent permanece Compile; L2 não conhece o backend.

## P1137-C-001 — superfície principal do CLI (AGUARDA CONFIRMAÇÃO ADR-0127)

### Medição anterior à decisão

No vanilla ratificado `a51e02804`, `typst --help` apresenta uso
`typst [OPTIONS] <COMMAND>`, os comandos `compile` (`c`), `watch` (`w`),
`init`, `eval`, `fonts`, `completions` e `info`, e somente `--color`, `--cert`,
`--help` e `--version` no nível global. No cristalino medido em 2026-08-23, o
uso ainda é `typst [OPTIONS] [INPUT] [OUTPUT] [COMMAND]`; `eval` e o legado
deprecated `query` são os únicos subcomandos visíveis e todas as opções de
compilação aparecem globalmente.

A primeira linha também diverge por identidade: o vanilla imprime a versão e
o hash do build vanilla; o cristalino imprime a sua descrição/hash. Essa
diferença de proveniência é mecânica deliberada e não deve ser apagada nem
normalizada como igualdade literal (ADR-0107).

### Contrato proposto — mudança pública

1. `compile` torna-se o subcomando público canónico, com alias `c`, e recebe
   todos os argumentos/opções hoje pertencentes a `CompileIntent`.
2. A invocação histórica `typst INPUT [OUTPUT] ...` permanece aceite por uma
   tradução de compatibilidade anterior ao parsing; fica omitida do help.
3. `query` permanece aceite por compatibilidade, mas oculto do help principal.
4. `eval` permanece funcional e visível.
5. `watch` (`w`), `init`, `fonts`, `completions` e `info` **não serão anunciados
   antes de existir implementação real**. Cada um exige entrega própria; não
   criar stubs que façam o produto declarar capacidade ausente.
6. `--color` permanece global. `--cert` só entra quando o certificado for
   realmente fiado ao downloader; não será opção decorativa.
7. A comparação de `P1137-C-001` muda de stdout literal para inventário
   estrutural de comandos/opções/defaults. Identidade, wrapping e texto
   editorial são reportados separadamente.

Esta entrega corrige `compile`/alias e a localização das opções, mas classifica
o conjunto global como `PARTIAL` enquanto os cinco comandos funcionais e
`--cert` permanecerem ausentes. Torná-los visíveis sem implementação é
proibido pelo contrato.

### Gate

Esta secção altera contrato público e comportamento de parsing. A implementação
só pode começar após confirmação explícita do dono, conforme ADR-0127.
## P1137-CERT — CA customizada global

`Args` aceita `--cert PATH` global. L2 resolve `flag.or(TYPST_CERT)`, portanto
a flag vence o ambiente, e transporta `Option<PathBuf>` nas intenções que
podem construir um `SystemWorld` (`compile`, `eval`, `query`) e em `info` para
indicar somente presença. L2 não lê nem valida o ficheiro; essa responsabilidade
é exclusivamente L3, conforme `shell/custom-ca-cert.md`.

## P1137-INIT — intenção de projeto

O subcomando `init TEMPLATE [DIR]` produz `InitIntent` com o template cru,
destino opcional e a CA global já resolvida. Parsing do package, versão e I/O
não pertencem a L2.
## P1137-WATCH — comando contínuo

Medição anterior à decisão: o parser concentrava as opções de compilação em
`CompileArgs`, mas não possuía `watch`; a grafia `w` era tratada como input
legado. O contrato medido no vanilla ratificado está em `shell/watch.md`.

Decisão: `Command::Watch(CompileArgs)` reutiliza integralmente as opções de
compilação e expõe `w` como alias visível. `RunIntent::Watch(WatchIntent)`
transporta um `CompileIntent` resolvido pelo mesmo caminho de `compile`.
`normalize_legacy_args` reconhece `watch` e `w` como comandos explícitos. L2
apenas traduz a interface pública; não observa ficheiros nem executa o ciclo.

## P1140.6 — flag `--no-pdf-tags`

### Medição antes da decisão

O vanilla ratificado expõe `--no-pdf-tags` nas opções de compilação; ausência
da flag produz PDF tagueado. O cristalino não possui esta polaridade pública.

### Decisão

`CompileArgs` ganha `#[arg(long)] no_pdf_tags: bool` e `CompileIntent` ganha
`pub no_pdf_tags: bool`. A flag pertence igualmente a `compile` e `watch`, que
reutiliza `CompileArgs`. Ausente significa tags habilitadas; presente significa
tags desabilitadas. L2 transporta apenas o booleano com a polaridade da CLI e
não importa `PdfTags`; L4 traduz para o enum L3. A flag não altera `compact`,
não tem efeito em PNG/SVG/HTML e não promete conformidade PDF/UA.
