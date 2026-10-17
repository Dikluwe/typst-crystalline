# Item 2 — custo fixo residente das fontes

## Medição anterior à decisão

Estado inicial: HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. `custo-fixo-fontes-item2-2026-10-08/before-state.json` guarda UTC, lista de ficheiros alterados, `git diff HEAD --stat`, bytes e hashes de código/L0 e binário do instante anterior à correção. Os JSON `probe-results-before.json` e `cli-results-before.json` guardam amostras, comandos, fontes, fixtures, ambiente e hashes. O pin vanilla é `9f2b6e8715237cb086899a42873660fe744622e8`, com binário SHA-256 registrado nos resultados; não se usou a string `--version` como prova.

Sonda de processo separado, três repetições por modo, feita com as APIs existentes e um alocador diagnóstico que soma tamanhos requisitados ainda vivos. RSS vem de `/proc/self/status`; mappings vêm de `smaps`. Esses números medem objetos desse processo diagnóstico, não se somam diretamente ao RSS do CLI. Alocação requisitada não inclui overhead do alocador, páginas livres retidas, `.rodata` nem mmap de ficheiros. O ensaio da CLI usa `%M` para pico RSS de processos novos, três blocos com quatro pares por fixture, ordem alternada, um aquecimento por bloco, CPU fixa, Rayon 1, epoch 0 e fontes comuns. Sem build/teste/profiler concorrente durante as amostras.

| Componente, antes da correção | Medição direta |
|---|---:|
| Assets embutidos, 17 faces | 9.682.300 bytes (9,24 MiB) |
| Buffer próprio do catálogo embutido, vivo | ~9,24 MiB requisitados |
| Páginas do executável tocadas pelo catálogo completo | ~9,3 MiB adicionais na sonda |
| `fontdb::Database` de sistema, ainda viva | ~0,49 MiB requisitados |
| `fontdb::Database` após `drop` | ~0,001 MiB requisitados |
| Metadados das 1.112 faces de sistema, sem slots/cobertura | ~0,128 MiB |
| Metadados + slots, sem cobertura | ~0,357 MiB |
| Catálogo de sistema produtivo, com cobertura | ~2,542 MiB |

A diferença de ~2,185 MiB entre catálogo produtivo e metadados+slots localiza a maior retenção do sistema na cobertura. A sonda independente que enumera codepoints e constrói `Coverage` encontrou 2,314 MiB para metadados+cobertura, coerente com essa diferença, mas usa o caminho antigo e não substitui o loader produtivo. O catálogo produtivo reportou 9.487.240 pontos cobertos somados entre faces; isto não representa 9,5 milhões de bytes persistentes, porque `Coverage` usa runs. A `fontdb::Database` não fica viva depois de `load_system_fonts` (`fontdb.rs:52-89`); o RSS após seu `drop` pode reter páginas livres do alocador.

No binário produtivo pré-correção, `embedded_fonts.rs:92-94` fazia `data.to_vec()` para cada asset válido e `fonts.rs:115-123` conservava todos os Vetores. A cópia existia além das páginas estáticas do executável. O vanilla pinado passa os slices para `Bytes::new(data)` em `lab/typst-original/crates/typst-kit/src/fonts.rs:136-141`; `typst-library/src/foundations/bytes.rs:48-62` documenta que `&'static [u8]` é o backing sem Vec. Isto classifica o problema como duplicação mecânica, não divergência de linguagem; não se infere intenção histórica de uma medição de RSS.

A CLI inicial confirmou o problema em trabalho equivalente para texto simples: `pipeline-single-font` teve 41,83 MiB de pico mediano no Tekt e 29,21 MiB no vanilla; ambos extraíram o mesmo texto e o raster coincidiu. O controle `shapes`, sem demanda de fontes, teve 19,75 MiB no Tekt. Esses picos não provam que toda a diferença de 12,62 MiB vem de um único Vec: incluem layout, exportação, páginas estáticas e comportamento do alocador.

## Decisão e implementação

Correção interna pelo ADR-0127, sem campo/variante/assinatura pública, flag, default ou fase nova. Os L0 `prompts/infra/fonts.md` e `prompts/infra/embedded_fonts.md` foram atualizados antes do código e ressellados. `FontSlot` mantém o construtor público `new_embedded(Vec<u8>)` e seu compartilhamento Arc; o loader dos assets usa origem estática interna. `source_bytes` empresta a fatia. O `get` valida a face e cria bytes próprios apenas para a face efetivamente pedida, guardando sucesso/falha no OnceLock. Reconstrução de coleções, fontes de disco/mmap e a variante `Font` existente conservam seus caminhos.

O teste causal de ponteiro estático falhou antes (`red-test.txt`) e passou depois (`green-test.txt`). Após a mudança, a sonda reteve apenas ~5,7 KiB requisitados com o catálogo embutido e nenhum `get`; quando ela força `get` de **todas** as 17 faces, volta a ~9,24 MiB, como esperado. O catálogo de sistema ficou em ~2,574 MiB, cerca de 0,032 MiB acima por conta da representação maior dos slots; a parcela estimada de cobertura continua ~2,185 MiB. A sonda pode variar em páginas residentes entre builds e não é o gate de produto.

## Resultado no CLI e verificação

| Caso | Tekt antes | Tekt depois | Vanilla depois |
|---|---:|---:|---:|
| Texto simples (`pipeline-single-font`) | 41,83 MiB | **27,15 MiB** | 29,22 MiB |
| Formas sem fonte (`shapes`) | 19,75 MiB | 19,75 MiB | 21,12 MiB |

Texto simples reduziu 15.028 KiB de pico mediano (14,68 MiB, ~35,1%). As medianas por bloco Tekt foram 42.826/42.898/42.836 KiB antes e 27.824/27.688/27.828 KiB depois. Não usar a subtração do RSS para declarar 14,68 MiB de objetos `Vec` vivos: a sonda demonstra 9,24 MiB de buffers retirados e mostra que o novo loader toca menos páginas estáticas antes da seleção. Os dados completos estão em `cli-results-before.json` e `cli-results-after.json`.

`cargo test -p typst-infra --lib`: 1.254 passaram, 2 ignorados. Builds debug/release e `crystalline-lint .` concluíram com código zero; o linter ainda emite 158 avisos V5 do working tree herdado, nenhum nos dois consumers alterados. `git diff --check` passou. O controle de saída em `qualification.json` verificou páginas/palavras/caixas e raster a 36 dpi: `context100`, `context300` e `plain300` conservaram caixas exatamente e raster idêntico ao PDF Tekt anterior ao item 2; texto simples diferiu no máximo 0,000002 pt nas caixas frente ao vanilla e teve raster/texto visível iguais; `shapes` teve raster igual e não tem palavras. A comparação não certifica paridade global de renderização.

## Duplicações e limites restantes

O Tekt ainda materializa uma cópia da face embutida **selecionada** em `Font::Vec`; a API pública atual não transporta diretamente `&'static [u8]`. Eliminar essa última cópia exigiria alterar o contrato público de `Font` e passar pelo gate do ADR-0127 antes do código. A cobertura de sistema (~2,19 MiB) permanece residente para consultas de fallback; removê-la sem uma política substituta mudaria trabalho entre aquisição e consulta. Fontes de disco usam mmap lazy, e coleções extraídas precisam de buffer independente. Não há prova aqui de outro catálogo de fontes permanentemente duplicado, nem perfil por objeto vivo no processo CLI completo. O RSS residual do texto inclui trabalho não atribuído às fontes; item 3 trata memória contextual após a equivalência do item 1.

`custo-fixo-fontes-item2-2026-10-08/completion.json` conserva UTC final, HEAD, working tree, `git diff HEAD --stat`, hashes de fontes/código/L0/binários e recibos das validações. Nenhum commit foi criado. Tempo antes/depois não foi medido nesta bateria de memória; a decisão conserva o ganho de RSS e a equivalência de saída observada, sem afirmar ganho de velocidade.
