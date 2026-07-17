# Relatório de Paridade — P772a (lote 3)

**Data:** 2026-07-16
**Passo:** P772a
**Objetivo:** Reconfirmar contagem dos módulos pendentes e classificar item a item o módulo `typst_library::pdf::accessibility`.

---

## Estado base da medição

- **Commit base:** `dacc0909bb8cbe937c4e75cacb1ae1a8f985517a` (P779)
- **Fonte de lacunas:** `00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt`
- **Comando de contagem:**

```bash
awk -F'\t' '$1=="lacuna-inventario"' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt \
  | cut -f5 | sed 's/::[^:]*$//' | sort | uniq -c | sort -rn | head -20
```

- **Validação:** `cargo test --workspace --release` passou; `crystalline-lint .` sem violações relacionadas (único warning pré-existente V7 sobre `package_version_resolution.md`).

---

## Recontagem por módulo (não tratados)

| Contagem | Módulo |
|----------|--------|
| 45 | `typst_library::foundations::calc` |
| 32 | `typst_library::math::style` |
| 24 | `typst_library::layout::grid::resolve` |
| 24 | `typst_library::diag` |
| 20 | `typst_library::foundations::ops` |
| 14 | `typst_utils` |
| 13 | `typst_library::visualize::image::raster` |
| **12** | **`typst_library::pdf::accessibility`** |
| 10 | `typst_syntax::span` |
| 8 | `typst_syntax::package` |
| 8 | `typst` |
| 7 | `typst_syntax::reparser` |
| 7 | `typst_library::visualize::image::svg` |
| 7 | `typst_library::foundations::scope` |

**Módulo escolhido:** `typst_library::pdf::accessibility` (12 itens), o maior não tratado após `image::raster`.

---

## Classificação item a item — `typst_library::pdf::accessibility`

| Item | Tipo | Classificação | Justificação |
|------|------|---------------|--------------|
| `ArtifactElem` | Elemento | Scope-out documentado | Implementado em `01_core/src/engine/stdlib/pdf.rs` como passthrough do `body`. A marcação de artefacto na tag tree PDF requer tagged PDF, que o exportador cristalino não suporta. L0 `stdlib/pdf.md` registou este scope-out em P735. |
| `ArtifactKind` | Enum | Scope-out documentado | Tipo auxiliar de `ArtifactElem`; sem tagged PDF, não tem efeito observável no cristalino. |
| `table_summary#1` | Função | Infra-estrutura / feature flag | No vanilla está gated em `Feature::A11yExtras`, off por omissão. Não está presente no binário de referência medido. Confirmado com teste: `#pdf.table.summary(...)` dá erro `module pdf does not contain table` no vanilla 0.15.0 padrão. |
| `table_summary#2` | Função | Infra-estrutura / feature flag | Mesmo que acima. |
| `header_cell#1` | Função | Infra-estrutura / feature flag | Gated em `A11yExtras`; não observável no binário padrão. |
| `header_cell#2` | Função | Infra-estrutura / feature flag | Gated em `A11yExtras`; não observável no binário padrão. |
| `data_cell#1` | Função | Infra-estrutura / feature flag | Gated em `A11yExtras`; não observável no binário padrão. |
| `data_cell#2` | Função | Infra-estrutura / feature flag | Gated em `A11yExtras`; não observável no binário padrão. |
| `PdfMarkerTag` | Elemento | Infra-estrutura interna | Marcadores internos do vanilla para construir a tag tree PDF. Não são símbolos de língua expostos. |
| `PdfMarkerTagKind` | Enum | Infra-estrutura interna | Tipo auxiliar de `PdfMarkerTag`. |
| `TableCellKind` | Enum | Infra-estrutura interna | Classificação interna de células; usada pelo vanilla para estrutura PDF. |
| `TableHeaderScope` | Enum | Infra-estrutura interna | Tipo auxiliar de escopo de header cell. |

---

## Bugs reais encontrados

**Nenhum.** Todos os 12 itens são ou scope-out documentado (`ArtifactElem`/`ArtifactKind`) ou infra-estrutura interna/feature flag (`A11yExtras`). Não há divergência observável no binário padrão do vanilla a corrigir neste passo.

---

## Decisões

1. **Não implementar tagged PDF neste passo.** O scope-out de `pdf.artifact` já foi registado no L0 `stdlib/pdf.md` (P735); implementar tagged PDF seria um projecto à parte, fora do scope de P772a.
2. **Não expor `pdf.table.summary`, `pdf.header-cell`, `pdf.data-cell`.** Estas funções não existem no vanilla 0.15.0 padrão (feature `A11yExtras` desligada); expô-las no cristalino seria uma extensão não solicitada.
3. **Marcar `pdf::accessibility` como coberto.** Nenhum item requer correcção imediata; o módulo está em conformidade com o L0 vigente.
4. **Próximo passo:** P772b (lote 4), provavelmente `typst_syntax::span` (10 itens) ou `typst_syntax::package` (8 itens), conforme a mesma metodologia.
