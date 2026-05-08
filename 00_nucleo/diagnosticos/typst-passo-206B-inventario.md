# P206B — Inventário interno (reactivar harness + smoke vanilla CLI)

**Data**: 2026-05-08.
**Spec**: `00_nucleo/materialization/typst-passo-206B.md`.
**Output 1 de 3** (inventário interno).

---

## §1 C1 — Inventário empírico (5 sub-secções)

Etiquetas: `CONFIRMADO` / `AJUSTE NECESSÁRIO`.

### §1.1 C1.1 — `tests/layout_parity.rs:69`

**Status**: `CONFIRMADO`.

Estado pré-fix:

- Linha 29 (import): `use typst_core::rules::introspect::introspect;`
- Linha 68: `let state = introspect(content);`
- Linha 69: `let doc = layout(content, state);`

Erro `cargo check --all-targets`:

```
error[E0061]: this function takes 1 argument but 2 arguments were supplied
   --> parity/tests/layout_parity.rs:69:15
    |
 69 |     let doc = layout(content, state);
    |               ^^^^^^          ----- unexpected argument #2 of type `TagIntrospector`
note: function defined here
   --> 01_core/src/rules/layout/mod.rs:1480:8
    |
1480| pub fn layout(content: &Content) -> PagedDocument {
    |        ^^^^^^
```

Migração P190I: `layout(content, state)` → `layout(content)`.
A construção do introspector é interna (per ADR-0073 +
P190I). Variável `state` torna-se obsoleta — remover
linha 68. Import linha 29 fica unused — remover para
evitar warning.

Ajustes adjacentes detectados:

- Linha 67 (`let content = module.content()?;`) **mantida**
  — necessária para call.
- Linhas 65, 66 (eval, module) **mantidas** — produzem
  content.

### §1.2 C1.2 — `src/value_dto.rs:83`

**Status**: `CONFIRMADO`.

Estado pré-fix:

- Linha 82-110: `match v { ... 18 arms ... }`.
- Variants cobertos: None, Auto, Bool, Int, Float, Str,
  Array, Dict, Module, Datetime, Func, Content, Length,
  Ratio, Angle, Color, Fraction, Align (18).
- Variant `Value::Location` (P179) **ausente**.

Erro `cargo check --all-targets`:

```
error[E0004]: non-exhaustive patterns: `&Value::Location(_)` not covered
  --> parity/src/value_dto.rs:83:15
   |
 83|         match v {
   |               ^ pattern `&Value::Location(_)` not covered
note: `Value` defined here
  --> 01_core/src/entities/value.rs:73
   |
 73|     Location(crate::entities::location::Location),
   |     -------- not covered
```

Pattern arms outros (linhas 84-109):

- Variants opacos com tipo struct: `format!("{x:?}")` →
  string DTO. Ex: `Length(l) => ValueDTO::Length(format!("{l:?}"))`.
- Variants com inner value primitivo: extracção directa.
  Ex: `Bool(b) => ValueDTO::Bool(*b)`.

`Location` é struct opaco com `from_raw(u128)` API.
Pattern coerente: `format!("{loc:?}")` → string DTO.

ValueDTO **não tem variant `Location` próprio**; spec C3
fixa "1 line arm". Decisão D1 (§4): usar catch-all
`Other(String)` per convenção docstring l.65-67
("Variants vanilla sem equivalente cristalino mapeiam
para Other"). Ajuste: docstring catch-all assume
divergência vanilla; cristalino-only Location pode
justificar variant próprio em P206C/D — deferred.

Fix literal:
```rust
Value::Location(loc) => ValueDTO::Other(format!("location:{loc:?}")),
```

Adicionado pós linha `Value::Align(al) => ...`.

Docstring linha 80-81 actualizada de "18 variants" para
"19 variants (P206B: +Location per P179)".

### §1.3 C1.3 — Outros breaks não documentados

**Status**: `CONFIRMADO` — sem breaks adicionais.

`cargo check --manifest-path lab/parity --all-targets`
output completo (filtered for errors):

```
error[E0061]: this function takes 1 argument but 2 arguments were supplied
error[E0004]: non-exhaustive patterns: `&Value::Location(_)` not covered
error: could not compile `typst-parity` (test "eval_parity") due to 1 previous error
error: could not compile `typst-parity` (test "layout_parity") due to 1 previous error
```

**Apenas 2 erros únicos** (E0061 + E0004); P204F.div-1
auditoria exhaustiva confirmada empíricamente. Sem
`P206B.div-N` necessário.

Warnings pré-existentes (não breaks): código preparado
em `frame_dto.rs` para futura vanilla integration —
estruturas declaradas mas não invocadas (3+5 warnings
unused). Pre-P206B; preservados.

### §1.4 C1.4 — `parity-runner` smoke pré-fix

**Status**: `CONFIRMADO funcional`.

```
$ cargo run --manifest-path lab/parity/Cargo.toml --bin parity-runner -- "Hello *world*"
Finished `dev` profile [unoptimized + debuginfo]
Running `lab/target/debug/parity-runner 'Hello *world*'`
✓ Paridade confirmada (13 bytes)
```

Bin `parity-runner` (P1 paridade parse) **NÃO afectado**
pelos breaks (que vivem em `tests/`, não em `src/main.rs`
ou `src/compact.rs`). Funcionalidade preservada
pre-fix; deve permanecer pós-fix.

### §1.5 C1.5 — Convenção CLI detection

**Status**: `AJUSTE NECESSÁRIO`.

Grep empírico em workspace:
- `01_core/src` (L1 puro): zero usos de `Command::new`
  ou `std::process` — esperado per CLAUDE.md "L1 zero
  I/O".
- `03_infra/src`: zero usos directos de
  `Command::new` (grep vazio).
- `lab/parity/src/main.rs`: usa `std::process::exit`
  (linhas 11, 52) — pattern para abort. Não usa
  `Command::new`.

**Sem helper pre-existente** para invocação de binários
externos. Decisão D2 (§4): criar test novo que usa
`std::process::Command` directamente. `lab/` é
quarentena (per CLAUDE.md); não viola topologia L1.

Skip graceful: pattern `match Command::new("typst").
arg("--version").output() { Ok(o) => ..., Err(e) =>
{ eprintln!("..."); return; } }`. `Err` quando ausente;
`Ok` quando presente.

Localização escolhida (per spec C4): `lab/parity/tests/
vanilla_cli_smoke.rs` (novo ficheiro). Justificação:
- Test (não helper), porque fim último é
  sentinela executável via `cargo test`.
- Ficheiro dedicado, não anexar a layout_parity ou
  eval_parity, porque vanilla CLI é cross-cutting
  pré-condição para P206C/D.

---

## §2 C2 — Fix `layout_parity.rs:69`

Edições aplicadas (3 mudanças coordenadas):

### §2.1 Remover import unused

`tests/layout_parity.rs:29`:
```diff
 use typst_core::contracts::world::World;
-use typst_core::rules::introspect::introspect;
 use typst_core::rules::layout::layout;
```

### §2.2 Remover `let state = introspect(content);` + actualizar call

`tests/layout_parity.rs:67-71` (pré):
```rust
let module = result.ok()?;
let content = module.content()?;
let state = introspect(content);
let doc = layout(content, state);
Some(doc)
```

(pós):
```rust
let module = result.ok()?;
let content = module.content()?;
// P206B (P190I migration): `layout(content)` 1-arg.
// Introspector é construído internamente via
// `layout_with_introspector` ou semelhante per ADR-0073.
let doc = layout(content);
Some(doc)
```

Linhas afectadas: 68 (removida); 69 (signature
actualizada); +3L comentário inline a documentar a
migração.

---

## §3 C3 — Fix `value_dto.rs:83`

Edições aplicadas (2 mudanças):

### §3.1 Adicionar arm `Value::Location`

`src/value_dto.rs` (após arm `Value::Align`):
```diff
             Value::Align(al)    => ValueDTO::Align(format!("{al:?}")),
+            // P206B: variant `Value::Location` adicionada por P179.
+            // Mapeada para `Other("location:N")` per convenção
+            // catch-all (l.65-67) — extensão futura para
+            // `ValueDTO::Location` se P206C/D exigir.
+            Value::Location(loc) => ValueDTO::Other(format!("location:{loc:?}")),
         }
```

### §3.2 Actualizar docstring "18 → 19 variants"

`src/value_dto.rs:80-81`:
```diff
-    /// Conversão a partir do `Value` cristalino. Cobre os 18
-    /// variants existentes em `01_core/src/entities/value.rs`.
+    /// Conversão a partir do `Value` cristalino. Cobre os 19
+    /// variants existentes em `01_core/src/entities/value.rs`
+    /// (P206B: +`Location` per P179).
```

---

## §4 C4 — Vanilla CLI smoke

Ficheiro novo: `lab/parity/tests/vanilla_cli_smoke.rs`
(~75 LOC).

### §4.1 Estrutura

- Doc-comment top: contexto P206B + ADR-0075 +
  semântica skip graceful.
- `const VANILLA_EXPECTED_VERSION_PREFIX: &str = "0.14";`
  — match prefixo (não exacto `0.14.2 (b33de9de)`)
  per spec §8 robustez.
- 2 tests:
  - `p206b_vanilla_cli_disponivel_e_versao_compativel`
    — `typst --version` + match prefixo + skip graceful
    via `eprintln!` se ausente.
  - `p206b_vanilla_cli_query_subcomando_existe` —
    `typst query --help` confirma subcomando para
    pré-condição P206C.

### §4.2 Skip graceful semantics

`Command::new("typst").arg("--version").output()`:

- `Ok(o)`: continua com asserts.
- `Err(e)` (binário ausente em PATH): `eprintln!("[p206b
  smoke] vanilla \`typst\` ausente em PATH ({}); skip
  graceful. Para activar comparação vanilla, instalar
  via cargo install --git ... typst-cli ou package
  manager.", e)` + `return`.

Test **completa com sucesso** mesmo sem vanilla CLI —
não falha a suite. Per ADR-0075 §"Plano de validação"
cond 6 ("abort gracefully se ausente"). Per spec §8
risco "vanilla CLI smoke fragile" — evitado.

### §4.3 Versão pinning

Tolerância: prefixo `0.14` (matches `0.14.2 (b33de9de)`,
`0.14.0`, `0.14.5`, etc.). Falha se versão diverge
(0.15+ ou 0.13-) — aviso para sync com
`lab/parity/Cargo.toml typst-syntax v0.14.2`.

---

## §5 Decisões durante a leitura

### D1 — `Value::Location` mapeado para `Other`, não variant próprio

Spec C3 fixa "1 line arm". Adicionar `ValueDTO::Location`
variant próprio exigiria editar enum (mais arms; tests
PartialEq updated). `Other(format!("location:{loc:?}"))`
respeita convenção catch-all docstring (l.65-67) e
mantém minimal. P206C/D pode estender se vanilla
integration produzir output Location-shaped que exija
variant próprio para diff — deferred.

### D2 — Smoke test em `tests/`, não helper em `src/`

Vanilla CLI smoke é **sentinela executável** —
`cargo test --test vanilla_cli_smoke` confirma estado
em runtime. Helper em `src/` seria invocado por outros
módulos mas não testado isoladamente. Spec C4 deu
liberdade entre os dois; `tests/` aligna com pattern
de sentinelas existentes (eval_parity, layout_parity,
parse_parity).

### D3 — 2 tests no smoke (não 1)

Spec C4 menciona "test (ou função helper)". Decidi 2
tests por separação de concerns:
- Versão check (vanilla disponível + compatível).
- Subcomando query (pré-condição P206C).

Ambos com skip graceful idêntico. Custo marginal: ~30
LOC duplicados em padrão `match...output()`. Benefício:
falha granular — sabe-se se a regressão é em
disponibilidade ou em subcomando específico.

### D4 — Comentário inline em `layout_parity.rs:67-71`

Adicionado comentário curto referenciando P206B + P190I
migration. Útil para auditor futuro entender que linha
68 não foi removida por engano — foi migration
deliberada. Per CLAUDE.md "comentários só quando WHY
é não-óbvio": migration de signature outdated é WHY
não-óbvio sem comment.

### D5 — Docstring de 18 → 19 variants

Spec C3 fixa "1 line arm". Mas docstring linhas 80-81
afirma "Cobre os 18 variants" — desfazendo a verdade
empírica pós-fix. Ajuste docstring é continuação do
mesmo fix; não inflação.

### D6 — Sem `P206B.div-N` registado

C1.3 confirmou exhaustivamente: 2 breaks únicos. Per
spec §8 risco "assumir 2 breaks são únicos sem
verificar": auditoria empírica confirmou. Sem
divergência.

### D7 — Linter deve cobrir lab/?

Per CLAUDE.md "lab/ é quarentena, nunca importado por
L1-L4". `crystalline-lint .` actual reporta `0
violations` mesmo com lab/parity tocado — confirma
que linter ignora lab/ ou aplica regras compatíveis
com quarentena. Decisão correcta: não preocupação.

---

## §6 Resumo — métricas

| Métrica | Valor |
|---------|-------|
| Tests workspace cristalino antes | 1860 |
| Tests workspace cristalino depois | **1860** (∆ 0 — P206B é lab/parity) |
| Tests lab/parity antes | 52 (50 parse + 1 eval + 1 layout) |
| Tests lab/parity depois | **54** (+2 vanilla_cli_smoke P206B) |
| Linter violations | 0 (sem alteração) |
| Ficheiros código modificados | 2 (`tests/layout_parity.rs`; `src/value_dto.rs`) |
| Ficheiros código novos | 1 (`tests/vanilla_cli_smoke.rs`) |
| Ficheiros docs novos | 2 (este + relatório) |
| Ficheiros docs modificados | 1 (ADR-0075 §P206B anotação) |
| LOC novas (código) | ~75 (smoke test) + ~5 (fix comments) = ~80 |
| LOC removidas | ~3 (2 fix lines + 1 import) |
| Cargo deps adicionados | 0 |
| Refactor mid-execution | 0 |
| `P206B.div-N` registadas | 0 |
