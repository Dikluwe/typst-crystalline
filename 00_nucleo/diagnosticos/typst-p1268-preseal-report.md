# P1268 — relatório de execução e preseal V3

**Estado:** PRESEAL EMITIDO; SELO FINAL NÃO EMITIDO  
**Medição final:** 2026-08-28T23:40:28-03:00  
**HEAD observado:** `7df0e3174d0d651189c9281c81c4508e3c691478`  
**Baseline vanilla:** upstream/main `a51e02804`, binário `/usr/local/bin/typst` pinado por SHA-256 no manifesto  
**Escopo:** contrato, oráculos vanilla-first, ataques e preseal; sem alteração de produto, L0 ou testes produtivos

## Veredito

O contrato corrigido V3 foi congelado e o verificador independente emitiu o
preseal `P1268-VERIFIER-V3-PRESEAL`. O resultado vale apenas para o fragmento
observável congelado pelo P1268; não é alegação de equivalência funcional geral
nem selo final de um candidato.

O score mutacional foi `24/24 = 1.0`: os 24 mutantes negativos válidos foram
classificados como `Violated`, o controle positivo foi preservado e os 10 casos
explicitamente opacos permaneceram `Unknown`. Nenhum mutante negativo necessário
foi aceito como `Unknown`.

## Medições fechadas

| Gate | Resultado |
|---|---:|
| população positiva | 96/96; 24 por par |
| oráculos positivos disponíveis | 96/96; `Unknown` necessário = 0 |
| `M/R/E` | 1320 / 848 / 11188 |
| intervalos | 1224 total; 848 elegíveis; 376 inelegíveis |
| máximos `s/a/d/b/q` | 64 / 31 / 63 / 62 / 125 |
| limites `s/a/d/b/q` | 64 / 63 / 127 / 126 / 253 |
| máscaras | 96 manifestos; 192 SVG/PNG rehashados |
| coincidências | 180; zero `Unknown` exact-point/epsilon-right |
| degenerados S20/S21 | 4/4 e 4/4 |
| determinismo | 192/192 semântico; 96/96 tripletos SVG byte-idênticos |
| ataques | 24/24 negativos mortos; 1/1 controle preservado |
| opacos declarados | 10/10 `Unknown`; nunca contados como sucesso |

Os budgets foram derivados da execução vanilla atual antes da comparação
histórica. A comparação pós-freeze reproduziu 94/96 linhas históricas; somente
`S13-linear-linear-rgb` e `S13-radial-linear-rgb` diferiram, sem adaptar o budget
novo ao histórico ou à implementação candidata.

## Refinamentos selados

O contrato V3 corrige os quatro pontos identificados pela readjudicação P1267-R2:

1. separa o ponto coincidente exato da sonda epsilon-right;
2. distingue os limites de 127 decisões de midpoint, 126 subdivisões e 253
   chamadas totais ao sampler;
3. define `M` como a contagem de stops públicos efetivos;
4. preserva `logical=904` como contagem histórica distinta e corrige o agregado
   para `M=1320`, `E=11188` e bound afim `78432`.

## Segregação e capacidade de leitura

Os papéis de autor do contrato, oráculo, adversário e verificador foram executados
com contexto conversacional não herdado e allowlists processuais distintas. Isso
não constitui atestação técnica de isolamento: todos os papéis continuaram com a
capacidade física de leitura concedida pelo mesmo filesystem compartilhado. O
resultado é, portanto, **segregação processual sem atestação técnica de isolamento
do filesystem**.

Duas tentativas foram excluídas antes do V3: a primeira abriu dois diagnósticos
fora da allowlist; a segunda enumerou paths e pesquisou ADRs além da capacidade
textual então congelada. Uma execução tecnicamente verde também ficou sem preseal
porque escreveu em `oracle_clean` quando a raiz autorizada era `oracle`. Nenhuma
dessas tentativas entrou no denominador do score final.

## Deriva de HEAD

O HEAD mudou durante a execução de
`697eaf31e8ce6aaa4eef7d61d7808e377005c3c5` para
`7df0e3174d0d651189c9281c81c4508e3c691478`. O verificador aceitou a mudança
somente como deriva de proveniência depois de revalidar 44 identidades protegidas
por conteúdo. Passo, contrato, predecessor, corpus, matriz, budgets históricos,
L0s e fonte vanilla permaneceram nos hashes congelados. Qualquer alteração futura
em uma dessas entradas invalida o preseal e reinicia a primeira fase afetada.

## Materialização

Os 35 artefatos autorais, de oráculo, adversário e verificador foram copiados para
`00_nucleo/diagnosticos/` com identidade byte a byte, comprovada por
`p1268-materialization-receipt.tsv`. Os 192 binários de máscara permaneceram como
saída reproduzível temporária; o manifesto, os 192 hashes revalidados, o runner e
o fechamento do verificador foram materializados no repositório.

`git diff HEAD --stat` permaneceu vazio porque todas as saídas P1268 são novos
diagnósticos não rastreados. Nenhum ficheiro em `00_nucleo/prompts/`, `01_core/`,
`02_shell/`, `03_infra/`, `04_wiring/` ou `lab/parity/` foi modificado por esta
execução.

A validação final executou `cargo build` e `crystalline-lint .`, ambos com exit
code zero. O build e o linter ainda reportam warnings preexistentes no produto
(incluindo diagnósticos informativos V16/V19/V20); o P1268 não os alterou nem os
usa como evidência de preseal. O fechamento mecânico também confirmou 52/52
hashes do manifesto e zero TSV malformado antes da inserção deste recibo.

## Artefatos de decisão

- `p1268-preseal.tsv` — preseal independente, SHA-256
  `365d1d30847fb93423305c4c3bd259da2a0f2140f0e4435771ec2b8916e432c2`.
- `p1268-verifier-v3-receipt.tsv` — recibo do verificador, SHA-256
  `4b3bb4faa1ac60e0f7ca9c812a7fc71e9cfe360e98f020cba3bfdb357ec85d1d`.
- `p1268-materialization-receipt.tsv` — 35/35 identidades, SHA-256
  `009232347e4e5df5d925d240f875e442fd347647b898ee4a40673bff3f9d715f`.
- `p1268-suite-freeze-v3.tsv` — suíte julgada, SHA-256
  `fc0b0b5c21449f945c886df10fb0a7ea80219dd92de856bd88d37825defa7ba7`.
- `p1268-head-drift.tsv` — auditoria da mudança de HEAD, SHA-256
  `c9660d1451f0dafdbc631cae2153266e8fa0085426d6935fa85e79246c6ad1a2`.
- `p1268-validation.tsv` — validação final do repositório e dos artefatos.

## Próximo gate

O P1268 termina no preseal. Um candidato futuro só pode ser avaliado contra estes
artefatos congelados; este passo não autoriza escrever ou adaptar implementação.
