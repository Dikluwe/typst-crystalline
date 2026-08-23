# L0 — Passo 1132: Divergência Vertical de Magnitude Variável — Secções 43, 27, 9

**Gate**: `ADR-0127` se confirmar necessidade de correcção. Este passo é
investigação.

**Base**: nota externa. Três secções com espaço bloco-a-bloco divergente,
magnitude **não fixa** (ao contrário do achado da margem `5.50pt`,
P1131) — varia por transição, entre `1.00pt` e `3.00pt`. **Secção 43 tem
sinal oposto às outras duas** (`+2.65pt`, cresce; secções 27/9 encolhem).

---

## 1. Não agrupar secção 43 com 27/9 sem confirmar causa comum

A nota já assume "mesma causa de fundo" para as três, mas o sinal
oposto da secção 43 é o mesmo tipo de sinal que esta investigação já
usou repeatedly para separar causas (ex.: `hat(x+y)` vs `dif`, secções
33/34). Investigar a secção 43 (título→primeiro bloco de `stack`)
**separadamente**, sem presumir a mesma origem.

## 2. Secção 43 — gap título→stack cresce

Ler o mecanismo real de transição heading→`stack()` — já diferente do
mecanismo heading→bloco/equação já corrigido (P1061-1108), dado que
`stack()` tem o seu próprio caminho de layout (já visto em P1121, P1127
— "Família B", motor de container de documento, distinto da "Família A",
motor matemático, per a distinção já estabelecida no P1127 `§2`).
Confirmar se este caso pertence à Família B e se algum mecanismo dessa
família (não da Família A, já corrigida para outros casos) está a
aplicar um `above`/gap a mais.

## 3. Secções 27/9 — hipótese da própria nota: ascent/descent de blocos complexos subestimado

Concordo com a análise da nota — a magnitude variável (não fixa) aponta
para um erro de medição dependente de conteúdo, não uma constante
errada. Padrão em comum entre os 3 casos que erram: envolvem `cases()`
(secção 9, duas das três transições) ou equação com fracção+múltiplos
subscritos (secção 27, a 4ª equação, a mais complexa da secção).

**Hipótese a testar**: o `ext.ascent`/`ext.descent` medido para blocos
com estrutura vertical complexa (`cases()`, fracção com subscritos
empilhados) pode estar a usar uma aproximação que subestima a extensão
real quando há múltiplos "ramos" verticais — mesma família de causa já
vista nesta investigação para `radical_extra_ascender` (P1104) e
`text_ink_bounds` (P1086), mas aplicada a um tipo de conteúdo diferente
(estrutura de múltiplas linhas dentro de `cases`, não glifo único).

## 4. Secção 9 — resíduo interno da 2ª equação (2 linhas) vs 1ª (3 linhas)

Achado adicional dentro da secção 9: espaçamento interno das 3 linhas
da 1ª equação bate exacto; das 2 linhas da 2ª equação tem `-1.00pt`.
Isto é estranho por si só — não seria de esperar que **menos** linhas
tivesse **mais** erro, se a causa fosse simplesmente "estrutura mais
complexa erra mais". Investigar isto com atenção antes de aceitar
qualquer explicação genérica — pode revelar que a causa real é
diferente do que a hipótese do §3 sugere (ex.: pode ser sobre o número
par/ímpar de linhas, ou sobre algo específico ao conteúdo de cada
`cases`, não sobre "complexidade" em geral).

## Critérios de verificação

1. Secção 43: gap título→stack convergindo, causa confirmada como
   Família B (ou outra), não presumida igual às 27/9.
2. Secção 27: gap `E²→Rμν` convergindo, causa em `ext.ascent`/`descent`
   confirmada com código real.
3. Secção 9: os 3 gaps convergindo, incluindo o resíduo interno
   contraintuitivo (2 linhas pior que 3 linhas) explicado, não ignorado.
4. Re-rodar P1086-1131 — zero regressão, atenção especial a `cases()`
   (secção 9 já teve o `|` corrigido em P1128 — confirmar que essa
   correcção não interfere com esta).
5. `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- §1: secção 43 tratada como investigação separada até prova de causa
  comum com 27/9.
- §3: hipótese de "ascent/descent subestimado em conteúdo complexo"
  confirmada com código real, não aceite só por padronizar com achados
  anteriores.
- §4: resíduo contraintuitivo (2 linhas pior que 3) explicado
  especificamente, não deixado sem resposta.
