# Passo 996 — `\` em math é quebra de linha ANTES do emparelhamento lr: `(n \ k)` não estica parênteses

**Precede este passo**: achado de P995 (reconciliação, `typst-passo-995-relatorio.md`) +
confirmação do dono citando a documentação oficial (typst.app/docs/reference/math/): a barra
invertida `\` em math é definida **apenas como quebra de linha** ("Line Breaks — Formulas can
also contain line breaks"). O cristalino errou ao tratar `(n \ k)` como grelha delimitada com
parênteses esticados — no vanilla são duas linhas centradas com parênteses de tamanho natural.

**Pré-condição de árvore**: `git status`. Confirmar HEAD ≥ `5074d5021` (P995 + adendo).

---

## Fase A — medições já feitas (registadas aqui, não repetir)

Todas com o vanilla (`/usr/local/bin/typst`, baseline ratificado):

1. `$ (n \ k) $` — duas linhas centradas (centros 39.31 = 39.31pt), parênteses
   **naturais (~11pt)**, um por linha: `(n` em cima, `k)` em baixo. Idêntico no
   0.15.1 oficial (PDF cacheado pré-sync) e no build main.
2. `$ ( a = b \ c = d ) $` — idem para linhas multi-token: `(`/`)` naturais, um
   por linha; linhas centradas.
3. `$ (n \ sum_(i=1)^n k) $` — mesmo com a 2ª linha alta (~32pt, ∑ com limites),
   o `)` fica **natural** (11pt) — o vanilla NUNCA estica delimitadores sobre
   quebra de linha.
4. Mecanismo vanilla (`file:line`, lab actual): o `\` vira `RawMathItem::Linebreak`
   (`ir/resolve.rs:179-180`); `expand_multiline_fence`
   (`ir/ir/multiline.rs:56-107`) divide o corpo "fenced" em segmentos por linha e
   pendura o delimitador de abertura só no 1º segmento e o de fecho só no último,
   dimensionado pelo conteúdo do PRÓPRIO segmento (`SharedFenceSizing`,
   `ir/item.rs:1053-1081`) — nunca pela pilha inteira.
5. Cristalino actual: o lexer emparelha os parênteses (`Expr::MathDelimited`) e
   `eval_math_expr` emite `Content::math_delimited(open, body, close)`
   (`eval/math.rs:361-368`) com o `Linebreak` DENTRO do corpo → o layout estica
   os delimitadores sobre a grelha (1 coluna, centrada desde P991). Daí o n a
   0.00/0.00pt (a "folga zero" da investigação paralela).

## Fase B — TDD directo

1. L0 primeiro (`engine/eval.md` §P996 + nota em `math/layout/_comum.md` se o
   layout for tocado).
2. Teste: `$ (n \ k) $` — não há peças esticadas de delimitador (`⎛/⎜/⎝/⎞/⎟/⎠`
   ausentes; `(` e `)` como texto simples, tamanho natural); n e k centrados um
   sob o outro (centros iguais ±0.05pt). Guardas: `$ binom(n, k) $` inalterado
   (estica, caminho próprio); `$ (a \ b) $` com `&` (grelha com alinhamento)
   inalterado; `cases`/`mat` inalterados (funções, caminho próprio).
3. Implementação proposta (lado eval, mínima): no braço `Expr::MathDelimited`,
   se o corpo avaliado contém `Content::Linebreak` ao nível do topo, **não
   emparelhar** — emitir `MathSequence[MathText(open), …corpo…, MathText(close)]`
   (parênteses como glifos normais, tamanho natural), deixando o caminho de
   quebra/grelha existente centrar as linhas. Se a Fase B revelar efeito
   colateral (ex.: encadeamento com `&` dentro dos parênteses, ou aninhamento
   `((n \ k))`), registar e decidir com medição, não presumir.

## Fase C — Revalidação

1. Secção 7 do canónico: `(n \ k)` idêntico ao vanilla (parênteses pequenos,
   linhas centradas); `binom(n, k)` inalterado (esticado, com padding).
2. Confirmação visual a alta resolução.
3. Benchmark canónico, 7 cenários, zero regressão.
4. Relatório **com hash do commit final** (regra reforçada — sem placeholder).

## Resultado esperado

- `\` quebra a linha antes de qualquer emparelhamento lr — paridade com a
  documentação e o vanilla medido.
- A "folga zero em n" desaparece por construção (não há delimitador esticado
  para encostar ao n).
