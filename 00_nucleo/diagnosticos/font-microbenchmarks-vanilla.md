# Microbenchmarks de fontes: Tekt e vanilla pinado

Este relatório compara APIs públicas reais do vanilla `9f2b6e8715237cb086899a42873660fe744622e8` com o Tekt atual no mesmo executável diagnóstico. A bateria anterior comparava um algoritmo histórico extraído; ela permanece intacta e não serve como a população Tekt desta comparação. Os dois engines são medidos novamente nas mesmas fontes e rodadas.

Foram executados **96 casos, quatro fontes, três rodadas e 30 amostras por caso/rodada**. Todos os blocos passaram nos controles de integridade; **88 casos falharam na estabilidade absoluta**. A coleta é exploratória, não uma baseline estável.

O vanilla foi mais rápido no caminho público de metadata+cobertura em todas as fontes/rodadas desta composição diagnóstica. Já o construtor genérico Tekt foi mais rápido em todas elas. Isso mostra por que comparar somente o construtor genérico levaria à interpretação errada do carregamento real. No primeiro acesso CJK/TTC, Tekt levou aproximadamente 19,5–24,4 ms e vanilla 7,0–8,3 ms, com razões Tekt/vanilla de 2,70×–3,33×. Essas são composições distintas; não isolam a parcela da reconstrução TTC nem explicam por si o gap end-to-end.

## O que a fonte comprova antes das medições

`lab/typst-original/crates/typst-library/src/text/font/info.rs:294` mostra que o construtor genérico `Coverage::from_vec` ordena/deduplica pontos antes de construir runs. Entretanto, `:119` constrói a cobertura real de `FontInfo` diretamente de cada subtable Unicode da cmap e une coberturas. Comparar somente construtores genéricos não representa o caminho real de leitura de fontes do vanilla.

No Tekt, `01_core/src/entities/font_book.rs:172` compacta sequências em intervalos antes de ordenar/mesclar. O benchmark de metadata+cobertura usa composição diagnóstica de APIs públicas e duas parses; não expõe nem mede isoladamente o extrator privado. O vanilla também produz campos adicionais de metadata. Os controles verificam os mesmos conjuntos e glifos amostrados, sem declarar identidade de todos os contratos de metadata.

`03_infra/src/fonts.rs:153` reconstrói a face TTC em sfnt, copiando tabelas e recalculando checksums. `lab/typst-original/crates/typst-kit/src/fonts.rs:112` lê o arquivo e chama Font::new com o índice original, mantendo a coleção. Primeiro acesso mede essas composições inteiras e a obtenção do handle, não uma fase isolada de I/O, parsing ou cobertura. Acesso retido mede slot/store já inicializado; o vanilla inclui o lookup público por índice.

## Proveniência e reprodução

O novo crate, protocolo, scripts e dados estão em [font-microbenchmarks-vanilla/](font-microbenchmarks-vanilla/). Somente esse diagnóstico e este relatório foram adicionados. Nenhuma produção, API, L0, ADR, benchmark existente ou evidência anterior foi alterada. Sem formatação global ou resselo; rustfmt limitado a dois arquivos Rust novos.

Pin vanilla confirmado por todos os hashes do registro ratificado; árvore `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`. A cópia staged é byte a byte, com manifests originais, necessária porque dependências diretas na quarentena resolviam herança do workspace externo. Só o manifest diagnóstico aponta para `/tmp`. A diferença local do Cargo.lock vanilla já registrada continua explícita em staged-provenance.json.

O executável medido é uma nova linkagem combinada. Seu Cargo.lock não é a receita do CLI vanilla ratificado: dependency-graph-differences.json registra as diferenças. O CLI ratificado de SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` permanece intacto e não é o executável cronometrado. Não se usa string de versão como prova do pin.

HEAD, árvore dirty exata (incluindo untracked), patches, horários, consumidores, L0s, inputs, fontes originais/staged, scripts e executável possuem registros antes/depois. O arquivo consumers-inputs.tar.gz preserva fontes congeladas; fontes externas são identificadas pelos hashes e não redistribuídas. bin/font-vanilla-comparison preserva o executável independente do target mutável. Veja protocol.md e README.md para comandos e fronteiras de setup/destruição/cache.

## Coleta e resultados

HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado. Lista exata de arquivos alterados/untracked, diff-stat e patch em state/frozen.*, mais snapshots de cada preparação/rodada. Fingerprint congelado `9d29d3f8b1a0223b55eff92b955b4cd0735d205cd4c49970825ae2047bc44fe2`. Fonte original e staged do vanilla, todos os consumidores/L0s, fontes de input e executável coincidiram antes/depois dos três blocos. Nenhum build/teste/benchmark concorrente foi detectado pelo monitor de 100 ms.

Máquina Dell-G15-Linuxmint, AMD Ryzen 7 5800H, 16 CPUs lógicas; Linux 6.17.0-42-generic, x86_64. Rustc 1.92.0 / LLVM 21.1.3; perfil release opt-level 3, sem debug assertions/overflow checks, sem flags Rust ambientais. CPU 4 para medidas e CPU 6 para runner; Rayon com uma thread. Governor powersave e boost habilitado, sem alterações de política. Build-provenance.json e receipts guardam flags, comandos, toolchain completa, timestamps e leituras pontuais.

Rodadas UTC:

| Rodada | Início | Fim | Ordem |
| --- | --- | --- | --- |
| r1 | 2026-09-27T13:45:19.583629+00:00 | 2026-09-27T13:46:52.778432+00:00 | normal |
| r2 | 2026-09-27T13:47:25.612882+00:00 | 2026-09-27T13:49:01.759308+00:00 | invertida |
| r3 | 2026-09-27T13:49:34.880875+00:00 | 2026-09-27T13:51:08.290323+00:00 | normal |

Load médio de um minuto variou de 9,2 a 13,1; a máquina não estava ociosa. Frequências pontuais antes/depois variaram aproximadamente de 3,16 a 3,87 GHz. Não há série de frequência/thermal por caso que demonstre a causa da dispersão. Processos de fundo e a revisão automática de ferramentas não foram neutralizados; consultas leves de progresso ocorreram durante a coleta. Isso reforça o caráter exploratório e impede tratar a afinidade como exclusividade física da máquina.

O executável arquivado tem SHA-256 `3506975c46b26034a71b003deac1e72327f29d5df4d5e638353fbb3e6907ff0a`. A sonda com debug assertions ativadas recusou antes de fixtures/timers, e o release passou nos controles. `cargo bench --manifest-path benches/Cargo.toml --no-run --offline --locked --message-format=json` compilou os oito benchmarks existentes; perfis/hashes estão em benchmark-build-profiles.json. Falhas iniciais de preparação ficaram separadas em logs/preparation-notes.json, sem amostras.

Médias dos 30 valores tempo/iteração por rodada, **µs**. T/V é razão entre médias sequenciais da mesma fonte/rodada; a inversão de ordem não cria amostras pareadas. Todas as linhas seguintes falharam nos critérios absolutos. Não foram removidas rodadas ou outliers.

| Operação / fonte | Tekt r1 / r2 / r3 µs | Vanilla r1 / r2 / r3 µs | T/V r1 / r2 / r3 |
| --- | --- | --- | --- |
| Genérico / LibertinusSerif-Regular | 32.295 / 44.985 / 36.514 | 70.512 / 90.483 / 63.688 | 0.46× / 0.50× / 0.57× |
| Genérico / NewCMMath-Regular | 58.106 / 50.090 / 30.851 | 126.577 / 152.542 / 102.234 | 0.46× / 0.33× / 0.30× |
| Genérico / NotoSansCJK-Regular | 429.556 / 509.427 / 371.276 | 1558.819 / 1332.738 / 1220.423 | 0.28× / 0.38× / 0.30× |
| Genérico / NotoSansMath-Regular | 19.542 / 20.647 / 21.345 | 49.830 / 50.738 / 55.051 | 0.39× / 0.41× / 0.39× |
| Metadata+cobertura / LibertinusSerif-Regular | 41.404 / 124.338 / 35.470 | 23.734 / 43.309 / 24.933 | 1.74× / 2.87× / 1.42× |
| Metadata+cobertura / NewCMMath-Regular | 65.014 / 101.160 / 58.755 | 60.587 / 38.108 / 31.566 | 1.07× / 2.65× / 1.86× |
| Metadata+cobertura / NotoSansCJK-Regular | 768.623 / 909.172 / 723.095 | 486.112 / 575.798 / 423.815 | 1.58× / 1.58× / 1.71× |
| Metadata+cobertura / NotoSansMath-Regular | 31.638 / 32.034 / 35.593 | 30.940 / 22.369 / 20.576 | 1.02× / 1.43× / 1.73× |
| Primeiro acesso / LibertinusSerif-Regular | 36.103 / 31.134 / 28.629 | 86.938 / 220.157 / 61.940 | 0.42× / 0.14× / 0.46× |
| Primeiro acesso / NewCMMath-Regular | 48.646 / 67.843 / 32.928 | 177.023 / 222.498 / 135.100 | 0.27× / 0.30× / 0.24× |
| Primeiro acesso / NotoSansCJK-Regular | 24370.395 / 19511.820 / 23375.081 | 8328.582 / 7234.473 / 7015.622 | 2.93× / 2.70× / 3.33× |
| Primeiro acesso / NotoSansMath-Regular | 22.108 / 16.797 / 19.561 | 105.157 / 66.581 / 75.814 | 0.21× / 0.25× / 0.26× |

Acesso retido ficou na ordem de 31–56 ns por chamada, perto do custo de controle/batching. Somente o par Libertinus retido passou nos dois critérios: Tekt aproximadamente 30,9 / 32,4 / 32,1 ns e vanilla 31,3 / 32,1 / 31,9 ns. Esses resultados não suportam uma grande vantagem de cache retido para um dos engines. Demais consultas e controles possuem distribuições completas em summary.json.

Na TTF/OTF simples, o primeiro acesso Tekt ficou abaixo do vanilla, mas o Tekt retorna dados validados sem construir toda a metadata/cobertura que o Font vanilla incorpora. Não se interpreta isso como comparação de dois objetos com o mesmo trabalho já realizado. Em TTC o Tekt foi mais lento apesar dessa diferença, tornando reconstrução um candidato para intervenção, ainda sem parcela isolada.

Raw Criterion times/iters, estimates/confidence intervals e stdout/stderr estão em rounds/r*/criterion. Summary.json contém média, mediana, SD, min/max, p05/p95 de cada rodada, razões e classificações. Estabilidade exige diferença média/mediana das três médias <3% e range/mediana <=5%. Dos oito casos estáveis, dois são o par retido Libertinus, dois são coleta diagnóstica comum, três são consultas/acesso retido vanilla e um é controle. Não extrapolar esses pontos para descoberta completa ou pipeline.

## Hipóteses que precisam de intervenção

- **Cmap:** comparar num diagnóstico privado a mesma Face já parseada com coleta+construtor versus construção streaming, controlando metadata e destruição. Não confundir isso com medição do helper produtivo nem adicionar API produtiva. Refutação: diferença desaparece em workload equivalente e repetível.
- **TTC:** repetir primeiro acesso de ambos com a mesma face TTC e sua sfnt previamente reconstruída, verificando coverage e glifos, cache aquecido e destruição fora do timer. A intervenção também muda tamanho/container; não isola automaticamente checksum ou cópia. Refutação: grande diferença persiste em fontes simples equivalentes.
- **Cache:** comparar primeiro/retido com o mesmo catálogo e demanda, separando reconstrução, custo de catálogo e recriação do World. Refutação: o trabalho poupado não ocorre na execução CLI relevante ou o ganho não aparece no ensaio end-to-end correspondente.
- **Ruído:** repetir em máquina ociosa com clock/thermal por caso e ordem randomizada/intercalada; não descartar rodadas somente porque não favorecem uma hipótese.

Não somar fixtures diferentes, subtrair controles de outra operação, inferir causa pelo tamanho de razões ou atribuir exportação inteira ao stream. Esta bateria não mede shaping, layout, exportação nem o pipeline completo e não fecha a causa do gap end-to-end.
