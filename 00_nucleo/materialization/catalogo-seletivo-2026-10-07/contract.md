# Contrato ratificável — prioridade 3: aquisição seletiva do catálogo

Pacote de execução; não é L0. Candidatos L0 completos em candidates/. Nenhum código produtivo ou L0 canônico alterado nesta preparação.

## Medição que precede a decisão

Histórico medido em `00_nucleo/diagnosticos/catalogo-seletivo-2026-10-05.md`: aquisição/metadados 38,10→29,00ms e aquisição+coverage antiga 94,33→88,04ms. O primeiro protótipo, que copiava post inteiro, foi refutado. Esses valores são de sonda de05/10, não ganho atual/CLI, nem devem ser somados ao ganho da prioridade2.

Auditoria atual `00_nucleo/diagnosticos/catalogo-seletivo-preparacao-2026-10-07/audit.json`, com before.json.gz, HEAD/working tree exata/UTC/font hashes: 1.140 faces, máximo20 tabelas, buffer projetado máximo259.927 bytes; solicitação máxima259.959 bytes exclui até16 bytes extra de header/índice TTC. Todos os1.086 assets históricos conservam bytes; corpo font_info_from_bytes continua idêntico à árvore histórica limpa comprovada pelo recibo. Coverage mudou deliberadamente na prioridade2 e permanece vigente. Release anterior congelável SHAe44043d9471a6d2eec25da7562dd5b0a071d4e14ab0f307011ca978e46e49109; oracle vanilla pin9f2b6e8715237cb086899a42873660fe744622e8/bin7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.

Fontes primárias: `03_infra/src/fontdb.rs:42–113` exige with_face_data e alinhamento book/slots; `03_infra/src/fonts.rs:339–405` extrai name/OS2/post e delega coverage. fontdb0.23.0 lib.rs:721–727,900–920 remapeia Source::File e usa bytes retidos de SharedFile. ttf-parser0.25.1 lib.rs:1433–1482 usa OS2 para estilo/peso/largura e post para monospace; tables/post.rs:334–379 valida32 bytes/version e, na v2,34+2×indices. Tamanho integral declarado deve ser válido antes de truncar post. O tail dos nomes post não é usado pelos metadados. Consulta de nomes/exceções e callback de coverage são dos owners existentes, não serão duplicados.

## Classificação, decisão e refutação

Seleção de fonte, metadados e coverage são observáveis da língua. Forma do buffer, read/mmap, identidade de DTOs e bytes omitidos são mecanismos; igualdade de DTOs é controle interno adicional, não definição universal de paridade (ADR0107/0108). Não inferir intenção upstream do mecanismo observado. Checagem extra: SharedFile usa Arc retido, não o caminho; reabrir seu path pode consultar outra fonte.

Inferência: ler menos payload do Source::File pode melhorar catálogo/CLI após coverage por intervalos. Refuta-se por seleção/coverage/book/slots/erros/render alterados, ou por ausência de benefício repetível/por regressão material em controles equivalentes. Não prometer ganho prévio. Se refutada, conservar/repor o caminho produtivo anterior e registrar todos os resultados.

Proposta: tentativa seletiva por defeito somente para Source::File regular em Unix; Source::SharedFile continua no caminho retido with_face_data. Binary continua excluído. Qualquer rejeição da tentativa, inclusive metadata/coverage inválida, retorna ao callback antigo; fallback não é eager-read de FontSlot. Sem nova API pública, entidade, trait, flag, fase, ordem, política de cache ou alteração de aquisição final de fontes.

Gate: ADR0127 §2.2 (“troca de caminho por defeito do compilador”) e §5 (“em caso de dúvida sobre a classe ... parar”). Classifico esta substituição da cláusula obrigatória with_face_data por uma tentativa padrão de I/O seletivo como mudança de caminho padrão; a confirmação da prioridade2 ratificou apenas coverage e excluiu expressamente prioridade3. Confirmar este pacote antes de instalar L0/escrever consumer novo. A ratificação autoriza os detalhes privados, RED→GREEN, resselo, builds/lint/corpus e medições deste contrato sem novas paragens genéricas.

## Ownership e escopo

1. `00_nucleo/prompts/infra/fontdb.md` ↔ `03_infra/src/fontdb.rs`: escolha File/SharedFile, fallback, composição dos loaders e slot/book.
2. Novo `00_nucleo/prompts/infra/fontdb/selective.md` ↔ novo `03_infra/src/fontdb/selective.rs`: aquisição limitada, projeção privada, estabilidade e extração de FontInfo. Child privado de fontdb; função privada de fachada `acquire(path:&Path,index:u32)->Option<FontInfo>` pode ser pub(super), sem escapar de L3.

`fonts.rs`, coverage.rs, FontSlot, World, L1 e seus L0 permanecem iguais. Parent reutiliza seu callback antigo para fallback. Child reutiliza font_info_from_bytes e extract_coverage vigentes sobre buffer privado normalizado para índice0. Nenhum consumer aponta ao passo, diagnóstico ou núcleo; nenhum prompt compartilhado1:N.

## Limites e aquisição

- Abrir File e obter metadata do mesmo handle; só arquivo regular. Fast path requer identidade Unix (dev/ino, len, mtime/mtime_nsec e ctime/ctime_nsec); se não disponível, declinar. Outros sistemas usam o caminho antigo.
- Confirmar assinatura sfnt 0x00010000/OTTO/true ou TTCv1/v2 elegível. Validar índice original, limites de array TTC/diretório e offsets absolutos com checked arithmetic/get/read_exact. Índice inválido, formato desconhecido, contagem excessiva ou layout não coberto declinam, não fabricam face.
- Máximo4.096 registros sfnt; diretório cabe nesse limite. Máximo8MiB de buffer projetado incluindo header/diretório/tabelas/padding. Máximo16MiB de bytes solicitados por tentativa, contando rereads/prefixos. Conferir budgets e limites antes de seek/alocação/leitura; usar try_reserve para retornar rejeição em falha de reserva. Limites são privados, não truncam metadata; atingidos, retorna ao caminho original.
- Preservar ordem e registros integrais do diretório, inclusive duplicatas; copiar head/hhea/maxp/name/OS2/post/cmap. Tabelas omitidas têm offset/length zerados; não ordenar/deduplicar entradas. Validar extensão integral de toda tabela selecionada contra tamanho original antes de aproveitar prefixo. Bounds inválidos declinam ao fallback, sem transformar opcional inválida em tabela válida.
- post: conservar min(length,32) para versões sem array v2; versão2 com campo de contagem completo conserva min(length,34+2×count). Se campo/array é truncado, conservar invalidade ou declinar; não trocar versão, pitch ou criar índices. O parser tipado continua determinando validade. Campos de nomes post não servem ao catálogo e não são fonte para glyph names/export.
- Reescrever offsets para buffer normalizado sfnt; nenhum byte/referência desse buffer vai a FontSlot/shaping/export. Extrair metadata e coverage com os owners atuais; retorno possui apenas FontInfo. Se qualquer parse/metadata falhar, declinar.
- Antes de aceitar resultado, comparar metadata/identidade final do handle e do path com o início; path trocado/truncamento/mtime/ctime diferentes declinam. Sem garantia inventada de snapshot atômico sob escritor concorrente: casos de input instável não qualificam números/paridade; fallback segue comportamento legado.
- Não cachear buffers, handles, FontInfo globalmente ou porpath; não deduplicar faces nem iniciar leitura seletiva sobre bytes embutidos/SharedFile. O resultado não muda falhas cacheadas/mmap final nem cria fallback eager de arquivo final.

## Sequência e aceitação autorizadas após ratificação

L0 canônico primeiro; testes no owner antes da implementação e RED runtime para projeção válida/caps/leitura reduzida, não erro de compilação ou threshold de velocidade. GREEN cobre fixtures TTF/OTF/variáveis, nomes/exceções/pitch/coverage, TTC índices e offsets absolutos, post versões/tamanhos/arrays/tail volumoso, duplicatas/ordem, tabelas opcionais/obrigatórias, assinatura/header/truncamento/count/overflow, limite em/fora do budget, falhas Read/Seek/reserva quando injetável, detecção de identidade alterada, File fallback e SharedFile com bytes retidos distintos do arquivo. Compare conjuntos membership/iter e metadados selecionáveis; DTO equality/limites são controles de mecanismo.

Comparar metadata/coverage de cada face estável instalada/embutida/fixture e ordem/book/slots à implementação anterior; inputs acima dos caps, rejeitados e SharedFile devem demonstrar fallback antigo. Filtrar FontInfo None exatamente como antes. Não tratar ausência ambiental como aprovação de fallback multiescrita.

Resselo somente desses dois pares; cargo build, release, workspace core/infra/shell, query CLI e crystalline-lint sem erros, owners sem V5/V6/V15/V26. Registrar warnings históricos e preservar todos os demais bytes produtivos/L0 recebidos, sem commit/reset/globalfix.

Medição combinada obrigatória: bytes solicitados (não I/O físico), catálogo, CLI completa e RSS contra release anterior congelado e vanilla pinado. Três rodadas/12 trios, permutações/warmups, CPU4/Rayon1/jobs1, gate30s/monitor/hashestáveis; mesmos oito fixtures/bytes/fonts da prioridade2, qualificação de texto/fonte/página/posição e raster de image/shapes/multiscript perante anterior. Repetibilidade <3% para afirmações precisas; negativas/dispersão preservadas. Proveniência porHEAD/working tree exata/diffstat/fullsources/UTC/argv/assets/bin SHA. Benchmark catálogo anterior usa coverage por intervalos já vigente; não volta ao callback por pontos nem soma ganhos históricos. Relatório/planodebitos/passoregistram conclusão ou hipótese refutada.
