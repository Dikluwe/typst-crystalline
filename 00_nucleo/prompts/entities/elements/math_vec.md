# Prompt L0 — `entities/elements/math_vec` — `MathVecElem`
Hash do Código: 1c388636

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a

**Estado:** P1292 MATERIALIZADO.

**Camada:** L1
**Alvo planejado:** `01_core/src/entities/elements/math_vec.rs`
**Regras comuns:** `entities/elements/_comum.md`
**Vanilla ratificado:** `a51e02804`

## Medição anterior à decisão

O cristalino degrada `vec` a `MathMatrix` em
`01_core/src/compiler/eval/math.rs:1030-1062`. O vanilla declara `VecElem`
distinto em `lab/typst-original/crates/typst-library/src/math/matrix.rs:18-68`,
com delimitador, alinhamento horizontal, gap relativo e filhos variádicos.
Probes de 2026-08-31 confirmam que zero filhos é válido e conserva identidade
`vec(children: ())`.

## Contrato público P1292

```rust
pub struct MathVecElem {
    pub children: Vec<Content>,
    pub delim: (char, char),
    pub align: HAlign,
    pub gap: Rel<Length>,
    pub explicit: MathVecExplicit,
}
```

Defaults: `delim = ('(', ')')`, `align = HAlign::Center`,
`gap = Rel { rel: 0, abs: 0.2em }`. `MathVecExplicit` conserva um bit para
cada named `delim`, `align` e `gap`; os bits não mudam layout, mas participam
da morfologia/identidade para que um default explicitamente escrito continue
no `repr`. O sentinel de delimitador ausente segue a
convenção tipada já usada por matrix/cases; não se converte `none` em glifo.
`Start` e `End` permanecem valores distintos no payload.

Elemento math estrutural não-locatável. `plain_text` concatena os filhos em
ordem; `map_content` recursa em cada filho e preserva `delim/align/gap`;
`map_text` é terminal; igualdade/hash incluem todos os campos pela convenção
canónica para escalares, inclusive presença. Zero filhos não é normalizado
para conteúdo vazio nem para matrix. O constructor canónico transporta
`children`, `delim`, `align`, `gap` e presença; uma conveniência Rust sem named
inicializa os três bits como omitidos.

O percentual de `gap` permanece dado não resolvido. A resolução geométrica é
obrigação de `compiler/math/layout/vec.md` e é fechada pelo contrato
`P1292.vec-region-gap` de `compiler/math/layout/_comum.md`; usar `em` como base
do percentual é proibido.

## Aceitação linguística

A entidade preserva identidade vetorial, cardinalidade/ordem dos filhos,
delimitadores inferidos, alinhamento e gap. Os casts e erros pertencem a
`compiler/stdlib/structural/math.md` e `compiler/eval/math.md`.

A forma default é `vec(children: ())` ou
`vec(children: ([a], [b]))`. Named presentes aparecem antes de `children`, em
ordem `delim`, `align`, `gap`; delimiter é normalizado como par, `gap` como
relative length. Assim `delim: "["` aparece `delim: ("[", "]")` e
`gap: 1em` aparece `gap: 0% + 1em`. Singleton em `children` conserva vírgula.

## Reuso de contêineres sem substituição — 2026-10-10

Medição anterior à decisão: `lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz` captura HEAD/working tree/UTC/fontes completas/owners/release. `01_core/src/entities/elements/math_vec.rs:60` conserva a travessia vigente e materializa o elemento mesmo quando os descendentes não mudam. `01_core/src/entities/content/traversal.rs:111–192` embala os resultados destes owners em Some incondicionalmente. Isso é custo mecânico; a linguagem e a identidade semântica dos elementos são preservadas. Inferência refutável: propagar a ausência explícita de mudança reduz alocações e retenção temporária; as sondas e os controles de tempo podem refutar o ganho.

Refinamento interno ADR-0127: disponibilizar somente helper crate-private map_content_if_changed nos elementos já owned por este consumer. Retornar None apenas se nenhum filho efetivamente visitado foi substituído; terminais sem filhos retornam None. Transformar filhos na mesma ordem do map_content vigente, uma única vez, e interromper no primeiro erro. Some do callback, inclusive conteúdo igual, sempre é mudança. Preservar todos os campos, valores, presença explícita, slots omitidos/none/presentes, flags, spans e metadados; não usar igualdade/hash/endereço como certificado. Vetores/linhas só alocam após o primeiro filho/linha alterado; os irmãos conservam seu conteúdo compartilhado. Assinaturas públicas e métodos de trait ficam iguais, com adaptação interna à nova prova; map_content_pre e map_text mantêm seu contrato anterior. Nenhuma ratificação de contratos históricos propostos é ampliada por este refinamento.

A lógica específica permanece neste owner; o match exaustivo em Content apenas delega estaticamente. Sem vtable, enum novo, campos novos, estado global, I/O ou fase nova. Aceitação: RED→GREEN de reuso, ordem, erro, substituição igual, opcionais/slots e metadados; suite core, builds e lint, qualificação de texto/morfologia/render/query contra release anterior e vanilla pinado; alocações e tempos pareados com proveniência. Não alegar ganho universal a partir de casos isolados.


A prova compartilhada de ausência de mudança obedece às claims desse núcleo. O algoritmo, os campos transformados, sua ordem e a preservação específica permanecem owned por este prompt e seu único consumer.
