# Prompt L0 — `entities/elements/boxed` — `BoxedElem`
Hash do Código: b179614c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/boxed.rs`
**Origem**: modelo D (ADR-0105), **Lote 14 P329** (reclassificado da triagem
DEBT-58 — element-shaped denso, não primitivo). Trait e glossário (§A.0): ver
`entities/elements/_comum.md`. **Não-locatável**. Contentor — `map_*` recursam
no `body`. Família densa do L12 (box inline container).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct BoxedElem {
    pub body:     Content,            // era Box<Content>
    pub width:    Option<Length>,
    pub height:   Option<Length>,
    pub inset:    Sides<Length>,
    pub baseline: Length,
    pub outset:   Sides<Length>,
    pub radius:   Corners<Length>,
    pub clip:     bool,
    pub fill:     Option<Color>,
    pub stroke:   Option<Stroke>,
}
```

`Content::Boxed { … }` → `Content::Boxed(Arc<BoxedElem>)`. Construtor ergonómico
preservado: `Content::boxed(body, width, height, inset, baseline)` — **cobre só
5 dos 10 campos** (defaults outset/radius/clip/fill/stroke). Logo as construções
de campo-completo (materialize, testes) usam **Arc-wrap** (regra C2):
`Content::Boxed(Arc::new(BoxedElem { … }))`.

> **`Hash` manual via Debug**: `Length`/`Sides<Length>`/`Corners<Length>`/`Color`/
> `Stroke` carregam `f64` → `impl Hash { format!("{self:?}").hash(state) }`
> (paridade `content_hash`; ressalva `-0.0`≠`0.0`). `PartialEq` deriva.

## `impl Element for BoxedElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1768`) |
| `is_empty` | **override**: `self.body.is_empty()` (`content.rs:1635`) |
| `map_content` | **recursivo** no `body`, preserva os 9 cosméticos (`content.rs:2054`) |
| `map_text` | **recursivo** no `body`, preserva os 9 cosméticos (`content.rs:2235`) |
| `get_field`/`element_kind`/`to_payload` | default |

## `eq`

`#[derive(PartialEq)]` compara os 10 campos (paridade `content.rs:1904`).

## P1408 — carrier fracionário de largura

Para representar a morfologia devolvida por `outline.entry.inner`, a unidade
acrescenta `width_fraction: Option<f64>`. Constructors públicos históricos e
struct literals anteriores a P1408 usam `None`; o constructor interno
owner-local de P1404 e, após o gate público de P1408, `native_box(width: Nfr)`
podem usar `Some(weight)`. Walkers, igualdade e hash preservam o campo sem
normalizar sinal ou magnitude. `width` e `width_fraction` são mutuamente
exclusivos em construções públicas válidas.

## Reuso de contêineres sem substituição — 2026-10-10

Medição anterior à decisão: `00_nucleo/diagnosticos/content-container-reuse-2026-10-10/before.json.gz` captura HEAD/working tree/UTC/fontes completas/owners/release. `01_core/src/entities/elements/boxed.rs:53` conserva a travessia vigente e materializa o elemento mesmo quando os descendentes não mudam. `01_core/src/entities/content/traversal.rs:111–192` embala os resultados destes owners em Some incondicionalmente. Isso é custo mecânico; a linguagem e a identidade semântica dos elementos são preservadas. Inferência refutável: propagar a ausência explícita de mudança reduz alocações e retenção temporária; as sondas e os controles de tempo podem refutar o ganho.

Refinamento interno ADR-0127: disponibilizar somente helper crate-private map_content_if_changed nos elementos já owned por este consumer. Retornar None apenas se nenhum filho efetivamente visitado foi substituído; terminais sem filhos retornam None. Transformar filhos na mesma ordem do map_content vigente, uma única vez, e interromper no primeiro erro. Some do callback, inclusive conteúdo igual, sempre é mudança. Preservar todos os campos, valores, presença explícita, slots omitidos/none/presentes, flags, spans e metadados; não usar igualdade/hash/endereço como certificado. Vetores/linhas só alocam após o primeiro filho/linha alterado; os irmãos conservam seu conteúdo compartilhado. Assinaturas públicas e métodos de trait ficam iguais, com adaptação interna à nova prova; map_content_pre e map_text mantêm seu contrato anterior. Nenhuma ratificação de contratos históricos propostos é ampliada por este refinamento.

A lógica específica permanece neste owner; o match exaustivo em Content apenas delega estaticamente. Sem vtable, enum novo, campos novos, estado global, I/O ou fase nova. Aceitação: RED→GREEN de reuso, ordem, erro, substituição igual, opcionais/slots e metadados; suite core, builds e lint, qualificação de texto/morfologia/render/query contra release anterior e vanilla pinado; alocações e tempos pareados com proveniência. Não alegar ganho universal a partir de casos isolados.


A prova compartilhada de ausência de mudança obedece às claims desse núcleo. O algoritmo, os campos transformados, sua ordem e a preservação específica permanecem owned por este prompt e seu único consumer.
