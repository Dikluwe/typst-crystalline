# Prompt L0 — `entities/elements/math_underline` — `MathUnderlineElem`
Hash do Código: c1814644

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a

**Estado:** CONTRATOS P1292 E P1441 MATERIALIZADOS

**Camada:** L1
**Alvo planejado:** `01_core/src/entities/elements/math_underline.rs`
**Regras comuns:** `entities/elements/_comum.md`
**Vanilla ratificado:** `a51e02804`

## Medição anterior à decisão

O cristalino possui `entities/elements/underline.rs`, mas essa unidade é a
decoração textual com `stroke`, `offset` e `extent`. O vanilla declara o
elemento matemático separado e body-only em
`lab/typst-original/crates/typst-library/src/math/underover.rs:4-14`; o probe
de linguagem de 2026-08-31 confirma `math.underline == underline` como `false`.
Logo a equivalência visual parcial não autoriza alias de identidade.

## Contrato público P1292

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathUnderlineElem {
    pub body: Content,
}
```

Elemento math estrutural não-locatável. `plain_text` delega ao body;
`map_content` recursa no body e reconstrói `MathUnderline`; `map_text` é
terminal; igualdade/hash incluem body; campos/introspection de elemento usam
os defaults não-locatáveis. O construtor público é
`Content::math_underline(body)`. A função `math.underline` aceita exatamente
um `Content` posicional obrigatório; strings seguem o cast canónico para
`Content`. Zero posicionais produz `missing argument: body`, tipo
não-convertível produz `expected content, found <tipo vanilla>`, segundo
posicional produz `unexpected argument` e qualquer named produz
`unexpected argument: <nome>`.

É proibido reutilizar `Content::Underline`, transportar campos textuais
`stroke/offset/extent` ou expor o nome de função textual. A geometria pertence
exclusivamente a `compiler/math/layout/underline.md`, e a variante ao owner
`entities/content.md`.

## Aceitação linguística

`math.underline([x])` preserva identidade `MathUnderline`, plain text `x` e um
único body. Sua forma pública é `underline(body: [x])`, e
`math.underline == underline` permanece `false`. Falta, excesso, tipo inválido
e named desconhecido coincidem com as mensagens medidas no registro vanilla
P1292; não se aceita argumento ignorado.

## P1441 — identidade pública `MathOverlineElem` (APROVADO NO GATE ADR-0127 EM 2026-09-18)

### Medição anterior à decisão

O vanilla declara `OverlineElem` matemático body-only, separado tanto de
`UnderlineElem` matemático quanto da decoração textual `overline` com stroke,
offset e extent. O cristalino possui as duas famílias adjacentes, mas ainda
não a identidade math superior.

### Decisão proposta

No mesmo owner coeso, adicionar `MathOverlineElem { body: Content }`.
`plain_text` delega ao body; `map_content` desce no body e reconstrói
`MathOverline`; `map_text` conserva a variante; igualdade/hash incluem body.
É proibido reutilizar `Content::Overline` textual ou inferir esta identidade
da presença de uma regra acima.

## Reuso de contêineres sem substituição — 2026-10-10

Medição anterior à decisão: `lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz` captura HEAD/working tree/UTC/fontes completas/owners/release. `01_core/src/entities/elements/math_underline.rs:33` conserva a travessia vigente e materializa o elemento mesmo quando os descendentes não mudam. `01_core/src/entities/content/traversal.rs:111–192` embala os resultados destes owners em Some incondicionalmente. Isso é custo mecânico; a linguagem e a identidade semântica dos elementos são preservadas. Inferência refutável: propagar a ausência explícita de mudança reduz alocações e retenção temporária; as sondas e os controles de tempo podem refutar o ganho.

Refinamento interno ADR-0127: disponibilizar somente helper crate-private map_content_if_changed nos elementos já owned por este consumer. Retornar None apenas se nenhum filho efetivamente visitado foi substituído; terminais sem filhos retornam None. Transformar filhos na mesma ordem do map_content vigente, uma única vez, e interromper no primeiro erro. Some do callback, inclusive conteúdo igual, sempre é mudança. Preservar todos os campos, valores, presença explícita, slots omitidos/none/presentes, flags, spans e metadados; não usar igualdade/hash/endereço como certificado. Vetores/linhas só alocam após o primeiro filho/linha alterado; os irmãos conservam seu conteúdo compartilhado. Assinaturas públicas e métodos de trait ficam iguais, com adaptação interna à nova prova; map_content_pre e map_text mantêm seu contrato anterior. Nenhuma ratificação de contratos históricos propostos é ampliada por este refinamento.

A lógica específica permanece neste owner; o match exaustivo em Content apenas delega estaticamente. Sem vtable, enum novo, campos novos, estado global, I/O ou fase nova. Aceitação: RED→GREEN de reuso, ordem, erro, substituição igual, opcionais/slots e metadados; suite core, builds e lint, qualificação de texto/morfologia/render/query contra release anterior e vanilla pinado; alocações e tempos pareados com proveniência. Não alegar ganho universal a partir de casos isolados.


A prova compartilhada de ausência de mudança obedece às claims desse núcleo. O algoritmo, os campos transformados, sua ordem e a preservação específica permanecem owned por este prompt e seu único consumer.
