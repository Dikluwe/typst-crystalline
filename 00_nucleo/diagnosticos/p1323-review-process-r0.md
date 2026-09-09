# P1323 — A/B inicial bloqueia fechamento

Revisor `/root/p1323_review`, sucessor da revisão RED, sem alteração de
artefatos julgados. O candidato segue o L0: completa somente a constante
privada e emite no mesmo ponto/gate. O receipt unit-GREEN SHA-256
`c0070a3cfce8814e389741a26493e7a51a9a96a76b092cf1fad355e81b3a1ade`
executa o mesmo teste focal com sucesso em `2026-09-08T23:25:05.261230+00:00`.

O A/B `p1323-ab-process-receipt.json`, SHA-256
`c8979502743b3060d2ecf46de37fbc00a7e01e41d2df90daf16ea34aa0d4e61b`,
inicia em `2026-09-08T23:25:44.399223+00:00`, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado
identificado pelo diff/stat de proveniência. O baseline binário tem SHA-256
`756b1c85a5879ea0afb79fc35184525aebfafa6ccb926ae86679a868691f99fa`;
o candidato, `f0b251746274d537c63929ab357b0e84e0e0955c7986d58621d83a0ce3a4e5ee`.

Os quinze casos foram executados nas ordens normal, repetida e inversa.
Em cada ordem, `preserve-pdf` resulta em `preservation=Violated`, exclusivamente
pelo hash do artefato; exit, stdout, stderr residual, sucesso esperado e
presença do arquivo passam. A estabilidade também falha para o PDF do próprio
baseline e do próprio candidato nas duas ordens adicionais. Os demais casos
e todas as verificações do warning passam.

Isso refuta a hipótese de igualdade exata de bytes PDF reproduzível sob o
comando congelado. Não prova regressão de linguagem causada por P1323 nem
prova sua ausência. Metadado variável é hipótese, não causa demonstrada.
Como o runner apagou seu diretório temporário ao terminar, os receipts
preservam hashes/tamanhos, mas os PDFs desta execução não estão disponíveis
para explicar o delta interno.

Veredito: fechamento bloqueado até retificação focal explícita pelo autor
dos testes, preservando suite/receipt originais. É necessário medir a causa,
validar um observável adequado e conservar os artefatos da medição sucessora.
Não converter o resultado vermelho em sucesso porque o processo Python
terminou com exit zero, nem atribuir uma falha presumida somente ao harness.
O estado atual permite afirmar apenas que o warning plain e os demais
controles exercitados passaram; não que todos os gates P1323 passaram.
