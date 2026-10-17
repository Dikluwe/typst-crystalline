# Prompt L0 — `entities/elements/math_class_override` — `MathClassOverrideElem`
Hash do Código: e9fe4e25

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_class_override.rs`
**Origem**: modelo D (ADR-0105), **P772y** (`math.class(class, body)`). Trait, regras partilhadas
e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável**
(paralelo P296/P298 — `MathAccent`/`MathCancel`/`MathOp`). Mecanismo vanilla:
`ClassElem` (`math/mod.rs`) — `#[elem(Mathy)] pub struct ClassElem { #[required]
pub class: MathClass, #[required] pub body: Content }`.

---

## Contexto

`math.class(class, body)` força a `MathClass` de `body` para efeitos de
espaçamento automático entre símbolos matemáticos adjacentes — override do
valor inferido automaticamente por `default_math_class`/
`rules/math/layout/spacing.rs::node_math_class`. Não afecta o layout/
renderização do `body` em si (que continua a ser layoutado normalmente via
`MathLayouter::layout_node` — ver `rules/math/layout/_comum.md`).

Caso de uso canónico (P772w, achado original): definir um símbolo
personalizado que se comporta tipograficamente como uma relação, por
exemplo `#let loves = math.class("relation", sym.suit.heart)`, de modo que
`$x loves y$` produza o mesmo espaçamento (THICK) que `$x = y$`.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathClassOverrideElem {
    pub class: MathClass,
    pub body:  Content,
}
```

`Content::MathClassOverride(Arc<MathClassOverrideElem>)`.
Construtor ergonómico: `Content::math_class_override(class: MathClass, body: Content)`.

## `impl Element for MathClassOverrideElem`

| método | comportamento |
|---|---|
| `plain_text` | `self.body.plain_text()` |
| `is_empty` | `self.body.is_empty()` (delega — diferente do default `false` de `MathCancelElem`, pois `class` sem `body` visível não é conteúdo estrutural) |
| `map_content` | **recursivo** no `body`: `Content::MathClassOverride(Arc::new(MathClassOverrideElem { class: self.class, body: self.body.map_content(f)? }))` |
| `map_text` | **terminal** (paralelo `MathCancel`/`MathStyled` — família math não desce em `map_text`; `MathClassOverrideElem::map_text` existe pelo contrato do trait mas o dispatcher central `Content::map_text` clona directamente, tal como os restantes membros da família math) |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `class` e `body`.

## Despacho no hub (`content.rs`)

Mesmo padrão dos restantes membros da família math (mirror de `MathCancel`):
Debug/Display (`"math.class({:?})"`), constructor, `plain_text`, `PartialEq`,
`map_content` (recursivo), `map_text` (terminal — bloco de clonagem directa).
Também precisa de braço em `rules/introspect.rs` (2×, não-locatável/terminal),
`rules/introspect/locatable.rs` (não-locatável), `compiler/layout/mod.rs`
(fallback de math fora de contexto — `layout_math_fallback`),
`rules/eval/repr.rs` (`class("<nome>", <repr(body)>)`) e
`03_infra/src/query_helpers.rs` (2×, terminal sem texto próprio).

## Critério

`plain_text`/`is_empty` transparentes ao body; `map_content` recurse o body
preservando `class`; `map_text` terminal (clona directamente, class
preservada); igualdade estrutural compara `class` e `body`.

## Reuso de contêineres sem substituição — 2026-10-10

Medição anterior à decisão: `00_nucleo/diagnosticos/content-container-reuse-2026-10-10/before.json.gz` captura HEAD/working tree/UTC/fontes completas/owners/release. `01_core/src/entities/elements/math_class_override.rs:40` conserva a travessia vigente e materializa o elemento mesmo quando os descendentes não mudam. `01_core/src/entities/content/traversal.rs:111–192` embala os resultados destes owners em Some incondicionalmente. Isso é custo mecânico; a linguagem e a identidade semântica dos elementos são preservadas. Inferência refutável: propagar a ausência explícita de mudança reduz alocações e retenção temporária; as sondas e os controles de tempo podem refutar o ganho.

Refinamento interno ADR-0127: disponibilizar somente helper crate-private map_content_if_changed nos elementos já owned por este consumer. Retornar None apenas se nenhum filho efetivamente visitado foi substituído; terminais sem filhos retornam None. Transformar filhos na mesma ordem do map_content vigente, uma única vez, e interromper no primeiro erro. Some do callback, inclusive conteúdo igual, sempre é mudança. Preservar todos os campos, valores, presença explícita, slots omitidos/none/presentes, flags, spans e metadados; não usar igualdade/hash/endereço como certificado. Vetores/linhas só alocam após o primeiro filho/linha alterado; os irmãos conservam seu conteúdo compartilhado. Assinaturas públicas e métodos de trait ficam iguais, com adaptação interna à nova prova; map_content_pre e map_text mantêm seu contrato anterior. Nenhuma ratificação de contratos históricos propostos é ampliada por este refinamento.

A lógica específica permanece neste owner; o match exaustivo em Content apenas delega estaticamente. Sem vtable, enum novo, campos novos, estado global, I/O ou fase nova. Aceitação: RED→GREEN de reuso, ordem, erro, substituição igual, opcionais/slots e metadados; suite core, builds e lint, qualificação de texto/morfologia/render/query contra release anterior e vanilla pinado; alocações e tempos pareados com proveniência. Não alegar ganho universal a partir de casos isolados.


A prova compartilhada de ausência de mudança obedece às claims desse núcleo. O algoritmo, os campos transformados, sua ordem e a preservação específica permanecem owned por este prompt e seu único consumer.
