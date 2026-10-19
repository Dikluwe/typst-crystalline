# Passo 154A — Relatório (diagnóstico Model + ADR-0060 PROPOSTO + DEBT-55)

**Data**: 2026-04-25
**Natureza**: passo **L0-puro / diagnóstico-primeiro**.
**Zero código tocado**. **Zero testes**. **1 ADR PROPOSTO**
criada (0060). **1 DEBT aberto** (DEBT-55).
**Mudança de prioridade registada**: série paridade
suspensa em P153; foco passa para gap real de cobertura
Model.
**Precondição**: Passo 153 encerrado; matriz P2 + P3
cristalino-only baseline; 1113 tests cristalino; 12 DEBTs
abertos; inventário 148 actualizado por P149.

---

## 1. Sumário

Inventário 148 §A.6 declara categoria Model (structural) com
cobertura 38%. P154A diagnosticou empiricamente
`lab/typst-original/crates/typst-library/src/model/` (22
ficheiros) confronto com `01_core/src/entities/content.rs`
e revelou cobertura real **32-36%** (revisão para baixo).

**Outputs**:

- `00_nucleo/diagnosticos/diagnostico-model-passo-154a.md`
  (~280 linhas; 8 secções).
- `00_nucleo/adr/typst-adr-0060-model-structural-roadmap.md`
  (status `PROPOSTO`; roadmap Fase 1+2+3).
- DEBT-55 aberto: bibliography + cite XL (bloqueado por
  ADR-0061 que autorizará `hayagriva`).
- Inventário 148 actualizado: linha Model 4/4/5/8/0=21 →
  **3/4/5/10/0=22**; cobertura user-facing total 54% → 53%.
- README dos ADRs: 59 → 60 ADRs; `PROPOSTO` 10 → 11.
- Cabeçalho de DEBT.md: 12 → 13 DEBTs abertos.

**Tests cristalino**: 1113 inalterados.

---

## 2. Inventário detalhado (sub-passo 154A.1)

22 elementos vanilla em `model/`; ~28 com sub-elementos:

- **Top-level elements**: AssetElem, BibliographyElem,
  CiteElem, DividerElem, DocumentElem, EmphElem, EnumElem,
  FigureElem, FootnoteElem, HeadingElem, LinkElem, ListElem,
  OutlineElem, ParElem, QuoteElem, RefElem, StrongElem,
  TableElem, TermsElem, TitleElem.
- **Sub-elementos**: BibliographyEntryElem, FigureCaptionElem,
  FootnoteEntryElem, EnumItemElem, ListItemElem, TermItemElem,
  TableCellElem, TableHeaderElem, TableFooterElem,
  TableHLineElem, TableVLineElem, OutlineEntryElem,
  ParBreakElem.

Detalhe completo no diagnóstico §1.

---

## 3. Estado actual cristalino (sub-passo 154A.2)

| Estado | Contagem | Lista |
|--------|----------|-------|
| `implementado` | 3 | emph, strong, outline |
| `implementado⁺` | 4 | heading (com ressalvas), figure, ref, numbering |
| `parcial` | 5 | link, list, enum, par, caption (inline) |
| `ausente` | 10 | bibliography, cite, footnote, quote, terms, table, document, divider, asset, title |
| `scope-out` | 0 | — |
| **Total** | **22** | (ajustado de 21 — caption isolado de figure; document/divider/asset/title individualizados) |

**Cobertura empírica**: (3+4)/22 = **32%** ou (4+4)/22 =
**36%** (heading classificado como impl ou impl⁺). Inventário
148 declarava 38% — revisão para baixo de 2-6 pp.

---

## 4. Bloqueantes arquitecturais (sub-passo 154A.3)

10 bloqueantes identificados, quantificados por custo:

- **Trivial (S)**: `Content::Divider`, `Content::Asset`,
  `Content::Terms` + `Content::TermItem`.
- **Médio (M)**: `Content::Quote {body, attribution, block}`,
  `Content::Table` + sub-elementos, `Content::Footnote`.
- **Médio-+ (M+)**: Cell layouting (table; trabalho similar
  a DEBT-34d/e mas distinto), Numbering rules ricas.
- **Largo (L)**: Page model footnote area (depende de Layout
  Fase X).
- **Extra-largo (XL)**: `Content::Bibliography` +
  `Content::Cite` + CSL parser via `hayagriva`.

`Content::Styled` (ADR-0026 perfil) é **inadequado** para
Model structural — features têm semântica que excede styling.

---

## 5. Arqueologia (sub-passo 154A.4)

10 ausentes, com classificação per critério P149:

| Elemento | Razão | Classificação |
|----------|-------|---------------|
| bibliography | ADR-0017 + exige `hayagriva` | adiamento priorizável (Fase 2) |
| cite | depende de bibliography | mesma |
| footnote | bloqueado por Page model | adiamento condicional (Layout Fase X) |
| quote | sem registo | candidato Fase 1 |
| terms | sem registo | candidato Fase 1 |
| table | DEBT-34d/e abertos para grid cells | adiamento priorizável (Fase 2) |
| document | metadata emitida directamente em export | divergência intencional |
| divider | sem registo; trivial | candidato Fase 1 |
| asset | imagens via `Image` cobrem; alt-text adicional | candidato Fase 3 |
| title | depende de document | divergência intencional |

**DEBT-34d / DEBT-34e** confirmados abertos mas focam em
**grid cell layouting** (Passo 80), não em `Content::Table`
estrutural. Trabalho similar mas distinto.

---

## 6. Crates externas (sub-passo 154A.5)

| Elemento | Crate | Cache (P152) | Custo autorização |
|----------|-------|--------------|-------------------|
| bibliography + cite | `hayagriva` 0.9.1 | ✓ | **ADR-0061** (a criar) |
| outras | nenhuma | — | nenhuma |

`hayagriva` confirmado em cache (probe P152 §3); fetch
online não é necessário.

---

## 7. Priorização (sub-passo 154A.6)

Matriz custo × valor:

```
              Alto valor          Médio valor          Baixo valor
S       [F1: terms,                                    [F3: asset,
         divider]                                       title]
M       [F1: footnote*]    [F1: quote]                 [F2: document*]
M+      [F2: table]                                    [F3: stroke-obj]
L       [F2: figure-kinds]                             [—]
XL      [F2: bibliography                              [—]
         + cite]
```

`*` = `footnote` reclassificado de Fase 1 → condicional
(depende Layout); `document` em Fase 3 (divergência aceite).

**Fase 1 final**: terms + divider + quote → cobertura
~50% (8 → 11/22).

**Fase 2**: table + figure kinds + (ADR-0061 +
bibliography + cite) → cobertura ~68% (11 → 15/22).

**Fase 3**: asset + (decisão sobre document/title) →
77-82% potencial.

---

## 8. Plano de materialização proposto

5 sub-passos no caminho crítico:

| Passo | Escopo | Features | ADR adicional? |
|-------|--------|----------|-----------------|
| **154B** | S agregado | terms + divider | — |
| **155** | M | quote | — |
| **156** | M+ | table foundations | — |
| **157** | M | figure kinds | — |
| **ADR-0061 + 158** | XL | bibliography + cite | ADR-0061 |
| (futuro) | M-L | footnote | depende Layout Fase X |
| (Fase 3) | S+condicional | asset, document, title | — |

Detalhes em ADR-0060 §"Plano de materialização".

---

## 9. ADR-0060 produzida

Ficheiro: `00_nucleo/adr/typst-adr-0060-model-structural-roadmap.md`.

Estrutura (cabeçalho canónico P145):

```
# ⚖️ ADR-0060: Model (structural) roadmap — Fase 1 + Fase 2 + Fase 3

**Status**: `PROPOSTO`
**Validado**: Passo 154A — diagnóstico.
**Data**: 2026-04-25
**Diagnóstico prévio**:
[...diagnostico-model-passo-154a.md...]
```

Secções: Contexto, Decisão (5 itens), Alternativas (tabela 5
opções), Consequências (positivas/negativas/neutras), Plano
de materialização (tabela com 5 passos críticos + 2
condicionais), Referências (10 ADRs + DEBT-55 + DEBT-34d/e
+ inventário 148).

**Status `PROPOSTO`** transita para `IMPLEMENTADO` apenas
quando Fase 1 + Fase 2 + ADR-0061 concluírem.

---

## 10. DEBTs novos

### DEBT-55 — Bibliography + Cite (XL)

Aberto. Bloqueado por ADR-0061 (autorização `hayagriva`)
que ainda não foi criada (será materializada em passo
dedicado durante Fase 2 do roadmap).

Plano cumulativo:
- ADR-0061 criada.
- `Cargo.toml` + `crystalline.toml` configurados.
- `Content::Bibliography` + `Content::Cite` variants.
- `native_bibliography` + `native_cite` em stdlib.
- Pipeline introspect para resolução cruzada.
- Render layout.
- Tests + corpus paridade.
- Inventário 148 reclassifica ambas para `implementado⁺`.

Critério de fecho: 4 itens. Estimativa: ~5-8h.

### Sem outros DEBTs novos

Critério explícito (P154A Decisão diferida 11): trabalho
mecânico de Fase 1/2 (terms, divider, quote, table
foundations, figure kinds, footnote eventual) é **roadmap
items** em ADR-0060, não DEBTs. Apenas `bibliography + cite`
qualifica como DEBT (XL + autorização externa).

DEBT-34d / DEBT-34e (grid cell layouting Passo 80)
**permanecem abertos**; relacionados mas distintos do roadmap
Model. Referenciados em ADR-0060.

---

## 11. Inventário 148 actualizado

`typst-cobertura-vanilla-vs-cristalino.md` actualizado:

- **Tabela A linha "Model"**: 4/4/5/8/0=21 → 3/4/5/10/0=22.
  Heading reclassificado (impl → impl⁺); caption isolado;
  document/divider/asset/title individualizados.
- **Total user-facing**: 54/21/21/40/2=138 → 53/21/21/42/2=139.
- **Cobertura user-facing total**: 54% → **53%**.
- **§7 Top divergências entrada 7**: extendida com
  referência cruzada ao diagnóstico 154A; menciona ADR-0060
  + DEBT-55.

---

## 12. README dos ADRs actualizado

`00_nucleo/adr/README.md`:

- Cabeçalho: 59 → 60 ADRs (58 → 59 números únicos).
- Tabela "Estado por ADR": linha nova ADR-0060 (`PROPOSTO`).
- Distribuição: `PROPOSTO` 10 → 11 (adiciona 0060).
- Entrada nova "Passo 154A" em "Passos-chave da história
  dos ADRs".

---

## 13. Próximo passo

**P154B — Fase 1 primeira sub-fase**: materializar
`Content::Terms` + `Content::TermItem` + `Content::Divider`
em `01_core/src/entities/content.rs`. Custo S agregado.

Sequência ADR-0060 (Fase 1 + 2):
- 154B — terms + divider.
- 155 — quote.
- 156 — table foundations.
- 157 — figure kinds.
- ADR-0061 + 158 — bibliography + cite.

Decisão entre arrancar P154B imediatamente ou priorizar
DEBT-54 (vanilla integration série paridade) ou outro
trabalho fica a cargo do utilizador.

---

## 14. Verificação final

| Item | Estado |
|------|--------|
| Diagnóstico em `00_nucleo/diagnosticos/diagnostico-model-passo-154a.md` (8 secções) | ✅ |
| ADR-0060 criada com cabeçalho canónico P145 + status `PROPOSTO` | ✅ |
| Cada elemento Model classificado com referência | ✅ |
| Bloqueantes arquitecturais identificados | ✅ |
| Crates externas listadas (apenas `hayagriva`) | ✅ |
| Priorização ranqueada (matriz custo × valor) | ✅ |
| Plano de materialização com 5 sub-passos críticos + 2 condicionais | ✅ |
| DEBT-34d/e estado verificado (abertos; relacionados mas distintos) | ✅ |
| DEBT-55 aberto (bibliography + cite XL) | ✅ |
| Inventário 148 actualizado (linha Model + total + §7) | ✅ |
| README dos ADRs actualizado (total 59 → 60; PROPOSTO 10 → 11; Passos-chave) | ✅ |
| Nenhum ficheiro em L1/L2/L3/L4 cristalino tocado | ✅ |
| Nenhum ficheiro em `lab/parity/` tocado | ✅ |
| `cargo test --workspace --lib`: 1113 inalterado | ✅ |
| `crystalline-lint .`: zero violations | ✅ |
| Relatório do passo escrito | ✅ |

**Pós-154A**:
- 60 ADRs (1 nova PROPOSTO).
- 13 DEBTs abertos (1 novo).
- Inventário 148 com referência cruzada para o diagnóstico
  154A.
- Roadmap Model claro com 5 sub-passos críticos +
  condicionais.
- **Próximo substantivo**: 154B (Fase 1 primeira sub-fase)
  ou outra prioridade humana.

**Reformulação 7 da série paridade efectivamente
encerrada em P153**. P154A inicia **nova linha de trabalho**
focada em gap real de cobertura Model. Padrão
diagnóstico-primeiro continua a aplicar-se (5ª aplicação:
131A/132A/140A/148/154A).
