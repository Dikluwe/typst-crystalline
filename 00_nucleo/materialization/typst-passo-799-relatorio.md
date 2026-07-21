# Relatório — typst-passo-799 (achado P798 #13): `math::attach` — sub/superscript quebrado

**Data:** 2026-07-21
**Executor:** Kimi Code (a pedido do utilizador, nesta conversa — prompt lido de `00_nucleo/materialization/typst-passo-799.md`)
**Proveniência das medições:** commit base `0661aef91c2ebc80d754d59e936c3a6350bfd543` (2026-07-21 08:02:32 -0300). Sonda "antes" corrida com working tree limpo sobre esse commit. Validação "depois" corrida com working tree não commitado contendo exactamente: `00_nucleo/prompts/engine/math/layout/attach.md`, `01_core/src/engine/math/layout/attach.rs`, `01_core/src/engine/math/layout/tests.rs` (`git diff HEAD --stat`: 3 ficheiros, +121/-9). Hora da validação: 2026-07-21 ~15:32 -0300.
**Binários:** `./target/release/typst` (cristalino, rebuild 15:25–15:32), `lab/typst-original/target/release/typst` (vanilla 0.15.0, rev `969087ec`).
**Fonte `.typ` original do achado (confirmada em `temp/p798/13_math_attach.typ`):** `$integral_0^1 x^2_3$`

---

## Passo 1 — Sonda (antes da correcção)

### 1.1/1.2 — Caso original e casos isolados (texto extraído, indicador fraco — regra §5)

Comandos: `./target/release/typst -o <out>.pdf <fonte>.typ` (cristalino) e `lab/typst-original/target/release/typst compile <fonte>.typ <out>.pdf` (vanilla); extracção com `pdftotext <pdf> -`. (Nota de processo: a primeira tentativa usou `compile` — sintaxe vanilla — com o binário cristalino, que não tem subcomandos; o output posicional do cristalino funciona e foi verificado. Sem impacto nas medições.)

| Fonte | Cristalino (antes) | Vanilla |
|---|---|---|
| `$integral_0^1 x^2_3$` | `∫10x23` | `1` / `∫ 𝑥23` / `0` (3 linhas — limites acima/abaixo na extracção) |
| `$x^2$` | `x2` | `𝑥2` |
| `$x_1$` | `x1` | `𝑥1` |
| `$x_1^2$` | `x21` | `𝑥21` |

Nota: a nota anterior sobre "CLI rejeita output posicional" foi removida — refutada na sessão (causa: uso acidental de `compile`, sintaxe vanilla).

### 1.1b — Posições reais dos glifos (prova mecânica, regra §5 — `mutool trace`)

`mutool trace temp/p799/13_math_attach_c.pdf` (cristalino, ANTES), glifos de `$integral_0^1 x^2_3$`:

```
∫  x=70.867  y=769.285  (11pt)
1  x=74.882  y=773.267  (7.7pt)   ← sup
0  x=78.462  y=767.855  (7.7pt)   ← sub: à DIREITA do sup, não empilhado
x  x=78.462  y=769.285  (11pt)    ← base seguinte SOBREPÕE-SE ao sub (mesmo x)
2  x=83.852  y=773.267  (7.7pt)
3  x=87.433  y=767.855  (7.7pt)   ← sub outra vez em sequência horizontal
```

Diagnóstico: com sub+sup simultâneos, o cristalino compunha os dois scripts **em sequência horizontal** (o cursor avançava depois do sup e o sub era colocado a seguir a ele), e a largura total do attach ignorava o sub — daí a sobreposição do `x` seguinte. Com apenas um script (`$x^2$`, `$x_1$`) a posição já estava correcta (sup ~4pt acima, escala 0.7 — análogo ao vanilla), o que explica o achado só se manifestar na combinação.

Fonte nos dois lados (regra §5.5): cristalino usa `CrystallineFont` (fallback) em todo o math; vanilla usa `NewCMMath-Book`. **Não** é a mesma família — diferença de fonte math registada como causa separada (ver §Sobreposições).

### 1.3 — Testes de sobreposição (antes)

- **Achado #7 (P800), `#if true [Hello $x^2$]`** (`temp/p798/7_kind.typ`): cristalino `2` / `Hello x` (duas linhas na extracção); vanilla `Hello 𝑥2`. Diferenças residuais: (a) ordem de extracção do sup (mecânica do `pdftotext`, geometria do sup já correcta) e (b) `x` plain vs `𝑥` itálico matemático.
- **P786 §7, itálico matemático** (`$alpha beta$`): cristalino `αβ`; vanilla `𝛼𝛽`.

### 1.4/1.5 — Pontos exactos do código

**Vanilla** — `lab/typst-original/crates/typst-layout/src/math/scripts.rs:170-175`:

```rust
let base_x = pre_width;
...
let tr_x = pre_width + base_width + tr_kern;   // post-superscript
let br_x = pre_width + base_width + br_kern;   // post-subscript — MESMA origem x
```

e largura total `width = pre_width + base_width + post_width` com
`post_width = max(t_post, b_post, tr_post, br_post)` (linhas 164-168).

**Cristalino** — `01_core/src/engine/math/layout/attach.rs`, braço não-`is_limits` (antes): cursor `x` único, colocava o sup em `x` e depois **avançava** `x += sup_box.width + kern_sup`, colocando o sub a seguir ao sup; `MathBox { width: x, .. }` nunca incluía a largura do sub.

### 1.6 — Sobreposições: confirmação/refutação

- **#7 (P800, `syntax::kind`)**: **REFUTADA como mesma causa.** A geometria de `$x^2$` isolado já estava correcta antes deste passo (verificado por `mutool trace`); a diferença residual de `#if true [Hello $x^2$]` é (a) mecânica de extracção e (b) ausência de estilo itálico matemático — não o mecanismo de `attach`. O caminho `#if` avalia correctamente (o conteúdo aparece). P800 segue em passo próprio para classificar o residual.
- **P786 §7 (itálico matemático)**: **REFUTADA como mesma causa.** É causa distinta: selecção de fonte/estilo em modo math (cristalino não mapeia letras para alfanuméricos matemáticos nem usa fonte math; `αβ` vs `𝛼𝛽`, `x` vs `𝑥`). Não é layout de `attach`. Fica em aberto, como estava.

## Passo 2 — Implementação

L0 actualizado primeiro (`00_nucleo/prompts/engine/math/layout/attach.md`, nova secção "Scripts laterais sub+sup partilham a origem x — P799"); hash do cabeçalho corrigido com `crystalline-lint --fix-hashes .` (`b3b29f07` → `572a89a1`).

Diff de `01_core/src/engine/math/layout/attach.rs` (braço não-`is_limits`):

```diff
-            // Right-scripts partem de base_offset_x + base_width.
-            let mut x = base_offset_x + base_width;
+            // P799 — sub e sup partem AMBOS de base_offset_x + base_width
+            // (cada um com o kern do seu quadrante), empilhados verticalmente,
+            // não compostos em sequência horizontal. Paridade vanilla
+            // `scripts.rs`: `tr_x = br_x = pre_width + base_width + kern`.
+            // A largura total é base + max(sup + kern_sup, sub + kern_sub).
+            let scripts_x = base_offset_x + base_width;
+            let mut post_width: f64 = 0.0;
             ...
                 for item in sup_box.items {
-                    items.push(offset_item(item, Pt(x + kern_sup), Pt(-sup_offset)));
+                    items.push(offset_item(item, Pt(scripts_x + kern_sup), Pt(-sup_offset)));
                 }
-                x += sup_box.width + kern_sup;
+                post_width = post_width.max(sup_box.width + kern_sup);
             ...
                 for item in sub_box.items {
-                    items.push(offset_item(item, Pt(x + kern_sub), Pt(sub_offset)));
+                    items.push(offset_item(item, Pt(scripts_x + kern_sub), Pt(sub_offset)));
                 }
+                post_width = post_width.max(sub_box.width + kern_sub);
             }
-            MathBox { width: x, ascent, descent, items }
+            MathBox { width: scripts_x + post_width, ascent, descent, items }
```

## Passo 3 — Validação (depois da correcção)

### 3.2 — Mesmos comandos, posições reais

`mutool trace` do cristalino DEPOIS (`temp/p799/13_math_attach_c2.pdf`):

```
∫  x=70.867  y=769.285  (11pt)
1  x=74.882  y=773.267  (7.7pt)   ← sup
0  x=74.882  y=767.855  (7.7pt)   ← sub: MESMO x do sup (empilhado)
x  x=78.462  y=769.285  (11pt)    ← base seguinte depois dos scripts, sem sobreposição
2  x=83.852  y=773.267  (7.7pt)
3  x=83.852  y=767.855  (7.7pt)   ← empilhado
```

Geometria agora equivalente à do vanilla (scripts partilham a origem x; elemento seguinte após `base + max(sup, sub)`). O texto extraído (`pdftotext`) mantém-se `∫10x23` — indicador fraco que não reflecte empilhamento (regra §5.3); a prova é o trace acima. Casos isolados `$x^2$` / `$x_1$` re-corridos: inalterados (já correctos). `#if true [Hello $x^2$]` e `$alpha beta$`: inalterados — confirmam que as sobreposições eram causas distintas.

### 3.3 — Testes novos (escritos primeiro; falharam antes da correcção)

`01_core/src/engine/math/layout/tests.rs`:
- `math_attach_sub_sup_partilham_origem_x` — sub+sup na mesma origem x (falha antes: `sup_x=7.2 sub_x=12.24`).
- `math_attach_sub_sup_largura_max_nao_soma_nucleo_multi_char` — núcleo multi-caractere `ab^22_333` + elemento seguinte `z`: largura = base + max(sup,sub) (falha antes: `sup_x=14.4 sub_x=24.48`).

Cobertura pedida pelo passo: sub sozinho (`math_attach_sub_baixado`, pré-existente), sup sozinho (`math_attach_sup_elevado`, pré-existente), combinação e núcleo multi-caractere (os dois novos).

### 3.4 — Suíte `typst-core`

- ANTES (commit `0661aef`, working tree limpo): `cargo test -p typst-core --lib` → **4317** testes (medido indirectamente: filtrando pelos 2 testes novos, `4317 filtered out` + 2 = 4319 total depois de os adicionar).
- DEPOIS: `cargo test -p typst-core --lib` → `test result: ok. **4319** passed; 0 failed` (4317 + 2 novos = 4319 ✓ bate com o declarado).

### 3.5 — Lint

`crystalline-lint .` → exit 0, zero violações. Nota de limpeza necessária: dois scripts descartáveis de P798 (`temp/run_p798_tests2.py`, `temp/run_p798_tests3.py`, não commitados, das 10:54/11:01 de hoje, conteúdo re-criável a partir dos `.typ` preservados em `temp/p798/`) estavam a disparar V8/V1 (fora da topologia) desde antes deste passo; movê-los para `temp/p798/` não chegou (o linter varre `.py` em qualquer ponto de `temp/`), pelo que foram **removidos** para repor o gate. Os 6 warnings V7 (prompts órfãos) são pré-existentes e não são violações (nível warning).

## Passo 4 — Estado dos achados

- **#13 (`math::attach`)**: corrigido e validado por posições reais de glifos.
- **#7 (P800)**: NÃO fechado por este passo — causa distinta (residual: extracção + itálico matemático); segue no seu próprio passo.
- **P786 §7 (itálico matemático)**: NÃO fechado — causa distinta (fonte/estilo math); continua em aberto.
- Handoff: sem alteração neste passo (a regra manda actualizar só se fechar #7/P786§7); a remoção do #13 da fila fica para a actualização do handoff no fim do lote.

### Observação fora de âmbito (registada)

Nenhuma. (A suspeita inicial de regressão do CLI foi refutada na própria sessão: o erro veio de usar `compile`, sintaxe do vanilla, com o binário cristalino — ver nota de processo em §1.1.)
