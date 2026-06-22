# Prompt L0 — `stdlib/panic` — aborto de avaliação
Hash do Código: df1f2f88

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib/panic.rs`
**Origem**: Passo 392 (`typst-passo-392.md`) — dívida genuína acidental (balde D), XS, zero deps.
**ADRs**: ADR-0033 (paridade vanilla), ADR-0107 (paridade linguagem), ADR-0017 (não aplica — sem variant novo).

---

## 1. Contexto

O vanilla expõe `panic(msg)` — aborta a avaliação com uma mensagem de erro. É helper puro: entrada `Str`, efeito aborto de eval, sem I/O, sem layout, sem tipo novo.

## 2. Arquitetura

- **Sem tipo novo**: reutiliza `SourceDiagnostic` e o mecanismo de erro existente.
- **Sem layout/render**: aborta em eval-time.
- **Convenção de assinatura e helpers**: ver `stdlib/_comum.md`.

## 3. Função nativa

Assinatura `fn native_panic(ctx: &mut EvalContext<'_>, args: &Args) -> SourceResult<Value>` (ver `_comum.md`).

- `msg`: único argumento posicional obrigatório, `Str`.
- Rejeita argumentos nomeados.
- Tipo errado → erro (não panic).
- Devolve `Err(vec![SourceDiagnostic::error(..., msg)])`, abortando a avaliação.

## 4. Paridade vanilla

A paridade é semântica (ADR-0107): `panic(msg)` aborta a avaliação e reporta `msg` ao utilizador. Não é necessário reproduzir o tipo de erro exacto do vanilla.

## 5. Testes

- `panic("fail")` → `Err`; mensagem "fail" presente no diagnóstico.
- `panic("")` → `Err`; mensagem vazia.
- `panic(123)` → `Err` (tipo inválido).
- Argumento nomeado inválido → `Err`.

## 6. Scope-out

- Não criar tipo `Value` ou `Content` novo.
- Não tocar em layout/render.
- Não centralizar mensagem em catálogo i18n (ainda não existe).
