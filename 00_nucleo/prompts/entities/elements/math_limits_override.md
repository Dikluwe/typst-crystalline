# Prompt L0 — `entities/elements/math_limits_override` — `MathLimitsOverrideElem`
Hash do Código: 2507c5f3

**Camada**: L1 · **Alvo**: `01_core/src/entities/elements/math_limits_override.rs`
**Origem**: Passo 992 (achado externo 2026-08-07, secção 32 do documento
estendido). Trait e regras partilhadas: ver `entities/elements/_comum.md`.
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

**Cristalino consolida os dois elementos vanilla num só** (ADR-0107 —
paridade é com a língua, não com a mecânica): `MathLimitsOverrideElem {
body, limits: bool, inline: bool }`, onde `limits` é o discriminador
scripts()=false / limits()=true, e `inline` só é lido quando
`limits == true` (paralelo à consolidação já feita para `binom()` via
`math_matrix`, P899 Parte D). `scripts(body)` constrói com
`limits: false` (o campo `inline` fica `true` por convenção mas é
ignorado — ver fórmula em `math/layout/attach.md` §P992).

Não afecta `MathClass`/espaçonto do `body` (vanilla `resolve_scripts`/
`resolve_limits`: resolvem o `body` normalmente, só sobrescrevem
`item.set_limits(...)` no item já resolvido) — **diferente** de
`MathClassOverride`, que força a classe. `body` é layoutado normalmente
via `MathLayouter::layout_node` e recursa em `apply_math_default` (ao
contrário de `MathClassOverride`, que não recursa — ver
`math/layout/_comum.md` §P992: o caso de uso canónico de `limits()`/
`scripts()` inclui bases de 1 letra como `A`, que precisam do itálico por
defeito).

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
`engine/introspect.rs` (2×, não-locatável/terminal),
`engine/introspect/locatable.rs` (não-locatável), `engine/layout/mod.rs`
(fallback de math fora de contexto — `layout_math_fallback`),
`engine/eval/repr.rs` (`limits(<repr(body)>, inline: <bool>)` quando
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

## Débito registado (retificação 2026-08-11, auditoria pós-P992)

**Paridade de erros em named args — NÃO implementada** (escopo-out deste
passo, a tratar em passo próprio):
1. `scripts(A, foo: 1)` / `limits(A, foo: 1)` — o vanilla rejeita
   (`error: unexpected argument: foo`); o cristalino **ignora
   silenciosamente** named args desconhecidos no loop de argumentos
   (`eval/math.rs`, braço de `limits`/`scripts`).
2. `limits(A, inline: 5)` — o vanilla rejeita (`error: expected boolean,
   found content`); o cristalino descarta o erro (`if let Ok(Value::Bool)`)
   e `inline` fica preso no default `true`.
Mensagem de erro é observável ao nível da língua (ADR-0107) — a paridade
está incompleta neste ponto. Não é regressão (as funções não existiam
antes de P992), mas fica registado como débito explícito, não implícito.
