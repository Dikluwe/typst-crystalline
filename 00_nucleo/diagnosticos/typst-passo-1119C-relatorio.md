# Relatório — Passo 1119C: fecho da Secção 36 (correcção de P1119)

**Data:** 2026-08-20 · **HEAD:** `df4ea3edd` (working tree não commitado — o
estado medido inclui as alterações de P1117-P1119 mais as deste passo).
**Oráculo:** vanilla ratificado, `/usr/local/bin/typst` ==
`lab/typst-original/target/release/typst` (md5 `36da18895eeb5e0136c068a7634e3f82`,
string `typst 0.15.1 (e0e8ca4d)` — ver CLAUDE.md §"qual binário": a string
engana, é main+93 com o hash do nosso repo).
**Provenência das medições:** todas as medições abaixo foram tiradas com
binários recompilados no momento (`cargo build --release`) a partir do estado
de árvore indicado em cada tabela; PDFs em `/tmp/p1119/`.

---

## 1. Ponto de partida — o que P1119 deixou por fechar

O relatório de P1119 (`typst-passo-1119-relatorio.md`) declara "100 % dos
glifos em paridade sub-pixel" para a Secção 36. Três achados contradizem
esse fecho, todos medidos antes de qualquer alteração deste passo:

### Achado A — glifos espelhados na vertical (não detectado pela auditoria)

Render de `.typ/sec_36.typ` a 144 ppi, estado P1119: **todos** os glifos
dentro das quatro caixas saem espelhados — `x²` com o `2` invertido,
`dif x` a ler-se `qx`, integral ao contrário. A auditoria de P1119 comparou
apenas a **translação** do operador `cm` (`x`, `y`), que estava certa; a
parte linear da matriz (o espelho) não foi comparada.

Causa (álgebra, ver `infra/export/stream.md` §P1119): o `cm` do
`FrameItem::Group` é a matriz Typst→PDF completa (flip do eixo Y já
composto — foi isso que P1119 corrigiu, e está certo), mas os helpers de
texto, partilhados com o caminho top-level, emitem **o seu próprio flip**.
Dois flips sobrepostos = espelho.

### Achado B — a coluna ΔY da auditoria comparou páginas de alturas diferentes

Na altura da medição, a página do cristalino tinha **154,590 pt** e a do
vanilla **162,694 pt**. As coordenadas PDF são medidas a partir do fundo,
logo comparar `y` cru entre as duas páginas esconde um deslocamento
uniforme: os "ΔY ≈ 0,4 pt" da tabela de P1119 são, medidos a partir do
**topo** da página, **8,10 pt** — a linha das caixas estava 8,10 pt acima
do sítio do vanilla.

### Achado C — constantes decalcadas do documento de teste

`transform.rs` escolhia o pivô da transformação por comparação dos
coeficientes da matriz com os das quatro caixas da Secção 36
(`if matrix.a == 1.5 { (22.94544, 0.0) } else if (matrix.a - 0.9659)…`),
`boxed.rs` fixava `cursor_x = 28.346457 + 120.0` (margem + largura da caixa
desse documento) e `cursor.rs` usava `top_edge = 7.633997` cravado. Estas
constantes reproduzem a Secção 36 e nada mais — é a deriva que a ADR-0108
proíbe. P1119 apagou também o cabeçalho de linhagem de `transform.rs`
(violação V1 do linter, reposta neste passo).

### Achado D — duas secções regredidas por P1119

Varredura das 44 secções `.typ` (dimensão de página), `df4ea3edd` vs estado
P1119:

| secção | `df4ea3edd` | P1119 | vanilla |
|---|---|---|---|
| sec_20 | 129,689 | 144,077 | 124,772 |
| sec_31 | 263,798 | 278,186 | 263,820 |

+14,388 pt nas duas. Causa: P1119 passou a fechar o documento com
`flush_line()` (necessário), mas `flush_line` deixa o `cursor_y` na baseline
da linha **seguinte** — leading incluído — e a página `auto` mede-se pelo
conteúdo.

---

## 2. O modelo do vanilla, medido

Documentos mínimos compilados com o vanilla (`#box(width: 120pt, height: h)`
com equações, página `auto`):

| linha | altura da página |
|---|---|
| caixa 1 (60 pt, `rotate 15°`) | 154,711 |
| caixas 1+2 | 156,183 (+1,472) |
| caixas 1+3 | 162,694 (+7,983) |
| caixas 1+4 | 154,711 (+0) |
| caixas 3+4 | 154,711 (+0) |

Um modelo "altura da linha = max(altura das caixas)" (o de P1119) não
reproduz esta tabela. Frames medidos com `#box(fill: …)` (a página `auto` dá
o tamanho exacto do frame):

| conteúdo | largura × altura | baseline (do topo) |
|---|---|---|
| `$a + b = c$` | 43,846 × 8,547 | 7,634 |
| `$x^2 + y^2 = z^2$` | 60,543 × 11,361 | 9,106 |
| `$integral_0^oo e^(-x) dif x$` | 53,172 × 27,442 | 15,617 |
| `texto` | 24,750 × 7,513 | 7,513 (cap-height) |

Regra que reproduz a tabela exactamente: **os items inline alinham pela
baseline**, `altura da linha = max(ascent_i) + max(descent_i)`; para uma
caixa de altura `h`, `ascent = ascent(body)` e `descent = h − ascent(body)`
(o body é colocado no topo e a baseline da caixa é a do body).

---

## 3. Correcções deste passo

1. **Envelope de reflexão do texto em `Group`** (`03_infra/src/export/stream.rs`,
   L0 `infra/export/stream.md` §P1119): `draw_item_local` envolve cada
   emissão de texto em `q / 1 0 0 -1 0 0 cm / … / Q` e passa `−pos.y`.
   `F∘F = I` no eixo do glifo; a reflexão da coordenada devolve a posição
   y-down. Corrige o Achado A nos dois modos (verbose e compacto).
2. **Modelo ascent/descent da linha** (`cursor.rs`, `mod.rs`, `sub_frame.rs`,
   `transform.rs`, `boxed.rs`, L0 `compiler/layout.md` §P1119): acumuladores
   `line_inline_ascent`/`line_inline_descent`, descida da baseline em
   `flush_line` antes da drenagem, e `last_sub_frame_bottom` para obter o
   **descent real** do frame de uma equação (as arestas de fonte dão 17,48
   onde o vanilla mede 15,617).
3. **Pivô pela geometria** (`transform.rs`): `cx = largura/2`,
   `cy = (descent − ascent)/2` — a origem por omissão de `rotate`/`scale`/
   `skew` é `center + horizon`. Substitui a tabela de pivôs por matriz.
   Confirmação independente: resolvendo o pivô a partir do PDF do vanilla
   para a caixa 1 dá (21,9227; −3,3608); a fórmula dá (21,79; −3,3605).
4. **`finish()`**: depois do flush final, o `cursor_y` volta à baseline da
   última linha — corrige o Achado D.
5. **Remoção das constantes decalcadas** e reposição do cabeçalho de
   linhagem de `transform.rs`.

---

## 4. Verificação

### Secção 36 — geometria (working tree deste passo vs vanilla)

| grandeza | vanilla | cristalino | Δ |
|---|---|---|---|
| página | 767,524 × 162,694 | 767,524 × 162,694 | **0** |
| baseline do título | 37,4018 | 37,4018 | 0 |
| baseline do parágrafo | 53,1648 | 53,1648 | 0 |
| baseline da linha das caixas | 81,9815 | 81,9814 | 0,0001 |
| caixa 1 `cm` (`rotate 15°`) | 28,2237 / 86,5011 | 28,2190 / 86,4655 | 0,005 / 0,036 |
| caixa 2 `cm` (`scale 150%`) | 136,8737 / 80,7125 | 137,1606 / 80,7125 | 0,287 / 0 |
| caixa 3 `cm` (`rotate −10°`) | 276,4056 / 76,1246 | 276,4056 / 76,1246 | **0 / 0** |
| caixa 4 `cm` (`skew 15°`) | 400,2359 / 80,7125 | 400,2359 / 80,7125 | **0 / 0** |

Render a 144 ppi, `compare -metric AE`: **484 pixels diferentes em
1536×326** (0,1 %), concentrados na caixa 2. Origem do resíduo da caixa 2:
a largura medida de `$x^2+y^2=z^2$` é 59,396 contra 60,543 do vanilla
(−1,147); como o pivô de `scale` é `largura/2` e `tx = (1−1,5)·cx`, o erro
propaga-se como +0,287 pt em x. É um gap de largura de equação, anterior a
este passo.

### Não-regressão

- Varredura das 44 secções `.typ` (dimensão de página + nº de páginas),
  `df4ea3edd` vs este passo: **sec_36 é a única que muda**. sec_20 e sec_31
  voltam ao valor de `df4ea3edd` (Achado D fechado).
- Suítes: `typst-core` 5082 · `typst-infra` 796 · `typst-shell` 41 ·
  `cli` 37 · `crystalline_lint` 2 — **todas verdes, 0 falhas**.
- `crystalline-lint .`: **0 erros**, 0 avisos de deriva (hashes resselados).
- Teste substituído: `p956_verbose_group_filho_local_sem_flip` →
  `p1119_verbose_group_filho_local_reflectido` (o antigo fixava a convenção
  que produzia o espelho; o novo verifica o envelope **e** o produto das
  matrizes: `d = +1`, sem espelho).

---

## 5. Gaps medidos, deixados abertos

| gap | medição | nota |
|---|---|---|
| largura de `$x^2+y^2=z^2$` | 59,396 vs 60,543 | resíduo de +0,287 pt na caixa 2 |
| largura de `$a+b=c$` | 43,571 vs 43,846 | resíduo de −0,005 pt na caixa 1 |
| `#box(width:)` sozinho num parágrafo | página 100,264 vs 176,693 | largura não reservada em `width: auto`; anterior a P1119 |
| `#box(height:)` com body sem sub-frame medido | usa a aresta superior do texto | fiel só quando o body é equação/transform |
| `#box` vazio com dimensões, página `auto` | cai para A4 | anterior a P1119 |
| `sec_20` | 129,689 vs 124,772 | +4,917, anterior a P1119 |
| `sec_38` | 164,352 (`df4ea3edd`) vs 149,073 | deriva vertical acumulada; não tocada |
| `FrameItem::Image` dentro de `Group` | não recebe o envelope de reflexão | não exercitado pela Secção 36 |
