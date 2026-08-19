# L0 — Passo 1088: Fechar os Dois Resíduos do P1087

**Gate**: `ADR-0127` — mudança de comportamento por defeito.

**Base**: P1087 corrigiu `BLOCK_SPACING` e `glyph_ink_bounds`, mas deixou dois
resíduos não resolvidos, apresentados como "aprovados" sem estarem em
paridade exacta:
1. Cabeçalho→Eq1: +1.829pt (era +1.96pt) — redução de só 7%.
2. Eq2→Eq3: +0.0776pt (era −2.02pt) — muito melhor, mas não zero.

**Nenhum dos dois é ruído aceitável.** Ambos têm causa a encontrar, não a
tolerar.

---

## 1. Resíduo 1 — Cabeçalho→Eq1 (+1.829pt)

### 1.1 Hipótese principal, já levantada no L0 do P1087 e nunca resolvida

`layout_equation` não participa de `block_chain_active`/
`prev_block_below_pending` (protocolo do P1061, estendido a heading no
P1063). O heading fecha com `prev_block_below_pending = 8.25pt` e
`block_chain_active = true` (P1063) — se `layout_equation` não lê esse
estado, o espaço acima da equação é aditivo puro (`spacing + ascent`), nunca
colapsando contra o `below` pendente do heading. Isto explica um resíduo
positivo (falta de colapso = espaço a mais), consistente com o sinal
observado.

### 1.2 Mecanismo

Ler `01_core/src/compiler/layout/equation.rs` (entrada do layout, onde o
`spacing` acima é aplicado — linha ~119, já editada no P1087, mas só o valor,
não o protocolo). Aplicar o mesmo padrão de `heading.rs` (P1063) e `block.rs`
(P1061): antes de avançar `cursor_y`, calcular `gap` via
`self.block_chain_active` + `self.prev_block_below_pending.max(spacing_pt)`,
não somar `spacing` incondicionalmente.

Confirmar também o lado de saída — `layout_equation` precisa de **registar**
`prev_block_below_pending`/`block_chain_active` ao terminar, para que o
elemento seguinte (parágrafo, outro heading, outra equação) possa colapsar
contra ela. Sem isto, mesmo corrigindo a entrada, a transição Eq→algo-a-seguir
continuaria quebrada.

### 1.3 `Content::Sequence` — terceira peça, já vista nesta conversa

P1061 acrescentou `Content::Parbreak` à lista de tipos preservados pelo
`Sequence` consumer; P1063 acrescentou `Content::Heading`. Confirmar se
`Content::Equation` também precisa de entrar nessa lista — mesmo padrão,
mesmo arquivo (`sequence.rs`), não presumir que já está coberto.

## 2. Resíduo 2 — Eq2→Eq3 (+0.0776pt), causa real, não aproximação

Não aceitar como "resto de arredondamento". 0.0776pt é maior que o ruído
sub-pixel já estabelecido nesta conversa como baseline normal (~0.0005pt,
visto repetidamente em P1057-1087). É ~150x maior que esse ruído — precisa de
causa própria.

### 2.1 Hipóteses a testar, não escolher uma sem verificar

- Eq3 (`∇f(x*)+...=0`) é a equação com o glifo mais alto/complexo da amostra
  — se `glyph_ink_bounds` tiver alguma imprecisão específica a glifos
  compostos (acentos, operadores grandes com sub/sobrescrito simultâneo),
  isso apareceria justamente aqui e não nas outras transições.
- Verificar se `glyph_ink_bounds` (P952b) já foi ele próprio validado com
  precisão sub-pixel nalgum passo anterior, ou se esse é o primeiro uso real
  em produção que expõe uma imprecisão nunca antes medida.
- Confirmar se o resíduo muda de sinal ou magnitude se o teste for repetido
  com uma equação diferente de complexidade semelhante a Eq3 — se for
  específico a este glifo, aponta para o mecanismo de ink bounds; se for
  genérico a "qualquer terceira equação", pode ser outra coisa (acumulação de
  estado ao longo da sequência).

### 2.2 Medir antes de corrigir

Isolar Eq3 sozinha (sem Eq1/Eq2 antes) contra o vanilla — se a divergência
persistir isolada, é `glyph_ink_bounds` para aquele glifo específico; se só
aparece em sequência, é acumulação de estado (candidato: mesmo resíduo do
Resíduo 1, propagando erro residual adiante).

## 3. Não fechar este passo com números "muito melhores mas não exactos"

Critério de aceitação: os 3 deltas do P1086 chegam a
**0.0000pt ± 0.0005pt** (a mesma tolerância sub-pixel já usada em toda esta
conversa desde P1057), não "melhor que antes".

## Critério de conclusão

- Resíduo 1: protocolo de colapso integrado em `layout_equation` (entrada e
  saída), `Content::Equation` confirmado ou adicionado ao `Sequence`
  consumer.
- Resíduo 2: causa real identificada (não "arredondamento"), com o teste de
  isolamento do §2.2 executado.
- Os 3 deltas dentro de ±0.0005pt, não "muito reduzido".
- Re-rodar P1059-1063 e P1087 — zero regressão.
- `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.
