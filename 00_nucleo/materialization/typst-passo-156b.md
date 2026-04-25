# Passo 156B — Diagnóstico Layout Fase X: page model + multi-column + footnote area

**Série**: 156B (passo **L0-puro / diagnóstico-primeiro**;
**oitava aplicação** do padrão diagnóstico-primeiro;
**primeira aplicação** a categoria Layout do inventário 148).
**Precondição**: Passo 156A encerrado (historiograma);
ADR-0060 IMPLEMENTADO; Fase 1 Model fechada; 1145 tests; 60
ADRs; 13 DEBTs abertos; cobertura Layout **38%** (per
inventário 148 §3); footnote bloqueado em Model Fase 2 por
falta de page model footnote area (per diagnóstico 154A §4).

**Numeração**: `P156B` segue P156A (historiograma) na
convenção simples letras consecutivas. Precedentes do
projecto: 131A→131B, 132A→132B, 140A→140B, 154A→154B.

**Decisão administrativa registada**: ADR-0060 (Model Fase 2
roadmap) reservava P156 para "table foundations". Esta
reserva é **renumerada** consoante a sequência real:

| Antes (ADR-0060 original) | Depois (pós-P156B) |
|---------------------------|---------------------|
| P156 = Model table foundations | **P157** = Model table foundations |
| P157 = Model figure-kinds | **P158** = Model figure-kinds |
| P158 = Model bibliography (XL) | **P159** = Model bibliography (XL) |

Razão: P156A foi consumido pelo historiograma; P156B é este
passo (Layout). Renumeração mantém numeração contígua sem
saltos.

**ADR-0060** será actualizada com nota de renumeração.
**DEBT-55** (bibliography) actualizada para referenciar P159.

**Natureza**: passo **L0-puro / administrativo / diagnóstico
+ ADR proposta**. **Zero código**. **Zero testes**.
**1 ADR proposta** (status `PROPOSTO`); **0-3 DEBTs novos**
consoante arqueologia. **Possível actualização ao inventário
148** se diagnóstico revelar reclassificação.

**Justificação do passo (derivada do blueprint + análise da
árvore de módulos)**:

1. **Mais perto da raiz arquitectural**: page model vive em
   `01_core/src/entities/layout_types.rs`; multi-column flow
   exige extensão de tipos fundamentais (Frame, Page,
   PagedDocument).

2. **Maior fan-out conhecido**: ~10 features bloqueadas
   directamente — footnote (Model Fase 2), columns (Layout),
   stack (Layout), block (Layout), pad (Layout), hide
   (Layout), repeat (Layout), box (Layout), page break
   refinement (Layout), overflow strategy (Layout). Cobre 2
   categorias do inventário 148 (Layout 38% + parcial Model).

3. **Desbloqueia footnote sem precisar Model Fase 2 directo**:
   resolve a única feature bloqueada da Fase 1 Model
   estendida.

4. **Padrão diagnóstico-primeiro com retorno 6/6 (historiograma
   §4.1)**: aplicar é decisão derivada de evidência empírica.

**ADRs aplicáveis**:
- **ADR-0026** + **ADR-0026-R1** + **ADR-0038** —
  `Content` enum fechado; novos elementos exigem variants
  novas ou re-uso via `Content::Styled`.
- **ADR-0033** — paridade funcional para cada feature
  Layout materializada.
- **ADR-0034** — diagnóstico obrigatório (cumprido por este
  passo).
- **ADR-0036** — atomização progressiva (cada feature
  Layout terá consumer explícito).
- **ADR-0037** — coesão por domínio (Layout permanece em
  `rules/layout/` e `entities/layout_types.rs`).
- **ADR-0054** — perfil observacional graded (features
  Layout aceitas em forma aproximada se cobertura básica é
  cumprida).

---

## Contexto

Diagnóstico Model 154A revelou que **footnote** está
bloqueado por **page-model footnote area** — trabalho de
Layout Fase X (categoria 38% também). Inventário 148 §3
lista Layout em **16 entradas** com distribuição:

| Estado | Contagem | Notas |
|--------|----------|-------|
| `implementado` | 6 | paragraph, alignment, spacing, paged document, font dispatch (P140B/141/146), hyphenation (P144) |
| `implementado⁺` | 0 | — |
| `parcial` | 2 | a confirmar empiricamente |
| `ausente` | 8 | a confirmar (block, columns, stack, hide, repeat, pad, box, e provavelmente outras) |
| `scope-out` | 0 | — |

Cobertura **38%** (6/16). **Segundo gap maior** depois de
Introspection (17%). **Maior fan-out** entre as 3 categorias
fracas.

A pergunta primária do diagnóstico é: dadas as 16 entradas
de Layout, **qual o trabalho realmente necessário** para
desbloquear footnote (Model Fase 2) e elevar cobertura
Layout para um valor target (60%? 80%?), em que ordem?

Pergunta secundária: que entradas exigem **decisão
arquitectural** (ADR) antes de materialização, e quais são
trabalho mecânico (consumer + tests)?

**Hipóteses a confirmar empiricamente** (não compromisso):

- **Page model**: actual cristalino tem `Page` simples com
  conteúdo único; multi-column exige refactor estrutural.
- **Footnote area**: requer reservar espaço no fim da página
  durante layout principal; não-trivial.
- **Column flow**: exige re-flow de Frame quando coluna
  enche; multi-column é trabalho L+.
- **Box/Block**: provavelmente granularidade fina; pode ser
  via `Content::Styled` (per ADR-0026 perfil) ou variant
  novo.
- **Pad/Hide**: triviais (S cada); structural simples.
- **Repeat**: estrutural simples mas com lazy semantic;
  pode exigir consumer especializado.
- **Stack**: composição de Frames; M.
- **Overflow strategy**: decisão arquitectural — truncate,
  ellipsis, page break automático? Vanilla tem semantic
  específico.
- **Page break refinement**: refino do existente, não
  variant novo.

**Estado actual confirmar**: alguns elementos podem estar
classificados como `ausente` mas existir em forma parcial
(análogo ao salto de 38% declarado para 32-36% empírico em
Model 154A). Diagnóstico tem que confirmar empiricamente.

---

## Objectivo

Ao fim do passo:

1. **Documento de diagnóstico** em
   `00_nucleo/diagnosticos/diagnostico-layout-passo-156b.md`
   com 8 secções:

   1. **Inventário detalhado** — explodir 16 entradas em
      sub-features (atributos, métodos, regras `#show`
      esperadas). Confronto com vanilla
      `lab/typst-original/crates/typst-library/src/layout/`.
   2. **Estado actual em cristalino** — para cada entrada e
      sub-feature: existe? `parcial`? `ausente`? Referência
      canónica (Passo / ADR).
   3. **Tipos arquitecturais bloqueantes** — para cada
      entrada `parcial` ou `ausente`: que tipos faltam ou
      estão incompletos. Atenção especial a `Frame`,
      `Page`, `PagedDocument` em
      `01_core/src/entities/layout_types.rs`.
   4. **Arqueologia das ausências** — para cada `ausente`:
      passo materializador esperado nunca aconteceu, ou foi
      adiado por ADR? Razões registadas (ou ausência
      declarada).
   5. **Crates externas necessárias** — provavelmente
      nenhuma (Layout é trabalho L1 puro), mas confirmar.
      Particularmente: column-flow algorithms; bidi (já
      coberto); page break heuristics.
   6. **Priorização proposta** — matriz custo (S/M/L/XL) ×
      valor user-facing (alto/médio/baixo). 5-8 entradas
      ranqueadas. **Especial atenção**: identificar
      sub-fase mínima que desbloqueia footnote
      (page-model footnote area).
   7. **Plano de materialização** — N passos sugeridos com
      escopo cumulativo. Inclui possível passo dedicado
      ADR para escolhas arquitecturais (ex: column flow
      algorithm).
   8. **Resumo executivo** — 2-3 parágrafos.

2. **ADR proposta** em
   `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`
   com status `PROPOSTO`:
   - **Cabeçalho canónico P145** (`⚖️`,
     `**Status**: \`PROPOSTO\``,
     `**Validado**: Passo 156B — diagnóstico`).
   - Decisão de roadmap: priorização ranqueada do
     diagnóstico.
   - Decisão sobre re-uso de `Content::Styled` vs novos
     `Content::*` variants para Layout.
   - Plano de materialização com 3-5 passos (ranqueamento
     S/M; XL adiados).
   - **Sub-fase prioritária explícita**: footnote area
     mínima (sub-fase A) deve ser materializável
     independentemente do resto.
   - Alternativas consideradas (atacar tudo de uma vez vs
     sub-fases vs adiamento total).
   - Consequências.

   **Reserva ADR-0061**: estava reservada para
   `hayagriva` autorização (per blueprint + relatório
   154A). Reocupação é decisão deste passo. Razão:
   `hayagriva` autorização fica para Fase 2 Model
   bibliography (P159 ainda futuro, era P158 antes desta
   renumeração); Layout precisa de ADR já. Passo dedicado
   bibliography pode usar ADR-0062 quando chegar.

3. **Possíveis DEBTs novos** (0-3 candidatos):
   - **DEBT-XX (L+)**: column flow algorithm — escopo L
     no mínimo se materializado integralmente.
   - **DEBT-YY**: outras features adiadas conforme
     arqueologia.
   - Critério para abrir DEBT vs deixar como item do
     roadmap: **DEBT** se trabalho exige trabalho
     dedicado >5h e bloqueia outras features; **item de
     roadmap** se trabalho é mecânico e cabe em passo
     simples.

4. **Inventário 148 actualizado** se diagnóstico revelar
   reclassificação:
   - Tabela A linha "Layout": ajustar contagens se
     reclassificações justificadas.
   - Tabela C: adicionar bloqueantes detectados.
   - §7 Top divergências: actualizar se relevante.

5. **README dos ADRs actualizado**:
   - Tabela "Estado por ADR".
   - Distribuição: `PROPOSTO` 10 → 11.
   - Total: 60 → 61.
   - Entrada em "Passos-chave da história dos ADRs" para
     P156B.

6. **ADR-0060 actualizada** com nota de renumeração:
   passos de Model Fase 2 deslocam de P156/157/158 para
   P157/158/159.

7. **DEBT-55 actualizada**: referência ao passo de
   bibliography muda de P158 para P159; pré-condição muda
   de ADR-0061 para ADR-0062.

8. **Reserva de ADR-0062** documentada: bibliography
   `hayagriva` autorização passa de ADR-0061 (reservado
   antes) para **ADR-0062** (novo reservado).

9. **Relatório do passo** em
   `00_nucleo/materialization/typst-passo-156b-relatorio.md`.

Este passo **não**:

- Toca código em L1/L2/L3/L4.
- Toca testes.
- Implementa qualquer feature de Layout.
- Materializa ADR-0061 como `IMPLEMENTADO`.
- Resolve footnote ou outras features bloqueadas.
- Toca série paridade (suspensa em P153).
- Decide entre opções de Fase 2 Model (table/figure-kinds/
  bibliography) — separado.
- Importa crates externas.
- Materializa column flow ou page model novo.

---

## Decisões já tomadas

1. **Diagnóstico-primeiro**, padrão estabelecido com
   evidência 6/6 (historiograma §4.1). Oitava aplicação;
   primeira aplicada a Layout.

2. **Numeração P156B** seguindo precedentes (131A→131B,
   132A→132B, 140A→140B, 154A→154B). Materialização
   posterior será P156C ou P157+ consoante diagnóstico.

3. **Renumeração de ADR-0060** (Model Fase 2): P156 →
   P157, P157 → P158, P158 → P159. Documentada neste
   passo.

4. **ADR proposta `PROPOSTO`** — não `EM VIGOR` nem
   `IMPLEMENTADO`. Materialização em passos posteriores.

5. **Zero código tocado**.

6. **Reserva ADR-0061 alterada** de `hayagriva` para
   Layout roadmap. `hayagriva` passa para ADR-0062.

7. **Footnote area como sub-fase prioritária** explícita —
   desbloquear footnote é objectivo declarado.

## Decisões diferidas (resolvidas neste passo)

8. **Target de cobertura post-roadmap**: 60%? 80%?
   Aspiração registada em ADR-0061. Default: aspirar
   60-70% sem comprometer-se a número exacto.

9. **Forma de page model com footnote area**:
   - Opção A: `Page { content, footnote_area:
     Option<Frame> }` — extensão de struct existente.
   - Opção B: novo tipo `PageWithFootnotes` separado.
   - **Default A** — extensão minimalista; backward-
     compatible com Frame existente.

10. **Column flow**: incluir em Fase 1 Layout ou separar?
    - Decisão default: **separar** (Fase 2 ou 3 Layout).
      Footnote pode funcionar com single-column inicialmente.

11. **Re-uso de `Content::Styled` para Layout**: ADR-0026
    perfil aceita ambos. Decisão por feature em diagnóstico:
    - Block, Box, Pad, Hide → provavelmente
      `Content::Styled` (semantic distintos mas pequena).
    - Columns, Stack, Repeat → provavelmente variants novos.
    - Footnote → provavelmente variant novo (semantic
      complexa).

12. **Conflito reserva ADR-0061**: alterar é decisão deste
    passo. ADR-0062 reservada para `hayagriva` registada
    em README ADRs + DEBT-55 (que mencionava ADR-0061;
    actualizar).

13. **Possibilidade de Layout Fase X ser >3 sub-fases**:
    aceitável. Diagnóstico decide. Comparação com Model
    Fase 1 (3 sub-fases): Layout pode ter mais por
    granularidade (footnote isolada; columns isolada;
    pad/hide/box agregados).

14. **Outros gaps de raiz** (Content::Table, Introspection,
    Numbering rico, Value::Regex): registados como
    candidatos posteriores. **Não atacados** neste passo.

---

## Escopo

**Dentro**:

- Leitura de
  `lab/typst-original/crates/typst-library/src/layout/`
  para inventário detalhado.
- Leitura de `01_core/src/entities/layout_types.rs`,
  `01_core/src/rules/layout/mod.rs` e ficheiros relacionados.
- Cross-reference com `00_nucleo/adr/`,
  `00_nucleo/materialization/`, `00_nucleo/DEBT.md`.
- Escrita do diagnóstico (com 8 secções).
- Escrita de ADR-0061 (PROPOSTO; Layout roadmap).
- Possível abertura de 1-3 DEBTs.
- Actualização do inventário 148.
- Actualização do README ADRs.
- Actualização de ADR-0060 (renumeração Model Fase 2).
- Actualização de DEBT-55 para reflectir P159 + ADR-0062.
- Relatório do passo.

**Fora**:

- Modificação de código em L1/L2/L3/L4.
- Modificação de testes.
- Materialização de feature Layout.
- Importação de crates novas.
- Materialização de ADR-0061 (PROPOSTO permanece).
- Decisão final sobre target de cobertura específico.
- Modificação de outros ADRs (excepto ADR-0060
  renumeração e reserva ADR-0062 documentada).
- Trabalho na série paridade.
- Trabalho em Model Fase 2 (table foundations).
- Materialização de footnote em Model Fase 2.

---

## Sub-passos

### 156B.1 — Inventário detalhado

**A.1.1 — Listar Layout vanilla**:

```bash
ls lab/typst-original/crates/typst-library/src/layout/
```

Esperado: ~32 ficheiros conforme inventário 148 §2.1.
Categorias prováveis: align/, columns/, grid/, page/,
pad/, place/, repeat/, stack/, ...

**A.1.2 — Para cada ficheiro, extrair elementos**:

```bash
for f in lab/typst-original/crates/typst-library/src/layout/*.rs; do
  echo "=== $f ==="
  grep -E "^#\[elem\]|^pub struct \w+Elem|^pub fn \w+_elem" "$f" | head -10
done
```

Registar tabela:

| Elemento | Ficheiro | Atributos públicos | Métodos públicos | `#show` rules suportadas |
|----------|----------|---------------------|-------------------|--------------------------|
| `BlockElem` | block.rs | width, height, ... | ... | show block: ... |
| ... | ... | ... | ... | ... |

**A.1.3 — Confronto com cristalino**:

```bash
view 01_core/src/entities/content.rs   # já 43 variants pós-P155
view 01_core/src/entities/layout_types.rs
grep -nE "Frame|Page|PagedDocument" 01_core/src/entities/layout_types.rs
grep -nE "fn layout_(block|columns|stack|hide|pad|repeat|box|footnote)" \
  01_core/src/rules/layout/
```

Para cada elemento vanilla, registar correspondente
cristalino (existe? `parcial`? `ausente`?) e justificar.

**A.1.4 — Page model actual**:

```bash
view 01_core/src/entities/layout_types.rs
grep -nE "struct Page\b|struct PagedDocument" 01_core/src/entities/layout_types.rs
```

Confirmar:
- Forma actual de `Page` (provavelmente single-content).
- Forma actual de `PagedDocument`.
- Como página é construída no layouter.
- Onde page break acontece em `01_core/src/rules/layout/`.

### 156B.2 — Estado actual cristalino

Tabela final por elemento:

| Elemento | Cristalino estado | Cobertura sub-features | Bloqueantes |
|----------|-------------------|------------------------|-------------|
| paragraph | implementado | leading/justify/indent OK | — |
| alignment | implementado | left/center/right/horizon OK | — |
| spacing | implementado | h(), v() OK | — |
| paged document | implementado | basic page model OK | multi-column ausente |
| font dispatch | implementado | P140B/141/146 | — |
| hyphenation | implementado⁺ | P144 (ADR-0057) | shaping ausente (DEBT-53) |
| footnote area | **ausente** | — | bloqueante crítico |
| block | **ausente** | — | width, height, breakable |
| columns | **ausente** | — | column flow algorithm |
| stack | **ausente** | — | direction (vertical/horizontal) |
| hide | **ausente** | — | structural simples |
| pad | **ausente** | — | structural simples |
| repeat | **ausente** | — | lazy semantic |
| box | **ausente** | — | inline container |
| page break | parcial | só implícito por overflow | manual `pagebreak()` ausente |
| overflow strategy | parcial | truncate? | semantic vanilla a confirmar |

### 156B.3 — Tipos arquitecturais bloqueantes

Para `parcial`/`ausente` em §B.2, listar tipos faltantes:

| Bloqueante | Quem precisa | Custo estimado | Decisão arquitectural? |
|------------|--------------|----------------|------------------------|
| `Page::footnote_area: Option<Frame>` | footnote | M | sim — extensão Page |
| `Content::Block` ou `Style::Block` | block | S-M | sim — variant ou Styled? |
| Column flow algorithm | columns | L+ | sim — ADR dedicada futura |
| `Content::Stack` | stack | S-M | sim — variant |
| `Content::Hide` | hide | S | sim — variant trivial |
| `Content::Pad` | pad | S | sim — variant trivial |
| `Content::Repeat` | repeat | M | sim — lazy semantic |
| `Content::Box` | box | S-M | sim — variant ou Styled? |
| `pagebreak()` stdlib func | page break manual | S | trivial — função stdlib |
| Overflow strategy | layout overflow | M | depende de vanilla semantic |

### 156B.4 — Arqueologia das ausências

Para cada `ausente`, classificação per critério P149:

| Elemento | Razão | Classificação |
|----------|-------|---------------|
| footnote area | bloqueado por footnote (Model 154A); nunca materializado | candidato Fase 1 (sub-fase A) |
| block | sem registo de razão; granularidade fina | candidato Fase 2 |
| columns | trabalho L; column flow não trivial | adiamento priorizável (Fase 3 ou DEBT) |
| stack | sem registo; estrutural simples | candidato Fase 2 |
| hide | sem registo; trivial | candidato Fase 1 ou 2 (incluir com pad) |
| pad | sem registo; trivial | candidato Fase 1 ou 2 |
| repeat | semantic lazy; menos usado | candidato Fase 3 |
| box | sem registo; granularidade fina | candidato Fase 2 |
| page break manual | infrastructural; trivial | candidato Fase 1 ou 2 |
| overflow strategy | refino vanilla complexo | candidato refino futuro |

### 156B.5 — Crates externas

Confirmar empiricamente. Hipótese: nenhuma crate externa
necessária. Layout é trabalho L1 puro. Confirmar:

- Page break heurísticas: nenhuma crate; algoritmo simples.
- Column flow: trabalho próprio; sem dependência.
- Bidi (já coberto): `unicode-bidi` já em deps cristalino.
- Footnote area: nenhuma crate; trabalho próprio.

Se confirmado, **nenhuma ADR de autorização** necessária.
Se algum elemento exigir crate (improvável), registar como
DEBT futura.

### 156B.6 — Priorização proposta (matriz custo × valor)

```
              Alto valor              Médio valor          Baixo valor
S       [F1: pad/hide,           [F2: pagebreak]      [F3: stroke-obj]
         page break manual]
M       [F1: footnote area]     [F2: stack, box]    [F3: repeat]
M+      [F2: block]
L       [F3: columns]
XL      [—]
```

### 156B.6.1 — Fase 1 proposta (sub-fase mínima que
desbloqueia footnote)

**Decisão default**:

1. **Page model com footnote area** (M, alto valor —
   crítico): extensão de `Page::footnote_area:
   Option<Frame>`. Layouter reserva espaço; footnote em
   Model Fase 2 popula.
2. **Pad** (S, alto valor — trivial): `Content::Pad`.
3. **Hide** (S, médio valor — trivial): `Content::Hide`.
4. **Page break manual** (S, alto valor): `pagebreak()`
   stdlib + Style::PageBreak.

**Aspiração de cobertura post-Fase 1**: 6/16 → 10/16 =
**62%**.

**Footnote desbloqueado** após esta sub-fase (Model Fase
2 pode então abrir passo de footnote sem aguardar mais
Layout).

### 156B.6.2 — Fase 2 proposta

5. **Block** (M+, alto valor): `Content::Block` ou
   `Content::Styled` per ADR-0026.
6. **Stack** (S-M, médio valor): `Content::Stack` com
   direção.
7. **Box** (S-M, médio valor): inline container.

**Aspiração post-Fase 2**: 10/16 → 13/16 = **81%**.

### 156B.6.3 — Fase 3 proposta (condicional)

8. **Columns** (L, alto valor mas complexo): column flow
   algorithm. Possível DEBT dedicado.
9. **Repeat** (M, baixo valor): lazy semantic.
10. **Refino overflow strategy**.

**Aspiração total**: 13/16 → 15-16/16 = **94-100%**.

### 156B.7 — Plano de materialização

#### A.7.1 Sub-passos sugeridos

Numeração sequencial pós-P156B (sem conflito com Model
Fase 2 renumerada para P157+):

1. **Passo 156C** — Page model com footnote area + pad +
   hide + page break manual (M agregado; Fase 1 Layout).
   - Forma: 4 features Fase 1 num passo (análogo a P154B
     que materializou terms+divider em S agregado).
   - Crítico: footnote area é a sub-fase mínima que
     desbloqueia Model Fase 2 footnote.

**Após P156C**, sequenciamento humano possível:

- **P157** — Model Fase 2 table foundations (per ADR-0060
  renumerada).
- **P158** — Model figure-kinds (depende de P157).
- **P159** — Model bibliography + ADR-0062 (XL; depende
  de DEBT-55 fechar).
- **Passo dedicado footnote** — Model Fase 2 footnote
  (desbloqueado por P156C); número a decidir.
- **Passo Layout Fase 2** — block + stack + box; número a
  decidir.
- **Passo Layout Fase 3** — columns (L); possivelmente
  DEBT dedicado.

Numeração final fica para a ADR-0061 + decisão humana
posterior.

#### A.7.2 Regra Content::Styled vs variant novo (ADR-0026)

Para cada feature da Fase 1/2/3:

| Feature | Recomendação | Razão |
|---------|--------------|-------|
| footnote area | **Page::footnote_area** extensão | extensão Page; não Content variant |
| pad | **`Content::Styled` ou variant** | structural simples; decidir empiricamente |
| hide | **variant novo** | semantic distinta (display: none) |
| page break manual | **Style::PageBreak** | é estilo, não conteúdo |
| block | **variant ou Styled** | granularidade fina; decisão arquitectural |
| stack | **variant novo** | atributos não reduzíveis a Style |
| box | **variant ou Styled** | granularidade fina |
| columns | **variant novo** | column flow exige consumer dedicado |
| repeat | **variant novo** | lazy semantic |

Decisões finais ficam para cada passo de materialização;
ADR-0061 fornece guia.

#### A.7.3 Relação com ADRs existentes

- **ADR-0026 + ADR-0026-R1**: `Content` enum fechado;
  novos variants exigem nova entrada. ADR-0061 propõe.
- **ADR-0036**: atomização — cada feature consumer
  explícito.
- **ADR-0037**: coesão por domínio — Layout permanece em
  `rules/layout/`.
- **ADR-0054**: perfil observacional graded — features
  Fase 1 cumprem com aproximações aceites (e.g. footnote
  area mínima sem column flow).

### 156B.8 — Resumo executivo

2-3 parágrafos no diagnóstico:

> Layout (categoria 38% — segunda mais fraca) tem 16
> entradas vanilla; cristalino tem ~6 implementadas.
> Diagnóstico revela X entradas como Fase 1 viável, Y
> como Fase 2 dependente, Z como Fase 3 condicional.
>
> Ataque proposto: 4 passos (P156C → outros) elevam
> cobertura para ~94% sem novas crates. Sub-fase Fase 1
> (page model + footnote area + pad + hide + page break)
> desbloqueia footnote em Model Fase 2 sem requerer todo
> o resto.
>
> Trabalho restante (~2-3 entradas Fase 3) é condicional
> ou L; registado no roadmap mas não obriga a executar.
> **DEBT-XX** (column flow) pode ser aberto se Fase 3 for
> priorizada.

### 156B.9 — Escrever ADR-0061

Cabeçalho:

```markdown
# ⚖️ ADR-0061: Layout Fase X — page model + multi-column +
footnote area roadmap

**Status**: `PROPOSTO`
**Data**: (data execução)
**Validado**: Passo 156B — diagnóstico.
**Diagnóstico prévio**:
[`00_nucleo/diagnosticos/diagnostico-layout-passo-156b.md`](...)
```

Estrutura (~150-200 linhas): Contexto, Decisão (5 itens —
Fase 1 features, Fase 2 features, Fase 3 condicional,
regra Content::Styled vs variant, número de passos),
Alternativas (tabela), Consequências, Plano de
materialização (4 sub-passos), Referências (ADRs 0017,
0026, 0026-R1, 0033, 0034, 0036, 0037, 0038, 0054, 0060).

**Reserva ADR-0061** alterada de `hayagriva` para Layout.
Documentar a alteração explicitamente:

```markdown
**Nota**: ADR-0061 estava reservada para autorização
`hayagriva` (per blueprint + relatório 154A); reocupada por
este passo. **`hayagriva` passa a reserva ADR-0062**.
DEBT-55 actualizado.
```

### 156B.10 — Possíveis DEBTs novos

Critério explícito (per Decisão diferida 12 — análogo a
154A):

- **Columns + column flow algorithm** → **DEBT-NN
  (próximo número)**: "Column flow Fase 3 Layout — L+;
  exige algoritmo de re-flow". Aberto neste passo se Fase
  3 for marcada como condicional. Plano: ADR dedicada +
  passo dedicado se priorizado.

- **Outros candidatos** identificados durante 156B.4-6:
  conforme arqueologia.

DEBT-55 (bibliography) actualizada para reflectir nova
reserva ADR-0062 + passo P159.

### 156B.11 — Actualizar inventário 148

Em
`00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

- **Tabela A linha "Layout"**: ajustar contagens se
  diagnóstico revelar reclassificação. Esperado:
  reclassificação para baixo similar a Model 154A
  (38% → ~30-35% real).
- **Tabela C**: adicionar bloqueantes detectados (footnote
  area; column flow; etc.).
- **§7 Top divergências**: actualizar se relevante.

### 156B.12 — Actualizar README ADRs

(Se ADR-0061 criada, esperado.)

- Tabela "Estado por ADR": linha nova ADR-0061
  (`PROPOSTO`).
- Distribuição: `PROPOSTO` 10 → 11.
- Total: 60 → 61.
- Entrada em "Passos-chave" para P156B.
- Reserva ADR-0062 (hayagriva) registada.

### 156B.13 — Actualizar ADR-0060 + DEBT-55

**ADR-0060** (Model Fase 2 roadmap):

```diff
**Plano de materialização**:
- 1. Passo 156 — table foundations
- 2. Passo 157 — figure-kinds extension
- 3. Passo 158 — bibliography + cite (ADR-0061 autorização)
+ 1. Passo 157 — table foundations
+    (renumerado pós-P156B; era P156)
+ 2. Passo 158 — figure-kinds extension
+    (renumerado pós-P156B; era P157)
+ 3. Passo 159 — bibliography + cite (ADR-0062 autorização)
+    (renumerado pós-P156B; era P158; ADR-0062 reservada
+    em P156B; era ADR-0061)
```

Anotação:

```markdown
**Anotação Passo 156B (data execução)**: renumeração
de Fase 2. P156A foi historiograma; P156B é diagnóstico
Layout; consequentemente Fase 2 Model desloca-se para
P157/158/159. ADR-0061 reocupada para Layout roadmap;
hayagriva passa para ADR-0062.
```

**DEBT-55** (Bibliography + cite):

```diff
**Plano**: ADR-0061 + passo dedicado em Fase 2 Model
- (P158)
+ **Plano**: ADR-0062 + passo dedicado em Fase 2 Model
+ (P159; renumerado pós-P156B)
+ **Pré-condição**: ADR-0062 autorização hayagriva
+ (era ADR-0061; reocupada por Layout em P156B).
```

### 156B.14 — Relatório do passo

Ficheiro:
`00_nucleo/materialization/typst-passo-156b-relatorio.md`.

Secções:
1. Sumário.
2. Inventário detalhado (resumo da Tabela 156B.1).
3. Estado actual cristalino (resumo 156B.2).
4. Bloqueantes arquitecturais (resumo 156B.3).
5. Arqueologia (resumo 156B.4).
6. Crates externas (resumo 156B.5; provavelmente
   "nenhuma").
7. Priorização (matriz 156B.6).
8. Plano de materialização proposto (referência a
   ADR-0061).
9. ADR-0061 produzida.
10. DEBTs novos (se houver).
11. Inventário 148 actualizado.
12. README ADRs actualizado.
13. ADR-0060 renumeração registada.
14. DEBT-55 actualizada.
15. Próximo passo: 156C (Fase 1 Layout, primeira
    sub-fase) ou outra prioridade humana.
16. Verificação final.

---

## Verificação

1. ✅ Diagnóstico em
   `00_nucleo/diagnosticos/diagnostico-layout-passo-156b.md`
   com 8 secções factuais.
2. ✅ ADR-0061 criada em status `PROPOSTO` (cabeçalho
   canónico P145).
3. ✅ Cada elemento Layout classificado com referência
   canónica.
4. ✅ Bloqueantes arquitecturais identificados.
5. ✅ Crates externas listadas (provavelmente "nenhuma").
6. ✅ Priorização ranqueada (matriz custo × valor).
7. ✅ Plano de materialização com 3-5 passos sugeridos.
8. ✅ Sub-fase Fase 1 que desbloqueia footnote
   identificada explicitamente.
9. ✅ Possíveis DEBTs novos abertos consoante critério.
10. ✅ Inventário 148 actualizado se aplicável.
11. ✅ README ADRs actualizado.
12. ✅ ADR-0060 renumeração registada (P156→P157,
    P157→P158, P158→P159).
13. ✅ DEBT-55 actualizada (P159 + ADR-0062).
14. ✅ Reserva ADR-0062 (hayagriva) registada.
15. ✅ Nenhum ficheiro tocado em L1/L2/L3/L4.
16. ✅ Nenhum ficheiro tocado em `lab/parity/`.
17. ✅ `cargo test --workspace --lib`: 1145 inalterado.
18. ✅ `crystalline-lint .`: zero violations.
19. ✅ Relatório do passo escrito.

---

## Critério de conclusão

1. Cada uma das ~16 entradas Layout classificada com
   base empírica.
2. ADR-0061 propõe roadmap com Fase 1 / 2 / 3.
3. Sub-fase Fase 1 que desbloqueia footnote explicitamente
   identificada.
4. DEBTs novos abertos só consoante critério explícito.
5. Reserva ADR-0062 (hayagriva) documentada;
   DEBT-55 actualizada; ADR-0060 renumerada.
6. Próximo passo (156C ou outra prioridade) tem âncora
   documental.
7. Sem código tocado.
8. Relatório do passo escrito.

---

## O que pode sair errado

- **Inventário detalhado revela mais de 16 sub-features**:
  esperado. Cada elemento pode ter 5-15 atributos. Documento
  cresce; aceitável (~400-500 linhas).

- **Page model footnote area exige refactor profundo de
  layout**: hipótese. Se footnote area exige refactor de
  PagedDocument grande, ADR-0061 pode propor sub-fase
  inicial pequena (footnote area como Optional, sem
  ainda popular) seguida de passo dedicado para popular.
  **Decisão**: footnote area mínima.

- **Crates externas surgem (improvável)**: ex, vanilla
  usa biblioteca específica para column flow. Listar como
  DEBT e ADR autorização (análogo a hyphenation P144 +
  ADR-0057).

- **Sub-fase Fase 1 ainda não desbloqueia footnote**: ex,
  footnote depende não só de footnote area mas também de
  page break refinement não-trivial. Ajustar Fase 1 para
  incluir page break refinement; documentar dependência
  em ADR-0061.

- **Layout actual diverge de inventário 148**: ex,
  algumas features classificadas como `ausente` existem
  em forma parcial (análogo a Model 154A 38% → 32-36%).
  Reclassificar; actualizar inventário.

- **Conflito de reserva ADR-0061** com `hayagriva`: aceitável
  (decisão deste passo). DEBT-55 + README ADRs +
  ADR-0060 + blueprint actualizam para ADR-0062.
  **Verificar consistência** em todos os documentos.

- **Surge dependência circular**: ex, page break refinement
  depende de footnote area que depende de page model que
  depende de page break heuristic. Resolver com ordem
  topológica em 156B.7.

- **Diagnóstico conclui que Layout Fase X não é prioridade
  real**: improvável dado fan-out 10 features. Se
  acontecer, ADR-0061 documenta razão; Fase 1 fica
  condicional.

- **Volume excede passo único**: aceitável.
  Diagnóstico-primeiro pode crescer para 600+ linhas. Se
  exceder ~800 linhas, considerar dividir em
  156B.1 + 156B.2 (núcleo + specialized).
  **Pausar** em 800 linhas e consultar.

- **Algum elemento revelar-se já materializado**:
  classificação corrige-se; inventário 148 ganha
  reclassificação no relatório.

- **Footnote area "mínima" não é mínima**: descobrir que
  footnote area exige column-aware layout. Documentar e
  propor sub-fase ainda mais minimalista (e.g. apenas
  reservar Frame vazio; popular pode aguardar).

- **Renumeração ADR-0060 confunde leitores futuros**:
  documentar explicitamente em ADR-0060 anotação +
  README ADRs + DEBT-55.

---

## Notas operacionais

- **Modelo: diagnóstico-primeiro a categoria Layout**.
  **Oitava aplicação** do padrão (após 131A/132A/140A/148/
  154A/156A + este P156B). Primeira aplicação a Layout.

- **ADR-0061 é a primeira ADR pós-historiograma P156A**.
  Status `PROPOSTO` reservado até primeira sub-fase
  materializar (provavelmente P156C). Modelo análogo a
  ADR-0055 + Passos 140A/140B/141 e ADR-0060 + Passos
  154A/154B/155.

- **ADR-0062 reservada** para autorização `hayagriva`
  quando bibliography for atacada (P159 ainda futuro).
  ADR-0063 reservada para outra crate específica se
  surgir.

- **Numeração**: este é P156B; materialização sub-fase
  primeira será P156C (modelo P154A→P154B→P155 mas com
  três letras). Numeração final decidida em ADR-0061.

- **Renumeração ADR-0060**: P156→P157, P157→P158,
  P158→P159. Decisão registada explicitamente. Risco
  baixo de confusão se documentação reflectir
  consistentemente.

- **DEBTs vs items de roadmap**: critério explícito
  (Decisão diferida 11). Conservador: trabalho de Fase 1
  é roadmap; XL ou condicional vira DEBT.

- **Inventário 148 actualizado se necessário**: precedente
  P149 + P153 + P154A. Padrão coerente: novos
  diagnósticos refinam inventário com referências
  cruzadas.

- **Relação com Model Fase 2**: P156B não bloqueia P157
  Model Fase 2 table foundations (renumerado). Decisão
  humana posterior:
  - Atacar Layout primeiro (este passo + P156C)
    desbloqueia footnote indirectamente.
  - Atacar Model Fase 2 directamente (P157 table
    foundations) cobre table mas footnote permanece
    bloqueado.
  - Atacar ambos em paralelo (e.g. P156C + P157
    alternados) é defensável.

- **Pós-156B**:
  - Documento de diagnóstico Layout.
  - ADR-0061 PROPOSTA (Layout roadmap).
  - Eventual DEBT-NN (column flow Fase 3).
  - ADR-0062 reservada (hayagriva).
  - DEBT-55 actualizada.
  - ADR-0060 renumerada.
  - **Próximo substantivo**: P156C (Fase 1 Layout
    primeira sub-fase) ou P157 (Model Fase 2 table) ou
    outra prioridade humana.

- **Quarentena vanilla**: continua opção 3 (princípio sem
  regra absoluta). Sem mudança neste passo.

- **Série paridade**: continua suspensa em P153. Sem
  mudança neste passo.

- **Padrão emergente: passos por categoria múltiplos em
  paralelo**: Model Fase 2 (P157+) + Layout Fase X
  (P156B → P156C) podem co-existir. Historiograma §3.3
  mostra que clusters temáticos densos funcionam;
  alternância entre clusters pode ser consequência
  natural da escolha humana.

- **Pós-roadmap completo Layout**: cobertura ~80-95%,
  alinhada com a ambição declarada do projecto sem
  comprometer ADR-0017 (estratégia gradual).
