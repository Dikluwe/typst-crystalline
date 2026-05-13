# Passo 210A — Diagnóstico-primeiro: Counter/State extras Q1=β

**Série**: 210 (sub-passo `A` = diagnóstico-primeiro
reduzido).
**Marco**: M9c (Bloco V — Counter/State extras forma
minimal).
**Tipo**: diagnóstico-primeiro reduzido (zero código
tocado).
**Magnitude**: S-M (~45 min).
**Pré-condição**: P209E concluído; série P209 fechada;
ADR-0077 ACEITE; trait 26 métodos; Selector 6 variants;
stdlib funcs ~52 (incluindo `here()`/`locate()`);
`EvalContext.current_location` field (P208B minimal);
tests 1935 verdes; 0 violations; ADR-0076 PROPOSTO;
blueprint §3.0sexies.
**Output**: 1 ficheiro (relatório curto consolidando
auditoria + decisões + plano P210B+).

---

## §1 Trabalho

Mapear empíricamente o gap entre Counter/State actual
cristalino e o target Q1=β:

- **Q1=β fixado** (humano P207A C10): "manter forma
  minimal cristalino + adicionar `counter.step()` /
  `counter.display(numbering)` / `state.get()` here-aware
  como funcs stdlib separadas". **Não** materializar
  rich `Counter`/`State` types.

P210A produz:
1. Mapeamento empírico Counter/State actual.
2. Decisão sobre escopo concreto (3 funcs ou subset).
3. Plano P210B+ (sub-passos sem ramos).

Reuso de dados toda a trajectória M9c:

- Pattern stdlib `native_X(ctx, args, world, current_file,
  figure_numbering)` consolidado (P208/P209).
- `EvalContext.current_location` minimal (P208B C2).
- `native_here()` + `native_locate()` materializados
  (P208B/C).
- Convenção emergente P208B §3: stdlib funcs P169+
  inline-documentadas; sem L0 separado.
- Pattern "Caminho 1 anti-inflação" 7 aplicações
  consecutivas.

---

## §2 Cláusulas de auditoria (A1–A5)

### A1 — `Counter` cristalino actual

Localizar literalmente:

- Stdlib funcs counter-relacionadas em
  `01_core/src/rules/stdlib/`.
- `CounterRegistry` sub-store (per P207A A4 lista de 9
  sub-stores).
- Pattern actual de invocação de counter em `.typ`
  source (esperado: pre-P169 + extensions).

Output: 4-6 linhas literais.

### A2 — `State` cristalino actual

Localizar literalmente:

- Stdlib funcs state-relacionadas.
- `StateRegistry` sub-store (per P207A A4).
- Pattern actual de invocação de state em `.typ`
  source.

Output: 4-6 linhas literais.

### A3 — Vanilla `counter.step()` + `counter.display()`
+ `state.get()` here-aware

Localizar literalmente em
`lab/typst-original/crates/typst-library/src/introspection/`:

- `counter.rs` — métodos `step`, `display`, `update`,
  `at`, `final`.
- `state.rs` — métodos `update`, `get`, `at`, `final`.
- Assinatura literal: `Counter::step()`,
  `Counter::display(numbering)`, `State::get(...)`.

Output: ~8-12 linhas literais.

### A4 — Cristalino infraestrutura para here-aware
operations

Per P208B:

- `EvalContext.current_location: Option<Location>` field
  exposto.
- `with_current_location` setter.
- Walk advance **não implementado** (P208B Opção i
  minimal).
- `native_here()` invoca `ctx.current_location` → erro
  contextual se None.

Implicação para Q1=β:

- `counter.display(numbering)` precisa de current_location?
  Vanilla: precisa para resolver "this counter at this
  location".
- `state.get()` precisa de current_location? Vanilla:
  precisa.
- Cristalino minimal sem walk advance: estes funcs
  retornam erro se invocados fora de contexto. Aceitar?

### A5 — Consumers reais imediatos

Re-grep em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/` por:
- Tests existentes que usariam `counter.step` /
  `counter.display` / `state.get` here-aware.
- `.typ` source fixtures que mencionem essas
  primitives.

Output: contagem literal (esperado: zero per pattern
consistente M9c P208/P209).

---

## §3 Cláusulas de decisão (C1–C5)

Fixadas **depois** da auditoria.

### C1 — Forma das 3 funcs

Per Q1=β: materializar como funcs stdlib separadas
(não rich types). Para cada uma das 3:

- `counter.step()`: assinatura mínima viável (sem args
  ou 1 arg counter ID).
- `counter.display(numbering)`: pattern numbering string
  ("1", "I", "α") + current_location lookup.
- `state.get()`: lookup em `StateRegistry` + retorno
  `Value`.

C1 fixa **uma assinatura por func** com base em A1+A2+A3.

### C2 — Comportamento sem `current_location`
populated

Per A4 — walk advance minimal:

- **Opção A** — Erro contextual: `"counter.display requer
  contexto locatable; current_location não populado em
  P208B minimal"`. Honest, simétrico a `here()`.
- **Opção B** — Stub vazio: retornar string vazia ou
  `Value::None`. Semântica degenerada.
- **Opção C** — Adiar (não materializar até walk advance
  ser implementado).

Critério: simplicidade + paridade com `here()`. Hipótese
provável: **Opção A** (erro contextual paralelo a
`here()`).

### C3 — Caminho 1 anti-inflação vs materialização
completa

Decisão honesta:

- **Caminho 1 — Adiar série inteira P210**: zero
  consumers reais; pattern anti-inflação 8ª aplicação.
  P210 inteira documental (skip ou minimal anotações).
- **Caminho 2 — Materializar Q1=β literal**: 3 funcs
  stdlib + tests sintéticos (paralelo a `here()`
  mock-tested). Honra `P207A.div-1` aprovado.
- **Caminho 3 — Subset minimal**: apenas 1-2 das 3
  funcs (ex: `counter.step` trivial sem here; pular
  `counter.display`/`state.get` que dependem de
  current_location).

Critério: honesta + valor real adicionado vs custo.

Hipótese provável: **Caminho 2 ou 3**. Q1=β fixou
materialização; mas se `counter.display`/`state.get`
sem current_location funcional retornam só erros, valor
é marginal. Caminho 3 (subset) pode ser honesto.

### C4 — Plano P210B+

Sub-passos sem ramos. Quantidade depende de C3.

Hipótese:
- Se Caminho 1: P210A só + ADR-0076 anotada; sem B+.
- Se Caminho 2: P210B + C + D + E (4 sub-passos: 1 por
  func + encerramento).
- Se Caminho 3: P210B (1-2 funcs subset) + P210C
  encerramento (2 sub-passos).

### C5 — Magnitude agregada P210

Reportar com base em C1-C4.

Range plausível:
- Caminho 1: S documental (~30min).
- Caminho 2: M (~3-4h) — 3 funcs com tests + scope
  register.
- Caminho 3: S-M (~1.5-2h).

---

## §4 Output

1 ficheiro:
`00_nucleo/diagnosticos/typst-passo-210A-relatorio.md`.

Estrutura (~5-7 KB) com 6 §s padrão (paralelo a P208A /
P209A diagnósticos).

---

## §5 Não-objectivos

- Materializar funcs (P210B+ se Caminho 2/3).
- Rich Counter/State types (Q1=β excluiu).
- Walk advance automático (deferred per P208B).
- `query_count_before` (Q4=β deferred).
- Trait method extensions.
- ADR nova ou transição.

---

## §6 Riscos a evitar

1. **Inflar para rich types**: Q1=β fixou. Forma
   minimal apenas.
2. **Aceitar Caminho 2 sem critério**: se A5 confirmar
   zero consumers reais imediatos E A4 mostrar que
   funcs sem current_location são pouco úteis,
   Caminho 1 ou 3 é honesto. Não materializar funcs que
   só retornam erro.
3. **Pre-fixar Caminho 2 por "fidelidade ao `P207A.div-1`
   aprovado"**: o `div-1` aprovou escopo amplo; mas
   cada série pode honestamente reduzir se empírico
   justificar. Pattern P207A.div-1 → P208 + P209 já
   aplicaram redução intra-série.
4. **Inflar diagnóstico**: P210A é reduzido.
5. **Esquecer regra P207B §5**: Counter/State funcs
   stdlib não tocam trait Introspector. Trait mantém
   26 métodos.

---

## §7 Hipótese provável

A1+A2 mostrarão Counter/State minimal pre-existing.
A3 mostrará vanilla com rich types + métodos. A4
confirmará current_location minimal (sem walk advance).
**A5 confirmará zero consumers reais** (per pattern
M9c).

C1 fixará assinaturas minimal.

C2 fixará Opção A (erro contextual paralelo a `here()`).

C3 fixará **Caminho 3** (subset minimal — provavelmente
só `counter.step` que pode ser trivial sem here;
`counter.display`/`state.get` deferred até walk advance).
OU Caminho 1 puro se A5 zero + A4 funcs todas dependem
de current_location.

C4 fixará 1-2 sub-passos consoante C3.

C5 reportará S-M ou S documental.

Mas é hipótese, não decisão. C1-C5 fixam-se empíricamente.

---

## §8 Nota sobre roadmap remanescente

Per relatório P209E §7, P210 e P211 podem ser minimal
ou skip dependendo de empírico. P210A é o diagnóstico
honesto que decide. Mesma estrutura aplica-se a P211
(Outline configurável) — diagnóstico próprio decidirá
Caminho 1 vs 2.

Se P210 fechar como minimal/skip + P211 fechar como
minimal/skip, M9c fecha rapidamente via P212. Se P210
materializar Caminho 2/3 + P211 idem, M9c estende-se.

Esta é honestidade empírica, não regressão do escopo
amplo aprovado.
