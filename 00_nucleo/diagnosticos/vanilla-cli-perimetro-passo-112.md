# Passo 112.C — Perímetro do vanilla CLI

**Data**: 2026-04-23
**Fonte**: `lab/typst-original/crates/typst-cli/src/`
**Propósito**: ter visão do perímetro para decidir o que é mínimo
essencial e o que é adiável.

**Nota**: não é paridade — é mapa. Uma linha por subcomando,
sem transcrever código.

---

## Estrutura do vanilla CLI

18 ficheiros, ~4092 linhas total:

| Ficheiro | Linhas | Propósito |
|----------|-------:|-----------|
| `args.rs` | 823 | Definições clap de todos os subcomandos/flags |
| `compile.rs` | 739 | Orquestração do comando `compile` |
| `info.rs` | 614 | Info de build/ambiente/versões |
| `world.rs` | 383 | Wrapper de mundo para CLI (inclui caching) |
| `update.rs` | 263 | Self-update |
| `deps.rs` | 178 | Gerir dependências de pacotes |
| `query.rs` | 156 | Extrair metadata do documento |
| `watch.rs` | 163 | File watching |
| `main.rs` | 152 | Dispatcher principal |
| `init.rs` | 126 | Criar projecto de template |
| `eval.rs` | 98 | Avaliar snippet de código |
| `terminal.rs` | 93 | Helpers de stdout/stderr coloridos |
| `timings.rs` | 90 | Profiling temporal |
| `greet.rs` | 67 | Mensagem de boas-vindas |
| `download.rs` | 63 | Download de recursos |
| `fonts.rs` | 53 | Listar fontes disponíveis |
| `packages.rs` | 18 | Gerir pacotes |
| `completions.rs` | 13 | Completions de shell |

### Dependências externas visíveis

- **`clap`** com derive + `clap_complete` (completions).
- **`codespan-reporting`** para diagnostics coloridos.
- **`ecow`** (EcoString).
- **`serde` + `serde_json` + `serde_yaml`** (query output).
- **`semver`** (versões).
- **`color-print`** (help templates coloridos).
- **`sigpipe`** (handle SIGPIPE).
- **`notify`** (watch).
- **`typst`** crate (L1 equivalente).

---

## Subcomandos (`Command` enum)

```rust
pub enum Command {
    Compile(CompileCommand),       // alias "c"
    Watch(WatchCommand),           // alias "w"
    Init(InitCommand),
    Query(QueryCommand),
    Eval(EvalCommand),
    Fonts(FontsCommand),
    Update(UpdateCommand),         // [cfg(feature = "self-update")]
    Completions(CompletionsCommand),
    Info(InfoCommand),
}
```

### Uma linha por subcomando

| Subcomando | Propósito (1 linha) |
|------------|---------------------|
| `compile` | Compila input Typst para PDF/PNG/SVG/HTML. |
| `watch` | Vigia o ficheiro e recompila em mudanças. |
| `init` | Cria projecto novo a partir de template `@preview/xxx`. |
| `query` | Extrai metadata do documento via selector (JSON/YAML). |
| `eval` | Avalia snippet de código Typst (moderno; substitui query). |
| `fonts` | Lista fontes em paths de sistema + custom. |
| `update` | Self-update via GitHub releases (feature-gated). |
| `completions` | Gera scripts de completion para shell. |
| `info` | Informação de debug (versão, build flags, paths). |

---

## Flags principais de `compile`

Agrupadas via `#[clap(flatten)]`:

### `CompileArgs` (específicas a compile)
- **`input: Input`** (positional) — path ou `-` para stdin.
- **`output: Option<Output>`** (positional) — path ou `-` para
  stdout.
- **`--format / -f`** — `pdf | png | svg | html`.
- Flags de PDF (compliance, timestamps).
- Flags de PNG (`--ppi`).
- `--open` — abrir output no viewer default.

### `WorldArgs` (partilhada com `watch`, `query`, `eval`)
- **`--root / TYPST_ROOT`** — directório raiz do projecto.
- **`--input key=value`** — injectar em `sys.inputs`.
- **`FontArgs`** nested:
  - **`--font-path / TYPST_FONT_PATHS`** — paths adicionais.
  - `--ignore-system-fonts`.
- **`PackageArgs`** nested (package-path, package-cache-path).
- `--creation-timestamp / SOURCE_DATE_EPOCH`.

### `ProcessArgs` (partilhada)
- `--jobs / -j` — paralelismo.
- `--features` — features em desenvolvimento.
- `--diagnostic-format` — `human | short | json` (vanilla).

---

## Comportamento default (sem args)

`main.rs:42-49`:

```rust
static ARGS: LazyLock<CliArguments> = LazyLock::new(|| {
    CliArguments::try_parse().unwrap_or_else(|error| {
        if error.kind() == ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand {
            crate::greet::greet();
        }
        error.exit();
    })
});
```

Sem args → **chama `greet::greet()`** (imprime banner + help) e
sai com erro de clap.

---

## Minimum viable product (MVP) identificado

Partindo do vanilla, o subconjunto **mínimo** que dá valor real:

1. **`compile` com positional `input output`**.
2. **`--root`** para paths absolutos.
3. **`--font-path`** para fontes custom.
4. **Diagnostics em stderr** (warnings + errors).
5. Output só PDF (adiar PNG/SVG/HTML).

Tudo isto está coberto pelo perímetro vanilla — sem inventar nada.

---

## O que fica visivelmente por fora no MVP

- `watch` — requer `notify`.
- `init` — requer rede (`@preview/xxx`) ou templates locais.
- `query` — requer serde + selectors.
- `eval` — snippet evaluation.
- `fonts` — enumeração (útil, mas não crítico para compile).
- `update` — self-update via GitHub.
- `completions` — gera bash/zsh/fish.
- `info` — debug info.
- PNG/SVG/HTML output — cristalino só tem PDF por agora.
- `sys.inputs` — mecanismo do cristalino não tem (seria extensão
  de `eval()` e da lib).

---

## Conclusões 112.C

1. **Perímetro vanilla é largo** (4092 linhas) mas assimétrico: 3
   comandos grandes (compile, info, update) consomem ~1600 linhas;
   os restantes ~2500 são args + world + dispatchers.
2. **MVP focado em `compile`** é o pattern natural. Outros
   subcomandos vêm em passos dedicados se/quando houver procura.
3. **clap é a dep natural** se o cristalino decidir argparsing
   declarativo — vanilla usa extensivamente.
4. **Diagnostics com cores** são feature cosmética (codespan-reporting).
   O Passo 111 cobre formato; cores ficam para passo dedicado.
5. **`compile` tem ~15 flags** quando contadas as nested (PDF,
   PNG, World, Process). Um MVP pode ter 3-5 (input, output, root,
   font-path, format).
