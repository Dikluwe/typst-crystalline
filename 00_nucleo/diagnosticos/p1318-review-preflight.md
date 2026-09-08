# P1318 — revisão independente de preflight

Veredito: GO de classe e escopo; ainda não é GO pré-patch nem aprovação final.

Revisor `/root/p1318_review`, com acesso de leitura aos materiais julgados e
escrita declarada somente em `00_nucleo/diagnosticos/p1318-review-*`.
Skill `tekt-materializacao-segregada` e ambas as referências lidas, assim como
ADRs 0107, 0108, 0127 e 0129. Não foi encontrada ADR local de segregação.
Regime A/B executado sem atestação de isolamento; nenhum selo de refinamento
ou equivalência geral. O revisor não altera L0, solução ou oráculos.

Medição anterior à classificação: HEAD
`bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, working tree não commitado.
Snapshot `p1318-measurement.json` SHA-256
`c19f749ad22b06e07d3078d6d9a72017e3c2e1f2b2546f738042f34362302378`,
instantes `2026-09-08T15:22:15.186161+00:00` a
`2026-09-08T15:22:23.277969+00:00`, contém diff/stat, fontes integrais e
70 execuções (35 casos bilaterais). Os hashes before/after coincidem.
Todos os artefatos P1315/P1316/P1317 registrados permanecem com os hashes
originais. Código atual ainda coincide integralmente com o snapshot,
SHA-256 `39cf6ef6c3cd55872dfc40cbe0787faab7379013beb3e399183d8cc97d22d86c`.
L0 revisado integralmente SHA-256
`ee3e5a74aaad1a9add4ae16d4b9147634ad95cb9eee3fdfc624b720d93eb7f3e`.

Fonte conferida: `loading.rs:944-1006,1304-1319` descarta posição e
conserva somente origem causal em Bytes. Vanilla ratificado `a51e02804`,
`loading/csv.rs:138-157` fornece offset do parser ou fallback ordinal/1;
`diag.rs:845-925,1025-1035` seleciona texto/binário pelo buffer inteiro;
`typst-syntax/src/lines.rs:88-95,252-274` e `lexer.rs:1144-1152` definem
chars, conjunto de newlines e CRLF. A medição efetiva distingue CRLF válido
`at 1:5` de inválido `at 1:1`, ordinal de registro de linha física, e
UnequalLengths anterior ao byte inválido. A sonda path da medição só prova
I/O/resolução virtual; não prova parsing Path.

Classificação: correção interna de paridade diagnóstica no fluxo contínuo
ADR-0127. A mensagem de erro é observável de linguagem (ADR-0108); não há
nova API pública, tipo, fase, modo ou dependência. A delegação privada no
mesmo owner preserva a bijeção ADR-0129. Só csv(Bytes) ganha sufixo;
decode_csv público e Path/Str retêm mensagens, valores e origens legadas.
O efeito em parsing com excesso está explicitamente normatizado e separado
de paridade com o erro de excesso vanilla. Nenhuma mudança de precedência,
ordinal, causa, span, hints ou traces foi autorizada.

Condições do GO pré-patch: RED local e freeze independente anteriores ao
candidato; manifesto explícito do delta dos testes anteriores contra o
snapshot (somente expectativas Bytes com o novo sufixo, sem retirar casos,
asserções de origem/causa ou controles); produção anterior idêntica salvo
metadata de linhagem. O fallback sem offset e offsets impossíveis devem
ficar seguros e sem origem fabricada. Gates finais ainda pendentes.
