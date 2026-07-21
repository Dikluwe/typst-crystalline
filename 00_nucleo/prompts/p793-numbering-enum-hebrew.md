# Prompt L0: P793 — Numeração, Enum e Warning Hebrew-Zero
Hash do Código: 16da3f4c

Este prompt especifica a arquitetura da função global `numbering()`, o auto-incremento de `enum` em layout single-pass e a emissão do warning de fallback para numeração hebraica com valor zero.

## 1. Função Standalone `numbering`

A função nativa `numbering` deve estar disponível no escopo global do compilador.
Assinatura: `numbering(pattern, ..numbers)`

### 1.1. Resolução de Padrão (Shorthand String)
Se o argumento `pattern` for uma string, ela deve ser analisada como um padrão Typst:
- Counting symbols suportados: `'1'` (decimal), `'a'` (lower-alpha), `'A'` (upper-alpha), `'i'` (lower-roman), `'I'` (upper-roman), `'א'` (Alef hebraico).
- Divisão em prefixo, peças de numeral systems e sufixo.
- Formatação de cada número usando o respectivo numeral system da peça.
- Se o Alef hebraico `'א'` receber o valor `0`, emite-se um warning contendo a mensagem:
  `"the numeral system \`hebrew\` cannot represent zero"`
  e faz-se o fallback para numerais decimais árabes padrão (`0`).
- Se houver mais números do que peças no padrão, a última peça é repetida.

### 1.2. Resolução de Closure (Function)
Se o argumento `pattern` for uma função (`Value::Func`), a execução deve invocar a closure repassando os números como argumentos posicionalmente, retornando o valor gerado.

## 2. Auto-incremento de `EnumItem`

O layout de itens do enum (`+ primeiro`) deve resolver o número correto do item de forma sequencial durante o layout single-pass:
- A struct `Layouter` ganha um campo `enum_counter: Option<u32>` para controlar o número atual da lista ordenada ativa.
- Em `01_core/src/engine/layout/sequence.rs`, se o item não for um `EnumItem`, o contador é resetado para `None`.
- Em `01_core/src/engine/layout/enum_item.rs`, se o item não contiver `number` definido (`None`), o layouter calcula `enum_counter.unwrap_or(0) + 1`, atualiza seu estado e formata usando o respectivo esquema do enum. Se o item definir `number` explicitamente (`Some(n)`), o layouter atualiza o contador para `Some(n)`.
