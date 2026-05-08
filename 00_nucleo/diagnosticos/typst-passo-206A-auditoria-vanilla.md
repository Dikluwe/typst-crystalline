# P206A — Auditoria empírica vanilla integration (DEBT-53/54)

**Data**: 2026-05-07.
**Spec**: `00_nucleo/materialization/typst-passo-206A.md`.
**Output 1 de 4** (auditoria empírica).
**Etiquetas**: CONFIRMADO / DIVERGÊNCIA / N/A.

---

## §1 Bloco 1 — Estado actual do harness

### A1 — Estrutura `lab/parity/`

**CONFIRMADO**.

```
lab/parity/
├── Cargo.toml                        553B  (workspace-excluded; usa typst-syntax + typst-core paths)
├── src/                              (5 ficheiros)
│   ├── compact.rs                    — DTO compacto SyntaxNode (parse comparison)
│   ├── frame_dto.rs                  — FrameDTO para comparação estrutural P3
│   ├── main.rs                       54L   — bin parity-runner
│   ├── report.rs                     — ParityMatrix + render_markdown + escrita latest+history
│   └── value_dto.rs                  189L  — ValueDTO para P2 (eval comparison)
├── tests/                            (3 ficheiros)
│   ├── eval_parity.rs                — corpus_completo_p2 (semantic/)
│   ├── layout_parity.rs              169L  — corpus_completo_p3 (todas categorias)
│   └── parse_parity.rs               — corpus parse comparison
├── corpus/                           (36 .typ files; 25 .typ.toml companions)
│   ├── code/      (2)
│   ├── markup/    (7)
│   ├── math/      (2)
│   ├── semantic/  (10) com 10 companions
│   └── visual/    (15) com 15 companions
└── reports/                          — output histórico/latest da matriz
```

Cargo.toml declara binary `parity-runner` + 3 deps:
- `typst-syntax` path `../typst-original/crates/typst-syntax`
- `typst-core` path `../../01_core`
- (dev) `typst-infra` path `../../03_infra` (P150)

### A2 — Pre-existing breaks identificados em P204F

**CONFIRMADO empíricamente**. Exactamente 2 breaks; ambos
em testes (não em src ou bin).

`cargo check --manifest-path lab/parity/Cargo.toml --all-targets`:

1. **`tests/layout_parity.rs:69`** — `error[E0061]`:
   ```rust
   let doc = layout(content, state);  // 2 args
   //        ^^^^^^          ----- unexpected argument #2 of type `TagIntrospector`
   //        função actual: pub fn layout(content: &Content) -> PagedDocument
   ```
   P190I migrou para `layout(content)` (1 arg). Trivial:
   remover argumento + remover `let state = introspect(content)` line 68.

2. **`tests/eval_parity.rs`** indirectamente via
   `src/value_dto.rs:83` — `error[E0004]`:
   ```rust
   match v {
       // ... 18 arms cobrindo variants antigas
       // missing: &Value::Location(_) — added P179
   }
   ```
   Solução trivial: adicionar
   `Value::Location(_) => ValueDTO::Other("location".into())`
   ou mapear directamente.

**Empírico — sem outros breaks**:
- `cargo check --all-targets` reporta apenas estes 2.
- bin `parity-runner` compila (lib + bin compilam sem erro).
- tests compilam falhando pelos 2 erros listados.

### A3 — Driver `parity-runner`

**CONFIRMADO funcional**.

Smoke test:
```
$ cargo run --manifest-path lab/parity/Cargo.toml --bin parity-runner -- "Hello *world*"
✓ Paridade confirmada (13 bytes)
```

CLI semantics:
- arg 1: string inline ou path para ficheiro `.typ`.
- output stdout: `✓ Paridade confirmada (N bytes)` ou `✗ Divergência detectada` + `/tmp/parity_{expected,actual}.txt` + diff hint.
- exit code: 0 sucesso, 1 divergência.

Implementação `src/main.rs:54L`: invoca
`typst_syntax::parse(&input)` (vanilla) + `typst_core::
rules::parse::parse(&input)` (cristalino), normaliza
ambos via `compact_*` para `SyntaxNodeDTO`, compara.

**Driver é parse-only** (P1 paridade); não invoca eval
ou layout. Para P2/P3, harness vive em `tests/`.

### A4 — `layout_parity.rs` test harness

**CONFIRMADO** (com break A2#1 confirmado).

Estrutura `tests/layout_parity.rs` 169L:

- `compile_cristalino(src)` → `Option<PagedDocument>` —
  pipeline: `SystemWorld::new` + `eval_to_module_with_sink`
  + `module.content` + `introspect` + `layout`. **Linha
  69 quebrada** — mas migration trivial.
- `read_corpus(base)` — itera 4 categorias (markup,
  math, code, visual) lendo `.typ`. **Skip** ficheiros
  começando por "error" (parse-only fixtures).
  **Categoria semantic não está incluída** em
  `layout_parity.rs` — semantic é P2 eval, processada
  por `eval_parity.rs`.
- `corpus_completo_p3()` test — itera corpus,
  compila cristalino, constrói `FrameDTO::from_cristalino(&doc)`,
  acumula em `CategoryRow`. **Sem `assert!` global** —
  paridade é medição, não verificação. Falhas individuais
  são informação, não causam test failure.
- Output: matriz markdown via `report.rs` em
  `reports/{latest,history}/`. Colunas:
  `text_content / structural / geometric` ficam **N/A**
  até DEBT-53 fechar (vanilla DTO).

**Ponto de extensão vanilla**: linhas 132-135 onde
está marcado `Placeholder: vanilla DTO ainda não disponível`.
P206C activará `FrameDTO::from_vanilla(...)` ou
similar via vanilla CLI invocation.

---

## §2 Bloco 2 — Dependências vanilla

### A5 — Vanilla typst binário

**CONFIRMADO** (pre-built disponível).

```
$ which typst
/usr/local/bin/typst
$ typst --version
typst 0.14.2 (b33de9de)
```

Versão 0.14.2 corresponde a `lab/typst-original/crates/typst-syntax v0.14.2`
(declared em `lab/parity/Cargo.toml`). Paridade de versão
**confirmada**.

`lab/typst-original/Cargo.toml` (workspace-level)
**NÃO existe** — apenas crates individuais têm Cargo.toml.
Compilação vanilla "from source" exigiria criar workspace
ou compilar typst-cli explicitamente. **Mas como o
binário pre-built funciona**, este caminho é
desnecessário.

### A6 — Workspace setup vs quarentena

**CONFIRMADO**.

Cristalino `Cargo.toml` raíz:
```toml
[workspace]
members = ["01_core", "02_shell", "03_infra", "04_wiring",
           # lab/typst-original não é membro do workspace cristalino
           # compile separadamente se necessário para testes de paridade
]
```

Vanilla `lab/typst-original/`:
- `Cargo.toml` workspace-level **ausente**.
- `crates/typst-syntax/Cargo.toml` etc. existem
  individualmente.
- Vanilla crates compilam **só quando dependents path**
  os incluem (lab/parity → typst-syntax).

DEBT-54 ("vanilla workspace setup") originalmente
contemplava transformar lab/typst-original em workspace
ou criar harness completo. **Pre-built binário torna
isto irrelevante** — vanilla CLI install é pré-requisito
ambiental, não código a manter.

### A7 — PDF diff tools

**CONFIRMADO** (toolchain completa).

```
$ which compare convert pdftocairo pdfinfo gs
/usr/bin/compare              ImageMagick 6.9.12-98 — pixel-perfect compare
/usr/bin/convert              ImageMagick — PDF → PNG
/usr/bin/pdftocairo           poppler 24.02.0 — PDF → PNG/SVG
/usr/bin/pdfinfo              poppler — metadata
/usr/bin/gs                   ghostscript
```

Cristalino **não declara** dependência directa para
nenhuma. Tools são externas/ambientais.

Convenção de tolerância: **a definir empíricamente**.
Cristalino usa `FixedMetrics` enquanto vanilla usa
`FontBookMetrics` (real fonts) — divergência geométrica
estrutural per ADR-0054 perfil graded. **Pixel-perfect
é inviável por design**; fuzzy match exigiria escolha
arbitrária de tolerância.

### A8 — Comparação estrutural (queries)

**CONFIRMADO funcional** (smoke test).

Vanilla CLI:
```
$ typst query lab/parity/corpus/visual/query-metadata.typ "metadata" \
              --field value --format json
["primeiro",{"tag":"secundario","peso":42},"terceiro"]
```

Match exacto com `expectations.cristalino` em
`query-metadata.typ.toml`:
```
metadata_count = 3
metadata_first_kind = "string"
metadata_second_kind = "dict"
metadata_third_kind = "string"
```

`typst query` produz JSON estrutural directamente
comparável com cristalino `Introspector::query_metadata()`.
Cristalino test helper precisa ser construído (~30 LOC)
para extrair output análogo via `intr.query_metadata()`.

JSON diff via `serde_json::Value` ou ferramenta
`diff-json` é trivial — pattern padrão.

---

## §3 Bloco 3 — Corpus actual

### A9 — Listagem dos 36 ficheiros corpus

**CONFIRMADO**. Total **36 .typ files** distribuídos:

| Categoria | Quantos | Ficheiros |
|-----------|---------|-----------|
| code | 2 | let.typ, set.typ |
| markup | 7 | empty.typ, error.typ, heading.typ, parbreak.typ, plain.typ, spaces.typ, strong.typ |
| math | 2 | block.typ, simple.typ |
| semantic | 10 | array-literal, bool-true, closure-aplicada, condicional, dict-literal, float-divisao, funcao-builtin, int-aritmetica, string-concat, tipo-inspeccao |
| visual | 15 | cite-bibliography, counter-heading, equation-ref, figure-ref, heading-simples, math-basico, multi-font, outline-toc, paragrafo-justificado, query-metadata, set-text-bold, set-text-fill, set-text-size, set-text-tracking, show-strong |

**Adicionados em P204F** (commit d38f3e2fd "Passo 204a-f"):
6 ficheiros visual/ introspection:
- cite-bibliography.typ + (refs.yaml fixture)
- counter-heading.typ + companion
- equation-ref.typ + companion
- figure-ref.typ + companion
- outline-toc.typ + companion
- query-metadata.typ + companion

Pre-P204 corpus: 30 ficheiros (P155-P204G); P204F
elevou para 36.

### A10 — Companions `.typ.toml` actuais

**DIVERGÊNCIA** vs hipótese spec ("companions têm
features, modo_p3, notes; P204F adicionou
[expectations.cristalino]").

Distribuição empírica:

- **25 companions** existem (todos semantic/ + todos
  visual/).
- **11 ficheiros sem companion** (todos code/, markup/,
  math/).

Categoria sem companion = ficheiros simples sem
expectativas estruturais; essencialmente smoke tests
de compilação. A ausência de companion **não é break**
— harness skipa gracefully.

Formato companion (per `query-metadata.typ.toml`):
```toml
features    = ["metadata", "query", "introspection"]
modo_p3     = "structural"
notes       = "P204F opcional 1: ... per DEBT-53."
[expectations.cristalino]
metadata_count = 3
metadata_first_kind = "string"
...
```

P204F adicionou bloco `[expectations.cristalino]`
apenas em ficheiros introspection (5 dos 6 P204F).
Outros companions são pre-P204 com `features +
modo_p3 + notes` only.

**Gap para expectations vanilla**: precisa de bloco
`[expectations.vanilla]` paralelo (ou comparação directa
via `typst query` JSON output em runtime, dispensando
expectations explícitas).

### A11 — Features que cristalino ainda não suporta

**CONFIRMADO** (com nuance).

Hipótese spec mencionou `here()`/`locate()` como SKIP
P204F. Empírico: **NENHUM ficheiro do corpus actual
usa `here()` ou `locate()`** — `here-locate.typ`
referenciado na spec **não existe**.

Grep: `grep -l "here\|locate\|counter" lab/parity/corpus/`
não encontra usos críticos (apenas `counter-heading.typ`
que usa `#set heading(numbering)` — feature suportada
em cristalino).

Features potencialmente em risco:
- `cite-bibliography.typ` usa `bibliography("refs.yaml")` +
  `@ref-alfa` cite syntax. Cristalino implementa via
  P181 (BibStore + cite arm em layout/references). **Vanilla
  CLI compila** (smoke test passou). Cristalino **deve**
  compilar — verificar em P206B.
- `outline-toc.typ` usa `#outline()`. Cristalino
  implementa via P200B. Verificar.
- `set-text-fill.typ` usa cor RGB. Cristalino
  Color::Rgb suportado (P155).
- `multi-font.typ` usa `#set text(font: ...)` com font
  alternativos. Cristalino tem stub FontBook.

**Lista honesta de gaps**: zero categóricos
identificados. Possíveis edge-cases emergem em runtime
P206B compilação cristalino.

---

## §4 Bloco 4 — Caminhos viáveis

### A12 — Custo "Reactivar harness"

**Estimativa S–M (~1-2h)** com decomposição:

| Item | LOC | Tempo | Complexidade |
|------|-----|-------|--------------|
| Fix `layout_parity.rs:69` | 2L (1L modify + 1L delete) | 5 min | trivial |
| Fix `value_dto.rs:83` | 1L (Value::Location arm) | 5 min | trivial |
| Vanilla CLI invocation helper | ~30 LOC | 30 min | baixa (`std::process::Command`) |
| Comparação estrutural via typst query | ~50 LOC | 30 min | baixa (JSON diff) |
| Cobertura 36 ficheiros (matriz update) | ~20 LOC | 15 min | baixa (loop existente) |
| Tests sentinelas + smoke | ~30 LOC | 15 min | baixa |
| Documentação inline | — | 15 min | trivial |

**Total estimado**: ~1-2h. Magnitude **S-M**.

### A13 — Custo "Construir do zero"

**Estimativa M–L (~4-6h)** com decomposição:

| Item | LOC | Tempo | Complexidade |
|------|-----|-------|--------------|
| Design driver novo (CLI + entry-points) | ~80 LOC | 1h | média |
| FrameDTO redesign (clean slate) | ~150 LOC | 1.5h | média |
| ParityMatrix redesign | ~100 LOC | 1h | média |
| Vanilla CLI invocation | ~30 LOC | 30 min | baixa |
| Comparação estrutural | ~50 LOC | 30 min | baixa |
| Migration de 25 companions | ~50 LOC | 30 min | média |
| Tests + sentinelas | ~50 LOC | 30 min | baixa |

**Total estimado**: ~4-6h. Magnitude **M-L**.

Reuso do existente: zero (caminho B = clean slate).
Risco: re-introduzir bugs já fixados; perder convenção
estabelecida.

### A14 — Comparação A12 vs A13

| Critério | A — Reactivar | B — Construir |
|----------|----------------|----------------|
| Custo total | **S-M (~1-2h)** | M-L (~4-6h) |
| Pre-existing breaks | 2 (triviais) | 0 (não reusa) |
| Pre-existing infra reusable | **5 src + 3 tests** | 0 |
| Risco de regressão | baixo | médio (clean slate) |
| Qualidade de design | imperfeito mas funcional | potencialmente mais limpo |
| Time-to-value | rápido | lento |
| Compatibilidade com 36 corpus actual | imediata | exige migration |
| Compatibilidade com matriz histórica | preserva | reset |

**Caminho A é claramente mais barato e pragmático**.
Risco "subestimar A por preguiça" (per spec §9):
auditoria empírica mostra que pre-existing breaks são
literalmente 2 lines; reuso é alto; design existente
é funcional para P3 cristalino-only baseline (P150).

Caminho C (Híbrido) inflado sem benefício porque
Caminho A já é "fix breaks + estender" — é o que
"híbrido" significaria.

---

## §5 Bloco 5 — Estado pós-M8+F3 e contexto

### A15 — Condição 9 de ADR-0073 (referência)

**CONFIRMADO PARCIAL** pós-F3.

ADR-0073 §"Plano de validação" cond 9:
> **Saída cristalino sanity-check** vs vanilla nos 5-7
> ficheiros corpus paridade — sem regressões observable.

Estado actual (per `00_nucleo/adr/typst-adr-0073-comemo-introspector.md:365`):
```
| 9 | Sanity-check cristalino vs vanilla observable | ⚠️ PARCIAL (`P204F.div-1`) |
```

**Não mudou em F3**: ADR-0074 endereçou Layouter
sub-stores trackable (sealing post-iteração); F3 **não
toca cond 9** (que é sobre paridade observable). F3
expandiu infraestrutura (Position concrete via
SealedPositions) mas paridade observable continua
PARCIAL.

**Cumprir cond 9 transita ADR-0073 de "estruturalmente
fechado" para "completo final"?** Verificação literal
do ADR:
- `Status: ACEITE (estruturalmente fechado em P204H
  2026-05-07; condição 9 PARCIAL documentada — ver bloco
  abaixo)`. Linha 3-4.

A transição é de "estruturalmente fechado" para
"completo final". P206 endereçaria isso explicitamente
em P206E (encerramento série).

### A16 — Outras condições/expectativas pendentes

**Detectado**:

1. **DEBT-53** (vanilla integration bloqueada por
   DEBT-54). P206 endereça directamente.
2. **DEBT-54** (vanilla workspace setup). Hipótese
   inicial: P206 trata. Mas A5+A6 mostram que
   vanilla CLI pre-built torna workspace setup
   irrelevante. **DEBT-54 fecha por irrelevância** —
   não código a manter.
3. **`P204F.div-1`** (vanilla integration deferred). P206
   fecha.
4. **Condição 9 ADR-0073 PARCIAL**. P206 fecha.

**Pendências implícitas**:

- **Paridade observable desktop vs CI**: vanilla typst
  está em `/usr/local/bin/` no desktop. CI pode não
  ter — exigirá install step ou skip-if-no-vanilla
  decorator. Documentar em ADR-0075.
- **Versão vanilla pinning**: `lab/parity/Cargo.toml`
  declara `path = "../typst-original/crates/typst-syntax"`
  versão 0.14.2; binário CLI 0.14.2 confirmado.
  Coerência via assert em test helper (smoke test
  `typst --version` no início).

Sem outras pendências críticas pós-F3.

---

## §6 Resumo final da auditoria

| Bloco | Etiqueta | Sub-itens |
|-------|----------|-----------|
| 1 — Estado actual harness (A1-A4) | **CONFIRMADO** | 4/4 itens com evidência empírica |
| 2 — Dependências vanilla (A5-A8) | **CONFIRMADO** | 4/4; vanilla CLI pre-built funcional |
| 3 — Corpus actual (A9-A11) | **CONFIRMADO + 1 DIVERGÊNCIA** | A10 11 ficheiros sem companion (esperado); A11 zero gaps categóricos |
| 4 — Caminhos viáveis (A12-A14) | **CONFIRMADO** | A é S-M; B é M-L |
| 5 — Estado pós-M8+F3 (A15-A16) | **CONFIRMADO** | cond 9 PARCIAL; DEBT-54 obsoleto |

**Decisões habilitadas pela auditoria**:

- C1 = Caminho A (Reactivar): fundamentado em A12 vs
  A13 (custo 2× menor; reuso alto; risco baixo).
- C5 = Caminho b (Pre-built binário): A5 confirma
  binário disponível com versão certa.
- C3 = Mecanismo C (estrutural apenas): A7 mostra
  pixel-perfect inviável (FixedMetrics divergência).
- C4 = Mecanismo D (typst query JSON): A8 confirma
  funcional.

Sem `P206A.div-N` necessário — escopo 36 ficheiros é
viável com decisões fixas (reactivar + estrutural-only).
