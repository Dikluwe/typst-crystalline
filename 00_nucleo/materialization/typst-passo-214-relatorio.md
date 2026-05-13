# Relatório do passo P214 — Recálculo categorias restantes pós-M9c

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-214.md`.
**Tipo**: recálculo de cobertura paralelo (administrativo
XS-S documental ampliado).
**Magnitude planeada**: XS-S (~1h-1h30min). **Magnitude real**:
S (~1h).
**Marco**: nenhum (segundo passo pós-M9c; estende método P213).

---

## §1 O que foi feito

Recalculadas empíricamente as 8 categorias restantes do
quadro §2.1 (todas excepto Introspection — já recalculada
em P213). Auditoria detectou 3 tipos de Δ: **2 sincronizações
§2.1 ↔ Tabela A** (Layout +40pp, Model +5pp); **1 reclassificação
material** (Markup +17pp via 3 entries quote/terms/smart-quotes
ausente → implementado em P154B+P155); **5 categorias
inalteradas**. Tabela A inventário 148 + §A.1 entries +
blueprint §2.1 reordenado por cobertura decrescente +
marca §3.0undecies adicionada. Footnote ³⁹ documenta
reclassificações + sincronizações. Zero código tocado;
tests/lint preservados.

---

## §2 Reconta empírica per categoria

| Categoria | §2.1 antes | Tabela A actual (calculado) | Δ pós-P214 | Causa |
|-----------|------------|------------------------------|------------|-------|
| Math | 92% | 12/13 = 92% | 0 | inalterada |
| Foundations stdlib | 67% | 10/15 = 67% | 0 | inalterada |
| `#let`/`#set`/`#show` | 62% | 8/13 = 62% | 0 | inalterada |
| **Markup syntactic** | **61%** | **11/18 = 61%** (pré-recálculo) → **14/18 = 78%** (pós-recálculo material) | **+17pp** | reclassificação material — quote (P155) + terms (P154B) + smart quotes (P155) ausente → impl |
| Visualize | 54% | 7/13 = 54% | 0 | inalterada |
| Text features | 52% | 12/23 = 52% | 0 | inalterada |
| **Model (structural)** | **45%** | **11/22 = 50%** | **+5pp** | sincronização (Tabela A já reflectia footnotes ¹²³²²²⁴²⁹ via P154/155/157/158/159) |
| **Layout** | **38%** | **14/18 = 78%** | **+40pp** | sincronização (Tabela A já reflectia footnotes ⁵⁶⁸¹⁰¹²¹³¹⁵¹⁷¹⁹²¹ via P156B-L) |

**Distribuição Δ tipos**:
- 2 sincronizações puras (Layout, Model): Tabela A já
  correcta; só §2.1 desactualizado.
- 1 reclassificação material (Markup): Tabela A precisou
  actualização + §A.1 entries reescritas inline.
- 5 categorias inalteradas: confirmação empírica (Math,
  Foundations, let-set-show, Visualize, Text features).

---

## §3 Tabela A inventário 148 actualizada

**Markup syntactic**:

Antes (pré-P214):
```
Markup syntactic | 8 | 3 | 3 | 4 | 0 | 18  (61%)
```

Depois (pós-P214):
```
Markup syntactic ³⁹ | 11 | 3 | 3 | 1 | 0 | 18  (78%)
```

**Total user-facing**:
```
Antes: 66/24/25/24/2 = 141  (~64%)
Depois: 69/24/25/21/2 = 141  (~66%)
```

3 entradas movidas de ausente → implementado; total
preservado.

**§A.1 entries reescritas inline** (3 entries):
- `> blockquote`: ausente → **implementado** (P155 — `Content::Quote`
  4 attrs).
- `/ term: definition`: ausente → **implementado** (P154B —
  `Content::Terms` + `Content::TermItem`).
- Smart quotes: ausente → **implementado** (P155 —
  `rules/lang/quotes.rs` 6 idiomas).

**Footnote ³⁹ adicionada** (paridade pattern footnote ³⁸
P213): per-entrada detalhe + sub-passo de fecho +
sincronizações §2.1 documentadas + crescimento patterns
emergentes.

---

## §4 Blueprint §2.1 reordenado por cobertura + §3.0undecies marca

**§2.1 reordenado** por cobertura decrescente (decisão
P214 C5 — paridade visual com prioridade):

```
| Math                                  | 92% | quase total                                       |
| Introspection ⁽ᴾ²¹³⁾                  | 83% | quase total (paridade arquitectural pós-M9c)      |
| Layout ⁽ᴾ²¹⁴⁾                         | 78% | quase total (Fase 1+2+3 sub-passo 1 fechadas)     |
| Markup syntactic ⁽ᴾ²¹⁴⁾               | 78% | quase total (Fase 1 fechada — quote/terms/smart) |
| Foundations stdlib                    | 67% | parcial                                           |
| `#let`/`#set`/`#show`                 | 62% | parcial                                           |
| Visualize                             | 54% | parcial                                           |
| Text features                         | 52% | parcial                                           |
| Model (structural) ⁽ᴾ²¹⁴⁾             | 50% | em curso (Fase 1 fechada; Fase 2 em curso)        |
```

3 categorias com Δ marcadas com ⁽ᴾ²¹⁴⁾ (Layout, Markup,
Model); Introspection mantém ⁽ᴾ²¹³⁾.

**Marca §3.0undecies adicionada** (Opção γ fixada em C6):
"Recálculo paralelo de 8 categorias paralelo a P213,
escala ampliada qualitativamente distinta de §3.0decies".

Conteúdo §3.0undecies:
- Tabela Δ por categoria (9 linhas).
- 3 tipos de Δ documentados.
- Pattern emergente cresce N=1 → 9.
- Subpadrão administrativo XS cresce N=3 → 4.
- Reordenação §2.1 por cobertura decrescente (decisão
  estética).
- Política "sem novas reservas" preservada.
- Estado pós-P214 (4 categorias ≥ 78%; 5 categorias
  50-67%).

---

## §5 Decisões substantivas

- **Opção γ marca §3.0undecies** (vs Opção α "idêntica P213"
  vs β "expandida"): híbrida — preserva pattern marca-por-fecho
  minimal mas regista escala ampliada via tabela Δ per
  categoria. Seriado 11ª marca cumulativa
  (§3.0/bis/ter/quater/quinquies/sexies/septies/octies/
  nonies/decies/undecies = 10 cirúrgicas pós-§3.0bis/ter
  = 11 total).
- **"Tudo de uma vez" vs sub-passos** (vs P213 "uma categoria
  por passo"): fixado em P214 spec — 8 categorias em
  paralelo. Justificação: zero overhead repetido (~10min
  por categoria × 8 = ~80min total); pattern emergente
  cresce 1→9 saltando limiar formalização N=3-4. Sem ADR
  meta (per política P158).
- **Reordenação §2.1 por cobertura decrescente**:
  trade-off documentado — paridade visual com prioridade
  (4 categorias quase total no topo) vs paridade categórica
  alfabética (preservaria ordem original 2026-04-25). P214
  fixou cobertura decrescente para destacar ganhos
  cumulativos pós-M9c.
- **3 entries §A.1 reescritas inline**: paridade com P213
  que reescreveu §A.9 inline. Reescrita preserva semântica
  histórica via referência ao sub-passo de fecho (P154B/
  P155).
- **Sincronização vs reclassificação material distinguidas**:
  Layout +40pp e Model +5pp são sincronizações §2.1 ↔
  Tabela A (Tabela A já correcta via footnotes existentes;
  só §2.1 precisa actualização). Markup +17pp é
  reclassificação material (Tabela A precisou actualização).
  Distinção qualitativamente nova vs P213 (que só teve
  reclassificações materiais).
- **5 categorias inalteradas confirmadas empíricamente**:
  Math/Foundations/let-set-show/Visualize/Text features.
  Não inflação de auditoria — confirmação empírica é
  evidência válida de "estado preservado".

---

## §6 Métricas pós-P214

| Métrica | Pré-M9c | Pós-M9c (P212) | Pós-P213 | Pós-P214 (Δ doc) |
|---------|---------|-----------------|----------|--------------------|
| Trait `Introspector` métodos | 20 | 26 | 26 | 26 |
| `Selector` enum variants | 1 | 6 | 6 | 6 |
| Sub-stores L1 | 23 | 25 | 25 | 25 |
| Stdlib funcs registadas | ~50 | ~53 | ~53 | ~53 |
| Allowlist L1 deps externas | 11 | 12 | 12 | 12 |
| Tests workspace | 1873 | 1939 | 1939 | 1939 |
| `crystalline-lint` violations | 0 | 0 | 0 | 0 |
| ADRs ACEITES M9c | — | 2 | 2 | 2 |
| Categoria Introspection §A.9 | (17%) | (17%) | **83%** | **83%** |
| Categoria Markup syntactic | (61%) | (61%) | (61%) | **78%** (P214) |
| Categoria Layout §2.1 | (38%) | (38%) | (38%) | **78%** (P214 sync) |
| Categoria Model §2.1 | (45%) | (45%) | (45%) | **50%** (P214 sync) |
| **Cobertura user-facing total** | (~58%) | (~61%) | **~64%** (P213) | **~66%** (P214) |
| Blueprint marcas cirúrgicas | 3 | 9 | 10 | **11** (+§3.0undecies) |
| Footnotes inventário 148 | 37 | 37 | 38 | **39** (+³⁹ P214) |

---

## §7 Pattern emergente

**"Diagnóstico-recálculo pós-marcos" cresce N=1 → N=9**:

P214 estende método P213 a 8 categorias. Auditoria empírica
produziu:
- 1 categoria com material reclass (Markup).
- 2 categorias com sincronização §2.1 (Layout + Model).
- 5 categorias inalteradas (Math/Foundations/let-set-show/
  Visualize/Text features).

**Total**: P213 (1 categoria material) + P214 (1 material +
2 sync + 5 inalteradas confirmadas) = **9 evidências
empíricas** do pattern. Ultrapassa limiar formalização
N=3-4.

**Promoção a ADR meta diferida** per política "sem novas
reservas" P158. Promoção fica para passo dedicado se
humano julgar útil para sessões futuras.

**Subpadrão "passo administrativo XS" cresce N=3 → N=4**:

`ADR-0062-create` + P160A + P213 + **P214** = N=4.

Limiar formalização N=3-4 ultrapassado claramente. ADR
meta candidata real:
- Conteúdo: "passo administrativo XS recálculo cobertura
  pós-fecho de marco — quando aplicar / forma / política
  de reservas".
- Custo: XS documental (~30min se humano decidir promover).
- Diferida em P214 per política P158 — promoção é decisão
  separada, não mecânica.

---

## §8 Próximo passo (fora P214)

P214 fecha pendência documental ampla pós-M9c. Próximo
passo fica em aberto para decisão humana entre opções:

| Opção | Trabalho | Magnitude | Prioridade subjectiva |
|-------|----------|-----------|------------------------|
| **Materialização Bloco A** | `measure(body)` stdlib expose — fecha §A.9 estricto 83% → 100% | S+ (~1-2h) | alta (close gap pequeno) |
| **Materialização Bloco B** | `position()` standalone stdlib + `query_count_before` — avança §A.9 estendido | S+ cada (~1-2h cada) | média |
| **ADR meta administrativa** | Formalizar pattern "diagnóstico-recálculo pós-marcos" + "passo administrativo XS recálculo" — N=9 + N=4 evidências cumpridas | XS (~30min documental) | média (reusabilidade futura) |
| **Layout Fase 3 columns/colbreak** | Endereçar DEBT-56 — Layout 78% → +X% | M+ (~3-5h) | média-alta |
| **Model bibliography hayagriva** | Endereçar DEBT-55 — Model 50% → +X% | L (~5-8h) | baixa-média (custo alto; wraps existing input) |
| **Outro módulo** | Refactor / cleanup / consolidation | varia | varia |

**Política "sem novas reservas" preservada per P158**:
candidatos identificados acima são **opções**, não
compromissos. Reservas pré-existentes (slot 0063 column
flow + 11 deferreds M9c) mantêm-se documentadas mas não
reforçadas em P214.

**Recomendação metodológica subjectiva** (não fixação):
- Se humano quer continuar reduzindo gaps: **Bloco A**
  (S+ trivial; fecha §A.9 estricto a 100%).
- Se humano quer formalizar metodologia: **ADR meta** (XS
  documental; reusabilidade alta para sessões futuras de
  recálculo paralelo a Visualize/Text/etc.).
- Se humano quer abrir nova fase de trabalho: **Layout Fase
  3** ou **Outro módulo**.

**Estado final pós-P214**:
- Marco M9c: ✅ ACEITE 2026-05-12 (preservado).
- ADRs M9c: 2 ACEITES (preservadas).
- ADR-0073: ACEITE com fecho retroactivo M9c (preservada).
- Categoria Introspection §A.9: **83%** (recalculada P213).
- Categoria Markup syntactic: **78%** (recalculada P214 —
  reclassificação material).
- Categoria Layout: **78%** (recalculada P214 — sincronização).
- Categoria Model: **50%** (recalculada P214 — sincronização).
- Cobertura user-facing total: **~66%** (recalculada P214).
- 4 categorias com cobertura ≥ 78% (vs 1 antes de P213).
- Tests workspace: **1939 verdes**; `crystalline-lint`: **0
  violations** (preservados).
- Trajectória aberta: 5-6 opções para próxima sessão;
  decisão humana.
