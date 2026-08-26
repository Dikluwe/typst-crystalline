# Prompt L0 — `entities/elements/math_limits_override` — `MathLimitsOverrideElem`
Hash do Código: 700f15ad

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
> `00_nucleo/corpus-docs/math/attach.typ:18-28`); guardas end-to-end em
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
}
```

`Content::MathLimitsOverride(Arc<MathLimitsOverrideElem>)`.
Construtor ergonómico: `Content::math_limits_override(body: Content, limits: bool, inline: bool)`.

## `impl Element for MathLimitsOverrideElem`

| método | comportamento |
|---|---|
| `plain_text` | `self.body.plain_text()` |
| `is_empty` | `self.body.is_empty()` (delega, mesmo padrão de `MathClassOverrideElem`) |
| `map_content` | **recursivo** no `body`: `Content::MathLimitsOverride(Arc::new(MathLimitsOverrideElem { body: self.body.map_content(f)?, limits: self.limits, inline: self.inline }))` |
| `map_text` | **terminal** (família math não desce em `map_text`) |
| `get_field` | default `None` |
| `element_kind`/`to_payload` | default `None` (não-locatável) |

## `eq` estrutural

`#[derive(PartialEq)]` compara `body`, `limits` e `inline`.

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
