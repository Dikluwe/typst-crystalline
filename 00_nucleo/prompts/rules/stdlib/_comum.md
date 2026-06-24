# Prompt L0 — `rules/stdlib` — comum (convenção e helpers partilhados)
Hash do Código: b7a9f5a5

**Camada**: L1
**Origem**: fatiado de `rules/stdlib.md` em **P314** (ADR-0104, atomicidade
para agentes). Este ficheiro guarda **só** o que é partilhado por várias
funções; os prompts finos por função citam-no. Índice da partição:
`rules/stdlib.md` (agora índice).
**Apontam para aqui** (linhagem `@prompt`): `stdlib/mod.rs` (registo),
`stdlib/transforms.rs`, `stdlib/gradients.rs` — os ficheiros cuja spec
detalhada **não** estava em `stdlib.md` (dependiam apenas da convenção
partilhada; spec dedicada é candidata futura, não inventada aqui).

> Nota: `stdlib/structural.rs` tem agora o seu próprio prompt em
> `00_nucleo/prompts/rules/stdlib/structural.md` (P430).
> Nota: `stdlib/layout.rs` tem agora o seu próprio prompt em
> `00_nucleo/prompts/rules/stdlib/layout.md` (P432).
> Nota: `stdlib/calc.rs` tem o seu prompt dedicado em
> `00_nucleo/prompts/rules/stdlib/calc.md` (actualizado no P433 para o subset
> trig/hiperbólicas/log/exp/constantes).
> Nota: `stdlib/assert.rs` tem agora o seu próprio prompt em
> `00_nucleo/prompts/rules/stdlib/assert.md` (P434).
> Nota: `stdlib/shapes.rs` tem agora o seu próprio prompt em
> `00_nucleo/prompts/rules/stdlib/shapes.md` (P435).

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
