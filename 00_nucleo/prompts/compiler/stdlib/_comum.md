# Prompt L0 — `rules/stdlib` — comum (convenção e helpers partilhados)
Hash do Código: 00014004

**Camada**: L1
**Origem**: fatiado de `rules/stdlib.md` em **P314** (ADR-0104, atomicidade
para agentes). Este ficheiro guarda **só** o que é partilhado por várias
funções; os prompts finos por função citam-no. Índice da partição:
`rules/stdlib.md` (agora índice).
**Apontam para aqui** (linhagem `@prompt`): `stdlib/mod.rs` (registo) —
os ficheiros cuja spec detalhada **não** estava em `stdlib.md` (dependiam
apenas da convenção partilhada; spec dedicada é candidata futura, não
inventada aqui).

> Nota: `stdlib/structural.rs` tem agora o seu próprio prompt em
> `00_nucleo/prompts/compiler/stdlib/structural.md` (P430).
> Nota: `stdlib/layout.rs` tem agora o seu próprio prompt em
> `00_nucleo/prompts/compiler/stdlib/layout.md` (P432).
> Nota: `stdlib/calc.rs` tem o seu prompt dedicado em
> `00_nucleo/prompts/compiler/stdlib/calc.md` (actualizado no P433 para o subset
> trig/hiperbólicas/log/exp/constantes).
> Nota: `stdlib/assert.rs` tem agora o seu próprio prompt em
> `00_nucleo/prompts/compiler/stdlib/assert.md` (P434).
> Nota: `stdlib/shapes.rs` tem agora o seu próprio prompt em
> `00_nucleo/prompts/compiler/stdlib/shapes.md` (P435).
> Nota: `stdlib/transforms.rs` tem agora o seu próprio prompt em
> `00_nucleo/prompts/compiler/stdlib/transforms.md` (P436).
> Nota: `stdlib/gradients.rs` tem agora o seu próprio prompt em
> `00_nucleo/prompts/compiler/stdlib/gradients.md` (P437).
> Nota: `stdlib/foundations.rs` tem agora o seu próprio prompt expandido em
> `00_nucleo/prompts/compiler/stdlib/foundations.md` (P438). Com este fecho,
> **DEBT-57 está encerrado** — todos os ficheiros stdlib de L1 têm spec L0
> dedicada.

---

## Contexto e Objetivo

Enquanto `eval.rs` é o motor que **caminha pela AST**, este módulo contém as
**ferramentas nativas** que Typst expõe no seu escopo global — funções
implementadas directamente em Rust e registadas como `Value::Func` durante a
inicialização do compilador.

**Separação de responsabilidades crítica:**
- `eval.rs`: sabe como avaliar `Expr::LetBinding`, loops, condicionais → produz `Value`
- `stdlib`: sabe o que `abs(-5)` retorna → implementa as funções que `eval` *chama*

## Convenção de assinatura (Passo 71 — DEBT-24)

```rust
fn native_X(ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value>
```
Funções sem I/O usam `_ctx` (prefixo underscore suprime o warning).
`native_image` usa `ctx.world.read_bytes(path)` para aceder ao ficheiro.
Aceita positional (`args.items`) e named args (`args.named`).
Funções que não aceitam named args chamam `expect_no_named(&args.named)?` no início.

## Helpers Internos

| Função | Uso |
|--------|-----|
| `coerce_to_f64(v, ctx)` | `Int`→`f64`, `Float`→`f64`, outros → Err com contexto |
| `guard_float(f)` | NaN → Err "não é um número", Inf → Err "infinito" |
| `format_float(f)` | compacto sem trailing zeros; garante ponto decimal (`"3.0"`) |
| `format_length(l)` | `Length` → `"12pt"`, `"1.5em"`, `"6pt + 1em"` |

## Sistema de Tipos — Regras de Promoção

```
Int + Int   → Int       (sem promoção)
Int + Float → Float     (coerce_to_f64)
Float pow Float → guarda NaN/Inf via guard_float
Int/Int divisão → Float (semântica eval.rs, não stdlib)
```

## P1215 — reexport de metadados de chamada

O hub reexporta `CollectionCallSpans` somente em `pub(crate)` para ligar
`eval/call_dispatch` ao owner `stdlib/collections`. O tipo não entra na API da
linguagem nem em contratos externos e não contém valores avaliados.

## P1289 — reexport interno do owner `float`

O registo raiz reexporta em `pub(crate)` somente
`float_type_field`, `is_float_instance_method` e `dispatch_float_method` do
owner `foundations/float`. O hub não contém lookup, fórmula ou validação e não
amplia a API Rust externa.

## P1292 — reexport interno mínimo de `native_flush`

### Medição anterior à decisão

No baseline SHA-256
`02d53b1588b008c295e2dcab0adc0e6ad2e8604d9972c5fa0b2503d340826f3f`,
`01_core/src/compiler/stdlib/mod.rs` declara `layout` como submódulo privado e
constitui a fachada já consumida por `compiler/eval/mod.rs`. A lista de
reexports de layout expõe as demais nativas necessárias ao registo, mas ainda
omite `native_flush`. A implementação/construção de `native_flush` já pertence
ao owner dedicado `compiler/stdlib/layout.md`, SHA-256
`6ff688ec12444582ec9ec87bb7b432019ff568882fbb1c66170eb47367e62dc4`;
movê-la para o hub violaria a atomização e o ownership 1:1.

### Decisão

O registo raiz reexporta somente dentro da crate:

```rust
pub(crate) use crate::compiler::stdlib::layout::native_flush;
```

O reexport é uma ligação de fachada, não um segundo owner. `native_flush`
continua definida e legitimada exclusivamente por `stdlib/layout.md`; este
prompt legitima apenas a linha de reexport em `stdlib/mod.rs`. O hub não cria
wrapper, lookup, cast, validação ou branch, não torna o módulo `layout`
público, não usa wildcard e não promove a função à API Rust externa.

A superfície Typst `place.flush`, seus argumentos/erros e a construção de
`Content::Flush` permanecem nos owners P1292 já selados. Esta correção só
permite que o consumer da fachada alcance a função dedicada; não altera
vetores A-D, default, fase de pipeline ou compatibilidade. Qualquer necessidade
de lógica no hub, `pub use` externo, mudança em `layout.rs` ou segundo caminho
de implementação refuta este amendment e exige novo owner/escopo.
