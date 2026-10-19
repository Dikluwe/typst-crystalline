# Aquisição e metadados do catálogo — 2026-10-05

## Resultado medido

A hipótese simples de trocar os remapeamentos por leitura segura integral foi refutada neste ambiente. O processamento de metadados passou de cerca de **38,5 ms para 579 ms**, enquanto o intervalo interno de extração caiu de aproximadamente **17,7 ms para 2,1 ms**. O timer interno não isola apenas o algoritmo: acessos a páginas do mapeamento ocorrem durante a extração. Não houve alteração produtiva ou novo ganho demonstrado da CLI.

## Fonte antes da decisão

L0s lidos: `00_nucleo/prompts/infra/fontdb.md`, SHA-256 `ff4211d843d3a9db5ec88222399bcf6584cee5f35ee68aa9e8a40d900f91ecda`, e `00_nucleo/prompts/infra/fonts.md`, SHA-256 `78fa68197c22294ec6ba3effa78b4d4131c25c1de4a95a5bd564e0cec5e2bb05`. A aquisição atual pertence ao primeiro; nomes, exceções e metadados ao segundo. Nenhum desses contratos foi modificado.

`03_infra/src/fontdb.rs:61–64` diz que os bytes de descoberta são reutilizados. Para o ramo File da dependência efetiva, isso não é exato: fontdb 0.23.0 `src/lib.rs:267–273` abre e mapeia o arquivo durante descoberta; `:721–726` delega `with_face_data` à Source; `:910–915` abre e mapeia novamente. O SharedFile usa dados compartilhados. Portanto há repetição de aquisição, mas o código atual não lê integralmente todo arquivo. O comentário não deve ser usado como prova de redução de I/O.

A descoberta também extrai metadados (`fontdb/src/lib.rs:1034–1054`). Não é seguro substituir `font_info_from_bytes` por FaceInfo: `:1062–1066` prefere nameID16, enquanto `03_infra/src/fonts.rs:351–360` usa ID1 com aparo de estilo e exceções. Há ainda diferenças de idiomas/decodificação e inferência de estilo. Isso pode mudar famílias selecionadas, que são observáveis da linguagem. Manter a semântica atual antes de decidir um reaproveitamento.

O vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, em `lab/typst-original/crates/typst-kit/src/fonts.rs:171–192`, também descobre via Database e depois chama `with_face_data(..., FontInfo::new)`. A aquisição duplicada não é particular ao Tekt. Não inferir que o ganho relativo contra vanilla virá automaticamente de alterar esse mecanismo.

## Experimento e proveniência

Working tree não commitado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`. Estado, lista exata de alterações (`git diff HEAD --stat`), hashes e conteúdos das fontes produtivas alteradas, fonte integral da sonda e lockfile, hashes dos arquivos de fonte, timestamps e resultados estão em `catalogo-aquisicao-2026-10-05/measurement.json.gz`. CLI preservada: SHA-256 `f8fc2599bc599b39678575df1eea3fd71b807f1ffd257abbb0ce3413b2344915`. Sonda release: `fdc418ea13038bd7d38989db0cbfddfae40aad4eb121c5b70de2fa1d9df62ae7`. Coleta de `2026-10-05T18:57:48.535043+00:00` a `2026-10-05T18:58:05.495955+00:00`.

Três processos fixados à CPU 0, seis iterações alternando a ordem das alternativas. Cada processo faz uma descoberta real, depois compara somente aquisição e metadados. **Cobertura, inserção no FontBook, carregamento final de fontes e PDF não entram neste intervalo.** Cada variante mantém a ordem e índice originais e chama a mesma função pública produtiva de extração de metadados; resultados completos são comparados fora do timer. Todos coincidiram nas 1.112 faces, 1.086 caminhos. Os dados foram comparados novamente com with_face_data após cada lote, portanto o estudo é repetido e com cache do sistema aquecido; não caracteriza startup com disco frio.

| Processo | Descoberta única, ms | Remap + metadados, mediana ms | Metadados dentro do remap, mediana ms | Read integral + metadados, mediana ms | Metadados após read, mediana ms |
|---|---:|---:|---:|---:|---:|
| 1 | 36,37 | 38,53 | 17,69 | 584,90 | 2,10 |
| 2 | 42,79 | 39,15 | 18,15 | 578,97 | 2,08 |
| 3 | 36,69 | 37,95 | 17,71 | 574,10 | 2,09 |

A alternativa segura usa std::fs::read e retém somente o último arquivo, reaproveitando faces consecutivas da mesma coleção. Foram 1.086 leituras e **4.296.275.016 bytes copiados por lote**. Isto é soma de payload lido, não pico de memória nem bytes físicos lidos do disco. O custo da descoberta permanece separado em ambas as variantes. O primeiro lote read do primeiro processo levou 1.563,05 ms; está preservado nos recibos, não foi descartado da mediana. Não foi adicionado unsafe. O mmap interno já existente pertence à dependência, não é uma implementação nova nesta etapa.

## Decisão e frentes

1. **Não materializar a leitura integral.** O custo é grande neste catálogo; poucas coleções explicam por que reaproveitar faces consecutivas não compensa copiar todos os bytes.
2. **Não priorizar ajustes de strings de metadados agora.** Inferência: os aproximadamente 2 ms após read indicam que o custo puro de extração é menor que o antigo timer de 18–19 ms sugeria. A inferência pode ser refutada por instrumentação que separe faltas de página, tabelas e decodificação no mesmo caminho. As medianas aninhadas não devem ser somadas nem subtraídas como uma decomposição exata.
3. **Investigar aquisição seletiva de tabelas ou reuso dos dados na descoberta.** Uma leitura seletiva segura poderia evitar copiar payloads volumosos de glifos, mas precisa primeiro provar parsing de coleções, offsets, nomes, exceções, cmap e falhas equivalentes. Não há implementação, estimativa de ganho ou aprovação de um novo padrão de aquisição. Uma integração com callback de descoberta precisa preservar a ordem e as regras da dependência; não basta reaproveitar FaceInfo ou chamar preload.
4. **Eliminar o segundo parse é uma frente pequena e independente.** O estudo anterior mediu 0,45 ms naquele snapshot; não transportamos esse número como ganho garantido. Compartilhar Face pode ser explorado depois de definir ownership sem ampliar contrato por conveniência.

A próxima melhoria com evidência mais forte continua sendo a cobertura por intervalos, conforme `cmap-intervalos-2026-10-05.md`; seu L0 público permanece pendente de ratificação. A solicitação desta etapa foi tratada como investigação de aquisição/metadados, não como confirmação daquele contrato.
