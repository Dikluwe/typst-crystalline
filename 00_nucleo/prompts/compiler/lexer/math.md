# Prompt L0 — `compiler/lexer/math` — modo Math
Hash do Código: 186e943e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lexer/mode-boundaries.toml sha256:aae80d538980eeec87b884712269e3b777fe44c0dae6b3503b5fa9ee4f9ad76f


**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/lexer/math.rs`
**Criado em**: 2026-08-26 (P1199; individualização de `lexer/mod.md`)
**ADRs**: ADR-0003, ADR-0037, ADR-0129

---

## Medição antes da decisão

`math.rs` possui shorthands, átomos, delimitadores, graphemes e inferência de
argumentos matemáticos. Não possui dispatch universal nem troca de modo.
P1199 individualiza essas regras sem alterar morfologia.

## Responsabilidade

Tokenizar expressões em `SyntaxMode::Math`.

## Tokens matemáticos

Sequências de setas, relações, pontos e operadores reconhecidas pela tabela do
match produzem `MathShorthand`, com as formas mais longas testadas antes dos
prefixos. Pontuação, hash, underscore, dollar, slash, hat, alignment point,
raízes e bang possuem kinds próprios. Aspas simples consecutivas formam um
único `MathPrimes`; aspas duplas reutilizam string Code.

Parênteses têm kinds próprios. Delimitadores cuja classe matemática padrão é
Opening ou Closing são convertidos em `LeftBrace` ou `RightBrace`, incluindo
as formas `[|` e `|]`.

## Identificadores e texto

Identificadores matemáticos multigrapheme produzem `MathIdent`; acessos por dot
constroem `FieldAccess`. Um único grapheme permanece `MathText`. Números e
clusters grapheme são consumidos integralmente; decimal numérico só incorpora
o ponto quando há dígitos posteriores.

## Argumentos inferidos

`maybe_math_named_arg` reconhece identificador seguido diretamente de `:`,
sem confundir `:=`/`::=`; `_` gera nó de erro. `maybe_math_spread_arg`
reconhece `..` somente quando há argumento seguinte e não colide com `...`,
separador ou fim de math. Falha restaura o cursor original.

## Critérios de verificação

- Shorthands longos vencem prefixos.
- Identificador multigrapheme e field access preservam o nó estruturado.
- Primes consecutivos permanecem um token e smart quotes não vazam para Math.
- Inferência malsucedida de named/spread restaura o cursor.
- O smoke test e a suíte completa do lexer passam.
- P1199 não modifica corpos Rust.
