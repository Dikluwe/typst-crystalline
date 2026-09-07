# Prompt L0 — `stdlib/panic` — aborto de avaliação
Hash do Código: 277f949d

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

## P1308 — origem do aborto de panic

Medição anterior à decisão: a matriz pública R6 final, no baseline P1308
SHA-256 `62c53690cbe3dd36b5b79168ea59a32f6eb88cc52ea52a8ca97365c5a9459394`,
separa âncora args-list de call inteiro em callbacks map/filter. Fonte
ratificada `foundations/mod.rs:140–156` devolve erro cuja origem de chamada
é fornecida pelo avaliador, não pelos valores exibidos na mensagem.

O aborto final usa args.span da chamada inteira transportada pelo owner
call_dispatch, inclusive alias/With. A nativa continua validando named e
compondo mensagem na mesma ordem; não procura AST/World, não extrai origem
do primeiro valor e não inventa trace. Aplicação sintética usa o agregado
recebido. Esta precisão de contrato não exige alterar a implementação da
mensagem; controles de validação preexistentes permanecem explícitos.

O recibo P1308 `ce758b5c2a18288bf9c8433178f577b50c52df80cedcddcb1f4daa4e785573fc`
mede que named inválido ainda diverge em mensagem e origem no baseline.
Essa dívida permanece fora desta correção: não trocar sua âncora pela
chamada inteira nem declarar esses controles paritários.
Preservação refere-se à validação, mensagem e span primário. O overlay de
Source pode tornar resolvível e acrescentar o trace natural de `trace_call`
a esse erro detached; não se exige stderr baseline sem trace. Esse delta
causal não autoriza reancorar ou traduzir o diagnóstico primário.
