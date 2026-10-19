# L0 — compiler/layout/finish
Hash do Código: bfd8e315

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:4e487a99ea1f1e82a67dfaced0958f98392fb26b1a7d292355230d6ffa7f2d0b
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:ff71dfc6648d6deb4b2b8054e3d58a93777d17837129f0f1b1e7e138ffb6943c
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:3ee5760ffa44b5fd889b9d6772972e3b8417be332e13726cd9eea24b32e686f8

## Medição anterior à decisão

No HEAD f7195cdf6 (árvore limpa, 2026-09-30), o bloco está em `01_core/src/compiler/layout/mod.rs:1957-2281`. Esta observação estabelece o limite da extração; não mede desempenho ou nova paridade bilateral.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/finish.rs`. Composição terminal do documento paginado; preserva flush, fixups, camadas e transporte do resultado. Manter contrato público, comportamento por defeito, fase do pipeline, compatibilidade, pureza e despacho estático.

## Aceitação

A suíte L1, o build e crystalline-lint passam. A separação não constitui prova de ganho de desempenho.

## Migração do carrier bidi ratificado (2026-10-05)

Medição anterior à decisão: o inventário `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifica as referências desta unidade e os owners 1:1. O contrato do manifesto 1ae7513e foi confirmado; nesta unidade ajustar constructors não certificados com None e patterns de leitura com `..`, preservando os observáveis de linguagem existentes. Transformações obedecem ao núcleo pinado de origem; sem ampliar fase/API.

## Posições no referencial composto — 2026-10-14

### Medição anterior à decisão

box-contexto-2026-10-14/probe.json.gz, fontes/binários/HEAD/working tree/UTC
preservados: Box, inset2pt, prefixo e aninhamento publicam (0,0), mesmo
quando a tinta se desloca. A diferença da baseline do context direto é
uma questão separada: não copiar o vanilla apenas por diferir. mod.rs:
1131-1148/1180-1200 escreve pontos locais em runtime.positions; Box agrupa
o corpo em outro referencial; finish.rs:295-302 sela o mapa sem recompor
esses pontos. Realized escreve com or_insert; labels têm mapa paralelo.
HTML já transporta markers privados pelo frame em html_frame.rs:30-199.
O registro single-pass e sealing por sub-store são preservados, ADR-0073/
0074 §§posição/modelo híbrido; não se adota introspecção global vanilla.
A inspeção do núcleo coordinates e seus owners mostrou usos absolutos
em grid e explicitamente locais em Place/Box. A qualificação do núcleo
conserva ambos e exige transformação até a página, sem dupla origem.
Snapshot anterior: posicoes-compostas-2026-10-14/before.json.gz.
Geometria/identidade são observáveis; o carrier e seu índice são mecânica.

### Contrato deste owner

Após todas as transformações/fixups/camadas finais e antes de mover pages para PagedDocument e selar os sub-stores, projetar/remover os carriers via helper privado. Continuar consumindo runtime.positions e label maps pelas APIs existentes. Não reconstruir introspecção global, mudar fase, repetir layout ou criar store público.

Correção interna ADR-0127: sem API/campo público, flag/default ou mudança
de fase. L0 primeiro; RED→GREEN de coordenadas, ausência de tinta/altura
fictícia, identidade, ordem, clones visuais e rollback; revalidar os
controles HTML, geometria, contexto/queries e suíte completa.

## Captura de links através de fronteiras — 2026-10-14

### Medição anterior à decisão

lab/results/construction/diagnostics/links-paginacao-2026-10-14.md e os recibos
posicoes-compostas-2026-10-14/automatic-boundary-probes registram fonte,
HEAD/árvore suja/UTC, binários e pin vanilla. Página 160×40pt/margem 10pt:
texto comum atravessa a quebra; Link URL, Ref e Ref Page abortam com índice
antigo após esvaziamento dos buffers. references.rs:270 aparece na pilha;
link.rs:22-32 usa o mesmo mecanismo. outline.rs:28-50 também guarda índices
antes do corpo. Limitar o índice não prova preservação do prefixo, ordem,
fragmentos, destino ou geometria. Snapshot anterior deste reparo:
lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz.
Os observáveis são conteúdo, destino, regiões e bounds; limites privados
são mecânica. Correção interna ADR-0127, sem API, default ou fase nova.

### Contrato deste owner

Após toda composição e fixups, resolver limites privados em links por fragmento, com bounds no referencial final. Fazer isto antes da projeção de posições, que deve atravessar os links resultantes. Nenhum limite privado escapa ao PagedDocument.

Aceitação: RED→GREEN de quebras antes/dentro do corpo, preservação de
prefixo/sufixo, regiões, destinos/bounds, corpo rico/aninhamento/colunas,
conteúdo diferido, contexto/rollback e ausência de limites privados na saída.
Revalidar os controles sem quebra e a suíte, build e linhagem.
