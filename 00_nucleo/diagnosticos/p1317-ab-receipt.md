# P1317 — recibo A/B independente

Resultado do fragmento observado: **PASS A/B**, 4.644 comparações integrais, zero divergências e zero Unknown. Não é veredito da implementação, selo geral, mutation score ou prova de equivalência geral CSV.

Regime: A/B executado sem atestação de isolamento técnico. Executor `/root/p1317_tests` em filesystem compartilhado; entradas e escritas declaradas no freeze. Skill `tekt-materializacao-segregada`, referências obrigatórias, L0 completo e ADR-0127 lidos antes do corpus. Nenhum owner produtivo, teste local, código/diff candidato ou recibo de build/medição com fonte foi lido. Inspeção auxiliar de processos restringiu-se aos comandos A/B em curso. Artefatos escritos somente em `p1317-ab-*` e `/tmp/p1317-ab-fixtures`.

## Cadeia e identidades

- HEAD nas medições: `bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, working tree não commitado. Diff/stat exato e status antes/depois, UTC individual, argv e cwd estão nos dois arquivos de execuções.
- L0 normativo SHA-256: `059aad946b30515b24ecfdef4c8490ab735d1bb86d374ce1371c899130ab0d51`. Excluída somente uma linha canônica `Hash do Código`; nenhum outro conteúdo normalizado.
- Freeze anterior à liberação do candidato: `0a701c7f10630c94c131f6e2923b45272a72290d29545592e49d8f370d6169f1`; UTC `2026-09-08T15:02:45.464967+00:00`.
- Baseline P1316: `/tmp/p1316-target.1c6HK7/release/typst`, SHA-256 `1178fcde18dee54cb6b5c0feadcc79c8066346e0bb005060db7a2217a072e8ba`.
- Vanilla ratificado upstream `a51e02804`: `/usr/local/bin/typst`, SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- Candidato recebido após freeze: `/tmp/p1317-target.y5u9ah/release/typst`, SHA-256 `9fcb4cbe830c74ec589506b0b86f52982abfdaab558f62cbf7a8f7de8665ccab`.
- Runner: `79b87269d30fa3f174e7bda2e06a2fe38d7a33fcd4a1683a28d7a7ffcc1ebf33`; casos: `55be7e01ebf5efb88ed9d02b3c152b5c147875c72168aa69b7fc1035cf830420`.
- Observações baseline: `cc8696aae969995174cc04129ba1e16ecad9877c3b6722a0577e90f5e6162831`; candidato: `3b83fc33ca7524056dcbad55240e5f8afa90be8428b49bb76096695bf19176f6`; comparação: `b3634213bbaaaa65620c44777368189125474b877a831dc073739821848dcdcd`.

## Escopo e resultado

387 casos: 330 replays P1316 e 57 casos novos; quatro perfis default/html/a11y/html+a11y. Os 1.320 pares caso/perfil históricos reproduziram exatamente a saída P1316 antes de qualquer transformação. Em 52 casos Utf8 pré-declarados, a expectativa muda exclusivamente a primeira linha de stderr para `error: failed to parse CSV (file is not valid UTF-8)`, preservando exit, stdout e restante literal do diagnóstico baseline. Os demais 335 casos preservam toda a observação.

Baseline: 208 expectativas RED e 1.340 preservadas. Candidato: 624 comparações do delta e 4.020 de preservação passaram em normal/repeat/reverse. Todos os hashes de entradas protegidas e L0 normativo foram revalidados; cardinalidade, identidade, ordem, expressão e cwd foram conferidos.

Novos casos cobrem header/dados, bytes 255 e continuação isolada, sequências truncadas/overlong/surrogate/acima do máximo Unicode, quoting/multilinha, array/dictionary, With/Args/sink/named-before/map-detached, Path/Str, precedência UnequalLengths e controles de opções/cast/I/O/namespace/read e outros decoders. Valores Unicode válidos e quoted coincidiram integralmente com vanilla na medição bilateral e foram preservados pelo candidato.

A fixture binária `/tmp/p1317-ab-fixtures/invalid.csv` contém bytes `[97,44,98,10,255,44,49]`; SHA-256 `110c5d8bada2323b793ba083e1767fb9307332a47068f05e0c82b4488dca076e`. Foi gerada de array declarado pelo preparador, sem alterar fixtures históricas; bytes/hash também constam no corpus.

## Limites mantidos

Vanilla inclui posição `at l:c` ou `in invalid.csv:2:1`; a expectativa não remove esses sufixos da observação vanilla nem declara igualdade integral. O requisito exercitado é a causa textual exata com preservação do diagnóstico cristalino restante. Path/Str continuam detached. Map conserva origem detached legítima. UnequalLengths vence o erro UTF-8 nas colisões declaradas.

Parsing Utf8 com positional excessivo continua erro de parsing cristalino, enquanto vanilla dá `unexpected argument`: os dois casos novos são efeitos normativos explícitos, não provas de paridade. CLI não atesta ausência de chamadas World, decoder puro detached, seleção interna por ErrorKind, Args Rust sintético ou value_span versus occurrence.span; esses pontos pertencem à verificação local segregada.

## Custo e reprodução

Uma rodada baseline/vanilla: 1856 processos, 69.366 s. Uma rodada candidata normal/repeat/reverse: 4644 processos, 247.601 s. Total: 6500 processos, 316.967 s de execução medida. Quatro workers, timeout 30 s por processo. Zero revisões/calibrações do corpus após medição; nenhuma adaptação ao candidato.

```sh
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1317-ab-runner.py run --binary /tmp/p1317-target.y5u9ah/release/typst --binary-sha256 9fcb4cbe830c74ec589506b0b86f52982abfdaab558f62cbf7a8f7de8665ccab --orders normal,repeat,reverse --output 00_nucleo/diagnosticos/p1317-ab-rerun.json
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1317-ab-runner.py compare --freeze 00_nucleo/diagnosticos/p1317-ab-freeze.json --measurement 00_nucleo/diagnosticos/p1317-ab-rerun.json --output 00_nucleo/diagnosticos/p1317-ab-recomparison.json
```

Preservar os artefatos originais imutáveis. Os exemplos gravam novas observações e comparação em arquivos separados; se esses destinos já existirem, escolher outros nomes novos sob `p1317-ab-*` antes de repetir.

Comandos executados no host com autorização de acesso aos binários/fixtures em `/tmp`. Unknown (identidade ambígua, entrada alterada, observação ausente/duplicada/inválida, construção sem suporte, timeout/crash) bloqueia; não houve conversão implícita em sucesso. O recibo registra somente os observáveis congelados.
