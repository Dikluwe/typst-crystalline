# P1339 — revisão delimitada dos L0 de indexação

Resultado: não identifiquei bloqueador substantivo remanescente nos parágrafos
de indexação examinados, na revisão identificada pelos hashes abaixo. A
captura de Strong/Emph e a ordem de combinadores foram completadas pelo autor
principal durante a revisão e relidas antes deste resultado. Isso permite
prosseguir com a preparação dos gates do recorte; não aprova implementação,
não sela contrato e não afirma paridade bilateral.

## Regime, autoridade e escopo

Regime: **executado sem atestação de isolamento**. Executor:
`/root/p1339_index_l0_review`, no workspace compartilhado do projeto. A skill
`tekt-materializacao-segregada` e suas referências orientaram a delimitação:
papel de revisor L0, fontes baseline permitidas, ausência de implementação
candidata como entrada, e escrita apenas deste diagnóstico. O ambiente permite
tecnicamente outras escritas; a restrição foi operacional, não atestada.

Entradas permitidas: instruções recebidas, CLAUDE/ADRs, fontes baseline
`01_core/src`, Prompts L0 e artefatos `diagnosticos/p1339-where*`. Não houve
leitura de conteúdo de materialization/context, nem fonte candidata. Não
escrevi os L0 julgados, contrato, oráculo ou solução. Comunicação ao autor
principal apontou a lacuna inicial de ordenação de Or; ele alterou o L0 e o
revisor releu a alteração. Logo este documento identifica a revisão final
examinada, não um congelamento anterior a toda interação entre autoridades.

Escopo: os parágrafos P1339 dos owners `entities/elements/strong.md`,
`entities/elements/emph.md`, `compiler/introspect/extract_payload.md`,
`compiler/introspect/locatable.md`, `compiler/introspect.md`,
`entities/introspector.md`, `compiler/layout.md` e
`compiler/eval/selector_matching.md`. A leitura do grande owner introspect foi
focal, abrangendo esses parágrafos, a cláusula anterior de snapshot e as fontes
baseline correspondentes; não constitui auditoria integral do seu histórico.

Counters filtrados têm outro desenho em curso. A ausência desse desenho neste
recorte não foi tratada como defeito da indexação, nem se inferiu autorização
para alterar seus métodos. Tampouco foram executados build, lint, testes,
mutantes ou comparação com candidato.

## Medição anterior ao julgamento

Baseline: HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não
commitado. Inspeção final de estado/hashes em `2026-09-10T01:12:36Z`.
`git diff HEAD --stat -- 01_core/src` não produziu saída. O diffstat completo
naquele instante foi:

```text
 .../prompts/compiler/eval/bindings/field_access.md |  43 +++++++++
 .../compiler/eval/bindings/value_methods.md        |  89 ++++++++++++++++++
 .../compiler/eval/call_dispatch.md                |  53 +++++++++++
 .../compiler/eval/operators/equality.md           |  44 +++++++++
 .../compiler/eval/repr.md                         |  38 ++++++++
 .../compiler/eval/rules.md                        |  34 +++++++
 .../compiler/eval/selector_matching.md            |  78 ++++++++++++++++
 .../compiler/introspect.md                       | 100 +++++++++++++++++++++
 .../compiler/introspect/extract_payload.md        |  27 ++++++
 .../compiler/introspect/locatable.md              |  24 +++++
 .../compiler/layout.md                           |  27 ++++++
 .../compiler/stdlib/foundations/query.md           |  31 +++++++
 .../compiler/stdlib/foundations/selector.md        |  35 ++++++++
 .../entities/element_payload.md                  |  88 ++++++++++++++++++
 .../entities/elements/emph.md                     |  25 ++++++
 .../entities/elements/strong.md                   |  26 ++++++
 .../entities/introspector.md                      |  31 +++++++
 .../entities/selector.md                          |  82 +++++++++++++++++
 .../entities/show.md                              |  48 ++++++++++
 19 files changed, 923 insertions(+)
```

Os caminhos abreviados acima são relativos a `00_nucleo/prompts/`. O diffstat
não inclui diagnósticos untracked. As medições vanilla utilizadas são as do
recibo de ocorrência pinado abaixo, não foram reexecutadas nesta revisão;
seu baseline é `a51e02804`, binário `/usr/local/bin/typst`, com identidade,
horários, fontes de fixtures e resultados preservados no próprio recibo.

- `01_core/src/compiler/introspect.rs:1278` condiciona Location ao payload;
  `:1345` registra label/payload e `:1350` inicia a construção da entrada
  canônica. Strong/Emph em `:1399` apenas descem no body no baseline.
- `01_core/src/compiler/introspect.rs:789` registra labels antes do match de
  payload; `:1086` restringe o parent index aos payloads reais; `:1947` emite
  End usando o hash do Content integral. Esses locais comportam um marcador
  unit sem identidade genérica de função ou novo Kind.
- `01_core/src/compiler/introspect.rs:1613` transmite a label ao alvo e
  `:1898` conserva essa label ao atravessar Styled. Strong/Emph passam None
  ao próprio body (`:1399`), evitando atribuir a label exterior aos filhos.
- `01_core/src/entities/introspector.rs:559` até `:896` contém o impl concreto
  completo. Seus acessos são aos campos públicos da struct e métodos dos
  sub-stores. O trait e a construção/injeção estão fora desse bloco.
- `01_core/src/entities/introspector.rs:659` concatena os resultados de Or
  por ramo e remove repetições pela primeira aparição. Já
  `00_nucleo/diagnosticos/p1339-where-occurrence-probe.md:61` e `:72` registram
  união em ordem documental de funções intercaladas. Ordenar somente cada
  folha Element não corrigiria essa diferença observável.
- `01_core/src/compiler/layout/mod.rs:1484` concentra o avanço de Locator e
  Position; `:1525` chama esse ponto antes do dispatch. Strong/Emph possuem
  dispatch próprio em `:1894`, enquanto a medição em `:2640` usa a rota de
  tamanho. Essa distinção exige teste integrado, não avanço adicional em
  todos os braços que mencionem Strong/Emph.
- `01_core/src/compiler/eval/bindings/field_access.rs:1024` projeta body do
  Strong cru e marca delta ausente; `01_core/src/compiler/stdlib/structural/markup.rs:34`
  rejeita named. `01_core/src/entities/elements/strong.rs:28` só armazena body.
  A sonda `p1339-where-occurrence-probe.md:69` distingue essa projeção crua da
  ocorrência consultada com delta realizado.

## Julgamento delimitado da revisão final

**Ocorrência e ownership coerentes.** Os novos contratos de Strong/Emph
delegam o marcador pelo método Element existente e deixam identidade e
snapshot no compiler. Os dispatchers `extract_payload.md:124` e
`locatable.md:155` promovem somente nós próprios e preservam Styled como
transporte. `compiler/introspect.md:68` cobre Start/End, label, entrada,
parent index e descida única; o marcador não é tratado como função nem Kind.
Isso está dentro do membership aprovado em
`p1339-where-payload-approval.json`, sem novo campo de entidade.

**Transferência integral do impl coerente.**
`entities/introspector.md:500` e `compiler/introspect.md:117` concordam na
transferência ao consumer já existente. Trait, struct, derives, construtores
e injeção continuam no owner de dados. Não se demonstrou necessidade de nova
API, callback no dado, import reverso ou módulo adicional para alojar o bloco.
As cláusulas de preservação de métodos e testes de algoritmo delimitam a
mudança; não autorizam aproveitar a transferência para alterar counters.
Esta é avaliação de viabilidade pela fonte, não prova de compilação.

**Snapshot e labels explicitados.** Na primeira leitura faltava a captura
Strong/Emph concreta. A revisão final em `compiler/introspect.md:84`–113
agora exige Some completo no Start, body semântico, delta default do Strong e
label causal por último; não insere func/location nos campos. Em
`compiler/eval/selector_matching.md:209`–218, Some é autoridade completa,
ausência de campo não faz fallback, None mantém a projeção legada, e igualdade
continua no owner de linguagem. A nova cláusula é a especialização P1339 da
regra anterior de famílias sem snapshot; não exige alterar StrongElem para
guardar campo realizado.

**Delta/set fora do lote está declarado.**
`compiler/introspect.md:105`–113 exclui named delta e set strong(delta),
reconhece a dívida medida e impede aceitar futuro produtor variável enquanto
fixa silenciosamente 300. Assim o default capturado é justificado no domínio
representável declarado. Os casos de delta explícito/set da sonda não podem
ser apresentados como conformidade cristalina por esta revisão.

**Ordem corrigida no texto julgado.** A lacuna inicialmente identificada no
Or legado foi comunicada ao autor. `compiler/introspect.md:132`–139 passou a
normalizar resultados únicos em ordem documental quando a árvore contém
Element, preservando árvores inteiramente legadas. Essa cláusula cobre o
contraexemplo de funções intercaladas da sonda, inclusive quando Or está
aninhado em And/Within. Não persiste a pendência de ordenar apenas folhas.

**Sincronização possui obrigação adequada, ainda sem evidência executada.**
`compiler/layout.md:40`–52 exige o ponto canônico de avanço, evita avanço nas
medidas, preserva replay/rollback e pede associação da Position ao nó correto.
Os critérios incluem corpo vazio, nesting, labels e elementos posteriores.
O texto não confunde payload/locatable unitários verdes com prova da sequência
completa; não exige mudança da fase ou do render.

Classificação ADR-0107/0108: identidade de função, body, label, multiplicidade,
campos realizados e ordem pública de query são linguagem. Variante unit,
localização do impl Rust e ordenação por Location são escolhas internas.
Inferência desta revisão: os carriers baseline bastam para o recorte
expressamente delimitado. Refutadores: precisar de campo público novo, perder
label/body/snapshot, produzir ocorrência extra a partir de Styled ou de render,
reordenar união por função, ou desalocar a associação walk/layout após a
promoção. Qualquer desses resultados exige rever o contrato correspondente;
nenhum seria absolvido por build/lint verde.

## Hashes das entradas decisórias

SHA-256 abaixo é dos bytes do arquivo, não cálculo de hash efetivo Tekt nem
verificação de pins V26. Os hashes identificam precisamente a revisão
examinada; alteração posterior exige nova avaliação do delta relevante.

```text
856e6795e57c220252d6d340647df461fe33a14b739a3289c59447103dc2aded  00_nucleo/prompts/entities/elements/strong.md
02fb4e64aced1c9191b49010bb5cd9f4a982a567b640f998049d8ff053f353cd  00_nucleo/prompts/entities/elements/emph.md
cfdf29749ac82f26d27d96ab973ff82b19de004a2cacba16134b10b9a172517c  00_nucleo/prompts/compiler/introspect/extract_payload.md
cdeb753844af1cc8af8e138c8e92533ce272a5c07b938961ad0da47a99c90183  00_nucleo/prompts/compiler/introspect/locatable.md
4beec8cad57cb9c13773cc79031d7aaedd409289cbecd1cd122c9a4e7970f456  00_nucleo/prompts/compiler/introspect.md
d47eb8459a4062ea902ad38f5aeb5a97bc4fea4fb62ac5230107fbdc20707944  00_nucleo/prompts/entities/introspector.md
5959c62058b10dfb81fd9b547979a2e69c86564a441c5b05a774b35160ca49cd  00_nucleo/prompts/compiler/layout.md
93fc3e5f20894fa09a54996e13328201f5dd2216c5ca260b67dd42e8dcc91bba  00_nucleo/prompts/compiler/eval/selector_matching.md
419d56209cda385cedb09205b1dde60be6719897dc467f583b637ce389df6084  00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml
9ec797c6f904aa00983561d9656b0117115621f5bfd5c505ac50b16f65554143  00_nucleo/diagnosticos/p1339-where-payload-approval.json
fba184a53444bf0a5d1aeea73bd7b10ccccbf1f1b2ba4908f21699cb5a8a7aca  00_nucleo/diagnosticos/p1339-where-occurrence-probe.md
bdf16ebfb94c8ab085311902b3e9786fba18b2aa117e4cabebf56e375e34444a  00_nucleo/diagnosticos/p1339-where-occurrence-probe-runs.json
baffc0c4bc9f39ffb34a415691efa01c2aea60dae97d817498d0d0804ec2d874  00_nucleo/diagnosticos/p1339-where-query-design-review-r2.md
553f602a5b3c38f2b7c8ff9add6495fe8feeeb37b3a6fc409fcebd2084568f02  01_core/src/entities/introspector.rs
73a2e32adb41c0f116074720d366ce071613a03e220765b59900e14b949c6040  01_core/src/entities/locator.rs
9fce06920302748bfc768e265ec3260c80b8227b7bc79f6d0b2afc4b008bd20a  01_core/src/entities/elements/strong.rs
22320416e21a6eac073b1b0a9d45f9193a6d3df7cf79c219fe1ebebb75271a47  01_core/src/entities/elements/emph.rs
09029233b75b358953024023dd11b060544890ac791b3151129afd7687a3066e  01_core/src/compiler/introspect.rs
9d4aa8e56a4e9157cbab345ddd40cfaae73c4c4c60f7fac85741b4c79e05069d  01_core/src/compiler/layout/mod.rs
09014da2b4a4737db6e1e996a3d001cc9fbdcd356f84890f5020f4a324ec80f9  01_core/src/compiler/eval/selector_matching.rs
67eee58a73878438d54be8044c97d570e276889d6637ad43b305dd30c6a51937  01_core/src/compiler/eval/bindings/field_access.rs
dd7293909aec9bf3bb6a7f26f3d311943abf22ee74894a2b8c5c2604ce7ebf18  01_core/src/compiler/stdlib/structural/markup.rs
```

Normas/instruções lidas, com hashes dos bytes:

```text
bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0  CLAUDE.md
d358e69f77ceb88bfd7f64bbf64f90b88b6e97c0bac88566f0eb94315b48829f  01_core/CLAUDE.md
66990d349a9e89851686cd94590a84711c69364f76b6df501230f64daf3b0c48  /home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md
f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417  /home/dikluwe/.codex/skills/tekt-materializacao-segregada/references/papeis-e-capacidades.md
16db4af3a8a21a27e1bfc4a5dd00c976f0fc946ba46dc8df1a663171d823f72d  /home/dikluwe/.codex/skills/tekt-materializacao-segregada/references/artefatos-e-gates.md
e5df7708d0a333879e6cec0130794cae6811e387ab9bd56f56b690104e80c0f1  00_nucleo/adr/typst-adr-0068-location-aware-layouter.md
1cbd1cb2ded7e3284b8f8d68da5caa996ed79bf69bb18038f84a9f53139eeb58  00_nucleo/adr/typst-adr-0069-post-recursion-tag-emission.md
e680d22bbf4486cf93f5bfb4ec85f4ae965e6db788c2a18c48f3be000029d49d  00_nucleo/adr/typst-adr-0107-paridade-linguagem-nao-mecanica.md
31daec5ae9e84cb5bbdcb806e9a2b6cb9160b7e90519df53e6e0bd8809076405  00_nucleo/adr/typst-adr-0108-disciplina-anti-deriva.md
5e8581b5f9ebb0798d4213e59e39ee8dfcd4f41b34d4ca639b00b1f287699ad9  00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md
64756b81ce58ca62e1a166b3776303759bc7af507a1c97a4e3ad91a8dc5b906e  00_nucleo/adr/typst-adr-0129-nucleos-tekt-l0-compartilhado.md
b6d0fe48e525e3cc695110c9c8480087b865425f9653a37d0ce8fb0783dd60c2  00_nucleo/adr/typst-adr-0130-referencias-canonicas-l0.md
```

Não foi encontrada ADR de segregação na busca restrita a `00_nucleo/adr/`.
Permanecem obrigatórios os gates de contrato/oráculos/RED, mutantes válidos e
verificação posterior. Este relatório não os substitui nem conclui o P1339.
