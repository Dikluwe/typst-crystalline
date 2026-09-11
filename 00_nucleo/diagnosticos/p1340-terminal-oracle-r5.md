# P1340 — recibo do oráculo terminal R5

Autor independente substituto: `/root/p1340_terminal_oracle_r5b`. Regime:
`executado sem atestação de isolamento`. O worktree é compartilhado, portanto
não há alegação de isolamento por sistema operacional; a allowlist declarada
foi respeitada. Baseline R5, outputs/adaptadores candidatos, `01_core`,
`03_infra`, `materialization/` e `context/` não foram lidos.

## Harness sucessor

O novo `p1340-contract-terminal-harness-r5.rs` tem SHA-256
`cff1314502b1b113f86cd335c423a65b6772eedb3cdabe45bc21a4eff676a6a6`.
A transformação mecânica encontrou cada um dos três literals exatamente uma
vez e adicionou quatro prefixos `let _ =`. O tamanho passou de 7912 para 7944
bytes. O conteúdo coincide integralmente com a transformação requerida; a
inversão recupera o R2 pinado, SHA-256
`85c3f151cbf1e88c241acea0b68df1f6c468ba0a1a956e5a3f6af91cd0a19e7d`.
Nenhum assertion, expectativa, sink, carrier, ordem de casos ou fonte T04–T06
mudou.

## Focais vanilla

Binário: `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97f62785f78284c63ff7b8`,
referência ratificada `a51e02804`. A string `--version` foi registrada apenas
como informação não confiável para identidade do pin. Foram executadas duas
iterações dos mesmos três focais — a segunda apenas para separar stdout e
stderr —, zero execuções candidatas e zero suítes terminais completas.

- T01, fonte `a3da3e…cef8`: exit 0, stdout/stderr vazios.
- T02, fonte `4b4c28…8a0c`: exit 0, stdout vazio; stderr de 559 bytes,
  SHA `6356f6…6dd8`, contendo somente o warning permitido de não convergência
  e a sequência observada 0,1,2,3,4/final 5.
- T03, fonte `9f9672…b6eb`: exit 0, stdout/stderr vazios.

Os comandos, UTCs, hashes/tamanhos dos canais e PDFs das duas iterações estão
no recibo JSON irmão. A medição demonstra apenas que o erro alheio de join
Array/Content desapareceu e que as fontes sucessoras são válidas no vanilla.
Não demonstra carrier opaco, reachability produtiva, política terminal, sinks
ou equivalência funcional geral.

## Estado de gate

`Unknown` permanece para comportamento opaco, ligação ao caminho produtivo e
resultado terminal. O verificador independente deve conferir eagerness/ordem,
full-byte preservation e a ligação prospectiva antes de qualquer selo. Este
autor não emite selo nem veredito sobre candidato.
