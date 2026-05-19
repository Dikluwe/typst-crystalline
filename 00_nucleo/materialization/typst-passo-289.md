# Passo 289 — `Style::Weight` variant

**Frente**: `P-style-weight-variant` (rank 1 do relatório P288 §9).
**Origem**: P288 §7 risco terciário materializado positivamente —
"4 fields restantes (`weight`/`tracking`/`leading`/`font`) podem
virar passos P288.1-P288.4". P289 = P288.1 (sequência directa).
**Pré-requisitos**: nenhum bloqueador.
**Origem secundária**: Tabela B.3 lista 6 variants pós-P288 (sem
`Weight`). Tabela B.4 linha 350 lista `StyleDelta.weight` como
`implementado⁺` desde P139 (faux-bold; ADR-0054). Mesma assimetria
de P287/P288 (lang) — variant ausente apesar de feature
parseada-activa.

---

## §1 — Objectivo

Adicionar variant `Style::Weight(...)` ao enum `Style` em
`01_core/src/entities/style.rs`, integrar com a cascade existente
(parse `#set text(weight: 700)` → `Style::Weight(...)` → acumula
em `StyleDelta.weight`), e fechar 1/4 da assimetria residual P288
registada em §7 risco terciário do relatório anterior.

Razão de ser (paralelo arquitectural directo a P288):

1. **Fecha 1/4 da assimetria residual** — P288 §A.1.2 enumerou
   `weight`/`tracking`/`leading`/`lang`/`font` como 5 fields sem
   variant `Style` correspondente; P288 fechou `lang`; P289 fecha
   `weight`.
2. **Reaplicação directa do padrão §8.2 P288** — "activação
   posterior de feature graded". Estado N=4 cumulativo pós-P288;
   se P289 reaplicar, atinge **N=5** → candidato a promoção ADR
   meta paralela a ADR-0098.
3. **Aplicação directa de ADR-0098 (recém-formalizada)** — Fase A
   inclui secção obrigatória "potencial de reuso" (per P288 §9
   ponto operacional). Hash `export.rs 66cb8ac3` deve ser
   preservado por construção (weight não toca emit — é decisão
   pre-shaping em font selection / faux-bold).
4. **Risco de novos bugs latentes detectáveis** — padrão §8.4 P288
   inaugural ("Bug latente fixed durante materialização de feature
   dependente"). P289 pode revelar bugs análogos em consumers
   actuais de `chain.weight()`.

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória)

Cinco ambiguidades factuais (uma a mais que P288, secção
"potencial de reuso ADR-0098" explícita per orientação operacional):

### A.0 — Potencial de reuso ADR-0098 (nova secção obrigatória pós-P288)

Per directiva operacional P288 §9: Fase A de passos futuros deve
citar explicitamente potencial de reuso (hash `export.rs`
preservado ou justificadamente alterado).

Verificar antes da materialização:

1. `grep "weight\|Weight" 03_infra/src/export.rs` — esperado **zero
   hits funcionais**. Se aparecer, registar como bug-evidência
   contra reuso single-source.
2. `FrameItem::Text` precisa de novo field `weight`? — esperado
   **não**, dado P139 já implementou faux-bold sem persistir
   weight em emit.
3. Hash `export.rs` esperado preservado bit-exact (6º passo
   consecutivo pós-P288).

**Output**: secção `A.0` do diagnóstico
`00_nucleo/diagnosticos/diagnostico-style-weight-passo-289.md`
com a citação literal de ADR-0098 + verificação empírica.

### A.1 — Inventário literal do caminho actual `weight`

Listar (8 sub-secções paralelas a P288 §A.1.1-A.1.8):

1. **A.1.1 — `Style` enum** — confirmar pós-P288 6 variants
   incluindo `Lang(Lang)`.
2. **A.1.2 — `StyleDelta.weight` tipo** — verificar literalmente
   (Tabela B.4 indica `implementado⁺`; verificar se é `u16`,
   `FontWeight`, `Option<u16>`, etc.).
3. **A.1.3 — `push_styles` cascade match** — exaustivo sobre 6
   variants; `Style::Weight(...)` adicionará 7º arm.
4. **A.1.4 — `delta.weight` write site único actual** — esperar
   `eval/rules.rs:???` (parse-driven `#set text(weight: ...)`;
   precedente P139).
5. **A.1.5 — `delta.weight` read sites** — onde é consultado em
   layout? `chain.weight()` análogo a `chain.lang()`?
6. **A.1.6 — Consumers actuais de `style.weight`** — esperar P139
   faux-bold consumer + potencialmente FontVariant selection. Em
   particular, **verificar se há outros consumers latentes** que
   ainda não foram exercitados em teste.
7. **A.1.7 — `FrameItem::Text` emit** — confirmar zero hits em
   `export.rs` (paradigma ADR-0098).
8. **A.1.8 — Diagrama de fluxo** — caminho lateral idêntico ao P288
   (2ª fonte de entrada via variant; consumer chain inalterado).

### A.2 — Estrutura do variant `Style::Weight`

Decisão arquitectural (per ADR-0065 critério #1):

| Opção | Estrutura | Prós | Contras |
|---|---|---|---|
| **(a)** `Weight(u16)` — paralelo `HeadingLevel(u8)` | Mínimo; `u16` é `Copy` → `Style` mantém `Copy` | Tipo "raw" sem domínio explícito (100-900 step 100) |
| **(b)** `Weight(FontWeight)` — se tipo dedicado existir | Domínio explícito (100-900) | Verificar A.1.2 se `FontWeight` existe; pode não |
| **(c)** `Weight(W)` onde `W = u16` — alias type | Compromisso | Indirecção sem ganho material |

Default sugerido: **(b)** se A.1.2 mostrar que `FontWeight` (ou
tipo dedicado análogo) já existe — preserva domínio explícito.
**(a)** se A.1.2 mostrar que `StyleDelta.weight` é literal `u16`
sem tipo wrap — paridade simétrica ao P288 (que usou `Lang(Lang)`
porque `Lang` era o tipo já presente).
**(c)** rejeitada — sem valor adicional.

### A.3 — Integração com `StyleDelta`

Decisão de integração cascade:

| Opção | Comportamento |
|---|---|
| **(α)** `delta.weight = Some(*w)` — last-write wins (paridade absoluta P288 `Lang`) | Padrão dos 6 arms existentes |
| **(β)** Merge com semântica específica (e.g., weights aditivos: parent `bold` + child `italic` mantém bold) | Vanilla typst: weights são last-write wins; merge ausente |

Default sugerido: **(α)** salvo se A.1 revelar comportamento de
merge atípico no parse-driven path actual (improvável).

### A.4 — Impacto em `FrameItem::Text` e emit (aplicação ADR-0098)

Decisão crítica (registada para confirmar aderência ADR-0098):

| Opção | Mecanismo | Implicação `export.rs` |
|---|---|---|
| **(i)** `FrameItem::Text` consulta `weight` indirectamente via `chain.weight()` (P139 paradigm) — P289 apenas estende caminho de entrada | `export.rs` **preservado bit-exact** (6º passo consecutivo) → confirma ADR-0098 robusta |
| **(ii)** `FrameItem::Text` precisa de novo field `weight: u16` (e.g. para faux-bold persistido em emit) | `export.rs` **muda** — viola ADR-0098 nominalmente; investigar se há razão estrutural genuina |
| **(iii)** Híbrido — reflector trivial | `export.rs` muda mas trivialmente; ADR-0098 ainda aplicável com nota |

Default sugerido: **(i)** se A.0 + A.1.7 confirmarem zero hits em
`export.rs`. **(ii)** não esperado dado P139 já implementou
faux-bold sem emit-side state. **(iii)** registar como divergência
se ocorrer.

**Implicação gatilho N=5 padrão §8.2**: se A.4 → (i), este passo
cita o padrão "activação posterior de feature graded" P288 §8.2
N=4 → P289 = **N=5**. Promoção ADR meta paralela a ADR-0098 fica
**condicional** — se gatilho disparar empiricamente.

### A.5 — Detecção de bugs latentes (padrão P288 §8.4)

P288 §8.4 inaugurou padrão N=1 — bug NBSP descoberto em P287
durante materialização. Por construção, P289 deve incluir testes
que exercitem `Style::Weight` injectado via `Content::Styled` num
caminho que actualmente é parse-only:

- Teste com `Style::Weight(700)` (bold) injectado via cascade não-parse.
- Teste com `Style::Weight(100)` (thin) — fronteira inferior.
- Teste com `Style::Weight(900)` (black) — fronteira superior.
- Teste com `Style::Weight(450)` — valor não-canónico (vanilla typst
  aceita; verificar comportamento cristalino).

**Se algum destes testes revela divergência ou bug**, registar como
ganho colateral (paralelo P288 NBSP fix).

---

## §3 — Materialização

Após Fase A produzir inventário + estrutura + integração + impacto
emit + plano de detecção bugs:

1. Adicionar `Weight(<tipo per A.2>)` ao enum `Style`.
2. Implementar match-exhaustive arm em `StyleChain::push_styles`
   (paralelo P288 §2.3): `Style::Weight(w) => delta.weight = Some(*w)`.
3. Garantir match exaustivo continua (compilador detecta omissões
   por construção — defesa cumulativa P288).
4. Testes (4 cenários A.5 + paralelos P288 §4):
   - L1 unitário variant: ctor; PartialEq; serialização (se
     aplicável).
   - L1 unitário cascade: `push_styles` projecta no delta.
   - L1 unitário injection: `Content::Styled(body,
     Styles::from_iter([Style::Weight(700)]))` faz `chain.weight()
     == Some(700)`.
   - L1 unitário last-write: 2 `Style::Weight` consecutivos →
     último vence.
   - L1 unitário catalog: actualizar test catalog 6 → 7 variants.
   - L1 unitário fronteiras: 100/700/900/450 (per A.5).
   - L3 integração: `#set text(weight: 700); texto` produz output
     com faux-bold activo via caminho `Content::Styled` (paralelo
     ao parse).
5. Aplicação ADR-0098 obrigatória:
   - Diagnóstico A.0 cita ADR-0098 + verificação empírica.
   - Relatório §1 confirma hash `export.rs` preservado.
   - Se preservado → registar em §3 ponto 5 como evidência cumulativa
     padrão §8.2 P288 N=5.
6. Promoção ADR meta condicional:
   - Se N=5 do padrão §8.2 dispara empiricamente (5 aplicações
     cumulativas confirmadas: P285 + P286 + P287 + P288 + **P289**),
     promover ADR-009X "Activação posterior de feature graded
     como padrão de pendência".
   - Se não dispara (e.g. A.5 revela bug grave que força refactor
     fora do padrão), **não promover**.
   - **Apenas uma ADR meta por passo** — se §8.3 (refutação
     pragmática) também atingir N=6 neste passo, escolher uma e
     adiar outra (per P273.17 §0 anti-padrão; precedente P288).
7. Actualizar L0:
   - `00_nucleo/prompts/entities/style.md` (ou caminho per A.1).
   - Tabela B.3 — adicionar `Weight(...)` como 7º variant.
   - Tabela B.4 linha 350 — nota cruzada P289 (2ª fonte de entrada
     documentada).
   - Propagar hashes via `crystalline-lint --fix-hashes`.
8. Actualizar diagnóstico:
   - `diagnostico-style-weight-passo-289.md` com 5 secções
     A.0-A.5.

**Sem caps** (per P282 §7). Estimativa de testes: ~6-10 (modesto;
paralelo P288 +7 mas sem reactivação de testes pré-existentes — o
delta é puramente material).

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P288: 2 755 testes.
  Esperado: ~2 761 a ~2 765.
- `crystalline-lint` zero violations.
- Hash L0 `style.md` muda (+1 variant em B.3).
- Hash L0 `content.md` **preserved**.
- Hash L0 `stdlib.md` **preserved**.
- Hash L0 `export.rs` **condicional**:
  - Preserved se A.4 → (i) → confirma ADR-0098 vigente; 6º passo
    consecutivo.
  - Muda se A.4 → (ii) → registar como investigação obrigatória
    (violação nominal ADR-0098 exige justificação estrutural).
- **Regressão bit-exact validada** — todos os testes weight-aware
  pré-P289 (P139 faux-bold) continuam verdes byte-exact.
- Tabela B.3 actualizada com `Weight(...)` (7º variant) + nota
  assimetria residual (3 fields restantes: `tracking`/`leading`/
  `font`).
- Tabela B.4 linha 350 com nota cruzada P289.
- Diagnóstico A.0+A.1+A.2+A.3+A.4+A.5 produzido.
- **Condicional**: se A.4 → (i) confirma ADR-0098 + 5 aplicações
  cumulativas de §8.2 P288, **ADR-009X promovida** com 5 citações
  formalizadas:
  - N=1: P285 §8.3 (`stroke` activated).
  - N=2: P286 §8.1 (wrap-aware activated).
  - N=3: P287 §8.2 (smartquote stdlib materialized).
  - N=4: P288 §8.2 (`Style::Lang` activated).
  - N=5: **P289** (`Style::Weight` activated).
- Bug latente colateral (se descoberto) registado e fixado.

---

## §5 — Não-objectivos

- **Não** materializar `Style::Tracking`/`Style::Leading`/
  `Style::Font` simultaneamente. P289 é cirúrgico (1 variant); a
  sequência P288→P289 estabelece patamar para reaplicação per-field
  em passos separados.
- **Não** activar font-file Bold dedicado (`FontVariant` selection
  variant-aware). Continua ADR-0055bis candidata; fora de scope.
- **Não** materializar weight constantes simbólicas
  (`thin`/`bold`/etc.) como aliases stdlib se P139 não as
  implementou. Verificar A.1; se ausentes, registar como
  scope-out.
- **Não** promover ADR meta paralela a ADR-0098 se A.4 → (ii)/(iii).
  Promoção é **condicional** ao gatilho disparar genuinamente —
  P273.17 §0 anti-padrão.
- **Não** promover **mais que uma** ADR meta neste passo. Se
  múltiplos padrões atingem limiar simultaneamente (§8.2 + §8.3),
  escolher a com **mais evidência cumulativa concreta** (provável
  §8.2) e adiar outra para passo próprio.
- **Não** alterar `StyleDelta.weight` semanticamente. Se já é
  `Option<u16>` ou similar (verificar A.1.2), apenas adicionar 2ª
  fonte de entrada via `Style::Weight`.
- **Não** validar input weight range (100-900 step 100). Vanilla
  typst aceita valores fora deste range; cristalino deve preservar
  paridade.

---

## §6 — Pendências relacionadas

Resolve:
- **Assimetria Tabela B.3 vs B.4 para `weight`** — 1/4 da assimetria
  residual P288 §7 risco terciário.

Não resolve (continua aberto):
- Outros 3 fields sem variant `Style` correspondente
  (`tracking`/`leading`/`font`) — passos próprios candidatos
  P289.1-P289.3 (paralelos a P288.1-4 não-reservados).
- `FontVariant` selection variant-aware (Tabela A.3 linha 376) —
  ADR-0055bis candidata; bloqueada por shaping XL.
- Show rules sobre weight (`#show set text(weight: bold): ...`)
  — bloqueada por regex em L1.

---

## §7 — Risco residual

Risco principal: A.4 → (ii) (precisa novo field em `FrameItem::Text`)
viola nominalmente ADR-0098. Mitigação: A.0 inclui verificação
empírica directa (`grep` em `export.rs`); se A.0 já mostra hits,
P289 não procede com promoção ADR meta + investigação separada
sobre por que P139 documentou faux-bold sem campo emit.

Risco secundário: assimetria oculta nos 3 fields restantes
(`tracking`/`leading`/`font`) — algum pode ter consumer não-paralelo
exigindo abordagem diferente. Mitigação: §5 não-objectivo proíbe
materializar todos juntos; cada um vai a passo próprio.

Risco terciário: promoção ADR meta indevida do padrão §8.2.
Mitigação: §5 condicional; relatório distingue citação de gatilho
real (paralelo a §A.4 P288 estrito).

Risco quaternário: bug latente em consumer P139 faux-bold detectado
em A.5 — pode ser benéfico (paralelo NBSP P288) mas pode também
forçar refactor que ultrapassa scope XS. Mitigação: §3 ponto 4
inclui testes fronteira (100/700/900/450); se bug grave
emergir e refactor for >50 LOC, abrir sub-passo P289.0 dedicado e
adiar P289 propriamente dito. Decisão na Fase A.5.

Risco quinário: P289 e P288 conjuntamente disparam **2 padrões
meta simultaneamente** (§8.2 N=5 + §8.3 N=6). Mitigação: §5
explicita "apenas uma ADR meta por passo"; preferir §8.2 (mais
evidência empírica directa neste passo) e adiar §8.3 para passo
próprio.

---

## §8 — Ponteiros

- Tipo a modificar: `01_core/src/entities/style.rs` (`Style` enum,
  6 variants pós-P288).
- Tipo relacionado: `StyleDelta` (campo `weight: Option<u16>` ou
  similar, verificar A.1.2).
- Caminho parse `#set text(weight: ...)`: P139 (faux-bold);
  precedente directo do parse-driven path actual.
- Precedente directo (mesma assimetria fechada): **P288**
  (`Style::Lang(Lang)`).
- Precedente "activação posterior de feature graded" §8.2 P288 N=4:
  P285 + P286 + P287 + P288. P289 = candidato N=5.
- Precedente "refutação pragmática" §8.3 P288 N=5: candidato a
  promoção ADR meta paralela (não escolhido P289).
- ADR aplicável: **ADR-0098** (recém-formalizada P288) — single
  source of truth como invariante anti-bug. P289 confirma robustez.
- ADR processual: ADR-0065 (inventariar-primeiro; 5 critérios
  cobertos por A.0-A.5).
- ADR cultural: P273.17 §0 (anti-padrão over-formalização;
  condicional promoção; uma ADR meta por passo).
- ADR estilo: ADR-0038 (`Style` enum divergência intencional);
  ADR-0040 (`#set text` activation P102); ADR-0054 (`weight`
  graded por faux-bold).

---

*Spec P289 produzida 2026-05-19 pós-P288 (ADR-0098 formalizada
+ assimetria `lang` fechada). Frente `P-style-weight-variant` —
extensão simétrica do enum `Style` com `Weight(...)` paralela a
P288 `Lang(Lang)`. Fecha 1/4 da assimetria residual P288 §7 risco
terciário (4 fields → 3 restantes pós-P289). Fase A obrigatória
com **5 secções** (A.0-A.5 — uma a mais que P288 por novo requisito
operacional "potencial de reuso ADR-0098" + secção dedicada à
detecção de bugs latentes per padrão P288 §8.4 N=1). Critério de
fecho condicional em A.4: se A.4 → (i), hash `export.rs 66cb8ac3`
preservado pelo 6º passo consecutivo + N=5 do padrão §8.2 "activação
posterior de feature graded" dispara empiricamente → **promoção ADR
meta paralela a ADR-0098**. Aplicação directa de ADR-0098 confirma
invariante perene. Honestidade epistémica explícita: `Weight(u16)`
ou `Weight(FontWeight)` é variant atómico (não rico) — padrão N=4
"variant rico" permanece inalterado. **Apenas uma ADR meta por
passo** (P273.17 §0) — se §8.3 (refutação pragmática) também atingir
limiar simultâneo, preferir §8.2. Sem caps LOC ou magnitude (P282
§7).*
