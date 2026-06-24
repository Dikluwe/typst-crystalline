# P428 — Relatório de fecho (DEBT-59 + DEBT-60b)

> **Data:** 2026-06-23  
> **Executor:** assistente IA (Kimi Code CLI)  
> **Branch:** Tekt  
> **Foco:** Fechar 2 débitos técnicos acumulados — DEBT-59 (CLI `--full-error`) e DEBT-60b (supplement "Secção" no outline).

---

## Resumo executivo

Ambos os débitos foram fechados com alterações pontuais e direcionadas:

- **DEBT-59 (S → FECHADO):** a flag `--full-error` foi adicionada ao parser CLI,
  fiada de `RunIntent` (L2) até `EvalContext.full_error` (L1) pelo caminho interno
  de L3, sem alterar a assinatura pública `compile_to_pdf_bytes`.
- **DEBT-60b (XS-S → FECHADO):** o outline passou a usar um campo `number` puro,
  separado do `resolved_text` partilhado com as referências de corpo, removendo
  o supplement "Secção" dos itens de TOC.

**Saldo de débitos:** com o fecho de DEBT-59 e DEBT-60, o conjunto de DEBTs
abertos diminui. A contagem histórica em P407 também foi corrigida de `10` para
`8` (ver Secção 5).

---

## 1. DEBT-59 — CLI `--full-error`

### 1.1 Mudanças de código

| Ficheiro | Linha(s) | Descrição |
|----------|----------|-----------|
| `02_shell/src/cli.rs:98-99` | `Args` ganha `#[arg(long = "full-error", action = clap::ArgAction::SetTrue)] full_error: bool` | Declaração da flag CLI. |
| `02_shell/src/cli.rs:141` | `full_error: args.full_error` | Mapeamento de `Args` para `RunIntent`. |
| `04_wiring/src/main.rs:50` | `use typst_infra::pipeline::compile_to_pdf_bytes_full_error;` | Importa o caminho interno de L3. |
| `04_wiring/src/main.rs:56-58` | Destructuring inclui `full_error` | O campo deixa de ser ignorado pelo `..`. |
| `04_wiring/src/main.rs:88` | `compile_to_pdf_bytes_full_error(&world, &source, full_error)` | Propagação até L3. |
| `03_infra/src/pipeline.rs:25` | `use typst_core::rules::eval::eval_with_full_error;` | Importa o eval com flag. |
| `03_infra/src/pipeline.rs:38-42` | `eval_to_module_with_sink` chama `eval_to_module_with_sink_full_error(..., false)` | Preserva API pública. |
| `03_infra/src/pipeline.rs:50-69` | `fn eval_to_module_with_sink_full_error(..., full_error)` | Helper interno que invoca `eval_with_full_error`. |
| `03_infra/src/pipeline.rs:92` | `compile_to_pdf_bytes` chama `compile_to_pdf_bytes_full_error(..., false)` | Preserva API pública. |
| `03_infra/src/pipeline.rs:102-107` | `#[doc(hidden)] pub fn compile_to_pdf_bytes_full_error(..., full_error)` | Caminho interno exposto a L4 como detalhe de implementação. |

### 1.2 Decisão arquitetural

A assinatura pública `compile_to_pdf_bytes(world, source)` **não mudou**. A flag
entra por uma função irmã `compile_to_pdf_bytes_full_error`, marcada
`#[doc(hidden)]`, que existe apenas porque L4 vive num crate separado
(`typst-wiring`). Comportamento default continua `false`, garantindo paridade
vanilla off-by-default.

### 1.3 Verificação E2E

Ficheiro de teste (`/tmp/recursivo.typ`):

```typst
#show heading: it => { if it.body == [a] {[= b]} else {[= a]} }
= a
```

Sem a flag (2 hints):

```text
/tmp/recursivo.typ:<detached>: error: maximum show rule depth exceeded
  hint: maybe a show rule matches its own output
  hint: maybe there are too deeply nested elements
```

Com `--full-error` (3º hint classificado):

```text
/tmp/recursivo.typ:<detached>: error: maximum show rule depth exceeded
  hint: maybe a show rule matches its own output
  hint: maybe there are too deeply nested elements
  hint: erro completo: recursão CÍCLICA — uma forma de conteúdo repetiu-se no caminho de revisitação (a regra de #show reescreve para algo que reaparece)
```

Critério satisfeito: sem `--full-error` a mensagem é byte-idêntica à baseline de
2 hints; com a flag aparece o 3º hint.

---

## 2. DEBT-60b — outline sem supplement "Secção"

### 2.1 Mudanças de código

| Ficheiro | Linha(s) | Descrição |
|----------|----------|-----------|
| `01_core/src/entities/element_payload.rs:226-235` | `HeadingForToc { label, number: Option<String>, body, level }` | Novo campo `number` no payload. |
| `01_core/src/entities/introspector.rs:190` | `fn headings_for_toc(&self) -> &[(Label, Option<String>, Content, usize)]` | Assinatura do trait actualizada. |
| `01_core/src/entities/introspector.rs:298` | `pub headings_for_toc: Vec<(Label, Option<String>, Content, usize)>` | Sub-store actualizada. |
| `01_core/src/entities/introspector.rs:555` | Implementação do trait retorna o novo tuple | — |
| `01_core/src/rules/introspect/heading.rs:22-32` | `fn format_heading_number(...)` | Computa `"1."` / `"1.1."` a partir de `formatted_counter_at`, sem supplement. |
| `01_core/src/rules/introspect/heading.rs:66-76` | `compute_heading_for_toc(...)` | Retorna `(Label, Option<String>, Content, usize)`. |
| `01_core/src/rules/introspect.rs:608-614` | `populate_intr_from_tag_start` armazena `number` no sub-store | — |
| `01_core/src/rules/introspect.rs:815-825` | Walk arm Heading invoca `compute_heading_for_toc` com `intr`, `loc` e `numbering_active` | — |
| `01_core/src/rules/layout/outline.rs:35` | `Vec<(_, _, _, _)>` | Tuple com 4 elementos. |
| `01_core/src/rules/layout/outline.rs:40-61` | Renderização usa `number` puro; label mantém-se só para lookup de página | — |

### 2.2 Decisão arquitetural

O `resolved_text` usado pelas referências de corpo continua a incluir o
supplement (ex. "Secção 1"). O outline passa a ter o seu próprio `number`,
computado do counter e independente do supplement — paridade vanilla literal.
Headings sem numeração têm `number = None` e a linha do TOC começa
directamente pelo título.

### 2.3 Verificação

Teste `layout_outline_mostra_numero_sem_supplement_seccao` passa:

```text
test rules::layout::tests::layout_outline_mostra_numero_sem_supplement_seccao ... ok
```

Asserções do teste:

- `text.contains("Intro") && text.contains("Motiv")` — títulos listados.
- `!text.contains("Secção")` — nenhum supplement no TOC.
- `text.contains("1.") && text.contains("1.1.")` — números puros no TOC.

---

## 3. Testes

### 3.1 Comandos e resultados

```bash
RUST_MIN_STACK=8388608 cargo test --workspace
```

Resultado: **todos os testes passam**. O teste
`p350c_flag_on_nao_convergente_classifica` requer stack maior que o default em
devido à recursão artificial do cenário de teste; com `RUST_MIN_STACK=8388608`
passa.

Testes direccionados:

```bash
cargo test -p typst-core layout_outline
cargo test -p typst-core introspect_
cargo test -p typst-core headings_for_toc
```

Todos verdes.

### 3.2 Lint

```bash
crystalline-lint .
```

Resultado: zero violações estruturais. Apenas 2 warnings de "prompt órfão"
(`adr-stub-vs-fallback.md`, `show-regex.md`), preexistentes e fora do escopo do
P428.

---

## 4. Actualizações em `00_nucleo/diagnosticos/debt/DEBT.md`

- **DEBT-59** reclassificado como **✅ FECHADO (P428)**.
- **DEBT-60** reclassificado como **✅ FECHADO (P428)**, com subsecções:
  - **DEBT-60a** — contador de heading diverge: **aceite como divergência consciente**.
  - **DEBT-60b** — supplement "Secção" no outline: **fechado**.
- **P407** — corrigida a contagem de DEBTs abertos de `10` para `8`, com nota
  explicativa a referir que `10` era herança desactualizada da contagem
  anterior à auditoria P275.

---

## 5. Notas epistémicas / divergências declaradas

- **DEBT-59:** a função `compile_to_pdf_bytes_full_error` é pública por
  necessidade de linkagem inter-crate, mas está marcada `#[doc(hidden)]` e
  documentada como detalhe de implementação. A API estável continua a ser
  `compile_to_pdf_bytes`.
- **DEBT-60b:** o número do outline e o `resolved_text` das referências de
corpo são agora gerados pelo mesmo `formatted_counter_at`, mas em caminhos
separados. Isto evita regressões no corpo do heading enquanto corrige o TOC.
- **Stack de teste:** o teste `p350c_flag_on_nao_convergente_classifica` é
  intencionalmente recursivo até ao teto; recomenda-se correr os testes com
  `RUST_MIN_STACK=8388608` (ou equivalente CI) para evitar stack overflow em
  ambientes com stack default pequena.

---

## 6. Próximo passo

Com P428 fechado, os próximos candidatos prioritários são:

- **DEBT-63** — cache de style (`resolved_style`) dentro de `BibliographyElem`
  (S-M; Forma B já aplicada: excluído de `PartialEq`/`Hash`; falta decisão sobre
  mover para fora do struct).
- **DEBT-50** — show selector Strong/Emph não distingue origem (S; dívida
  latente).

---

## 7. Checklist de fecho

- [x] `typst --full-error` produz 3º hint em erro de recursão `#show`
- [x] Sem `--full-error`, mensagem de erro é byte-idêntica à baseline (2 hints)
- [x] `#outline()` não emite "Secção" para heading entries
- [x] Teste E2E `layout_outline_mostra_numero_sem_supplement_seccao` verde
- [x] DEBT-59 e DEBT-60b reclassificados como **FECHADO** em `DEBT.md`
- [x] `cargo test --workspace` verde (com `RUST_MIN_STACK=8388608`)
- [x] `crystalline-lint` zero violações estruturais
