# D08B3-A — execução ratificada e limites de certificação

O dono ratificou o manifesto `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b` por `comfirmo`. O recibo está em `00_nucleo/materialization/table-cell-realizacao-d08b3-aprovacao-2026-10-07.json`. A implementação dos 43 owners e do núcleo de realização foi instalada com L0 primeiro; os testes de eval e seu L0 são um suplemento explícito de verificação, não um consumer produtivo novo. O manifesto original permanece imutável.

## O que foi materializado

A inserção prepara Table antes de Cell, resolve ocupação, vazios e propriedades conhecidas, e aplica a regra da Cell antes de realizar o body emitido. O plano tem um dono de placement; aliases preservam escopo causal e cada inserção ganha sua ocorrência. Conteúdo ocultado por show não produz efeitos internos. A Cell conserva corpo lexical, presença e projeção dos campos resolvidos; sua visualização é distinta desse corpo. Sem location artificial para Cell. Layout, introspecção, observação, serialização e HTML consomem o plano sem repetir ocupação ou regra.

O reparo mantém a emissão de uma Cell quando `it` sobrevive, e remove seus defaults quando a regra retorna uma substituição. A ordenação dos efeitos de show continua na ordem de resolução, enquanto o índice de conteúdo respeita o fim dos spans. O Boolean exibido por markup produz Raw `typc`, conforme a superfície de linguagem do vanilla, sem assar fonte/tamanho em eval.

As medições adicionais refutaram duas tentativas de fórmula/condição para tabelas aninhadas. Os registros `geometry-eighth.json`, `after-ninth.json.gz` e `geometry-ninth.json` não são certificados GREEN: o primeiro conserva gap incorreto; o segundo perdeu conteúdo por divergência entre medição e emissão. A decisão final usa a mesma fronteira de sub-frame para ambas, e exige sobrevivência de todos os termos antes de comparar coordenadas.

## Proveniência e validação

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. O snapshot `table-cell-realizacao-d08b3-2026-10-07/tested-tenth.json.gz` contém UTC, lista exata de alterações, diffstat, conteúdo e SHA-256 dos arquivos. As coletas finais contêm fontes integrais, argumentos, diretório, resultados, binários congelados e UTC; ambiente NO_COLOR=1, SOURCE_DATE_EPOCH=0, RAYON_NUM_THREADS=1. O pin vanilla é `9f2b6e8715237cb086899a42873660fe744622e8`; seu binário tem SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, proveniência ratificada P1489, árvore `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`.

Os resultados finais e seus nomes de coleção, hashes, testes, linter e preservação restrita constam de `table-cell-realizacao-d08b3-2026-10-07/final-results.json`. Logs RED, GREEN intermediários e erros de harness permanecem distinguíveis; não se usa falha de compilação de fixture como prova RED. O patch preserva alterações anteriores; não houve commit, reset, reseal global ou atualização global de interfaces.

## Pendências que impedem fechamento universal

A geometria de Boolean/Raw continua divergente. `geometry-eighth.json` e a recoleta final mostram texto semelhante com agrupamento e avanço diferentes: Tekt usa serif 11pt; vanilla usa Mono 8.8pt. A fonte pinada `lab/typst-original/crates/typst-library/src/text/raw.rs:658–670` aplica show-set padrão de 0.8em e DejaVu Sans Mono; `01_core/src/compiler/layout/raw.rs:103–123` conserva tamanho e fonte exteriores. O owner `00_nucleo/prompts/compiler/layout/raw_highlight.md` não integra o manifesto ratificado e expressamente conserva esse estilo corrente. Não falsificar fechamento geométrico com normalização de whitespace nem substituir a regra por um caso especial para booleanos. O próximo reparo deve medir Raw isolado, defaults, show-set explícito, herança, estilos/tamanhos e largura/altura de células antes de definir seu L0. O whitespace de Raw permanece débito independente.

D08B3-B ainda deve decidir defaults de Cell isolada e reconstrução fresca dentro de show. `raw-cell-no-placement`, `additional/outside-fields` e `show-cell-reconstruct` permanecem abertos, como previsto no pacote A. `order/table-root-equality` pertence aos defaults/fields genéricos de Table. Casts ricos, funções em propriedades, strokes/insets/paint, gutters/linhas, spans multipágina e conformidade PDF/UA permanecem fora da certificação. Os três negativos conhecidos de D06 não são fechados por esta etapa. A implementação A está materializada; a certificação geométrica completa aguarda o reparo separado de Raw.

## Resultado final desta execução

Binário Tekt SHA-256 `58456b9b54041983612e5946f89fef2cf4591b9f31a09d8dcce9ae5c8a9c0eea`, snapshot tested-tenth e UTC por execução no final-results. Os 17 negativos primários do B2 foram resolvidos: presença 92/92, selector 23/23. A coleção B3 dá 60/62 e refutadores 16/18; os quatro restantes estão nominalmente separados acima. D05 29/29, D07 29/29 + 14/14, query 36/36 e D06 38/41 com os mesmos três limites (incluindo mensagens de erro), sem normalização.

Geometria final: nested conserva os três termos e tem delta máximo de bbox 0.00004pt; filtro de posição e dois aliases têm delta máximo 0.00002pt. O quinto controle de Raw/Boolean continua divergente e não tem delta de pares certificado, pois a lista de palavras difere. Texto extraído igual não basta para fechar esse controle.

6600 testes core, 1194 infra, 90 shell e 10 integração query passaram; 5 ignorados no core. Builds debug/release passaram. Linter: zero erros, 544 warnings e 1618 notes; não declarar zero ocorrências nem esconder avisos existentes. Preservação: 89 arquivos de implementação/L0 mudaram em relação ao snapshot de aprovação (43 pares, núcleo novo e par suplementar de testes); zero alterações fora desse conjunto. implementation.patch compara com a base aprovada, não com HEAD anterior às mudanças preexistentes. Não houve commit.

## Atualização posterior — reparo separado de defaults Raw

raw-defaults-execucao-2026-10-07.md supersede a pendência de fonte/tamanho/cor
no renderer. Esta correção contínua tem owners próprios; não altera o manifesto
A. geometry-raw-defaults.json preserva todas as palavras nos cinco controles: 
Boolean tem delta máximo bbox0.001793pt e demais até0.00004pt. O serif11pt
descrito acima é registro histórico, não o estado corrente. Raw bloco e
resíduos inline continuam explicitamente abertos; B3-B/igualdade genérica
de Table e demais scope-outs permanecem.
