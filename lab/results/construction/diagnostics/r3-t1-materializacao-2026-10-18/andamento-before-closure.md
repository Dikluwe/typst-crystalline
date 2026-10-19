# R3-T1 — materialização em andamento

A implementação não está encerrada. O objetivo geral continua ativo. A aprovação humana autoriza a ordem causal e transporte privado, sem commit/publicação/release.

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, branch Tekt, working tree não commitado. before-sources.json.gz conserva o estado inicial integral, status/diffstat e prova do predecessor; candidate*-sources.json.gz conservam cada candidato. Cada execução tem recibo de recursos com comando, UTC, HEAD/diffstat e limite 12GiB/High8GiB/swap0/jobs1.

## Evidência já obtida

| Log | Resultado registrado |
|---|---|
| acceptance-red.log | ('FAILED', '26', '30', '0', '7328') |
| style-unit-red.log | ('FAILED', '0', '8', '0', '7384') |
| unit-candidate1.log | ('FAILED', '51', '13', '0', '7328') |
| unit-candidate2.log | ('FAILED', '63', '1', '0', '7328') |
| unit-candidate3.log | ('ok', '64', '0', '0', '7328') |
| unit-candidate4-valid.log | ('FAILED', '64', '1', '0', '7328') |
| core-candidate4.log | ('FAILED', '7373', '15', '5', '0') |
| core-candidate5b-skip-reuse.log | ('FAILED', '7376', '10', '5', '2') |

O candidato3 passou as 56 sondas textuais e oito de cores. As rodadas amplas seguintes refutaram a conclusão da etapa: registraram regressões e aborts de Table. A execução com dois testes excluídos é somente investigação, nunca evidência de fechamento. regex-oracle.json conserva a medição independente de composição Emph/Strong no vanilla pinado.

## Trabalho que permanece obrigatório

- Resolver todas as regressões amplas, sem exclusões na validação final.
- Confirmar exportação PDF/HTML e SVG reais, incluindo confinamento de cores.
- Revalidar controles anteriores, workspace/doctests, Clippy/Rustdoc estritos, formato, linhagens e delta SARIF.
- Congelar o resultado, preparar manifesto e relatório final.

progress.json contém as referências, hashes dos logs e recursos. Este arquivo é um registro de andamento; não é um L0 nem declaração de conclusão.
