# Relatório — typst-passo-801 (achado P798 #4): `utils::protected` — representação de array de 1 elemento diverge

**Data:** 2026-07-21
**Executor:** Kimi Code (a pedido do utilizador, nesta conversa — prompt lido de `00_nucleo/materialization/typst-passo-801.md`)
**Proveniência das medições:** commit base `0661aef91c2ebc80d754d59e936c3a6350bfd543`. Sonda "antes" com working tree contendo P799+P800 (zonas não relacionadas — math layout). Validação "depois" com working tree não commitado: P799+P800+P801. Hora da validação: 2026-07-21 ~16:05 -0300.
**Binários:** `./target/release/typst` (rebuild 16:05), `lab/typst-original/target/release/typst` (vanilla 0.15.0).

---

## Passo 1 — Sonda (antes)

Comandos: `./target/release/typst -o <out>.pdf <fonte>.typ` / `lab/typst-original/target/release/typst compile <fonte>.typ <out>.pdf`; extracção `pdftotext <pdf> -`.

Fonte original de P798 (`temp/p798/4_protected.typ`): `#context [ #counter("mycounter").get() ]`

- cristalino (antes): `(0)`
- vanilla: `(0,)`

Controlo de outros tamanhos (`#repr(()) #repr((1, 2)) #repr((5,))`):

- cristalino (antes): `() (1, 2) (5)`
- vanilla: `() (1, 2) (5,)`

Confirmado: a diferença é **apenas** a vírgula final no caso de 1 elemento; 0 e 2+ já estavam correctos.

## Pontos exactos do código

**Vanilla** — `lab/typst-original/crates/typst-library/src/foundations/array.rs:1188-1200`:

```rust
impl Repr for Array {
    fn repr(&self) -> EcoString {
        ...
        repr::pretty_array_like(&pieces, self.len() == 1).into()  // trailing_comma = (len == 1)
    }
}
```

**Cristalino** — `01_core/src/engine/eval/repr.rs` (braço `Value::Array`, antes):

```rust
Value::Array(arr) => {
    let items: Vec<String> = arr.iter().map(repr_value).collect();
    format!("({})", items.join(", "))   // sem vírgula final para len == 1
}
```

O display embutido em markup (`#context [ #counter(...).get() ]`) passa por `value_to_display_content` (`01_core/src/engine/eval/mod.rs:707-713`), que delega em `repr_value` para arrays — logo uma única correcção cobre `repr()` e o display embutido (confirmado pelas duas sondas acima).

## Passo 2 — Implementação

L0 actualizado primeiro: `00_nucleo/prompts/engine/stdlib/foundations.md`, linha `array` da tabela de `repr` passa a registar a regra da vírgula final (P801). Hash corrigido com `crystalline-lint --fix-hashes .` (`stdlib/foundations.rs` → `17c5d094`).

Diff de `01_core/src/engine/eval/repr.rs`:

```diff
         Value::Array(arr) => {
             let items: Vec<String> = arr.iter().map(repr_value).collect();
-            format!("({})", items.join(", "))
+            // P801 — array de exactamente 1 elemento leva vírgula final
+            // (paridade vanilla `pretty_array_like(_, len == 1)`),
+            // distinguindo-o de parênteses de agrupamento: `(5,)` ≠ `(5)`.
+            if items.len() == 1 {
+                format!("({},)", items[0])
+            } else {
+                format!("({})", items.join(", "))
+            }
         }
```

## Passo 3 — Validação (depois)

Mesmos comandos, saídas literais:

- `#context [ #counter("mycounter").get() ]`: cristalino `(0,)` == vanilla `(0,)` ✓
- `#repr(()) #repr((1, 2)) #repr((5,))`: cristalino `() (1, 2) (5,)` == vanilla `() (1, 2) (5,)` ✓ (controlos 0 e 2+ inalterados)

Teste novo (escrito primeiro; falhou antes em `repr.rs:589`):
- `repr_value_array_um_elemento_virgula_final` — 1 elemento `"(5,)"`, aninhado `"((5,),)"`, controlos `"()"` e `"(1, 2)"`.

Suíte `typst-core` (`cargo test -p typst-core --lib`):
- ANTES (fim de P800): **4320** passed + 1 ignored (total 4321).
- DEPOIS: **4321** passed; 0 failed; 1 ignored (total 4322 = +1 teste novo ✓).

Lint: `crystalline-lint .` → exit 0, zero violações.
