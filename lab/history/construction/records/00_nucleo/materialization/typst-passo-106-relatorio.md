# Passo 106 — Relatório de encerramento (canal Sink → L3, DEBT-51)

**Data**: 2026-04-23
**Precondição**: Passo 105 encerrado; auditoria DEBTs confirmou
alinhamento; 803 L1 + 174 L3 + 6 ignorados; zero violations.
**ADR criada**: ADR-0043 "Canal de saída do Sink — TrackedMut no
caller, formatação em L3" — **PROMOVIDA A EM VIGOR** em 106.E.

---

## Sumário

Canal de warnings end-to-end activo. Caller L3 (`do_eval_with_sink`
em `03_infra/integration_tests.rs`) constrói `Sink`, passa
`track_mut()` a `eval()`, e lê `sink.into_diagnostics()` após
retorno. Formato mínimo: `"warning: <Span-debug> <message>"` via
`eprintln!`.

Micro-piloto em `eval()`: emite warning quando `source.text().is_empty()`.
Prova de vida do canal — validado por 4 testes de integração L3.

**DEBT-51 ENCERRADO**. DEBT-49 permanece aberto mas **desbloqueado**
(agora que o canal existe, migrar sítios silenciados traz valor
prático imediato).

Zero regressão funcional: **803 L1 + 178 L3 (+4) + 6 ignorados**.
`crystalline-lint .` → zero violations.

---

## 106.A — Inventário

Inventário em `00_nucleo/diagnosticos/inventario-sink-canal-passo-106.md`.

### Callers actuais de `eval()`

| Ficheiro | Tipo | Construía Sink | Lia warnings |
|----------|------|---------------:|-------------:|
| `01_core/rules/eval/tests.rs:31` (`eval_for_test`) | Helper L1 | ✓ | ✗ |
| `03_infra/src/integration_tests.rs:67` (`do_eval`) | Teste L3 | ✓ | ✗ |

Ambos descartavam o Sink. Nenhum caller produção real existe hoje
(04_wiring/main.rs é stub).

### Pilot escolhido

Preferência 1 do spec (DEBT-49 site) requer propagação de 3 níveis —
excede gate. **Preferência 2** escolhida: emitir dentro de `eval()`
via método tracked de `Sink`. Warning: "ficheiro vazio: sem conteúdo"
quando `source.text().is_empty()`.

### Bloqueio técnico identificado e resolvido

- `Sink` precisava de `Clone` para satisfazer o contrato de
  `#[comemo::track]` (comemo clona estado para rollback de tracked
  mutations). **Adicionado** `#[derive(Clone)]` à struct — O(n)
  no número de diagnósticos acumulados; aceitável porque dedup
  garante N baixo.
- Argumentos Hash: `Span` (Copy+Hash) + `&str` (Hash) compatíveis.

---

## 106.B — ADR-0043

Criada em `00_nucleo/adr/typst-adr-0043-canal-sink-saida.md`.
**Promovida a EM VIGOR em 106.E**.

Decisões principais:

- **Canal**: `TrackedMut<Sink>` caller-managed (assinatura de `eval()` preservada).
- **Formato**: L1 fornece `Vec<SourceDiagnostic>`; L3 formata. Mínimo:
  `"warning: <Span-debug> <message>"` via `eprintln!`.
- **Método tracked `Sink::warn_note(span, &str)`** para emissão via
  `TrackedMut` (comemo-compatível).
- **API não-tracked `Sink::warn(diag)`** mantida para callers com
  `&mut Sink` directo (DEBT-49 futuro).
- Alternativas rejeitadas: tuple return (invasivo), formatter em L1
  (confunde responsabilidades), propagar `&mut Sink` (DEBT-49
  completo fora do escopo).

---

## 106.C — Implementação

### `entities/sink.rs`

- `#[derive(Clone)]` adicionado à struct `Sink` (requisito comemo).
- Método `record(&mut self, diag)` como helper partilhado entre
  `warn` (API directa) e `warn_note` (API tracked em world_types.rs).

### `entities/world_types.rs` — bloco `#[comemo::track] impl Sink`

Antes: vazio (`impl Sink {}`).

Depois:

```rust
#[comemo::track]
impl Sink {
    pub fn warn_note(
        &mut self,
        span: crate::entities::span::Span,
        message: &str,
    ) {
        let diag = crate::entities::source_result::SourceDiagnostic::warning(
            span,
            message.to_string(),
        );
        self.record(diag);
    }
}
```

### `rules/eval/mod.rs` — pilot

`_sink: TrackedMut<Sink>` → `mut sink: TrackedMut<Sink>`.

```rust
if source.text().is_empty() {
    sink.warn_note(
        Span::detached(),
        "ficheiro vazio: sem conteúdo",
    );
}
```

### `03_infra/integration_tests.rs` — drenagem L3

Novo helper `do_eval_with_sink` que devolve `(SourceResult<Module>,
Vec<SourceDiagnostic>)`. `do_eval` original continua a existir para
testes que não se importam com warnings (usa `do_eval_with_sink`
internamente e descarta warnings).

Novo helper `drain_warnings_to_stderr(&[SourceDiagnostic])` com
formato `"warning: <Span-debug> <message>"` — disponível para a CLI
futura.

---

## 106.D — Testes

4 testes em `03_infra/src/integration_tests.rs`:

1. `sink_canal_emite_warning_para_ficheiro_vazio` — input `""` →
   1 warning com mensagem "ficheiro vazio".
2. `sink_canal_vazio_quando_sem_trigger` — input `"Olá mundo"` →
   `warnings.is_empty()` confirma que pilot não dispara.
3. `sink_canal_formato_minimo` — formato `"warning: ..."` aplicado
   a `SourceDiagnostic` produz string esperada.
4. `sink_canal_cada_run_tem_proprio_sink` — dois `eval`s
   independentes com input vazio produzem 1 warning cada
   (confirma que Sinks não partilham estado entre runs).

Todos passam.

---

## 106.E — Encerramento

### Verificação

```
$ cargo test --workspace | grep "test result"
test result: ok. 803 passed; 0 failed; 0 ignored ...     (L1 inalterado)
test result: ok. 178 passed; 0 failed; 6 ignored ...     (L3 +4)

$ crystalline-lint .
✓ No violations found
```

### DEBT-51 ENCERRADO

Movido para Secção 2 com secção "Encerramento (Passo 106)" detalhando
mudanças aplicadas.

### DEBT-49 estado

- Permanece **aberto** (não foi atacado neste passo).
- **Desbloqueado**: migrar sítios silenciados em `eval_set_rule` e
  similares agora traz valor prático (warnings chegam ao utilizador).
- Padrão arquitectural claro: propagar `sink: &mut Sink` pelas
  `eval_*` — 5ª aplicação da ADR-0036.

### ADR

**ADR-0043** `EM VIGOR`.

---

## Números finais

| Métrica | Antes | Depois |
|---------|------:|-------:|
| L1 tests | 803 | 803 (inalterado) |
| L3 tests | 174 | **178** (+4) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 42 | **43** (+0043) |
| DEBTs abertos | 14 | **13** (−DEBT-51) |

---

## Lições

1. **Pilot informativo em vez de DEBT-49 resolvido**: o site DEBT-49
   exige propagação 3 níveis — excede gate. Pilot "ficheiro vazio"
   é informativo mas serve de prova-de-vida do canal. Mecânica
   correcta; aplicação semântica forte fica para DEBT-49 completo.

2. **`Clone` exigido por comemo**: `#[comemo::track] impl` exige
   que a struct derive `Clone` (para rollback de mutações tracked).
   Descoberto por teste de compilação — não óbvio do spec comemo.
   Adicionado sem complicações (O(n) em N baixo).

3. **API dupla (tracked + não-tracked)**: `Sink` agora tem duas APIs:
   - `warn_note(span, &str)` tracked — para `TrackedMut<Sink>`,
     minimalista, compatível comemo.
   - `warn(diag: SourceDiagnostic)` não-tracked — para `&mut Sink`,
     rica (hints, trace), compatível com o consumo do DEBT-49
     futuro.
   
   Coexistência natural; cada consumidor usa a que se encaixa.
   Helper partilhado `record(diag)` garante dedup consistente.

4. **Canal sem CLI real**: este passo não materializa CLI de
   produção. O caller L3 (`do_eval_with_sink`) vive em testes. O
   padrão está pronto para replicação quando `04_wiring/main.rs` for
   materializado — será `let mut sink = Sink::new(); eval(...,
   sink.track_mut(), ...); for diag in sink.into_diagnostics() {
   eprintln!("warning: {:?} {}", diag.span, diag.message); }`.

5. **Formato opaco aceito**: `Span` imprime como `Span(N)` — não dá
   linha/coluna. Resolver exige `Source`, trabalho extra. ADR-0043
   regista a limitação; formato rico é passo dedicado.

---

## Estado pós-Passo 106

### Pipeline warnings end-to-end

```
source vazio
       ↓
eval(sink: TrackedMut<Sink>, ...)
       ↓
sink.warn_note(span, "ficheiro vazio: sem conteúdo")   [tracked]
       ↓
Sink acumula (com dedup por (span, message))
       ↓
eval retorna SourceResult<Module>
       ↓
caller drena: sink.into_diagnostics() -> Vec<SourceDiagnostic>
       ↓
formatter L3: eprintln!("warning: {:?} {}", span, message)
       ↓
stderr do utilizador
```

### Trabalho futuro identificado

1. **DEBT-49 agora accionável**: propagar `sink: &mut Sink` pelas
   `eval_*` (5ª aplicação ADR-0036) + migrar sítios silenciados.
2. **Formato rico**: resolver Span para linha/coluna via Source;
   cores ANSI; modos JSON/SARIF. Passo dedicado.
3. **CLI real em 04_wiring**: adicionar argumentos, orquestrar com
   Sink/warnings, retornar códigos de erro adequados.
4. **Integração com editores (LSP)**: warnings estruturados em
   JSON para consumers que não sejam CLI.
