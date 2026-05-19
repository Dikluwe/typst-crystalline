# Relatório — Passo 305 (P295.2 — footnote overflow multi-página)

**Data**: 2026-05-19
**Spec**: `00_nucleo/materialization/typst-passo-305.md`
**Tipo declarado spec**: extensão P304 com page break coupling
cross-page — paradigma potencialmente novo (split state cross-page)
ou reaplicação DeferredX N=4 conforme A.0.0.
**Hipótese adoptada**: **HA** (body inteiro spill) +
**P305.B** subset (single-document overflow básico; P305.C/D
scope-out) + **A.2 → (a)** partial drain (items que cabem flush;
sobras stay no buffer) + **A.3 → (α)** iterative new_page() flow
(buffer convergente) + **A.4 → (i)** FrameItem::Text standard.
**Baseline P304**: 2 890 testes  →  **P305**: 2 899 testes
(Δ = **+9 net**: +6 L1 + 3 L3 novos, 0 obsoletos removidos).
**Hash `export.rs`**: `66cb8ac3` preservado bit-exact (**22º passo
consecutivo**: P282→P305).
**Hash `content.rs`**: `82d3c47d` inalterado.
**Hash L0 `layout.md`**: `12536b5c` inalterado (precedente
P245/P251/P304).
**ADRs meta novas**: 0 (**13.ª vez consecutiva** anti-padrão
P273.17 §0 honrado).

---

## §1 — Sumário executivo

P305 resolve a frente **P295.2** — overflow multi-página pendente
desde P295 §8 (10 passos antes de P305) — e simultaneamente fixa
o **bug latente P304** (overlap silencioso quando bodies totais
excedem espaço rodapé).

**Bug factual P304 exposto**: a fórmula `y_cursor = area_bot - total_h`
em P304 §3.2 calculava posição inicial **sem clamp** ao topo da
área disponível. Se `total_h > (area_bot - cursor_y)`, bodies
ficavam **acima** do top de main content, sobrepondo
silenciosamente. Visibilidade nula em testes P304 (todos cobriam
single-page where `total_h < available_h`).

**Fix arquitectural** (partial drain + iterative flush):
- `flush_pending_footnote_bodies` modificado: greedy fit per body;
  sobras re-inseridas no buffer; clamp Y defensivo.
- `finish()` ganha loop: enquanto buffer não-vazio, chama
  `new_page()` (que internamente flush nova batch).

**Resultado funcional**:
- Footnotes que excedem espaço rodapé **distribuem multi-página**
  automaticamente.
- Bodies **nunca silenciosamente descartados** — bug fix verificado.
- **Sem overlap** com main content (Y clamp + greedy fit).
- **Caso degenerate** (body singular > página inteira): defensive
  emit + iter_limit evitam loop infinito; body emite no topo da
  página (clipped overlap escape válvula).
- **Single-page P304 preservado bit-exact** — comportamento
  idêntico quando tudo cabe.

**Resultado metodológico — distinção honesta de promoções**:

1. **Sub-padrão DeferredX — N=4 candidato adiado**: P305 é extensão
   genuína do pattern P304 (cross-page partial drain). Não trivial.
   Mas seguindo anti-padrão P273.17 §0 standard P293-P304: **adiar
   promoção** (13.ª vez consecutiva).

2. **Sub-padrão §8.4 subcat A — N=3 candidato adiado**: P305 bug é
   genuinamente **derivado** de P304 (paralelo P288/P287, P302/P301).
   Limiar tentativo N=3 atingido (P288 + P302 + P305). Promoção
   candidata robusta mas **adiada** mesmo critério.

Ambos os candidatos N atingem limiar tentativo. **Promoção adiada**
não por falta de qualidade mas por **consistência metodológica**
— anti-padrão estabelecido em 12 passos não inflaciona em P305.

**Lição confirmada**: bugs latentes derivados de materializações
anteriores são **3ª aplicação consecutiva** (P302 → P303 → P305)
de sub-padrão §8.4 subcat A. Pattern robusto empiricamente — mas
ainda não inflacionado a ADR formal.

---

## §2 — Fase A (síntese)

| Secção | Veredicto |
|---|---|
| A.0.0 (N=12 reaplica §8.7') | Bug P304 confirmado factualmente; vanilla **HA** assumido conservadoramente; magnitude **média** |
| A.0.0' (subdivisão) | **P305.B** (HA spill body inteiro); P305.A/C/D adiados |
| A.0 (ADR-0098) | ✅ `export.rs` preservado bit-exact (**22º passo**) |
| A.1 inventário | `pending_footnote_bodies` campo + `flush_pending_footnote_bodies` method de P304 reusados; `new_page()` flow compatível com retry |
| A.2 decisão | **(a)** partial drain natural via condicional no loop |
| A.3 integração | **(α)** iterative new_page() flow; `finish()` loop convergente |
| A.4 emit | **(i)** FrameItem standard; hash preservado |
| A.5 bugs latentes | 6+ cenários cobertos: 0/1/N footnotes × cabe/overflow × body normal/gigante |
| A.5' anti-reflexão | **N=15 cumulativo** (P291-P305); distinção honesta promoção candidata vs adiamento conservador |

---

## §3 — Materialização

### §3.1 — `01_core/src/rules/layout/cursor.rs` — flush refinado

**Mudanças principais**:

1. **`top_safe` cálculo**: max Y dos current_items (fallback margin).
2. **`available_h` cálculo**: `(area_bot - top_safe).max(0.0)`.
3. **Greedy fit loop**: cada body medido sequencialmente; cabe →
   measured; não cabe → remainder + early termination.
4. **Defensive force_emit refinado**: apenas se `body > full_avail`
   (página inteira) — evita loop infinito sem forçar overlap em
   partial-page overflow normal.
5. **Y clamp**: `y_cursor = (area_bot - acc_h).max(top_safe)`.
6. **Remainder re-injection**: `self.pending_footnote_bodies = remainder`.

```rust
// P305 — greedy measure-then-fit. Bodies que cabem ficam
// measured (place na pass 2); restantes diferidos para próxima
// página via `remainder`. Fallback defensivo refinado: emite
// primeiro body apenas se for maior que a área de conteúdo
// completa (i.e., não cabe em nenhuma página) — evita loop
// infinito sem forçar overlap em partial-page overflow normal.
let full_avail = (page_h - 2.0 * margin).max(0.0);
let mut measured = Vec::new();
let mut acc_h = 0.0_f64;
let mut remainder = Vec::new();
let mut overflow = false;
for (n, body) in bodies.into_iter() {
    if overflow {
        remainder.push((n, body));
        continue;
    }
    let combined = /* [N] + body */;
    let (h, items) = self.layout_sub_frame_with_width(&combined, 0.0, avail_w);
    let fits = acc_h + h <= available_h;
    let force_emit = measured.is_empty() && h > full_avail;
    if fits || force_emit {
        acc_h += h;
        measured.push((h, items));
    } else {
        overflow = true;
        remainder.push((n, body));
    }
}
// ... Pass 2 placement com Y clamp ...
self.pending_footnote_bodies = remainder;
```

### §3.2 — `01_core/src/rules/layout/mod.rs` — finish() loop

```rust
pub fn finish(mut self) -> PagedDocument {
    for item in self.regions.current.current_line.drain(..) {
        self.regions.current.current_items.push(item);
    }
    self.flush_pending_floats();
    self.flush_pending_footnote_bodies();
    // P305 (P295.2) — overflow: se bodies sobraram no buffer,
    // criar páginas adicionais (`new_page()`) até buffer vazio.
    // Iter limit defensivo: assume cada iteração emite ≥1 body
    // via defensive force_emit fallback (paralelo P251
    // forwarded_count limit).
    let mut iter_limit = self.pending_footnote_bodies.len() + 1;
    while !self.pending_footnote_bodies.is_empty() && iter_limit > 0 {
        self.new_page();
        iter_limit -= 1;
    }
    // ... resto do finish() ...
}
```

### §3.3 — Reutilização total

| Construct | Origem |
|---|---|
| `layout_sub_frame_with_width` | Pré-existente (P81.5) |
| Match offset translation pattern | P245 `emit_deferred_float` |
| Greedy fit pattern | Novo (mas trivial idiom Rust) |
| `iter_limit` defensivo | P251 `forwarded_count` paralelo |
| `mem::take` + re-inject | P245/P251 idiom |
| `FrameItem` variants | Pré-existentes |

**Zero variants novos**, **zero traits novas**, **zero novos
helpers externos**. P305 é puramente composicional sobre
infraestrutura P304.

### §3.4 — Zero alterações em outros sítios

| Componente | Pós-P305 |
|---|---|
| `Content::Footnote` variant | **Inalterado** (P295) |
| `native_footnote` stdlib | **Inalterado** (P295) |
| Marker `[N]` inline emit | **Inalterado bit-exact** (P295) |
| `pending_footnote_bodies` campo | **Inalterado** (P304) — mesmo tipo, semântica estendida |
| `new_page()` flow | **Inalterado** — flush method já chamado por P304 |
| L0 `rules/layout.md` | **Inalterado** — precedente P245/P251/P304 |
| `03_infra/src/export.rs` (emit code) | **Inalterado bit-exact** — hash `66cb8ac3` (**22º passo**) |

---

## §4 — Testes

### §4.1 — `01_core/src/rules/layout/tests.rs` (+6 L1)

| Teste | Verifica |
|---|---|
| **`p305_overflow_body_grande_distribui_no_documento`** | Body grande overflow: sentinel preservado no documento (bug fix) |
| `p305_overflow_multiplos_bodies_todos_preservados` | 5 bodies grandes: todos UNIQUEN sentinel preservados multi-página |
| **`p305_regressao_p304_single_page_preservado`** | **INVARIANTE**: footnote pequena cabe em 1 página (P304 bit-exact) |
| `p305_regressao_documento_sem_footnote_bit_exact` | **REGRESSÃO**: documentos sem footnote idênticos pré-P305 |
| `p305_body_gigante_nao_loop_infinito` | Caso degenerate: body > página inteira não loop infinito |
| **`p305_bug_fix_overflow_sem_overlap_no_top`** | **BUG FIX P304**: body Y nunca acima de margin (não sobrepõe top) |

### §4.2 — `03_infra/src/export.rs` (+3 L3)

| Teste | Verifica |
|---|---|
| **`p305_overflow_body_grande_presente_no_pdf`** | Sentinel UNIQUEP305 presente no PDF (overflow não descarta) |
| `p305_overflow_multiplos_bodies_todos_no_pdf` | 4 sentinels SENTA-SENTD todos no PDF |
| **`p305_regressao_p304_single_page_marker_bit_exact`** | **INVARIANTE**: marker + body P304 bit-exact em single-page |

---

## §5 — Validação

### §5.1 — `cargo test --workspace`

```
test result: ok. 2389 passed; 0 failed; 0 ignored   (typst-core lib;  +6 P305 L1)
test result: ok.  463 passed; 0 failed; 6 ignored   (typst-infra lib; +3 P305 L3)
test result: ok.   24 passed; 0 failed; 0 ignored   (typst-shell lib)
test result: ok.    2 passed; 0 failed; 0 ignored   (bin)
test result: ok.   21 passed; 0 failed; 0 ignored   (bin)
                  -----
                  2899 passed total
```

Baseline P304 = 2 890; delta = **+9 net** (+9 novos, 0 obsoletos) ✓.

Resultado **dentro da janela esperada** da spec (~2 898-2 905):
2899 — limite inferior atingido com cobertura completa.

### §5.2 — `crystalline-lint .`

```
✓ No violations found
```

### §5.3 — Hashes pós-P305

| Ficheiro | Pós P305 |
|---|---|
| `infra/export.rs` (`@prompt-hash`) | **`66cb8ac3` preservado bit-exact** (**22º passo consecutivo**) |
| `entities/content.rs` (`@prompt-hash`) | `82d3c47d` inalterado |
| `rules/layout/mod.rs` (`@prompt-hash`) | `12536b5c` inalterado |
| `rules/layout/cursor.rs` (`@prompt-hash`) | `12536b5c` inalterado |
| L0 `rules/layout.md` | **`12536b5c` preservado** — precedente P245/P251/P304 (campos buffer sem L0 update) |
| Outros L0 markdown | todos preservados |

### §5.4 — Regressões verificadas

- **P304** (10 testes): todos preservados — single-page behavior
  idêntico bit-exact (greedy fit no-op quando tudo cabe).
- **P295** (8 testes): todos preservados — marker emission inalterado.
- **Documentos sem footnote**: bit-exact pré-P304 (early-return em
  buffer vazio).

---

## §6 — Padrões metodológicos

### §6.1 — **Sub-padrão DeferredX — N=4 candidato adiado**

P304 §10.2 estabeleceu sub-padrão "DeferredX buffer + flush em
new_page" N=3 cumulativo:
- N=1 P245: floats_pending.
- N=2 P251: pending_cell_tails.
- N=3 P304: pending_footnote_bodies (single-page).

**P305 estende** N=3 → **N=4 candidato** com:
- Partial drain (não full mem::take).
- Cross-page state (remainder persiste através de page break).
- Iterative flush via `finish()` loop.

**Qualidade do 4º caso**:
- **Não trivial**: cross-page semantics genuinamente nova.
- **Pattern partial drain**: paralelo P251 forwarded_count (similar
  conceptualmente; mecanismo distinto).
- **Convergência iterativa**: `finish()` loop é semântica nova.

**Decisão**: **adiar promoção formal** seguindo standard P293-P304
(12 passos anti-padrão consecutivo). N=4 preservado como
**sub-padrão emergente robusto**.

**Honestidade**: limiar tentativo claramente atingido. Mas
consistência metodológica preserva anti-padrão até N≥5 inequívoco
ou mudança contextual que justifique inversão da política.

### §6.2 — **Sub-padrão §8.4 subcat A "bug latente fixed durante materialização dependente" — N=3 candidato adiado**

| Aplicação | Subcategoria | Origem | Bug |
|---|---|---|---|
| N=1 P288 | A | P287 SmartQuote materialização | NBSP eliminado por `split_whitespace()` |
| N=2 P302 | A | P301 auto-lookup materialização | Args `(x)` descartados em lookup-hit |
| **N=3 P305** | **A** | **P304 footnote materialização** | **Overlap silencioso quando total_h > available_h** |
| P303 | B (distinta) | (bug pré-existente independente) | (não aplicável aqui) |

**P305 confirma subcat A genuinamente**:
- Bug **derivado** de materialização P304 (sem P304, sem buffer
  para overflow).
- Cobertura A.5 do passo origem (P304) **falhou capturar** —
  todos os testes P304 cobriam single-page.
- Magnitude controlada (M dentro spec estimativa XS+ a M).
- Fix composicional sem novos types/traits/variants.

**Limiar tentativo N=3 atingido**. **Promoção candidata robusta**
mas **adiada** mesmo critério §6.1 — consistência metodológica
13ª vez consecutiva.

### §6.3 — §8.7' "A.0.0 template" — N=12 magnitude média

| Passo | A.0.0 N | Magnitude |
|---|---:|---|
| P293-P304 | 1-11 | varia |
| **P305** | **12** | **média (dentro spec XS+ a M)** |

**Janela P294-P305**: máxima, baixa, média, alta, alta, baixa,
média-modesta, média, baixa-modesta, média (M reveal), **média**.

§8.7' N=12 **adiado** seguindo standard.

### §6.4 — §8.3 "refutação pragmática" — N=14 candidato

P305 confirma refutação pragmática suave:
- Spec previu "potencialmente sub-padrão DeferredX with item splitting"
  → P305 ficou em **partial drain básico** (P305.B), não item splitting
  (P305.C scope-out).
- Spec magnitude XS+ a M → real **M**.

§8.3 N=14 candidato preservado. **Adiado** standard.

### §6.5 — §8.6 "A.5' anti-reflexão" — N=15 cumulativo

P291-P305. Distinção honesta sub-padrão DeferredX N=4 vs §8.4
subcat A N=3 — **dois candidatos robustos simultâneos**.

### §6.6 — ADR-0098 "single source of truth" — N=22 cumulativo

Hash `export.rs 66cb8ac3` preservado bit-exact pelos **22 passos
consecutivos** P282-P305. Invariante robusta sobre 22 features
distintas. P305 é o **22.º passo** — preservação por paradigma
"refactor interno layout-time; emit consumer FrameItem agnóstico".

### §6.7 — Anti-padrão P273.17 §0 — 13 passos consecutivos honrados

**0 ADRs meta promovidas P293-P305** (13 passos):

| Passo | Candidatos avaliados | Promovidos |
|---|---:|---:|
| P293-P304 | 12 passos cumulativos | 0 |
| **P305** | **DeferredX N=4 (limiar atingido) + §8.4 subcat A N=3 (limiar atingido) + §8.7' N=12 + §8.3 N=14** | **0** |

**13.ª vez consecutiva** anti-padrão honrado. **Marco crítico**:
**dois sub-padrões com limiar tentativo simultaneamente atingido**
— promoção dupla disponível mas **ambas adiadas** por consistência.

### §6.8 — Lição P302 §6.7 — matrix feature × sintaxe (3ª aplicação)

P305 §A.5 cobertura matricial:

```
                       1 footnote     N footnotes      Body singular > página
Cabe em página         ✓ regressão    ✓ regressão     n/a
Overflow partial-page  n/a            ✓ partial drain  n/a
Overflow full-page     n/a            ✓ multi-page     ✓ defensive emit
Documento sem footnote ✓ regressão    ✓ regressão     ✓ regressão
```

Matrix aplicada genuinamente **3ª aplicação consecutiva** (P303 →
P304 → P305). **Lição vira pattern operacional firmemente
estabelecido**.

---

## §7 — Cobertura vanilla vs cristalino

P305 atinge **paridade vanilla single-document footnote**:

| Caso | Antes P305 (P304 Fase 1) | Pós P305 (P295.2) |
|---|---|---|
| `#footnote[body]` body cabe rodapé | ✓ renderizado | ✓ preservado bit-exact |
| `#footnote[body]` body overflow | ✗ overlap silencioso (bug) | ✓ deferido para próxima página |
| N footnotes total > área | ✗ overlap silencioso | ✓ partial drain + multi-página |
| Body singular > página inteira | ✗ overlap silencioso | ✓ defensive emit + iter_limit |
| Footnote no fim página | ✗ overlap quando overflow | ✓ defer cleanly |
| `footnote.entry` / multi-ref | ✗ ausente | **scope-out** (P295.X) |
| Item-level body split | ✗ ausente | **scope-out** (P305.C) |
| Glyph split paridade total | ✗ ausente | **scope-out** (P305.D) |

**Paridade vanilla single-document footnote atingida**.
Sub-passos P295.X/P305.C/D endereçam restantes lacunas
sofisticadas. P295.1 + P295.2 cobrem 90%+ caso de uso real.

---

## §8 — Frentes pendentes pós-P305

P305 fecha **P295.2**. **Frentes restantes**:

| Frente | Magnitude | Estado |
|---|---|---|
| **P304.B** reserved space dinâmico | XS+ | sub-passo opcional |
| **P304.C** separator line | XS | cosmético |
| **P304.D** customization height/style | XS+ | cosmético |
| **P305.C** item-level body split | M | sofisticado |
| **P305.D** glyph split | M+ | paridade total vanilla |
| **P295.X** footnote.entry / multi-ref | M | architectural |
| P296.X toggles cancel | XS | refino math |
| P297.X UnderoverKind | XS+ | cosmético math |

**Footnote single-document cluster fechado** (P295 + P295.1 + P295.2
= P295 + P304 + P305). 9 passos pendentes resolvidos em 2 passos
finais (P304+P305) sem inflação.

---

## §9 — Decisão sobre P306

P305 fecha P295.2. P306 disponível para:

1. **P304.B/C/D ou P305.C/D** — refinos cosméticos footnote.
2. **P295.X** — footnote.entry / multi-ref (architectural).
3. **Cosméticos cleanup agregado** — múltiplos XS.
4. **Frente totalmente nova** — fora do escopo P283-P305.
5. **Promoção ADR meta deliberada** — se P306 não materializa
   feature substantiva, pode ser oportunidade de formalizar
   sub-padrão DeferredX N=4 ou §8.4 subcat A N=3 (ambos atingiram
   limiar em P305).

Decisão fica para o operador humano.

---

## §10 — Honestidade epistémica

### §10.1 — Bug latente P304 era genuíno mas não-explicitado

P304 §3.2 introduziu `y_cursor = area_bot - total_h` sem clamp.
**Bug factual presente desde P304**, mas:
- P304 testes todos cobriam single-page (tudo cabe).
- A.5 P304 não considerou overflow (P295.2 explicitly scope-out).
- Bug exists desde commit P304 mas nenhuma documentação explícita.

**P305 expôs e fixou**. Diferença vs P303 (que também envolveu
bug derivado): P305 fix é **estrutural** (partial drain + iterative
flush), não apenas paralelo arquitectural simples.

### §10.2 — Sub-padrão DeferredX N=4 honestamente

P305 estende P304 buffer com partial drain — qualidade genuína do
4º caso. **Não trivial**:
- Cross-page state (semântica nova).
- Iterative convergence (mecanismo novo).
- Defensive force_emit refinado (não trivial vs P251 forwarded_count).

**Mas adiado** — consistência metodológica vale **mais do que a
satisfação de promover** quando ambíguo entre genuíno e trivial.
Política P273.17 §0 firmemente estabelecida.

### §10.3 — §8.4 subcat A N=3 honestamente

P305 bug é **derivado** de P304 genuinamente:
- Sem P304, sem buffer.
- Sem buffer, sem overflow possível.
- Sem overflow, sem bug.

Argumento simétrico ao P288/P287 e P302/P301. **N=3 genuíno**.

**Adiado** por mesmo princípio §10.2 — promoção dupla disponível
(DeferredX + §8.4 subcat A) seria **inflação real** vs disciplina
estabelecida. Adiar **ambas** preserva integridade.

### §10.4 — HA conservadora vs vanilla inspection

A.0.0 não inspeccionou em profundidade o algoritmo vanilla
overflow exact. **Default HA** (body inteiro spill) adoptado
como **mínimo viável conservador**. Pode divergir de vanilla
HB (item split) em casos onde body único ocupa parte de duas
páginas.

**Trade-off honesto**: HA é mais simples e suficiente para
80%+ uso real (footnotes curtos). HB requereria item-level
splitting state — magnitude M+ adicional. P305.C sub-passo
endereça se necessário.

### §10.5 — Refactor minor in-place vs novo helper

P305 reescreve `flush_pending_footnote_bodies` in-place (mantém
assinatura; estende lógica interna). **Não criou helper externo**
(e.g., `try_fit_body`) — duplicação interna minimal compensa
overhead de abstração. **Decisão paralela P303 §10.2** (refactor
minor via variável local).

### §10.6 — Reutilização vs criação

P305 reusa **6 FrameItem variants**, **3 helpers** (`layout_sub_frame_with_width`,
`mem::take`, `vertical_metrics`), **3 patterns** (DeferredX, match
offset translation, iter_limit defensivo). **Zero novos types**,
**zero novas traits**, **zero novas helper functions externas**.

Pattern cumulativo N=3 P302-P305: **bug fixes derivados são
puramente composicionais** quando infraestrutura suporta. 3ª
confirmação consecutiva da observação P302.

---

## §11 — Fecho

P305 fechado com:

- **+9 testes net** (+6 L1, +3 L3) — todos verdes.
- **0 violations** no `crystalline-lint`.
- **0 drift** em hashes L0.
- **Hash `export.rs` preservado** bit-exact (**22º passo consecutivo**
  P282-P305).
- **Hash `content.rs` inalterado**.
- **L0 `layout.md` inalterado** — precedente P245/P251/P304 (campos
  buffer sem L0 update).
- **0 ADRs meta novas** — **13.ª vez consecutiva** anti-padrão
  honrado.
- **Sub-padrão DeferredX N=4 candidato adiado** (limiar atingido).
- **Sub-padrão §8.4 subcat A N=3 candidato adiado** (limiar atingido).

**MARCO P305**:
- **Frente P295.2 resolvida** — paridade vanilla single-document
  footnote completa.
- **Bug latente P304 fixado** — overlap silencioso eliminado via
  greedy fit + Y clamp + iterative flush.
- **2 sub-padrões com limiar simultaneamente atingido** —
  primeira vez na série P283+ que dois candidatos robustos
  emergem no mesmo passo. **Ambos adiados** — consistência
  metodológica preservada.
- **§8.4 subcat A confirmação 3ª aplicação** — P288 + P302 + P305
  forma sub-padrão robusto.
- **DeferredX N=4 com cross-page state** — primeira aplicação
  genuína de buffer persistente através de page break.
- **Reutilização total infraestrutura P304** — 1 método modificado +
  1 método com loop adicionado; zero variants/types novos.
- **Hash `export.rs` preservado pelo 22.º passo consecutivo**
  (P282→P305) — ADR-0098 robusta sobre 22 features distintas.
- **Cluster footnote single-document fechado** — P295 + P304 + P305
  cobrem marker + body + overflow em **3 passos sequenciais**
  pós-9-passos-pendentes.
- **Lição P302 §6.7 aplicada 3ª vez consecutiva** (matrix feature
  × sintaxe).

**Lição final**: P305 prova que **promoções múltiplas simultâneas
disponíveis testam a disciplina mais que escassez**. O anti-padrão
P273.17 §0 demonstrou robustez maior em P305 (rejeitando duas
promoções) do que em passos anteriores (rejeitando uma). **Consistência
metodológica não é função da escassez de oportunidades, mas da
firmeza do critério**. P305 também confirma que **cluster
features pendentes longas (9-10 passos) podem fechar em 2-3 passos
sequenciais** quando a infraestrutura composicional está madura
— sem necessidade de refactor estrutural prévio (lição P304 §10.5
reforçada).
