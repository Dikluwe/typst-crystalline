# Prompt L0 — `stdlib/panic` — aborto de avaliação
Hash do Código: fc5d7407

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/panic.rs`
**Origem**: Passo 392 (`typst-passo-392.md`) — dívida genuína acidental (balde D), XS, zero deps. P843 (F6): assinatura e mensagem em paridade com o vanilla (medida).
**ADRs**: ADR-0033 (paridade vanilla), ADR-0107 (paridade linguagem), ADR-0017 (não aplica — sem variant novo).

---

## 1. Contexto

O vanilla expõe `panic(..)` — aborta a avaliação com uma mensagem de erro. É helper puro: efeito aborto de eval, sem I/O, sem layout, sem tipo novo.

## 2. Arquitetura

- **Sem tipo novo**: reutiliza `SourceDiagnostic` e o mecanismo de erro existente.
- **Sem layout/render**: aborta em eval-time.
- **Convenção de assinatura e helpers**: ver `stdlib/_comum.md`.

## 3. Função nativa

Assinatura `fn native_panic(ctx, args, world, current_file) -> SourceResult<Value>` (ver `_comum.md`).

- **Variádico** (P843 F6 — paridade vanilla `foundations/mod.rs:140-152`):
  zero ou mais argumentos posicionais de qualquer tipo.
- Rejeita argumentos nomeados.
- Devolve sempre `Err(vec![SourceDiagnostic::error(span, msg)])`, abortando a avaliação.

## 4. Paridade vanilla (P843 F6 — mensagem medida em `temp/p843/f6_*.typ`)

A mensagem é o observável (ADR-0107), verbatim do vanilla:

- Com argumentos: `panicked with: {v0}, {v1}, ...` — separador `", "`;
  valores `Str` entram crus (sem aspas), todos os outros via `repr` da
  linguagem.
- Sem argumentos: `panicked` (sem ` with:`).

Exemplos medidos:

```
panic("this is wrong")   -> Err "panicked with: this is wrong"
panic(42)                -> Err "panicked with: 42"
panic("a", 1, (x: 2))    -> Err "panicked with: a, 1, (x: 2)"
panic()                  -> Err "panicked"
```

## 5. Testes

- `panic("fail")` → `Err "panicked with: fail"`.
- `panic("")` → `Err "panicked with: "`.
- `panic(42)` → `Err "panicked with: 42"` (não-string via repr).
- `panic()` → `Err "panicked"`.
- Argumento nomeado inválido → `Err`.

## 6. Scope-out

- Não criar tipo `Value` ou `Content` novo.
- Não tocar em layout/render.
- Não centralizar mensagem em catálogo i18n (ainda não existe).
- A keyword alternativa `error(...)` do vanilla (`#[func(keywords = ["error"])]`) não está implementada.
