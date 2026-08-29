# P1250B — fechar quatro regressões do workspace após o saneamento

**Estado:** FECHADO — PASS LIMITADO AO FRAGMENTO  
**Predecessor:** P1250A  
**Escopo:** uma regressão de `color.mix(..., space: rgb)` e três expectativas
obsoletas de fechamento suave de `curve`.

## Baseline congelado

- HEAD: `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`;
- working tree não commitado;
- `cargo test --workspace`: 5291/5295 testes `typst-core` passam;
- falhas: `p744_space_nomeado_mix_negate_rotate`,
  `p293_curve_move_line_basico`, `p294_curve_quadratic_e_cubic_misturados` e
  `p513_curve_aceita_content_curve_como_argumento`.

## Medição antes da decisão

1. `p744` esperava `#805a88`, conforme
   `00_nucleo/prompts/entities/color.md`; a implementação devolve `#805b87`.
   As constantes P1253 são os literais `f32` do vanilla e não podem voltar a
   ser reconstruídas por bytes. A hipótese a testar é ordem aritmética da
   interpolação do `palette`, não mudança das constantes nem do oracle.
2. Os três testes de curve esperam `Move/Line-or-Cubic/Close`. O L0 vigente de
   `stdlib/shapes` determina que `close()` é `CloseMode::Smooth` por defeito e
   adiciona o cúbico de fechamento antes de `ClosePath`. A implementação atual
   produz exatamente um item extra; os testes antigos ainda afirmam a forma
   anterior a P1226.

## Obrigações

- `mix` sRGB deve reproduzir `#805a88` sem regredir pesos 0, 0.25, 0.5, 0.75 e
  1 nem os demais espaços;
- `curve.close()` suave deve produzir o cúbico de fechamento e `ClosePath`;
- `curve.close(mode: "straight")` deve continuar sem cúbico adicional;
- atualizar somente oráculos comprovadamente obsoletos; não mascarar perda de
  segmento, bbox ou modo de fechamento;
- preservar L0, contratos públicos, defaults e fases do pipeline.

## Protocolo e gates

Protocolo completo Tekt, com contrato, ataques, implementação e verificação
segregados em capacidades, mas sem atestação de isolamento forte por workspace
compartilhado.

Gates: testes focais RED→GREEN, suíte de color/curve, `cargo test --workspace`,
`cargo build --workspace`, V1/V5/V15/V26, lint diferencial P1250A e
`git diff --check`.

## Resultado

- Vanilla pinado medido diretamente: `red.mix(blue, space: rgb)` = `#805b87`;
  L0 e oracle P744 retificados, sem mudar a implementação correta P1253.
- Tuple legado `("close",)` restaurado para fechamento reto.
- `curve.close()` namespaced preserva default suave; P513 agora verifica o
  cúbico, seus controles, endpoint e `ClosePath`.
- Mutações `tuple→Smooth`, `namespaced→Straight` e `P744→#805a88` são
  rejeitadas pelos testes focais.
- `cargo test --workspace`, build, V1/V5/V15/V26, lint completo e diff check:
  PASS.

Veredito segregado: PASS limitado ao fragmento. A árvore e o contexto são
compartilhados, portanto não há atestação forte de isolamento nem alegação de
equivalência geral de cor, curve ou Typst.
