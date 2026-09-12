# P1355 — contrato congelado da ressincronização vanilla

## Intenção e fronteira

Substituir exclusivamente a autoridade de paridade upstream/main de
`a51e028041cac426f97d34335bb01d8f1d8e5e8f` por
`586e1bd43fae6c9a973218163d3165c53ab8d16d`, sem corrigir no mesmo trabalho
as novas divergências observadas e sem reescrever recibos históricos.

O alvo novo tem árvore `2847920eeaa98d26a41dc6fb0f9c02334b9d593b`. O delta é
ancestral e contém 35 commits, 199 paths upstream, 5.285 inserções e 2.552
remoções. O patch `git diff --binary --full-index` tem SHA-256
`3ebad68debae9b15347c7d9b2d4b8b2857655cee13919269660868007cab8250`.

## Observáveis obrigatórios

1. O lab deve reproduzir a árvore nova, admitindo somente o overlay local
   `crates/p1140-inventory/**` e sua entrada em `Cargo.lock`.
2. O vanilla deve ser construído com o SHA completo em `TYPST_COMMIT_SHA`.
3. `lab/typst-original/target/release/typst` e `/usr/local/bin/typst` devem
   ser byte-idênticos e ter SHA-256 medido depois do build.
4. `sys.version` deve continuar `version(0, 15, 1)`; a revisão não é inferida
   dessa versão.
5. Pins vivos de workflow, matriz e inventário devem apontar ao novo alvo;
   medições e contratos congelados de passos anteriores permanecem no pin antigo.
6. A matriz geral e o inventário devem ser reexecutados; incapacidade do
   comparador continua `Unknown` e nunca conta como paridade.

Sentinelas focais cobrem a nova superfície `format`, `luma()` obrigatório,
diagnóstico de `array.to-dict`, `columns.separator`, metadata HTML por autor,
CLI `-i`/`--no-fullscreen`/opções de formato, pontuação CJK, expansão vertical
com tags e fronteiras de duração.

## Política de Unknown

Associação ambígua entre fonte e binário, feature/fonte/ferramenta ausente,
timeout, parser sem representação do novo observável ou reutilização de expected
do pin antigo produz `Unknown`. Ausência pública comprovada é divergência, não
`Unknown`. Apenas fixtures deliberadamente opacas podem esperar `Unknown`.

## Segregação e budget

- contrato: agente `/root/contrato_resync`, somente leitura, sem patch candidato;
- adversário/oráculo: agente `/root/adversario_resync`, somente leitura, sem patch;
- auditor do mecanismo: agente `/root/mecanismo_resync`, somente leitura;
- implementação e integração: `/root`, depois do congelamento acima;
- veredito: verificação determinística e revisão adversarial posterior ao patch.

Budget: uma sincronização integral, uma revisão focal se algum gate falhar e uma
execução completa final. Duas revisões sem ganho obrigariam redesenho, nunca
relaxamento do contrato.

A separação foi aplicada por capacidade e ordem, mas não é tecnicamente atestada
por sandboxes de filesystem distintos; o veredito deve usar a formulação
“executado sem atestação de isolamento”.
