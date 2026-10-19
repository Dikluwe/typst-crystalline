# Prioridade 4 — investigação de retenção no watch

A aquisição repetida do catálogo está confirmada no binário atual. A proposta
para a próxima implementação é conservar somente uma geração validada dos
recursos de fontes e criar dados de arquivos novos por compilação. Esta rodada
é a investigação prevista na prioridade 4; **nenhuma otimização foi materializada**.
D10 permanece aberto. Não há ganho de velocidade certificado neste passo.

## Medição antes da decisão

Medição em 2026-10-07, horário de São Paulo; receipts UTC entre
2026-10-08 00:34:38 e 00:39:15. HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree não commitado**.
A lista exata dos arquivos alterados, `git diff HEAD --stat`, hashes dos
consumers/L0 e fontes/configurações de fontconfig estão nos snapshots gzip
`watch-retencao-2026-10-07/before.json.gz`, `after.json.gz`,
`audit-final.json.gz`, `content-before.json.gz` e `content-after.json.gz`.
`verification.json` confirma igualdade dos arquivos produtivos/L0, fontes e
binários entre os limites das medições. Alterações herdadas foram preservadas.

- Tekt: `target/release/typst`, SHA-256
  `96f22a1d63d51bfcc258ca2849563b037e66a67b7e34febaf4779da2dd146a4b`.
- Vanilla: `lab/typst-original/target/release/typst`, SHA-256
  `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`,
  pin `9f2b6e8715237cb086899a42873660fe744622e8`; proveniência de build P1489.
- Mesmo CPU permitido fixado, `RAYON_NUM_THREADS=1`, `SOURCE_DATE_EPOCH=0`,
  UTC e locale C.UTF-8. CLI/argv completos nos receipts. PDF, sem servidor,
  tagging e pretty desativados; vanilla `--jobs 1`, Tekt `--timings-json`.
- Três processos por documento/motor, dez edições por processo. Execuções
  sequenciais, alternando ordem dos motores entre rodadas. CPU é a soma do
  runtime das threads do próprio watcher via `/proc/.../schedstat`.
  Latência inclui polling e publicação. RSS/HWM vêm de `/proc/.../status`.
  Observações de RSS após publicar não são medição da memória de um futuro pool.
- Linha de base **exploratória**: sem monitor completo de carga concorrente,
  sem certificado de repetibilidade estrito e sem versão otimizada para comparar.
  Não derivar porcentagem de ganho, nem atribuir todo o delta ao catálogo.

| Edição que altera conteúdo | CPU por ciclo Tekt / vanilla (ms) | Edição até PDF Tekt / vanilla (ms) | RSS Tekt / vanilla (MiB) |
|---|---:|---:|---:|
| Hello, revisão no texto | 109,10 / 1,55 | 135,11 / 102,16 | 39,88 / 29,70 |
| Tabela 320 células, revisão na primeira célula | 143,68 / 8,01 | 171,07 / 108,69 | 40,77 / 46,11 |

São medianas das trinta edições por caso/motor. O pico HWM observado na mesma
amostragem foi 43,79 / 29,74 MiB no Hello e 46,25 / 46,11 MiB na tabela.
No Tekt, o timer interno total teve mediana 108,43 ms no Hello e 142,55 ms na
tabela; são fases do pipeline, não latência de publicação. O timer vanilla de
watch exclui descoberta inicial de fontes e não serve de comparação end-to-end.

A primeira sonda de tabela alterava somente comentários: CPU 166,07 / 2,54 ms,
latência 190,71 / 102,99 ms. Foi conservada em `results.json`, mas a decisão usa
o testemunho de **edição de célula**, em `content-results.json`. As duas séries
possuem workloads e momentos diferentes; sua diferença não é ganho de código.
No Hello inicial, a mediana de parede desde criar o processo até PDF foi
132,42 / 145,37 ms, contra os ciclos acima. Não misturar startup com edição.

## Contagem e comportamento

`04_wiring/src/watch.rs:63` chama a compilação a cada ciclo;
`04_wiring/src/compile.rs:277` cria SystemWorld e `:288` configura as fontes.
`03_infra/src/world/fonts.rs:104` inicializa o catálogo uma vez por World.
O GDB, executado **separadamente dos tempos**, contou aquisições de
`typst_infra::fontdb::load_system_fonts`: **1, 2, 3, 4 cumulativas** para a
compilação inicial e três edições. Receipt: `debugger-escalated.json`.
A primeira tentativa no sandbox recebeu `ptrace: Operation not permitted`;
a repetição com execução escalada autorizada funcionou. Não foram contados
constructors de World nem aquisições individuais de bytes/FontSlot; os anchors
acima são evidência estática dessas operações, não contagem dinâmica delas.

Os casos comportamentais em `results.json` passaram em ambos os motores:
edição do principal, import e read; remoção e recriação do read; erro de sintaxe
e recuperação; import inexistente e criação posterior. Nas falhas, o SHA do
último PDF publicado permaneceu intacto. Nas recuperações, o texto esperado
voltou a aparecer. Os PDFs dos ciclos temporizados tiveram texto extraído igual
entre motores; isso qualifica este testemunho, não toda a paridade de render.

`resources.json` registra a alteração de PNG vermelho para azul: a imagem
publicada mudou nos dois motores. As sondas raster coincidiram nos dois estados;
hashes são receipts diagnósticos, não regra arquitetural de igualdade de bytes.
A tentativa inicial com SVG revelou a limitação existente do Tekt
`SVG images are not supported yet`; ela não foi corrigida nesta investigação.

Uma fonte de projeto com família exclusiva foi derivada de Noto Sans por
fontTools 4.46.0, apenas no fixture temporário. Seu SHA está no receipt.
A primeira tentativa com Noto Sans sem renomear era mascarada pela fonte do
sistema: não serviu para concluir sobre invalidação e foi descartada como prova.

Na sonda válida, editar somente a fonte não disparou publicação nos 300 ms de
observação em nenhum motor. Ao editar o principal em seguida:

- **Tekt:** a substituição por outra família foi reconhecida e ocorreu fallback;
  remover esse arquivo e adicionar a família exclusiva em outro caminho fez
  essa família voltar a ser usada.
- **Vanilla pinado:** continuou usando a fonte original retida, inclusive após
  substituição/remoção. O nome presente no último PDF não prova redescoberta:
  nessa execução ele nunca deixou de estar presente.

A retenção upstream é mecânica. Manter a fonte antiga após sua substituição não
vira requisito do Tekt (ADR-0107). Uma otimização nossa deve preservar o
comportamento atual de atualizar recursos na próxima recompilação.

## Decisão e fronteiras da implementação

**Inferência:** eliminar descoberta/metadados repetidos é um candidato forte,
porque a aquisição reaparece dinamicamente e o trabalho de recompilação simples
é relevante. Seria refutada como otimização útil se a validação da geração
custasse tanto quanto adquirir, se não reduzisse CPU/latência, se aumentasse
memória sem limite ou mantivesse recursos antigos. A contagem sozinha não mede
a contribuição causal em milissegundos.

O recurso a conservar é a geração **pareada** de FontBook/FontSlots/coverage;
fontes carregadas pertencem a essa geração. Seus índices não atravessam troca
de geração. FontSlot retém inclusive resultados negativos em OnceLock: uma
falha precisa expirar quando o recurso muda. Não compartilhar caches de métricas,
shaping ou contextos por slot sem suas próprias provas de identidade.

Sources, read_cache, path_to_id, dependências e resultados negativos de arquivos
continuam novos por compilação. Inputs são configuração do processo e foram
mantidos iguais; mudança de opções é fronteira de sessão. Host de plugins,
downloader/pacotes, data/relógio, Library e caches de eval/layout permanecem com
seus contratos atuais. Não foi feita série de mutações de plugins/pacotes/inputs.
Nenhuma nova memoização ampla nem novo unsafe foi autorizado por esta decisão.

Antes de escrever código, a próxima experiência deve medir uma validação de
geração que observe arquivos **e diretórios** de descoberta, configuração
fontconfig e caminhos de projeto. Observar somente as fontes já descobertas
perderia adições; observar somente mtime de diretório perderia substituições.
A lista de `fc-list` não é garantia de equivalência com a descoberta fontdb:
`fontdb/src/lib.rs:356-387,503-565` percorre diretórios e configuração próprios.
Erro ou incerteza na validação deve cair para aquisição fresca, preservando ordem,
fontes adicionadas/removidas e associação book/slots. Medir também seu custo.

Depois disso, redigir o contrato concreto de sessão e invalidação nos owners:
`00_nucleo/prompts/infra/system-world.md`,
`00_nucleo/prompts/infra/system-world/fonts.md`,
`00_nucleo/prompts/infra/system-world/world_impl.md`,
`00_nucleo/prompts/wiring/compile.md` e
`00_nucleo/prompts/wiring/watch.md`; um eventual módulo novo terá owner L0 único.
Os L0 vigentes foram lidos e conservados: hoje especificam lifetime por World.
Não escolher reset indiscriminado nem alterar um trait L1 para contornar isso.
Uma assinatura pública nova passará pelo gate ADR-0127 com proposta concreta;
**esta investigação não solicita confirmação de uma API ainda não especificada**.

## Reprodução e verificação

`python3 00_nucleo/diagnosticos/watch-retencao-2026-10-07/run.py` mede Hello e
comentários de tabela; `measure-content.py` mede conteúdo de célula; `audit.py`
só faz sondas comportamentais e tentativa de debugger. `debugger-only.py` requer
um ambiente que permita ptrace e deve rodar separado do benchmark.
`run-measured.py` conserva o harness exato da primeira medição; `run.py` ganhou
apenas limpeza do import temporário antes de repetir os testes de criação.
Os arquivos gerados e watchers pertencem ao harness; ele encerra só seus processos.

`verification.json` contém hashes dos receipts/scripts. Sem alterações em
produção ou L0; build e testes de produção não foram repetidos para documentação.
Os débitos de incremento amplo, reuso de células e SVG permanecem independentes.
