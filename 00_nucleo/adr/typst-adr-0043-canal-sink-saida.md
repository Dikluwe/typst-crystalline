# ADR-0043 — Canal de saída do `Sink` — `TrackedMut` no caller, formatação em L3

**Status**: EM VIGOR (Passo 106.E) — validado empiricamente com 4
testes L3 integrados a passar (canal end-to-end, ausência, formato
mínimo, sinks independentes por run), zero violations.
**Data**: 2026-04-23
**Autor**: Humano + IA
**Passo associado**: 106

---

## Contexto

ADR-0042 (Passo 104) materializou `Sink` em L1 com API real
(`warn/is_empty/into_diagnostics`) mas **sem canal de saída**:
nenhum caller chama `into_diagnostics` após `eval()` retornar.
Warnings acumulam e são descartados. DEBT-51 aberto no Passo 104
documenta a lacuna.

Este ADR define o canal end-to-end.

## Decisão

### Forma do canal

- `TrackedMut<Sink>` permanece na assinatura de `eval()` (Passo 12).
  **Não muda**.
- Caller constrói `Sink` localmente, passa `sink.track_mut()` a
  `eval`, e **após retorno** drena via `sink.into_diagnostics()`.

### Propriedades do canal

1. **Caller-owned**: o `Sink` vive no caller. A `TrackedMut` é só
   um handle.
2. **Read-after-return**: o caller só lê warnings depois do `eval`
   retornar. Durante a execução, warnings ficam invisíveis.
3. **Zero signature change**: `eval()` preserva os 6 parâmetros
   actuais. ADR-0036 continua válida.

### Formato em L1 vs L3

- **L1 fornece dados** (`Vec<SourceDiagnostic>`).
  - `SourceDiagnostic { severity, span, message, hints, trace }`.
  - Nenhuma formatação para humanos em L1.
- **L3 formata**. Para este passo, formato mínimo:
  ```
  warning: <Span-debug> <message>
  ```
  emitido em `stderr` via `eprintln!`.
- Formato rico (linha/coluna resolvidos via `Source`, cores
  ANSI, JSON, SARIF) é trabalho de passo futuro. Não decidido
  aqui.

### Método tracked `warn`

`Sink` ganha um método tracked para permitir o pilot em L1:

```rust
#[comemo::track]
impl Sink {
    pub fn warn(&mut self, span: Span, message: &str) {
        // delega para a API não-tracked (com dedup)
    }
}
```

Argumentos `Span` (Hash + Copy) e `&str` (Hash) são compatíveis
com comemo tracking.

A API não-tracked em `sink.rs` (`warn(&mut self, diag:
SourceDiagnostic)`) continua para callers que precisem de emitir
com hints/severity explícito. Serão usados quando DEBT-49 for
atacado com propagação directa (`&mut Sink` parâmetro).

### Micro-piloto

`eval()` emite warning quando `source.text().is_empty()`:

```rust
pub fn eval(
    _routines: &Routines,
    world: &dyn World,
    _traced: Tracked<Traced>,
    sink: TrackedMut<Sink>,   // deixa de ser _
    _route: Tracked<Route>,
    source: &Source,
) -> SourceResult<Module> {
    if source.text().is_empty() {
        sink.warn(Span::detached(), "ficheiro vazio: sem conteúdo");
    }
    // ... resto do eval
}
```

### Alternativas rejeitadas

1. **Tuple return** (`eval → (Module, Vec<SourceDiagnostic>)`):
   invasivo, toca todos callers + ergonomia pior.
2. **Formatter em L1** (função `fmt_diag(d) -> String`):
   confunde responsabilidades; `SourceDiagnostic` já tem campos
   suficientes.
3. **Propagar `&mut Sink` pelos `eval_*`** (DEBT-49 completo):
   trabalho dedicado; fora do escopo do canal.
4. **Migrar DEBT-49 site como pilot** (`#set text(font: ...)`):
   requer propagação 3 níveis — excede o gate. Pilot vira
   "fala-pouco" em `eval()` top level.

### Relação com DEBT-49

Este ADR abre o canal. **Não resolve** DEBT-49 (migração dos
sítios silenciados em `eval_set_rule`, etc.). DEBT-49 é
trabalho dedicado que propagará `sink: &mut Sink` pelas funções
`eval_*` — 5ª aplicação da ADR-0036.

Com o canal aberto, DEBT-49 passa a ter valor prático imediato
(warnings chegam ao utilizador). Antes do Passo 106 não tinha.

### O que esta ADR não decide

- **Formato rico** (linha/coluna, cores, JSON): adiado.
- **CLI real com argumentos**: `04_wiring/src/main.rs` permanece
  stub. Novo passo materializará.
- **Warnings síncronos durante eval** (em vez de batched
  end-of-eval): adiado; exige stream de warnings que quebra
  contrato TrackedMut actual.

## Consequências

### Positivas

- DEBT-51 fecha. Warnings chegam ao caller.
- Padrão claro para DEBT-49: migrar sítios silenciados é passo
  mecânico — chamar `sink.warn(...)` onde for pertinente,
  propagando `&mut Sink` ou usando `TrackedMut::<Sink>` conforme
  contexto.
- Sink ganha primeira utilidade real após Passo 104.

### Negativas

- Formato `"warning: <Span-debug> <message>"` é feio. `Span` é
  índice opaco. Resolver para linha/coluna exige passo futuro.
- Micro-piloto (`ficheiro vazio`) é informativo, não resolve
  DEBT-49. O caso mais visível (`#set text(font: ...)`) continua
  silenciado.

### Neutras

- `_sink` em `eval()` passa a `sink` (sem underscore) — reflecte
  que o parâmetro é usado.

---

## Referências

- `00_nucleo/diagnosticos/inventario-sink-canal-passo-106.md`
- `00_nucleo/materialization/typst-passo-106.md`
- ADR-0042 (Sink materializado).
