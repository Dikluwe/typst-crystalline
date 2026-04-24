# Passo 115 — Migração de argparsing para `clap` (flags em CLI)

**Série**: 115 (passo médio; toca L4 e eventualmente testes L4).
**Precondição**: Passo 114 encerrado (CLI tests automatizados);
811 L1 + 195 L3 + 5 L4 + 6 ignorados; zero violations.
**ADRs aplicáveis**: ADR-0046 (CLI mínima).
**ADR nova**: ADR-00NN "CLI argparsing com clap — flags
básicas" — `PROPOSTO` em 115.B, `EM VIGOR` em 115.E.

---

## Objectivo

Substituir o argparsing manual (`std::env::args` em 113) por
`clap` com derive macros. Ao fim do passo:

1. `clap` adicionado ao workspace e a `04_wiring`.
2. Estrutura `Args` com derive `Parser`.
3. `typst --help` mostra ajuda automática.
4. `typst --version` mostra versão do crate.
5. Flags concretas: decididas em 115.A (mínimo helpdversion vs
   `-o/--output` vs subset).
6. Compatibilidade com testes 114: decidida em 115.A.

Este passo **não**:
- Adiciona `--root`, `--font-path` excepto se 115.A decidir pelo
  "subset útil".
- Implementa subcomandos (`compile`, `watch`, `query`, etc.). CLI
  continua sem subcomando — invocação é `typst [flags] input ...`.
- Toca L1 ou L3.
- Muda pipeline (eval, layout, export_pdf).

---

## Decisões já tomadas

1. **Migrar para clap** — decisão tomada pelo próprio passo.
2. **Versão e features**: clap 4.x com `derive`. Versão exacta
   alinhada com vanilla se possível (confirmar em 115.A).
3. **`--help` e `--version` automáticos**: sempre incluídos
   (derive activa-os por default).

## Decisões diferidas (115.A)

4. **Escopo de flags funcionais**:
   - **(a)** Mínimo — help/version grátis, positional mantido
     (`input.typ output.pdf`).
   - **(b)** Mínimo + `-o/--output` — output passa a flag;
     default derivado (`input.typ` → `input.pdf`).
   - **(c)** Subset útil — adiciona `--root`, `--font-path`.
     Maior, requer decisões semânticas por flag.
5. **Compatibilidade com testes 114**:
   - **(A)** Manter positional — testes 114 inalterados.
   - **(B)** Dupla aceitação (positional **ou** flag) — testes
     114 passam sem mudança; utilizadores podem usar qualquer.
   - **(C)** Só flag — actualizar testes 114.

Matriz de combinações plausíveis:

| Escopo | Compat | Nota |
|--------|--------|------|
| (a) | (A) | Migração pura; testes intactos. |
| (b) | (A) | Conflito — `-o` existe mas positional também, ambiguous no default |
| (b) | (B) | Dupla aceitação para `output`. Complexidade extra no parsing. |
| (b) | (C) | UX mais limpa; testes 114 migram para `-o`. |
| (c) | (A) | Escopo maior + positional mantido. |
| (c) | (C) | Escopo maior + testes migram. |

A escolha determina tamanho do passo e impacto em testes.

---

## Sub-passos

### 115.A — Inventário e decisões

**Parte 1 — Versão de clap no vanilla**:

1. `grep` por `clap` em `lab/typst-original/Cargo.toml` e
   `lab/typst-original/crates/typst-cli/Cargo.toml` (ou
   equivalente).
2. Registar versão e features usadas. Alinhar se razoável;
   divergir só com razão.

**Parte 2 — Idioma clap predominante**:

1. `view` em `lab/typst-original/crates/typst-cli/src/args.rs`
   (ou equivalente). Focar em:
   - Estrutura `Args` — usa `#[derive(Parser)]` ou manual?
   - `-o/--output` — existe? Como é declarado?
   - Positional — está lá ou tudo é flag?
   - Default values — quais têm?
2. Registar o padrão observado (não transcrever código
   completo).

**Parte 3 — Decisões**:

Com base nas partes 1-2, escolher:

- **Escopo** (a/b/c).
- **Compatibilidade** (A/B/C).

Critérios:
- **Preferir (a)+(A)** se o objectivo é migração pura sem
  mudança de UX. Testes 114 intactos.
- **Preferir (b)+(C)** se o vanilla usa `-o` como padrão e o
  projecto quer alinhar. Testes 114 migram — impacto ≤ 5
  linhas.
- **Evitar (c)** — cada flag nova é passo dedicado. `--root`
  e `--font-path` exigem semântica própria (como resolvem,
  como afectam SystemWorld).
- **Evitar (B)** (dupla aceitação) — complexidade sem valor
  obvio; clap não o faz limpo sem `Option<PathBuf>` positional,
  o que é confuso para utilizadores.

Documentar decisão em
`00_nucleo/diagnosticos/decisoes-clap-passo-115.md`.

**Gate 115.A**: se o vanilla tem `-o` obrigatório (sem positional
fallback) e o utilizador do projecto espera compatibilidade com
vanilla, escolher (b)+(C). Se o vanilla aceita positional,
(a)+(A) é alinhado.

### 115.B — ADR nova

Criar `00_nucleo/adr/typst-adr-00NN-cli-clap.md` com `PROPOSTO`.

Conteúdo:

- **Contexto**: CLI nasceu com argparsing manual (Passo 113).
  Passo 114 confirmou regressões via testes. Crescer CLI
  (futuras flags, cores, subcomandos) com argparsing manual
  torna-se rapidamente inviável. Clap é o standard Rust.
- **Decisão**:
  - `clap` 4.x com `derive` adicionado a `[workspace.dependencies]`
    e a `04_wiring/Cargo.toml`.
  - Estrutura `Args` com `#[derive(Parser)]` em `main.rs`.
  - `--help` e `--version` automáticos.
  - Escopo de flags funcionais: [escolha de 115.A].
  - Compatibilidade com testes 114: [escolha de 115.A].
- **Alternativas rejeitadas**:
  - **Manter manual**: não escala para subcomandos.
  - **`argh` / `pico-args`**: mais leves mas menos idiomáticas.
    Standard Rust é clap.
- **Limitações documentadas**:
  - Sem subcomandos neste passo.
  - Sem `--color`, `--root`, `--font-path` neste passo (se
    escopo (a) ou (b)).

Promover a `EM VIGOR` em 115.E.

### 115.C — Implementação

**115.C.1 — Dependências**:

Adicionar a `Cargo.toml` raiz:

```toml
[workspace.dependencies]
clap = { version = "4.x.y", features = ["derive"] }
```

Versão exacta de 115.A.1.

Adicionar a `04_wiring/Cargo.toml`:

```toml
[dependencies]
clap = { workspace = true }
```

**115.C.2 — Estrutura `Args`**:

Em `04_wiring/src/main.rs`. Esboço para **escopo (a) + compat (A)**:

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "typst", version, about = "Typst compiler (crystalline)")]
struct Args {
    /// Input .typ file
    input: PathBuf,
    /// Output PDF file
    output: PathBuf,
}
```

Esboço para **escopo (b) + compat (C)**:

```rust
#[derive(Parser, Debug)]
#[command(name = "typst", version, about = "Typst compiler (crystalline)")]
struct Args {
    /// Input .typ file
    input: PathBuf,
    /// Output PDF file; defaults to input with .pdf extension
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,
}
```

Se `output` é `Option`, calcular default:

```rust
let output = args.output.unwrap_or_else(|| args.input.with_extension("pdf"));
```

**115.C.3 — Substituir `parse_args` manual**:

Remover:

```rust
fn parse_args(args: &[String]) -> Option<(PathBuf, PathBuf)> { ... }
```

Substituir `main()`:

```rust
fn main() -> ExitCode {
    let args = Args::parse();
    // ... resto do pipeline, usando args.input e args.output
}
```

Clap faz o parsing; em erro de args, imprime mensagem em stderr
e faz `exit(2)` **automaticamente**. Não é preciso handling
manual.

**115.C.4 — Versão do crate para `--version`**:

Clap derive expõe `--version` se `version` é passado ao
`#[command(...)]`. A versão vem de `Cargo.toml`
(`version.workspace = true`). Confirmar:

```bash
cargo run --release -- --version
```

Output esperado: `typst <versão do Cargo.toml>`.

**115.C.5 — Actualizar testes 114 (se compat = C)**:

Se a decisão foi `(C) Só flag`, os 5 testes de 114 migram:

```rust
// Antes
.arg(&input)
.arg(&output)

// Depois (com -o)
.arg(&input)
.arg("-o")
.arg(&output)
```

E o teste `cli_sem_argumentos` pode ter assertion sobre stderr
ligeiramente diferente (clap usa formato próprio). Ajustar:

```rust
// Antes
assert!(stderr.contains("Usage"));

// Depois (clap usa "Usage:" também, mas o resto muda)
assert!(stderr.contains("Usage:") || stderr.contains("error:"));
```

Se compat = A, testes 114 ficam intactos.

### 115.D — Validação manual

1. `cargo build --release`.
2. Testes `--help` e `--version`:
   ```bash
   $ typst --help
   Typst compiler (crystalline)
   
   Usage: typst <INPUT> <OUTPUT>
   
   Arguments:
     <INPUT>   Input .typ file
     <OUTPUT>  Output PDF file
   
   Options:
     -h, --help     Print help
     -V, --version  Print version
   
   $ typst --version
   typst 0.x.y
   ```
3. Fluxo normal (como em 113.D):
   ```bash
   $ typst /tmp/test.typ /tmp/test.pdf
   /tmp/test.typ:5:11: warning: text: propriedade 'font' ...
   ```
4. Erro de args:
   ```bash
   $ typst
   error: the following required arguments were not provided:
     <INPUT>
     <OUTPUT>
   Usage: typst <INPUT> <OUTPUT>
   $ echo $?
   2
   ```

Se escopo (b), adicionar:

```bash
$ typst /tmp/test.typ     # output default
$ ls /tmp/test.pdf        # existe
```

### 115.E — Encerramento

1. `cargo build --release` passa.
2. `cargo test --workspace`: ≥ linha de base + L4 inalterado ou
   com testes 114 migrados.
3. `crystalline-lint` zero violations.
4. Validação manual 115.D passa.
5. ADR promovida.
6. Relatório `typst-passo-115-relatorio.md`:
   - Decisão de escopo e compat (com razão).
   - Versão de clap escolhida.
   - Output `--help` literal.
   - Output `--version` literal.
   - Se testes 114 foram migrados, diff.
   - Limitações aceites.

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 115.A escrito.
2. ADR-00NN criada e promovida.
3. `clap` no workspace e em `04_wiring`.
4. `Args` com derive Parser.
5. `typst --help` e `typst --version` funcionam.
6. Pipeline de compilação inalterado em comportamento observável.
7. Testes 114 passam (inalterados ou migrados conforme 115.A).
8. `cargo test --workspace` passa.
9. `crystalline-lint` zero violations.
10. Relatório 115.E escrito.

---

## O que pode sair errado

- **Versão de clap não alinha com vanilla**. Vanilla pode ter
  clap 3.x ou 4.x específico. Se for 3.x e quisermos 4.x, há
  diferença de idioma (attribute-based vs derive-based).
  Preferência: clap 4.x mesmo se vanilla usa 3.x — 4.x é
  actual.
- **`workspace = true` em Cargo.toml depende de `[workspace.dependencies]`**.
  Se o workspace não tem essa secção, adicionar primeiro. A
  análise 112 registou que `[workspace.dependencies]` existe.
- **Clap exit code default é 2**. Alinha com `cli_sem_argumentos`
  (exit 2 esperado), mas se clap emitir mensagem diferente, o
  assert `stderr.contains("Usage")` pode falhar. Verificar no
  115.D.
- **Decisão (c) "subset útil" pode inchar**. Se 115.A escolher
  adicionar `--root` e `--font-path`, cada um requer integrar
  com `SystemWorld::new(root, main)`. **Se propagação ≥ 2
  lugares**, decisão (c) fora do escopo — reverter para (a) ou
  (b).
- **`cargo test -p typst-wiring` a meio da migração falha**.
  Migrar `main.rs` quebra compilação dos testes se forem
  chamadas a funções que não existem. Strategy: actualizar em
  passos pequenos, `cargo check` a cada passo.
- **`--version` devolve output inesperado**. Se `Cargo.toml` tem
  `version.workspace = true` mas workspace não tem `version`,
  clap pode dar erro ou versão vazia. Verificar em 115.D.
- **`Option<PathBuf>` positional em clap**. Se escolha for (b)+(A)
  (dupla), tentar `output: Option<PathBuf>` como positional
  não compila bem com `input: PathBuf` obrigatório. Clap complica.
  Confirmar em 115.C.2 se esta combinação for tentada. Recomendação:
  evitar dupla.

---

## Notas operacionais

- Este é o primeiro uso de clap no projecto. Decisões aqui
  (escopo, compat) estabelecem padrão para futuros passos que
  adicionem flags/subcomandos.
- Se escolha for (C), teste `cli_sem_argumentos` pode ter
  assertion mais frágil porque clap muda output mais frequentemente
  entre versões. Ver se assertion sobre exit code (2) e presença
  de "error" ou "Usage" chega.
- Clap 4.x derive tem boa documentação — `#[arg(...)]`,
  `#[command(...)]`. Reusar convenções padrão onde possível.
- Se 115.A decidir (c), considerar dividir em 2 passos:
  115 faz só migração (a/b), 116 adiciona flags funcionais.
- Removendo `parse_args`, verificar que **nada mais o usa** em
  `main.rs`. Grep antes de remover.
- Versão do crate visível em `--version` deve ser consistente
  com versão no `Cargo.toml`. Se o projecto tem `version = "0.1.0"`
  literal em vez de workspace inherit, clap ainda funciona —
  muda apenas a fonte.
