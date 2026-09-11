# P1339 — parecer final sobre a minuta de payload

Verificação documental em **2026-09-10T00:29:09Z**, HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, sem diff produtivo. Regime
executado sem atestação de isolamento. Não houve novo probe, teste,
implementação ou edição de L0; R1 permanece intacta.

Entrada confirmada por `sha256sum`:

```text
3e5071128e9a6c42ba7b75abf88659d33e6e712b0614e7c2b88ff228a4a66628  00_nucleo/prompts/entities/element_payload.md
```

O diff/stat documental observado foi:

```text
 .../prompts/compiler/eval/bindings/field_access.md | 43 +++++++++++
 .../compiler/eval/bindings/value_methods.md        | 89 ++++++++++++++++++++++
 00_nucleo/prompts/compiler/eval/call_dispatch.md   | 53 +++++++++++++
 .../prompts/compiler/eval/operators/equality.md    | 44 +++++++++++
 00_nucleo/prompts/compiler/eval/repr.md             | 38 +++++++++
 00_nucleo/prompts/compiler/eval/rules.md            | 34 +++++++++
 .../prompts/compiler/eval/selector_matching.md     | 59 ++++++++++++++
 .../compiler/stdlib/foundations/selector.md         | 35 +++++++++
 00_nucleo/prompts/entities/element_payload.md       | 86 +++++++++++++++++++++
 00_nucleo/prompts/entities/selector.md              | 82 ++++++++++++++++++++
 00_nucleo/prompts/entities/show.md                  | 48 ++++++++++++
 11 files changed, 611 insertions(+)
```

## Constatações anteriores ao parecer

- `element_payload.md:328-334` agora identifica Content::Strong e
  Content::Emph próprios como produtores com unidade de ocorrência conhecida.
  A promoção de Styled depende explicitamente de medição e L0 proprietário
  fixando cardinalidade, ordem, parent e ausência de dupla alocação; flags
  isoladas não autorizam novas Locations. Resolve a ressalva de origem da R1.
- `:335-336` exclui promoção implícita de outros Content, Text, elementos
  dinâmicos e tipos futuros. A variante unit não vira catch-all.
- `:338-350` conserva a opção sem novo ElementKind, sem identidade por nome
  ou hash, usando Content/snapshot e os carriers de Tag/Location existentes.
- `:368-370` reconhece que o hash do Content nu não prova estabilidade de
  campos derivados apenas da chain, exige medição pelo owner de captura e
  rejeita duplicar o mesmo hash como reparo. Resolve a ressalva causal da R1.
- `:354-358` mantém o gate público adicional e a exigência de L0s dos consumers
  antes de código. `:360-371` delimita aceitação futura e não declara paridade
  geral ou encerramento do passo.

## Veredito limitado

**APRESENTÁVEL AO GATE ADR-0127.** As duas ressalvas documentais da revisão
anterior foram resolvidas na entrada de SHA-256 acima. Não há finding novo
que impeça apresentar ao dono esta proposta de uma variante unit adicional.

Este parecer valida a precisão e os limites da proposta pública. Não é a
aprovação humana, não autoriza implementação por si só e não é PASS de
query/counter, sincronização de Locations, convergência ou origens Styled.
Essas obrigações permanecem nos owners e gates explicitamente exigidos pela
minuta.
