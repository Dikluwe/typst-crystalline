# L0 — infra/fontdb/generation
Hash do Código: PENDENTE

**Layer:** L3
**Exclusive consumer:** `03_infra/src/fontdb/generation.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/watch-font-generation.toml sha256:391cd1604976262b1b6e019b802ed72e78782260d2788bb04999297559605509

## Medição anterior à decisão de retenção (candidato não ratificado)

00_nucleo/diagnosticos/watch-retencao-2026-10-07.md registra HEAD/árvore/binários/UTC,
edições reais, recuperação e contagem dinâmica de aquisição por ciclo.
00_nucleo/diagnosticos/watch-validacao-2026-10-07/measurement.json mede validação por
descoberta versus aquisição completa no mesmo estado; controls.json testa adição,
remoção, substituição, mtime restaurado, links e configuração. São custos isolados,
não ganho de CLI ou prova de intenção upstream. FontBook/slots/cobertura devem
preservar seleção, conteúdo, diagnóstico e render na linguagem; duração/Arc são
mecânica. Inferimos benefício possível, refutado por custo, stale resources,
regressão ou retenção ilimitada. Fixtures/proveniência/limites constam do relatório.

### Decisão proposta sob ADR-0127 §2.1/§2.2

Aguardar ratificação do pacote 00_nucleo/materialization/watch-catalogo-2026-10-07/manifest.json
antes de instalar este candidato ou escrever código produtivo. Cada owner continua 1:1.

## Preparação e chave de validade, somente L3 crate-private

Não oferece API pública. PreparedDiscovery possui Database do fontdb, os slots
atuais de projeto e chave comparável. Preparar invoca Database::load_system_fonts
atual e fonts::discover_fonts nos caminhos fornecidos, preservando exatamente as
ordens e duplicados. A chave inclui configuração de caminhos fornecidos e lista
ordenada de origem/índice/identidade; IDs transitórios do Database não entram.

Para Source::File e cada slot de projeto, em Unix capturar path e índice,
dev/ino/tamanho/mode/mtime-sec-nsec/ctime-sec-nsec de arquivo regular. Rechecar as
identidades depois de preparar e novamente antes de reutilizar/publicar geração.
Uma leitura de metadata errada, arquivo não regular, diferença entre amostras ou
Source::SharedFile recusa preparo/reuso; SharedFile conservado pelo caminho fresco
continua com os bytes retidos originais. Binary não persistente permanece ignorado.
Fora de Unix recusar sem implementar identidade fraca substituta.

A descoberta é repetida em cada nova demanda de World: não manter lista antiga de
fontes/diretórios. Assim entradas novas, diretórios aninhados e configuração de
fontconfig passam pelos mesmos loaders. Mudança de configuração que produz a mesma
lista/identidade não exige regenerar book imutável. Não usar fc-list, watchers
novos, snapshots somente de diretórios, cache em disco ou fingerprints por mtime
sozinho. A observação é em fronteira de recompilação, tal como o caminho fresco;
não prometer snapshot atômico de todo filesystem nem detectar mudanças depois da
fronteira sem novo ciclo. Mudança em fontes sozinha não adiciona evento novo.

Recusar retenção se lista exceder 16384 descritores ou soma dos bytes dos caminhos
exceder 1 MiB; aritmética checked e erro de alocação recusam quando recuperáveis.
Esses limites restringem só elegibilidade de cache; o loader fresco permanece
capaz de usar o catálogo completo. Não truncar fontes nem book para caber.

Testes inline RED→GREEN em fixtures: inalterado, adição/remoção, negativo→válido,
substituição atômica, mesmo tamanho e mtime restaurado, link retarget, mudança
fontconfig, projeto aninhado, limite e recusa por identidade instável. Separar
backend determinístico de descoberta de dependência ambiental. Cabe à sessão
reagir às recusas; este owner não materializa metadados nem possui FontBook.
