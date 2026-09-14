# P1383 — relatório de paridade

## Veredito

`MATCH` no recorte exato em que um `Text` exterior direto é seguido por um
`Styled(Text)` maior, cujo delta altera exclusivamente `text.size`, antes de
um `Block` fracionário e sem quebra de linha. Vanila e cristalino produziram
uma página e as mesmas coordenadas: `Lead=12,010`, `More=5,752`,
`One=32,448`, `Two=7,404` e `After=Tail=220,166` pontos.

Não há alegação de paridade geral. Ordem inversa, fragmento único maior, três
fragmentos e delta que combina tamanho com preenchimento continuam `Unknown`
e fail-closed.

## Implementação

O preflight de sequência reconhece apenas dois fragmentos textuais: o primeiro
direto no estilo exterior e o segundo maior com delta exclusivamente de
tamanho. Para essa forma, divide `par.spacing` com o leading exterior que o
cursor P1382 usa ao fechar a linha mista. O estado continua escalar, sem
recursão, coleção auxiliar, API pública ou mudança de pipeline.

A candidata v1 foi rejeitada porque usou o estilo do último fragmento para
leading e fronteira, deixando o bloco focal 4,55pt acima do fundo. A medição
mostrou que o cursor P1382 fecha a linha pelo estilo exterior; o contrato e os
ataques foram atualizados antes da candidata v2.

## Evidência

- RED P1383: 2/3; apenas o oráculo positivo falhou. GREEN final: 3/3.
- Controles: P1382 3/3, P1381 3/3, P1380 4/4 e P1374–P1379 16/16.
- Matriz bilateral: 2 `MATCH`, 4 `Unknown`; os controles opacos não foram
  absorvidos acidentalmente.
- `cargo check --workspace`, `cargo fmt --check` e `git diff --check`: sucesso.
- `crystalline-lint .`: zero bloqueios, 1433 advisories preexistentes e zero
  advisory novo.
- Build e testes seriais sob 4 GiB; matriz serial sob 2 GiB/45 s. Picos:
  teste 1 923 584 KiB, release 1 143 716 KiB, matriz 37 536 KiB; nenhum OOM,
  timeout ou SIGKILL.

Proveniência final: HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`,
working tree não commitado e agregado, 47 ficheiros no diff (`4687`
inserções, `419` remoções) e 334 entradas de status em
2026-09-13T21:42:02-03:00. Binários: vanila `eb60986b...`, cristalino
`5213f5ce...`, teste `eb8438e3...`.

Regime Tekt: protocolo completo sem atestação de isolamento. A mesma autoridade
produziu contrato, testes, candidato, ataques e veredito; hashes demonstram
ordem e identidade, não independência.
