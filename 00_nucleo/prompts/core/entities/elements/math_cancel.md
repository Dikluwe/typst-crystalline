# Prompt L0 — `entities/elements/math_cancel` — `MathCancelElem`
Hash do Código: e56b151f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_cancel.rs`
**Origem**: modelo D (ADR-0105), **Lote 2 P317** (família math). Trait, regras
partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`. **Não-locatável** (confirmado
P317). P1291 materializou o payload completo e a realização de callback. P1292
não cria outro elemento nem outro runtime: fecha a exposição canónica em
`math.cancel` e a presença morfológica dos argumentos.

---

## Histórico P317 — payload minimal body-only (REVOGADO por P1291)

O bloco seguinte registra a forma materializada originalmente em P317 para
explicar a linhagem. Ele não é contrato vigente e não autoriza reintroduzir um
segundo `MathCancelElem` body-only:

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathCancelElem {
    pub body: Content,   // era Box<Content>
}
```

`Content::MathCancel { body }` → `Content::MathCancel(Arc<MathCancelElem>)`.
Construtor ergonómico: `Content::math_cancel(body: Content)`.

### `impl Element` histórico

| método | comportamento (idêntico ao braço atual) |
|---|---|
| `plain_text` | `self.body.plain_text()` (`content.rs:1646`) |
| `is_empty` | default `false` (`content.rs:1568`) |
| `map_content` | **recursivo** no `body` (`content.rs:2135`): `Content::MathCancel(Arc::new(MathCancelElem { body: self.body.map_content(f)? }))` |
| `map_text` | **terminal** (math structural; `content.rs:2632`): `Content::MathCancel(Arc::new(self.clone()))` |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

### `eq` estrutural histórico

`#[derive(PartialEq)]` compara `body` (paridade `content.rs:1833`).

### Critério preservado do payload histórico

`plain_text` transparente ao body; `map_content` recurse o body; `map_text`
terminal; igualdade estrutural.

## P1292 — payload completo já materializado e exposição canónica

### Medição anterior à decisão

- Antes de P1291, o consumer cristalino em
  `01_core/src/entities/elements/math_cancel.rs:18-21` transportava somente
  `body`; essa medição é histórica e foi superada pela materialização full.
- O vanilla ratificado `a51e02804`, em
  `lab/typst-original/crates/typst-library/src/math/cancel.rs:18-115`, declara
  `body`, `length`, `inverted`, `cross`, `angle`, `stroke` e `background`;
  `angle` aceita ângulo ou função e o valor público também admite `auto`.
- Probes de 2026-08-31 registraram a ordem morfológica
  `body,length,inverted,cross,angle,stroke,background` e os domínios de erro no
  diagnóstico histórico. A forma de `repr` e a
  distinção entre default omitido e explícito continuam sob P1290; estes dados
  legitimam o transporte sem transferir ownership de `repr.rs`.

### Contrato vigente e delta de exposição P1292

O payload minimal histórico acima foi substituído em P1291. A única forma
vigente, completada em P1292 com presença morfológica, é:

```rust
pub enum MathCancelAngle {
    Auto,
    Angle(Angle),
    Func(Func),
}

pub struct MathCancelElem {
    pub body: Content,
    pub length: Rel<Length>,
    pub inverted: bool,
    pub cross: bool,
    pub angle: MathCancelAngle,
    pub stroke: Option<Stroke>,
    pub background: bool,
    pub span: Span,
    // Metadado de presença, sem efeito geométrico: um bit para cada named.
    pub explicit: MathCancelExplicit,
}
```

Defaults semânticos: `length = 100% + 0.3em`, `inverted = false`,
`cross = false`, `angle = Auto`, `stroke = None`, `background = false`.
`stroke = None` significa exclusivamente o traço MATH derivado no consumo:
espessura `0.05em` e paint do texto ativo; não significa ausência de linha.
Um `Stroke` explícito preserva todos os seus campos. `cross` prevalece sobre
`inverted`, mas ambos permanecem transportados porque são observáveis como
argumentos/set rules.

`span` preserva o local da chamada para erro tardio da callback, mas não é
campo da linguagem. `MathCancelExplicit` conserva, separadamente para
`length`, `inverted`, `cross`, `angle`, `stroke` e `background`, se a chave
apareceu na chamada/set-rule. Os bits não mudam layout, mas participam da
morfologia e da identidade observável: um valor explicitamente igual ao
default continua no `repr`. O construtor compatível usa `Span::detached()` e
nenhum bit; a nativa usa `Args::span` e marca exatamente as chaves presentes.
Igualdade/hash ignoram `span`, incluem os sete valores de linguagem e os bits
de presença.

Como `Rel<Length>`, `Angle` e `Stroke` contêm escalares de ponto flutuante, o
hash estrutural deve usar a convenção canónica já vigente para esses tipos; não
se autoriza converter grandezas em strings nem eliminar campos para obter
`derive(Hash)`.

O construtor compatível `Content::math_cancel(body)` permanece e inicializa os
defaults como omitidos. O construtor completo tipado, de ownership de
`entities/content.md`, transporta os sete valores, `span` e presença sem
reexecutar ou reinterpretar callback. `plain_text` e os walkers
continuam transparentes/recursivos somente em `body`; igualdade e hash incluem
todos os campos.

### Runtime preservado

`MathCancelAngle::Func` continua a usar o transcript puro já materializado em
P1291, definido em `compiler/math/layout/callbacks.md` e realizado entre
passagens por `infra/pipeline.md`. P1292 não reimplementa esse caminho. É
proibido executar a callback na entidade/stdlib/layout, ignorá-la ou tratá-la
como `Auto` no documento exportável.

Prompts proprietários relacionados: `entities/content.md`,
`compiler/stdlib/structural/math.md`, `compiler/math/layout/cancel.md` e
`compiler/math/layout/_comum.md`.

## Reuso de contêineres sem substituição — 2026-10-10

Medição anterior à decisão: `lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz` captura HEAD/working tree/UTC/fontes completas/owners/release. `01_core/src/entities/elements/math_cancel.rs:109` conserva a travessia vigente e materializa o elemento mesmo quando os descendentes não mudam. `01_core/src/entities/content/traversal.rs:111–192` embala os resultados destes owners em Some incondicionalmente. Isso é custo mecânico; a linguagem e a identidade semântica dos elementos são preservadas. Inferência refutável: propagar a ausência explícita de mudança reduz alocações e retenção temporária; as sondas e os controles de tempo podem refutar o ganho.

Refinamento interno ADR-0127: disponibilizar somente helper crate-private map_content_if_changed nos elementos já owned por este consumer. Retornar None apenas se nenhum filho efetivamente visitado foi substituído; terminais sem filhos retornam None. Transformar filhos na mesma ordem do map_content vigente, uma única vez, e interromper no primeiro erro. Some do callback, inclusive conteúdo igual, sempre é mudança. Preservar todos os campos, valores, presença explícita, slots omitidos/none/presentes, flags, spans e metadados; não usar igualdade/hash/endereço como certificado. Vetores/linhas só alocam após o primeiro filho/linha alterado; os irmãos conservam seu conteúdo compartilhado. Assinaturas públicas e métodos de trait ficam iguais, com adaptação interna à nova prova; map_content_pre e map_text mantêm seu contrato anterior. Nenhuma ratificação de contratos históricos propostos é ampliada por este refinamento.

A lógica específica permanece neste owner; o match exaustivo em Content apenas delega estaticamente. Sem vtable, enum novo, campos novos, estado global, I/O ou fase nova. Aceitação: RED→GREEN de reuso, ordem, erro, substituição igual, opcionais/slots e metadados; suite core, builds e lint, qualificação de texto/morfologia/render/query contra release anterior e vanilla pinado; alocações e tempos pareados com proveniência. Não alegar ganho universal a partir de casos isolados.


A prova compartilhada de ausência de mudança obedece às claims desse núcleo. O algoritmo, os campos transformados, sua ordem e a preservação específica permanecem owned por este prompt e seu único consumer.
