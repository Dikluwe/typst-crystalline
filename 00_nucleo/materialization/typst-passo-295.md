# Passo 295 — `Content::Footnote` cluster

**Frente**: `P-footnote-cluster` (rank 2 candidato P294 §10).
**Origem**: P282 §6 listou frente como #7 (M; Model 60% → 70%).
Tabela A.6 linha 178: `footnote(body)` em `model/footnote.rs` →
**ausente**. Tabela C linha 387: bloqueador duplo registado —
`Content::Footnote` + `locate runtime` adiados.
**Pré-requisitos**: A.0.0 verifica empiricamente o estado de
`locate`/`introspection` runtime (Tabela A.9 linha 228 sugere que
P208B+C já materializaram — **possível refutação significativa
da linha 387**).
**Tipo**: **frente ortogonal genuína de magnitude M** — primeira
desde término da série cumulativa P288-P292 + extensão P293-P294.
**Marco**: 2º passo ortogonal pós-série cumulativa **com magnitude
maior** que P293/P294.

---

## §1 — Objectivo

Materializar `footnote(body)` como entrada de primeira classe no
sistema, criando:

1. Variant novo `Content::Footnote { body, numbering: Option<...> }`
   (estrutura per A.2).
2. Função stdlib `native_footnote(body, ...)` (paralelo
   `native_cite` P159A).
3. Consumer Layouter — duas fases:
   - **Marker fase**: `[N]` superscript no fluxo de texto onde
     `#footnote[...]` aparece.
   - **Nota fase**: corpo da footnote renderizado no rodapé da
     página correspondente.
4. Integração com Introspector (P208C) — numeração automática
   incremental + tracking page-position.

Razão de ser:

1. **Frente recomendada P292 §9.2 rank #1 (cluster M)** —
   primeiro cluster M pós-série cirúrgica cumulativa P288-P294.
   Quebra natural do viés "magnitude pequena" das séries
   anteriores.
2. **Bloqueador histórico potencialmente desactualizado** —
   Tabela C linha 387 regista *"locate ADR-0017 adiada"* mas
   Tabela A.9 linha 228 mostra `here()`/`locate()` **implementado
   P208B+C (M9c)**. **A.0.0 obrigatória verifica empiricamente
   se Tabela C 387 está desactualizada**. Se sim, refutação
   significativa = **§8.3 N=7 candidato** (genuíno; conta-se
   sobre N=6 candidato adiado em P294 §7.1).
3. **Promoção condicional de ADR meta**:
   - **§8.3 refutação pragmática N=7** — candidato genuíno
     adiado desde P294 §7.1; P295 pode promover se refutação
     factual confirmada.
   - **§8.7' A.0.0 template N=3** — limiar tentativo se
     reaplicação é genuína (cluster M é qualitativamente
     diferente de XS-S anteriores).
   - **§8.6 A.5' anti-reflexão N=5** — limiar passado mas anti-
     padrão adia.
4. **Reaplicação ADR-0098** — N=12 se hash `export.rs`
   preservado (footnote consume via `FrameItem::Text` capture e
   layout-time numbering; emit não muda).
5. **Reaplicação ADR-0099** — N=11 se "activação posterior"
   aplicável (mas footnote é **materialização nova**, não
   activação — registar honestamente em A.5').

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória; 7 secções)

### A.0.0 — Verificação literal de bloqueador histórico (N=3 cumulativo §8.7')

**Crítico**: Tabela C linha 387 regista bloqueador duplo —
`Content::Footnote` + `locate runtime`. Tabela A.9 linha 228
contradiz a segunda parte (P208B+C materializaram).

Verificações literais obrigatórias:

1. **`grep -rn "fn locate\|here\|Introspector::query"
   01_core/src/`** — confirmar que `locate`/`here`/`query` estão
   funcionais.
2. **Inspeccionar `01_core/src/rules/stdlib/introspection.rs`** —
   API exposta; tipos `Selector`, `Location`.
3. **Inspeccionar `01_core/src/walker/introspector.rs`** —
   `Introspector::query.first()` (referência P208C) — funcional?
4. **Inspeccionar `lab/typst-original/.../footnote.rs`** — como
   vanilla typst integra footnote com `locate`. Esperar uso de
   `Counter` + `query`.
5. **Verificar Tabelas C linhas 387/391/392/404** — todas dizem
   *"depende de ADR-0017 (adiada)"* mas A.9 linha 228 contradiz.
   Reclassificar empiricamente.

**Decisão A.0.0**:
- Se P208B/C confirmados funcionais → bloqueador único permanece
  (variant `Content::Footnote` ausente). **Refutação
  significativa registada — §8.3 N=7 candidato disparado**.
- Se `locate` runtime parcial ou bloqueado em algum aspecto
  crítico para footnote → P295 procede com scope reduzido (e.g.
  numeração estática sem introspector).
- Se `locate` completamente ausente (improvável dado A.9) →
  P295 abortado; reagendar pós-locate materialização.

**Hipóteses possíveis para scope concreto P295** (paralelo H1-H6
P293/P294):

| Hipótese | Scope | Magnitude |
|---|---|---|
| **HA** — Cluster M completo | Variant + stdlib + Layouter 2 fases + Introspector integration | M (planeado) |
| **HB** — Subset Fase 1 | Variant + stdlib + Layouter Fase 1 (marker only; sem nota rodapé) | S |
| **HC** — Subset numeração estática | Variant + stdlib + numeração contadora simples (sem introspector) | XS-S |
| **HD** — `Counter` reuso integral | Reusa P60 `Counter` machinery directamente; sem variant novo footnote-específico | S |
| **HE** — Multi-fase em sub-passos | P295 = Fase 1 (variant + marker); P295.1 = nota rodapé; P295.2 = introspector | XS+XS+S |

**Decisão A.0.0** sobre hipótese: condicional a inspecção literal.
**Default sugerido**: **HE** (multi-fase) se A.0.0 revela
complexidade real do consumer Layouter (renderizar texto + nota
rodapé na mesma página requer 2-pass layout).

### A.0 — Potencial de reuso ADR-0098

| Verificação | Esperado | Procedimento |
|---|---|---|
| `grep "footnote\|Footnote" 03_infra/src/export.rs` | **Zero hits** — footnote é consumida em layout, emit lê `FrameItem::Text` agnóstico | Inspecção literal |
| `FrameItem::Text` precisa novo field? | **Não** — footnote produz `FrameItem::Text` standard + `FrameItem::Footnote` (variant novo em `FrameItem`?) ou similar | A.1.7 verifica |
| Hash `export.rs 66cb8ac3` esperado | **Preservado bit-exact** pelo 12º passo consecutivo se HA-HE não tocam emit | Verificação final |

**Risco**: se hipótese HA exige novo `FrameItem` variant para
rastrear footnote markers vs corpo, emit pode ter que mudar.
Verificar A.1.7 literalmente.

### A.1 — Inventário literal

8 sub-secções:

1. **A.1.1 — `Content` enum pós-P293/P294** — número exacto de
   variants.
2. **A.1.2 — `FrameItem` enum** — variants actuais. **Crítico
   para HA**: se rastreio de footnote markers requer variant
   `FrameItem::Footnote` ou similar, identificar.
3. **A.1.3 — Match exaustivos sobre `Content`** — defesa
   compilador para `Content::Footnote` arm novo.
4. **A.1.4 — Vanilla `FootnoteElem`** — atributos: `body`,
   `numbering: Option<NumberingPattern>`, etc. Listar
   literalmente.
5. **A.1.5 — `Counter` machinery (P60)** — API actual,
   integração com Introspector. **Reuso obrigatório se HD**.
6. **A.1.6 — Consumer Layouter para `cite`** — `cite` (P159A-F)
   é precedente conceptual mais próximo (key + numeração +
   resolução cross-document). Inspeccionar implementação literal.
7. **A.1.7 — Emit consumers** — verificar literalmente que
   `FrameItem::Text` + posicionamento Y resolve representação
   de footnote markers + nota.
8. **A.1.8 — Diagrama de fluxo** — produzir genuinamente; deve
   mostrar 2 fases (marker no flow + nota rodapé página).

### A.2 — Estrutura do variant `Content::Footnote`

Decisão arquitectural (per ADR-0065 critério #1):

| Opção | Estrutura | Prós | Contras |
|---|---|---|---|
| **(a)** `Footnote { body: Box<Content> }` — mínimo | Paridade `Quote` P155 (body required); numeração resolvida via Counter no Layouter | Sem atributos cosméticos extra; precisa Counter pre-existente |
| **(b)** `Footnote { body: Box<Content>, numbering: Option<NumberingPattern> }` | Paridade vanilla atributo `numbering` | Numbering custom é cosmético; ADR-0054 graded |
| **(c)** `FootnoteRef(EcoString)` + `FootnoteBody { body, ... }` — par variants separados | Permite footnote reference `#footnote.entry("key")` (vanilla typst) | Sobre-complica para Fase 1 |

Default sugerido: **(a)** se HE (Fase 1 só); **(b)** se HA (cluster
completo); **(c)** apenas se A.1.4 revelar uso significativo de
`footnote.entry()` em vanilla. **(b)** atinge **N=5 do padrão
"variant rico com cosméticos opcionais"** pós-P156G/H/I+P284
(P287 não qualificou per refutação A.2.0). Esta seria a **primeira
qualificação genuína do padrão N=5** desde P287 — pode disparar
ADR meta paralela.

**Implicação gatilho**:
- (a)/(c) — variant atómico; padrão "variant rico" inalterado.
- (b) — qualificação genuína N=5 do padrão. **Mas (b) só se
  vanilla usa `numbering` realmente (verificar A.1.4)**.

### A.3 — Integração com Counter/Introspector

| Opção | Comportamento |
|---|---|
| **(α)** Counter dedicado `footnote-counter` reusa P60 machinery | Single source of truth — paridade `cite` P159F |
| **(β)** Numeração interna ao Content::Footnote (independente de Counter) | Divergência arquitectural; pior |
| **(γ)** Numeração via Introspector::query (filter on Content::Footnote) | Paridade vanilla mas exige query lookup em cada acesso |

Default sugerido: **(α)** se A.1.5 + A.1.6 confirmam Counter (P60)
+ Introspector::query (P208C) funcionais. **(γ)** se Counter não
suporta semântica footnote (improvável).

### A.4 — Layouter consumer (2 fases)

**Decisão crítica de magnitude — define HA vs HB vs HE**:

| Cenário Layouter | Implementação |
|---|---|
| **Fase 1 (marker only)** | Layouter consume `Content::Footnote` produzindo `[N]` superscript inline; body é **ignorado neste cenário** |
| **Fase 2 (nota rodapé)** | Layouter reserva espaço bottom de cada page; body é layout aí; numeração consistente com marker |
| **Fase 3 (page-overflow)** | Footnote ocupa próximas páginas se rodapé não chega; vanilla typst suporta |

Default sugerido: **Fase 1 só (HB/HC/HE)** se A.4 revelar
complexidade não-trivial da Fase 2. Fase 2/3 são sub-passos
P295.1/P295.2 candidatos.

### A.5 — Detecção de bugs latentes

Cenários fronteira:

- Footnote vazia (`#footnote[]`) — body vazio; comportamento?
- Footnote em math context (`$x_#footnote[a]$`) — anidamento.
- Footnote em footnote (`#footnote[#footnote[recursão]]`) —
  vanilla typst proíbe? Permite?
- Footnote no início de documento — numeração arranca em 1.
- Footnote dentro de figure — interacção com figure caption
  counter.
- Footnote em show-rule (`#show heading: it => [#footnote[]
  #it]`) — pode causar loop infinito se mal projectada.

### A.5' — Verificação anti-reflexão (N=5 do padrão §8.6)

P291-P294 cumulativo. **N=5 atinge limiar formalização tentativo**
mas anti-padrão adia.

4 verificações:

1. **Comparação literal A.1.6 P288-P295** — esperar paradigma
   genuinamente novo:
   - P288 lang: cross-module.
   - P289 weight: TextStyle method.
   - P290 tracking: per-glyph + Tc emit.
   - P291 leading: per-line via peek.
   - P292 font: FontBook indirect resolution.
   - P293 cubic: activação posterior variant inerte.
   - P294 quadratic: transform-on-build (conversão construct-time).
   - **P295 footnote: 2-fase consumer com page-position
     tracking** — paradigma novo (todos os anteriores eram
     single-fase ou inline-only).
2. **A.0 produzido empiricamente** — em P295 inclui A.0.0
   refutação significativa de bloqueador histórico.
3. **Elementos estructuralmente novos identificados**:
   - **Cluster M** (primeira spec de magnitude maior pós-série
     cumulativa).
   - **Refutação significativa de bloqueador histórico** (Tabela
     C linha 387 desactualizada).
   - **Possível qualificação N=5 do padrão "variant rico"** se
     A.2 → (b).
   - **2-fase Layouter** — paradigma novo.
4. **Decisão sobre promoção ADR meta**:
   - **§8.3 N=7** candidato adiado P294 — **P295 dispara
     promoção condicional** se A.0.0 confirma refutação
     significativa.
   - **§8.7' N=3** limiar tentativo se A.0.0 reaplicada
     genuinamente.
   - **§8.6 N=5** limiar passado mas anti-padrão adia.
   - **"variant rico" N=5** se A.2 → (b) genuinamente.
   - **Uma ADR meta por passo no máximo** — escolher uma.
     Preferência:
     - **§8.7' N=3** se reaplicação genuína (template inaugural
       precisa consolidação per P294 §7.1).
     - **§8.3 N=7** se refutação significativa for o ganho
       metodológico maior.

---

## §3 — Materialização

Após Fase A (incluindo decisão hipótese HA/HB/HC/HD/HE):

1. **HA (cluster completo)**:
   - Variant `Content::Footnote{...}` per A.2.
   - `native_footnote(body, ...)` em stdlib.
   - Layouter consumer 2-fase per A.4.
   - Counter integration per A.3.
   - Testes ~20-30 (todas fases + cenários fronteira A.5).

2. **HB/HE (subset Fase 1)**:
   - Variant `Content::Footnote{...}` per A.2.
   - `native_footnote(body, ...)` em stdlib.
   - Layouter consumer Fase 1 (marker only).
   - **P295.1 candidato registado** para Fase 2 (nota rodapé).
   - Testes ~10-15.

3. **HC (numeração estática)**:
   - Subset minimal; sem introspector.
   - Testes ~8-12.

4. **HD (reuso Counter integral)**:
   - Sem variant novo; integração via Counter machinery existente.
   - Testes ~10-15.

5. Promoção ADR meta condicional per A.5'.

6. Actualizar L0:
   - `entities/content.md` (+1 variant Content::Footnote).
   - `rules/stdlib.md` política inalterada (não enumera funções).
   - Tabela A.6 linha 178 — `ausente` → `parcial` (HE) ou
     `implementado` (HA).
   - Tabela B Content — +1 variant.
   - **Tabela C linha 387** — corrigir literalmente conforme
     A.0.0:
     - Se locate funcional confirmado: remover "locate ADR-0017
       adiada"; reclassificar.
   - Tabelas C linhas 391/392/404 — reclassificar se A.0.0
     descobre que `state()`/`measure()` também não dependem
     mais (refutação coletiva).
   - Propagar hashes via `crystalline-lint --fix-hashes`.

7. Diagnóstico produzido:
   `diagnostico-footnote-cluster-passo-295.md` com 7 secções
   A.0.0+A.0+A.1+A.2+A.3+A.4+A.5+A.5'.

**Sem caps** (per P282 §7). Estimativa de testes: 10-30 (depende
fortemente de hipótese).

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P294: 2 808 testes.
  Esperado: ~2 818-2 838 (depende hipótese).
- `crystalline-lint` zero violations.
- Hash L0 `content.md` muda (+1 variant).
- Hash L0 `stdlib.md` **preserved** (política única).
- Hash L0 `entities/geometry.md` **preserved**.
- Hash L0 `export.rs` **condicional**:
  - HA-HE sem variant `FrameItem` novo: **preserved bit-exact**
    pelo 12º passo consecutivo (P282-P295).
  - Se A.1.2 obriga variant `FrameItem` novo: hash **muda
    intencionalmente** — alteração justificada per ADR-0098
    §"alterações justificadas".
- Tabela A.6 linha 178 reclassificada (`ausente` → `parcial`
  ou `implementado`).
- **Tabela C linha 387 corrigida** se A.0.0 confirmar refutação.
- Tabela B Content +1 variant.
- Diagnóstico A.0.0+A.0+A.1+A.2+A.3+A.4+A.5+A.5' produzido.
- **Promoção ADR meta condicional**:
  - §8.3 N=7 se refutação confirmada.
  - §8.7' N=3 se reaplicação genuína.
  - Uma por passo (P273.17 §0).
- Bug latente colateral (se descoberto em A.5) registado.

---

## §5 — Não-objectivos

- **Não** materializar `bibliography(...)` (Tabela A.6). Cite
  já existe parcial (P159A-F); footnote é distinto. `Bibliography`
  passo dedicado (frente XL).
- **Não** materializar `figure.placement` para footnote-on-page.
  Apenas footnote padrão rodapé.
- **Não** suportar footnote recursiva (`#footnote[#footnote[...]]`).
  Vanilla typst proíbe — verificar A.5; preservar paridade.
- **Não** materializar todas as fases se HE escolhida. P295.1
  + P295.2 são sub-passos próprios.
- **Não** promover múltiplas ADRs meta. Uma por passo (P273.17 §0).
- **Não** mexer no Counter machinery P60. Reuso integral.
- **Não** assumir bloqueador histórico (Tabela C linha 387) sem
  verificar A.0.0 — protege contra escrita baseada em assumptions
  desactualizadas (lição P293 §A.0.0 e P294 §11).

---

## §6 — Pendências relacionadas

Resolve (condicional a hipótese):
- **Linha 178** `ausente` → `parcial`/`implementado`.
- **Linha 387** corrigida (bloqueador locate desactualizado).
- **Frente P-footnote-cluster** P282 §6 rank #7.

Não resolve:
- Bibliography full (DEBT-55) — passo XL distinto.
- Cross-document footnote references — passo dedicado.
- Footnote-aware page layout (multi-page overflow) — fase
  futura.
- Show rules sobre footnote (`#show footnote: ...`) — bloqueado
  por regex em L1.

---

## §7 — Risco residual

Risco principal: **A.0.0 confirma bloqueador `locate` ainda
parcial** apesar de A.9 linha 228 sugerir contrário. Mitigação:
inspecção literal antes de qualquer materialização; se locate
parcial, P295 procede com HC (numeração estática) ou abortado.

Risco secundário: **Layouter 2-fase complexidade subestimada**.
Mitigação: A.4 inspecção literal; se Fase 2 (nota rodapé) requer
2-pass layout não-suportado pelo cristalino actual, escolher HE
(multi-fase em sub-passos).

Risco terciário: **A.2 → (b) qualifica N=5 do padrão "variant
rico"** — disparo histórico (primeiro genuíno desde P287
refutação). Mitigação: §5 explicita "uma ADR meta por passo";
se §8.7' N=3 e "variant rico" N=5 ambos disparam, escolher
"variant rico" (mais directo, mais cumulativo).

Risco quaternário: **§8.3 N=7 promoção apressada**. Mitigação:
A.0.0 refutação tem que ser **significativa empiricamente**
(não apenas "linha desactualizada"). Se refutação trivial,
adiar promoção; manter §8.3 pendente para passo posterior.

Risco quinário: **paradigma "2-fase consumer" novo dispara §8.X
adicional** (e.g. padrão "single-flow vs deferred-rendering").
Mitigação: anti-padrão over-formalização; **uma ADR meta por
passo** mesmo se múltiplos padrões emergem.

Risco senário: **cluster M magnitude maior introduz bugs em
features adjacentes** (Counter P60, Introspector P208B/C).
Mitigação: A.5 cenários fronteira; regression bit-exact
obrigatória em features pre-existentes.

Risco septenário (anti-meta): **"sequência reflexa" via A.0.0
template** — terceira reaplicação consecutiva (P293+P294+P295).
Mitigação: A.5' verifica genuinamente; se A.0.0 P295 é
copy-paste de P293/P294, registar honestamente.

---

## §8 — Ponteiros

- Tipo a modificar: `01_core/src/entities/content.rs` (`Content`
  enum, N variants pós-P293+P294).
- Função stdlib: `01_core/src/rules/stdlib/model.rs` (ou
  caminho equivalente — paralelo `native_cite` P159A).
- Counter machinery: `01_core/src/walker/counter.rs` (P60).
- Introspector: `01_core/src/walker/introspector.rs` (P208C).
- Layouter consumer: `01_core/src/rules/layout/mod.rs`.
- Vanilla: `lab/typst-original/crates/typst-library/src/model/footnote.rs`.
- Precedente arquitectural mais próximo: `cite` (P159A-F).
- Precedente "variant rico com cosméticos": P156G/H/I + P284 —
  N=4 cumulativo. P295 = **N=5 candidato** se A.2 → (b).
- ADR aplicável: **ADR-0098** + **ADR-0099**.
- ADR processual: ADR-0065 (inventariar-primeiro; 7 secções).
- ADR cultural: P273.17 §0 (anti-padrão; uma ADR meta por passo).
- Padrão §8.7' A.0.0 template (N=3 cumulativo se reaplicado).
- Padrão §8.3 refutação pragmática (N=7 candidato adiado P294).

---

*Spec P295 produzida 2026-05-19 pós-P294 (curve quadratic
fechado; A.0.0 N=2 inaugurou refutação significativa). Frente
`P-footnote-cluster` — **primeiro cluster M pós-série cumulativa**
P288-P294 cirúrgica. Magnitude reset: spec abandona expectativa
XS-S e abraça M. Fase A obrigatória com **7 secções**: A.0.0
(N=3 cumulativo template §8.7'; verifica empiricamente bloqueador
histórico Tabela C linha 387 vs A.9 linha 228 contradição) +
A.0-A.5 (paralelo P294) + A.5' (N=5 cumulativo padrão §8.6,
limiar tentativo). Cinco hipóteses possíveis (HA-HE) para scope
concreto — default sugerido HE (multi-fase em sub-passos) se
Layouter 2-fase complexidade revelada. Critério de fecho
condicional: hash `export.rs` preservado pelo 12º passo se
HA-HE não exigem `FrameItem` variant novo. **Promoção ADR meta
condicional**: §8.3 N=7 se A.0.0 confirma refutação significativa;
§8.7' N=3 se reaplicação genuína; "variant rico" N=5 se A.2 →
(b). **Uma ADR meta por passo no máximo** (P273.17 §0). Risco
septenário novo registado: terceira reaplicação A.0.0 consecutiva
pode degenerar em rubber-stamp se não vigilada. Sem caps LOC ou
magnitude (P282 §7).*
