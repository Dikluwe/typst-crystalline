# Relatório — typst-passo-803 (achado P798 #8): `visualize::curve` — mensagem de erro da API diverge

**Data:** 2026-07-21
**Executor:** Kimi Code (a pedido do utilizador, nesta conversa — prompt lido de `00_nucleo/materialization/typst-passo-803.md`)
**Proveniência das medições:** commit base `0661aef91c2ebc80d754d59e936c3a6350bfd543`. Sonda "antes" com working tree contendo P799–P802 (zonas não relacionadas). Validação "depois" com working tree não commitado: P799–P803. Hora da validação: 2026-07-21 ~16:35 -0300.
**Binários:** `./target/release/typst` (rebuild 16:35), `lab/typst-original/target/release/typst` (vanilla 0.15.0).

---

## Passo 1 — Sonda (antes)

Fonte original de P798 (`temp/p798/8_curve.typ`): `#curve((0pt, 0pt), (10pt, 10pt))`

Comandos: `./target/release/typst -o <out>.pdf temp/p798/8_curve.typ 2>&1` / `lab/typst-original/target/release/typst compile temp/p798/8_curve.typ <out>.pdf 2>&1`.

- cristalino (antes): `...8_curve.typ:1:6: error: curve(): segmento 0: primeiro elemento deve ser string (kind)`
- vanilla (2 erros, um por argumento):
  ```
  error: expected content, found array
    ┌─ temp/p798/8_curve.typ:1:7
  1 │ #curve((0pt, 0pt), (10pt, 10pt))
  ...
  ```

## Pontos exactos do código

**Vanilla**: os argumentos posicionais de `curve` são componentes (content produzido por `curve.move`/`curve.line`/etc.); um array bare falha a validação genérica de tipo do argumento → `expected content, found array` (`crates/typst-library/src/visualize/curve.rs`, casts de args). Não é mensagem específica de `curve` — é a validação genérica `expected X, found Y`.

**Cristalino**: `01_core/src/engine/stdlib/shapes.rs`, `native_curve` (antes) — qualquer argumento não-`Content::Curve` caía no caminho de tuplo legado: não-array → "curve(): argumento N não é um array de segmento válido"; array sem string no 1º elemento → "curve(): segmento N: primeiro elemento deve ser string (kind)".

## Causa local ou genérica?

**Local a `curve`.** A validação genérica de tipos do projecto já usa o padrão `expected X, found {type_name()}` noutros pontos (`foundations.rs:102`, `shapes.rs:478`); era `native_curve` que a contornava com mensagens próprias ao assumir que todo o argumento não-content era tuplo legado. A correcção foi feita em `native_curve`, não na origem comum (que já estava correcta).

## Passo 2 — Implementação

L0 actualizado primeiro: `00_nucleo/prompts/engine/stdlib/curve.md`, nota "P803 — erro de tipo paridade vanilla" na secção Consumo. Lint: `crystalline-lint --fix-hashes .` → "Nothing to fix" (nenhum ficheiro referencia `curve.md` por hash no cabeçalho; `crystalline-lint .` exit 0).

Diff de `01_core/src/engine/stdlib/shapes.rs` (`native_curve`):

```diff
-        let arr = match val {
-            Value::Array(a) if !a.is_empty() => a,
-            _ => {
-                return Err(vec![SourceDiagnostic::error(
-                    args.span,
-                    format!("curve(): argumento {} não é um array de segmento válido", i),
-                )])
-            }
-        };
-
-        let kind =
-            match &arr[0] {
-                Value::Str(s) => s.as_str(),
-                _ => return Err(vec![SourceDiagnostic::error(
-                    args.span,
-                    format!(
-                        "curve(): segmento {}: primeiro elemento deve ser string (kind)",
-                        i
-                    ),
-                )]),
-            };
+        // P803 — erro de tipo paridade vanilla: argumento que não é
+        // `Content::Curve` nem tuplo legado (array com 1º elemento string)
+        // → `expected content, found {type}` (vanilla medido:
+        // `#curve((0pt, 0pt))` → "expected content, found array").
+        let arr = match val {
+            Value::Array(a) if !a.is_empty() && matches!(a[0], Value::Str(_)) => a,
+            _ => {
+                return Err(vec![SourceDiagnostic::error(
+                    args.span,
+                    format!("expected content, found {}", val.type_name()),
+                )])
+            }
+        };
+
+        let kind = match &arr[0] {
+            Value::Str(s) => s.as_str(),
+            _ => unreachable!("guard acima garante arr[0] string"),
+        };
```

Os erros internos do tuplo legado (aridade, coordenada inválida, kind desconhecido) mantêm-se inalterados — `curve(("move", ...))` e `curve(("bogus", ...))` comportam-se como antes (coberto pelos testes P293/P294 existentes, que passam).

## Passo 3 — Validação (depois)

Mesmo comando:

- cristalino: `...8_curve.typ:1:6: error: expected content, found array` ✓ (mensagem igual à do vanilla)
- vanilla: `error: expected content, found array` (×2)

Observação registada (fora de âmbito): o vanilla emite **dois** erros (um por argumento inválido, com spans distintos) porque acumula diagnósticos; o cristalino pára no primeiro `Err`. Paridade de mensagem alcançada; a multiplicidade de diagnósticos é comportamento do colector de erros, não deste achado.

Teste novo (escrito primeiro; falhou antes em `stdlib/mod.rs:5140`):
- `p803_curve_arg_nao_content_erro_paridade_vanilla` — array bare de lengths → "expected content, found array"; controlo `Value::Int` → "expected content, found int".

Suíte `typst-core` (`cargo test -p typst-core --lib`):
- ANTES (fim de P802): **4323** passed + 1 ignored (total 4324).
- DEPOIS: **4324** passed; 0 failed; 1 ignored (total 4325 = +1 teste novo ✓).

Lint: `crystalline-lint .` → exit 0, zero violações.
