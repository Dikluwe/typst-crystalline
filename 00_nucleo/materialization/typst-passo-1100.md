# L0 — Passo 1100: Corrigir Geometria de `box()`/`layout_external` (Horizontal e Vertical)

**Gate**: `ADR-0127` — mudança de comportamento por defeito (qualquer
`box()`/conteúdo externo com `stroke`/`inset` embutido em equação é
afectado).

**Base**: P1098 (causa horizontal, meia-borda do stroke, `δ=0.25pt` por
caixa, confirmada por progressão aritmética exacta) + P1099 (causa vertical,
`ascent`/`descent` inflado em `scan_external_verticals`, confirmado por
bateria de 5 casos isolando a interacção com `block_chain_active`).

---

## 1. Duas correcções, verificar se são independentes ou uma cascata

Hipótese a confirmar, não presumida: corrigir só `scan_external_verticals`
(vertical) deve fazer o sintoma de `-1.2370pt`/`-2.0870pt` desaparecer nos 5
casos já medidos, **sem precisar de tocar em `block_chain_active`** — o
protocolo de colapso está a funcionar correctamente com um input errado, não
tem bug próprio. Confirmar isto explicitamente depois da correcção 1.2, não
assumir.

### 1.1 Correcção horizontal — `layout_external`

Meia-borda do `stroke` a somar incorrectamente `+0.25pt` (metade de
`stroke: 0.5pt`) por caixa no avanço horizontal. Ler
`01_core/src/compiler/math/layout/mod.rs` (função onde `layout_external` ou
equivalente calcula a largura de um `Content::Boxed`/`Shape` embutido em
equação) antes de corrigir — confirmar o ponto exacto onde a meia-borda
entra a mais, não presumir a partir da descrição.

### 1.2 Correcção vertical — `scan_external_verticals`

`01_core/src/compiler/math/layout/mod.rs:865-880` (já citado). O `ascent`/
`descent` do `MathBox` resultante de um `box()` embutido está a medir a
partir da borda exterior do rectângulo (incluindo `inset`+`stroke`), não do
glifo/conteúdo interno. Corrigir para medir a partir da extensão real do
conteúdo interno, com a borda tratada como decoração visual que não deve
influenciar a métrica vertical usada para posicionamento de linha — mesmo
princípio já usado noutras correcções desta investigação (medir tinta real,
não caixa declarada).

## 2. Verificação — reaproveitar os 5 casos já medidos, não remedir do zero

| Caso | ΔY antes | ΔY alvo pós-correcção |
|---|---|---|
| 1. Eq1 isolada | −2.0870pt | 0.0000pt (±0.0005pt) |
| 2. Eq2 isolada | −2.0870pt | 0.0000pt |
| 3. Heading+Eq1 | −2.0870pt (Eq1); +0.8500pt (heading) | ambos 0.0000pt |
| 4. Heading+Eq2 | −2.0870pt (Eq2); +0.8500pt (heading) | ambos 0.0000pt |
| 5. Documento completo | −1.2370/−2.0870/+1.7000pt | todos 0.0000pt |

Se o caso 5 convergir para zero **sem** tocar em `block_chain_active`,
confirma a hipótese do §1 (cascata, não bug duplo). Se não convergir, o
protocolo de colapso tem mesmo um problema próprio a investigar à parte —
não presumir qual dos dois antes de medir.

## 3. Verificação horizontal

Reaproveitar os casos já medidos (progressão `-0.5000` a `+0.2500pt` em 4
caixas, `-0.3750` a `+0.1250pt` em 3 caixas) — confirmar que convergem para
`0.0000pt` em todos os glifos, não só que a progressão pára de crescer.

## 4. Não regressão

Re-rodar toda a cadeia P1086-1097 (espaçamento de equações) — `box()`
embutido em equação pode interagir com mecanismos já corrigidos nessa
cadeia (protocolo de colapso, `glyph_ink_bounds`, etc.). Zero regressão.

## Critérios de verificação finais

1. Os 5 casos do §2 — `ΔY=0.0000±0.0005pt` em todos.
2. Progressão horizontal do §3 — `ΔX=0.0000±0.0005pt` em todos os glifos.
3. §1 respondido — cascata confirmada ou bug duplo identificado.
4. `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- Código real de `layout_external`/`scan_external_verticals` lido e citado
  antes de editar.
- Hipótese de cascata (§1) confirmada ou refutada com dados, não presumida.
- Todos os casos já medidos nesta investigação (P1098/1099) reconfirmados
  em zero, não "muito melhor".
