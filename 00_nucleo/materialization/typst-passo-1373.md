# Passo 1373 — transportar e resolver altura relativa de `block`

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Corrigir o gap vertical S07 confirmado no P1372: `block(height: 100%)` deve
preservar a percentagem até o layout e resolvê-la contra a altura útil da
região, em vez de truncá-la para zero durante eval.

## Medição anterior à decisão

No P1372, com página `200pt × 240pt`, margem `10pt`, carga textual idêntica e
marcador `After`, os dois perfis medidos produziram:

- vanila: lone e controle em página 1, `yMin=7,404`;
- cristalino: lone e controle em página 0, `yMin=42,230`;
- delta lone-controle: página 0 e Y `0,000` nos dois produtos.

Logo o metadata e a cardinalidade do body não explicam a diferença; o eixo
causal é a altura do container. A fonte cristalina confirma o mecanismo:
`stdlib/layout.rs:861-874` converte todo `Value::Ratio` em `Length::ZERO`, e
`native_block` usa esse extrator para `height` em `1535-1539`. A entidade
armazena `Option<Length>` (`entities/elements/block.rs:23-27`), portanto a
informação percentual já foi perdida antes do layout.

No vanila ratificado, `container.rs:257-284` conserva a altura como `Sizing`,
cuja variante relativa guarda `Rel`; `flow/block.rs:323-331` resolve esse
valor contra a altura-base da região. Esta é medição da fonte, não inferência
a partir do comportamento.

## Classificação e owner

É uma correção de paridade da linguagem, mas exige alterar o campo público
`BlockElem::height`. Owners afetados:

- entidade/transporte: `prompts/entities/elements/block.md`;
- construtor Rust compatível: `prompts/entities/content.md`;
- entrada Typst: `prompts/compiler/stdlib/layout.md`;
- consumo: `prompts/compiler/layout/block.md`.

Pelo ADR-0127, mudança de campo público exige parada após atualizar L0 e antes
de RED/código. Não usar sentinela em `Length`, não converter percentagem em
`em` e não consultar geometria de página durante eval.

## Contrato proposto

1. `BlockElem::height` passa de `Option<Length>` para
   `Option<Rel<Length>>`; `None` continua significando auto.
2. `native_block(height:)` aceita `Length`, `Float`, `Int`, `Ratio` e
   `Relative`. Valores absolutos são elevados para `Rel { rel: 0, abs }`,
   `Ratio` para `Rel { rel: ratio, abs: 0 }` e `Relative` é preservado.
3. `Content::block` mantém sua assinatura pública atual com
   `Option<Length>` e eleva internamente a altura absoluta, evitando quebra
   desnecessária nesse construtor.
4. O layout resolve uma única vez a altura relativa contra a altura útil da
   região externa, antes de restringir a região dos descendentes. O mesmo
   valor resolvido alimenta fitting, `parent_bbox`, avanço mínimo e shape.
5. `height: 100%` em página `240pt` com margens verticais totais de `20pt`
   resolve para `220pt`. A sonda P1372 deve mudar de `DIFFERENCE` para `MATCH`
   nos dois perfis sem alterar o payload `ProbeAfter`.

## Testes RED planejados

- stdlib conserva `100%` como `Rel { rel: 1, abs: 0 }`;
- construtor Rust absoluto conserva `height: 100pt` como parte absoluta;
- layout focal resolve 100% contra altura útil, não font-size nem altura
  restante;
- controle absoluto existente continua verde;
- teste bilateral P1372 passa nos dois perfis.

## Incompletude explícita

P1373 cobre somente `height: Length|Ratio|Relative`. O **Passo P1374** deve
completar `width` relativa e a variante fracionária (`height: 1fr`) mediante
contrato próprio; nenhum desses valores pode ser silenciosamente aceito e
truncado neste passo.

## Execução limitada após confirmação

Executar um único teste focal Rust por vez, sem suíte ampla, com timeout de
120 s, memória virtual de 3 GiB, arquivo de 256 MiB e canais em arquivo
temporário. Preferir binário de teste já compilado se estiver ligado às fontes
atuais; só compilar o package L1 se necessário. Depois executar uma única
matriz P1372 sequencial sob seus limites de 2 GiB/16 MiB.

## Estado

**CONFIRMADO NO GATE ADR-0127.** O humano autorizou em 2026-09-13 a mudança
pública e os quatro L0s propostos. RED e código podem prosseguir sob o contrato
congelado.

## Resultado da materialização

**IMPLEMENTADO E VERIFICADO em 2026-09-13.** A altura relativa agora atravessa
eval como `Rel<Length>`, resolve contra os 220pt úteis e a primeira linha
posterior progride antes de ser emitida fora da região. A matriz final obteve
`MATCH` 2/2: vanila e cristalino posicionaram `After` na página de índice 1,
`yMin=7,404`, nos casos lone/controle e nos dois perfis.

O RED focal observou `ProbeAfter` retido na primeira página. O GREEN executou
4/4 testes P1373. Build e testes foram seriais com teto de 4 GiB. Uma tentativa
de link com arquivo limitado a 256 MiB foi recusada porque o binário de testes
mede 308.865.296 bytes; a repetição incremental usou 512 MiB. Não houve OOM,
timeout ou `SIGKILL`. O P1374 permanece responsável por width relativa e
altura fracionária.
