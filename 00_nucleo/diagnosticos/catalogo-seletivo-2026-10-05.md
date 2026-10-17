# Aquisição seletiva de metadados — experimento, 2026-10-05

A leitura seletiva tornou-se um candidato após reduzir a tabela `post` ao prefixo validado necessário aos metadados. No experimento, o trecho de aquisição/metadados passou de **38,10 para 29,00 ms** (23,9%); com a cobertura atual incluída, **94,33 para 88,04 ms** (6,7%). São medianas das medianas dos processos de uma sonda independente, não tempos da CLI. Não houve materialização produtiva, troca do default de aquisição, mudança de API ou resselo de L0.

## Evidência antes da decisão

Os L0 vigentes foram lidos: `00_nucleo/prompts/infra/fonts.md` e `00_nucleo/prompts/infra/fontdb.md`. O primeiro define metadados e slots, o segundo exige aquisição via Database::with_face_data. O experimento não altera esses contratos. O relatório anterior `catalogo-aquisicao-2026-10-05.md` refutou a leitura integral e documentou a aquisição repetida, também presente no vanilla pinado.

`ttf-parser` 0.25.1 `src/lib.rs:1102–1111` e `:1230–1233` exigem head/hhea/maxp. O metadata owner `03_infra/src/fonts.rs:339–399` também consulta name, OS/2 e post (monospace via Face). Em `tables/post.rs:334–379`, a validade do cabeçalho depende de tamanho/version; para versão 2, também dos índices de nomes. O conteúdo dos nomes posteriores não é validado nesse parse nem usado pelo catálogo.

O adapter diagnóstico `catalogo-seletivo-2026-10-05/probe/src/selective.rs:8` abre o arquivo com std::fs::File, lê o cabeçalho/diretório, encontra o índice da coleção e copia somente as tabelas necessárias. A ordem completa dos registros é mantida; offsets de tabelas copiadas são reescritos num buffer temporário. Dados de outras tabelas são omitidos. O buffer é exclusivo dos metadados e da cobertura; **não é fonte para shaping/exportação nem substituto de FontSlot**. Não há unsafe novo.

Primeiro, copiar as tabelas selecionadas inteiras levou cerca de 230 ms para metadados e 256–291 ms com cobertura, pior que o remap. O recibo `measurement-full-post.json.gz` preserva essa versão e coleta. O recibo `table-inventory.json.gz`, com script, hashes das fontes e estado da árvore, registra o inventário dos diretórios: ele mostrou que post domina os bytes selecionados: aproximadamente 304 MB, contra 3 MB de name e 14,5 MB de cmap nesse ambiente. O primeiro protótipo lia 307.686.075 bytes por lote sem cmap, 322.212.248 com cmap. A decisão de reduzir post veio desta medição e da leitura do parser, não apenas do resultado favorável final.

No segundo protótipo, post mantém 32 bytes nas versões que não requerem índices; versão 2 conserva 34 + 2×indexes_count, limitado ao tamanho original. O limite integral declarado é validado contra o arquivo antes de aproveitar o prefixo. Tamanho/versão/index-array inválidos continuam inválidos; não se substitui a versão ou fabricam índices. Essa versão leu **36.596.745 bytes por lote sem cmap**, **51.122.918 com cmap**. São bytes solicitados pelas leituras, incluindo prefixos relidos, não I/O físico ou RSS.

## Medições e controles

Working tree não commitado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`. O recibo `measurement.json.gz` inclui lista exata de alterações (`git diff HEAD --stat`), status, hashes e conteúdo de fontes alteradas, programa completo, Cargo.lock, hashes dos arquivos de fonte, comandos/timestamps e amostras. CLI produtiva intacta, SHA-256 `f8fc2599bc599b39678575df1eea3fd71b807f1ffd257abbb0ce3413b2344915`. Sonda final medida: `0b02515e5109108aaf7f4be195d48f96e2796ebdd282a2d7e38252701ecfc7c1`. Coleta de `2026-10-05T20:06:50.433758+00:00` a `2026-10-05T20:06:59.675979+00:00`.

Três processos fixados à CPU 0, quatro iterações de cada modo com ordem alternada. A comparação usa o mesmo extrator público atual. O modo com cobertura aplica também a enumeração cmap e Coverage::from_codepoints atuais — não usa o experimento por intervalos, ainda pendente de ratificação pública. Igualdade integral de FontInfo é conferida fora do timer para cada face e cada lote, incluindo coverage nos modos correspondentes.

| Processo | Descoberta, ms | Remap/metadados, ms | Seletivo/metadados, ms | Remap/metadados+cobertura, ms | Seletivo/metadados+cobertura, ms |
|---|---:|---:|---:|---:|---:|
| 1 | 36,66 | 38,10 | 29,73 | 94,33 | 88,04 |
| 2 | 41,47 | 38,23 | 28,71 | 93,32 | 88,97 |
| 3 | 36,73 | 37,79 | 29,00 | 95,14 | 86,08 |

São medianas por processo. Descoberta ocorre separadamente uma vez por processo; as demais linhas não incluem inserção no book, composição do World, shaping ou exportação. Comparações posteriores ao timer aquecem cache do sistema: não é estudo de disco frio. Todas as comparações nas **1.112 faces / 1.086 caminhos** coincidiram. Isto prova os DTOs desta coleção; não é prova completa de seleção de fonte e PDF na integração.

Seis testes passaram em debug e release: fixtures TTF/OTF/variáveis e casos de nomes/exceções; coleção sintética com offsets absolutos e índices inválidos; versões/tamanhos/arrays de post e pitch fixo; limites inválidos de tabelas opcionais/obrigatórias; cabeçalhos truncados/assinaturas inválidas; redução de bytes com post volumoso. Os logs, fontes finais dos testes e horários estão em `execution.json.gz`.

A primeira execução debug encontrou uma assertion do parser de referência em uma coleção malformada que declara u32::MAX faces. O log inicial está preservado. Esse caso extremo foi separado do oráculo: agora verifica a rejeição sem panic do adapter; não se afirma equivalência do comportamento do parser debug nesse caso. Os demais casos continuam com comparação direta. Não há alteração dessa dependência ou do código produtivo.

## Decisão e limite para integração

Inferência: há um possível ganho pequeno na aquisição real do catálogo, sem copiar o payload completo. A hipótese é refutada se a CLI pareada não melhorar ou se ordem, seleção de fonte, texto, geometria ou PDF divergirem. Não somar o ganho deste trecho ao ganho de cmap por intervalos; a combinação requer nova coleta.

Não materializar o protótipo tal como está. Ele ainda precisa de política explícita para erros de leitura, arquivos que mudam durante aquisição, limites de alocação e fallback para fontes/tabelas não cobertas. Manter o caminho original para validação e fallback precisa de L0 definido antes de código; não assumir que qualquer falha de mmap pode virar leitura eager.

A próxima etapa produtiva deve atualizar o L0 de aquisição e definir owner exclusivo do helper, preservando ADR-0129 e separação L1/L3. Como isso altera o caminho de aquisição atualmente especificado, classificar o gate da ADR-0127 antes da materialização; em dúvida sobre alteração de default, ratificar com o dono. A fonte final/shaping/exportação continua original. Medir CLI e controles de linguagem/PDF, sem concluir ganho a partir deste benchmark isolado.
