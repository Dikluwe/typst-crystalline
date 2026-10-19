# Prompt L0 — `entities/elements/math_op` — `MathOpElem`
Hash do Código: a5d0c14d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_op.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub. Mecanismo vanilla:
`OpElem` (P298; `limits` é discriminador de layout cross-variant — afeta o
`MathAttach` pai; semântica inalterada, em `rules/math/layout`).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathOpElem {
    pub text:   Content,   // era Box<Content>
    pub limits: bool,
}
```

`Content::MathOp { text, limits }` → `Content::MathOp(Arc<MathOpElem>)`.
Construtor ergonómico: `Content::math_op(text: Content, limits: bool)`.

> **Fonte de paridade**: documentação `https://typst.app/docs/reference/math/op/`
> (corpus `lab/corpus/docs/math/op.typ:7-27`); mecanismo vanilla
> `lab/typst-original/crates/typst-library/src/math/op.rs`
> (`OpElem { text, limits }`) e consumo de `limits` em
> `lab/typst-original/crates/typst-layout/src/math/ir/resolve.rs`
> (`resolve_op`).

## `impl Element for MathOpElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.text.plain_text()` (`limits` é layout-only; `content.rs:1655`) |
| `is_empty` | default `false` (`content.rs:1568`) |

> **Nota de verificação**: a afirmação "`limits` é layout-only" refere-se à
> entidade `MathOpElem` — a flag não altera `plain_text`/`is_empty`/
> `map_content`/`map_text`/`eq`. O consumo real de `limits` acontece em
> `layout_attach` (`attach.rs:78`, ver `math/layout/attach.md` §P992).
| `map_content` | **recursivo** em `text`, `limits` preservado (`content.rs:2151`): `Content::MathOp(Arc::new(MathOpElem { text: self.text.map_content(f)?, limits: self.limits }))` |
| `map_text` | **terminal** (math structural; `content.rs:2637`): `Content::MathOp(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `text + limits` (paridade `content.rs:1838`).

## Critério

`plain_text` só o `text`; `map_content` recurse `text` e preserva `limits`;
`map_text` terminal; igualdade estrutural.

## Reuso de contêineres sem substituição — 2026-10-10

Medição anterior à decisão: `lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz` captura HEAD/working tree/UTC/fontes completas/owners/release. `01_core/src/entities/elements/math_op.rs:30` conserva a travessia vigente e materializa o elemento mesmo quando os descendentes não mudam. `01_core/src/entities/content/traversal.rs:111–192` embala os resultados destes owners em Some incondicionalmente. Isso é custo mecânico; a linguagem e a identidade semântica dos elementos são preservadas. Inferência refutável: propagar a ausência explícita de mudança reduz alocações e retenção temporária; as sondas e os controles de tempo podem refutar o ganho.

Refinamento interno ADR-0127: disponibilizar somente helper crate-private map_content_if_changed nos elementos já owned por este consumer. Retornar None apenas se nenhum filho efetivamente visitado foi substituído; terminais sem filhos retornam None. Transformar filhos na mesma ordem do map_content vigente, uma única vez, e interromper no primeiro erro. Some do callback, inclusive conteúdo igual, sempre é mudança. Preservar todos os campos, valores, presença explícita, slots omitidos/none/presentes, flags, spans e metadados; não usar igualdade/hash/endereço como certificado. Vetores/linhas só alocam após o primeiro filho/linha alterado; os irmãos conservam seu conteúdo compartilhado. Assinaturas públicas e métodos de trait ficam iguais, com adaptação interna à nova prova; map_content_pre e map_text mantêm seu contrato anterior. Nenhuma ratificação de contratos históricos propostos é ampliada por este refinamento.

A lógica específica permanece neste owner; o match exaustivo em Content apenas delega estaticamente. Sem vtable, enum novo, campos novos, estado global, I/O ou fase nova. Aceitação: RED→GREEN de reuso, ordem, erro, substituição igual, opcionais/slots e metadados; suite core, builds e lint, qualificação de texto/morfologia/render/query contra release anterior e vanilla pinado; alocações e tempos pareados com proveniência. Não alegar ganho universal a partir de casos isolados.


A prova compartilhada de ausência de mudança obedece às claims desse núcleo. O algoritmo, os campos transformados, sua ordem e a preservação específica permanecem owned por este prompt e seu único consumer.
