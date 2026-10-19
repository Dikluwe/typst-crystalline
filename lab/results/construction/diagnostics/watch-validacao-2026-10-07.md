# Validação de geração de fontes — continuação da prioridade 4

A validação por **descoberta atual + identidade dos arquivos** custou menos que
a aquisição completa nas duas séries. A proposta de retenção foi preparada em
`00_nucleo/materialization/watch-catalogo-2026-10-07/`; aguarda ratificação do
contrato público. Produção, L0 vigentes e binário CLI continuam intactos.

## Medição antes da decisão

O probe diagnóstico usa `fontdb 0.23.0` e os loaders produtivos atuais, build
release offline. `probe/Cargo.toml`, `probe/Cargo.lock`, fontes e seus hashes são
parte do receipt. A primeira resolução offline tentou uma versão de zerocopy-derive
não disponível; foi usado o lock já disponível do probe de prioridade 3, atualizado
pelo Cargo apenas para este pacote. Não houve acesso à rede nem mudança no lock da raiz.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
`watch-validacao-2026-10-07/before.json.gz` e `after.json.gz` registram UTC exato,
lista de arquivos alterados, `git diff HEAD --stat`, hashes produtivos/L0,
fontes/configuração e binários. `dirty-before.patch.gz` conserva o diff.
A receita e o hash do executável do probe estão em `measurement.json`.
Os binários CLI permanecem Tekt SHA-256
`96f22a1d63d51bfcc258ca2849563b037e66a67b7e34febaf4779da2dd146a4b` e vanilla
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, pin
`9f2b6e8715237cb086899a42873660fe744622e8`. Nenhum deles foi temporizado neste probe.

Cada série tem três rodadas de doze pares, alternando ordem, CPU fixado,
`RAYON_NUM_THREADS=1`, com monitor separado de processos conhecidos a cada 50 ms.
Nenhuma concorrência reconhecida foi detectada. Isso não é garantia de toda a
máquina ociosa. O tempo inclui descarte das estruturas temporárias; no caminho
validação inclui comparação da chave. Fonte de configuração do fontdb foi
registrada com SHA no receipt.

| Série | Validação, mediana (ms) | Aquisição completa, mediana (ms) |
|---|---:|---:|
| Inicial, oito mutações + inalterado | 65,56 | 128,62 |
| Final, controles de links/configuração adicionados | 49,26 | 100,04 |

Os controles novos alteram somente o ramo de validação comportamental do harness;
o ramo de timing é o mesmo. A variação absoluta entre séries é significativa;
**ambas são indicativas, sem certificado estrito de repetibilidade**. Não chamar
essa diferença de ganho obtido no compilador, nem combinar com tempos históricos.
A primeira série e seu código exato estão em `initial/`. A final está em
`measurement.json`; rodadas finais: validação 50,46 / 51,71 / 48,29 ms e aquisição
102,99 / 103,25 / 98,17 ms. Ainda não existe comparação com uma sessão produtiva.

## O que foi validado

O probe cria Database e executa a descoberta atual do sistema. Conserva, na
mesma ordem, path/índice das faces Source::File; Binary é ignorado e SharedFile
recusa elegibilidade. Também usa `fonts::discover_fonts` para fontes de projeto.
Captura path/índice/dev/inode/len/mode/mtime(sec,ns)/ctime(sec,ns) de arquivos
regulares e recheca os stamps depois de percorrer a lista. Qualquer diferença ou
falha de metadata recusa; não há hash fraco nem uso de mtime isolado.

O catálogo de sistema desta máquina apresentou os mesmos caminhos e índices
ordenados na descoberta validada e no loader produtivo completo. Isso não garante
que toda face descoberta seja selecionável em qualquer catálogo: o filtro de
FontInfo continua necessário e permanece no owner produtivo atual.

`controls.json` tem treze controles assertados: adição/remoção de fonte de sistema;
substituição de mesmo tamanho com mtime restaurado; substituição atômica;
adição/remoção e subdiretório de projeto; arquivo inválido tornando-se válido;
inserção/retarget de link; stamp ausente recusado; alteração de diretório em
FONTCONFIG_FILE; catálogo inalterado elegível. Fonte de fixture e mutações são
conservadas no código do probe, snapshots e receipts. Esses controles provam
validade de chave nos testemunhos, **não equivalência de texto/render nem cache
produtivo**, cuja aceitação permanece na materialização proposta.

Não é necessário copiar a interpretação de fontconfig para outro walker. Repetir
a descoberta real vê diretórios/configuração atuais, inclusive entradas antes
inexistentes, e conserva ordem. Ainda custa ler/interpretar fontes para o Database;
o que a retenção pode eliminar é a extração posterior de FontInfo/cobertura e a
reconstrução de FontSlots, mantendo os slots carregados de uma geração válida.
O probe mede validação isolada e aquisição isolada; não implementa essa retenção.

## Decisão depois da medição

**Inferência:** a alternativa tem custo suficientemente menor nos testemunhos para
justificar uma implementação controlada. Seria refutada por validação mais cara,
nenhum ganho repetível no watch, recursos antigos, regressão de outputs ou memória
retida excessiva. O benchmark não atribui causalidade a todo o delta Tekt/vanilla.

A sessão proposta armazena uma geração apenas. Cada World continua novo para
sources/bytes/dependências e serviços atuais. O builder continua lazy; validação
acontece na primeira demanda de catálogo na fase existente. Uma geração nova usa
Database/slots já descobertos, evitando uma segunda descoberta de sistema; a
rechecagem de identidades ocorre antes do reuso e depois da materialização.
Instabilidade/recusa descarta elegibilidade e usa a aquisição fresca já existente.
SharedFile segue no caminho fresco que preserva bytes retidos, como prioridade 3.

Fora de Unix a retenção recusa. A proposta limita só elegibilidade a 16384
stamps e 1 MiB de bytes de caminhos; exceder não trunca fontes, apenas usa a
rota fresca. São limites conservadores propostos, ainda não implementados nem
exercitados pelo probe. Erros/limites/postcheck/concorrência exigem testes RED→GREEN
produtivos antes de chamar a implementação concluída.

O instante observado continua a fronteira de recompilação. Não promete snapshot
atômico de todo filesystem nem novo evento de font-watch: fontes alteradas ficam
visíveis na próxima recompilação, como o Tekt atual. Não conserva comportamento
upstream de manter recurso substituído obsoleto. Nada de caches globais, disco,
novo unsafe, alterações em World L1 ou memoização de eval/layout neste lote.

## Pacote concreto e gate

Sete owners candidatos, cada um com consumer exclusivo: world, world/fonts,
world/font_session (novo), fontdb, fontdb/generation (novo), wiring/compile e
wiring/watch. Núcleo candidato pinado `watch-font-generation.toml` compartilha
somente as invariantes de geração/frescor; nunca é @prompt de código.
Os candidatos ficam na materialization até ratificação, sem criar órfãos nos L0
ativos. Hashes de código antigos nos candidatos existentes são históricos até
resselo; novos consumers estão PENDENTE. Manifesto conserva bases/candidatos e pins.

API proposta: tipo opaco `typst_infra::world::FontCatalogSession`, constructor
`FontCatalogSession::new()` e builder `SystemWorld::with_font_catalog_session`
recebendo `Arc<FontCatalogSession>` e caminhos de fontes. Compile comum conserva
seu builder atual; watch passa a sessão entre ciclos. Todos os outros owners
produtivos permanecem fora do pacote.

ADR-0127 §2.1 exige confirmação de contrato público; §2.2 também rege a mudança
proposta do caminho de aquisição padrão do watch. O próximo passo após ratificação
é instalar L0, escrever testes que falham, implementar, ressellar só esses owners,
build/lint/testes e medir watch contra o binário anterior congelado, com corpus
real de arquivos/fontes/recuperação, RSS e startup separado. Não considerar D10 fechado
até isso passar. Ownership ativo V15/V26 verificado sem violations; o grafo candidato
foi auditado separadamente e ainda não está instalado.
