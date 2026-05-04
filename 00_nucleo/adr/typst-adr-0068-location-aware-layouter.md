# ⚖️ ADR-0068: Layouter location-aware via Locator dedicado determinístico

**Status**: `ACEITE`
**Validado**: 2026-05-03 — P185D 4 tests E2E confirmaram
sincronização-por-construção; P185E ratificou em relatório
consolidado.
**Data**: 2026-05-03 (PROPOSTO) → 2026-05-03 (ACEITE)
**Diagnóstico prévio**:
- `00_nucleo/diagnosticos/diagnostico-location-aware-layouter-passo-185a.md` (P185A).
- `00_nucleo/materialization/typst-passo-182e-relatorio.md` §5.2 (descoberta original).
- `00_nucleo/materialization/typst-passo-184-relatorio-consolidado.md` §4.2 (ratificação inversão).
**Materialização**:
- `00_nucleo/materialization/typst-passo-185b-relatorio.md` (trait methods).
- `00_nucleo/materialization/typst-passo-185c-relatorio.md` (Layouter integration).
- `00_nucleo/materialization/typst-passo-185d-relatorio.md` (tests E2E sincronização).
- `00_nucleo/materialization/typst-passo-185-relatorio-consolidado.md` (encerramento P185E).

---

## Contexto

Após P184F, o estado M5/M4 é 6/12 read-sites migrados de
`CounterStateLegacy` para `Introspector`. Os 6 restantes
incluem:

- **C1 heading prefix** (`mod.rs:310`,
  `format_hierarchical("heading")`) — bloqueado em P183B.
- **C2 equation counter** (`equation.rs:97`,
  `get_flat("equation")`) — bloqueado em P183C.
- 4 sites fora de escopo M4 (TOC, fixpoint side-channels,
  resolved labels pendente P183E).

C1 e C2 partilham causa raíz documentada em P183B/C: **eixo 1
da regra dos 2 eixos falha** — Layouter precisa de valor durante
walk de layout (snapshot-during-walk), enquanto Introspector
oferece snapshot final pós-walk via `formatted_counter` (P170)
ou `is_numbering_active` (P182B).

P177 já adicionou ao trait `formatted_counter_at(key, location)`
que resolve a leitura por Location. **Falta o caminho oposto**:
o Layouter precisa de saber qual `Location` consultar no ponto
da leitura.

Esta ADR fixa o mecanismo de propagação de `Location` ao
Layouter — pré-condição cross-cutting para desbloquear C1+C2.

---

## Decisão

**Mecanismo M3 — Locator dedicado do Layouter com cursor
interno**, sincronizado por construção via determinismo do
`Locator` + invariante `is_locatable`.

Forma esperada (a materializar em P185C):

```rust
pub struct Layouter<M: FontMetrics, S: ImageSizer> {
    // ... fields existentes ...
    pub introspector:      TagIntrospector,

    /// Locator dedicado do Layouter, sincronizado por
    /// construção com o Locator do walk de introspect:
    /// produz a mesma sequência de Locations para o mesmo
    /// Content (determinismo demonstrado em
    /// `entities/locator.rs::tests`).
    locator:               Locator,

    /// Location actual quando o Layouter está dentro de um
    /// nó locatable. `None` antes do primeiro nó locatable
    /// ou em transições entre nós irmãos.
    current_location:      Option<Location>,
}
```

`layout_content` adicionada gating no início:

```rust
pub fn layout_content(&mut self, content: &Content) {
    let prev_loc = self.current_location;
    if is_locatable(content) {
        self.current_location = Some(self.locator.next());
    }
    match content {
        // ... arms ...
    }
    self.current_location = prev_loc;
}
```

Consumers C1+C2 (em P187/P188) consultam
`self.current_location` no ponto da leitura.

---

## Justificação

### Determinismo do `Locator` torna sincronização trivial

`entities/locator.rs::tests::duas_instancias_paralelas_produzem_sequencias_iguais`
prova empiricamente que dois `Locator::new()` independentes
produzem `Location`s idênticas para a mesma sequência de
chamadas a `next()`. Esta propriedade está documentada como
garantia explícita (line 67-72) e é design-intent de P161.

Como o predicate `is_locatable` (P164) é função pura
`&Content → bool`, e o walk de introspect (`introspect.rs:329-330`)
chama `locator.next()` exatamente quando
`do_extract_payload(content).is_some()` — ou seja, quando
`is_locatable(content) == true` (invariante explícito em
`locatable.rs:11`) — o Layouter pode chamar `self.locator.next()`
no mesmo gating point e obter a mesma `Location`.

A sincronização é **estrutural por construção**, não
operacional. Sem partilha de Locator, sem comunicação cross-walk.

### Coerência com P163 walk puro

P163 invariante walk puro cobre o **walk de introspect** —
introspect.rs deve não ter side-effects para além de
populate `state` legacy + `tags`. Layouter já é mutável por
design (cursor_x, cursor_y, current_line, etc.); adicionar
`locator: Locator` + `current_location: Option<Location>` é
extensão natural, não violação.

Q3 (compat walk puro) confirmada empiricamente: o mecanismo
M3 não toca o walk de introspect.

### Coerência com ADR-0036 (atomização) + ADR-0067 (attribute-grammar)

**ADR-0036 atomização**: a alternativa M2 (parâmetro
propagado) cascata-ia mudanças de assinatura em ~30 métodos
de layout. M3 isola a mudança ao Layouter — single struct,
single gating point. Atomização preservada.

**ADR-0067 attribute-grammar**: PROPOSTO sugere
attribute-grammar para scoping. Location é attribute herdado
top-down (parent → children) via `current_location` field;
`prev_loc` save/restore implementa scoping léxico por
construção. M3 alinha com ADR-0067 sem replicar trabalho —
quando ADR-0067 materializar, `current_location` pode
generalizar para attributes adicionais.

### Custo de implementação

P185C estima ~30 LOC: dois fields novos no Layouter +
gating de 4 linhas em `layout_content` + Locator init em
`Layouter::new`. Magnitude S — alinha com sub-passo singular.

Alternativa M2 (parâmetro propagado) estimou-se em ~150 LOC
de cascata + risco de partial typing errors em métodos
internos. Magnitude M-L. Reversibilidade pior (cascata
inversa).

### Precedente vanilla

Vanilla typst usa M2 (Locator owned passed through layout
function signatures, com `Locator::split` para sub-walks
e `LocatorLink::measure` para measurement mode). É mais
sofisticado mas exige o `Locator` ser ele próprio um
recurso "consumable" (não-Clone, non-Copy intencional).

Cristalino `Locator` é mais simples (u64 counter,
não-Clone mas determinístico). M3 capitaliza esta
simplicidade — vanilla precisaria de M2 porque o seu Locator
tem state link próprio que torna determinismo impossível
sem o passing.

Não bloqueia futura adopção de mecanismo vanilla-style se
M3 for revelar limitações em features de measurement
(ex.: relayout de footnotes).

---

## Alternativas consideradas

### M1 — Walk sincronizado com Locator partilhado

Walk de introspect e walk de layout usam **o mesmo** `Locator`
em sequência ou paralelo.

**Rejeitada** porque:
- Adiciona acoplamento entre componentes (introspect e layout
  precisam de coordenar acesso ao Locator).
- Perde a independência testável dos dois walks.
- Não traz benefício: M3 já garante sincronização por
  construção via determinismo.

### M2 — Parâmetro propagado

`Location` actual passa via parâmetro nos métodos de layout:
`fn layout_content(&self, content, location)`,
`fn layout_heading(&self, heading, location)`, etc.

**Rejeitada** porque:
- Cascata de mudanças de assinatura em ~30 métodos.
- Custo M-L vs S de M3.
- Cada caller (incluindo callers externos como tests E2E)
  precisa de gerir Location explicitamente.
- Vanilla usa esta forma mas é forçado pela sua estrutura
  de Locator com link próprio; cristalino não tem essa
  obrigação estrutural.

### Pré-compute Locations no walk de introspect

Walk de introspect produz `Vec<Location>` em ordem;
Layouter consome a sequência via cursor.

**Rejeitada** porque:
- Adiciona campo extra ao output do introspect (já produz
  `(state, introspector)`).
- Conceptualmente equivalente a M3 com fonte externa em
  vez de Locator próprio determinístico.
- Acopla introspect a layout — qualquer mudança no walk de
  introspect (ex.: skip de nós irrelevantes) requer
  sincronização explícita.

---

## Implicações

### Implementação (P185B–E)

P185B: trait methods location-aware em falta:
- `flat_counter_at(key, location) -> Option<usize>` — para
  C2 equation counter (que retorna `usize` não string).
- `is_numbering_active_at(key, location) -> bool` — para
  correctness em re-update (P182E §5.2 identificou).
  Replicar padrão `formatted_counter_at` (P177).

P185C: integração Layouter:
- Field `locator: Locator` + `current_location: Option<Location>`.
- Gating em `layout_content` com `is_locatable`.
- `Layouter::new` inicializa `Locator::new()`.

P185D: tests E2E confirmam mecanismo funciona via consultas
sintéticas em pipeline real (sem migrar C1/C2 ainda).

P185E: relatório consolidado.

C1+C2 migração: P187+P188 (passos dedicados após P185).

### Consumers afectados (futuros)

- C1 heading prefix (P187): substituirá
  `self.counter.format_hierarchical("heading")` por
  `self.introspector.formatted_counter_at("heading",
  self.current_location.unwrap())`.
- C2 equation counter (P188): pré-requisito P186 promove
  `Content::Equation` a locatable (`is_locatable`,
  `extract_payload`, arm em `from_tags`); depois consumer
  consulta `flat_counter_at("equation", current_location)`.

### Janela compat M6

Após C1+C2 migrados, o fallback `||` legacy
(`numbering_active`, `format_hierarchical`, `get_flat`)
deixa de ter rede de segurança útil. M6 elimina
`CounterStateLegacy.numbering_active`,
`CounterStateLegacy.hierarchical/flat` e o fallback
respectivo nos consumers.

### Fora de escopo desta ADR

- Mecanismo de measurement-mode (relayout em footnotes etc.)
  — adoptar vanilla-style se necessário em milestone
  posterior.
- Generalização para outros attributes herdados (cor, dir,
  lang propagados top-down) — escopo de ADR-0067 quando
  materializar.

---

## Critério de validação

ADR transita para `ACEITE` quando:

1. P185C materializa Layouter location-aware com forma
   acima.
2. P185D tests E2E confirmam que `current_location` reflecte
   correctamente a Location em cada nó locatable visitado
   (com paridade contra `introspect.rs::walk`).
3. P185E relatório consolidado regista que magnitude S
   estimada está dentro de ±50% da realidade.

ADR transita para `REJEITADA` se:
- P185C revelar que sincronização por construção falha em
  algum caso bordo (ex.: layout salta nós que walk visitou).
- Custo real for substancialmente superior a S
  (multiplicador >2×) — sinal de que M2 ou M1 seriam
  preferíveis.

---

## Histórico

| Data | Estado | Motivo |
|------|--------|--------|
| 2026-05-03 | `PROPOSTO` | P185A diagnóstico fixou mecanismo M3 com 6 cláusulas decididas. Validação pendente em P185C+. |
| 2026-05-03 | `ACEITE` | P185D §"Resumo": 4 tests E2E (`mod p185d_locator_sync` em `01_core/src/rules/layout/tests.rs`) passam — sincronização-por-construção empiricamente confirmada. Sequências de `Location` produzidas pelo Layouter coincidem com walk de introspect em todos os cenários (3 locatables; mistura locatable+não-locatable; `current_location` `None` antes do primeiro locatable; pipeline end-to-end com `is_numbering_active_at` consultado via `current_location`). Custo real P185C ≈37 LOC em `mod.rs` vs estimativa ~30 LOC — multiplicador 1.23×, dentro ±50% per critério §3. P185E ratifica em relatório consolidado. |

---

## Notas pós-aceitação

### Divergência menor face à forma esperada (cláusula gate trivial)

ADR sugeriu pseudocódigo com save/restore explícito de
`prev_loc` em `layout_content`:

```rust
let prev_loc = self.current_location;
if is_locatable(content) {
    self.current_location = Some(self.locator.next());
}
match content { /* arms */ }
self.current_location = prev_loc;
```

P185C executou Opção 2 (cláusula 3): **avanço monotónico
sem save/restore**, por simetria literal com walk de
introspect (que avança cumulativamente):

```rust
fn advance_locator_if_locatable(&mut self, content: &Content) {
    if is_locatable(content) {
        self.current_location = Some(self.locator.next());
    }
}

pub fn layout_content(&mut self, content: &Content) {
    self.advance_locator_if_locatable(content);
    match content { /* arms */ }
}
```

Justificação: walk de introspect avança Locator
cumulativamente; sincronização-por-construção exige que
Layouter faça o mesmo. P185D test `.B`
(`sincronizacao_locator_layouter_iguala_walk_introspect`)
confirma que esta forma produz a mesma sequência de
`Location`s que o walk. Se P187/P188 detectarem necessidade
de scoping léxico em algum consumer específico, save/restore
poderá ser reintroduzido localmente nesse consumer (não em
`layout_content`).

### Field type: `Option<Location>` (decisão P185C)

ADR sugeriu `current_location: Option<Location>`. P185C
manteve esta forma — `None` antes do primeiro locatable,
`Some(loc)` após. P185D test `.D`
(`current_location_none_antes_de_primeiro_locatable`)
valida que `Location::from_raw(0)` como sentinel teria
criado ambiguidade indistinguível com a Location real do
primeiro `Locator::next`.
