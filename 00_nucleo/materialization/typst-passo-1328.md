# Passo 1328 — diagnóstico de calc.abs quando o argumento é conteúdo

## Problema e decisão

Próximo item elegível da fila P1322: `calc-abs-content-diagnostic`.
P1327 encerrou bare-import-warning. A pendência de identidade math de P1325
continua fora; este recorte não identifica módulos por nome nem muda math.

Medição nova: `00_nucleo/diagnosticos/p1328-baseline.json`, SHA-256
`e049418db46ea039235b336254bfdbffd492253ac66c38782ac6bc56564f2ffb`,
com HEAD, working tree não commitado, diff/stat, horários e binários fixados.
`$std.calc.abs(-1)$`, a rota por alias em math e `calc.abs([abc])`
falham bilateralmente. No cristalino, a mensagem diverge e perde o argumento;
o vanilla ratificado upstream/main `a51e02804` aponta o valor e informa
`expected integer, float, length, angle, ratio, fraction, or decimal, found content`.

O owner `01_core/src/compiler/stdlib/calc.rs:139-148` usa erro detached,
embora Args já transporte a origem causal do valor. Corrigir somente
conteúdo, preservando a precedência dos guards e os demais tipos/valores.
O contrato permanente pertence a `00_nucleo/prompts/compiler/stdlib/calc.md`,
não a este passo. Classificação ADR-0127: paridade diagnóstica contínua.

## Execução

1. Atualizar L0 antes de Rust e ressellar somente o par calc.
2. Congelar testes A/B independentes, no próprio owner, e corpus CLI completo
   antes do candidato. Classificar controles de dívida antes de implementar.
3. Executar RED sem falha de harness/build; integrar a correção local e exigir
   GREEN dos mesmos testes, sem adaptar expectativas ao candidato.
4. Validar build release/locked, workspace, fmt, lint geral e V5/V15/V26,
   linhagem recíproca e comparação CLI normal/repetida/invertida.
5. Registrar resultado substantivo, causa e dívidas em
   `00_nucleo/diagnosticos/p1328-final-report.md`, revisão e closure.

## Limites

Não aceitar conteúdo como número nem mover avaliação de math. Sem novos
tipos aceitos, alteração de Int mínimo, outras funções calc, named/aridade,
helpers globais, APIs, defaults, pipeline ou CLI. Conteúdo localizado é o
mesmo tipo da linguagem; Symbol não é. Origem sintética ausente permanece
detached, nunca deduzida pelo valor ou substituída pelo agregado.
Os caminhos com origem real são obrigatórios; Unknown bloqueia. Necessidade
de outro owner exige reabrir o escopo, nunca enfraquecer o oracle.

Regime Tekt A/B sem atestação técnica de isolamento, sem selo de refinamento.
Root escreve L0/candidato e integra testes congelados; autor independente
recebe L0/interfaces públicas, não o runtime candidato; revisor não edita
produto/oráculos. Até duas revisões sem ganho por causa antes de rever método.
Preservar todas as alterações/evidências anteriores. Temporários dedicados
em `/tmp/p1328-target.T7Tg57`, cópia sem hardlinks. Sem stage/commit/push e
sem iniciar o passo seguinte.
