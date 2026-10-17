# Prompt L0 — `entities/elements/math_delimited` — `MathDelimitedElem`
Hash do Código: fce9b931

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_delimited.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). Comportamento idêntico ao braço atual do hub.

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathDelimitedElem {
    pub open:  char,
    pub body:  Content,   // era Box<Content>
    pub close: char,
}
```

`Content::MathDelimited { open, body, close }` →
`Content::MathDelimited(Arc<MathDelimitedElem>)`. Construtor ergonómico:
`Content::math_delimited(open: char, body: Content, close: char)`.

## `impl Element for MathDelimitedElem`

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `format!("{}{}{}", self.open, self.body.plain_text(), self.close)` (`content.rs:1629`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** só no `body`, `open`/`close` preservados (`content.rs:2111`): `Content::MathDelimited(Arc::new(MathDelimitedElem { open: self.open, body: self.body.map_content(f)?, close: self.close }))` |
| `map_text` | **terminal** (math structural; `content.rs:2624`): `Content::MathDelimited(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `open + body + close` (paridade `content.rs:1822`).

## Critério

`plain_text` `{open}{body}{close}`; `map_content` recurse só o body; `map_text`
terminal; igualdade estrutural.

## P1436 — carrier público `MathLrElem` (APROVADO NO GATE ADR-0127 EM 2026-09-17)

### Medição anterior à decisão

O vanilla pinado declara `LrElem { size: Rel<Length>, body: Content }`; o
carrier cristalino deste owner representa apenas o par mecânico já separado
`open/body/close`. Reutilizar `MathDelimitedElem` apagaria `lr` sem cercas,
body variádico exato, cascata e presença explícita de `size`.

### Decisão proposta

Este consumer passa a possuir também `MathLrElem`, semanticamente distinto de
`MathDelimitedElem`, com `body: Content`, `size: Rel<Length>` e
`size_explicit: bool`. Derivar `Debug`, `Clone`, `PartialEq` e `Hash` sobre os
três campos. `plain_text` delega ao body; `map_content` transforma somente o
body e reconstrói tamanho/presença byte a byte; `map_text` permanece terminal
como os demais carriers matemáticos estruturais. Cast, junção de posicionais,
cascata e stretch não pertencem a esta entidade.

`MathDelimitedElem` e seu constructor permanecem inalterados para usos
mecânicos existentes. Não inferir `MathLrElem` a partir de igualdade de
delimitadores.

### P1447 — projeção pública dos campos de `MathLrElem`

O carrier expõe `body` sempre e `size` somente quando `size_explicit=true`.
O valor de `size` é `Value::Relative(self.size)` e o body é o próprio conteúdo
armazenado. O default e a cascata geométrica não fabricam presença. Esta
projeção serve todas as construções `MathLr`, inclusive `math.abs`; não cria
campos próprios de `abs` nem altera `MathDelimitedElem`.

## P1437 — carrier público `MathMidElem` (PROPOSTA; AGUARDA GATE ADR-0127)

### Medição anterior à decisão

O vanilla pinado declara `MidElem { body: Content }`: um único campo público,
obrigatório e posicional. A identidade persiste no `repr` e participa da
resolução do `lr` mais próximo; não equivale a `MathDelimitedElem` nem ao
body nu.

### Decisão proposta

Este consumer passa a possuir `MathMidElem { body: Content }`, com `Debug`,
`Clone`, `PartialEq` e `Hash`. `plain_text` delega ao body;
`map_content` transforma só o body; `map_text` permanece terminal. Cast,
classe `Relation` e stretch pertencem a avaliação e layout.

## Reuso de contêineres sem substituição — 2026-10-10

Medição anterior à decisão: `00_nucleo/diagnosticos/content-container-reuse-2026-10-10/before.json.gz` captura HEAD/working tree/UTC/fontes completas/owners/release. `01_core/src/entities/elements/math_delimited.rs:65` conserva a travessia vigente e materializa o elemento mesmo quando os descendentes não mudam. `01_core/src/entities/content/traversal.rs:111–192` embala os resultados destes owners em Some incondicionalmente. Isso é custo mecânico; a linguagem e a identidade semântica dos elementos são preservadas. Inferência refutável: propagar a ausência explícita de mudança reduz alocações e retenção temporária; as sondas e os controles de tempo podem refutar o ganho.

Refinamento interno ADR-0127: disponibilizar somente helper crate-private map_content_if_changed nos elementos já owned por este consumer. Retornar None apenas se nenhum filho efetivamente visitado foi substituído; terminais sem filhos retornam None. Transformar filhos na mesma ordem do map_content vigente, uma única vez, e interromper no primeiro erro. Some do callback, inclusive conteúdo igual, sempre é mudança. Preservar todos os campos, valores, presença explícita, slots omitidos/none/presentes, flags, spans e metadados; não usar igualdade/hash/endereço como certificado. Vetores/linhas só alocam após o primeiro filho/linha alterado; os irmãos conservam seu conteúdo compartilhado. Assinaturas públicas e métodos de trait ficam iguais, com adaptação interna à nova prova; map_content_pre e map_text mantêm seu contrato anterior. Nenhuma ratificação de contratos históricos propostos é ampliada por este refinamento.

A lógica específica permanece neste owner; o match exaustivo em Content apenas delega estaticamente. Sem vtable, enum novo, campos novos, estado global, I/O ou fase nova. Aceitação: RED→GREEN de reuso, ordem, erro, substituição igual, opcionais/slots e metadados; suite core, builds e lint, qualificação de texto/morfologia/render/query contra release anterior e vanilla pinado; alocações e tempos pareados com proveniência. Não alegar ganho universal a partir de casos isolados.


A prova compartilhada de ausência de mudança obedece às claims desse núcleo. O algoritmo, os campos transformados, sua ordem e a preservação específica permanecem owned por este prompt e seu único consumer.
