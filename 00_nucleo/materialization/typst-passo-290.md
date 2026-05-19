# Passo 290 — `Style::Tracking` variant

**Frente**: `P-style-tracking-variant` (rank 1 do relatório P289 §9;
P289.1 candidato).
**Origem**: P289 §5.6 — "Assimetria residual: 3/4 ainda abertos —
`tracking`/`leading`/`font`". P290 = P289.1 (sequência directa
paralela ao P288→P289 mecanicamente, mas com risco distinto em A.0
por causa do `Tc` operator emit).
**Pré-requisitos**: nenhum bloqueador.
**Origem secundária**: Tabela B.3 lista 7 variants pós-P289 (sem
`Tracking`). Tabela A.3 linha 93 lista `text.tracking` como
`implementado` desde P137, com nota "`Tc` operator em PDF". Tabela
B.4 linha 351 lista `StyleDelta.tracking` como `implementado` desde
P137.

---

## §1 — Objectivo

Adicionar variant `Style::Tracking(Length)` ao enum `Style` em
`01_core/src/entities/style.rs`, integrar com a cascade existente
(parse `#set text(tracking: 0.1em)` → `Style::Tracking(...)` →
acumula em `StyleDelta.tracking`), e fechar 1/3 da assimetria
residual P289 §5.6 (3 fields → 2 restantes pós-P290).

Razão de ser (paralelo arquitectural directo a P288/P289):

1. **Fecha 1/3 da assimetria residual P289 §5.6** — `tracking` é o
   1º dos 3 fields restantes (`tracking`/`leading`/`font`).
2. **Reaplicação de ADR-0099 sem promoção nova** — limiar N=5 já
   atingido em P289; este passo é **N=6 cumulativo do padrão §8.1
   (formalizado em ADR-0099)** sem necessidade de promoção
   adicional. Padrão "patch cirúrgico sequencial paralelo" §8.5 P289
   N=2 → **N=3** pós-P290 (limiar formalização N≥4 ainda longe;
   relatório §8.5 desqualifica passos cumulativos triviais como
   reaplicações genuínas para esse padrão).
3. **Aplicação directa de ADR-0098** — Fase A.0 obrigatória (per
   directiva P288 §9 / template P289). **A.0 desta vez é
   não-trivial**: ao contrário de `lang` (P288 A.0 zero hits
   trivial) e `weight` (P289 A.0 zero hits trivial), `tracking`
   tem nota P137 mencionando "`Tc` operator em PDF" — necessário
   verificar literalmente se emit consulta `tracking` ou se é
   resolvido no Layouter (paradigma `TextStyle` capture).
4. **Confirmar ADR-0098 vigente pelo 7º passo consecutivo** se A.4
   → (i) — hash `export.rs 66cb8ac3` preservado. **Mais robusto
   que P289** dado risco real em A.0.

Não-objectivos arquitecturais explícitos em §5.

---

## §2 — Fase A — diagnóstico empírico (obrigatória)

Cinco secções A.0-A.5 paralelas a P289 §A.0-A.5.

### A.0 — Potencial de reuso ADR-0098 (não-trivial desta vez)

| Verificação | Esperado | Plano de inspecção |
|---|---|---|
| `grep "tracking\|Tc " 03_infra/src/export.rs` | **Hits prováveis** dado nota P137 "`Tc` operator em PDF" | Listar todos os sítios; classificar por tipo |
| Sítios encontrados consomem `FrameItem::Text.style.tracking` (P136 paradigm)? | Esperado **sim** | Se sim → ADR-0098 vigente; nenhum novo field em FrameItem |
| Sítios encontrados consomem `chain.tracking()` directamente em emit? | Esperado **não** (emit lê de FrameItem, não de chain) | Se sim → divergência grave do paradigma; investigar |
| Hash `export.rs` esperado | **Preservado bit-exact** pelo 7º passo se paradigma P136 confirmar | Verificação final pós-materialização |

**Diferença material vs P289 A.0**: P289 esperava zero hits
literais. P290 espera **hits no formato `Tc` operator** mas
verificados como consumo via `TextStyle` captado em `FrameItem::Text`
(paradigma P136 + P137). A.0 deve documentar a distinção
explicitamente.

Se A.0 revelar que tracking é consumido directamente via
`chain.tracking()` em emit (não via `FrameItem::Text.style`), isto
seria **violação nominal de ADR-0098** e exigiria investigação
separada (paralelo a risco quaternário P289 §7).

### A.1 — Inventário literal do caminho actual `tracking`

8 sub-secções paralelas a P289 §A.1:

1. **A.1.1 — `Style` enum pós-P289** — confirmar 7 variants
   (P290 → 8).
2. **A.1.2 — `StyleDelta.tracking`** — verificar tipo (esperado
   `Option<Length>` per Tabela B.4 linha 351; `Length` enum
   materializado P25/P127).
3. **A.1.3 — `push_styles` cascade** — match exaustivo sobre 7
   variants; `Style::Tracking(Length)` adiciona 8º arm.
4. **A.1.4 — `delta.tracking` write site único actual** — esperar
   parse-driven em `eval/rules.rs:???` (precedente P137).
5. **A.1.5 — `delta.tracking` read** — `chain.tracking()` walk
   up-the-chain.
6. **A.1.6 — Consumers actuais** — esperar:
   - Layouter consumer (P137 — propaga tracking para `TextStyle`).
   - `TextStyle.tracking` capture (paradigma P136/P139).
   - `export.rs` emit (P137 — `Tc` operator) **mas via**
     `FrameItem::Text.style.tracking`, não via `chain` directo.
7. **A.1.7 — `FrameItem::Text` emit** — **verificar literalmente**
   se emit lê `Tc` operator de `style.tracking` (esperado) ou via
   chain directo (violação ADR-0098).
8. **A.1.8 — Diagrama de fluxo** — paralelo P288/P289 §A.1.8 mas
   com nota explícita "emit consome via `TextStyle` capture, não
   via chain directo".

### A.2 — Estrutura do variant `Style::Tracking`

Decisão arquitectural (per ADR-0065 critério #1):

| Opção | Estrutura | Prós | Contras |
|---|---|---|---|
| **(a)** `Tracking(Length)` — paralelo `Lang(Lang)` P288 | Tipo `Length` enum materializado (P25/P127) | `Length` é `Copy`? — verificar A.1.2; se não, `Style` perde `Copy` (paralelo a P288 `Lang(Lang)`) |
| **(b)** `Tracking(f64)` — armazena valor em pontos directamente | Storage raw; `Style` mantém `Copy` trivial | Perde tipo semântico `Length` (Em vs Pt) — divergência com `StyleDelta.tracking: Option<Length>` |
| **(c)** `Tracking(LengthVariant)` — wrapper sub-enum só com variants em uso | Domínio restrito; preserva semântica | Indirecção desnecessária; replica `Length` parcial |

Default sugerido: **(a)** — paridade absoluta com
`StyleDelta.tracking` field (Tabela B.4); preserva tipo semântico.
**Verificar A.1.2**: se `Length` é `Copy` (esperado dado é enum
plano de variantes pequenas como `Pt(f64)`/`Em(f64)`), opção (a)
preserva `Style: Copy`. Se `Length` não for `Copy`, P288 já
estabeleceu precedente que `Style::Lang(Lang)` levou `Style` a
perder `Copy` mesmo paradigma — não-objectivo.

**(b)** rejeitada — perde semântica que `StyleDelta` preserva.
**(c)** rejeitada — divergência sem ganho.

### A.3 — Integração com `StyleDelta`

| Opção | Comportamento |
|---|---|
| **(α)** `delta.tracking = Some(*l)` — last-write wins (paridade absoluta P288/P289) | Padrão dos 7 arms existentes |
| **(β)** Merge aditivo — `delta.tracking = sum(parent, child)` | Vanilla typst: tracking é last-write; merge ausente |

Default sugerido: **(α)** salvo se A.1 revelar merge no parse path
actual (improvável).

### A.4 — Impacto em `FrameItem::Text` e emit (aplicação ADR-0098)

| Opção | Mecanismo | Implicação `export.rs` |
|---|---|---|
| **(i)** `FrameItem::Text.style.tracking` (P136 paradigm) já consultado em emit (P137 `Tc` operator); P290 apenas estende caminho de entrada | `export.rs` **preservado bit-exact** (7º passo consecutivo) → confirma ADR-0098 robusta |
| **(ii)** `FrameItem::Text` precisa novo field `tracking: Option<Length>` | Improvável — P137 já implementou `Tc` operator sem novo field |
| **(iii)** Híbrido — reflector trivial | Caso edge; A.0 deve detectar |
| **(iv) NOVA** | A.0 revela que emit lê `chain.tracking()` directamente (violação nominal ADR-0098) | Investigação separada obrigatória |

Default sugerido: **(i)** se A.0 + A.1.7 confirmarem paradigma
P136. **(iv) imprevisto**: violação de ADR-0098 disparada por
inspecção empírica. Mitigação detalhada em §7.

**Implicação gatilho meta**: padrão §8.1 ADR-0099 já formalizado;
sem promoção neste passo. Padrão §8.5 P289 "patch cirúrgico
sequencial paralelo" atinge N=3 (P288+P289+P290); aguardar N≥4 com
critério de "reaplicação não-trivial" (relatório §8.5 desqualifica
passos cumulativos triviais).

### A.5 — Detecção de bugs latentes (padrão P288 §8.4)

P288 inaugurou N=1 (NBSP fix). P289 §A.5 verificou 4 fronteiras
weight e não encontrou bugs (N=1 estável). P290 deve testar:

- `Style::Tracking(Length::Pt(0.0))` — zero tracking (default
  implícito).
- `Style::Tracking(Length::Pt(1.0))` — tracking pequeno positivo.
- `Style::Tracking(Length::Em(0.5))` — tracking em em-units (50%
  da font-size).
- `Style::Tracking(Length::Pt(-0.5))` — tracking **negativo**
  (vanilla typst aceita; verificar comportamento cristalino).
- `Style::Tracking(Length::Pt(10.0))` — tracking grande (verificar
  word-wrap continua a funcionar).

**Se algum revela divergência ou bug**, registar como ganho
colateral. **Atenção particular ao tracking negativo** — kerning
artificial via tracking negativo é uso vanilla legítimo.

---

## §3 — Materialização

Após Fase A produzir inventário + estrutura + integração + impacto
emit + plano de detecção bugs:

1. Adicionar `Tracking(Length)` ao enum `Style`.
2. Implementar match-exhaustive arm em `StyleChain::push_styles`:
   `Style::Tracking(l) => delta.tracking = Some(*l)` (ou
   `delta.tracking = Some(l.clone())` se `Length` não for `Copy`).
3. Match exaustivo continua — defesa cumulativa P288+P289+P290.
4. Testes (paralelo P289 §4 + cenários A.5):
   - L1 unitário variant: ctor; PartialEq; Clone se aplicável.
   - L1 unitário cascade: `push_styles` projecta no delta.
   - L1 unitário injection: `Content::Styled(body,
     Styles::from_iter([Style::Tracking(Length::Em(0.1))]))` faz
     `chain.tracking() == Some(Length::Em(0.1))`.
   - L1 unitário last-write: 2 `Style::Tracking` consecutivos.
   - L1 unitário catalog: actualizar 7 → 8 variants.
   - L1 unitário fronteiras: 5 cenários A.5.
   - L3 integração: `#set text(tracking: 0.1em); texto via
     Content::Styled` produz PDF com `Tc 0.1em` consistente com
     parse-driven path.
   - L3 integração regression: PDF pré-P290 (P137 parse-driven)
     produz bytes idênticos pós-P290.
5. Aplicação ADR-0098 obrigatória:
   - Diagnóstico A.0 cita ADR-0098 + verificação empírica do
     `Tc` operator (não-trivial; documentar caminho via
     `FrameItem::Text.style.tracking`).
   - Relatório §1 confirma hash `export.rs` preservado.
6. Promoção ADR meta: **não promover**.
   - ADR-0099 já cobre o padrão §8.1.
   - §8.5 ainda longe de N≥4 com critério não-trivial.
   - §8.3 estável N=5.
7. Actualizar L0:
   - `00_nucleo/prompts/entities/style.md` (+8º variant em B.3).
   - Tabela B.3 — adicionar `Tracking(Length)` como 8º variant.
   - Tabela B.4 linha 351 — nota cruzada P290 (2ª fonte de entrada).
   - Propagar hashes via `crystalline-lint --fix-hashes`.
8. Actualizar diagnóstico:
   - `diagnostico-style-tracking-passo-290.md` com 5 secções
     A.0-A.5.

**Sem caps** (per P282 §7). Estimativa de testes: ~8-12 (paralelo
P289 +9 mas com **5 fronteiras** em A.5 em vez de 4 — tracking
negativo é caso adicional).

---

## §4 — Critério de fecho

- `cargo test --workspace` verde. Baseline P289: 2 763 testes.
  Esperado: ~2 771 a ~2 775.
- `crystalline-lint` zero violations.
- Hash L0 `style.md` muda (+1 variant em B.3).
- Hash L0 `content.md` **preserved**.
- Hash L0 `stdlib.md` **preserved**.
- Hash L0 `export.rs` **condicional**:
  - Preserved se A.4 → (i) → confirma ADR-0098 vigente; **7º passo
    consecutivo** (P282+P285+P286+P287+P288+P289+P290).
  - Muda se A.4 → (ii)/(iv) → investigação obrigatória (violação
    nominal ADR-0098).
- **Regressão bit-exact validada** — todos os testes tracking-aware
  pré-P290 (P137 parse-driven) continuam verdes byte-exact.
- Tabela B.3 actualizada com `Tracking(Length)` (8º variant) + nota
  assimetria residual (2 fields restantes: `leading`/`font`).
- Tabela B.4 linha 351 com nota cruzada P290.
- Diagnóstico A.0+A.1+A.2+A.3+A.4+A.5 produzido.
- **Sem promoção ADR meta nova** — ADR-0099 reaplica sem promoção
  (limiar já atingido em P289).
- Bug latente colateral (se descoberto em A.5) registado e fixado.

---

## §5 — Não-objectivos

- **Não** materializar `Style::Leading`/`Style::Font` simultaneamente.
  P290 é cirúrgico (1 variant); sequência paralela P288→P289→P290
  estabelece patamar para reaplicação per-field.
- **Não** promover nova ADR meta. ADR-0099 já cobre o padrão
  "activação posterior de feature graded". P273.17 §0 proíbe
  promoção mecânica sem gatilho novo genuíno.
- **Não** validar input tracking range. Vanilla typst aceita
  valores positivos e negativos (kerning artificial); cristalino
  preserva paridade.
- **Não** alterar `StyleDelta.tracking` semanticamente. Se já é
  `Option<Length>` (esperado per Tabela B.4 linha 351), apenas
  adicionar 2ª fonte de entrada via `Style::Tracking`.
- **Não** materializar tracking constantes simbólicas. Se P137 não
  as implementou, registar como scope-out.
- **Não** unificar emit `Tc` operator. Se A.0 revelar que emit
  consome tracking de forma idiossincrática (e.g. dois caminhos
  paralelos em export.rs), **registar mas não refactorar** —
  passo distinto se necessário.

---

## §6 — Pendências relacionadas

Resolve:
- **Assimetria Tabela B.3 vs B.4 para `tracking`** — 1/3 da
  assimetria residual P289 §5.6.

Não resolve (continua aberto):
- Outros 2 fields sem variant `Style` correspondente
  (`leading`/`font`) — passos próprios candidatos P290.1 (leading)
  e P290.2 (font). Reaplicações de ADR-0099 sem promoção.
- Show rules sobre tracking — bloqueada por regex em L1.
- Tracking per-glyph (vanilla typst expõe via `text.tracking`
  global; cristalino paralelo) — paridade preservada.

---

## §7 — Risco residual

Risco principal: A.0 revela violação nominal ADR-0098 — emit lê
`chain.tracking()` directamente em vez de via
`FrameItem::Text.style.tracking`. Mitigação: §A.0 inspecção
literal antes de qualquer materialização. Se violação genuína
descoberta:

1. **NÃO procedemos com materialização P290** — abrir sub-passo
   P290.0 dedicado a investigação.
2. ADR-0098 ganha **1ª falha empírica** — registar honestamente.
3. P290 propriamente dito só procede após resolução P290.0.

Esta protecção é nova vs P289 (que tinha A.0 trivial); reflecte
elevação de risco genuino em tracking.

Risco secundário: `Length` não é `Copy` — `Style` enum perde
`Copy` trait. Mitigação: paralelo P288 `Lang(Lang)` estabelece
precedente; se P288 fez perder `Copy`, P290 não regressivo. Se
P288 preservou `Copy` (improvável dado `Lang` ser enum), `Length`
provavelmente também é `Copy` (variantes `Pt(f64)`/`Em(f64)` são
pequenas e copiables). Verificar A.1.2 literalmente.

Risco terciário: tracking negativo descoberto bug em A.5.
Mitigação: §3 ponto 4 inclui cenário explícito; se bug descoberto
e refactor > scope XS, abrir P290.0 paralelo a risco principal.

Risco quaternário: passos cumulativos P290+P290.1+P290.2 atingem
N≥4 do padrão §8.5 (formalização). Mitigação: relatório P289 §8.5
**explicitamente desqualifica** passos cumulativos triviais como
reaplicações genuínas para esse padrão — *"passos próprios
cumulativos não bastam — seria inflação por construção"*.
**P290 não promove §8.5** mesmo se contagem atingir limiar.

Risco quinário: P290 e seus sucessores P290.1/P290.2 entrarem em
"sequência reflexa" — automatização sem reflexão genuína. Mitigação:
cada passo mantém Fase A obrigatória completa; se Fase A degenera
em rubber-stamp idêntico, abrir passo "auditoria meta" antes de
continuar.

---

## §8 — Ponteiros

- Tipo a modificar: `01_core/src/entities/style.rs` (`Style` enum,
  7 variants pós-P289).
- Tipo relacionado: `StyleDelta.tracking: Option<Length>` (P137).
- Tipo `Length`: `01_core/src/entities/length.rs` (ou caminho
  equivalente; P25 + P127).
- Caminho parse `#set text(tracking: ...)`: P137 (Tabela A.3 linha
  93); precedente directo do parse-driven path.
- Caminho emit: P137 `Tc` operator em PDF (`03_infra/src/export.rs:???`).
  **A.0 verifica literalmente** o caminho.
- Precedente directo (mesma assimetria fechada): P288
  (`Style::Lang(Lang)`) + P289 (`Style::Weight(u16)`).
- Padrão §8.1 ADR-0099 — P290 é **6ª aplicação cumulativa** sem
  promoção (limiar já atingido em P289).
- Padrão §8.5 P289 — P290 é **3ª contagem** mas relatório
  desqualifica para promoção.
- ADR aplicável: **ADR-0098** + **ADR-0099** (ambas vigentes
  pós-P289).
- ADR processual: ADR-0065 (inventariar-primeiro; 5 secções
  A.0-A.5 paralelas a P289).
- ADR cultural: P273.17 §0 (anti-padrão over-formalização; **não
  promover** sem gatilho novo).
- ADR estilo: ADR-0038 (`Style` enum divergência intencional);
  ADR-0040 (`#set text` activation P102); ADR-0054 (`tracking`
  graded — mas P137 marca `implementado` não `implementado⁺`;
  verificar A.1).

---

*Spec P290 produzida 2026-05-19 pós-P289 (ADR-0098 + ADR-0099
formalizadas; assimetria `weight` fechada). Frente
`P-style-tracking-variant` — extensão simétrica do enum `Style`
com `Tracking(Length)` paralela a P288 `Lang(Lang)` e P289
`Weight(u16)`. Fecha 1/3 da assimetria residual P289 §5.6 (3
fields → 2 restantes pós-P290). Fase A obrigatória com 5 secções
A.0-A.5 (paralelo P289) **mas com A.0 não-trivial**: P137 emit usa
`Tc` operator em PDF; A.0 deve verificar literalmente que emit
consome via `FrameItem::Text.style.tracking` (paradigma P136) e
não via `chain.tracking()` directo. Critério de fecho condicional
em A.4: se A.4 → (i) confirma paradigma P136, hash `export.rs
66cb8ac3` preservado pelo 7º passo consecutivo (P282+P285+P286+
P287+P288+P289+P290) → confirma ADR-0098 robusta. **Sem promoção
ADR meta** — ADR-0099 cobre o padrão; P273.17 §0 proíbe promoção
sem gatilho novo. Padrão §8.5 P289 "patch cirúrgico sequencial
paralelo" atinge N=3 cumulativo mas relatório §8.5 desqualifica
passos cumulativos triviais para formalização. Honestidade
epistémica preservada: variant atómico (não rico); padrão N=4
"variant rico" inalterado. **Risco principal genuino novo**:
A.0 não-trivial pode revelar violação nominal ADR-0098 — se sim,
abrir P290.0 dedicado e não materializar P290 directamente. Sem
caps LOC ou magnitude (P282 §7).*
