# Passo 292 — `Style::Font` variant (último da série P288-P292)

**Frente**: `P-style-font-variant` (rank 1 do relatório P291 §9).
**Origem**: P291 §5.8 — assimetria residual 4/5 fechada; resta
apenas `font`. P292 = passo final da série cirúrgica P288-P292.
**Pré-requisitos**: nenhum bloqueador formal; **A.5' P291 valida
prossecução**.
**Origem secundária**:
- Tabela B.3 lista 9 variants pós-P291 (sem `Font`).
- Tabela A.3 linhas 88-91: `text.font` string (P140B); array (P141);
  multi-doc (P146); dict (scope-out per ADR-0054bis).
- Tabela B.4 linha 354: `StyleDelta.font: implementado` (P140B+P141+P146).
- P291 §9 ponto 1: **aviso explícito** — `FontList` provavelmente
  **não é `Copy`**; pode forçar `Style: !Copy`.

---

## §1 — Objectivo

Adicionar variant `Style::Font(...)` ao enum `Style` em
`01_core/src/entities/style.rs`, integrar com a cascade existente
(parse `#set text(font: "Inter")` → `Style::Font(...)` → acumula
em `StyleDelta.font`), e **fechar 5/5 da assimetria residual P289
§5.6**. Pós-P292, série cirúrgica P288-P292 termina naturalmente.

Razão de ser:

1. **Marco arquitectural: fecho da série P288-P292** — última
   reaplicação do padrão "activação posterior" para campos
   `StyleDelta` sem variant correspondente. Não há mais campos
   ortogonais para fechar pós-P292; sequência cirúrgica
   termina.
2. **Risco arquitectural genuíno em A.2** — pela primeira vez,
   opção (a) trivial paralela aos passos anteriores
   (Length/u16/Lang) **pode não ser estructuralmente forçada**.
   `FontList` é wrapper composto; estrutura de A.2 está em aberto.
3. **Potencial gatilho §8.3 N=6** — refutação pragmática
   significativa nova se A.2 escolher opção (b)/(c)/(d) em vez de
   (a). N=5 está estável desde P287; refutação significativa
   nova em P292 atinge N=6 → candidato a promoção ADR meta.
4. **Aplicação directa de ADR-0098 + ADR-0099** — esperado
   preservar hash `export.rs` (9º passo consecutivo) se A.4 → (i).
   Reaplicação ADR-0099 N=8 cumulativo.
5. **A.5' anti-reflexão N=2** — segunda aplicação do padrão §8.6
   inaugurado em P291. Cuidado especial: P292 tem **mais
   probabilidade de revelar paradigma consumer genuinamente novo**
   (font selection via `FontVariant`/`FontBook`) do que P291 →
   protege contra rubber-stamp por **construção factual**, não
   apenas vigilância.

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória; 6 secções)

Cinco secções A.0-A.5 paralelas a P291 + **A.5' reaplicada**.

### A.0 — Potencial de reuso ADR-0098

| Verificação | Esperado | Procedimento |
|---|---|---|
| `grep "font" 03_infra/src/export.rs` | Hits prováveis (PDF font selection consome `style.font`) | Listar e classificar |
| `FrameItem::Text` precisa de novo field `font`? | Esperado **não** — `TextStyle.font` já existe (P140B/P141/P146) | Inspecção literal |
| Sítios consomem via `FrameItem::Text.style.font` (P136 paradigm)? | Esperado **sim** | A.0 documenta |
| Sítios consomem via `chain.font()` directo em emit? | Esperado **não** | Se aparecer, violação ADR-0098 |
| Hash `export.rs` esperado | **Preservado bit-exact** pelo 9º passo se paradigma P136 confirmar | Verificação final |

**Diferença material vs P290 A.0**: P290 confirmou paradigma com
`Tc` operator em tracking (2 hits via `style.tracking`); P292 espera
algo análogo mas para **font selection** (e.g. `Tf` operator que
selecciona font dictionary no PDF). Verificar literalmente.

### A.1 — Inventário literal do caminho actual `font`

8 sub-secções com atenção especial à **A.1.2** (tipo
`StyleDelta.font`):

1. **A.1.1 — `Style` enum pós-P291** — confirmar 9 variants.
2. **A.1.2 — `StyleDelta.font` tipo** — **CRÍTICO**. Verificar
   literalmente:
   - É `Option<FontList>`?
   - É `Option<Vec<FontFamily>>`?
   - É `Option<EcoVec<FontFamily>>` ou tipo persistente partilhado?
   - `FontList` (ou tipo equivalente) é `Copy`? `Clone`? `PartialEq`?
3. **A.1.3 — `push_styles` cascade match** — 9 variants exaustivas;
   `Style::Font(...)` adiciona 10º arm.
4. **A.1.4 — `delta.font` write site único actual** — esperar
   parse em `eval/rules.rs:???` (P140B+P141+P146 referências).
5. **A.1.5 — `delta.font` read sites** — `chain.font()` walk
   up-the-chain.
6. **A.1.6 — Consumers actuais (DISTINTIVO)** — verificar
   literalmente; provável envolver `FontBook::select`/`FontVariant`
   resolver:
   - Layouter consumer (P140B single dispatch via `FontBook::select`).
   - Array fallback consumer (P141).
   - Multi-doc consumer (P146).
   - **`FontBook` global resolver** — interage com cascade?
   - **`TextStyle.font` capture?** — confirmação.
   - **Emit consumer** — `Tf` operator em PDF? Via
     `FrameItem::Text.style.font` (paradigma P136)?
7. **A.1.7 — `FrameItem::Text`/`Frame` emit** — verificar `grep
   "font" export.rs` literalmente.
8. **A.1.8 — Diagrama de fluxo** — produzir genuinamente;
   provavelmente mostrará caminho consumer adicional (font
   resolution via `FontBook::select` antes do consumo em layout).

### A.2 — Estrutura do variant `Style::Font` (decisão arquitectural não-trivial)

**Pela primeira vez na série P288-P292, opção (a) trivial pode não
ser estructuralmente forçada.** Decisão genuína:

| Opção | Estrutura | Prós | Contras |
|---|---|---|---|
| **(a)** `Font(FontList)` — paralelo absoluto P288-P291 | Simetria absoluta; código trivial | Se `FontList` não é `Copy`, **`Style: Copy` perdido** (impacto cumulativo cross-codebase) |
| **(b)** `Font(Arc<FontList>)` — wrapper Arc | `Arc<T>: Copy` se `T: ?Sized + Clone`; preserva `Style: Copy` | Indirecção runtime; alocação heap em cada style push |
| **(c)** `Font(Box<FontList>)` — wrapper Box | Preserva semântica owned | `Box: !Copy`; mesmo problema (a) |
| **(d)** `Font(EcoString)` — apenas primeiro nome da lista | Trivial `Copy`; perde semântica array fallback (P141) | **Divergência semântica grave** vs `StyleDelta.font: Option<FontList>` |
| **(e)** Aceitar perda de `Copy` em `Style` enum | Estrutura natural; código L1 puro | Impacto cumulativo em call sites que assumem `Copy` (chamadas via `*style` desreferenciamento) |
| **(f)** `Font(EcoVec<EcoString>)` — manipulação cuidadosa | `EcoVec: Clone + cheap` (estrutura persistente Typst-like) | Adiciona dependência; verificar A.1.2 se já existe |

**Decisão genuína** (per ADR-0065 critério #1):

- Se A.1.2 revelar `StyleDelta.font: Option<EcoVec<EcoString>>` ou
  tipo `Copy`-friendly equivalente, **(a)** preserva paradigma da
  série.
- Se A.1.2 revelar `Option<Vec<FontFamily>>` (não `Copy`), opção (a)
  **força** `Style: !Copy` — decisão estructural que merece A.2.1
  detalhada sub-secção.

**Default sugerido**: não há. Decisão genuína condicional a A.1.2.

**Implicação gatilho §8.3 N=6**: se A.2 escolher (b)/(e) com
justificação empírica genuína (refutação significativa do paradigma
trivial paralelo), §8.3 atinge **N=6 cumulativo** — candidato a
promoção ADR meta paralela.

### A.3 — Integração com `StyleDelta`

| Opção | Comportamento |
|---|---|
| **(α)** `delta.font = Some(...)` — last-write wins (paridade 9 arms) | Padrão |
| **(β)** Merge específico — array fallback chain combinar parent + child? | Vanilla typst: last-write per documentado por ADR-0040 |

Default sugerido: **(α)**. **(β)** rejeitada salvo se A.1 revelar
merge no parse path actual (improvável dado P141 array é dentro
de uma única chamada `#set text`, não cross-set).

### A.4 — Impacto em `FrameItem::Text` e emit

Verificação ADR-0098:

| Opção | Mecanismo | Implicação `export.rs` |
|---|---|---|
| **(i)** `Tf` operator (PDF font select) lê de `FrameItem::Text.style.font` (paradigma P136) | `export.rs` **preservado** se P140B/P141/P146 já estabeleceram este paradigma; 9º passo consecutivo |
| **(ii)** Emit lê `chain.font()` directamente | Violação ADR-0098 |
| **(iii)** Híbrido — reflector | Caso edge |
| **(iv)** `FrameItem::Text.font_id` (já existente, não `font: Option<FontList>`) — verificar A.1.7 | Caso especial: emit pode consultar `font_id` (resolvido em layout), não `style.font` directo |

Default sugerido: **(i)** ou **(iv)** consoante A.1.7. **(iv)** é
possibilidade real dado layout resolve font name → font_id antes
do emit.

### A.5 — Detecção de bugs latentes

Cenários fronteira:

- `FontList` com 1 elemento — caso típico.
- `FontList` com 3+ elementos (fallback chain) — caso P141.
- `FontList` vazia — comportamento vanilla? Erro? Default?
- `FontList` com nome de font não-disponível — fallback P141 activa.
- **Caso edge**: 2 `Style::Font` consecutivos em cascade — last-write
  wins (paridade P288-P291).

**Atenção particular ao `FontList` vazia** — pode revelar bug
latente em consumer P140B/P141 que não foi exercitado em testes
prévios (cascade-driven).

### A.5' — Verificação anti-reflexão (N=2 do padrão §8.6 P291)

Reaplicação do padrão inaugurado em P291. 4 verificações:

1. **Comparação literal A.1.6 P288/P289/P290/P291/P292** — esperar
   5 paradigmas distintos:
   - P288 lang: cross-module.
   - P289 weight: TextStyle method.
   - P290 tracking: per-glyph + Tc emit.
   - P291 leading: per-line via peek.
   - **P292 font (a confirmar)**: provavelmente "resolução via
     `FontBook::select` antes do consumo" — paradigma novo
     "indirect resolution via global registry".
2. **A.0 produzido empiricamente** (não citado).
3. **Elemento estructuralmente novo identificado**:
   - **A.2 não-trivial em si** já é elemento novo (vs P288-P291
     onde opção (a) era forçada estructuralmente). Decisão A.2
     genuína vs trivial é evidência factual de não-rubber-stamp.
   - Adicionalmente: paradigma `FontBook::select` indirect
     resolution.
4. **Decisão sobre passo seguinte** — P293 será **passo
   ortogonal obrigatório** (math-accent-cancel, curve-geometry,
   ou outro), porque:
   - Série P288-P292 fecha 5/5 da assimetria.
   - Não há mais campos `StyleDelta` para reaplicação cumulativa.
   - Próximo passo cumulativo paralelo é estructuralmente
     impossível (sem assimetria residual).
   - Quebra natural da sequência — não há decisão a tomar.

---

## §3 — Materialização

Após Fase A (incluindo A.5' anti-reflexão) produzir todos os
inventários e decisões:

1. Adicionar `Font(<tipo per A.2>)` ao enum `Style`.
   - Se A.2 → (a): `Font(FontList)`; impacto `Style: Copy` per
     A.1.2.
   - Se A.2 → (b): `Font(Arc<FontList>)`.
   - Se A.2 → (e): `Font(FontList)` + remoção explícita de
     `Style: Copy` derive (refactor cumulativo em call sites
     `*style`).
   - Outras opções conforme A.2.
2. Implementar match-exhaustive arm em `StyleChain::push_styles`:
   `Style::Font(f) => delta.font = Some(<acessor per A.2>)`.
3. Match exaustivo continua (defesa cumulativa P288-P292).
4. Testes:
   - L1 unitário variant: ctor; PartialEq; Clone (se aplicável).
   - L1 unitário cascade: `push_styles` projecta no delta.
   - L1 unitário injection: `Content::Styled(body,
     Styles::from_iter([Style::Font(<list>)]))`.
   - L1 unitário last-write: 2 `Style::Font` consecutivos.
   - L1 unitário catalog: 9 → 10 variants.
   - L1 unitário fronteiras: 5 cenários A.5.
   - L1 unitário **`FontList` vazia**: comportamento documentado.
   - L3 integração: `#set text(font: "Inter"); ...` via
     `Content::Styled` produz mesmo PDF que via parse.
   - L3 integração regression: PDFs pré-P292 (P140B/P141/P146)
     produzem bytes idênticos pós-P292.
5. Aplicação ADR-0098 obrigatória — A.0 + A.1.7 documentam.
6. **Promoção ADR meta condicional**:
   - Se A.2 → (a) trivial + A.5' N=2 do padrão §8.6 inalterado:
     **sem promoção**.
   - Se A.2 escolher (b)/(e) com refutação genuína do paradigma
     trivial paralelo: §8.3 **atinge N=6**; **considerar
     promoção ADR meta paralela** "Refutação pragmática como
     padrão epistémico"; **mas apenas se gatilho disparar
     genuinamente** (P273.17 §0 anti-padrão; uma ADR meta por
     passo).
   - Se §8.6 A.5' atinge N≥3 cumulativo com paradigmas
     genuinamente distintos: **considerar promoção também**.
     **Mas apenas uma por passo** — escolher a com mais evidência.
7. Actualizar L0:
   - `00_nucleo/prompts/entities/style.md` (+10º variant).
   - Tabela B.3 — `Font(...)` 10º variant.
   - Tabela B.4 linha 354 — nota cruzada P292.
   - Propagar hashes.
8. Actualizar diagnóstico:
   - `diagnostico-style-font-passo-292.md` com 6 secções
     A.0-A.5+A.5'.

**Sem caps** (per P282 §7). Estimativa de testes: ~10-15 (paralelo
P291 +11 mas com **cenário FontList vazia** + possíveis testes
adicionais consoante decisão A.2).

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P291: 2 783 testes.
  Esperado: ~2 793 a ~2 798.
- `crystalline-lint` zero violations.
- Hash L0 `style.md` muda (+1 variant em B.3).
- Hash L0 `content.md` **preserved**.
- Hash L0 `stdlib.md` **preserved**.
- Hash L0 `export.rs` **condicional**:
  - Preserved se A.4 → (i)/(iv) → confirma ADR-0098 vigente; **9º
    passo consecutivo**.
  - Muda se A.4 → (ii)/(iii) → investigação obrigatória (violação
    nominal ADR-0098).
- **Regressão bit-exact validada** — todos os testes font-aware
  pré-P292 (P140B+P141+P146 parse-driven) continuam verdes
  byte-exact.
- Tabela B.3 actualizada com `Font(...)` (10º variant) + nota
  **"assimetria residual fechada 5/5"**.
- Tabela B.4 linha 354 com nota cruzada P292.
- Diagnóstico A.0+A.1+A.2+A.3+A.4+A.5+A.5' produzido.
- **Promoção ADR meta condicional ao gatilho disparar genuinamente**:
  - §8.3 N=6 se A.2 escolhe não-trivial.
  - §8.6 N=2 se A.5' identifica paradigma genuinamente novo.
  - **Uma ADR meta por passo no máximo**.
- Bug latente colateral (se descoberto em A.5) registado e fixado.
- **Marco arquitectural registado**: série P288-P292 termina;
  próximo passo é ortogonal por construção.

---

## §5 — Não-objectivos

- **Não** materializar `text.font` dict (gap 8 DEBT-52;
  ADR-0054bis condicional). Requer `regex` em L1; fora de scope.
- **Não** alterar `FontList` semanticamente. Se A.1.2 revelar
  estrutura específica, preservar.
- **Não** promover múltiplas ADRs meta neste passo. Mesmo se
  §8.3 N=6 e §8.6 N=2 disparem simultaneamente, escolher uma
  (preferir §8.3 — mais evidência cumulativa).
- **Não** continuar série cirúrgica cumulativa pós-P292. P293
  será ortogonal por construção (não há mais campos para
  reaplicação).
- **Não** materializar `FontVariant` selection variant-aware
  (Tabela A.3 linha 376; ADR-0055bis candidata). Distinto.
- **Não** alterar `Style: Copy` semanticamente sem justificação
  empírica explícita em A.2. Se opção (e) escolhida, registar
  detalhadamente o impacto cumulativo.

---

## §6 — Pendências relacionadas

Resolve:
- **Última 1/5 da assimetria residual P289 §5.6** — `font` é o
  último campo.
- **Marco arquitectural**: fecho completo da assimetria B.3↔B.4.

Não resolve (continua aberto):
- `text.font` dict (gap 8) — ADR-0054bis condicional; passo
  distinto.
- `FontVariant` variant-aware (linha 376) — ADR-0055bis candidata.
- Show rules sobre font — bloqueada por regex em L1.

---

## §7 — Risco residual

Risco principal: **A.2 escolhe opção que força `Style: !Copy`**
(opção (e)). Impacto cumulativo em call sites que assumem
`Copy` (via `*style` desreferenciamento). Mitigação: A.1 deve
incluir **inventário literal de call sites** que dependem de
`Style: Copy` antes de A.2 decidir. Se >10 sítios, considerar
opção (b) Arc-wrapper apesar do custo runtime.

Risco secundário: A.1.2 revela tipo inesperado para
`StyleDelta.font` (e.g. `Option<EcoString>` simplificado, sem
`FontList`). Mitigação: A.1.2 explícito; se simplificação radical
descoberta, opção (a) torna-se trivial paralelo (não há refutação
significativa).

Risco terciário: emit consume `Tf` operator de forma idiossincrática
que viola paradigma P136 (e.g. via `FrameItem::Text.font_id`
resolvido em layout em vez de `style.font` directo). Mitigação:
A.4 → (iv) acomoda este caso; não é violação ADR-0098 — é
paradigma alternativo consciente. Documentar.

Risco quaternário: `FontList` vazia revela bug latente em
P140B/P141 não exercitado em testes cascade-driven. Mitigação:
A.5 cenário dedicado; se bug grave (e.g. panic ou comportamento
indefinido), abrir sub-passo P292.0 antes de prosseguir.

Risco quinário: **sequência reflexa materializada apesar de
A.5'**. Mitigação: A.2 não-trivial **por construção factual** (não
apenas vigilância) — opção (a) trivial paralela tem chance real
de ser estructuralmente errada. Se A.2 confirma trivial paralelo
(a) sem refutação genuína, A.5' regista honestamente — mas
materialização **prossegue** porque é o último passo da série
(não há "passo seguinte cumulativo" para evitar).

Risco senário (anti-meta): promoção ADR meta indevida por §8.3
N=6 forçada artificialmente. Mitigação: critério §3 ponto 6
condicional ao gatilho disparar **genuinamente**. Se A.2 → (a)
trivial, §8.3 não atinge N=6; não promover.

---

## §8 — Ponteiros

- Tipo a modificar: `01_core/src/entities/style.rs` (`Style` enum,
  9 variants pós-P291).
- Tipo relacionado: `StyleDelta.font` — verificar A.1.2.
- Tipo `FontList`: localização e estrutura literal verificadas em
  A.1.2.
- Caminho parse: P140B (string), P141 (array), P146 (multi-doc).
- Caminho consumer: `FontBook::select` (P140B) + array fallback
  (P141) + multi-doc (P146).
- Precedente directo (mesma assimetria fechada): P288/P289/P290/P291.
- Padrão §8.1 ADR-0099 — P292 = N=8 cumulativo (sem promoção;
  limiar atingido P289).
- Padrão §8.2 ADR-0098 — P292 = N=9 cumulativo (sem promoção;
  ADR-0098 formalizada P288).
- Padrão §8.3 "refutação pragmática" — N=5 estável; **P292
  candidato a N=6** se A.2 não-trivial.
- Padrão §8.6 A.5' anti-reflexão — N=1 inaugural P291; **P292
  reaplica N=2**.
- ADR aplicável: **ADR-0098** + **ADR-0099**.
- ADR processual: ADR-0065.
- ADR cultural: P273.17 §0 (anti-padrão; uma ADR meta por passo).
- ADR estilo: ADR-0038, ADR-0040, ADR-0054.
- ADR scope-out: ADR-0054bis condicional (font dict gap 8) —
  fora de scope deste passo per §5.

---

*Spec P292 produzida 2026-05-19 pós-P291 (assimetria 4/5 fechada;
A.5' anti-reflexão inaugurada). Frente `P-style-font-variant` —
**último passo cumulativo da série P288-P292**. Fecha 1/1 da
assimetria residual P291 §5.8 (5/5 total). Fase A obrigatória com
**6 secções** A.0-A.5+A.5' (paralelo P291). **Diferença material
crítica vs P288-P291**: A.2 **decisão arquitectural não-trivial
genuína** — `FontList` (tipo per A.1.2) pode não ser `Copy`,
forçando escolha entre 6 opções (a)-(f) com trade-offs concretos.
Critério de fecho condicional em A.4: se A.4 → (i)/(iv), hash
`export.rs 66cb8ac3` preservado pelo 9º passo consecutivo →
confirma ADR-0098 robusta. **Promoção ADR meta condicional ao
gatilho disparar genuinamente**: §8.3 N=6 se A.2 escolhe
não-trivial com refutação significativa do paradigma trivial
paralelo; §8.6 N=2 se A.5' identifica paradigma consumer
genuinamente novo; **uma ADR meta por passo no máximo** (P273.17
§0). **Marco arquitectural**: pós-P292, série cirúrgica termina
naturalmente; P293 será ortogonal por construção (não há mais
campos para reaplicação cumulativa). A.5' P292 é primeira
reaplicação após P291 inaugural — protege contra rubber-stamp por
**construção factual** (A.2 não-trivial em si), não apenas
vigilância. Honestidade epistémica preservada: variant atómico
ou wrapper conforme A.2; padrão N=4 "variant rico" inalterado.
Sem caps LOC ou magnitude (P282 §7).*
