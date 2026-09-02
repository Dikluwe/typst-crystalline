# Shell CLI — typst-shell::cli
Hash do Código: 26707ade

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:0b28776068ad6b8e85a028a26cfc359679770b03f76d250bfd3ad68ff72643e3
- 00_nucleo/prompts/_nuclei/shell/build-identity.toml sha256:a97f32705be8700feaa6a5c89440ffa49d15c11145ea83744307aefc65a50297

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

### P1224 — proposta de serialização JSON de `Stroke` (GATE ADR-0127)

> **Estado:** contrato confirmado pelo dono em 2026-08-26. A confirmação
> aprova somente a branch nominal de `Value::Stroke`; fallback genérico para
> `repr` permanece proibido. Resselar antes da materialização em
> `02_shell/src/cli.rs`.

Medição em 2026-08-26 contra o vanilla ratificado `a51e02804`:

```text
typst eval 'stroke(paint: red, cap: "round")'
vanilla    -> "(paint: rgb(\"#ff4136\"), cap: \"round\")" + newline, exit 0
cristalino -> cannot serialize stroke to JSON, exit 1
```

A regra genérica acima (“tipo não representável produz diagnóstico”) passa a
ter uma exceção explícita: `Value::Stroke` é serializado como string JSON com
sua representação pública Typst. Isso não converte dict/array em `repr` e não
amplia `--format raw`. O formatter L2 deve consumir uma representação L1
pública estável ou reproduzir apenas o contrato de `Stroke`; não pode acessar
módulo `pub(crate)` nem depender da mecânica Rust do valor.

Critérios RED futuros:

- stroke simples deixa de produzir `cannot serialize stroke to JSON`;
- `cap`, `join`, `dash` e `miter-limit` aparecem separadamente quando definidos;
- defaults omitidos permanecem morfologicamente equivalentes ao vanilla;
- array de dash conserva ordem e phase negativa;
- `--pretty` continua sendo apenas formatação JSON;
- tipos não listados continuam a falhar em vez de cair genericamente em repr.

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

## P1285 — formatos estruturados, valores públicos e query genérica

### P1293 — projeção semântica dos slots de `MathAttach`

#### Medição anterior à decisão

O inventário de call sites encontrou um oitavo consumer produtivo além dos
sete owners centrais: `02_shell/src/cli.rs:792-805` lê `t,b,tl,bl,tr,br` ao
converter `MathAttach` para o objeto semântico de query. Com `Option`, esse
caminho não distingue `none` explícito de conteúdo vazio presente. A medição
causal P1293 SHA-256
`80a9543c9450f2350a42fa48df2e42cac63109bd074ebb37c2ec424cba473d5c`
fecha os três estados antes desta decisão; as chamadas de constructor depois
de `#[cfg(test)]` são testes mecânicos e não owners produtivos adicionais.

#### Decisão

Na serialização pública de `MathAttach`, `Omitted` não cria field;
`ExplicitNone` preserva a projeção histórica que o serializer aplicava a
`Some(Content::Empty)`; `Present(content)` cria field pelo serializer recursivo
vigente, inclusive o conteúdo vazio estrutural. A normalização pública de
`t`/`b` versus `tr`/`br` e sua precedência ficam exatamente como estão; apenas
a seleção tipada substitui a leitura mecânica de `Option`. O serializer não
infere estado pelo conteúdo e não usa `repr`, sentinel ou heurística.

Esta adaptação é consequência do contrato público MathAttachSlot confirmado
em `2026-09-02T08:06:37-03:00`; não altera argumentos CLI, formatos, defaults,
target, fase nem outros tipos de Content.

> **Estado do gate ADR-0127:** CONFIRMADO PELO DONO EM 2026-08-30. Esta secção
> substitui, somente para P1285, as restrições históricas acima que adiavam
> YAML, proibiam o fallback público por `repr` e limitavam query a headings.
> A confirmação autoriza a materialização dos contratos públicos abaixo.

### Medição anterior à decisão

Medição em 2026-08-30 contra binários imutavelmente identificados:

- vanilla ratificado `a51e02804`, `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- cristalino produzido após P1284, `target/release/typst`, SHA-256
  `8b85f933b7cd1fa74e46e2c18902b8343d9064f11b2a76844a476a252835a57e`.

Os cinco consumers L0 auditados para este passo estavam byte-a-byte em HEAD
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88` antes desta alteração de L0:
`02_shell/src/cli.rs`, `01_core/src/compiler/eval/mod.rs`,
`01_core/src/compiler/eval/repr.rs` e
`01_core/src/compiler/eval/selector_matching.rs`, além de
`03_infra/src/query_helpers.rs`. A restante working tree
continha materializações P1281–P1284, mas não alterava esses consumers.

Resultados públicos medidos:

```text
eval --help
vanilla    -> json, yaml, raw
cristalino -> json, raw

eval 'sys.version' --format json
vanilla    -> "version(0, 15, 1)"\n, exit 0
cristalino -> cannot serialize value to JSON, exit 1

eval '(a: 1, b: (2, 3), v: sys.version)' --format yaml
vanilla    -> mapping/sequence YAML e v: version(0, 15, 1), exit 0
cristalino -> yaml rejeitado pelo clap, exit 2

eval 'calc.inf' --format json
vanilla    -> null\n, exit 0
cristalino -> cannot serialize non-finite float to JSON, exit 1

query --help
vanilla    -> json, yaml
cristalino -> json

query de metadata rotulada, sem --field
vanilla    -> objeto {func: metadata, value: {...}, label: <meta>}, exit 0
cristalino -> query serialization currently supports headings only, exit 1

query de figure --field body
vanilla    -> content text estruturado, exit 0
cristalino -> query serialization currently supports headings only, exit 1
```

Fonte do vanilla que produz a medição:

- `lab/typst-original/crates/typst-cli/src/eval.rs:129-153` separa
  `json|yaml|raw`, mantendo raw restrito a string/bytes;
- `lab/typst-original/crates/typst-cli/src/main.rs:113-131` usa JSON com
  `pretty` opcional e YAML sem efeito de `pretty`;
- `lab/typst-original/crates/typst-library/src/foundations/value.rs:343-362`
  serializa `none`, bool, int, float, str, bytes, symbol, content, array e dict
  pela forma nativa; os outros valores viram string da sua `repr` pública;
- `lab/typst-original/crates/typst-library/src/foundations/content/mod.rs:709-718`
  serializa content como mapa `func` + campos públicos, sem caso especial de
  heading.

Classificação ADR-0107/0108: escolha de crate, enum Rust e árvore intermediária
são mecânica livre; formatos anunciados, estrutura semântica dos valores,
stdout, erro e exit code são observáveis públicos. A intenção é confirmada
pela implementação genérica e pelos tipos `Serialize`, não inferida apenas dos
bytes medidos. A conclusão seria refutada se um tipo público nominal do
vanilla falhasse na serialização genérica ou se YAML não constasse do help e
do dispatcher pinados.

### Decisão pública sujeita ao gate

`EvalFormat` passa a anunciar e aceitar as três variantes do vanilla:

```rust
pub enum EvalFormat { Json, Yaml, Raw }
```

Query passa a transportar explicitamente o formato estruturado:

```rust
pub enum QueryFormat { Json, Yaml }

pub struct QueryIntent {
    // campos existentes
    pub format: QueryFormat,
}
```

`serialize_eval` e `serialize_query` usam um único modelo semântico recursivo:

- `none`, bool, int, float finito, string, symbol, array e dict mantêm forma
  estruturada; dict preserva a ordem pública de inserção;
- float não-finito segue o formato: JSON produz `null`; YAML preserva
  infinito conforme o escalar YAML;
- bytes usam a forma serializada pública medida do tipo (`"bytes(N)"`), sem
  despejar os bytes nem usar `Debug`;
- `Content` é objeto `func` + campos públicos recursivamente serializados;
- os demais `Value` públicos são strings com `repr` morfológica de L1. Isso é
  o contrato genérico medido do vanilla, revogando a proibição histórica de
  fallback genérico; `Debug`, `PartialEq` e layout Rust continuam proibidos;
- JSON acrescenta um newline; YAML preserva um documento YAML válido e o
  comportamento observável de terminação do vanilla; `--pretty` só altera
  JSON e é neutro em YAML;
- raw continua byte-exato e restrito a string/bytes. O erro nomeia o tipo
  público real (`version`, `length`, etc.), nunca o fallback `value`.

Para query, o serializer deixa de ter uma branch exclusiva de heading. Todo
`Content` recuperado é convertido pelo mesmo caminho `func` + campos públicos;
um tipo ainda não representável falha nominalmente, sem `Debug`, mas a
existência de qualquer tipo diferente de heading não é por si só erro.
Quando L3 transporta um resultado rotulado como `Content::Label`, L2 serializa
o elemento interno (`func` + fields) e acrescenta `label: "<nome>"`; o wrapper
não aparece como `func: label` e não muda a cardinalidade da query.

Sem `--one`, `--field F` descarta elementos onde `F` não existe e serializa os
valores existentes. Com `--one`, primeiro exige exatamente um elemento e então
campo ausente falha com `no such field found for element`. O formato JSON/YAML
aplica-se depois dessa seleção. A warning de deprecação permanece em stderr.

### Aceitação RED→GREEN após confirmação

- helps de eval/query anunciam exatamente os formatos implementados;
- Version, valores nominais selecionados, conteúdo e compostos aninhados
  preservam a semântica JSON/YAML medida;
- JSON não-finito é `null`; YAML `--pretty` não muda o valor;
- query de metadata e figure deixa de encontrar o sentinela
  `supports headings only` e expõe `value`/`body` estruturados;
- `--field` e `--one` obedecem à ordem e aos erros acima;
- raw e os casos heading existentes não regridem.

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

## P1165 — `--features html` e default off (RASCUNHO; ADR-0127)

**Medição:** o help vanilla ratificado expõe `--features <FEATURES>` nos
argumentos comuns de compile/eval, valores `html`, `bundle`, `a11y-extras`.
`compile --format html` sem a flag erra exigindo `--features html`; com a flag
compila e avisa que HTML é experimental. O cristalino aceita HTML sem feature
e rejeita a flag como argumento inesperado.

Após aprovação, Compile e Eval aceitam `--features html` repetível segundo a
sintaxe medida e transportam uma coleção tipada no intent; default vazio.
Selecionar `.html`, `--format html` ou target HTML não ativa a feature. Compile
HTML sem feature reproduz diagnóstico/hints e exit 1. `info` não aceita a flag
como argumento próprio e reporta o estado/default efetivo (`false`). Bundle e
a11y-extras são nomes medidos, mas não são implementados por este corte.

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
é exclusivamente L3. As invariantes compartilhadas estão no Núcleo Tekt
`network/custom-ca-cert.toml` pinado acima.

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

## P1286 — aceitação compatível de `--pdf-standard`

### Medição anterior à decisão

O caso congelado de artifact background invoca `compile --pdf-standard 2.0`.
O cristalino ainda não possui política nem enum pública de standards PDF; a
emissão já reproduz o fragmento medido e rejeitar a opção impede observar a
semântica. Expor a opção no help também ativaria indevidamente o caso PDF/A-3,
cujo `/AFRelationship` continua fora do recorte normal.

### Decisão

`CompileArgs` aceita `--pdf-standard <STANDARD>` como argumento oculto de
compatibilidade e não o transporta para `CompileIntent`. O valor não altera a
versão, conformidade, defaults ou pipeline e não anuncia capacidade PDF/A.
Isto cobre somente a invocação medida de PDF 2.0; suporte real e exposição no
help exigem contrato próprio. O gate público de P1286 foi confirmado pelo
humano em 2026-08-30.

## P1288 — perfil CLI `a11y-extras` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

- `02_shell/src/cli.rs:77-91` mede `FeatureArg` restrito a `Html` e resolução
  tipada pelo path antigo `entities::html`.
- `02_shell/src/cli.rs:137-141` e `:310-322` medem `--features` em compile e
  eval, mas sem `value_delimiter = ','` e sem `A11yExtras`.
- A fonte vanilla pinada mede `ProcessArgs.features` com delimitador vírgula em
  `typst-cli/src/args.rs:432-443` e as três grafias em `:646-654`; P1288 só
  materializa `html` e `a11y-extras`, mantendo `bundle` fora.

### Decisão proposta

Compile e Eval aceitam `--features a11y-extras`, repetição da opção, lista
separada por vírgula e combinação com `html`. A ordem e repetição são
idempotentes e resultam no mesmo `Features` canônico. Ausência produz conjunto
vazio. `bundle` continua rejeitado/scope-out neste recorte, nunca convertido
em HTML nem em `a11y-extras`.

`CompileIntent.features` e `EvalIntent.features` usam
`entities::compiler_features::Features`; L2 somente parseia e transporta.
Selecionar `--format html`, PDF ou outro target não acrescenta feature.
Feature desconhecida termina como erro de argumentos e jamais é reclassificada
como perfil desligado. `info` continua sem receber `--features` próprio; a
extensão da sua estrutura para reportar `a11y-extras: false` pertence ao
consumer L4.

### Aceitação pós-confirmação

- helps de compile/eval anunciam `html` e `a11y-extras`, não `bundle`;
- `--features html,a11y-extras` e as duas ordens repetidas produzem o mesmo set;
- sem flag, ambos permanecem false;
- nome desconhecido falha no parser e não alcança L1/L3/L4 como valor parcial.

## P1293.reopen-C — seleção explícita de serialização HTML (PROPOSTO; STOP ADR-0127)

### Medição anterior à decisão

Em `2026-09-02T12:14:19-03:00`, o recibo residual independente P1293/C
SHA-256 `4545df3baa07d09c5c004a77002d18eeaedb47a22aa4883dc2bc3ba12323676a`
mediu que o caminho HTML vigente produz uma serialização conservadora válida,
enquanto a forma vanilla ratificada também é HTML válido e diverge apenas na
seleção contextual de escapes. Busca read-only no estado recebido não encontrou
`HtmlSerialization`, `html_serialization` nem `--html-serialization` em
`02_shell/src/cli.rs`; `CompileArgs` está em `cli.rs:144` e `CompileIntent` em
`:344`. A decisão do dono é preservar a forma cristalina como default e expor a
forma vanilla como alternativa explícita; portanto é incorreto chamar qualquer
uma das duas de “não padrão”.

### Contrato público proposto

Somente o subcomando `compile` aceita
`--html-serialization crystalline|vanilla`. L2 representa o dado cru com enum
público fechado, separado do enum L3:

```rust
pub enum HtmlSerialization { Crystalline, Vanilla }

pub struct CompileIntent {
    // campos existentes
    pub html_serialization: HtmlSerialization,
}
```

A ausência da flag resolve para `HtmlSerialization::Crystalline`. A flag é
aceita em `compile` mesmo quando o formato final não é HTML, mas só produz
efeito quando `output_format == OutputFormat::Html`; para PDF/PNG/SVG é neutra.
`watch`, `eval`, `query`, `info` e os demais comandos não aceitam esta flag.
Em particular, a reutilização interna de argumentos de compile por `watch` não
autoriza expor a opção nesse subcomando.

L2 apenas valida `crystalline|vanilla` via clap e transporta o enum. Não
conhece `HtmlSerializationMode` de L3, não serializa HTML e não ativa feature ou
target. Valor diferente termina como erro de argumentos; formato HTML continua
dependente de `Feature::Html`, e selecionar qualquer modo não habilita feature.

### Gate e aceitação

Esta proposta acrescenta flag, enum e campo públicos e preserva deliberadamente
um comportamento por defeito próprio. É categoria 1 e 2 do ADR-0127: nenhuma
implementação ou resselo de lineage é autorizado antes de confirmação humana
explícita. Após confirmação, REDs devem provar default `crystalline`, igualdade
entre default e flag explícita, seleção distinta de `vanilla`, rejeição de valor
alheio e ausência da flag nos outros comandos. Refutam este owner qualquer
necessidade de lógica de escaping em L2, efeito fora de HTML ou alteração do
eixo feature/target.
