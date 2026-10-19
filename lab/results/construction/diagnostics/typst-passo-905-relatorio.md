# Relatório — Passo 905: `/` em argumentos de chamada de função em modo matemático

**Data:** 2026-07-25
**Commit de partida:** `1e8182221` (P904)

---

## Fase A — diagnóstico

### Sintoma catalogado (materialização)

`sqrt(x/y)` (e, por extensão de P899, `abs(x/y)`, `floor(x/y)`, etc.) produz saída malformada —
"só o primeiro operando aparece, com um glifo estranho por baixo".

### Isolamento do caso mínimo

Compilado directamente (`./target/release/typst`, sem passar por `sqrt`): `$ sqrt(1/2) $`
(dígitos) não mostrava o sintoma visualmente de forma óbvia — mas `$ sqrt(x/y) $` (variáveis, com
"y" tendo descendente) mostrava claramente: `√` pequeno, "x" normal, e um glifo de "y" cortado por
um traço horizontal a meio da altura (`pdftoppm -r 1200`, crop directo — ver reprodução abaixo).

**Achado decisivo, mais grave e mais geral do que a descrição original**: testado `$ x/y $` **sem
sqrt, sem chamada de função nenhuma** — o mesmo sintoma exacto reproduz-se. Testado ainda
`$ a/b $`, `$ 1/y $` — todos com o mesmo padrão: a barra de fracção corta o denominador sempre que
este tem um glifo com parte alta (ascendente) próxima da barra. **O bug não é específico a
argumentos de chamada de função — é um bug geral, pré-existente, em TODA a renderização de
fracções em modo matemático**, que também se manifesta (com o mesmo mecanismo) quando `/` aparece
dentro de um argumento de `sqrt`/`abs`/etc., porque esses argumentos são layoutados através do
mesmo `Content::MathFrac` → `layout_frac`.

Confirmado por recompilação do `.typ` de 30 secções
(`.typ/typst-math-comprehensive-test.typ`, hash inalterado desde P904): **praticamente todas** as
~15 fracções da Secção 1 ("Aritmetica Basica e Fracoes") e das secções seguintes (`d/dx`,
`n(n+1)/2`, `sin(x)/x`, `1/x`, `(delta q)/T`, etc.) mostravam o mesmo padrão de sobreposição.

### Causa exacta (leitura de código, não inferência)

`01_core/src/engine/math/layout/frac.rs::layout_frac` construía:

```rust
let num_y = 0.0_f64;
let den_y = num_box.height() + gap + rule_thickness + gap;
let rule_local_y = num_box.height() + gap + rule_thickness / 2.0;
```

assumindo (implicitamente — nunca declarado no código) a convenção "`local_y=0` = topo do
`MathBox`". A convenção **real**, já confirmada e documentada em **P901** (`root.md`, `root.rs`) via
leitura de `hconcat_spaced`/`layout_equation` (`math/layout/mod.rs`) e replicada correctamente em
`attach.rs` (base em `Pt(0.0)`, sup em `-sup_offset`, sub em `+sub_offset`), é: **`local_y=0` é a
BASELINE PRÓPRIA de cada `MathBox`, `y` cresce para baixo**. `frac.rs` nunca tinha sido auditado
contra essa convenção — o módulo é anterior a P800/P901 (Passo 9.8/136-137) e não foi tocado por
essa correcção, que só cobriu `root.rs`.

Sob a convenção errada, `num_y=0.0` deixava a baseline do numerador coincidir com a baseline da
própria fracção (sem subir), e `den_y`/`rule_local_y` eram calculados a partir de
`num_box.height()` — o resultado: a linha ficava correctamente posicionada em relação ao
**numerador** (que, por coincidência geométrica, acabava com folga de sobra acima da linha — daí
parecer "normal"), mas o **denominador**, colocado a `den_y` a partir da SUA PRÓPRIA baseline (que
também é `local_y=0` dentro da sua caixa), acabava com o seu ascendente a ultrapassar a linha —
sobreposição.

**Por que não foi apanhado antes**: o único teste de posição pré-existente,
`math_frac_numerador_acima_denominador`, verifica apenas **ordem** (`ys[0] < ys[1]`), que continua
verdadeira mesmo com o bug (`0 < valor_positivo`) — nunca verificou magnitude/gap.

### Vanilla como referência

Confirmado (`lab/typst-original/target/release/typst`) que `sqrt(x/y)` produz radical alto o
suficiente para cobrir `x` sobre `y`, com barra de fracção limpa entre os dois — sem sobreposição.

### Dispatch: hardcoded vs namespaced

Não relevante — a causa está inteiramente em `layout_frac` (chamado por `layout_node` para
qualquer `Content::MathFrac`, independentemente de como o `/` chegou lá — parsing de `a/b` solto,
argumento de `sqrt(...)`, ou de uma chamada namespaced). Não há dois caminhos de dispatch distintos
para `/` — um único ponto de falha.

---

## Fase B — Implementação

### Teste vermelho (RED, confirmado antes da correcção)

Três testes novos em `01_core/src/engine/math/layout/tests.rs`, verificando **magnitude** do gap
(não só ordem):

- `p905_frac_numerador_tem_gap_acima_da_linha` — passava mesmo com o bug (numerador tinha folga de
  sobra, consistente com o achado acima).
- `p905_frac_denominador_tem_gap_abaixo_da_linha_nao_sobrepoe` — **FAILED** antes da correcção:
  `den_top_ink=5.35 rule_y=11.08` (denominador 5.7pt ACIMA da linha — sobreposição).
- `p905_sqrt_de_fraccao_com_variaveis_nao_produz_saida_malformada` — caso literal
  `sqrt(frac(x,y))` da materialização — **FAILED** antes da correcção, com os mesmos números
  exactos do teste directo (confirma que o caminho `sqrt(...)` não introduz nenhuma lógica
  adicional — herda directamente o bug de `layout_frac`).

### Correcção

```rust
let rule_local_y = 0.0_f64;
let num_y = -(num_box.descent + gap + rule_thickness / 2.0);
let den_y = gap + rule_thickness / 2.0 + den_box.ascent;
```

A linha fica exactamente na baseline própria do `MathBox` da fracção; o numerador sobe o
suficiente para o seu descent parar `gap` acima da linha; o denominador desce o suficiente para o
seu ascent parar `gap` abaixo da linha. `ascent`/`descent` do `MathBox` resultante **não mudaram**
(a fórmula já estava correcta — só os offsets dos items internos estavam errados).

### Verde (GREEN, confirmado depois)

Os 3 testes novos passam. Suíte completa:

```
typst-core:    4746 passed; 0 failed; 3 ignored   (+3 vs P904, os 3 novos testes)
typst-infra:    734 passed; 0 failed; 5 ignored
typst-shell:     41 passed; 0 failed
```

Zero regressões.

### Frações normais fora de chamada de função

Confirmado explicitamente — não há dois caminhos: qualquer `/` em modo matemático (solto ou dentro
de argumento) passa por `layout_frac`. A correcção cobre ambos os casos pela mesma alteração (não
há risco de "consertar um e partir o outro", porque é literalmente o mesmo código).

### Confirmação visual

- `$ a/b $`, `$ x/2 $`, `$ 1/y $` (bare, sem função): barra limpa, sem sobreposição, em qualquer
  posição de gap.
- `$ sqrt(x/y) $`: `x` e `y` ambos visíveis, barra entre os dois, sem glifo cortado.
- Recompilado `.typ/typst-math-comprehensive-test.typ` (todas as 30 secções): todas as fracções
  observadas nas Secções 1, 4, 5, etc. (`d/dx`, `n(n+1)/2`, `sin(x)/x`, `(a+b)/(c+d)`,
  `sqrt(a^2+b^2)`) renderizam correctamente — sem sobreposição em nenhuma.

### Achado novo, separado, fora de âmbito: `√` não escala para radicando alto

Ao confirmar visualmente `sqrt(x/y)`, observado que o símbolo `√` continua **do mesmo tamanho** que
`sqrt(x)` (radicando de um único carácter) — não se expande para cobrir a fracção, apesar de
`root.rs` já usar `rad_box.ascent + rad_box.descent` (que, após esta correcção, já reflecte
correctamente a altura da fracção) para calcular `min_height_du` antes de chamar
`layout_stretchy_delimiter('√', min_height_du, style)`. Confirmado por `pdftotext -bbox`: a bbox do
`√` é **idêntica** (dentro de <1pt) entre `sqrt(x)` e `sqrt(x/y)` — `layout_stretchy_delimiter` não
está a escalar, apesar de receber um `min_height_du` maior. Isto é um bug **separado e
pré-existente** em `stretchy.rs`/`layout_stretchy_delimiter` (ou nos dados de variantes de glifo
consumidos por ele) — **não** o que P905 descreve como "malformado" (x e y aparecem ambos,
correctamente posicionados; o problema é apenas cosmético — o radical fica visualmente "apertado").
Fora de âmbito deste passo; registado em `frac.md` (secção P905) como achado a investigar num
passo dedicado a `layout_stretchy_delimiter`.

### `cargo run -- .` / `crystalline-lint`

L0 `00_nucleo/prompts/engine/math/layout/frac.md` actualizado com secção "P905" (achado, causa,
correcção — mesmo padrão da secção P901 já existente em `root.md`). `crystalline-lint --fix-hashes
.`: 1 ficheiro sincronizado (`frac.rs` → hash `9741236f`). Re-análise: **0 drift warnings**. Lint
completo (`crystalline-lint .`): só o warning V7 pré-existente e não relacionado
(`infra/package_version_resolution.md`, órfão desde antes deste passo).

---

## Fase C — Regressão

Benchmark, 7 cenários, `hyperfine --warmup 5 -N -m 20`. **Nota de proveniência**: os ficheiros de
cenário dos passos anteriores (P896-904) não estão persistidos no repositório (eram scratch de
sessão) — recriados nesta sessão em
`/tmp/.../scratchpad/bench/{01-hello,...,07-context}.typ` com conteúdo equivalente (mesmo género:
hello-world, prosa lorem, imagens/formas, math-heavy, tabelas, documento longo, contexto/estado) —
não são bit-a-bit os mesmos ficheiros das sessões anteriores, pelo que os números não são
directamente comparáveis byte-a-byte, apenas por ordem de grandeza:

| Cenário | Tempo (média) |
|---|---|
| 01-hello | 94.1 ms |
| 02-lorem | 123.2 ms |
| 03-images | 93.9 ms |
| 04-math | 137.2 ms |
| 05-tables | 99.5 ms |
| 06-long | 184.6 ms |
| 07-context | 107.3 ms |

Nenhum valor fora de ordem de grandeza face às baselines registadas em P903 (`01-hello` 95.1ms,
`04-math` 156.4ms) — sem sinal de regressão.

---

## Resumo

| Item | Veredicto | Estado |
|---|---|---|
| `/` em argumentos de função (sintoma catalogado) | Causa real é mais geral: bug em TODA fracção matemática, não só argumentos de função | ✅ Corrigido |
| `layout_frac` — convenção de coordenadas | Usava "topo do box" em vez de baseline-relativa (confirmada em P901, nunca auditada aqui) | ✅ Corrigido |
| `√` não escala para radicando alto (achado novo) | Bug separado em `layout_stretchy_delimiter`/`stretchy.rs`, cosmético, não "malformado" | ⚠️ Fora de âmbito, registado para passo dedicado |
