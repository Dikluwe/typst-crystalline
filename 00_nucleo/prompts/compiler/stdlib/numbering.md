# Prompt L0 — `stdlib/numbering` — função global `numbering()` e `format_pattern`
Hash do Código: (pendente — calculado por `crystalline-lint --fix-hashes .` no fecho do passo)

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/numbering.rs`
**Origem**: P847 — extraído de `stdlib/structural.rs` (um ficheiro, um prompt); a especificação perene destila o P793 §1 e o achado P844 #53.
**ADRs**: ADR-0033 (paridade observable), ADR-0107 (paridade linguagem vs mecânica), ADR-0108 (medir antes de decidir).
**Convenções partilhadas**: ver `00_nucleo/prompts/compiler/stdlib/_comum.md`.

---

## 1. Função standalone `numbering`

A função nativa `numbering` está disponível no escopo global do compilador.

**Assinatura**: `numbering(pattern, ..numbers)`

- `numbers`: inteiros não-negativos (`Value::Int >= 0`); qualquer outro tipo
  ou valor negativo é erro.
- Chamada sem argumentos é erro ("exige pelo menos 1 argumento (o padrão)").

### 1.1. Resolução de padrão (shorthand string)

Se `pattern` for uma string, é analisada como um padrão Typst:

- Counting symbols suportados: `'1'` (decimal), `'a'` (lower-alpha),
  `'A'` (upper-alpha), `'i'` (lower-roman), `'I'` (upper-roman),
  `'א'` (Alef hebraico), `'①'` (circled numbers).
- Divisão em prefixo, peças de numeral systems e sufixo.
- Formatação de cada número usando o numeral system da peça respectiva.
- Se o Alef hebraico `'א'` receber o valor `0`, emite-se um warning contendo
  `"the numeral system \`hebrew\` cannot represent zero"` e faz-se fallback
  para numerais decimais árabes (`0`).
- Circled numbers `'①'`: 0 → `⓪`; 1..=50 → `①..㊿`; >50 → warning medido no
  vanilla (``the number {n} is too large to be represented with the
  `arabic.o` numeral system``) + fallback decimal.
- Se houver mais números do que peças no padrão, a última peça é repetida
  (com o seu prefixo, ou com o sufixo se o prefixo da última peça for vazio).
- Um padrão sem nenhum counting symbol é erro ("padrão de numeração
  inválido").

### 1.2. Resolução de closure (function)

Se `pattern` for uma função (`Value::Func`), a execução invoca a closure
repassando os números como argumentos posicionais e devolve o valor gerado.

## 2. `format_pattern` partilhado

`format_pattern(engine, span, pat, numbers)` é `pub(crate)` e partilhado por
`counter.display(pattern)` (`stdlib/counter.rs`) — os dois caminhos usam o
mesmo algoritmo (tokens, descarte de tokens extra, repetição do último
token). O token `①` é válido tanto em `numbering()` como em
`counter.display()`.

Helpers privados do módulo: `format_numeral`, `nth_alpha_char`,
`to_roman_numeral`, `to_hebrew_numeral`, `to_circled_number`.

## 3. Testes canónicos

```
numbering("1.a", 3, 1) -> "3.a"
numbering("(I)", 5) -> "(V)"
numbering("1.a.I", 1, 2, 3, 4) -> repetição da última peça
numbering("א", 15) -> "טו"
numbering("א", 0) -> "0" + warning "the numeral system `hebrew` cannot represent zero"
numbering("①", 2) -> "②"
numbering(n => str(n) + "!", 5) -> closure invocada com os números
numbering() -> Err "exige pelo menos 1 argumento (o padrão)"
```

## P1157 — owner único de aplicação de `Numbering`

### Medição antes da decisão

Sondas no vanilla ratificado confirmaram que callbacks podem devolver qualquer
valor (`42` renderiza `42`; `none` renderiza vazio), que página visível passa
dois números e referência passa um. A fonte `model/numbering.rs:99-128`
concentra Pattern/Func e sua aplicação.

### Decisão

Este módulo continua owner único da aplicação: recebe
`entities::numbering::Numbering`, números posicionais, scopes, EvalContext,
Engine e span; Pattern delega a `format_pattern`, Func delega a `apply_func`.
Expõe internamente conversão do `Value` devolvido para `Content` com a mesma
morfologia de markup usada pela função global. Layout e referências não
duplicam dispatch nem chamam closures diretamente.

## P1159 — realizador partilhado

`realize_numbering` aplica o objeto cru a uma lista de inteiros: Pattern usa
`format_pattern`; Func usa `apply_func`. O resultado passa por
`value_to_content`, preservando a morfologia normal de valores Typst. O helper
é público apenas para a orquestração L3; layouter e PageStore nunca recebem
Engine nem executam callbacks.
