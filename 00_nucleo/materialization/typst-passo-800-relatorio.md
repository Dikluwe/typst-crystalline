# Relatório — typst-passo-800 (achado P798 #7): `syntax::kind` — ordem/conteúdo do output diverge em `#if` com math inline

**Data:** 2026-07-21
**Executor:** Kimi Code (a pedido do utilizador, nesta conversa — prompt lido de `00_nucleo/materialization/typst-passo-800.md`)
**Proveniência das medições:** commit base `0661aef91c2ebc80d754d59e936c3a6350bfd543`. Sonda "antes" corrida com working tree contendo apenas as alterações de P799 (irrelevantes para este achado — verificado: `$x^2$` isolado já posicionava correctamente antes de P799). Validação "depois" com working tree não commitado: P799 + P800 (`git diff HEAD --stat`: 8 ficheiros, +213/-31 — inclui os ficheiros de P799). Hora da validação: 2026-07-21 ~15:55 -0300.
**Binários:** `./target/release/typst` (rebuild 15:55), `lab/typst-original/target/release/typst` (vanilla 0.15.0).

---

## Passo 1 — Sonda (antes da correcção)

### 1.1/1.3 — Caso original e isolamento da variável

Comandos: `./target/release/typst -o <out>.pdf <fonte>.typ` / `lab/typst-original/target/release/typst compile <fonte>.typ <out>.pdf`; extracção `pdftotext <pdf> -`.

| Fonte | Cristalino (antes) | Vanilla |
|---|---|---|
| `#if true [Hello $x^2$]` (original, `temp/p798/7_kind.typ`) | `2` / `Hello x` (2 linhas) | `Hello 𝑥2` |
| `#if true [Hello]` (sem math) | `Hello` | `Hello` |
| `Hello $x^2$` (sem `#if`) | `2` / `Hello x` (2 linhas) | `Hello 𝑥2` |
| `#if true [$x^2$]` (só math) | `x2` | `𝑥2` |

### 1.2 — Em que consiste a diferença

`#if` é **irrelevante** — a divergência existe em `Hello $x^2$` sem `#if` e desaparece em `#if true [Hello]` sem math. Duas componentes:

1. **Baseline do math inline desalinhada** (causa geométrica real). `mutool trace` de `Hello $x^2$`, cristalino ANTES:
   ```
   H..o  y=763.785  (texto, 11pt)
   x     y=769.285  (math — 5.5pt = axis_pt acima da baseline do texto)
   2     y=773.267  (sup)
   ```
   Vanilla (`mutool trace`): texto e math na **mesma** baseline (spans com `y=0` na mesma transform). O desalinhamento de 5.5pt fazia o `pdftotext` partir a linha ("2" extraído em linha separada, antes de "Hello x") — a "ordem completamente diferente" do achado.
2. **Itálico matemático ausente** (`x` plain vs `𝑥` U+1D465) — a observação de P786 §7, causa distinta (selecção de fonte/estilo math), **não** tratada neste passo.

### 1.4/1.5 — Pontos exactos do código

**Vanilla**: math inline integra-se na linha de texto com baselines coincidentes — o shift vertical dos scripts é interno à fórmula (`lab/typst-original/crates/typst-layout/src/math/scripts.rs`, `compute_script_shifts`), não há deslocamento global da fórmula.

**Cristalino**: `01_core/src/engine/layout/equation.rs` (antes) — integração aplicava `offset_y = cursor_y - axis_pt` ("Passo 48": alinhar o **eixo matemático** à baseline do texto). Refutado por medição: o vanilla alinha a **baseline** (o eixo fica `axis_height` acima e só governa o centrado interno via `apply_axis_offset`). A regra errada estava consagrada no L0 (`00_nucleo/prompts/engine/layout/equation.md`) e no teste `equacao_inline_sobe_em_relacao_ao_baseline`.

### Confirmação de fusão com o achado #13 (P799)

**NÃO se fundiu.** Causas distintas: #13 era composição horizontal errada de sub+sup dentro de `layout_attach`; #7 era o offset global `axis_pt` na integração equação↔texto. `$x^2$` isolado já posicionava o sup correctamente antes de ambos os passos (verificado por `mutool trace` na sonda de P799).

## Passo 2 — Implementação

L0 actualizado primeiro: `00_nucleo/prompts/engine/layout/equation.md` (regra inline reescrita: baseline com baseline) e `00_nucleo/prompts/engine/math/layout/_comum.md` (nota P800: items de `layout_equation` são baseline-relativos). Hashes corrigidos com `crystalline-lint --fix-hashes .` (`equation.rs`: `6fea7989` → `d63d4e36`).

Os items de `MathLayouter::layout_equation` já vêm com `y = 0` na baseline da fórmula (`place(0.0, baseline_y = ascent)` ⇒ `parent_y = local_y`, e a composição interna é baseline-relativa — ex.: sup em `-sup_offset`). Logo a integração correcta é somar `cursor_y`, sem nenhum shift.

Diff de `01_core/src/engine/layout/equation.rs`:

```diff
-        // Equações inline: deslocar para cima por axis_pt de modo a que o
-        // eixo matemático (axis_height acima da baseline) coincida com o
-        // baseline do texto circundante (Passo 48).
-        let axis_pt = if block {
-            Pt(0.0)
-        } else {
-            let c = self.metrics.math_constants();
-            c.to_pt(c.axis_height, self.style.size)
-        };
-        let offset_y = self.regions.current.cursor_y - axis_pt;
+        // **P800** — Equações inline: a baseline da fórmula coincide com a
+        // baseline do texto circundante (paridade vanilla medida por
+        // `mutool trace` — texto e math partilham o mesmo y). Basta somar
+        // `cursor_y`. (...) Bloco: já era `cursor_y` — inalterado.
+        let offset_y = self.regions.current.cursor_y;
```

(Bloco inalterado — `axis_pt` já era 0 para `block: true`.)

Nota de processo: uma primeira versão deste passo subtraiu o `ascent` do `MathBox` (`offset_y = cursor_y - math_ascent`, com método novo `layout_equation_with_ascent`) — errada, piorou o desvio (8.8pt); os testes novos apanharam-na. Foi revertida dentro do próprio passo; o código final é o diff acima. Os L0s foram alinhados com a solução final.

## Passo 3 — Validação (depois da correcção)

### 3.2 — Mesmos comandos

`mutool trace` de `#if true [Hello $x^2$]`, cristalino DEPOIS:

```
H..o  y=763.785  (texto)
x     y=763.785  (math — MESMA baseline ✓)
2     y=767.767  (sup, 3.98pt acima — análogo ao vanilla ~4pt)
```

`pdftotext`: cristalino `Hello x2` vs vanilla `Hello 𝑥2`. Ordem e agrupamento de linha agora iguais; residual apenas `x` vs `𝑥` (itálico matemático — achado separado de P786 §7, continua em aberto). Nos termos da regra de disciplina §4: **corrigido para ordem/posição; conteúdo difere no estilo itálico matemático, não testado/corrigido neste passo**.

### 3.3 — Testes

Novos em `01_core/src/engine/layout/tests.rs` (escritos primeiro; falharam antes com a diferença exacta de `axis_pt`: `texto_y=78.567 math_y=73.067`):
- `equacao_inline_baseline_coincide_com_texto` — baseline de `$x$` == baseline de "Hello".
- `if_com_math_inline_attach_baseline_coincide` — caso original `#if true [Hello $x^2$]`: baseline coincide + sup acima.

Existente alterado: `equacao_inline_sobe_em_relacao_ao_baseline` — consagrava a regra Passo 48 refutada; marcado `#[ignore]` com justificação e ponteiro para o teste substituto (não apagado, para preservar o registo).

### 3.4 — Suíte `typst-core`

- ANTES (fim de P799): `cargo test -p typst-core --lib` → **4319** passed, 0 ignored.
- DEPOIS: `test result: ok. **4320** passed; 0 failed; **1 ignored`** (4319 + 2 novos = 4321 total; 4320 passed + 1 ignored = 4321 ✓).

### 3.5 — Lint

`crystalline-lint .` → exit 0, zero violações.

## Passo 4 — Estado dos achados

- **#7 (`syntax::kind`)**: causa real identificada e corrigida (baseline do math inline); não era `#if` nem `syntax::kind`. Residual `𝑥` vs `x` = itálico matemático, achado separado.
- **P786 §7 (itálico matemático)**: confirmado como causa distinta e **continua em aberto** — afecta todo o math (`$alpha beta$` → `αβ` vs `𝛼𝛽`; `x` vs `𝑥`). Candidato a passo próprio (selecção de fonte math / mapeamento para alfanuméricos matemáticos).
