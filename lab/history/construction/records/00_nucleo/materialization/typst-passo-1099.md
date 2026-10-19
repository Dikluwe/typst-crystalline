# L0 — Passo 1099: `$...$` Perde Processamento Matemático Dentro de Função de Layout Aninhada

**Gate**: `ADR-0127` — mudança de comportamento por defeito. **Prioridade
alta**, per a nota original (afecta qualquer combinação de estilo visual +
matemática, não caso de uso raro).

**Base**: nota externa (2026-08-08), documento
`decalque-ACAO-crystalline-math-aninhado.md`. Vanilla confirmado como
referência (`meu_extended_vanilla_v2.pdf`, Typst 0.15.1). Padrão: `$...$`
aninhado dentro do argumento de `text()`/`box()` (e possivelmente
`align()`/`pad()`, não testadas ainda) perde processamento matemático —
itálico de variável, interpretação de `^`/`_`, e no caso de `box()`, perde
também o efeito visual da própria função externa (borda, inset). Mais amplo
que o achado anterior de `attach()` — ali era função matemática específica
não reconhecida; aqui é `$...$` genérico dentro de qualquer função de layout.

---

## 1. Não presumir mesma causa que `attach()` — nem presumir causas diferentes

A nota diz explicitamente que isto é "mais amplo" que o achado de `attach()`
— mas não diz que são causas **diferentes**, só que o escopo do sintoma é
maior. Investigar os dois mecanismos (avaliação de função matemática nativa
não reconhecida vs perda de contexto matemático em aninhamento de função de
layout) antes de assumir se partilham raiz comum (ex.: ambos podem ser
sintomas de "o avaliador não propaga um estado de `in_math_mode` através de
certos tipos de chamada de função") ou são independentes.

## 2. Ler o código real antes de investigar

- `01_core/src/compiler/parse/math.rs` (já visto nesta conversa antes,
  achado do P1085/parsing de matemática).
- `01_core/src/compiler/eval/math.rs` (avaliação de expressões matemáticas,
  já citado em achados anteriores — `vec()`, `lr()`, delimitadores).
- `01_core/src/compiler/eval/closures.rs` ou onde `eval_func_call` vive
  (mecanismo de intercepção já visto nesta conversa para `measure()`/
  `layout()` — confirmar se `text()`/`box()` passam pelo mesmo caminho e se
  esse caminho preserva ou descarta o modo matemático do conteúdo do
  argumento).

## 3. Hipótese a testar — perda de flag/contexto ao entrar em argumento de função

Quando o parser/avaliador encontra `$...$` dentro do corpo de `[...]` que é
argumento de uma chamada de função (`text(size: X)[$b$]`), confirmar: o
conteúdo dentro dos `$...$` internos é reconhecido como `Content::Equation`
(ou variante math) no parse, ou é lido como `Content::Text` simples porque o
parser já não está em "modo math" ao entrar no argumento de função?

Isto distingue duas classes de bug bem diferentes:
- **Se for erro de parse**: o `$...$` interno nunca vira nó de matemática —
  problema sintáctico, provavelmente em `parse/math.rs` ou no parser
  principal, sobre como argumentos de função com colchetes são tratados
  quando o contexto externo já é matemático.
- **Se for erro de avaliação/layout**: o parse está correcto (produz
  `Content::Equation` interno), mas o layout de `text()`/`box()` não invoca
  o layouter matemático para o conteúdo do seu argumento, tratando-o como
  texto genérico.

## 4. Testar as funções ainda não confirmadas pela nota

`align()`, `pad()`, e outras funções de layout comuns — confirmar se o
padrão se repete (a nota já pede isto explicitamente, não testado ainda).
Se só `text()`/`box()` forem afectadas e outras não, isso restringe a causa
a algo específico dessas duas, não a um mecanismo geral de todas as funções
de layout.

## 5. Caso `box()` — segundo sintoma, não esquecer

Além de perder o itálico/sobrescrito, `box()` perde também o próprio efeito
visual (borda, inset) quando o argumento é `$...$`. Confirmar se isto é o
mesmo bug (o `box()` nem está a processar o argumento correctamente de todo)
ou dois bugs empilhados (o conteúdo perde formatação matemática E o `box()`
falha por outro motivo ao receber esse conteúdo malformado).

## Critério de conclusão

- §3 respondido — erro de parse vs erro de avaliação/layout, com código
  citado, não suposição.
- §4 — pelo menos `align()` e `pad()` testadas, resultado registado mesmo
  que negativo (padrão não se repete).
- §5 — os dois sintomas de `box()` (perda de matemática + perda de efeito
  visual) tratados como possivelmente duas causas, não uma só por padrão.
- §1 — relação com o achado de `attach()` confirmada ou descartada
  explicitamente, não deixada em aberto.
