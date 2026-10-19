# Diagnóstico — Fase A do Passo 300 (Retrospectivo metodológico)

**Data**: 2026-05-19
**Spec mãe**: `00_nucleo/materialization/typst-passo-300.md`
**Origem**: P299 §9 sugeriu marco numérico como oportunidade de
consolidação metodológica.
**Tipo declarado spec**: qualitativamente distinto — 1.ª spec
não-materializadora desde P283.
**Caminho adoptado**: **A (consolidação documental pura)** com nota
explícita sobre candidato §8.7' maduro mas adiado per anti-padrão
P273.17 §0.
**Magnitude da refutação A.0.0 N=8**: **inventário**, não refutação
— P300 inspecciona estado cumulativo, não código novo.

---

## A.0.0 — Inventário literal padrões cumulativos (N=8 §8.7')

### A.0.0.1 — §8.7' "A.0.0 template" (N=7)

Sequência completa de magnitudes documentadas em relatórios:

| Passo | Magnitude | Refutação central | Evidência factual |
|---|---|---|---|
| P293 | **alta** | H6 não-listada na spec | `PathItem::CubicTo` existia inerte; spec antecipou variant ausente |
| P294 | **máxima** | Spec inteira invalidada | Vanilla converte q→c em construct-time; sem variant `QuadraticTo` |
| P295 | **baixa** | Linha tabela admin desactualizada | Tabela C linha 387 vs A.9 228 contradição |
| P296 | **média** | Classificação inteira inválida | Tabela A.4 marca `parcial` mas zero hits L1 |
| P297 | **alta** | Vanilla NÃO tem wrapper unificado | `lab/.../math/underover.rs` fragmenta em 12 elementos |
| P298 | **alta** | Heurística limits-style já existia | `symbols::is_limit_function` hardcoded P50 |
| P299 | **baixa-modesta** | Cristalino tem `calc` precedente | `make_calc_module` paralelo arquitectural directo |

**Total**: 7 aplicações. Magnitudes: 2 altas + 1 máxima + 2 médias-baixas
+ 2 baixas. **Tendência não-monotónica** — flutuação saudável.

**Valor empírico documentado**:
- P294 evitou criação de variant `QuadraticTo` desnecessário.
- P297 evitou criação de wrapper `UnderoverElem` inexistente.
- P298 estendeu sem substituir heurística existente.
- P299 reusou padrão `calc` module sem reinvenção.

**Veredicto**: **MADURO mas AMBÍGUO para promoção formal**.

**Justificação ambiguidade**:
- A favor de promover (caminho B):
  - 7 aplicações cumulativas — limiar tentativo passado.
  - Valor empírico claro em P294/P297/P298.
  - Magnitudes variadas demonstram não-ritualismo.
- Contra promover:
  - Funciona organicamente **sem** formalização desde P293.
  - **Formalização traz overhead** se ADR for prescritiva ("A.0.0
    obrigatória em todos os passos") quando na verdade é
    **discricionária** (P273.17 §0 anti-rigidez).
  - Promoção P300 corre risco de **§7 risco terciário** spec
    ("retrospectiva degenera em celebração arquitectural") + **§7
    risco septenário** ("formaliza algo que era flexível").
  - **Honestidade**: cada passo P283-P299 aplicou A.0.0 **por
    decisão local**, não por regra. ADR formalizaria como regra
    o que era prática.

### A.0.0.2 — §8.3 "refutação pragmática" (N=11 candidato adiado)

Refutações cumulativas P283-P299:

| Origem | Refutação |
|---|---|
| P293 H6 | hipótese não-listada |
| P294 | toda estrutura spec invalidada |
| P295 | linha tabela 387 desactualizada |
| P296 | classificação Tabela A.4 inválida |
| P297 | wrapper unificado vanilla inexistente |
| P298 | heurística limits-style já existia |
| P299 | calc module precedente directo |
| + várias menores P285-P292 | ... |

**Veredicto**: **ROBUSTO mas DESCRITIVO**.

§8.3 é **observação sobre o sistema** (specs por vezes contêm
assumções factualmente incorrectas), não **princípio operacional**.
ADR seria descritiva ("este padrão emerge"), não prescritiva
("faça X"). Valor de formalização: **baixo**.

**Não promover** — preserva ambiguidade saudável; reavaliar P300+.

### A.0.0.3 — §8.6 "A.5' anti-reflexão" (N=9 cumulativo)

P291+P292+P293+P294+P295+P296+P297+P298+P299. Padrão metodológico
**interno** — não-promovido em todos os 9 passos consecutivos per
anti-padrão.

**Veredicto**: **INSTRUMENTAL INTERNO** — função metodológica útil
mas formalização não necessária. Manter como ferramenta sem ADR.

### A.0.0.4 — "Variant rico com cosméticos opcionais" (N=5 candidato)

| Tentativa | Veredicto |
|---|---|
| P156G Block (bool defaults) | Refutado P287 — cosméticos |
| P156H Boxed (bool defaults) | Refutado P287 |
| P156I Stack (bool defaults) | Refutado P287 |
| P284 Underline (Option<Length/Color>) | Categoria diferente |
| **P297 MathUnderover (Option<Box<Content>>)** | **Qualifica genuíno N=5** |
| P298 MathOp (bool limits) | Ambíguo — discriminador, não Option |

**Veredicto**: **N=1 GENUÍNO** (P297 só). Limiar tentativo N≥3 não
atingido. **Adiar**.

### A.0.0.5 — Sub-padrão "cluster math handler dedicado" (N=3 ambíguo)

P296 (`layout_accent`, `layout_cancel`) + P297 (`layout_underover`)
+ P298 (`layout_op` trivial delegate) = 3 handlers, mas P298 é
trivial (qualidade questionável).

**Veredicto**: **AMBÍGUO** — quantitativo N=3 atinge limiar mas
qualitativo questionável. Adiar.

### A.0.0.6 — Sub-padrão "cross-variant interaction" (N=1 inaugural)

P298 modificou `attach.rs is_limits` para detectar `MathOp`.
**Veredicto**: **INSUFICIENTE** — N=1; aguardar reaplicações.

### A.0.0.7 — Sub-padrão "module namespaced" (N=2 cumulativo)

P283 (`make_calc_module`) + P299 (`make_math_module`). Padrão de
construção arquitectural.

**Veredicto**: **INSUFICIENTE** — N=2 abaixo de limiar N≥3.
Reaplicação futura possível (e.g. `make_text_module` se justificada).

### A.0.0.8 — Sub-padrão "operadores pré-definidos via SSoT" (N=1 inaugural)

P299 registou 42 operadores como `Value::Content(MathOp)`.
**Veredicto**: **INSUFICIENTE** — N=1.

---

## A.0.0' — Decisão de caminho A/B/C/E

### A.0.0'.1 — Avaliação dos caminhos

| Caminho | Critério A.0.0 satisfeito | Decisão |
|---|---|---|
| **A** consolidação documental | Sempre disponível | **CANDIDATO** |
| **B** §8.7' promovida | §8.7' ambíguo per §A.0.0.1 — NÃO inequívoco | Rejeitado |
| **C** §8.3 promovida | §8.3 robusto mas descritivo per §A.0.0.2 — NÃO inequívoco | Rejeitado |
| **D** múltiplas | P273.17 §0 — REJEITADA estructuralmente | Rejeitado |
| **E** auditoria sem decisão | Disponível mas sobreposto a A | Sub-caminho |

### A.0.0'.2 — Decisão: caminho A com nota E

**Caminho A escolhido** + nota explícita E sobre §8.7' candidato
maduro adiado.

**Razões**:
1. **Nenhum padrão é inequívoco** per critério estrito spec §1.1.
2. §8.7' maduro mas formalização traz overhead (§A.0.0.1 contra).
3. **Anti-padrão over-formalização P273.17 §0** prefere conservador.
4. **Risco septenário** spec realizou-se hipoteticamente: ADR
   "A.0.0 obrigatório" formalizaria como regra absoluta o que era
   discricionária.

### A.0.0'.3 — Honestidade epistémica

**Caminho A é decisão consciente, não fuga**:

- §8.7' tem evidência cumulativa robusta (7 aplicações; valor
  empírico em 4+ passos).
- Formalização **não é prejudicial em si**, mas o ganho marginal
  vs overhead de ADR formal é baixo.
- **Decisão honesta**: padrão maduro mas formalização adiada por
  preferência conservadora **explícita e justificada**.

**P301+ pode reavaliar** se §8.7' atingir N=8+ com novos casos de
valor empírico inequívoco (e.g. A.0.0 descobrindo bug que outros
métodos teriam falhado).

### A.0.0'.4 — Estado de outros candidatos

| Padrão | Estado pós-P300 | Próxima reavaliação |
|---|---|---|
| §8.7' (N=7) | **maduro mas adiado** | P301+ se N=8 com valor inequívoco |
| §8.3 (N=11) | descritivo; adiado | quando puder ser prescritivo |
| "Variant rico" (N=1 genuíno) | **insuficiente** | quando N≥3 genuínos |
| "Cluster math handler" (N=3 ambíguo) | adiado | nova aplicação substantiva |
| "Cross-variant" (N=1) | insuficiente | reaplicação N≥3 |
| "Module namespaced" (N=2) | insuficiente | N=3 |
| "Operadores SSoT" (N=1) | insuficiente | N≥3 |

---

## A.0 — Aplicação ADR-0098 (hash preservation)

Caminho A: zero alterações em código de produção. Hash `export.rs
66cb8ac3` preservado bit-exact pelo **17º passo consecutivo**
(P282→P300).

ADR-0098 **N=17 cumulativo**.

---

## A.1 — Inventário documental

### A.1.1 — Tabelas cobertura modificadas P283-P299 (17 entries)

| Tabela | Linha | Passo |
|---|---|---|
| A.2 | trig+log+exp+constantes calc | P283 |
| A.3 | underline/strike/overline | P284 |
| A.6 | `footnote` parcial→implementado | P295 |
| A.4 | `accent` parcial→implementado | P296 |
| A.4 | `cancel` parcial→implementado | P296 |
| A.4 | `underover` parcial→implementado | P297 |
| A.4 | `op` parcial→implementado | P298 |
| A.4 | linha 121 actualizada com P299 | P299 |
| A.7 | `curve(...)` implementado⁺→implementado | P293 |
| A.7 | `curve.quadratic` activado via P294 | P294 |
| C | linha 387 footnote bloqueador refutado | P295 |
| (Style) | tracking/leading/font cumulativos | P290-P292 |
| (FrameItem) | Line.color | P285 |
| (smartquote) | implementado | P287 |
| (decorations wrap-aware) | implementado | P286 |

### A.1.2 — Hashes L0 actuais (pós-P299)

| Ficheiro L0 | Hash | Última alteração |
|---|---|---|
| `entities/content.md` | propagado P298 | P298 (+MathOp); P299 inalterado |
| `entities/content.rs` | `82d3c47d` | P298 |
| `infra/export.md` | `31a37c57` | inalterado desde P282 |
| `infra/export.rs` | `66cb8ac3` | **15 passos preservado** (P282→P299) |
| `entities/geometry.md` | propagado P293 | P293 |
| `rules/stdlib.md` | `21ade03a` | política única; inalterado |

### A.1.3 — ADRs vigentes

Base 84 + ADR-0098 (P288) + ADR-0099 (P289) = **86 ADRs**. P300
preserva contagem (caminho A).

### A.1.4 — Frentes pendentes catalogadas

Vide `frentes-pendentes-pos-p299.md` (ficheiro dedicado).

### A.1.5 — Bugs latentes descobertos

- NBSP em P287 SmartQuote consumer — fixed P290 via `layout_word`.
- Nenhum outro bug crítico latente conhecido.

### A.1.6 — Lições metodológicas emergentes

Vide `retrospectivo-p283-p299.md` (ficheiro dedicado):
- "Confirmação esperada" (P295 §10; P297 §A.5').
- "Flutuação saudável" magnitude A.0.0 (P299 §10).
- "Subdivision decision A.0.0'" (P299).
- "Cross-variant interaction" (P298).
- "Module namespaced via SSoT" (P299).
- "Refutação significativa via inspecção vanilla" (P294/P297).

### A.1.7 — Sequência cumulativa A.0.0 magnitudes (gráfico texto)

```
P293: ████████░░ alta
P294: ██████████ máxima
P295: ██░░░░░░░░ baixa
P296: ██████░░░░ média
P297: ████████░░ alta
P298: ████████░░ alta
P299: ███░░░░░░░ baixa
```

**Padrão visual**: 2 altas iniciais → baixa → média → 2 altas →
baixa. **Não-monotónico** — flutuação saudável documentada.

### A.1.8 — Diagrama cumulativo (paradigmas P288-P299)

```
P288 lang        ←→ cumulativo style/text
P289 weight      ←→ cumulativo (refinamento TextStyle)
P290 tracking    ←→ cumulativo (per-glyph + Tc emit)
P291 leading     ←→ cumulativo (per-line peek)
P292 font        ←→ cumulativo (FontBook indirect)
─── transição: série cumulativa fecha ───
P293 cubic       ←→ ortogonal (variant inerte activação)
P294 quadratic   ←→ ortogonal (transform-on-build)
P295 footnote    ←→ ortogonal (walker counter; Fase 1)
─── cluster math ───
P296 accent/cancel ←→ math layout handler dedicado (inaugural)
P297 underover    ←→ extensão directa P296 (Option estructural)
P298 op           ←→ extensão directa + cross-variant interaction
─── pós-cluster ───
P299 math module ←→ ortogonal pós-cluster (SSoT via MathOp)
P300 retrospec.   ←→ qualitativamente distinto (sem materialização)
```

**11 paradigmas distintos** identificados na sequência cumulativa.

---

## A.2 — Decisão sobre promoção (caminho A confirmado)

**0 promoções** ADR meta.

Justificações documentadas em A.0.0.1-8 e A.0.0'.

---

## A.3 — Integração documental (caminho A)

Per §3.A spec:
1. **`retrospectivo-p283-p299.md`** — documento metodológico
   cumulativo (criar).
2. **`frentes-pendentes-pos-p299.md`** — catálogo (criar).
3. Tabelas A/B/C — verificação consistência (sem alterações
   estruturais necessárias dado P300 não materializa).
4. Hashes L0 — verificação ausência drift via `crystalline-lint
   --fix-hashes` (esperado "Nothing to fix").

---

## A.4 — Impacto em hashes

**0 alterações** em código de produção (`01_core/`, `03_infra/`,
`02_shell/`, `04_wiring/`). Hashes preservados:

- `export.rs` `66cb8ac3` — **17º passo consecutivo P282-P300**.
- `content.rs` `82d3c47d` — inalterado.
- `geometry.rs` — inalterado.
- L0 markdown — inalterados.

ADR-0098 **N=17 cumulativo**.

---

## A.5 — Detecção de bugs latentes documentais

5 cenários verificados:

| Cenário | Veredicto |
|---|---|
| Tabela A.4 linha 119 (4 features) vs Tabela B Math (14 variants) | ✅ Consistente — 4 features user-facing mapeiam para 4 variants (Accent/Cancel/Underover/Op) + 6 internas (Sequence/Ident/Text/Frac/Attach/Root) + 4 (Delimited/AlignPoint/Matrix/Cases) = 14 total |
| Hashes L0 propagados | ✅ `--fix-hashes` retornou "Nothing to fix" após P299 |
| Frentes pendentes duplicação | ⚖ Verificar em `frentes-pendentes-pos-p299.md` (catálogo único) |
| ADRs vigentes 84+2=86 | ✅ Verificável via `00_nucleo/adrs/` |
| Documentação P296.2 status pós-P298 | ✅ P298 fechou; P299 fechou P298.X derivada |

**Sem bugs latentes documentais críticos identificados**.

---

## A.5' — Anti-reflexão N=10 cumulativo (P291-P300)

### A.5'.1 — Comparação paradigmas P288-P300

P293-P299 nove paradigmas materializadores. P300 **paradigma novo**:
**retrospectiva sem materialização**. Genuinamente distinto.

### A.5'.2 — A.0.0 N=8 reaplicação — magnitude **inventário**

P300 NÃO é refutação (não materializa feature). A.0.0 é
**inventário literal** dos padrões cumulativos — categoria nova.

### A.5'.3 — Elementos estructuralmente novos

5 elementos:

1. **1.º passo não-materializador** pós-série P283-P299.
2. **A.0.0 sobre estado cumulativo** (não código).
3. **A.0.0' decisão caminho A/B/C/D/E** — 2.ª aplicação após P299
   subdivision decision.
4. **Avaliação cumulativa de 8 padrões adiados** — auditoria
   sistemática.
5. **Caminho A (conservador) escolhido honestamente** apesar de
   §8.7' maduro — anti-padrão over-formalização honrado.

### A.5'.4 — Decisão sobre promoção ADR meta

**0 promoções** — confirmado per A.0.0' e A.0.0.

Anti-padrão P273.17 §0 **rigorosamente honrado** pela **8ª vez
consecutiva** (P292+P293+...+P300).

---

## §Métricas do impacto P300

| Métrica | Antes | Pós-P300 |
|---|---:|---:|
| `Content` variants | 69 | 69 (inalterado) |
| Stdlib funcs | inalterado | inalterado |
| Hash `export.rs` | `66cb8ac3` | **preservado** (17º passo) |
| Hash `content.rs` | `82d3c47d` | preservado |
| L0 hashes | preservados | preservados |
| ADRs | 86 | 86 (inalterado — caminho A) |
| Padrão §8.7' N | 7 | 7 + auditoria documental |
| Padrão §8.3 N | 11 | 11 + auditoria documental |
| Padrão §8.6 N | 9 | **10** (P300 reaplica) |
| Documentos consolidação novos | 0 | **2** (retrospectivo + frentes pendentes) |
| Testes | 2862 | 2862 (zero novos) |
| ADRs meta promovidas | 0 | 0 |

---

## §Risco residual mitigado

- **Risco principal** (promoção forçada P300 número redondo): ✅
  refutado — caminho A escolhido conservadoramente; honestidade
  documentada.
- **Risco secundário** (caminho B/C sem evidência inequívoca): ✅
  refutado — §A.0.0.1-8 demonstra ambiguidade.
- **Risco terciário** (celebração arquitectural): ✅ refutado —
  documentação factual obrigatória sem especulação.
- **Risco quaternário** (ADR-0100 §8.7' mal formulada): ✅ não
  aplica — ADR não criada.
- **Risco quinário** (expectativa retrospectivos periódicos): ⚖
  documentado em retrospectivo — P300 é evento único, não cadência.
- **Risco senário** (fragmentação frentes pendentes): ✅ catálogo
  único `frentes-pendentes-pos-p299.md`.
- **Risco septenário** (lição pré-empção): ✅ refutado — A.0.0
  preserved como **prática discricionária**, não regra absoluta.

---

## §Fecho da Fase A

Inventário literal completo dos 8 padrões cumulativos +
**A.0.0' caminho A (consolidação documental pura) escolhido
honestamente** + **0 ADRs meta promovidas** apesar de §8.7' maduro
— anti-padrão P273.17 §0 rigorosamente honrado.

**MARCO P300**:
- **1.º passo não-materializador** pós-série P283-P299 (17 passos
  cumulativos).
- **Retrospectivo metodológico qualitativamente distinto** —
  paradigma novo.
- **8 padrões cumulativos auditados** — todos adiados com
  justificações factuais.
- **§8.7' N=7 candidato maduro** mas adiado por preferência
  conservadora explícita (não por evidência ambígua).
- **ADR-0098 N=17 cumulativo** — hash `export.rs` preservado
  pelo 17º passo consecutivo.
- **Caminho A defensável**: marco numérico P300 NÃO foi pretexto
  para promoção; critério estrito factual honrado.

Procede-se a §3 (caminho A: retrospectivo + frentes pendentes).
