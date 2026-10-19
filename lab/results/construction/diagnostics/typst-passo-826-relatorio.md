# Relatório — typst-passo-826: `pdf::accessibility` — `pdf.artifact(kind:)` rejeitado (achado #14 de P810)

**Data:** 2026-07-22
**Executor:** Kimi Code (subagente, a pedido do agente principal — prompt lido de `00_nucleo/materialization/typst-passo-826.md`; único ficheiro acedido em `materialization/`).
**Proveniência das medições:** commit HEAD `2acc14eac28468795c9d14c8a450fa5e320bf888`; working tree **não commitado** nas medições ANTES e DEPOIS (`git diff HEAD --stat` no início: 46 ficheiros, +3556/-396 — passos P823…P822; ao fechar, 2026-07-22T05:00Z, os ficheiros deste passo somam +220/-7 adicionais: `01_core/src/engine/stdlib/pdf.rs` +219/-4, `00_nucleo/prompts/engine/stdlib/pdf.md` +2/-2). Binário cristalino rebuildado (`cargo build --release`, 18.21s) após a implementação e antes da medição DEPOIS.
**Binários:** `./target/release/typst` (cristalino — `typst <input> <out.pdf>`), `lab/typst-original/target/release/typst` (vanilla 0.15.0 — `typst compile <input> <out.pdf>`). Fixtures em `temp/p826/`.

**Âmbito (do prompt):** só o argumento `kind:` de `pdf.artifact` (achado #14 de P810). O achado original (`00_nucleo/diagnosticos/typst-passo-810-relatorio.md` §14) registava: cristalino `argumento nomeado inesperado: 'kind'` / vanilla aceita (named arg do `ArtifactElem`), com a nota "o `kind` só afecta o tag tree (scope-out global do exportador)".

---

## Passo 1 — Sonda (ANTES, literal)

Comandos exactos (em `temp/p826/`):
`../../lab/typst-original/target/release/typst compile <caso>.typ <caso>.vanilla.pdf` e
`../../target/release/typst <caso>.typ <caso>.crist.pdf`.

### Caso base — `k-header.typ`: `#pdf.artifact(kind: "header")[Cabeçalho decorativo]`

```text
VANILLA (exit=0):            PDF gerado
CRISTALINO ANTES (exit=1):   error: argumento nomeado inesperado: 'kind'
```

### Valores válidos medidos no vanilla (12 — todos exit=0)

```text
header footer watermark page-number line-number redaction bates page
pagination-other layout background other
```

Fonte: `lab/typst-original/crates/typst-library/src/pdf/accessibility.rs:58-98` — enum `ArtifactKind` com derive `Cast` (kebab-case), campo `kind` com `#[default(ArtifactKind::Other)]` em `accessibility.rs:48-49`.

### Erros medidos no vanilla

```text
k-invalid.typ  kind: "banana"   (exit=1):
error: expected "header", "footer", "watermark", "page-number", "line-number",
       "redaction", "bates", "page", "pagination-other", "layout", "background", or "other"

kt.typ         kind: 42         (exit=1): idem + sufixo `, found integer`
kc.typ         kind: "Header"   (exit=1): mesma mensagem de domínio (case-sensitive)
kp.typ         pdf.artifact("header")[x]  (exit=1): error: unexpected argument   (kind é só named)
ke.typ         kind: "header", foo: 1     (exit=1): error: unexpected argument: foo

CRISTALINO ANTES: todos os casos com `kind:` → error: argumento nomeado inesperado: 'kind' (exit=1)
```

### Estrutura PDF de saída (o que o vanilla faz com o `kind`)

```text
vanilla out-v-header.pdf — stream de conteúdo (mutool clean -d + strings):
  /Artifact<</Attached[/Top]/Subtype/Header/Type/Pagination>>BDC
cristalino out-c-header.pdf — trailer/Root sem /StructTreeRoot nem /MarkInfo
  (grep artifact|tag.?tree em 03_infra/src: zero matches — exportador sem tagging)
```

Confirma a nota de P810: o `kind` só afecta marcação de artefacto (tag tree / marked content), que é **scope-out global do exportador cristalino** (registado no L0 `prompts/engine/stdlib/pdf.md`: "o exportador PDF cristalino não suporta embedding nem tagging"). Paridade de render confirmada: `pdftotext` de vanilla e cristalino sobre `k-header` idênticos (`Cabeçalho decorativo`).

### Pontos localizados (registados antes de tocar em código)

| Ponto | Vanilla | Cristalino (ANTES) |
|---|---|---|
| assinatura `kind:` | `accessibility.rs:48-49` (`pub kind: ArtifactKind`, default `Other`) | inexistente — `native_pdf_artifact` rejeitava qualquer named via `expect_no_named` (`01_core/src/engine/stdlib/pdf.rs:68`) |
| enumeração | `accessibility.rs:58-98` (`ArtifactKind`, 12 variantes, derive `Cast`) | — |
| convenção de erro de cast | string fora do domínio sem sufixo; outro tipo com `, found {tipo}` | já replicada em `loading.rs:477-483` (`encoding:`, P824) — mesma convenção aplicada aqui |
| propagação ao PDF | marked content `/Artifact<</Subtype/Header/Type/Pagination>>BDC` no stream | exportador sem tagging (scope-out global, `03_infra` sem matches) |

## Passo 2 — Implementação

Ficheiros alterados (`git diff HEAD --stat` destes ficheiros, 2026-07-22T05:00Z: 2 ficheiros, +220/-7 — inclui testes):

- **`01_core/src/engine/stdlib/pdf.rs`** —
  - `ARTIFACT_KINDS`: os 12 valores medidos;
  - `ARTIFACT_KIND_EXPECTED`: mensagem de cast verbatim do vanilla;
  - `vanilla_type_name()` (nomes `integer`/`boolean`/`string`, cf. `loading.rs:561`);
  - `native_pdf_artifact`: named `kind:` aceite e **validado** (string no domínio → passthrough; string fora do domínio → erro verbatim sem sufixo; outro tipo → `, found {tipo}`); outros named continuam rejeitados com a mensagem existente (`argumento nomeado inesperado: '{key}'`); `expect_no_named` deixou de ser chamado aqui (continua em uso em `native_pdf_attach`);
  - doc header do módulo actualizada; `@updated 2026-07-22`;
  - 6 testes novos `#[cfg(test)] mod tests_p826` (2 valores válidos, controlo sem `kind:`, erro verbatim, sufixo `found integer`, named desconhecido).
- **`00_nucleo/prompts/engine/stdlib/pdf.md`** (L0) — cláusula P826: enumeração medida, validação, e registo de que o valor não é propagado (tag tree = scope-out global). `crystalline-lint --fix-hashes .` corrido → hash de `pdf.rs` actualizado (`51709162`), 0 drift warnings.

O `kind` **não** foi propagado a `03_infra`: o exportador cristalino não emite marked content nem tag tree (scope-out global registado no L0 desde P735); emitir `/Artifact ... BDC` seria uma feature nova de tagging, fora da forma "aceitar e validar" deste passo e sem as outras peças (StructTreeRoot) que a tornassem honesta. Verificado o que o vanilla emite (secção acima) e registado.

Pureza L1 mantida: só `match`/`format!`/strings; sem I/O, relógio, env ou estado global.

## Passo 3 — Validação (DEPOIS, literal)

### Testes novos confirmados a falhar ANTES da implementação

`cargo test -p typst-core p826` → `1 passed; 5 failed; 4460 filtered out` — os 5 falhados são exactamente os testes de `kind:` novos (o controlo sem `kind:` já passava).

### DEPOIS — binário rebuildado, mesmos comandos da sonda

```text
k-header      CRISTALINO DEPOIS (exit=0): PDF gerado                          ✓ como o vanilla
k-footer      CRISTALINO DEPOIS (exit=0): PDF gerado                          ✓ como o vanilla
k-noarg       CRISTALINO DEPOIS (exit=0): PDF gerado (controlo sem kind:)     ✓ sem regressão
k-invalid     DEPOIS (exit=1): error: expected "header", "footer", "watermark", "page-number",
              "line-number", "redaction", "bates", "page", "pagination-other",
              "layout", "background", or "other"                              ✓ verbatim vanilla
kt (kind: 42) DEPOIS (exit=1): idem + `, found integer`                       ✓ verbatim vanilla
kc ("Header") DEPOIS (exit=1): mesma mensagem de domínio                      ✓ case-sensitive como o vanilla
ke (kind+foo) DEPOIS (exit=1): error: argumento nomeado inesperado: 'foo'     ✓ rejeitado (texto PT — convenção existente do cristalino; vanilla `unexpected argument: foo`, divergência de texto pré-existente da mesma família de P810 §14)
kp (posicional) DEPOIS (exit=1): error: pdf.artifact() requer 1 argumento (body), recebeu 2
              ✓ rejeitado (texto PT pré-existente; vanilla `unexpected argument`)
```

Os 12 valores válidos, um a um, no cristalino DEPOIS: **todos exit=0** (`header footer watermark page-number line-number redaction bates page pagination-other layout background other`).

### Controlos (sem regressão)

```text
pdf.artifact[Sem kind] (sem named):                    VANILLA exit=0 · CRISTALINO DEPOIS exit=0 ✓
pdf.table-summary (gate A11yExtras):                   VANILLA exit=1 "module `pdf` does not contain `table-summary`"
                                                       CRISTALINO DEPOIS exit=1 "module 'pdf' does not contain field \"table-summary\"" ✓
                                                       (paridade de comportamento mantida, cf. P810 §14)
render do body: pdftotext vanilla == cristalino ("Cabeçalho decorativo") ✓
```

### Estrutura PDF (Passo 3.3 do prompt)

Medido e registado: o vanilla emite `/Artifact<</Attached[/Top]/Subtype/Header/Type/Pagination>>BDC` no content stream para `kind: "header"`; o exportador cristalino não emite marked content/tag tree — **scope-out global documentado no L0**, não introduzido neste passo. O observável de render (texto/pixels) é paridade. A validação ficou ao nível da **assinatura/aceitação do argumento + erro de cast**, como autorizado pela tarefa.

### Suítes (comando + contagem ANTES/DEPOIS)

| Suíte | ANTES | DEPOIS |
|---|---|---|
| `cargo test -p typst-core` | 4458 passed; 0 failed; 2 ignored | **4464 passed; 0 failed; 2 ignored** |
| `cargo test -p typst-infra` | 667 passed; 0 failed; 5 ignored | **667 passed; 0 failed; 5 ignored** (inalterado) |

4458 + 6 testes novos (`tests_p826`) = 4464 ✓ — a contagem bate com os testes declarados.

### Lint

`~/.cargo/bin/crystalline-lint .` → 6 warnings, todos **V7 "prompt órfão"** pré-existentes da working tree (`eval/field-access.md`, `layout/enum_item.md`, `model/document.md`, `stdlib/layout.md`, `stdlib/structural.md`, `infra/package_version_resolution.md`) — o mesmo conjunto reportado em P822, nenhum em ficheiro tocado por este passo, nenhum V3/V4/V5/V13/V14. `--fix-hashes` corrido após editar o L0 (hash `51709162`). **Zero violations novas.**

## Scope-outs e débitos (confirmados, não tentados)

- **Tag tree / marked content no exportador** (`03_infra`): o `kind` é validado mas não propagado — o cristalino não emite `/StructTreeRoot`, `/MarkInfo` nem `/Artifact ... BDC` (medido: vanilla emite, cristalino não). Scope-out global registado no L0 `pdf.md` desde P735; emitir tagging parcial seria desonesto sem a árvore completa. Candidato a passo próprio (grande) de acessibilidade/tagged PDF.
- **Texto das mensagens de named desconhecido / arg posicional extra**: cristalino em PT (`argumento nomeado inesperado: 'foo'`, `pdf.artifact() requer 1 argumento (body), recebeu 2`) vs vanilla (`unexpected argument: foo`, `unexpected argument`). Divergência de texto pré-existente (convenção `expect_no_named` usada em todo o stdlib), mantida; candidata a passo futuro de mensagens se o dono quiser verbatim.
- **Spans `<detached>`**: os erros novos de `kind:` usam `Span::detached()` como os restantes erros do módulo `pdf` (o vanilla anexa span do argumento). Registado como apresentação.
- **Propagação do `kind` ao valor de retorno**: o cristalino devolve o `body` directamente (passthrough), sem wrapper `ArtifactElem` — o valor devolvido é indistinguível do body; qualquer introspecção de `ArtifactElem` em content (ex.: `content.artifact(...)`) não existe no cristalino. Sem observável linguístico no binário de referência para além do tagging (scope-out acima).
