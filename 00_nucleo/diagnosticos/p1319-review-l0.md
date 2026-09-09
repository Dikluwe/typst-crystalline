# P1319 — revisão do L0 e medição, antes do candidato

Revisor `/root/p1319_review`, regime A/B sem atestação técnica de isolamento.
Continua o registro de autoridades e baseline de `p1319-review-preliminary.md`.
Nenhum produto/oráculo julgado foi editado pelo revisor.

## Entradas e verificações

L0 integral vigente anterior foi lido na revisão preliminar; nesta revisão
foi lido todo o delta P1319, localizado em §5. SHA-256 raw novo confirmado:
`5060ce50ec107b7921bea31e18630bda3138b0e15eaa2a14285a9073c973e318`.
Medição `p1319-measurement.json`, SHA-256 confirmado
`4396b098d5f5db30bfd832d210c1027eb9529d8120c18c5a63fd9bc16d8ae3bd`.
Todas as 52 observações foram lidas integralmente. Proveniência dessas
observações: HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, diff/stat
produtivo vazio, intervalo UTC 2026-09-08T15:56:51.876283 a
15:56:54.590275, executáveis, argv/cwd e bytes integrais no próprio recibo.

Preflight `p1319-lineage-preflight.json`, SHA-256 confirmado
`c9a4f4e36a9e7a11fd44f968d2a5dfb66b26a7c4995a5c088e642c6242cb935d`,
registra `crystalline-lint --checks v15,v26 --fail-on warning .`, exit 0.
Esse preflight antecede o novo L0 e valida o baseline: não é verificação de
linhagem do candidato nem resselo do L0 P1319.

## Achados e veredito desta fase

As observações de `invalid-after-earlier-ls` confirmam seleção binária pelo
buffer integral, preservação de UnequalLengths vencedor, caminho na causa,
span causal direto/Path/With/Args. As de `crlf-between` confirmam que texto
válido pede range externo, cuja ausência cristalina permanece fora deste
lote. As observações detached e excesso demonstram diagnósticos de estratos
diferentes no vanilla e justificam expectativas normativas próprias. I/O
ausente é controle de preservação, não paridade corrigida.

O L0 incorpora essas fronteiras sem ampliar casts, World, fase ou assinatura.
Prescreve Project/Package pela fonte ratificada; a medição root cobre Project,
portanto Package precisa dos controles locais e/ou medição independente já
exigidos na aceitação. Não há alegação prematura de paridade medida Package.
A ordem de classificação UTF-8 após erro e a preservação de texto válido
estão explícitas. Nenhum atalho include_path/source está autorizado.

Sem objeção normativa à classe fluxo contínuo ADR-0127 para esse recorte.
Não se exige nova confirmação humana para corrigir internamente os
diagnósticos delimitados. A raiz causal continua sendo o L0 proprietário.

Precisão documental menor comunicada ao root: a referência de consumer
baseline `loading.rs:1375-1376` deveria cobrir `1374-1375` (leitura+decode).
Isso não muda a conclusão nem configura bloqueio semântico.

Estado: L0/medição adequados; parecer pré-patch definitivo ainda depende da
inspeção dos deltas de testes, falhas RED e congelamento A/B. Este documento
não avalia implementação candidata e não é autorização para saltar tais gates.
