# P1339 — classificação estrita de um script histórico

Proposta mecânica, sem crédito de teste: acrescentar em `[excluded_files]`
de `crystalline.toml` somente o path exato
`00_nucleo/diagnosticos/p1339-ab-batch1-prepare-coverage-map.py`.
O arquivo é um rascunho histórico não executável, conservado para demonstrar
o incidente de sintaxe anterior ao sucessor r2. Não alterar seus bytes,
nome, path ou pin no selo original. Não excluir diretórios, outros scripts,
fixtures, consumers produtivos ou regras do linter.

O preflight sob a configuração original abortou antes de V15/V26: recibo
`p1339-array-lineage-preflight-r1.json`, SHA-256
`ac6db4bebdef2771b70c4d792a48164f25c5cecc742f5eb258ff8529de1aa2a8`.
Não foi zero violações. Essa falha continua registrada e não é corrigida
pela classificação do artefato como histórico.

O verificador confirmou o papel não executável no ledger original e a
exclusão exata antes do parsing no walker do linter. Após seu aceite,
registrar os hashes da configuração antes/depois e repetir o gate com a
configuração nova identificada. Manter a verificação dos inputs congelados,
incluindo o script histórico SHA-256
`8ede124840a8ed862f219f25c4c78290e56f25be11e6a7871944ca648469ac0c`.

Esta proposta não modifica o contrato semântico, expectativas nem orçamento
discriminatório. Não atribui PASS geral ao P1339.
