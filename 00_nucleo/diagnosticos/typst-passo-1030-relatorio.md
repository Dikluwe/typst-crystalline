# Passo 1030 — `#set math.*` ignorado: a Fase A muda o tamanho do problema

**Data**: 2026-08-13
**Estado**: Fases A e B feitas. **Fase C bloqueada no gate** (ADR-0127 categoria 2, como o
próprio plano do passo antecipou). L0 redigido; **zero linhas de código**.

---

## Resumo

O plano tratava isto como um problema de plumbing: o `#set` não chega aos elementos math,
logo generaliza-se o mecanismo de `set`. A Fase A confirma o defeito e a causa, mas mede
uma coisa que o plano não previa: **para 18 dos 21 parâmetros divergentes, o problema não
é o `#set` — é que a funcionalidade não existe**. Ligar o `#set` a um parâmetro que o
construtor ignora não produz comportamento nenhum.

O passo entrega, por isso, a medição completa do alcance e um L0 de **fatia 1** com âmbito
explicitamente declarado, em vez de um fix que aparentaria fechar o assunto.

---

## Proveniência

`HEAD = ea66d651a` (Passo 1026) mais os 3 ficheiros de materialização por rastrear;
`git status` sem alterações de código no momento da medição. Binários: cristalino
`typst 0.15.0 (0f8487b9)`, vanilla `typst 0.15.1 (e0e8ca4d)` — baseline ratificado
`a51e02804`. Medições de 2026-08-13, 11:05–11:40.

Scripts em `temp/p1030/`: `alcance.py` (pares elemento/parâmetro via `#set`),
`arg_explicito.py` (o mesmo par como argumento explícito).

**Método**: para cada par, o mesmo documento com e sem a regra, nos dois binários; render
PGM 150dpi comparado por **md5** (não por bounding box — a bbox não deteta mudanças de
alinhamento, o que me deu três falsos "sem efeito" na primeira passagem). Corrupção de
conteúdo detectada por multiset de glifos (`mutool trace`).

---

## Fase A

### 1. A causa, confirmada

`eval_set_rule` (`eval/rules.rs:826`) extrai o alvo com
`set.target().to_untyped().text_str()`, que devolve `""` para um `Expr::FieldAccess`.
`#set math.mat(...)` chega ao fallback (`rules.rs:1322`) com alvo vazio e sai por
`unsupported_target_warn("")` — daí `set: target '' ainda não suportado`, com o alvo
literalmente vazio na mensagem. O único caminho math que funciona é o braço escrito à mão
para `math.equation` + `numbering` (`rules.rs:870-905`).

Existe um mecanismo genérico (`rules.rs:1300-1317`): se o alvo resolve para um
`Value::Func` com `element_name()`, todos os argumentos nomeados são empurrados para a
chain como `"<alvo>.<campo>"`. **Não serve como está**: `element_name()` só é `Some` para
elementos de utilizador (`entities/func.rs:248`, teste `:456` confirma que nativas dão
`None`), e o alvo pontuado nunca lá chega.

### 2. Alcance — 22 pares onde o vanilla aplica e o cristalino ignora

`stretch.size`, `cancel.{length,inverted,cross,angle,stroke}`, `equation.supplement`,
`op.limits`, `frac.style`, `lr.size`, `vec.{delim,align,gap}`,
`mat.{delim,align,augment,gap,row-gap,column-gap}`, `cases.{delim,reverse,gap}`.

Já funciona: `equation.numbering`.

### 3. A medição que muda o tamanho do passo

O parâmetro existe como **argumento explícito** no cristalino? (com o argumento em
primeira posição — ver o achado lateral abaixo):

| grupo | parâmetros | o que falta de facto |
|---|---|---|
| **A — arg explícito já aplica** | `mat.delim`, `vec.delim`, `op.limits` | só o caminho `#set` |
| **B — arg explícito ignorado** | `mat.{align,augment,gap,row-gap,column-gap}`, `vec.{align,gap}`, `cases.{delim,reverse,gap}`, `lr.size`, `stretch.size` | a feature de layout |
| **C — arg explícito rejeitado com erro** | `cancel.{length,inverted,cross,angle,stroke}`, `accent.size` | a assinatura da função |
| **D — sem forma de chamada** | `equation.supplement`, `frac.style` | caminho próprio |

**Só o grupo A é plumbing.** Nos grupos B/C, ligar o `#set` não muda nada visível — o
construtor descarta o valor. Um fix que só generalizasse o `set` fecharia o bilhete sem
corrigir 18 dos 21 casos, e a validação por "o warning desapareceu" passaria.

### 4. Sentido inverso — o cristalino é permissivo onde o vanilla erra

| documento | vanilla | cristalino |
|---|---|---|
| `#set math.binom(lower: $k$)` | `error: unexpected argument: lower` | compila, aviso |
| `#set math.root(index: $4$)` | `error: unexpected argument: index` | compila, aviso |
| `#set math.underline(stroke: red)` | `error: unexpected argument: stroke` | compila, aviso |
| `#set math.overbrace(stroke: red)` | `error: unexpected argument: stroke` | compila, aviso |

### 5. Achado lateral — argumento nomeado depois do último `;` infla a grelha

`$ mat(-1, 1; 1, -1, delim: "[") $` no cristalino dá delimitador **montado** (`⎡⎢⎣`) onde o
vanilla dá `[` simples — mesmo conteúdo (`−1 1 1 −1`, sem perda), grelha mais alta. Com o
argumento em primeira posição (`mat(delim: "[", -1, 1; 1, -1)`) ou após `;` próprio, o
resultado é correcto. É bug de parsing posicional, independente do `#set`; registado no L0
como âmbito diferido.

Foi este achado que quase me fez publicar uma tabela errada: na primeira passagem, com o
argumento no fim, cinco parâmetros de `mat` apareciam como "aplica" no cristalino — mas o
render mudava porque o argumento inflava a grelha, não porque o parâmetro fosse honrado. A
verificação que apanhou o erro foi comparar o multiset de glifos, não o hash do render.

---

## Fase B — gate

Categoria 2 (comportamento por defeito: parâmetros hoje ignorados passam a ter efeito), tal
como o plano do passo classificou. **L0 escrito, código não.**

`00_nucleo/prompts/compiler/eval.md` §P1030 especifica a fatia 1:

- **(a)** alvo `FieldAccess` com `math` como base resolve para a chave `math.<campo>`, em
  vez de cair no fallback com alvo vazio; o braço de `math.equation` fica intacto e
  primeiro;
- **(b)** `push_custom("math.<elem>.<param>", valor)`, com o nome do parâmetro como escrito
  na fonte (`row-gap`, não `row_gap`) — canal já usado por `page.width` e
  `smartquote.enabled`;
- **(c)** leitura na construção (`eval_math_call`), com precedência **arg explícito >
  chain > default**;
- **(d)** diagnóstico: tabela de parâmetros da linguagem por elemento; dentro da tabela mas
  não implementado → aviso que **nomeia elemento e parâmetro**; fora da tabela → **erro**
  `unexpected argument: <nome>`, como o vanilla.

O âmbito diferido está declarado no próprio L0 (grupos B, C, D e o bug posicional),
conforme a regra de divisão entre passos.

**Duas decisões para o dono:**

1. **Aprovar a fatia 1** como está (grupo A + diagnóstico), ou pedir âmbito diferente.
2. **O ponto (d) em separado**: transformar em erro o que hoje compila com aviso quebra
   documentos que hoje passam. É paridade com o vanilla e é o que o "resultado esperado" do
   passo pede, mas é categoria 2/4 e pode ser faseado à parte do resto.

---

## Validação (do estado actual, sem código novo)

```
crystalline-lint .       → 0 erros; 3 avisos V7 pré-existentes
cargo test --workspace   → 5848 passed; 0 failed  (4977 + 789 + 41 + 2 + 37 + 2)
```

5848, não os 5842 do Passo 1026: os Passos 1027-1029 acrescentaram 6 testes entretanto.

`crystalline.toml` ganhou `temp_p1030` em `[excluded]`, pela convenção já usada para
`p1021`/`p1022`/`p1026` (os scripts de medição não são código cristalino e não levam
linhagem).
