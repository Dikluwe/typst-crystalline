# L0 — Passo 1102: Corrigir Captura Prematura de `offset_x` em `equation.rs`

**Gate**: `ADR-0127` — mudança de comportamento por defeito (qualquer
documento com equação de bloco embutida em parágrafo, sob `width/height:
auto`, é afectado).

**Base**: P1098-1101 (investigação). Causa confirmada com código real e
aritmética exacta: `offset_x` (L110 de `equation.rs`) é capturado do
`cursor_x` **antes** do `flush_line()` que fecha a linha de texto anterior,
e nunca é realinhado para `self.page_config.margin` depois. O valor errado
(posição do cursor no fim do texto anterior, não a margem) é gravado em
`pending_equation_centering` e usado por `compute_page_width`
(`mod.rs:815`), inflando a largura da página.

---

## 1. Mecanismo

Em `01_core/src/compiler/layout/equation.rs`, mover a captura de `offset_x`
para **depois** do `flush_line()` (linha ~135 no código já citado), não
antes (linha ~110). Depois do `flush_line()`, `cursor_x` deve já estar
resetado para o início da nova linha — confirmar se isso é
`self.page_config.margin` directamente ou se precisa de ser lido de outro
campo do `Layouter` pós-flush (não presumir, verificar contra o código real
do próprio `flush_line()`).

**Não presumir que basta mover a linha** — confirmar se `offset_x` é usado
para mais alguma coisa entre a L110 original e o ponto de inserção em
`pending_equation_centering` (L335-345) que dependesse do valor antigo
(pré-flush). Se houver, mover só a captura pode quebrar outro uso — ler o
corpo inteiro da função antes de editar.

## 2. Verificação — os 3 blocos da secção 31, não só o `sum`

| Bloco | `applied_offset` errado (antes) | `eq_width` | Esperado após correcção |
|---|---|---|---|
| `$ sum_(k=1)^n k^2 $` | 462.525pt | 35.945pt | `applied_offset = margin (28.3465pt)` |
| `$ a/b $` | 282.55pt (a confirmar) | 5.82pt | idem |
| `$ sqrt(x+1) $` | 300.65pt (a confirmar) | 28.90pt | idem |

Confirmar os três, não só recompilar e olhar para o resultado agregado —
cada bloco tem o seu próprio `offset_x` capturado de forma independente,
podem ter bugs distintos mesmo que a causa geral seja a mesma linha de
código.

## 3. Medição final

- `MediaBox Width` da secção 31: convergir para `494.5453pt` (valor vanilla
  já medido no P1101), não "muito mais perto".
- `MediaBox Height`: mesma verificação (`263.8196pt`).
- Cascata de centragem (P1101, `+16.1368pt` em cada bloco) deve desaparecer
  por si, sem correcção própria — confirmar isso, não presumir (mesmo
  padrão de verificação de cascata já usado em P1100).

## 4. Offset vertical de ~1.27pt — separado, não presumir resolvido

P1101 já identificou este como causa independente (`shift_up` de sobrescrito
em `attach.rs`, não relacionado a `offset_x`). Não deve mudar com esta
correcção — confirmar que continua exactamente no mesmo valor, não maior
nem menor (se mudar, a separação de causas do P1101 estava errada).

## 5. Não-regressão

Re-rodar P1086-1101 — `equation.rs` já foi tocado várias vezes nesta
investigação (P1090, P1096, P1100). Zero regressão nos casos já fechados.

## Critérios de verificação

1. Os 3 blocos da secção 31 com `applied_offset = margin`, não valor do
   cursor anterior.
2. `MediaBox` width/height convergindo para os valores vanilla exactos
   (±0.0005pt).
3. Offset de ~1.27pt inalterado (confirma separação de causas correcta).
4. `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass
   (confirmar contagem real).

## Critério de conclusão

- Código real de `equation.rs` lido por inteiro antes de mover a captura,
  não só o trecho já citado.
- Os 3 blocos verificados individualmente, não só o agregado da página.
- Offset vertical confirmado inalterado.
