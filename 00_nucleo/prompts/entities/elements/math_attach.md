# Prompt L0 — `entities/elements/math_attach` — `MathAttachElem`
Hash do Código: 95f70e3d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715

**Camada**: L1
**Alvo**: `01_core/src/entities/elements/math_attach.rs`
**ADRs**: ADR-0105, ADR-0107, ADR-0129

## Responsabilidade

`MathAttachElem` é o payload fechado e não-locatável dos anexos matemáticos.
Ele preserva, para cada slot, a diferença morfológica entre argumento omitido,
`none` explícito e conteúdo presente.

## Tipos canônicos

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum MathAttachSlot {
    Omitted,
    ExplicitNone,
    Present(Content),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathAttachElem {
    pub base: Content,
    pub t: MathAttachSlot,
    pub b: MathAttachSlot,
    pub tl: MathAttachSlot,
    pub bl: MathAttachSlot,
    pub tr: MathAttachSlot,
    pub br: MathAttachSlot,
}
```

`Content::MathAttach(Arc<MathAttachElem>)` é o único payload. O construtor
canônico é `Content::math_attach(base, t, b, tl, bl, tr, br)`.

`Omitted` representa ausência do argumento, `ExplicitNone` representa o
valor Typst `none` e `Present(content)` inclui `Present(Content::Empty)`
para markup `[]`. É proibido substituir os três estados por sentinel de
`Content`, texto, span ou heurística.

`sup()` devolve primeiro `tr: Present` e depois `t: Present`; `sub()`
faz o paralelo `br`/ `b`. Estados omitidos ou `none` não bloqueiam esse
fallback.

## Element

- `plain_text` concatena `^tl _bl base ^t _b ^tr _br` somente para slots
  `Present`.
- `map_content` transforma recursivamente a base e cada conteúdo `Present`,
  preservando os três estados.
- `map_text` é terminal e devolve o elemento clonado.
- `is_empty`, `get_field`, `element_kind` e `to_payload` conservam os
  defaults da família não-locatável.
- Igualdade e hash incluem base e os seis slots.

## Aceitação

Sintaxe e `math.attach` compartilham o payload. `repr` omite `Omitted`,
imprime `none` para `ExplicitNone` e `[]` para conteúdo vazio presente.
Layout materializa somente `Present`; por isso markup vazio ainda reserva
`SpaceAfterScript`, enquanto omissão e `none` são equivalentes no layout.

## P1444 — carrier público de primes (MATERIALIZADO)

### Medição anterior à decisão

O mesmo ficheiro vanilla que declara `AttachElem` declara `PrimesElem` com
campo público `count: usize`. O cristalino preserva apenas glyphs textuais e
não consegue expor identidade, count ou igualdade do elemento.

### Decisão proposta

Adicionar neste owner coeso `MathPrimesElem { pub count: usize }`, com
`Debug/Clone/PartialEq/Hash`. `get_field("count")` devolve o inteiro; qualquer
outro campo permanece ausente. `plain_text` usa U+2032/U+2033/U+2034/U+2057
para 1–4, repetição de U+2032 acima de 4 e vazio para zero. O elemento não tem
filhos; `map_content` e `map_text` conservam count e identidade. Não alojar
geometria neste ficheiro nem representar primes como `MathText`.

## Reuso de contêineres sem substituição — 2026-10-10

Medição anterior à decisão: `lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz` captura HEAD/working tree/UTC/fontes completas/owners/release. `01_core/src/entities/elements/math_attach.rs:45` conserva a travessia vigente e materializa o elemento mesmo quando os descendentes não mudam. `01_core/src/entities/content/traversal.rs:111–192` embala os resultados destes owners em Some incondicionalmente. Isso é custo mecânico; a linguagem e a identidade semântica dos elementos são preservadas. Inferência refutável: propagar a ausência explícita de mudança reduz alocações e retenção temporária; as sondas e os controles de tempo podem refutar o ganho.

Refinamento interno ADR-0127: disponibilizar somente helper crate-private map_content_if_changed nos elementos já owned por este consumer. Retornar None apenas se nenhum filho efetivamente visitado foi substituído; terminais sem filhos retornam None. Transformar filhos na mesma ordem do map_content vigente, uma única vez, e interromper no primeiro erro. Some do callback, inclusive conteúdo igual, sempre é mudança. Preservar todos os campos, valores, presença explícita, slots omitidos/none/presentes, flags, spans e metadados; não usar igualdade/hash/endereço como certificado. Vetores/linhas só alocam após o primeiro filho/linha alterado; os irmãos conservam seu conteúdo compartilhado. Assinaturas públicas e métodos de trait ficam iguais, com adaptação interna à nova prova; map_content_pre e map_text mantêm seu contrato anterior. Nenhuma ratificação de contratos históricos propostos é ampliada por este refinamento.

A lógica específica permanece neste owner; o match exaustivo em Content apenas delega estaticamente. Sem vtable, enum novo, campos novos, estado global, I/O ou fase nova. Aceitação: RED→GREEN de reuso, ordem, erro, substituição igual, opcionais/slots e metadados; suite core, builds e lint, qualificação de texto/morfologia/render/query contra release anterior e vanilla pinado; alocações e tempos pareados com proveniência. Não alegar ganho universal a partir de casos isolados.


A prova compartilhada de ausência de mudança obedece às claims desse núcleo. O algoritmo, os campos transformados, sua ordem e a preservação específica permanecem owned por este prompt e seu único consumer.
