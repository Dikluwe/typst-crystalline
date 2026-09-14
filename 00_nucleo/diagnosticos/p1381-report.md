# P1381 — relatório de paridade

## Veredito

`MATCH` no escopo estreito: um único `Styled(Text)` sem wrap, com
`text.size` menor ou igual ao estilo exterior, antes do primeiro Block `fr`
de um segmento explícito de colunas. O caso medido em 8pt coincide com o
vanila em `Lead=8,112`, `One=25,868`, `Two=7,404` e
`After=Tail=220,166`.

Não há alegação de paridade geral. Primeiro fragmento maior e linha composta
com tamanhos diferentes permanecem `Unknown`. A candidata v1 fechava a página
mas deslocava o corpo; a v2 acertou o corpo do misto, mas deixou seus dois
fragmentos 4,606pt acima do vanila. Após o budget de duas revisões, o desenho
foi reaberto e o contrato reduzido ao subconjunto comprovado, sem constante
empírica.

## Implementação

O preflight mantém acumuladores escalares: quantidade de fragmentos, ocorrência
de mudança de tamanho, top-edge inicial, maior estilo e estilo final. A
fronteira de bloco acompanha o leading do fragmento reduzido. Se houver mais de
um fragmento com mudança de tamanho ou se o primeiro exceder o estilo exterior,
o caminho fecha sem crédito. Não foi criada recursão, coleção proporcional,
simulação de layout nem estado novo no `Layouter`.

## Evidência

- RED P1381: 0/3; GREEN final: 3/3.
- Controles: P1380 4/4; P1374–P1379 16/16.
- `cargo check --workspace`, `cargo fmt --check` e `git diff --check`: sucesso.
- `crystalline-lint .`: exit 0, apenas advisories preexistentes.
- Build e testes seriais sob 4 GiB; matriz serial sob 2 GiB e timeout 45 s.
- Picos: testes 1 944 716 KiB, release 1 147 296 KiB, matriz 37 576 KiB;
  nenhum OOM, timeout ou SIGKILL.

Proveniência final: HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`,
working tree não commitado e agregado, 47 ficheiros no diff (`4396`
inserções, `350` remoções) e 312 entradas de status em
2026-09-13T20:50:30-03:00. Binários: vanila `eb60986b...`, cristalino
`2d353f53...`, teste `ad9e6351...`.

Regime Tekt: protocolo completo sem atestação de isolamento. A mesma autoridade
produziu contrato, testes, candidato, ataques e veredito; os hashes demonstram
ordem e identidade, não independência.
