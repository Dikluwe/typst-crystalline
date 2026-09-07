# Prompt L0 — `stdlib/numbering` — função global `numbering()` e `format_pattern`
Hash do Código: 13a8aa93

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

## P1307-R3 — Args sintético nos callbacks

### Medição anterior à decisão

`01_core/src/compiler/stdlib/numbering.rs:36-40` converte a lista usize do
realizador em Int; `:103-107` reconstrói Int dos números já convertidos a u32
pela nativa. Nenhum ponto conserva uma ocorrência lexical original. Fonte
congelada no baseline R3 SHA-256
`b50e726c5830c0f91a6875d2d0a758903bc93b0bd719f4b5357828522a999293`,
HEAD `b303f1f15b610e09872b567027e0d806387fde8c` mais o diff P1306 registrado.

### Contrato de migração

Os dois builders migram para `Args::from_parts(items, named, span)` do owner
Args, com named vazio e `occurrences: None` explícito. São valores calculados,
não encaminhamento dos Args de entrada: não atribuir spans lexicais falsos
nem copiar ocorrências de números antes da conversão. Manter span agregado,
valores, ordem e número de callbacks, assinaturas públicas, casts, erros,
padrões e conversão a conteúdo. Não corrigir incidentalmente truncamento
preexistente por u32 ou outra semântica de numbering.

Esta é adaptação à API Args proposta, não alegação de paridade diagnóstica
geral com vanilla para callbacks sintéticos. Testes devem verificar valores
e detached individuais sem quebrar as formas Pattern/Func. Necessidade de
conservar origens que os builders atuais já não têm refuta a adaptação
mecânica e exige nova medição/escopo. Gate ADR-0127 do carrier ainda pendente.
