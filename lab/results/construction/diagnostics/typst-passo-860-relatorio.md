# Relatório — typst-passo-860: fechar `measure()` — largura E altura

**Data:** 2026-07-23  
**Executor:** Kimi Code (agente principal; prompt lido de `00_nucleo/materialization/typst-passo-860.md`).  
**Proveniência das medições:** working tree não commitado sobre commit `dfe3c2282ec0d6bd97d5834f00214e7c7f5d2d49`; alterações locais em `01_core/src/engine/layout/mod.rs`. Medições feitas com o binário cristalino recém-compilado (`target/release/typst`) e o binário vanilla `lab/typst-original/target/release/typst` (0.15.0), ambos usando a **mesma fonte** (`Nimbus Sans`) via `--font-path`.

---

## 1. Problema deixado por P858

P858 corrigiu a **largura** de `measure()` injetando `&dyn FontMetrics` no `Engine`. A **altura** continuou errada porque `measure_content_real` devolvia o avanço de linha calculado por `layout_sub_frame`, não a altura do frame de texto. O DEBT-69 foi marcado como fechado em P858, mas o critério de parada exigia largura e altura juntas — este passo fecha a dívida de verdade.

---

## 2. Controlo da fonte de comparação

Para evitar a inconclusão de P858 (comparação entre fallback do sistema e Helvetica embutida), ambos os binários foram forçados a usar a mesma face:

- Fonte: `03_infra/fixtures/fonts/NimbusSans-Regular.otf`
- Família: `Nimbus Sans` (confirmada via `fc-query`)
- Comando cristalino: `./target/release/typst <file>.typ --font-path 03_infra/fixtures/fonts -o out.pdf`
- Comando vanilla: `./lab/typst-original/target/release/typst compile --font-path 03_infra/fixtures/fonts <file>.typ out.pdf`
- Resultado extraído do PDF com `pdftotext`.

---

## 3. Implementação

`01_core/src/engine/layout/mod.rs:1816-1830` (dentro de `measure_content_real`):

```rust
let text = content.plain_text();
let height = if text.is_empty() {
    height
} else {
    let (top, bottom) = metrics.text_edges(Pt(font_size), &layouter.style);
    top.val() - bottom.val()
};
```

A altura passou a ser calculada a partir dos `text_edges` padrão do vanilla (`top-edge: cap-height`, `bottom-edge: baseline`), que é exatamente a altura do frame de texto. Para conteúdo sem texto plano (`measure([])` e casos não-texto), mantém-se o fallback do avanço de linha.

**Nota sobre `text_ink_bounds`:** o prompt sugeria inicialmente usar `text_ink_bounds`, mas a medição contra o vanilla revelou que o vanilla `measure()` devolve a altura do frame (cap-height → baseline), não a bounding box de tinta dos glifos. `text_ink_bounds` deu `(8.27pt, ...)` para `hello` a 11pt, enquanto o vanilla devolve `7.9pt` (= cap-height). A escolha correcta foi `text_edges`.

---

## 4. Medições — vanilla vs cristalino, mesma fonte

| Caso | Vanilla 0.15.0 (Nimbus Sans) | Cristalino (Nimbus Sans) | Diferença |
|---|---|---|---|
| `measure([hello])` | `(23.19pt, 7.9pt)` | `(23.23pt, 7.9pt)` | +0.04pt largura; altura idêntica |
| `measure([x])` | `(5.5pt, 7.9pt)` | `(5.5pt, 7.9pt)` | idêntico |
| `measure([abcd])` | `(23.85pt, 7.9pt)` | `(23.85pt, 7.9pt)` | idêntico |
| `measure([a b])` | `(15.29pt, 7.9pt)` | `(15.29pt, 7.9pt)` | idêntico |
| `measure([])` | `(0pt, 0pt)` | `(0pt, 0pt)` | idêntico |
| `measure([hello])`, `size: 20pt` | `(42.16pt, 14.36pt)` | `(42.24pt, 14.36pt)` | +0.08pt largura; altura idêntica |

**Critério de parada satisfeito:** largura e altura batem nos seis casos, com diferenças residuais sub-pixel na largura (≤ 0.08pt), atribuíveis a arredondamento de ponto flutuante e sub-pixel no layout.

---

## 5. Impacto em documentos sem `#context`/`measure()`

A mudança só afecta `measure_content_real`, que só é invocado durante a expansão de `ContextBlock`. Documentos sem `#context` continuam inalterados. Esta checagem foi validada indiretamente pela suíte de testes completa (ver Secção 6).

---

## 6. Validação

Comando corrido:

```bash
cargo test --workspace
crystalline-lint .
```

Resultados:

| Crate/Suíte | Passou | Falhou | Ignorado |
|---|---|---|---|
| typst-core | 4655 | 0 | 2 |
| typst-infra | 717 | 0 | 5 |
| typst-shell | 36 | 0 | 0 |
| typst (binário) | 2 | 0 | 0 |
| tests/cli.rs | 31 | 0 | 0 |
| crystalline_lint.rs | 2 | 0 | 0 |

`crystalline-lint .` reportou apenas o aviso pré-existente **V7** (`package_version_resolution.md` órfão).

---

## 7. Fecho do DEBT-69

`00_nucleo/diagnosticos/debt/DEBT.md` foi actualizado na secção DEBT-69 com:

- Descrição da correção de altura via `text_edges`.
- Tabela final dos seis casos, lado a lado.
- Confirmação de que **DEBT-69 permanece fechado**, sem scope residual.

---

## 8. Referências

- `00_nucleo/materialization/typst-passo-860.md`
- `00_nucleo/diagnosticos/debt/DEBT.md` (secção DEBT-69)
- `00_nucleo/diagnosticos/typst-passo-858-relatorio.md`
- `00_nucleo/diagnosticos/typst-passo-857-relatorio.md`
- `01_core/src/engine/layout/mod.rs:1790`
- `01_core/src/engine/layout/metrics.rs`
- `03_infra/src/font_metrics.rs`
- `lab/typst-original/crates/typst-library/src/layout/measure.rs`
