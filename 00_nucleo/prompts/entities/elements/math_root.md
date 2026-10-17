# Prompt L0 — `entities/elements/math_root` — `MathRootElem`
Hash do Código: a8801b1f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_root.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathRootElem {
    pub index:    Option<Content>,   // era Option<Box<Content>>; None = raiz quadrada
    pub radicand: Content,           // era Box<Content>
    pub index_explicit: bool,        // P1426: presença causal do primeiro argumento
}
```

`Content::MathRoot { index, radicand }` → `Content::MathRoot(Arc<MathRootElem>)`.
Construtor ergonómico: `Content::math_root(index: Option<Content>, radicand: Content,
index_explicit: bool)`.

## `impl Element for MathRootElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `None => format!("sqrt({})", radicand…)`; `Some(i) => format!("root({}, {})", i…, radicand…)` (`content.rs:1625`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** (`content.rs:2107`): `index` via `.as_ref().map(map_content).transpose()?`; `radicand.map_content(f)?`; preserva `index_explicit` nominalmente |
| `map_text` | **terminal** (math structural; `content.rs:2623`): `Content::MathRoot(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `index + radicand + index_explicit`.

## Critério

`plain_text` `sqrt(...)`/`root(i, ...)`; `map_content` recurse preservando `None`
no index; `map_text` terminal; igualdade estrutural.

## P1140.3-A — produtor público `math.sqrt`

Além da sintaxe em modo math, `math.sqrt(radicand: Content)` é função pública
e produz o mesmo `Content::MathRoot` com `index: None`. A função pertence à
stdlib math e não a esta entidade: `MathRootElem` continua sendo dado puro,
sem dependência inversa para avaliação (ADR-0109, forma B). `calc.sqrt` é uma
função numérica distinta.

## P1426 — presença causal do índice (APROVADO no gate ADR-0127)

O índice tem três estados públicos: ausente (`None/false`, caso `sqrt`),
explicitamente `none` (`None/true`) e conteúdo presente (`Some(_)/true`, que
inclui conteúdo vazio). O bit não altera `plain_text` nem geometria, mas é
morfologia causal e deve sobreviver a clone, hash, igualdade e walkers.

A implementação desta secção foi confirmada explicitamente pelo dono em
2026-09-16.

## Reuso de contêineres sem substituição — 2026-10-10

Medição anterior à decisão: `00_nucleo/diagnosticos/content-container-reuse-2026-10-10/before.json.gz` captura HEAD/working tree/UTC/fontes completas/owners/release. `01_core/src/entities/elements/math_root.rs:35` conserva a travessia vigente e materializa o elemento mesmo quando os descendentes não mudam. `01_core/src/entities/content/traversal.rs:111–192` embala os resultados destes owners em Some incondicionalmente. Isso é custo mecânico; a linguagem e a identidade semântica dos elementos são preservadas. Inferência refutável: propagar a ausência explícita de mudança reduz alocações e retenção temporária; as sondas e os controles de tempo podem refutar o ganho.

Refinamento interno ADR-0127: disponibilizar somente helper crate-private map_content_if_changed nos elementos já owned por este consumer. Retornar None apenas se nenhum filho efetivamente visitado foi substituído; terminais sem filhos retornam None. Transformar filhos na mesma ordem do map_content vigente, uma única vez, e interromper no primeiro erro. Some do callback, inclusive conteúdo igual, sempre é mudança. Preservar todos os campos, valores, presença explícita, slots omitidos/none/presentes, flags, spans e metadados; não usar igualdade/hash/endereço como certificado. Vetores/linhas só alocam após o primeiro filho/linha alterado; os irmãos conservam seu conteúdo compartilhado. Assinaturas públicas e métodos de trait ficam iguais, com adaptação interna à nova prova; map_content_pre e map_text mantêm seu contrato anterior. Nenhuma ratificação de contratos históricos propostos é ampliada por este refinamento.

A lógica específica permanece neste owner; o match exaustivo em Content apenas delega estaticamente. Sem vtable, enum novo, campos novos, estado global, I/O ou fase nova. Aceitação: RED→GREEN de reuso, ordem, erro, substituição igual, opcionais/slots e metadados; suite core, builds e lint, qualificação de texto/morfologia/render/query contra release anterior e vanilla pinado; alocações e tempos pareados com proveniência. Não alegar ganho universal a partir de casos isolados.


A prova compartilhada de ausência de mudança obedece às claims desse núcleo. O algoritmo, os campos transformados, sua ordem e a preservação específica permanecem owned por este prompt e seu único consumer.
