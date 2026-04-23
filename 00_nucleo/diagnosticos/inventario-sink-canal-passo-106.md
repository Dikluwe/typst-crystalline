# Inventário canal Sink — Passo 106.A

Data: 2026-04-23.

---

## Parte 1 — Callers actuais do `eval`

### Ficheiros e classificação

| Ficheiro:linha | Classificação | Constrói `Sink`? | Lê warnings? |
|----------------|---------------|-----------------:|-------------:|
| `01_core/src/rules/eval/tests.rs:31` (`eval_for_test`) | H (helper L1) | Sim (`Sink::new()`) | Não (descartado) |
| `01_core/src/rules/eval/tests.rs:61` (`eval_for_test_with_limits`) | H (helper L1, test-only) | N/A (não chama eval) | N/A |
| `03_infra/src/integration_tests.rs:67` (`do_eval`) | T (teste L3) | Sim (`Sink::new()`) | Não (descartado) |

**Todos descartam o Sink**. Nenhum caller lê `into_diagnostics`
hoje. Isto é o sintoma do DEBT-51 — warnings acumulam em
silêncio.

### Padrão de construção

Todos usam o mesmo boilerplate:

```rust
let mut sink = Sink::new();
eval(
    &routines,
    world,
    traced.track(),
    sink.track_mut(),      // ← TrackedMut<Sink>
    route.track(),
    source,
)
```

`sink.track_mut()` é gerado pelo `#[comemo::track] impl Sink {}`
— mesmo com bloco vazio, a macro produz o método `track_mut`.

---

## Parte 2 — Candidatos a micro-piloto

### Preferência 1: DEBT-49 site

`rules/eval/rules.rs:294` em `eval_set_rule`. Propriedades de
`#set text(...)` não suportadas.

Propagação necessária: `eval() → eval_markup() → eval_expr() →
eval_set_rule()` = **3 níveis**.

Spec do Passo 104 estabeleceu gate "≤ 1 função". Passo 106 não
reitera o gate mas o raciocínio mantém-se — propagar 3 níveis
seria o DEBT-49 completo, não o piloto.

**Descartado para este passo**.

### Preferência 2: sítio onde `sink` já é recebido

`eval()` recebe `_sink: TrackedMut<Sink>`. É o único.

**Candidato**: emitir warning **dentro de `eval()`** usando a
`TrackedMut<Sink>` directamente. Para isso, `Sink` precisa de ter
um método tracked (o bloco `#[comemo::track] impl Sink {}` actual
é vazio).

### Decisão: adicionar método tracked `warn` a `Sink`

Adicionar ao `#[comemo::track] impl Sink { ... }` em
`world_types.rs` (ou migrar para `sink.rs`):

```rust
#[comemo::track]
impl Sink {
    pub fn warn(&mut self, span: Span, message: &str) {
        // delegar para o método não-tracked
        let diag = SourceDiagnostic::warning(span, message);
        // reuso da lógica de dedup já em sink.rs
    }
}
```

Trade-off: comemo rastreia esta mutação. O valor é trackable porque
`Span` é `Hash+Eq+Copy` e `&str` é `Hash`. Severity (enum pequeno)
e hints (Vec<String>) ficam fora desta API simplificada — se
quisermos emitir warnings com hints via canal tracked, API mais
rica fica para passo futuro.

### Warning concreto a emitir (pilot)

Mensagem: `"info: eval() processou o ficheiro principal"` — note
informativa, prova de vida do canal.

Alternativa: detectar condição real do `source.text()` (ex:
ficheiro vazio, ou com BOM). Mais semântico mas requer leitura do
texto.

**Decisão pragmática**: emitir warning quando `source.text()` está
vazio. Condição detectável em `eval()` sem propagação extra,
semanticamente honesta.

```rust
if source.text().is_empty() {
    sink.warn(source.root().span(), "ficheiro vazio: sem conteúdo");
}
```

Span do root node serve como ponto de ancoragem.

---

## Parte 3 — Bloqueios técnicos

### `SourceDiagnostic` não é `Hash`

Não bloqueia esta API: passamos `span: Span` + `message: &str`
(ambos Hash-compatíveis). O `SourceDiagnostic` é construído
dentro do método tracked — o argumento da API tracked é
minimalista.

### `#[comemo::track]` com `&mut self`

Confirmar que comemo aceita métodos `&mut self` tracked. O vanilla
tem `SinkSlot` com tracking; presumível OK.

**Gate: testar compilação antes de escrever testes completos**. Se
comemo recusar `fn warn(&mut self, span: Span, message: &str)`,
recuar: adicionar API non-tracked adicional como `&mut Sink` param
(mudança de assinatura do eval, fora do escopo). Neste caso reportar
e parar.

---

## Parte 4 — Construção em callers

### Caller L3 real de produção: não existe

`03_infra/src/integration_tests.rs` é `#[cfg(test)]`. Não há CLI
real em L3 ou L4 que chame `eval`.

`04_wiring/src/main.rs` existe mas limita-se a `println!`.

**Decisão**: este passo **não cria** CLI real. Em vez disso:
- Actualiza `do_eval` em `03_infra/integration_tests.rs` para ler
  warnings após eval e `eprintln!` cada um.
- Adiciona teste que input Typst dispara warning e confirma chegada.

Se e quando a CLI real for materializada (trabalho futuro), este
padrão serve de referência.

---

## Recomendação

1. **Adicionar método tracked `warn(span, message: &str)`** ao
   bloco `#[comemo::track] impl Sink` em `world_types.rs`. Delega
   para `sink.warn(...)` via dedup já existente.
2. **Micro-piloto em `eval()`**: emitir warning se `source.text()
   .is_empty()`. Ponto de emissão directo; zero propagação.
3. **Actualizar `do_eval`** em `03_infra/integration_tests.rs`
   para ler `sink.into_diagnostics()` após eval e `eprintln!`.
4. **Testes novos** (esperado ~4):
   - `sink_canal_emite_warning_para_ficheiro_vazio` — prova o
     canal funciona end-to-end.
   - `sink_canal_dedup_end_to_end` — ficheiro vazio processado
     duas vezes via caches comemo — dedup deve manter 1 warning
     (ou N conforme semântica comemo; decidir em 106.D).
   - `sink_canal_vazio_quando_sem_trigger` — ficheiro normal,
     `sink.is_empty()` após eval.
   - `sink_canal_format_minimo` — teste do formato `"warning:
     ... ..."` aplicado.

### DEBT-49 pendente após 106

- O DEBT-49 **não** é fechado por este passo. O piloto é
  informativo (ficheiro vazio), não o sítio DEBT-49.
- DEBT-49 continua a aguardar a propagação completa (5ª aplicação
  da ADR-0036).

### DEBT-51 fecha

- Canal provado end-to-end. Critério de conclusão do DEBT-51
  satisfeito.
