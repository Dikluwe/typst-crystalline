# L0 — Passo 1123: Regressão da Secção 35 + Cinco Achados Novos

**Gate**: `ADR-0127` se confirmar necessidade de correcção. Este passo é
investigação. **Prioridade**: (0) regressão da secção 35 — urgente, pode
indicar quebra de trabalho já fechado; (1) secção 27 (`+`/`dif`, afecta
tamanho de página); (2) secção 41 (sobrescrito empilhado); (3) secção 31
(sum inline); (4) secções 26/28 (pipe como fence); (5) resto, catalogar.

---

## 0. URGENTE — `crystalline` (build em teste) regrediu face a `oracle` na secção 35

**Antes de qualquer achado novo**: a secção 35 já tinha sido fechada no
P1115 (`descent` de sub-frame com `text(size:)` corrigido). O reteste
mostra `oracle` a bater quase exacto com o vanilla (`135.059` vs
`135.059pt` de altura), mas `crystalline` com `125.666pt` — `9.393pt` de
diferença, e o padrão de deslocamento por bloco é **inverso** ao já
corrigido (encolhe, não cresce).

**Isto não é achado novo — é candidato a regressão real.** Confirmar:
1. Que commit/estado exacto o `oracle` desta rodada representa (o
   correspondente ao P1115 fechado, ou mais recente?) — e que commit o
   `crystalline` de teste representa.
2. Se houver diferença de commits, fazer `git bisect` (ou equivalente
   manual) entre os dois estados para localizar qual passo entre P1115 e
   agora introduziu esta regressão — provavelmente algo em P1116-P1122
   que tocou `equation.rs`/`layout_external`/`cursor.rs` de novo sem
   revalidar a secção 35 especificamente.

**Não misturar isto com os achados novos abaixo** até a causa da
regressão estar identificada — pode ser a mesma causa de outros achados
desta lista, ou pode ser independente.

## 1. Secção 27 — espaço fixo extra após `+` antes de `dif`

**Achado já bem isolado pela nota**: gap `+→dif` tem `+2.45pt` fixo
(não escala), enquanto o gap sem `+` antes (justaposição directa) já bate
exacto. Hipótese da própria nota, plausível: o espaçamento "bin" (ao
redor do operador binário `+`) está a **somar-se** ao espaçamento próprio
de `dif` (`THIN`, já implementado em P1115), em vez de os dois
espaçamentos concorrentes serem resolvidos por `max()` (mesmo princípio
de fraqueza/precedência já usado nesta investigação, P1061 em diante,
para espaçamento vertical — aqui aplicado a espaçamento horizontal entre
classes matemáticas).

**Ler o código real**: mecanismo de espaçamento por par de classes
(`spacing.rs`, já usado em P1097/P1114) — confirmar se a soma em vez de
`max()`/substituição é mesmo o que acontece para o par `Bin→dif`
especificamente, não presumir sem ver.

**Impacto directo em tamanho de página** — prioridade alta per a nota,
concordo.

## 2. Secção 41 — sobrescrito empilhado ganha espaço extra; subscrito não

**Achado bem isolado**: subscrito (`(m,n)→(i,j)`) bate exacto nos dois
níveis; sobrescrito (`(k,l)→(o,p)`) ganha `+1.54pt` no primeiro nível e
mais `+1.01pt` no segundo — cresce com a profundidade, só do lado do
sobrescrito.

**Ler o código de `attach.rs`** — já extensamente trabalhado nesta
investigação (P1089-1105) para sub/sobrescrito simples; confirmar se o
caminho de **múltiplos** sobrescritos empilhados no mesmo átomo base
(`A^(k,l)^(o,p)`-like, embora a sintaxe real seja diferente) usa lógica
diferente do caminho de múltiplos subscritos, já correcto. Comparar os
dois caminhos lado a lado, per a recomendação da própria nota.

## 3. Secção 31 (reteste) — `sum` inline com limites comprimidos

**Achado**: bloco (equação isolada) correcto; inline (dentro de frase)
tem sobrescrito/subscrito do `∑` `~3.27pt` mais apertados entre si.

**Hipótese a testar**: escala de `MathSize` (`Script`/`ScriptScript`) já
corrigida para composição em P1092 — confirmar se essa correcção cobriu
o caso específico de limites de operador grande (`sum`/`prod`/`integral`)
em modo inline, ou só sub/sobrescrito comum. Ler `attach.rs`/`math/
layout/mod.rs` para o caminho de `is_limits`/scripts laterais em modo
`MathSize::Text` (já tocado em P1098-1104 para a questão de `limits`
padrão, mas ali era sobre empilhado-vs-lateral, não sobre a distância
entre os dois quando laterais).

## 4. Secções 26/28 — `|` classificado como classe errada (não-fence)

**Achado confirmado em dois contextos** (macro custom e `lr()` nativo do
Typst para valor absoluto) — mesmo valor (`~3.65pt`, próximo de
`THICK`/6mu), reforçando que a causa é a classificação do átomo `|`, não
algo específico à macro do usuário.

**Ler a tabela de classificação de átomo matemático** (mesma área já
usada para `spacing.rs`, P1097/P1114) — confirmar se `|` está classificado
como `Rel`/`Bin` em vez de `Fence` no cristalino.

## 5. Catalogar, prioridade baixa/menor — não investigar a fundo neste passo

- Secção 18: fracção contínua (`dx` por nível), `√a+√b+√c` (linha
  inteira desviada), `underbrace(overbrace())` — pequenos, registar.
- Secção 20: per P1122 (§3 desse L0) — ainda pendente de confirmação
  se é lacuna do P1108 ou não; não duplicar investigação aqui.
- Secção 23: oscilação horizontal na primeira equação, crescimento nas
  seguintes — per P1122, já em investigação.
- Secção 29: `log_2`, subscrito `1.75pt` baixo — isolado, baixa
  prioridade, registar para o caso de reaparecer com outra função
  nomeada.
- Secção 38 (reteste): bug de referência **não reproduz** (já corrigido
  por P1121, confirmado) — só resíduo pequeno (`0.6pt`) da família geral,
  catalogar.
- Secção 40: `box()` como mais um gatilho da família "espaço entre
  blocos" — juntar à lista de casos de teste já reunida (secção 35, 38,
  agora 40) para quando a causa raiz geral for investigada.

## Critérios de verificação

1. §0: causa da regressão da secção 35 localizada (qual passo/commit),
   corrigida, secção 35 revalidada a `0.0000pt`.
2. §1: gap `+→dif` convergindo, mecanismo de precedência (não soma)
   confirmado com código real.
3. §2: sobrescrito empilhado convergindo, código comparado lado a lado
   com subscrito (já correcto).
4. §3: `sum` inline convergindo.
5. §4: `|` classificado como `Fence`, ambos os contextos (macro e `lr()`
   nativo) corrigidos de uma vez.
6. Re-rodar P1086-1122 — zero regressão, com atenção especial à secção
   35 (§0) e a qualquer secção que já tenha usado `text(size:)`/
   `display`/`script`/`sscript`/`box()` em equação.
7. `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- §0 resolvido antes de qualquer outra correcção ser considerada fechada
  — regressão em trabalho já validado é a prioridade mais alta deste
  passo.
- Cada achado (1-4) com código real citado, não descrição em prosa.
- §5 registado, não necessariamente corrigido.
