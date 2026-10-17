# Prompt L0 — `entities/elements/block` — `BlockElem`
Hash do Código: 93bf8cea

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/block.rs`
**Origem**: modelo D (ADR-0105), **Lote 15 P330** (lote tardio da triagem
DEBT-58; **o último lote do roteiro**). Trait e glossário (§A.0): ver `entities/elements/_comum.md`.
**Não-locatável**. Contentor — `map_*` recursam no `body`. A **variante mais
densa** do roteiro (14 campos); família L12/`Boxed`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct BlockElem {
    pub body:      Content,            // era Box<Content>
    pub width:     Option<Rel<Length>>,
    pub height:    Option<BlockHeight>,
    pub inset:     Sides<Length>,
    pub breakable: bool,
    pub outset:    Sides<Length>,
    pub radius:    Corners<Length>,
    pub clip:      bool,
    pub fill:      Option<Color>,
    pub stroke:    Option<Stroke>,
    pub spacing:   Option<Length>,
    pub above:     Option<Length>,
    pub below:     Option<Length>,
    pub sticky:    bool,
}
```

### P1373 — altura relativa preservada

#### Medição anterior à decisão

`compiler/stdlib/layout.rs:861-874` reduz `Value::Ratio` a `Length::ZERO`, e
`native_block:1535-1539` aplica esse extrator a `height`. O P1372 mediu, nos
dois perfis, `After` na página 1 (`yMin=7,404`) no vanila e na página 0
(`yMin=42,230`) no cristalino. Lone e controle coincidem dentro de cada
produto, isolando a perda da altura, não a cardinalidade ou o metadata.

O vanila ratificado conserva `BlockElem::height` como `Sizing` e sua variante
relativa até o layout (`container.rs:257-284,474-505`).

#### Decisão

O campo público `height` é `Option<Rel<Length>>`: `None` representa auto;
comprimentos absolutos usam `rel=0`; percentagens puras usam `abs=0`; valores
mistos conservam ambas as partes. `map_content`, `map_text`, igualdade e hash
preservam o valor integralmente.

Esta é mudança de contrato público sujeita ao gate ADR-0127. Não codificar a
parte relativa em `Length::em`, `NaN`, sinal ou qualquer outra sentinela.

**Estado dividido:** P1373 não materializa `Sizing::Fr` nem torna `width`
relativa. O Passo P1374 completa esses dois subconjuntos; até lá, fração em
height deve continuar rejeitada e width percentual não recebe crédito.

### P1374 — contrato público proposto para width relativa e height fracionária

#### Medição anterior à decisão

No vanila ratificado, `BlockElem::width` é `Smart<Rel<Length>>` e height é
`Sizing { Auto, Rel, Fr }` (`layout/container.rs:257-284,476-505`). O
cristalino P1373 possui `Option<Length>` para width e
`Option<Rel<Length>>` para height: o primeiro perde percentagem e o segundo
não representa fração sem sentinela.

#### Decisão pública proposta

Declarar neste mesmo owner:

```rust
pub enum BlockHeight {
    Relative(Rel<Length>),
    Fractional(f64),
}
```

`BlockElem::width` torna-se `Option<Rel<Length>>` e height torna-se
`Option<BlockHeight>`; `None` continua auto. O carrier rejeita peso fracionário
negativo ou não finito nas fronteiras de construção. Igualdade e hash incluem
variante e payload. É proibido codificar `fr` em `Rel`, `Length::em`, NaN,
infinito, sinal ou outro sentinel.

O nome e a estrutura Rust podem divergir do upstream; o contrato de linguagem
é distinguir auto, relativo e fracionário. Esta mudança pública exige
confirmação ADR-0127 antes de qualquer código.

**Fatia declaradamente incompleta:** P1374 materializa distribuição em
Sequence direta de uma região finita. P1375 completa wrappers transparentes,
colunas e multi-região; até lá esses contextos são `Unknown` sem crédito.

`Content::Block { … }` → `Content::Block(Arc<BlockElem>)`. Construtor ergonómico
preservado: `Content::block(body, width, height, inset, breakable)` — **cobre só
5 dos 14 campos** (defaults outset/radius/clip/fill/stroke/spacing/above/below/
sticky). Logo as construções de campo-completo (materialize `..(**e).clone()`,
stdlib, testes) usam **Arc-wrap** (regra C2): `Content::Block(Arc::new(BlockElem
{…}))`.

> **`Hash` manual via Debug**: `Length`/`Sides<Length>`/`Corners<Length>`/`Color`/
> `Stroke` carregam `f64` → `impl Hash { format!("{self:?}").hash(state) }`
> (paridade `content_hash`; ressalva `-0.0`≠`0.0`). `PartialEq` deriva.

## `impl Element for BlockElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1737`) |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1604`) |
| `map_content` | **recursivo** no `body`, preserva os 13 cosméticos (`content.rs:1998`) |
| `map_text` | **recursivo** no `body`, preserva os 13 cosméticos (`content.rs:2166`) |
| `get_field`/`element_kind`/`to_payload` | default |

> **Introspecção**: o walk arm (`introspect.rs:1225`) só **desce no body** — não
> consome `Content::Block` para payload (contraste com `Labelled`/P195D). A
> migração só converte o lado `Content::Block` dos arms (walk, materialize).

## `eq`

`#[derive(PartialEq)]` compara os 14 campos (paridade `content.rs:1864`).

## Reuso de contêineres sem substituição — 2026-10-10

Medição anterior à decisão: `00_nucleo/diagnosticos/content-container-reuse-2026-10-10/before.json.gz` captura HEAD/working tree/UTC/fontes completas/owners/release. `01_core/src/entities/elements/block.rs:65` conserva a travessia vigente e materializa o elemento mesmo quando os descendentes não mudam. `01_core/src/entities/content/traversal.rs:111–192` embala os resultados destes owners em Some incondicionalmente. Isso é custo mecânico; a linguagem e a identidade semântica dos elementos são preservadas. Inferência refutável: propagar a ausência explícita de mudança reduz alocações e retenção temporária; as sondas e os controles de tempo podem refutar o ganho.

Refinamento interno ADR-0127: disponibilizar somente helper crate-private map_content_if_changed nos elementos já owned por este consumer. Retornar None apenas se nenhum filho efetivamente visitado foi substituído; terminais sem filhos retornam None. Transformar filhos na mesma ordem do map_content vigente, uma única vez, e interromper no primeiro erro. Some do callback, inclusive conteúdo igual, sempre é mudança. Preservar todos os campos, valores, presença explícita, slots omitidos/none/presentes, flags, spans e metadados; não usar igualdade/hash/endereço como certificado. Vetores/linhas só alocam após o primeiro filho/linha alterado; os irmãos conservam seu conteúdo compartilhado. Assinaturas públicas e métodos de trait ficam iguais, com adaptação interna à nova prova; map_content_pre e map_text mantêm seu contrato anterior. Nenhuma ratificação de contratos históricos propostos é ampliada por este refinamento.

A lógica específica permanece neste owner; o match exaustivo em Content apenas delega estaticamente. Sem vtable, enum novo, campos novos, estado global, I/O ou fase nova. Aceitação: RED→GREEN de reuso, ordem, erro, substituição igual, opcionais/slots e metadados; suite core, builds e lint, qualificação de texto/morfologia/render/query contra release anterior e vanilla pinado; alocações e tempos pareados com proveniência. Não alegar ganho universal a partir de casos isolados.


A prova compartilhada de ausência de mudança obedece às claims desse núcleo. O algoritmo, os campos transformados, sua ordem e a preservação específica permanecem owned por este prompt e seu único consumer.
