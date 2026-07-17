# P445 — Relatório: Smart quotes tipográficas

> **Data:** 2026-06-24  
> **Executor:** assistente IA (Kimi Code CLI)  
> **Branch:** `Tekt`  
> **Foco:** Materializar smart quotes para `"` e `'` em modo Markup, convertendo-as em aspas tipográficas (curly) conforme contexto de abertura/fecho/apóstrofo.

---

## Resumo executivo

O **lexer** do Cristalino já emitia `SyntaxKind::SmartQuote` para `"` e `'` em modo `Markup` (herança dos passos P155/P287), mas o `eval_markup` convertia esses tokens de forma ingénua: `"` alternava entre abertura/fecho e `'` era sempre mapeado para `U+2019`, ignorando o contexto adjacente e o `text.lang`.

O **trabalho do P445** foi:

1. Tornar a resolução **context-aware** no `eval_markup` (whitespace/início → abertura; letra+letra → apóstrofo; resto → fecho).
2. Adicionar **localização de aspas simples** em `rules/lang/quotes.rs` (inglês `U+2018`/`U+2019`; default curly).
3. Garantir que **Code** e **Math** permanecem inalterados (`"` string, `'` prime).
4. Adicionar tests de lexer e de integração, e actualizar a spec L0.

**Resultado:** P445 fechado; `cargo test --workspace` verde; `crystalline-lint .` sem novas violações (apenas 2 warnings órfãos pre-existentes).

---

## 1. Mudanças de código do P445

### 1.1 `01_core/src/engine/eval/mod.rs`

- No braço `SyntaxKind::SmartQuote` de `eval_markup`:
  - Acumula `byte_offset` sobre `node.clone().into_text()` para saber a posição do quote no texto-fonte.
  - Lê o caractere **anterior** (`prev`) e **seguinte** (`next`) ao quote.
  - Aspas **duplas**:
    - Se `prev` for início de texto, whitespace ou `(`/`[`/`{`/`<` → abertura.
    - Senão → fecho.
    - O par usado vem de `localize_quotes(lang)` com fallback `DEFAULT_QUOTES` (ASCII quando não há `lang`).
  - Aspas **simples**:
    - Se ambos os lados forem alfanuméricos (`don't`, `Alice's`) → apóstrofo/fecho (`U+2019`).
    - Senão, se contexto de abertura → `U+2018`.
    - Senão → `U+2019`.
    - O par vem de `localize_single_quotes(lang)` com fallback `DEFAULT_SINGLE_QUOTES` (curly inglês).

### 1.2 `01_core/src/engine/lang/quotes.rs`

- Adicionado `DEFAULT_SINGLE_QUOTES` (`U+2018`/`U+2019`).
- Adicionada tabela `LANG_SINGLE_QUOTES` (inglês → curly; outras línguas caem no default).
- Adicionada função pública `localize_single_quotes(lang: &Lang)`.
- Adicionados 2 tests unitários para aspas simples.

### 1.3 `01_core/src/engine/lexer/mod.rs`

- Adicionados 3 tests de lexer:
  - `lex_markup_smart_quote_double`: `"hello"` em Markup → SmartQuote, Text, SmartQuote.
  - `lex_markup_smart_quote_single`: `'hello'` em Markup → SmartQuote, Text, SmartQuote.
  - `lex_code_quote_continua_string_literal`: `"hello"` em Code → `Str`, sem `SmartQuote`.

### 1.4 `01_core/src/engine/eval/tests.rs`

- Adicionados 3 tests de integração:
  - `eval_markup_smart_quotes_duplas_curly_com_lang_en` → verifica `U+201C`/`U+201D` e ausência de `"`.
  - `eval_markup_smart_quotes_simples_curly_com_lang_en` → verifica `U+2018`/`U+2019` e ausência de `'`.
  - `eval_markup_apostrophe_possessivo_emite_u2019` → verifica que `'Alice's` usa `U+2019` e não `U+2018`.

### 1.5 `00_nucleo/prompts/engine/lexer/mod.md`

- Actualizado `Hash do Código` para o hash actual do lexer.
- Adicionada secção **Smart Quotes (Passo 445)** descrevendo:
  - Emissão de `SmartQuote` em Markup.
  - Conversão context-aware no `eval_markup`.
  - Comportamento em Code (`Str`) e Math (`MathPrimes`).
  - Critérios de verificação com 4 exemplos.

### 1.6 Hashes `@prompt-hash` nos ficheiros do lexer

- `01_core/src/engine/lexer/mod.rs`
- `01_core/src/engine/lexer/markup.rs`
- `01_core/src/engine/lexer/code.rs`
- `01_core/src/engine/lexer/math.rs`

Todos actualizados para o hash actual do prompt L0, eliminando 4 warnings de deriva (`drift`) do `crystalline-lint`.

---

## 2. Estado pré-existente (P155/P287)

| Componente | Ficheiro | Estado |
|------------|----------|--------|
| Token `SmartQuote` | `01_core/src/engine/lexer/markup.rs` | `"` e `'` já emitidos como `SmartQuote` em Markup |
| Localização de aspas duplas | `01_core/src/engine/lang/quotes.rs` | Tabela `LANG_QUOTES` + `localize_quotes` |
| Eval markup | `01_core/src/engine/eval/mod.rs` | Conversão ingénua `"` alternada e `'` → `U+2019` |

Não houve necessidade de alterar o parser nem o layout; a mudança ficou no eval, aproveitando a infraestrutura já existente.

---

## 3. Verificação

### 3.1 `cargo test --workspace`

```bash
RUST_MIN_STACK=8388608 cargo test --workspace
```

Resultado: **todos os testes passam**, incluindo:
- 3 novos tests de lexer (`lex_markup_smart_quote_*`, `lex_code_quote_continua_string_literal`).
- 3 novos tests de integração em `rules/eval/tests.rs`.
- 2 novos tests unitários em `rules/lang/quotes.rs`.
- Pipeline completo (`typst-core`, `typst_shell`, `typst_infra`, `typst_wiring`, CLI, `crystalline_lint`).

Resumo dos conjuntos:
- `3172 passed` (`typst-core` lib)
- `489 passed; 6 ignored` (`typst-core` tests integração)
- `24 passed` (`typst_shell`)
- `2 passed` (`typst_infra`)
- `21 passed` (`typst_wiring`)
- `2 passed` (`crystalline_lint`)

### 3.2 `crystalline-lint .`

Resultado: **zero novas violações**. Apenas os 2 warnings órfãos de prompts pre-existentes (`adr-stub-vs-fallback.md` e `show-regex.md`) permanecem.

---

## 4. Scope-out preservado

- Smart quotes em **code mode** continuam scope-out (`"` delimita strings).
- Smart quotes em **math mode** continuam scope-out (`'` é `MathPrimes`).
- Localização específica para outras línguas nas aspas simples continua limitada à tabela actual (inglês + default curly).
- Nested quotes são tratadas independentemente, como no vanilla.

---

## 5. Commits

- Branch: `Tekt`
- Commit: `P445: smart quotes context-aware em markup`

Alterações incluídas no commit:
- `01_core/src/engine/eval/mod.rs`
- `01_core/src/engine/lang/quotes.rs`
- `01_core/src/engine/eval/tests.rs`
- `01_core/src/engine/lexer/mod.rs`
- `01_core/src/engine/lexer/markup.rs`
- `01_core/src/engine/lexer/code.rs`
- `01_core/src/engine/lexer/math.rs`
- `00_nucleo/prompts/engine/lexer/mod.md`
- `00_nucleo/materialization/typst-passo-445.md`
- `00_nucleo/materialization/typst-passo-445-relatorio.md`
