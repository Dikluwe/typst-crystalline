# Prompt L0 — `entities/elements/math_cases` — `MathCasesElem`
Hash do Código: 746703e1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_cases.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathCasesElem {
    pub rows: Vec<Vec<Content>>,
}
```

`Content::MathCases { rows }` → `Content::MathCases(Arc<MathCasesElem>)`.
Construtor ergonómico: `Content::math_cases(rows: Vec<Vec<Content>>)`.
Delimitador esquerdo `{` fixo; sem delimitador direito (no layout, inalterado).

## `impl Element for MathCasesElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | linhas → células `join(" & ")`, linhas `join(", ")` (`content.rs:1639`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** em cada célula de cada linha (`content.rs:2123`): `Content::MathCases(Arc::new(MathCasesElem { rows: new_rows? }))` |
| `map_text` | **terminal** (math structural; `content.rs:2626`): `Content::MathCases(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `rows` (paridade `content.rs:1828`).

## Critério

`plain_text` células `& ` e linhas `, `; `map_content` recurse na grelha;
`map_text` terminal; igualdade estrutural.

## P1430 — carrier completo (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

O carrier vigente já acumulou `delim`, `reverse` e `gap`, mas guarda rows
pré-resolvidas, usa `(char, char)` obrigatório, `Option<Length>` e não conserva
presença explícita. O vanilla guarda children variádicos, par opcional e gap
relativo; a resolução de ramos pertence ao consumer.

### Contrato público proposto

`MathCasesElem` passa a guardar `children: Vec<Content>`,
`delim: (Option<char>, Option<char>)`, `reverse: bool`,
`gap: Rel<Length>` e `MathCasesExplicit { delim, reverse, gap }`. O tipo de
presença é público, `Copy + Eq + Hash`. Elemento, explicitude, delimitadores e
gap participam de igualdade/hash; floats relativos seguem a normalização dos
carriers vigentes. `map_content` desce em cada child; `map_text` continua
terminal; `plain_text` junta ramos com `, ` e deixa `MathAlignPoint` dentro do
child. Toda reconstrução preserva valores, ordem e bits.

## Reuso de contêineres sem substituição — 2026-10-10

Medição anterior à decisão: `00_nucleo/diagnosticos/content-container-reuse-2026-10-10/before.json.gz` captura HEAD/working tree/UTC/fontes completas/owners/release. `01_core/src/entities/elements/math_cases.rs:66` conserva a travessia vigente e materializa o elemento mesmo quando os descendentes não mudam. `01_core/src/entities/content/traversal.rs:111–192` embala os resultados destes owners em Some incondicionalmente. Isso é custo mecânico; a linguagem e a identidade semântica dos elementos são preservadas. Inferência refutável: propagar a ausência explícita de mudança reduz alocações e retenção temporária; as sondas e os controles de tempo podem refutar o ganho.

Refinamento interno ADR-0127: disponibilizar somente helper crate-private map_content_if_changed nos elementos já owned por este consumer. Retornar None apenas se nenhum filho efetivamente visitado foi substituído; terminais sem filhos retornam None. Transformar filhos na mesma ordem do map_content vigente, uma única vez, e interromper no primeiro erro. Some do callback, inclusive conteúdo igual, sempre é mudança. Preservar todos os campos, valores, presença explícita, slots omitidos/none/presentes, flags, spans e metadados; não usar igualdade/hash/endereço como certificado. Vetores/linhas só alocam após o primeiro filho/linha alterado; os irmãos conservam seu conteúdo compartilhado. Assinaturas públicas e métodos de trait ficam iguais, com adaptação interna à nova prova; map_content_pre e map_text mantêm seu contrato anterior. Nenhuma ratificação de contratos históricos propostos é ampliada por este refinamento.

A lógica específica permanece neste owner; o match exaustivo em Content apenas delega estaticamente. Sem vtable, enum novo, campos novos, estado global, I/O ou fase nova. Aceitação: RED→GREEN de reuso, ordem, erro, substituição igual, opcionais/slots e metadados; suite core, builds e lint, qualificação de texto/morfologia/render/query contra release anterior e vanilla pinado; alocações e tempos pareados com proveniência. Não alegar ganho universal a partir de casos isolados.


A prova compartilhada de ausência de mudança obedece às claims desse núcleo. O algoritmo, os campos transformados, sua ordem e a preservação específica permanecem owned por este prompt e seu único consumer.
