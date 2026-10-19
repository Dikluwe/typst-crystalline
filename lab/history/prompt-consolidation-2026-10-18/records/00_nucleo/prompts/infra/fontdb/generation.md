# L0 — infra/fontdb/generation
Hash do Código: 0cd789b1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/watch-font-generation.toml sha256:391cd1604976262b1b6e019b802ed72e78782260d2788bb04999297559605509

**Layer:** L3
**Exclusive consumer:** `03_infra/src/fontdb/generation.rs`


## Medição anterior à decisão de retenção ratificada

lab/results/construction/diagnostics/watch-retencao-2026-10-07.md registra HEAD/árvore/binários/UTC,
edições reais, recuperação e contagem dinâmica de aquisição por ciclo.
lab/results/construction/diagnostics/watch-validacao-2026-10-07/measurement.json mede validação por
descoberta versus aquisição completa no mesmo estado; controls.json testa adição,
remoção, substituição, mtime restaurado, links e configuração. São custos isolados,
não ganho de CLI ou prova de intenção upstream. FontBook/slots/cobertura devem
preservar seleção, conteúdo, diagnóstico e render na linguagem; duração/Arc são
mecânica. Inferimos benefício possível, refutado por custo, stale resources,
regressão ou retenção ilimitada. Fixtures/proveniência/limites constam do relatório.

### Decisão ratificada sob ADR-0127 §2.1/§2.2

Dono ratificou com Confirmo o pacote lab/history/construction/records/00_nucleo/materialization/watch-catalogo-2026-10-07/manifest.json. Receipt: lab/results/construction/diagnostics/watch-catalogo-execucao-2026-10-07/ratification.json. Cada owner continua 1:1.

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

A descoberta completa é repetida em cada nova demanda de World salvo quando a
prova rápida estrita abaixo demonstra geração idêntica. Não manter lista antiga
de fontes/diretórios como prova isolada. Entradas novas, diretórios aninhados e
configuração de fontconfig devem passar pelos mesmos loaders na mudança.
Mudança de configuração que produz a mesma lista/identidade não exige regenerar
book imutável. Não usar fc-list, watchers novos, snapshots somente de
diretórios, cache em disco ou fingerprints por mtime sozinho. A observação é em
fronteira de recompilação, tal como o caminho fresco;
não prometer snapshot atômico de todo filesystem nem detectar mudanças depois da
fronteira sem novo ciclo. Mudança em fontes sozinha não adiciona evento novo.

## Prova rápida estrita de geração — correção interna 2026-10-08

Medição antes da decisão: `lab/results/construction/diagnostics/watch-incremental-item5-2026-10-08/measurements.json`
mede 54,38/54,75/54,73 ms de CPU mediana Tekt em escrita de mesmos bytes,
palavra e expansão estrutural, todos com texto/caixas equivalentes ao vanilla.
`correction-probe.json` mede três ciclos quentes por controle: documento sem
texto 1,51 ms total, texto simples 50,45 ms, `#set text(font: ...)` 48,43 ms
com 47,48 ms atribuídos a eval. O custo muda de fase conforme a primeira
demanda do catálogo. `generation.rs:27-29` reexecuta
`Database::load_system_fonts`; `font_session.rs:23-52` chama prepare em toda
demanda. Inferência: essa redescoberta é o principal candidato causal; uma
medição direta ou ausência de ganho pareado após a mudança a refuta. Estas
medianas não são parcelas aditivas de um perfil único.

Permitir uma prova rápida L3 **somente** para a configuração Linux fontconfig
padrão elegível e sem fontes de projeto explícitas. O preparador fresco anterior
continua o oráculo e o fallback integral. A prova rápida não materializa book,
slots ou faces e não substitui a descoberta fresca após qualquer mudança.
Ela deve recolher a configuração efetiva pelo mesmo parser e política de
caminho usados pelo `fontdb` pinado, os bytes dos arquivos de configuração
mesclados e ambiente/cwd relevantes. Ao estabelecer uma geração, percorrer
recursivamente **todas** as entradas dos diretórios efetivos, inclusive
entradas não-fontes, com ordem observada e orçamento global. Para a
verificação quente, reparsear a configuração, comparar arquivos/ambiente/cwd
e revalidar identidades Unix (dev/ino/tamanho/mode/mtime/ctime) das raízes,
de todos os diretórios, dos candidatos pelas extensões que `fontdb` aceita e
dos symlinks, inclusive destino e ausências. Diretórios usam ctime para
invalidar criação, remoção e rename de entradas; candidatos incluem fontes
ilegíveis ou inválidas que podem se tornar válidas sem alteração do diretório.
Symlinks são observados mesmo sem extensão de fonte, pois a extensão do alvo
canonizado pode ser aceita. A ausência de raiz é registrada para detectar
criação. Arquivos comuns de extensão não-fontes podem ser omitidos da chave:
alterar seus bytes sem renomear não muda a descoberta do `fontdb`. Isso é uma
prova de conteúdo/associação de descoberta, **não** snapshot apenas de
diretórios ou mtime. Qualquer impossibilidade de revalidar obriga descoberta
fresca.

Qualquer configuração customizada/ilegível, erro de travessia ou metadata,
instabilidade, entrada não suportada, limite ou plataforma diferente recusa o
caminho rápido. Limitar o inventário ao mesmo orçamento conservador de 16384
entradas e 1 MiB de paths; recusa não trunca a descoberta fresca. Guardar a
prova apenas junto da geração pareada corrente. Se a configuração nova e as
identidades retidas forem idênticas às anteriores, e os stamps da geração
anterior ainda forem válidos, a sessão pode reusar o Arc sem
`Database::load_system_fonts` nem novo `read_dir` das fontes. Prova
diferente/recusada usa
`prepare` e sua validação anterior, reconstruindo geração se preciso.
Revalidar a prova depois de preparar e construir, antes de guardar; não exigir
segunda travessia completa quando as identidades retidas permanecem estáveis.
Falha/instabilidade
não conserva a elegibilidade rápida. O instante da demanda continua a
fronteira observável; não acrescentar evento novo nem atrasar reconhecimento
de fonte/configuração alterada na recompilação seguinte.

Testes RED→GREEN devem cobrir mesma geração, conteúdo alterado com tamanho e
mtime restaurados, adição/remoção de fonte e diretório aninhado, symlink
retarget, configuração incluída alterada, configuração personalizada recusada,
limites, erro/instabilidade e concorrência. Comparar o catálogo e a seleção
contra o preparador fresco nos testemunhos; medir CPU/latência/RSS/startup do
watch com três rodadas e os cenários do item 5. Se a prova de frescor não for
completa ou o benefício não se repetir, retirar a conexão rápida.

Recusar retenção se lista exceder 16384 descritores ou soma dos bytes dos caminhos
exceder 1 MiB; aritmética checked e erro de alocação recusam quando recuperáveis.
Esses limites restringem só elegibilidade de cache; o loader fresco permanece
capaz de usar o catálogo completo. Não truncar fontes nem book para caber.

Testes inline RED→GREEN em fixtures: inalterado, adição/remoção, negativo→válido,
substituição atômica, mesmo tamanho e mtime restaurado, link retarget, mudança
fontconfig, projeto aninhado, limite e recusa por identidade instável. Separar
backend determinístico de descoberta de dependência ambiental. Cabe à sessão
reagir às recusas; este owner não materializa metadados nem possui FontBook.


## Expressões equivalentes e CI estrito — 2026-10-14

### Medição anterior à decisão

`03_infra/src/fontdb/generation.rs` nas linhas [356, 571, 574] contém as expressões apontadas pelo inventário
Clippy. `lab/results/construction/diagnostics/ci-simplificacoes-2026-10-14/before.json.gz`
conserva fonte/owner, spans, HEAD, working tree não commitado, UTC e diffstat;
red.log registra os lints selecionados como erros antes de alterar código.
Os tipos e usos foram conferidos no consumer e no toolchain 1.92.0 pinado.
Esta é manutenção da expressão Rust; não é medição nova de paridade upstream.

### Responsabilidade e aceitação

Nos fixtures de raiz ausente e diretório de projeto aninhado, emprestar o PathBuf como slice unitária por std::slice::from_ref. Não clonar o path só para formar o argumento. Preservar ordem de criação/alteração, rechecagens, identidade de arquivos e todas as asserções.

Correção interna em fluxo contínuo ADR-0127, sem API pública, default, fase,
unsafe, dependência ou supressão de lint novos. L0 precede código e resselo
fica restrito ao owner. Usar os controles funcionais existentes; não alterar
fixtures, expectativas, tolerâncias ou registrar testes que apenas espelhem
esta simplificação. Os lints selecionados devem passar como erros, seguidos
de suíte completa, build, all-targets, rustdoc, fmt, linhagem e inventário total.
