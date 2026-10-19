# Passo 104 — Relatório de encerramento (`Sink` materializado em L1)

**Data**: 2026-04-23
**Precondição**: Passo 103 encerrado; `#show` validado; 795 L1 +
174 L3 + 6 ignorados; zero violations.
**ADR criada**: ADR-0042 "Sink materializado em L1" — **PROMOVIDA A
EM VIGOR** em 104.E.

---

## Sumário

`Sink` materializado em L1 (`entities/sink.rs`) com API real
(`warn`/`is_empty`/`into_diagnostics`) e dedup por `(span, message)`
via `FxHashSet`. Stub `Sink(())` removido de `world_types.rs`; path
histórico preservado via `pub use`.

**Sem migração de consumidor** neste passo — gate do spec triggered:
o único site silenciado (DEBT-49, `eval_set_rule`) requer propagação
de 3 níveis, excedendo o limite "1 função" do passo.

Zero regressão funcional: **795 → 803 L1** (+8 unitários de `Sink`),
174 L3 + 6 ignorados inalterados. `crystalline-lint .` → zero
violations.

---

## 104.A — Inventário

Inventário em `00_nucleo/diagnosticos/inventario-sink-passo-104.md`.

### Sink pré-Passo 104

```rust
// world_types.rs:385-401 (pré-104)
#[derive(Hash)]
pub struct Sink(());

impl Sink { pub fn new() -> Self { Self(()) } }
impl Default for Sink { ... }

#[comemo::track]
impl Sink {}
```

**Stub puro**. Nenhum método. `#[comemo::track]` vazio. `_sink:
TrackedMut<Sink>` passado a `eval()` mas nunca usado.

### Candidatos a piloto

| Site | Profundidade desde `eval()` |
|------|---------------------------:|
| `rules/eval/rules.rs:294` (`#set text(font:...)` silenciado) | **3 níveis** (`eval → eval_markup → eval_expr → eval_set_rule`) |

Apenas 1 site com silenciamento explícito. **Excede o gate
"≤ 1 função"**. Decisão: **não migrar consumer**. DEBT-49 permanece
pendente mas agora tem destino real.

### Compatibilidade `#[comemo::track]`

Bloco `#[comemo::track] impl Sink {}` permanece vazio. Métodos
tracked exigem argumentos `Send + Sync + Hash` — `SourceDiagnostic`
não deriva `Hash` (tem `Vec<String>`, `Vec<Spanned<Tracepoint>>`).
Integração comemo adiada; os métodos reais vivem num `impl Sink`
não-tracked.

### Saída de warnings para L3

`grep into_diagnostics 01_core/ 03_infra/` → zero matches. **Lacuna
identificada**: warnings acumulam mas nunca chegam ao caller.
DEBT-51 aberto.

---

## 104.B — ADR-0042

Criada em `00_nucleo/adr/typst-adr-0042-sink-materializado.md`.
**Promovida a EM VIGOR em 104.E**.

Conteúdo:

- API formalizada: `new/default`, `warn`, `is_empty`,
  `into_diagnostics`.
- Dedup por `(Span, String)` — severity/hints/trace fora da chave.
- `FxHashSet` para O(1) dedup (ADR-0018 autoriza).
- `#[comemo::track] impl Sink {}` permanece vazio; adiamento da
  integração comemo justificado.
- Decisão de **não migrar consumer** documentada com razão
  (gate triggered).
- Relação com ADR-0017 (stub inicial, agora resolvido).

---

## 104.C — Implementação

### Ficheiro novo `01_core/src/entities/sink.rs`

```rust
pub struct Sink {
    diagnostics: Vec<SourceDiagnostic>,
    seen: FxHashSet<(Span, String)>,
}

impl Sink {
    pub fn new() -> Self { Self::default() }
    pub fn warn(&mut self, diag: SourceDiagnostic) {
        let key = (diag.span, diag.message.clone());
        if self.seen.insert(key) {
            self.diagnostics.push(diag);
        }
    }
    pub fn is_empty(&self) -> bool { self.diagnostics.is_empty() }
    pub fn into_diagnostics(self) -> Vec<SourceDiagnostic> {
        self.diagnostics
    }
}
impl Default for Sink { ... }
```

### Stub removido em `world_types.rs`

```rust
// Antes:
#[derive(Hash)]
pub struct Sink(());
impl Sink { pub fn new() -> Self { Self(()) } }
impl Default for Sink { ... }

// Depois:
pub use crate::entities::sink::Sink;

// (bloco #[comemo::track] impl Sink {} permanece vazio)
```

### Prompt L0 novo

`00_nucleo/prompts/entities/sink.md` criado com hash `a31e1f86`.
Header `@prompt-hash` alinhado.

### Zero migração de consumer

Conforme inventário + ADR.

---

## 104.D — Testes

8 testes unitários em `entities/sink.rs`:

1. `sink_novo_esta_vazio` — `Sink::new().is_empty() == true`.
2. `warn_adiciona_um_diagnostico` — warn uma vez, `into_diagnostics`
   devolve 1.
3. `warn_duplicado_nao_acumula_segundo` — mesmo `(span, message)`
   duas vezes → 1 item.
4. `warn_mesma_span_message_diferente_nao_deduplica` — `(sp, "A")`
   + `(sp, "B")` → 2 itens.
5. `warn_preserva_ordem_de_insercao` — ordem de inserção mantida.
6. `warn_hint_diferente_mesmo_par_conta_como_duplicado` — hints
   fora da chave; primeiro ganha.
7. `warn_severity_diferente_mesmo_par_conta_como_duplicado` —
   severity fora da chave; primeiro ganha.
8. `into_diagnostics_consome_sink` — após consume, `Sink` não
   reutilizável.

Todos passam.

---

## 104.E — Encerramento

### Verificação

```
$ cargo test --workspace | grep "test result"
test result: ok. 803 passed; 0 failed; 0 ignored ...
test result: ok. 174 passed; 0 failed; 6 ignored ...

$ crystalline-lint .
✓ No violations found

$ grep "pub struct Sink(())" 01_core/src/
(zero matches — stub removido)

$ grep "pub struct Sink" 01_core/src/entities/sink.rs
pub struct Sink {    <- real
    diagnostics: Vec<SourceDiagnostic>,
    seen: FxHashSet<(Span, String)>,
}
```

### DEBTs

- **DEBT-49** actualizado: primeiro critério (Sink materializado)
  marcado ✓. Resto pendente — requer propagação sink pelas `eval_*`.
- **DEBT-51 aberto** — "Warnings do Sink não chegam ao caller
  L3/CLI". Critério de conclusão detalhado; bloqueador do valor
  prático do DEBT-49.

### ADR

- **ADR-0042** `EM VIGOR`.

---

## Números finais

| Métrica | Antes | Depois |
|---------|------:|-------:|
| L1 tests | 795 | **803** (+8) |
| L3 tests | 174 | 174 |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| Stubs em L1 | N | **N−1** (`Sink`) |
| ADRs activas | 41 | **42** (+0042) |
| DEBTs em aberto | 13 | **14** (+DEBT-51) |

---

## Lições

1. **Gate do spec respeitado**: o spec do Passo 104 estabeleceu
   claramente que "consumidor piloto" requer propagação ≤ 1 função.
   O inventário 104.A concluiu que o site silenciado (DEBT-49)
   requer 3 níveis. Em vez de violar o gate, o passo tornou-se
   "materialização + tests + ADR" — a fundação está entregue; o
   consumer migra em passo próprio.

2. **Stub → real sem ripple**: trocar `struct Sink(())` por `struct
   Sink { diagnostics, seen }` foi invisível aos callers porque:
   - `_sink: TrackedMut<Sink>` em `eval()` tem `_` prefix (unused).
   - `pub use crate::entities::sink::Sink` em `world_types.rs`
     preservou o path histórico.
   - `#[comemo::track] impl Sink {}` vazio garantiu que
     `TrackedMut<Sink>` continuasse a compilar.

3. **Dedup por chave minimalista**: `(Span, String)` é suficiente.
   Severity/hints/trace fora da chave porque o caso
   "mesmo warning em hot loop" é o que dedup precisa de cobrir, e
   esse caso tem sempre os mesmos campos. Dois testes explícitos
   (6, 7) documentam o comportamento.

4. **Lacuna L1→L3 documentada**: DEBT-51 aberto para a inoperância
   actual (Sink acumula mas ninguém lê). Sem o DEBT, o valor
   prático do Sink é zero. Com o DEBT, fica claro o passo seguinte:
   decidir o canal de saída e migrar consumers.

5. **Gate protege o passo de inchar**: sem o gate, a tentação seria
   "já que estou a materializar, migre também os consumers".
   Resultado: passo triplicaria de tamanho, propagação de sink
   misturada com materialização do tipo. O gate separou fundação
   (aqui) de migração (futuro DEBT-49/51).

---

## Estado pós-Passo 104

### Sink real, pronto para consumers

```
entities/sink.rs:
  struct Sink {
      diagnostics: Vec<SourceDiagnostic>,
      seen: FxHashSet<(Span, String)>,
  }
  
  impl Sink {
      pub fn new() / default();
      pub fn warn(&mut self, diag);
      pub fn is_empty(&self) -> bool;
      pub fn into_diagnostics(self) -> Vec<SourceDiagnostic>;
  }

world_types.rs:
  pub use crate::entities::sink::Sink;    // re-export
  #[comemo::track] impl Sink {}           // preparação futura

eval/mod.rs:
  pub fn eval(..., _sink: TrackedMut<Sink>, ...) -> SourceResult<Module>
  // _sink ainda não usado — propagação = DEBT-49
```

### DEBTs actualizados

| DEBT | Estado após Passo 104 |
|------|----------------------|
| DEBT-49 | Ponto 1 do critério ✓ (Sink materializado). Restantes pendentes (propagação + migração de consumers). |
| DEBT-51 (novo) | Lacuna L1→L3 documentada. Bloqueia valor prático de DEBT-49. |

### Trabalho futuro identificado

1. **DEBT-51**: decidir canal de saída (tuple ou TrackedMut lido pelo
   caller) e implementar. Sem isto, DEBT-49 não traz valor visível.
2. **DEBT-49**: após DEBT-51, migrar consumers silenciados (começando
   pelo site `eval_set_rule:294`). 5ª aplicação da ADR-0036.
3. **Integração comemo**: quando necessário. Exige `SourceDiagnostic`
   `Hash` ou wrapper tracked. Passo dedicado.
4. **Propriedades adicionais** (`text.font`, `lang`, `par.leading`):
   bloqueadas por tipos não materializados. Dependência sobre
   materialização de Font/Lang/Par.
5. **`Engine<'a>`**: agregador dos 9 parâmetros `eval_*`. Sink pode
   naturalmente viver nesse agregador — candidato para a 6ª
   aplicação da ADR-0036.
