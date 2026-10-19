# Reconciliação Retroativa — `ffabfd837` · `delim:` em `mat()`/`vec()`

**Tipo:** Relatório de reconciliação retroativa (código precedeu L0 — violação documentada)  
**Commit:** `ffabfd8370942ef4ab6c808382ceb363ffdeb3b4`  
**Data do commit:** 2026-07-25  
**Detectado em:** P916 (Parte C) · 2026-07-26  

---

## O que aconteceu

Durante a sessão que produziu P912–P914, o utilizador reportou que matrizes com
delimitadores personalizados (`mat(delim: "[", ...)`) não produziam o delimitador
correcto. A correcção foi implementada directamente no código sem seguir o protocolo
L0 → L1.

**Ficheiros alterados:**
- `01_core/src/engine/eval/math.rs` — extracção do named arg `delim:` em `mat()` e `vec()`
- `01_core/src/engine/math/layout/matrix.rs` — tratamento de `'\0'` (sem delimitador)
- `01_core/src/engine/eval/tests.rs` — 10 linhas de testes

## Spec (retroativa)

Adicionada a `00_nucleo/prompts/engine/eval.md §P914` no commit `a74760aa4`:

- `mat(delim: "(", ...)` → par `('(',')')` (padrão se omitido)
- `mat(delim: "[", ...)` → par `('[',']')`
- `mat(delim: "{", ...)` → par `('{','}')`
- `mat(delim: "|", ...)` → par `('|','|')`
- `mat(delim: "||", ...)` → par `('‖','‖')`
- `mat(delim: none, ...)` ou `mat(delim: "", ...)` → par `('\0','\0')` (sem delimitador)
- No layout (`matrix.rs`): `'\0'` suprime `layout_stretchy_delimiter` e padding lateral

## Violação de processo

**A trava arquitectural foi violada:** L1 foi escrito antes do L0. O L0 existe agora
com hash synchronizado (`crystalline-lint . → 0 V5`, confirmado em P916).

Este documento regista a violação com honestidade. O código está correcto e testado;
a ordem foi invertida.

## Validação

- `cargo test -p typst-core -- mat` → testes de mat/vec com delim passam
- `crystalline-lint . → 0 violations`
