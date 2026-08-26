# Prompt L0 — `compiler/lexer/code` — modo Code
Hash do Código: d31bc0eb

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lexer/mode-boundaries.toml sha256:aae80d538980eeec87b884712269e3b777fe44c0dae6b3503b5fa9ee4f9ad76f


**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/lexer/code.rs`
**Criado em**: 2026-08-26 (P1199; individualização de `lexer/mod.md`)
**ADRs**: ADR-0003, ADR-0037, ADR-0129

---

## Medição antes da decisão

`code.rs` implementa `Lexer::code`, strings, identificadores, números e
diagnósticos de operadores inválidos. Não possui o cursor, não despacha modos e
não implementa trivia universal. O antigo owner conjunto legitimava quatro
módulos; P1199 individualiza o modo sem mudar corpos Rust.

## Responsabilidade

Tokenizar a gramática lexical de expressões Typst em `SyntaxMode::Code`.

### Pontuação e operadores

Reconhece braces, brackets, parênteses, dollar, comma, semicolon, colon, dot e
os operadores simples. Operadores compostos têm precedência sobre seus
prefixos: `==`, `!=`, `<=`, `>=`, `+=`, `-=`, `*=`, `/=`, `..` e `=>`.
O sinal menos aceita `-` e U+2212.

### Identificadores

Um identificador começa por `is_id_start` e continua por `is_id_continue`.
Keywords são reconhecidas salvo após `.` ou `@` simples; `_` isolado produz
`Underscore`, e os demais produzem `Ident`.

### Números

`number` reconhece bases binária, octal, decimal e hexadecimal; floats
decimais, expoentes e sufixos `pt`, `mm`, `cm`, `in`, `deg`, `rad`, `em`,
`fr` e `%`. Distingue `Int`, `Float` e `Numeric`, preservando erros específicos
de hexadecimal, float incompleto, base inválida e sufixo inválido. `..` e
method access não são consumidos como separador decimal.

### Strings e erros

Strings terminam em aspas duplas não escapadas; fim sem fechamento gera erro.
Caracteres inválidos produzem hints próprios para hash redundante, `&&`, `||`,
`!` e `~=` sem introduzir aliases incompatíveis com a linguagem.

## Critérios de verificação

- Operadores compostos vencem seus prefixos simples.
- Bases e sufixos válidos produzem o kind correspondente; entradas inválidas
  preservam erro e hints.
- Aspas em Code produzem `Str`, nunca `SmartQuote`.
- O smoke test do módulo e a suíte completa do lexer passam.
- P1199 não modifica corpos Rust.
