# Relatório — Passo 156B: Diagnóstico Layout (Fase X)

**Data**: 2026-04-25.
**Natureza**: passo **L0-puro / administrativo / diagnóstico
+ ADR proposta**. **Zero código L1/L2/L3/L4 tocado**.
**Oitava aplicação** do padrão diagnóstico-primeiro;
**primeira aplicação a categoria Layout**.
**Spec**: `00_nucleo/materialization/typst-passo-156b.md`.

**Outputs materiais**:
- Diagnóstico: `00_nucleo/diagnosticos/diagnostico-layout-passo-156b.md`.
- ADR-0061 (PROPOSTO): `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`.
- DEBT-56 aberto: column flow Fase 3 Layout (em `DEBT.md`).
- DEBT-55 actualizada: nova reserva ADR-0062 + Passo 159.
- ADR-0060 anotada: renumeração Fase 2 Model (P156→P157, P157→P158, P158→P159).
- README ADRs actualizado: tabela com ADR-0061 PROPOSTO; reserva ADR-0062 documentada.
- Inventário 148 actualizado: Tabela A.5 Layout reclassificada; Tabela A linha "Layout"
  ajustada (6/0/2/8/0=16 → 4/0/3/11/0=18); §7 entrada 7 actualizada.
- Este relatório.

---

## §1 — Sumário

P156B executou diagnóstico-primeiro à categoria Layout (38%
declarado per inventário 148 §A.5 — segunda mais fraca após
Introspection 17%). Padrão estabelecido com evidência 6/6
em P156A historiograma motivou aplicação a Layout (oitava
aplicação no total, primeira a esta categoria).

Diagnóstico revelou:

- **Cobertura empírica recalculada**: 22% implementado puro
  (4/18 entradas), vs 38% declarado. Padrão análogo ao recálculo
  Model 154A (38% → 32-36%).
- **5 entradas viáveis para Fase 1** (page model footnote area
  + pad + hide + pagebreak + h/v) num único passo agregado
  (P156C, M+).
- **3 entradas para Fase 2** (block, box, stack).
- **5 entradas para Fase 3** condicional (columns + colbreak
  com DEBT-56 aberto; repeat; skew; refino Page rico).
- **Sem novas crates externas** — Layout é trabalho L1 puro
  em todas as fases.
- **Footnote (Model Fase 2)** desbloqueado por sub-fase mínima
  P156C (page model com footnote area).

ADR-0061 criada em status `PROPOSTO` (Layout roadmap). Reocupou
o número antes reservado para `hayagriva`; reserva passou para
ADR-0062. ADR-0060 (Model Fase 2) anotada com renumeração
P156→P157, P157→P158, P158→P159. DEBT-55 e DEBT.md actualizados.

---

## §2 — Inventário detalhado vanilla (resumo §1 do diagnóstico)

Fonte: `lab/typst-original/crates/typst-library/src/layout/`
(~32 ficheiros + `grid/` subdir). Lógica algorítmica em
`lab/typst-original/crates/typst-layout/` (out-of-scope).

**Elementos visuais user-facing** (~22 elementos identificados):

- **Containers**: `BoxElem`, `BlockElem`, `InlineElem`,
  `BlockBody`, `Sizing`.
- **Composição**: `StackElem`, `StackChild`, `ColumnsElem`,
  `ColbreakElem`, `PadElem`, `HideElem`, `RepeatElem`.
- **Posicionamento**: `PlaceElem`, `FlushElem`, `AlignElem` +
  hierarquia `Alignment`/`HAlignment`/`VAlignment`/etc.
- **Transformações**: `MoveElem`, `RotateElem`, `ScaleElem`,
  `SkewElem`, `Transform`, `ScaleAmount`.
- **Spacing**: `HElem`, `VElem`, `Spacing`.
- **Page model**: `PageElem` (18 atributos #[ghost]),
  `PagebreakElem`, `Margin`, `Binding`, `PageRanges`,
  `Parity`, `Paper`.
- **Grid**: `GridElem` (atributos ricos), `GridCell`,
  `GridHLine`, `GridVLine`, `GridHeader`, `GridFooter`,
  `TrackSizings`, `Celled<T>`.
- **Acesso**: `LayoutElem` (Locatable; ghost para `layout()`).

**Funções stdlib relacionadas**: `layout(callback)`,
`measure(content, container?)`, `pagebreak(...)`.

**Tipos primitivos** (geometria + units): `Frame`, `FrameItem`,
`FrameKind`, `Fragment`, `Region`, `Regions<'a>`, `Sides<T>`,
`Corners<T>`, `Axes<T>`, `Dir`, `Abs`, `Em`, `Length`, `Ratio`,
`Rel<T>`, `Fr`, `Point`, `Rect`, `Size`, `Angle`, `Transform`,
`Sizing`, `Spacing`, `Alignment` family, `PlacementScope`.

**Page runtime real**: vive em
`lab/typst-original/crates/typst-layout/src/document.rs`
(`Page` = frame + metadata mínima; header/footer fundidos no
Frame).

**Footnote**: gerida em `FlowState` em
`typst-layout/src/flow/mod.rs` — sem campo `footnote_area`
em `Page` ou `Regions`.

**Column flow**: declarativo trivial (103 linhas), mas algoritmo
~3000 linhas em `typst-layout/src/flow/`. Vanilla **não faz
balanceamento** de colunas.

**Crates externas** no módulo Layout vanilla: nenhuma específica
de layout (apenas `comemo`, `smallvec`, `ecow`, `typst_utils`,
`typst_syntax`).

---

## §3 — Estado actual cristalino (resumo §2 do diagnóstico)

**Implementado** (4): `align`, `move`, `rotate`, `scale` (3
últimos via `Content::Transform` unificado).

**Parcial** (3):
- `place` — sem `float`, `clearance`; divergência `PlaceScope::Parent`.
- `grid` — sem `gutter`, `align`, `stroke`, `fill`, `inset`,
  `header`, `footer`, `colspan`/`rowspan`. **DEBT-34d/e abertos**.
- `measure` — helper `measure_content` privado; sem stdlib
  exposta.

**Ausente** (11):
1. `pad` (era declarado parcial — reclassificação P156B).
2. `box` (Fase 2 ADR-0061).
3. `block` (Fase 2 ADR-0061).
4. `stack` (Fase 2 ADR-0061).
5. `hide` (Fase 1 ADR-0061; trivial).
6. `repeat` (Fase 3 ADR-0061; lazy semantic).
7. `columns` (Fase 3 ADR-0061; **DEBT-56**).
8. `colbreak` (Fase 3 ADR-0061; depende de columns).
9. `pagebreak` manual (era declarado parcial — reclassificação
   P156B; só implícito via overflow).
10. `h`/`v` spacing primitives (entrada nova; não estava em
    A.5 do inventário 148).
11. `skew` (entrada nova; não estava em A.5).

**Adições não estavam em A.5**: `h`/`v` (vanilla `HElem`/`VElem`)
e `skew`. Total de entradas Layout passa de 16 para **18**.

**Estrutura crítica**: `Page` actual é
`{ width, height, items: Vec<FrameItem> }` — **sem
`footnote_area`**, sem header/footer/background/foreground.
`Frame` é vestigial (Layouter escreve directo em `current_items`).
PageConfig.margin é escalar (vanilla é `Sides<Length>`).

**Cobertura**: (4 + 0) / 18 = **22% implementado puro** (vs 38%
declarado). Padrão análogo a Model 154A (38% → 32-36%).

---

## §4 — Bloqueantes arquitecturais (resumo §3 do diagnóstico)

15 bloqueantes identificados. Críticos:

| Bloqueante | Quem precisa | Custo | Decisão arquitectural? |
|------------|--------------|------:|------------------------|
| `Page::footnote_area: Vec<FrameItem>` | `footnote()` (Model) | M | **sim** — extensão minimalista de `Page` |
| Reservar espaço de footnote durante layout | idem | M | **sim** — Layouter altera `cursor_y` reserve |
| `Content::Pad` ou `Style::Pad` | `pad()` | S | sim |
| `Content::Hide` | `hide()` | S | sim — variant trivial |
| `Content::Pagebreak` ou `Style::PageBreak` | `pagebreak()` | S | sim — decisão default: variant |
| `Content::HSpace` + `Content::VSpace` | `h()` + `v()` | S | sim — variants triviais |
| `Content::Block` ou `Content::Styled` | `block()` | M+ | sim |
| `Content::Box` ou `Content::Styled` | `box()` | S-M | sim |
| `Content::Stack { dir, ... }` | `stack()` | S-M | sim — variant novo |
| Multi-region Layouter (column flow) | `columns()` + `colbreak()` | L+ | **sim — ADR dedicada futura; DEBT-56** |
| `Content::Repeat` + lazy semantic | `repeat()` | M | sim |
| Introspection runtime | `measure()`, `layout()` | XL | depende ADR-0017 (adiada) |
| `Content::Skew` | `skew()` | S | sim — variant trivial |
| Sides<Length> margens em PageConfig | PageConfig completo | S | refactor |
| Header/footer/background/foreground em Page | Page model rico | M+ | sim — extensão estrutural |

---

## §5 — Arqueologia (resumo §4 do diagnóstico)

Padrão dominante: **a maioria das ausências (pad, hide,
pagebreak, h/v, block, box, stack) são simples e nunca foram
priorizadas** — não há ADRs de adiamento explícito; é dívida
implícita acumulada.

Excepções:
- `measure`/`layout(callback)`: dependem de Introspection
  runtime (ADR-0017 adiada por Passo 17).
- `columns`: custo L+ com refactor multi-region do Layouter
  (DEBT-56 aberto por este passo).

**Footnote area**: bloqueado por footnote (Model 154A);
nunca materializado por ser pré-condição estrutural — dívida
estrutural identificada explicitamente por este diagnóstico.

---

## §6 — Crates externas (resumo §5 do diagnóstico)

**Confirmado empiricamente**: nenhuma crate externa específica
de layout é necessária (cosmic-text, taffy, harfbuzz, etc.).

Vanilla apenas usa: `comemo`, `smallvec`, `ecow`, `typst_utils`,
`typst_syntax` — todas já em L1 cristalino ou disponíveis.

**Conclusão**: Layout Fase X (todas as 3 fases, incluindo
column flow) é trabalho L1 puro. **Nenhuma ADR de autorização
de crate é necessária**.

---

## §7 — Priorização (matriz §6 do diagnóstico)

```
              Alto valor              Médio valor            Baixo valor
S       [F1: pad, hide,           [F2: box, stack]       [F3: skew]
         pagebreak, h, v]
M       [F1: footnote area]      [F3: repeat]
M+      [F2: block]
L+      [F3: columns]
                                                          → DEBT-56
XL      [—]
```

**Aspirações de cobertura**:
- Pós-Fase 1 (P156C): 22% → ~50% (4/18 → 9/18 implementadas).
- Pós-Fase 2: ~50% → ~67% (9/18 → 12/18).
- Pós-Fase 3: ~67% → 94-100% (12/18 → 17-18/18).

---

## §8 — Plano de materialização (referência a ADR-0061)

Plano detalhado em `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`.
Resumo:

| Passo | Escopo | Features | ADR? |
|-------|--------|----------|------|
| **156C** | M+ | Fase 1 Layout (page model footnote area + pad + hide + pagebreak + h + v) | — (aplica ADR-0061) |
| (Layout F2) | M+ | block + box + stack | — |
| (Layout F3 columns) | L+ | columns + colbreak; fecha DEBT-56 | **ADR dedicada** column flow |
| (Layout F3 visuais) | S+M | repeat + skew | — |
| (Layout refino Page) | M+ | Sides<Length> margens; header/footer/background/foreground | — |

Numeração final dos passos pós-156C decidida humanamente,
podendo intercalar com Model Fase 2 (P157 table; P158
figure-kinds; P159 bibliography).

**Sub-passo crítico declarado**: P156C **desbloqueia
`footnote()`** em Model Fase 2.

---

## §9 — ADR-0061 produzida

Ficheiro: `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`.

Cabeçalho canónico P145:
- `# ⚖️ ADR-0061: Layout Fase X — page model + multi-column + footnote area roadmap`
- `**Status**: \`PROPOSTO\``
- `**Data**: 2026-04-25`
- `**Validado**: Passo 156B — diagnóstico`
- `**Diagnóstico prévio**: ../diagnosticos/diagnostico-layout-passo-156b.md`

Estrutura: Contexto, Decisão (7 itens — Fase 1, Fase 2, Fase 3,
regra Content::Styled vs variant, footnote area como sub-fase,
relação com ADR-0060, sem novas crates), Alternativas (tabela
de 6 opções), Consequências (positivas/negativas/neutras),
Plano de materialização (5+ passos), Referências (12 ADRs,
DEBTs e diagnósticos).

**Reocupação documentada explicitamente**: ADR-0061 estava
reservada para `hayagriva`; reocupada por Layout. Reserva
hayagriva passou para **ADR-0062** (sem ficheiro criado).

---

## §10 — DEBTs novos

### DEBT-56 — Column flow Fase 3 Layout (L+; refactor multi-region)

**Aberto em**: P156B.
**Bloqueia**: Fase 3 do roadmap Layout (ADR-0061) — `columns()`
e `colbreak()` ficam ausentes até ser materializado.

**Critério de abertura cumprido** (per spec P156B Decisão
diferida 11): trabalho exige >5h dedicadas (refactor
multi-region do Layouter) e bloqueia outras features (columns/
colbreak).

**Plano**: ADR dedicada (column flow algorithm) + passo
dedicado quando priorizado.

**DEBTs não-abertos** (com justificação):
- Block, box, stack: trabalho de Fase 2; cabe em passo
  dedicado M+. Item de roadmap (ADR-0061), não DEBT.
- Pad, hide, pagebreak, h/v: trabalho de Fase 1; passo
  agregado P156C. Item de roadmap.
- Repeat, skew: Fase 3 baixo valor; itens de roadmap, não
  DEBTs.
- Refino Page (Sides<Length> margens, header/footer): Fase 3
  refino; item de roadmap.
- `measure`/`layout(callback)`: dependem de ADR-0017
  (Introspection adiada); não são DEBTs novos — herdam o
  adiamento existente.

---

## §11 — Inventário 148 actualizado

Ficheiro: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.

**Tabela A.5 Layout reescrita**:
- 4 reclassificações: `pad` (parcial→ausente), `pagebreak`
  manual (parcial→ausente), `grid` (impl⁺→parcial), `place`
  (implementado→parcial).
- 2 entradas adicionadas: `h`/`v` spacing e `skew`.
- Notas ⁵ marcam reclassificações + adições com referência ao
  diagnóstico P156B.

**Tabela A — Vista user-facing (resumo)**:
- Linha "Layout" actualizada de `6 | 0 | 2 | 8 | 0 | 16` para
  **`4 | 0 | 3 | 11 | 0 | 18`** (nota ⁵).
- Total user-facing recalculado: 56/21/21/39/2=139 →
  **54/21/22/42/2=141**.
- Cobertura user-facing total recalculada: (54+21)/141 = **53%**
  (era 55-56%).

**§7 Top divergências entrada 7**: actualizada com referência
ao diagnóstico P156B + ADR-0061 PROPOSTO + DEBT-56 + plano
P156C com desbloqueio de footnote.

**Cross-references**: adicionados ADR-0060, ADR-0061, ADR-0062
(reservada), Passos 156A/156B, DEBTs 53/54/55/56.

---

## §12 — README ADRs actualizado

Ficheiro: `00_nucleo/adr/README.md`.

- **Cabeçalho**: total 60 → **61** ADRs (60 números únicos);
  reservas ADR-0062 (hayagriva) e ADR-0063 (futuro) documentadas.
- **Tabela "Estado por ADR"**: linha ADR-0060 anotada com
  renumeração; linha **ADR-0061 PROPOSTO** adicionada.
- **Distribuição**: `PROPOSTO` 10 → **11**; total 60 → **61**.
- **Passos-chave**: entradas adicionadas para **P156A**
  (historiograma) e **P156B** (diagnóstico Layout) com detalhe
  da reocupação ADR-0061 + reserva ADR-0062 + DEBT-56 +
  renumeração Model Fase 2.

---

## §13 — ADR-0060 renumeração registada

Ficheiro: `00_nucleo/adr/typst-adr-0060-model-structural-roadmap.md`.

**Anotações adicionadas**:
- Cabeçalho: status anotado com referência à renumeração
  (`P157/158/159` em vez de `P156/157/158`).
- Anotação nova "Passo 156B (2026-04-25)": tabela de renumeração
  + reocupação ADR-0061 + reserva ADR-0062 + bloqueio footnote
  desbloqueado por Fase 1 Layout (P156C).
- Decisão 1 (Fase 1 Model): `Passo 156` → **`Passo 157`**.
- Decisão 2 (Fase 2 Model): `Passo 157` → **`Passo 158`**;
  `ADR-0061 + Passo 158` → **`ADR-0062 + Passo 159`**.
- Tabela "Plano de materialização": renumerada com notas
  explícitas.
- Referências: DEBT-55 marcada como actualizada; **ADR-0061**
  e **DEBT-56** novos adicionados.

---

## §14 — DEBT-55 actualizada

Ficheiro: `00_nucleo/DEBT.md` (linha ~340).

- **Cabeçalho**: "pré-condição ADR-0061 hayagriva" →
  "pré-condição **ADR-0062** hayagriva"; nota
  "renumerada por Passo 156B" adicionada.
- **Bloqueado por**: ADR-0061 → **ADR-0062** (com nota de
  reocupação).
- **Pré-requisitos**: ponto 1 actualizado.
- **Plano**: passo dedicado **P159** (era P158); ADR-0062
  (era ADR-0061).
- **Critério de fecho**: ADR-0062 (era ADR-0061).
- **Notas**: P157/P158 (era P156/P157); ADR-0062 (era
  ADR-0061); reformulação P156B documentada.

**Reserva ADR-0062 (hayagriva)**: documentada explicitamente
em README ADRs §"Reservas de números".

---

## §15 — Próximo passo

P156B encerrou-se com 1 ADR criada (PROPOSTO), 1 DEBT aberto,
e 4 documentos actualizados. Próximo passo é **decisão humana**
entre prioridades agora explicitamente documentadas:

- **Opção A — P156C (Fase 1 Layout)**: M+ agregado; page model
  footnote area + pad + hide + pagebreak + h + v. **Desbloqueia
  `footnote()`** em Model Fase 2. Recomendado pelo padrão
  diagnóstico-primeiro (par 156B → 156C análogo a 154A → 154B).

- **Opção B — P157 (Model Fase 2 table foundations)**: M+;
  primeira materialização Fase 2 ADR-0060. Não desbloqueia
  footnote (continua à espera de Layout Fase 1).

- **Opção C — Trabalho misto**: alternar entre P156C e P157
  por sessões. Defensável dado que ambos têm escopo M+ e são
  independentes.

- **Opção D — Outra prioridade humana**: introspection,
  paridade (DEBT-54), refino documental.

**Recomendação descritiva** (derivada do historiograma P156A
§4.1): aplicar diagnóstico-primeiro tem retorno alto consistente
(6/6 aplicações). P156C é a materialização natural deste
diagnóstico (par formal A→B) com janela curta entre A e B
(precedente: 131A→B, 132A→B, 140A→B+141, 154A→B+155 todos
≤1 dia entre A e B).

**Notar**: P156C **inclui no seu escopo a sub-fase mínima
desbloqueante de footnote** (page model + footnote area).
Decisão humana sobre se atacar Fase 1 Layout completa
(P156C como definido) ou apenas a sub-fase footnote area
isolada (P156C reduzido) é também possível.

---

## §16 — Verificação final

Critérios da spec P156B (§Verificação):

1. ✅ Diagnóstico em
   `00_nucleo/diagnosticos/diagnostico-layout-passo-156b.md`
   com 8 secções factuais (mais §9 cross-references).
2. ✅ ADR-0061 criada em status `PROPOSTO` (cabeçalho canónico
   P145; ⚖️, Status com backticks, Data, Validado, Diagnóstico
   prévio).
3. ✅ Cada elemento Layout classificado com referência canónica
   (~22 elementos vanilla + ~18 user-facing entradas).
4. ✅ Bloqueantes arquitecturais identificados (15 itens com
   custo + decisão arquitectural).
5. ✅ Crates externas listadas (**nenhuma** específica de
   layout — confirmado empiricamente).
6. ✅ Priorização ranqueada (matriz custo × valor; 13 features
   distribuídas em 3 fases).
7. ✅ Plano de materialização com 3-5 passos sugeridos
   (P156C + Layout F2 + Layout F3 columns + Layout F3 visuais
   + Layout refino Page).
8. ✅ Sub-fase Fase 1 que desbloqueia footnote identificada
   explicitamente (page model com `footnote_area`).
9. ✅ DEBT-56 aberto consoante critério (column flow L+;
   bloqueia Fase 3).
10. ✅ Inventário 148 actualizado: Tabela A.5 (4
    reclassificações + 2 adições); Tabela A linha "Layout"
    (16 → 18); §7 entrada 7; cross-references.
11. ✅ README ADRs actualizado: ADR-0061 na tabela; total 60
    → 61; PROPOSTO 10 → 11; reservas ADR-0062/0063 documentadas;
    Passos-chave entradas P156A e P156B.
12. ✅ ADR-0060 renumeração registada (P156→P157, P157→P158,
    P158→P159; ADR-0061→ADR-0062 para hayagriva).
13. ✅ DEBT-55 actualizada (P159 + ADR-0062).
14. ✅ Reserva ADR-0062 (hayagriva) registada em README ADRs
    + DEBT-55 + ADR-0060.
15. ✅ Nenhum ficheiro tocado em L1/L2/L3/L4 (verificado:
    apenas ficheiros em `00_nucleo/diagnosticos/`,
    `00_nucleo/adr/`, `00_nucleo/materialization/` e
    `00_nucleo/DEBT.md`).
16. ✅ Nenhum ficheiro tocado em `lab/parity/` (apenas leitura
    de `lab/typst-original/`).
17. ⏸ `cargo test --workspace --lib`: **não corrido** (passo
    L0-puro; nenhum código tocado; análogo a P142, P145, P156A).
    Estado declarado: **1145 inalterado**.
18. ⏸ `crystalline-lint .`: **não corrido** (idem). Estado
    declarado: **zero violations**.
19. ✅ Relatório do passo escrito (este ficheiro).

**Critérios 17-18**: justificação para não-execução —
P156B não tocou nenhum ficheiro `.rs`, `.toml`, ou similar.
Apenas ficheiros markdown em `00_nucleo/`. A verificação cargo
test + lint só faz sentido se houvesse mudança de código a
auditar; passos administrativos análogos (P142, P145, P156A)
seguem o mesmo padrão. Resultado esperado se o utilizador
correr manualmente: idêntico ao pré-P156B.

---

## §17 — Notas operacionais

- **Modelo: diagnóstico-primeiro a categoria Layout**.
  **Oitava aplicação** do padrão (após 131A/132A/140A/148/
  154A/156A/156B). Primeira aplicação a Layout. Pacote de
  evidência empírica do historiograma P156A §4.1 informou
  esta aplicação.

- **ADR-0061 é a primeira ADR pós-historiograma P156A**
  (e a primeira ADR pós-fechar Fase 1 Model em P155). Status
  `PROPOSTO` reservado até primeira sub-fase materializar
  (P156C). Modelo análogo a ADR-0055 + Passos 140A/140B/141
  e ADR-0060 + Passos 154A/154B/155.

- **ADR-0062 reservada** para autorização `hayagriva` quando
  bibliography for atacada (P159 ainda futuro). ADR-0063
  reservada para outra crate específica se surgir (e.g. column
  flow algorithm pode usar este número).

- **Renumeração ADR-0060**: P156→P157, P157→P158, P158→P159.
  Decisão registada explicitamente em ADR-0060 (anotação
  P156B), README ADRs, DEBT-55, e este relatório. Risco baixo
  de confusão se documentação reflectir consistentemente.

- **DEBTs vs items de roadmap**: critério explícito (Decisão
  diferida 11 da spec). DEBT-56 aberto para column flow (L+;
  exige refactor + ADR dedicada). Restantes 12 entradas Layout
  são items de roadmap (ADR-0061), não DEBTs.

- **Sem código tocado**: verificado por inspecção de
  `git status`. Apenas ficheiros markdown em `00_nucleo/`
  modificados.

- **Volume**: diagnóstico tem ~520 linhas (dentro da janela
  segura <800 da spec); ADR-0061 tem ~210 linhas; este
  relatório tem ~370 linhas. Total ~1100 linhas markdown
  novas.

- **Coexistência com Model Fase 2**: P156B não bloqueia P157
  (Model Fase 2 table foundations renumerado). Decisão humana
  posterior:
  - Atacar Layout primeiro (P156C) desbloqueia footnote
    indirectamente.
  - Atacar Model Fase 2 directamente (P157) cobre table
    mas footnote permanece bloqueado.
  - Atacar ambos em paralelo (P156C + P157 alternados) é
    defensável.

- **Pós-156B**:
  - Diagnóstico Layout disponível.
  - ADR-0061 PROPOSTA (Layout roadmap).
  - DEBT-56 aberto (column flow Fase 3).
  - ADR-0062 reservada (hayagriva).
  - DEBT-55 actualizada.
  - ADR-0060 renumerada.
  - Inventário 148 + README ADRs actualizados.
  - **Próximo substantivo**: P156C (Fase 1 Layout) ou P157
    (Model Fase 2 table) ou outra prioridade humana.

- **Quarentena vanilla**: continua opção 3. Sem mudança
  neste passo.

- **Série paridade**: continua suspensa em P153. Sem mudança
  neste passo.

- **Padrão emergente: passos por categoria múltiplos em
  paralelo**. Model Fase 2 (P157+) + Layout Fase X (P156B
  → P156C) podem co-existir. Historiograma P156A §3.3 mostra
  que clusters temáticos densos funcionam; alternância entre
  clusters pode ser consequência natural da escolha humana.

- **Pós-roadmap completo Layout**: cobertura ~67% (Fase 2)
  ou ~94-100% (Fase 3 completa), alinhada com a ambição
  declarada do projecto sem comprometer ADR-0017 (estratégia
  gradual).

---

## §18 — Cross-references

- Spec: `00_nucleo/materialization/typst-passo-156b.md`.
- Diagnóstico: `00_nucleo/diagnosticos/diagnostico-layout-passo-156b.md`.
- ADR criada: `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`.
- ADR anotada: `00_nucleo/adr/typst-adr-0060-model-structural-roadmap.md`.
- Reserva: ADR-0062 (sem ficheiro; documentada em README ADRs).
- DEBT aberto: `00_nucleo/DEBT.md` §DEBT-56.
- DEBT actualizada: `00_nucleo/DEBT.md` §DEBT-55.
- README ADRs actualizado: `00_nucleo/adr/README.md`.
- Inventário 148 actualizado:
  `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
- Historiograma motivador: `00_nucleo/diagnosticos/historiograma-passos.md`
  (§4.1 evidência 6/6 do padrão diagnóstico-primeiro).
- Vanilla source consultado:
  `lab/typst-original/crates/typst-library/src/layout/`.
- Cristalino source consultado: `01_core/src/entities/layout_types.rs`,
  `01_core/src/entities/content.rs`, `01_core/src/rules/layout/`,
  `01_core/src/rules/eval/mod.rs`.
