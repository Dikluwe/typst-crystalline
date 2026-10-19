# Passo 993 — Investigação: `$...$` aninhado dentro de função de layout perde processamento matemático

**Tipo**: Diagnóstico-primeiro (ADR-0034 / ADR-0084 / ADR-0065). **Não materializar código
neste passo.** Output esperado: diagnóstico imutável + decisão B1/B2/B3 (ADR-0084), não um
fix.
**Prioridade**: Alta — achado potencialmente mais amplo do que qualquer item pendente da
série P975-992.
**Executor**: Claude Code ou Kimi Code (agente de execução). Reportar de volta ao
orquestrador com o diagnóstico completo.
**Origem**: `decalque-ACAO-crystalline-math-aninhado.md` (2026-08-08), ficheiro
`test_extended_oracle.pdf` vs `meu_extended_vanilla_v2.pdf` (Typst 0.15.1 puro).
**ADRs relevantes**: ADR-0107 (paridade com a linguagem, não a mecânica), ADR-0108 (medir
antes de decidir), ADR-0123 (geometria matemática — ler o vanilla, `file:line` citado),
ADR-0127 (gate para mudança de contrato).

---

## Antes de começar

`git status` — confirmar que o estado do repositório bate com P992 como último passo
commitado. Se não bater, parar e reportar a divergência antes de investigar.

---

## Contexto do achado (literal, do decalque)

Toda vez que uma expressão `$...$` fica aninhada dentro do argumento de uma função de
layout comum (`text(size: ...)[$...$]`, ou uma função customizada que faz `box(...)[$#x$]`),
o crystalline não processa o conteúdo aninhado como matemática — trata como texto comum.

### Evidência 1 — `#text(size: 20pt)[$b$]`

Fonte: `$ a + #text(size: 20pt)[$b$] + c $`

- crystalline: "b" maior (tamanho aplicado), mas reto, não itálico. "a" e "c" (fora do
  `text()`) continuam itálicos normalmente.
- vanilla: "b" maior E itálico.

Fonte: `$ #text(size: 8pt)[$x^2 + y^2$] = #text(size: 16pt)[$z^2$] $`

- crystalline: mostra literalmente `x^2+y^2 = z^2`, com `^` como caractere de texto — não
  interpretado como sobrescrito.
- vanilla: `x²+y² = z²`, sobrescrito real, os dois tamanhos aplicados.

### Evidência 2 — `boxed(x) = box(stroke: 0.5pt, inset: 3pt)[$#x$]`

Fonte: `$ boxed(a) + boxed(b) = boxed(c) $` e `$ boxed(a)+boxed(a)+boxed(a)+boxed(a) $`

- crystalline: nenhuma caixa aparece — nem borda, nem inset. Letras retas, não itálicas.
- vanilla: caixa com borda visível, letra itálica dentro.

### Conclusão do decalque

Sistémico: `$...$` aninhado dentro do corpo de outra função de layout perde o
processamento de matemática (itálico de variável, interpretação de `^`/`_`) e, no caso do
`box()`, perde também o efeito visual da própria função externa. Mais amplo que `attach()`
(P992, scope-out isolado a uma função matemática específica) — aqui é qualquer matemática
que atravessa a fronteira de uma função de layout comum.

---

## Escopo desta investigação

### 1. Reprodução isolada (obrigatório antes de qualquer hipótese de causa)

Para cada evidência do decalque, criar um `.typ` mínimo isolado (1 construto por ficheiro)
e compilar com o binário cristalino actual e com o vanilla de referência
(`lab/typst-original/target/release/typst`, confirmar identidade pela string distintiva —
nunca só pelo nome do caminho, per lição P934). Guardar os `.typ` mínimos em
`00_nucleo/diagnosticos/` junto ao diagnóstico final.

Casos mínimos a isolar:
- `text(size:)` sem `^`/`_` — só itálico perdido (Evidência 1, primeiro exemplo).
- `text(size:)` com `^` — itálico perdido E `^` não interpretado como sobrescrito
  (Evidência 1, segundo exemplo).
- `box(stroke:, inset:)` com letra simples dentro — caixa inteira desaparece (Evidência 2).

### 2. Extensão do padrão a outras funções de layout

O decalque assinala explicitamente que `align()`, `pad()` (e possivelmente outras) não
foram testadas. Testar pelo menos:
- `align(center)[$...$]`
- `pad(...)[$...$]`
- `block(...)[$...$]`
- Qualquer outra função de layout já materializada no cristalino que aceite `content` como
  argumento e que possa envolver `$...$`.

Registar, por função testada, se o padrão se repete, se é parcial, ou se está ausente.

### 3. Localização da causa raiz — perguntas a responder com leitura de código, não suposição

#### 3.0 — Confirmar primeiro se é a mesma limitação já corrigida por P966, só na direcção oposta

`P906` (achado #5), `P961` (Parte B) e `P966` já investigaram e corrigiram uma limitação
estrutural de `apply_math_default`: a função não atravessa (não recursa em) certos
containers de `Content`. `P961` corrigiu para `Content::MathAccent`/`Content::MathUnderover`.
`P966` corrigiu especificamente para conteúdo produzido por função de utilizador chamada de
dentro de `math` (`bra()`/`ket()`) — achado de P966: a resolução do vanilla acontece em fase
de `ir/resolve.rs` (antes do layout), enquanto o cristalino aplicava o default matemático em
fase de layout, só em containers já nativamente math; conteúdo de função de utilizador já
tinha perdido a marcação de "dentro de math" nesse ponto.

O achado deste passo (P993) pode ser a mesma limitação estrutural, na direcção oposta: em
vez de "função de utilizador dentro de `math` não recebe default" (P966), é "`math` dentro
do argumento de uma função de layout comum não recebe/preserva o contexto matemático".

Antes de investigar como causa nova:
1. Ler o código actual de `apply_math_default` (já alterado por P961/P966) e confirmar, por
   leitura directa, se a condição que decide "aplicar o default" depende de uma direcção de
   atravessamento (pai→filho) que não foi desenhada para o caso "filho matemático dentro de
   pai não-matemático".
2. Se P966 moveu a aplicação do default para fase de `eval` (paridade com o vanilla,
   Direcção (b) do esboço de P966) — confirmar se essa mudança de fase cobre ou não o caso
   inverso deste passo. Não presumir; ler o código e testar com os casos mínimos da secção 1.
3. Se confirmado que é a mesma causa de fundo: isto reduz o escopo da investigação a
   estender a correcção já feita duas vezes, em vez de procurar um mecanismo genuinamente
   novo. Registar isto explicitamente no diagnóstico, não deixar implícito.

Sobre o `box()` desaparecendo por completo (não só o conteúdo matemático interno): é mais
provável ser uma causa distinta, não coberta por P906/961/966 (que nunca lidaram com uma
função de layout externa a desaparecer inteira). Manter como investigação separada (secção 3
adiante), mas só depois de confirmar/descartar a ligação a `apply_math_default` para a parte
do itálico/sobrescrito.

#### Perguntas adicionais (se 3.0 não fechar a causa sozinho)

- Quando o parser encontra `$...$` dentro do argumento de uma função (`[...]` ou `(...)`),
  que modo de parsing é usado? Continua em modo `math`, ou reverte para modo `markup`
  assim que entra no argumento da função externa?
- Se o parsing está correcto (produz a árvore `math` esperada) mas o **eval** ou o
  **layout** é que perdem o contexto: em que ponto exacto (`file:line`) o contexto
  matemático (itálico automático de variável, interpretação de `^`/`_` como
  sobrescrito/subscrito) deixa de ser aplicado quando o conteúdo passa pelo argumento de
  uma função de layout?
- No caso de `box()`: por que é que a própria função externa (a caixa, o stroke, o inset)
  desaparece por completo, e não só o conteúdo matemático interno? Isto sugere uma segunda
  causa distinta da perda de contexto matemático — investigar separadamente. Hipótese a
  confirmar ou descartar: o argumento `[$#x$]` está a ser avaliado de forma que falha
  silenciosamente antes de chegar ao `box()`, e nada é desenhado (nem a caixa, nem o
  conteúdo) — ou o `box()` está a ser processado normalmente mas o conteúdo dentro fica
  vazio/errado de outra forma que também esconde a caixa. Confirmar com `view`/`grep` no
  código real, não por dedução.
- Comparar com o vanilla: como é que o vanilla resolve o mesmo caso — o `math` "atravessa"
  a função de layout preservando o modo, ou o vanilla usa outro mecanismo (ex.: o modo
  `math` é uma propriedade do `Content`, não do parser)? Citar `file:line` do vanilla per
  ADR-0123.
- Confirmar se isto é uma classe já conhecida de bug no projecto: procurar por
  catch-alls silenciosos em `match` sobre `Content`/`FrameItem` no caminho de avaliação de
  `text()`/`box()` com argumento `content` — mesma classe de bug documentada em P972
  (lição 8 do handoff P975→P992).

### 4. Classificação da causa (Tabela B, per ADR-0084)

Para cada sub-achado (itálico perdido, `^`/`_` não interpretado, caixa de `box()`
desaparecendo), classificar:
- **Bug real de linguagem** (afecta documentos válidos) vs **diferença aceitável**
  (mecânica-não-língua, ADR-0107) vs **scope-out conhecido**.
- Causa única (um ponto de código) ou múltiplas causas independentes que coincidem no
  mesmo sintoma.

### 5. Não fazer neste passo

- Não escrever nenhum fix.
- Não propor a forma da correcção em detalhe — só apontar onde e por que, per ADR-0034.
- Se a investigação revelar que a correcção implica mudança de contrato público (novo
  campo, nova variante fechada tipo `Content`, mudança de assinatura) — isso só é decidido
  depois, com paragem obrigatória per ADR-0127. Não presumir a forma da correcção aqui.

---

## Output esperado

Diagnóstico imutável em
`00_nucleo/diagnosticos/diagnostico-math-aninhado-layout-fase-a-passo-993.md`, seguindo o
formato canónico ADR-0034/ADR-0085:
- Cabeçalho (data, executor, padrão, diagnóstico pai = este passo).
- Tabela A: cada caso testado (função de layout × sub-sintoma), com evidência literal
  (`grep`/`view` colado, coordenadas de `pdftotext -bbox` ou equivalente quando aplicável).
- Tabela B: agregação por causa raiz identificada, com coluna explícita "mesma causa de
  P906/961/966? (sim/não/parcial)" — não deixar essa determinação implícita na prosa.
- Secção "Decisão" B1/B2/B3 (ADR-0084): recomendação de próximo passo, sem executar o fix.

Reportar de volta ao orquestrador (dono do projecto) com este diagnóstico para decisão
sobre o(s) próximo(s) passo(s) de correcção.
