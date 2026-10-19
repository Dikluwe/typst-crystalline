# Relatório — typst-passo-835: `image::pdf` — PDF como fonte (scope-out P781 confirmado) + parâmetro `page:` (achado #20)

**Data:** 2026-07-22
**Executor:** Kimi Code (agente principal, sessão interactiva com o dono — prompt lido de `00_nucleo/materialization/typst-passo-835.md`).
**Proveniência das medições:** commit HEAD `174b37d90` (P834) + alterações deste passo (`01_core/src/engine/stdlib/figure_image.rs`, `01_core/src/engine/stdlib/mod.rs`, `00_nucleo/diagnosticos/debt/DEBT.md`, `00_nucleo/prompts/engine/stdlib/figure_image.md`). Sondas corridas com os binários release do commit + rebuild pós-implementação, fixtures em `temp/p831/` (`p835_page*.typ`).
**Baseline da suíte (antes):** `cargo test -p typst-core` → **4524 passed; 0 failed** (medido em P833).

---

## Passo 1 — Scope-out de PDF-como-fonte (P781): confirmado, mas nunca formalizado

A decisão de P781 (manter o scope-out por peso de dependência — `hayro` ~15 crates transitivas, exportador hand-rolled sem Form XObject) está registada em `00_nucleo/diagnosticos/paridade-producao-p781.md`, no código (`figure_image.rs`, erro explícito) e no L0 `engine/stdlib/figure_image.md`. **Não reaberta.**

**Achado processual**: como SVG (P834/DEBT-67) e `pdf.attach` (P807/DEBT-66), este scope-out **nunca teve entrada própria em `00_nucleo/diagnosticos/debt/DEBT.md`** — terceiro caso do padrão descoberto em P807. Corrigido neste passo: criada a entrada **DEBT-68** (com a decisão de P781 mantida, a nota do `page:` e o critério de reabertura); changelog do inventário actualizado (total abertos: 8 → 9).

## Passo 2 — Sonda do parâmetro `page:`

No vanilla, `page: NonZeroUsize` (default 1) — *"This attribute only has an effect for PDF files"* (`lab/typst-original/crates/typst-library/src/visualize/image/mod.rs:158-161`); só é usado no ramo PDF (`mod.rs:302-317`). Medições (exit codes reais, fixture `temp/p831/raster1.png`):

| Documento | Vanilla | Cristalino (antes) |
|---|---|---|
| `#image("raster1.png", page: 2) x` | exit 0 (compila; ignorado) | `error: argumento nomeado inesperado em image(): 'page'` (exit 1) |
| `#image("raster1.png", page: 0) x` | exit 1 — `error: number must be positive` | idem (mensagem genérica) |
| `#image("raster1.png", page: "2") x` | exit 1 — `error: expected integer, found string` | idem (mensagem genérica) |

Conclusão da sonda: `page:` só afecta PDF, mas o **cast é validado em qualquer fonte** — mesmo com PDF em scope-out, o cristalino diverge em três casos observáveis (um de aceitação, dois de mensagem de erro).

## Passo 3 — Decisão (do dono) e implementação

**Decisão do dono (2026-07-22, nesta sessão): aceitar `page:` como no-op validado** — paridade nos três casos medidos, sem efeito até DEBT-68 reabrir (documentado no código, no L0 e na própria DEBT-68).

Implementação (`01_core/src/engine/stdlib/figure_image.rs`):
- `page` adicionado à whitelist de named args de `image()`.
- Validação do cast com as mensagens verbatim do vanilla: `Int > 0` → aceite (ignorado); `Int ≤ 0` → `number must be positive`; outro tipo → `expected integer, found {long_type_name}`.

Testes novos em `01_core/src/engine/stdlib/mod.rs` (3, todos vermelhos antes — `page` não existia na whitelist):
- `p835_image_page_positivo_aceito_sem_efeito` — `page: 2` com PNG → `Ok(Content::Image)`.
- `p835_image_page_zero_erro_vanilla` — `page: 0` → `number must be positive`.
- `p835_image_page_tipo_errado_erro_vanilla` — `page: "2"` → `expected integer, found string`.

L0 `engine/stdlib/figure_image.md` actualizado (assinatura + regra do `page:`); hash refeito (`d9cefa20`).

## Passo 4 — Validação

- `cargo test -p typst-core` → **4527 passed; 0 failed**; 2 ignored. Cálculo: 4524 (baseline) + 3 testes novos = 4527. ✔
- `crystalline-lint .` → exit 0, zero violations.
- Binários (release rebuildado; exit codes reais, `temp/p831/p835_page*.typ`):

```text
page2:   cris=0 van=0
page0:   cris=1 van=1  — cris: ...p835_page0.typ:1:6: error: number must be positive
pagestr: cris=1 van=1  — cris: ...p835_pagestr.typ:1:6: error: expected integer, found string
```

## Ficheiros alterados

`01_core/src/engine/stdlib/figure_image.rs`, `01_core/src/engine/stdlib/mod.rs` (3 testes), `00_nucleo/prompts/engine/stdlib/figure_image.md`, `00_nucleo/diagnosticos/debt/DEBT.md` (DEBT-68 + changelog), este relatório.
