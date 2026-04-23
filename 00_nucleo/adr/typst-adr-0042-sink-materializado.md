# ADR-0042 — `Sink` materializado em L1

**Status**: EM VIGOR (Passo 104.E) — validado empiricamente com 803
testes L1 a passar (+8 unitários de `Sink` com dedup), zero
violations.
**Data**: 2026-04-23
**Autor**: Humano + IA
**Passo associado**: 104

---

## Contexto

Desde o Passo 12, `eval()` recebe `_sink: TrackedMut<Sink>` como
parâmetro — onde `Sink` era um stub vazio (`struct Sink(())`). A
intenção: colector de diagnósticos não-fatais (warnings) durante a
avaliação.

Com a activação de `#set` (Passo 102) e `#show` (Passo 103), vários
sítios do eval ganharam comentários de "silenciamento" para
propriedades/construções não suportadas. O mais visível: `#set
text(font: ...)` em `eval_set_rule` — propriedades fora do catálogo
caem num `_ => { }` com comentário `// DEBT-49: propriedades não
suportadas silenciadas`.

Sem um `Sink` real, não há canal estruturado para emitir esses
warnings. `DEBT-49` foi aberto no Passo 102 como sintoma.

Este ADR materializa `Sink` como fundação — 1 passo dedicado —
sem ainda migrar consumidores.

## Decisão

### Forma do `Sink`

Em `01_core/src/entities/sink.rs`:

```rust
use rustc_hash::FxHashSet;  // ADR-0018

pub struct Sink {
    diagnostics: Vec<SourceDiagnostic>,
    seen: FxHashSet<(Span, String)>,
}
```

- `diagnostics`: `Vec<SourceDiagnostic>` — preserva ordem de
  inserção.
- `seen`: `FxHashSet<(Span, String)>` — chave de dedup.

### API (em `impl Sink`, não-tracked)

```rust
impl Sink {
    pub fn new() -> Self;
    pub fn warn(&mut self, diag: SourceDiagnostic);
    pub fn is_empty(&self) -> bool;
    pub fn into_diagnostics(self) -> Vec<SourceDiagnostic>;
}
impl Default for Sink { ... }
```

### Semântica de dedup

Chave: `(Span, String)` — apenas `span` + `message`. **Severity** e
**hints** e **trace** **não** participam na chave.

Justificação:

- **Sem severity**: dois warnings com mesma mensagem e span mas
  severity diferente (erro vs warning) seriam semanticamente
  conflituosos — desduplica-se para o primeiro inserido. Na prática,
  o eval não emite erros via `Sink` (erros fluem por `Result::Err`);
  a colisão é teórica.
- **Sem hints**: hints são auxiliares; o mesmo warning pode ter
  hints ligeiramente diferentes em contextos distintos. Evitar hint
  rot duplicado é benéfico.
- **Sem trace**: idem.

Ordem preservada: `Vec<SourceDiagnostic>` mantém insertion order;
dedup preserva o primeiro a aparecer.

### Razão para dedup

Sem dedup, um warning em hot loop (ex: cada nó de markup com
`#set text(font: ...)` numa página longa) inunda o utilizador. Dedup
torna o `Sink` seguro para integração em qualquer ponto do eval.

Custo: `O(1)` por `warn` (hash). `FxHashSet` é autorizado em L1
desde ADR-0018.

### Decisão sobre `#[comemo::track]`

O `impl Sink {}` com `#[comemo::track]` actual é **vazio** e
**permanecerá vazio** neste passo. Métodos tracked exigem
argumentos `Send + Sync + Hash` — `SourceDiagnostic` não deriva
`Hash` (tem `Vec<String>` e `Vec<Spanned<Tracepoint>>`).

A integração comemo é adiada. Os métodos `warn`/`is_empty`/
`into_diagnostics` vivem num `impl Sink` separado. Acesso via
`&mut Sink` directo, não via `TrackedMut<Sink>`.

Quando a integração comemo for accionada, os métodos serão
migrados caso-a-caso (pode exigir `EcoString` em vez de `String`,
ou um wrapper tracked).

### Consumidor piloto (adiado)

O Passo 104.A inventariou os candidatos e identificou que
**nenhum** tem propagação ≤ 1 nível. A única função com silenciamento
explícito (DEBT-49) é `eval_set_rule`, 3 níveis abaixo do `eval()`
onde `_sink` entra.

A propagação `eval → eval_markup → eval_expr → eval_set_rule` é a
5ª aplicação da ADR-0036 — trabalho do DEBT-49 completo, **não**
deste passo.

**Decisão**: materializar `Sink` + API + tests. Não migrar
consumidor. DEBT-49 fica pendente mas agora tem destino real para
migrar quando for accionado.

### O que esta ADR não decide

- **Quando DEBT-49 migra**: depende de priorização. A arquitectura
  (propagar `sink: &mut Sink` pelas eval_*) está clara desde os
  passos 92–98.
- **Caminho warnings → L3**: lacuna identificada (Sink acumula mas
  nunca é consumido pelo caller). DEBT-51 aberto opcionalmente.
- **Integração comemo**: adiada até haver caso de uso real (e
  `SourceDiagnostic` ser `Hash`-able).

## Alternativas consideradas

1. **Consumidor piloto com 3 níveis de propagação**: propagar
   `sink: &mut Sink` de `eval` até `eval_set_rule`. Rejeitado pelo
   gate do spec ("limitar propagação a 1 função") — transforma o
   passo em DEBT-49 completo.

2. **Consumidor piloto artificial em `eval()` top-level**: emitir
   warning para alguma condição trivial (ex: `_routines` não
   usado). Rejeitado — warning sem contexto semântico; ruído
   sem valor.

3. **Não materializar, adiar Sink para DEBT-49**: rejeitado —
   DEBT-49 fica bloqueado sem um tipo real para onde migrar. Este
   passo é o pré-requisito natural.

4. **Chave de dedup inclui severity**: `(Span, Severity, String)`.
   Rejeitado — severity só tem 2 valores no enum; colisão
   teórica. Simplicidade ganha.

5. **`drain_diagnostics(&mut self)` em vez de `into_diagnostics(self)`**:
   permitiria reutilizar o `Sink`. Rejeitado — `Sink` é usado no
   contexto de um eval single-shot; consumir na extracção alinha
   com ciclo de vida do `eval()`.

## Relação com outros ADRs

- **ADR-0017** (stubs iniciais): `Sink` era um dos stubs listados.
  Este ADR resolve.
- **ADR-0018** (rustc_hash em L1): `FxHashSet` usado para dedup.
- **ADR-0036** (atomização): propagação de `Sink` pelas eval_*
  será 5ª aplicação. DEBT-49 é o passo onde essa aplicação acontece.
- **ADR-0040** (activação de `#set`): `DEBT-49` foi aberto nesse
  passo como sintoma da ausência de `Sink`. Este ADR dá a fundação.

## Consequências

### Positivas

- `Sink` real em L1 — tipo fundacional deixa de ser stub.
- API mínima, testada, com dedup.
- Prepara terreno para DEBT-49 completo.
- Zero propagação neste passo — sem arrasto de assinatura.

### Negativas

- Sink existe mas acumula warnings que ninguém lê. Lacuna
  L1→L3 documentada mas não resolvida.
- DEBT-49 continua aberto (já era).
- `#[comemo::track] impl Sink {}` continua vazio — integração
  futura exigirá cuidado adicional.

### Neutras

- `eval()` mantém `_sink: TrackedMut<Sink>` unused. Assinatura
  inalterada.

---

## Referências

- `00_nucleo/diagnosticos/inventario-sink-passo-104.md`
- `00_nucleo/materialization/typst-passo-104.md`
- ADR-0017 (stubs), ADR-0018 (rustc_hash), ADR-0040 (activação `#set`).
