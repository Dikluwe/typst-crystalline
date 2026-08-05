# Passo 966 — Relatório PARCIAL (Fase A + A.1; Fase B à espera da confirmação do dono)

**Data**: 2026-08-04
**Estado da árvore**: commit base `552fcbdeb` (P965); só o L0 `_comum.md`
§P966 escrito até aqui — **zero código**.
**Gate**: Fase A.1 do passo + ADR-0127 ponto 3 — **PARADO antes da Fase B**
(mudança na fronteira eval/layout é decisão arquitectural, mesmo que a
implementação final seja pequena).

---

## 1. Fase A — mecanismo mapeado (medição directa, não suposição)

**A.1 — o que os templates produzem** (debug print da árvore real, sonda
temporária removida depois): `#let bra(x) = [⟨#x\|]` invocado como
`bra(phi)` dentro de `$…$` produz

```text
math.sequence([ sequence([text("⟨"), math.text("φ"), text("|")]), … ])
```

ou seja, **`Content::Sequence` de markup** com filhos mistos
`Text`/`MathText`. O `MathText("φ")` já chega correcto da avaliação em
contexto math — só nunca recebe o mapeamento itálico porque
`apply_math_default` (P809/P812, no layout) só recursa em containers
`Math*` nativos.

**A.2 — onde o vanilla aplica o default** (leitura directa):
`ir/resolve.rs:127-146` — `resolve_into_self` chama
`(routines.realize)(RealizationKind::Math, …)`: o output de funções de
utilizador dentro de math é **re-realizado como math** na fase de
resolução, e `resolve_text` trata cada carácter como glifo math com o
default. O vanilla aplica o default ANTES/na expansão do conteúdo de
utilizador; o cristalino aplica DEPOIS, no layout, só a containers
nativamente math.

**A.3 — causa raiz confirmada**: é a diferença de camada (resolve vs
layout). No momento em que `apply_math_default` corre, o conteúdo de
utilizador já perdeu a marcação de contexto (está embrulhado em
`Sequence` de markup, que o recursor não reconhecia).

## 2. Fase A.1 — direção proposta (à confirmação do dono)

**(a) estender `apply_math_default`** — braços novos para
`Content::Sequence` e `Content::Styled` (recursão); folhas transformáveis
continuam só `MathIdent`/`MathText` de 1 carácter; `Content::Text` nunca
transformado (texto literal de markup fica reto — paridade com o
`resolve_text` do vanilla); wrappers `MathStyled` intocados (o `dif`
upright de P962 sobrevive); aninhamento de templates resolvido pela
própria recursão.

**(b) mover o default para o eval** — rejeitada como proposta: contradiz
P812 (o default vive no layout para preservar wrappers `MathStyled`;
regressão P812-A) e tem blast radius muito maior. Registada para memória.

Razão resumida: (a) é contida e casa exactamente com a árvore medida; o
risco de (a) (italicizar o que não devia) é mitigado por só transformar
`MathText`/`MathIdent` e nunca `Text`.

## 3. Estado do gate

- L0 `_comum.md` §P966 escrito com a medição + a decisão proposta.
- **Fases B/C NÃO iniciadas** — à espera de confirmação do dono sobre a
  direcção (a). Com a confirmação, Fase B segue o protocolo de dois
  agentes (P898) com os testes descritos no passo: bra/ket (caso
  motivador), um segundo template de utilizador diferente (generalização),
  guarda fora-de-math, e a revisão do orquestrador com template aninhado.
