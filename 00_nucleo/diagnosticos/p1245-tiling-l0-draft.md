# P1245 — proposta transposta aos L0 proprietários

Este documento é diagnóstico. A proposta foi transposta aos Prompts L0
proprietários, mas ainda aguarda confirmação explícita dos hashes pelo dono;
este recibo não legitima código.

## Owner `entities/tiling`

- transportar `Content` fechado e declarativo até layout, mantendo identidade
  do valor `tiling` e usando apenas indireção física permitida quando necessária;
- modelar `size:auto`, spacing, offset relativo, angle e relative
  auto/self/parent com defaults compatíveis com o vanilla ratificado;
- separar o valor público não resolvido do tile resolvido, sem armazenar I/O ou
  depender de L3 em L1;
- proibir fallback de conteúdo para cor como alegação de paridade.

## Owner `stdlib/tiling`

- aceitar Content posicional arbitrário e named args size, spacing, offset,
  angle e relative;
- validar finitude, proibir unidades font-relative onde vanilla proíbe e manter
  `tiling(t) == t`;
- não materializar frame durante eval.

## Owner futuro da materialização de layout

- resolver o corpo uma vez na fase layout;
- derivar size auto do frame, conservar size explícito, spacing e clipping;
- resolver offset percentual contra pitch e aplicar offset antes de angle;
- manter a representação resolvida privada à fase; não criar um segundo
  contrato público `ResolvedTiling`.

## Owners dos exporters

- SVG emite pattern a partir do tile resolvido e preserva fill/stroke;
- outros targets preservam a mesma morfologia ou declaram Unknown explícito;
- bytes, IDs e ordem de defs não são critérios, salvo observável equivalente.

## Gate

ADR-0127: decisão explícita ainda não recebida; aguardar confirmação dos novos hashes
antes de P1254. ADR-0129: o futuro consumer de layout precisa de Prompt L0
proprietário criado junto ao consumer; extrair Núcleo Tekt somente para claims
genuinamente compartilhadas por dois ou mais owners.
