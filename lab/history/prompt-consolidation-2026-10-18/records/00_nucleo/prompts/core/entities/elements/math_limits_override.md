# Prompt L0 — `entities/elements/math_limits_override` — `MathLimitsOverrideElem`
Hash do Código: de54ac83

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_limits_override.rs`
**Origem**: Passo 992 (achado externo 2026-08-07, secção 32 do documento
estendido). Trait, regras partilhadas e glossário (§A.0): ver `entities/elements/_comum.md`.
**Não-locatável** (paralelo P296/P298/P772y —
`MathAccent`/`MathCancel`/`MathOp`/`MathClassOverride`). Mecanismo vanilla:
**dois** elementos separados, `ScriptsElem { #[required] body: Content }` e
`LimitsElem { #[required] body: Content, #[default(true)] inline: bool }`
(`typst-library/src/math/attach.rs`).

---

## Contexto

`scripts(body)` força `body`, quando usado como base de um `MathAttach`
(`^`/`_`), a exibir os scripts **sempre lateralmente** (nunca empilhados),
mesmo que `body` seja naturalmente um operador grande ou função de limite
(`scripts(sum)_1^2 != sum_1^2`, doc vanilla). `limits(body, inline: true)`
força o oposto — scripts **sempre empilhados** — para qualquer base,
mesmo uma que não qualificaria naturalmente (`limits(A)_1^2 != A_1^2`).
`inline` (só significativo quando a intenção é empilhar) controla se o
empilhamento se aplica também fora de modo bloco/display — vanilla
`resolve_limits`: `inline=true → Limits::Always` (empilha sempre,
inline incluído); `inline=false → Limits::Display` (só em bloco, mesma
regra usada por omissão para um operador grande sem wrapper).

> **Fonte de paridade**: documentação
> `https://typst.app/docs/reference/math/attach/#functions-scripts` e
> `.../#functions-limits` (corpus
> `lab/corpus/docs/math/attach.typ:18-28`); guardas end-to-end em
> `01_core/src/compiler/math/layout/tests.rs:7560-7760`
> (`p992_tests`).

**Cristalino consolida os dois elementos vanilla num só** (ADR-0107 —
paridade é com a língua, não com a mecânica): `MathLimitsOverrideElem {
body, limits: bool, inline: bool }`, onde `limits` é o discriminador
scripts()=false / limits()=true, e `inline` só é lido quando
`limits == true` (paralelo à consolidação já feita para `binom()` via
`math_matrix`, P899 Parte D). `scripts(body)` constrói com
`limits: false` (o campo `inline` fica `true` por convenção mas é
ignorado — ver fórmula em `math/layout/attach.md` §P992).

> **Fonte de paridade**: mapeamento `inline=true/false` ↔ `Limits::Always/Display`
> confirmado no vanilla
> `lab/typst-original/crates/typst-library/src/math/attach.rs`
> (`LimitsElem::set_limits`) e
> `lab/typst-original/crates/typst-layout/src/math/ir/resolve.rs`
> (`resolve_limits`); guardas em
> `01_core/src/compiler/math/layout/tests.rs:7632-7683`
> (`p992_limits_forca_empilhamento_mesmo_inline` e
> `p992_limits_inline_false_respeita_modo`).

Não afecta `MathClass`/espaçonto do `body` (vanilla `resolve_scripts`/
`resolve_limits`: resolvem o `body` normalmente, só sobrescrevem
`item.set_limits(...)` no item já resolvido) — **diferente** de
`MathClassOverride`, que força a classe. `body` é layoutado normalmente
via `MathLayouter::layout_node` e recursa em `apply_math_default` (ao
contrário de `MathClassOverride`, que não recursa — ver
`math/layout/_comum.md` §P992: o caso de uso canónico de `limits()`/
`scripts()` inclui bases de 1 letra como `A`, que precisam do itálico por
defeito).

> **Fonte de paridade**: vanilla
> `lab/typst-original/crates/typst-layout/src/math/ir/resolve.rs`
> (`resolve_scripts`/`resolve_limits`); guarda de transparência de classe
> em `01_core/src/compiler/math/layout/tests.rs:7765`
> (`p992_scripts_nao_muda_classe_do_body`) e de itálico por defeito em
> `:7736` (`p992_limits_preserva_italico_por_defeito`).

---

## Struct

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathLimitsOverrideElem {
    pub body: Content,
    pub limits: bool,
    pub inline: bool,
    pub inline_explicit: bool,
}
```

`Content::MathLimitsOverride(Arc<MathLimitsOverrideElem>)`.
Construtor ergonómico: `Content::math_limits_override(body: Content, limits:
bool, inline: bool, inline_explicit: bool)`.

### P1425 — presença morfológica de `inline` (APROVADO no gate ADR-0127)

`inline_explicit` distingue omissão do named de presença explícita, inclusive
quando o valor fornecido é igual ao default `true`. A sintaxe bare e
`math.limits` qualificado preenchem o bit a partir da lista causal de
argumentos. `scripts` usa sempre `false`, pois `inline` não integra sua
assinatura.

O bit não altera geometria, classe, vazio ou texto. Walkers, igualdade e toda
reconstrução preservam-no. A `repr` usa-o exclusivamente para decidir se emite
`inline: <bool>`. Não inferir presença comparando o valor com o default.

Este campo público e a assinatura do constructor foram confirmados pelo dono
no gate P1425 em 2026-09-16.

## `impl Element for MathLimitsOverrideElem`

| método | comportamento |
|---|---|
| `plain_text` | `self.body.plain_text()` |
| `is_empty` | `self.body.is_empty()` (delega, mesmo padrão de `MathClassOverrideElem`) |
| `map_content` | **recursivo** no `body`, preservando `limits`, `inline` e `inline_explicit` |
| `map_text` | **terminal** (família math não desce em `map_text`) |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `body`, `limits`, `inline` e
`inline_explicit`.

## Despacho no hub (`content.rs`)

Mesmo padrão dos restantes membros da família math (mirror de
`MathClassOverride`): Debug/Display (`"math.limits({:?})"`), constructor,
`plain_text`, `PartialEq`, `map_content` (recursivo), `map_text`
(terminal — bloco de clonagem directa). Também precisa de braço em
`compiler/introspect.rs` (2×, não-locatável/terminal),
`compiler/introspect/locatable.rs` (não-locatável), `compiler/layout/mod.rs`
(fallback de math fora de contexto — `layout_math_fallback`),
`compiler/eval/repr.rs` (`limits(<repr(body)>, inline: <bool>)` quando
`limits=true`; `scripts(<repr(body)>)` quando `limits=false`) e
`03_infra/src/query_helpers.rs` (2×, terminal sem texto próprio).

**Adicional face ao padrão `MathClassOverride`**: `apply_math_default`
(`math/layout/_comum.md` §P992) ganha braço recursivo — `MathClassOverride`
não tem este braço (gap pré-existente, fora de escopo aqui), mas o caso de
uso de `limits()`/`scripts()` inclui bases de 1 letra que precisam do
itálico por defeito. `apply_math_style` (`bb()`/`bold()`/etc., P311b.4)
**não** ganha braço — cai no catch-all `other => other.clone()`, mesmo
estado que `MathClassOverride` já tem aí (fora de escopo, não é regressão).

## Critério

`plain_text`/`is_empty` transparentes ao body; `map_content` recurse o body
preservando `limits`/`inline`; `map_text` terminal (clona directamente,
campos preservados); igualdade estrutural compara `body`, `limits` e
`inline`; `apply_math_default` recursa no `body` (itálico por defeito
preservado); `base_math_class`/`node_math_class` transparentes ao `body`
(sem override de classe).

## P992b — paridade de erros em named args (implementada, a pedido do dono 2026-08-11)

O débito registado na auditoria pós-P992 foi fechado nesta secção, por
ordem directa do dono ("faça as correções que o passo anterior devia ter
escrito"). Mensagens medidas no vanilla (`/tmp/e1..e3.typ`):

1. `scripts(A, foo: 1)` / `limits(A, foo: 1)` → `error: unexpected
   argument: foo`. O braço de `scripts` (que antes descartava named args
   pelo `filter_map` de posicionais) e o `_ => {}` do loop de `limits`
   passam a acumular `SourceDiagnostic::error` com essa mensagem.
2. `limits(A, inline: 5)` → `error: expected boolean, found content`. O
   valor é avaliado; se não for `Value::Bool`, erro com o nome do tipo
   (`type_name()` — para o caso math-nativo, `5` vira `Value::Content` →
   "content", batendo byte-a-byte com o vanilla medido). Antes o
   `if let Ok(Value::Bool)` descartava o erro e `inline` ficava preso em
   `true`.

Mensagem de erro é observável ao nível da língua (ADR-0107). Correcção de
paridade — fluxo contínuo (ADR-0127), sem gate.

## Reuso de contêineres sem substituição — 2026-10-10

Medição anterior à decisão: `lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz` captura HEAD/working tree/UTC/fontes completas/owners/release. `01_core/src/entities/elements/math_limits_override.rs:39` conserva a travessia vigente e materializa o elemento mesmo quando os descendentes não mudam. `01_core/src/entities/content/traversal.rs:111–192` embala os resultados destes owners em Some incondicionalmente. Isso é custo mecânico; a linguagem e a identidade semântica dos elementos são preservadas. Inferência refutável: propagar a ausência explícita de mudança reduz alocações e retenção temporária; as sondas e os controles de tempo podem refutar o ganho.

Refinamento interno ADR-0127: disponibilizar somente helper crate-private map_content_if_changed nos elementos já owned por este consumer. Retornar None apenas se nenhum filho efetivamente visitado foi substituído; terminais sem filhos retornam None. Transformar filhos na mesma ordem do map_content vigente, uma única vez, e interromper no primeiro erro. Some do callback, inclusive conteúdo igual, sempre é mudança. Preservar todos os campos, valores, presença explícita, slots omitidos/none/presentes, flags, spans e metadados; não usar igualdade/hash/endereço como certificado. Vetores/linhas só alocam após o primeiro filho/linha alterado; os irmãos conservam seu conteúdo compartilhado. Assinaturas públicas e métodos de trait ficam iguais, com adaptação interna à nova prova; map_content_pre e map_text mantêm seu contrato anterior. Nenhuma ratificação de contratos históricos propostos é ampliada por este refinamento.

A lógica específica permanece neste owner; o match exaustivo em Content apenas delega estaticamente. Sem vtable, enum novo, campos novos, estado global, I/O ou fase nova. Aceitação: RED→GREEN de reuso, ordem, erro, substituição igual, opcionais/slots e metadados; suite core, builds e lint, qualificação de texto/morfologia/render/query contra release anterior e vanilla pinado; alocações e tempos pareados com proveniência. Não alegar ganho universal a partir de casos isolados.


A prova compartilhada de ausência de mudança obedece às claims desse núcleo. O algoritmo, os campos transformados, sua ordem e a preservação específica permanecem owned por este prompt e seu único consumer.
