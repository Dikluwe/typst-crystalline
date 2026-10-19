# L0 — Passo 1104: Fechar as Duas Pendências da Secção 31 (Protocolo de Parágrafo + Kerning de `item_width`)

**Gate**: `ADR-0127` — mudança de comportamento por defeito (afecta todo
documento com heading seguido de parágrafo, e todo texto medido por
`item_width`).

**Base**: P1102 (largura, já corrigido) + P1103 (altura, corrigido
parcialmente) + investigação desta troca (causa do resíduo de `1.25pt`:
`extra_below` em `heading.rs` é paliativo para o facto de `Content::Text`
nunca ter sido incluído no protocolo de colapso; causa do resíduo de
`0.91pt` no Caso 1: `item_width`, já diagnosticado no P1098, nunca
corrigido). Este passo fecha as duas, sem deixar pendência.

---

## 1. Correcção 1 — Parágrafo entra no protocolo de colapso, sem alargar o filtro do `Sequence`

**Decisão de implementação, não negociável sem motivo novo**: não
adicionar `Content::Text` à lista de tipos preservados em
`sequence.rs` (isso preservaria estado indefinidamente através de
qualquer texto, não só o primeiro depois de um bloco — efeito colateral
amplo, não pretendido).

Em vez disso, consumo pontual no ponto de entrada do parágrafo
(`layout_word`/`ensure_initial_baseline`, ou onde o primeiro item de texto
de um parágrafo é processado — confirmar o nome real da função antes de
editar):

- Se `layouter.block_chain_active == true` e
  `layouter.prev_block_below_pending > 0.0` no momento em que o **primeiro**
  texto do parágrafo é emitido: `cursor_y = prev_line_baseline +
  prev_block_below_pending + par.top_edge` (fórmula já derivada e
  confirmada: `9.0552+8.5800=17.6352`, bate com o avanço real medido).
- Consumir (zerar) `prev_block_below_pending` e `block_chain_active` nesse
  momento — só a primeira palavra do parágrafo lê o valor pendente; texto
  subsequente na mesma linha/parágrafo não deve reagir a isto de novo.

**Remover `extra_below` de `heading.rs`** — o heading passa a encerrar
sempre com `prev_block_below_pending = below_pt` e `block_chain_active =
true`, sem a manipulação ad-hoc de `cursor_y` que já existia para os outros
casos (bloco, equação, heading seguinte). Isto unifica heading→parágrafo
com os casos já correctos (heading→bloco, heading→heading).

## 2. Correcção 2 — `item_width` deve usar extensão real, não soma nominal

`helpers.rs` (`item_width`, já citado no P1098): substituir a soma de
`x_advance` nominais por glifo pela extensão real pós-shaping (a mesma
distinção "nominal vs bounding-box real" já resolvida noutros mecanismos
desta investigação — `glyph_ink_bounds` em vez de `cap_height`, P1086;
`text_ink_bounds_signed` em vez de unsigned, P1088). Confirmar se o shaper
já expõe a largura real do run inteiro (não just soma de avanços
individuais) antes de decidir a forma exacta da correcção — pode já existir
um mecanismo equivalente usado noutro lugar (`FrameItem::TextShaped`
provavelmente já carrega essa informação do HarfBuzz).

## 3. Verificação — ambas as correcções, nos casos já medidos

| Métrica | Alvo |
|---|---|
| Gap heading→parágrafo (texto puro e com math inline) | `15.7630pt`, ±0.0005pt |
| `MediaBox Height` secção 31 | `263.8196pt`, ±0.0005pt |
| `MediaBox Width` Caso 1 (bloco como último elemento) | convergir, não `+0.9131pt` |
| `ΔX` do bloco no Caso 1 | `0.0000pt`, não `+0.4565pt` |
| Casos A/B do controlo (texto puro vs math inline) | idênticos entre si e ao vanilla |

## 4. Não-regressão — obrigatória, ampla

Esta correcção toca `heading.rs` (já editado em P1063, P1074, P1078,
P1090) e `helpers.rs` (usado por múltiplos consumidores per P1098/P1101).
Re-rodar **toda** a cadeia P1057-1103, não só os casos da secção 31 — risco
de regressão real dado o número de passos anteriores que já tocaram estes
dois arquivos.

## 5. Não fechar com "muito melhor" — critério estrito

Mesma disciplina de toda esta investigação: todos os valores do §3 dentro
de `±0.0005pt`. Se algum não convergir totalmente, não apresentar como
"resolvido" — reportar o resíduo com a mesma causa raiz exigida em todos os
passos anteriores (código citado, não descrição).

## Critério de conclusão

- `extra_below` removido de `heading.rs`, substituído pelo protocolo
  genérico consumido no ponto de entrada do parágrafo.
- `item_width` corrigido para extensão real, código citado.
- §3 — todos os 5 itens convergindo, não parcialmente.
- §4 — cadeia P1057-1103 inteira revalidada, não só os casos novos.
- Nenhuma pendência nova aberta ao final deste passo.
