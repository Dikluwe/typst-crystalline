# P1382 — relatório de paridade

## Veredito

`MATCH` para baseline e avanço da linha seguinte quando uma linha contém ao
menos dois tamanhos textuais distintos. Nas duas ordens medidas, os fragmentos
ficaram na baseline `21,844pt` e a linha seguinte em `36,232pt`, exatamente
como o vanila. Três fragmentos 11pt→18pt→11pt também partilham `21,844pt`.

Não há alegação de paridade geral. Linha de tamanho único preserva o caminho
anterior como controle. Prefixo misto antes de distribuição fracionária segue
`Unknown` e o gate P1381 não foi relaxado.

## Implementação

`cursor::flush_line` detecta tamanhos distintos no acumulador escalar já
existente. Só nesse caso corrige a baseline pelo maior ascent textual/inline,
resolve leading e ascent seguinte pelo estilo ativo do parágrafo e avança por
`ascent_seguinte + descent_corrente + leading`. A mesma correção alimenta o
fitting. Não há recursão, coleção auxiliar, carrier ou API pública nova.

Duas candidatas amplas foram rejeitadas: a primeira paginou o controle P1381;
a segunda recuperou a página, mas deslocou o Block 2,1pt. Conforme o protocolo
Tekt, o desenho foi reaberto e reduzido ao domínio diretamente provado, em vez
de receber uma terceira correção incremental.

## Evidência

- RED P1382: 0/3; GREEN final: 3/3.
- Controles: P1381 4/4, P1380 4/4, leading 3/3, inline baseline 7/7 e
  P1374–P1379 16/16.
- Matriz bilateral: 6 `MATCH`, 1 `Unknown`; P1381 pequeno preserva todas as
  coordenadas (`Lead=8,112`, `One=25,868`, `Two=7,404`,
  `After=Tail=220,166`).
- `cargo check --workspace`, `cargo fmt --check` e `git diff --check`: sucesso.
- `crystalline-lint .`: zero bloqueios, 1433 advisories preexistentes e zero
  advisory novo.
- Build/testes seriais sob 4 GiB; matriz serial sob 2 GiB/45 s. Picos:
  teste 2 088 628 KiB, release 1 142 592 KiB, matriz 36 760 KiB; nenhum OOM,
  timeout ou SIGKILL.

Proveniência final: HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`,
working tree não commitado e agregado, 47 ficheiros no diff (`4565`
inserções, `419` remoções) e 323 entradas de status em
2026-09-13T21:22:36-03:00. Binários: vanila `eb60986b...`, cristalino
`e4a176a4...`, teste `23de97ea...`.

Regime Tekt: protocolo completo sem atestação de isolamento. A mesma autoridade
produziu contrato, testes, candidato, ataques e veredito; hashes demonstram
ordem e identidade, não independência.
