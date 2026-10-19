# Prompt L0 — `entities/elements/math_accent` — `MathAccentElem`
Hash do Código: 4beca359

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_accent.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub. Mecanismo vanilla:
`AccentElem` minimal em P296; P1429 propõe fechar `size`, `dotless`,
normalização e orientação (gate ADR-0127 pendente).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathAccentElem {
    pub base:   Content,   // era Box<Content>
    pub accent: Content,   // era Box<Content>
}
```

`Content::MathAccent { base, accent }` → `Content::MathAccent(Arc<MathAccentElem>)`.
Construtor ergonómico: `Content::math_accent(base: Content, accent: Content)`.

## `impl Element for MathAccentElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `format!("{}{}", self.base.plain_text(), self.accent.plain_text())` (`content.rs:1645`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** em `base`/`accent` (`content.rs:2131`) |
| `map_text` | **terminal** (math structural; `content.rs:2631`): `Content::MathAccent(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `base + accent` (paridade `content.rs:1831`).

## Critério

`plain_text` concat `base+accent`; `map_content` recurse ambos; `map_text`
terminal; igualdade estrutural.

## P1429 — carrier completo (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O carrier P296 guarda o acento como `Content` e não consegue provar
single-codepoint, orientação, tamanho, dotless nem presença explícita. O
vanilla ratificado guarda um `Accent(char)` normalizado, `size` relativo com
default `100%` e `dotless=true`.

### Contrato público proposto

`MathAccentElem` passa a guardar `base: Content`, `accent: char`,
`size: Rel<Length>`, `dotless: bool` e `MathAccentExplicit { size, dotless }`.
O tipo de presença é público, `Copy + Eq + Hash`; o elemento implementa
`PartialEq`/`Hash` observacional incluindo valores e bits, com floats
normalizados pelo padrão vigente dos carriers relativos. `map_content` desce
somente em `base`; `map_text` continua terminal; `plain_text` concatena base e
codepoint canônico. Toda reconstrução preserva valores e presença.

## Reuso de contêineres sem substituição — 2026-10-10

Medição anterior à decisão: `lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz` captura HEAD/working tree/UTC/fontes completas/owners/release. `01_core/src/entities/elements/math_accent.rs:70` conserva a travessia vigente e materializa o elemento mesmo quando os descendentes não mudam. `01_core/src/entities/content/traversal.rs:111–192` embala os resultados destes owners em Some incondicionalmente. Isso é custo mecânico; a linguagem e a identidade semântica dos elementos são preservadas. Inferência refutável: propagar a ausência explícita de mudança reduz alocações e retenção temporária; as sondas e os controles de tempo podem refutar o ganho.

Refinamento interno ADR-0127: disponibilizar somente helper crate-private map_content_if_changed nos elementos já owned por este consumer. Retornar None apenas se nenhum filho efetivamente visitado foi substituído; terminais sem filhos retornam None. Transformar filhos na mesma ordem do map_content vigente, uma única vez, e interromper no primeiro erro. Some do callback, inclusive conteúdo igual, sempre é mudança. Preservar todos os campos, valores, presença explícita, slots omitidos/none/presentes, flags, spans e metadados; não usar igualdade/hash/endereço como certificado. Vetores/linhas só alocam após o primeiro filho/linha alterado; os irmãos conservam seu conteúdo compartilhado. Assinaturas públicas e métodos de trait ficam iguais, com adaptação interna à nova prova; map_content_pre e map_text mantêm seu contrato anterior. Nenhuma ratificação de contratos históricos propostos é ampliada por este refinamento.

A lógica específica permanece neste owner; o match exaustivo em Content apenas delega estaticamente. Sem vtable, enum novo, campos novos, estado global, I/O ou fase nova. Aceitação: RED→GREEN de reuso, ordem, erro, substituição igual, opcionais/slots e metadados; suite core, builds e lint, qualificação de texto/morfologia/render/query contra release anterior e vanilla pinado; alocações e tempos pareados com proveniência. Não alegar ganho universal a partir de casos isolados.


A prova compartilhada de ausência de mudança obedece às claims desse núcleo. O algoritmo, os campos transformados, sua ordem e a preservação específica permanecem owned por este prompt e seu único consumer.
