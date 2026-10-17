# Prompt L0 — `entities/elements/math_frac` — `MathFracElem`
Hash do Código: 5a988015

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_frac.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathFracElem {
    pub num: Content,   // era Box<Content>
    pub den: Content,   // era Box<Content>
    pub line: bool,
}
```

`Content::MathFrac { num, den }` → `Content::MathFrac(Arc<MathFracElem>)`.
`Content::math_frac(num, den)` fixa `line: true`; o construtor interno
`Content::math_frac_unlined(num, den)` fixa `line: false` para `binom`.

## `impl Element for MathFracElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `format!("({})/({})", self.num.plain_text(), self.den.plain_text())` (`content.rs:1613`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** em `num`/`den` (`content.rs:2096`): `Content::MathFrac(Arc::new(MathFracElem { num: self.num.map_content(f)?, den: self.den.map_content(f)? }))` |
| `map_text` | **terminal** (math structural; `content.rs:2621`): `Content::MathFrac(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `num + den + line`.

## Critério

`plain_text` formato `(num)/(den)`; `map_content` recurse ambos; `map_text`
terminal; igualdade estrutural.

## P1132f — modo frac-like sem barra para `binom`

**Medição antes da decisão** (secção 7, `HEAD 781b207b4a5d`, working tree
não commitado, 2026-08-22): `binom(n,k)` cristalino usa `MathMatrix` e mede
13,156pt entre as baselines de `n` e `k`; o vanilla mede 14,993pt e resolve
`BinomElem` pela mesma `FractionItem` de `frac`, com `line = false`
(`ir/resolve.rs:715-759`). A diferença desloca as equações posteriores e não
pode ser corrigida por padding empírico de matriz.

**Decisão de contrato**: `MathFracElem` passa a transportar `pub line: bool`.
O valor integra a morfologia e a igualdade estrutural. Frações existentes
continuam com `true`; `binom` usa `false` e envolve a caixa em
`MathDelimited('(', ..., ')')`. Campo público novo: gate humano ADR-0127
obrigatório antes do código.

## P1427 — carrier completo de `math.frac` (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

No vanilla pinado, `math/frac.rs:24-123` define `num`, `denom`, style público
`vertical | skewed | horizontal` e dois bits internos que registam a remoção
de um par exterior. O carrier cristalino mede apenas `num`, `den` e `line`.

### Decisão proposta

Adicionar `MathFracStyle::{Vertical, Skewed, Horizontal}`, estilo opcional
explícito e `num_deparenthesized`/`denom_deparenthesized`. A ausência do estilo
é causalmente distinta de `Vertical` explícito. `line` permanece apenas para
o caminho interno de `binom`: quando falso, estilo e bits não o convertem em
fração pública nem lhe devolvem a barra. `map_content`, igualdade e
reconstrução preservam todos os campos; só `num` e `denom` são transformados.

## Reuso de contêineres sem substituição — 2026-10-10

Medição anterior à decisão: `00_nucleo/diagnosticos/content-container-reuse-2026-10-10/before.json.gz` captura HEAD/working tree/UTC/fontes completas/owners/release. `01_core/src/entities/elements/math_frac.rs:39` conserva a travessia vigente e materializa o elemento mesmo quando os descendentes não mudam. `01_core/src/entities/content/traversal.rs:111–192` embala os resultados destes owners em Some incondicionalmente. Isso é custo mecânico; a linguagem e a identidade semântica dos elementos são preservadas. Inferência refutável: propagar a ausência explícita de mudança reduz alocações e retenção temporária; as sondas e os controles de tempo podem refutar o ganho.

Refinamento interno ADR-0127: disponibilizar somente helper crate-private map_content_if_changed nos elementos já owned por este consumer. Retornar None apenas se nenhum filho efetivamente visitado foi substituído; terminais sem filhos retornam None. Transformar filhos na mesma ordem do map_content vigente, uma única vez, e interromper no primeiro erro. Some do callback, inclusive conteúdo igual, sempre é mudança. Preservar todos os campos, valores, presença explícita, slots omitidos/none/presentes, flags, spans e metadados; não usar igualdade/hash/endereço como certificado. Vetores/linhas só alocam após o primeiro filho/linha alterado; os irmãos conservam seu conteúdo compartilhado. Assinaturas públicas e métodos de trait ficam iguais, com adaptação interna à nova prova; map_content_pre e map_text mantêm seu contrato anterior. Nenhuma ratificação de contratos históricos propostos é ampliada por este refinamento.

A lógica específica permanece neste owner; o match exaustivo em Content apenas delega estaticamente. Sem vtable, enum novo, campos novos, estado global, I/O ou fase nova. Aceitação: RED→GREEN de reuso, ordem, erro, substituição igual, opcionais/slots e metadados; suite core, builds e lint, qualificação de texto/morfologia/render/query contra release anterior e vanilla pinado; alocações e tempos pareados com proveniência. Não alegar ganho universal a partir de casos isolados.


A prova compartilhada de ausência de mudança obedece às claims desse núcleo. O algoritmo, os campos transformados, sua ordem e a preservação específica permanecem owned por este prompt e seu único consumer.
