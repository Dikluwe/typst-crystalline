# Relatório — Passo 1132

**Estado:** secções 9, 27 e 43 fechadas.

**Proveniência:** `HEAD 781b207b4a`, working tree não commitado,
2026-08-21T23:22:35-03:00. No fecho da suíte, `git diff HEAD --stat` registava
31 ficheiros alterados, 873 inserções e 96 remoções (inclui P1129–P1132).

## Medição antes da decisão

- Secção 9: os intervalos internos das três linhas do primeiro `cases` e das
  duas linhas do segundo já eram idênticos ao vanilla. A divergência era
  externa: `cases.rs` devolvia apenas `grid_box.ascent/descent` e descartava
  a extensão da chaveta esticada.
- Secção 27: E²→Rμν mede 31,66900pt no cristalino e 31,669001pt no vanilla.
  O critério já estava exato; o desvio de −2,36281pt nasce na transição
  anterior e apenas é transportado até Rμν.
- Secção 43: `a→b` e `b→c` são exatos. O primeiro stack inteiro permanece
  +2,650986pt abaixo do vanilla; a divergência não nasce do espaçamento
  interno do stack. Tentativas de reinterpretar a origem do sub-frame como
  ascensão de tinta foram refutadas por render e foram removidas.

## Correção aplicada

`layout_cases` agora compõe grelha e delimitador numa baseline comum:
`ascent/descent` são o máximo dos componentes e os items recebem o offset
vertical `result_ascent - component_ascent`. O L0 de `cases` foi atualizado
antes do código e a linhagem foi resselada.

Resultado da secção 9: página cristalina 256,660 × 154,663pt, igual ao
vanilla 256,660 × 154,663pt. Os intervalos internos permanecem inalterados.

## Validação

- `cargo test --workspace`: verde (core 5092 testes; restantes crates e
  doctests verdes).
- `cargo build --release`: verde.
- `crystalline-lint .`: exit 0, zero violações; permanecem apenas avisos
  informativos preexistentes V16–V20.

## Adenda P1132b — fecho da secção 43

Nova medição isolou três parcelas independentes:

1. heading→primeiro stack: `ensure_initial_baseline` usava o top-edge textual
   de 7,513pt onde o primeiro filho math mede 4,862pt de ascensão de tinta;
   diferença exata de 2,651pt;
2. stack→stack: a margem publicada era 0,65em, ficando 0,55em = 6,05pt curta
   contra o espaçamento nominal de bloco de 1,2em;
3. `height: auto`: o stack horizontal não incorporava a descida recursiva do
   sub-frame shaped no seu fundo externo.

Após a correção, comparação por `pdftotext -bbox`:

| item | cristalino (yMin) | vanilla (yMin) | diferença |
|---|---:|---:|---:|
| a | 41,647640 | 41,647655 | −0,000015pt |
| b | 56,002640 | 56,002660 | −0,000020pt |
| c | 67,585640 | 67,585661 | −0,000021pt |
| x | 85,768640 | 85,768659 | −0,000019pt |
| = | 84,943640 | 84,943655 | −0,000015pt |
| y | 85,768640 | 85,768659 | −0,000019pt |

Dimensão da página: **391,392 × 125,236pt nos dois compiladores**. O teste de
integração `p1132b_stack_heading_e_stack_seguinte_paridade_sec43` fixa essa
paridade usando a fonte real, inclusive o deslocamento próprio de 0,825pt do
`=` relativamente a `x/y`.

O achado de `cases` e a secção 43 estão fechados. O intervalo E²→Rμν da secção
27 foi confirmado como já exato.

## Adenda P1132d — fecho da secção 27

**Proveniência:** `HEAD 781b207b4a5d`, working tree não commitado,
2026-08-22T00:01:21-03:00. No momento desta medição, `git diff HEAD --stat`
registava 61 ficheiros alterados, 1091 inserções e 137 remoções (estado
acumulado P1129–P1132).

A equação Γ isolada já produzia página idêntica nos dois compiladores
(`121,157 × 86,2389pt`), refutando a hipótese de perda na fração ou no
radical. A equação `E² = (pc)² + (mc²)²`, também isolada, media
`156,784 × 71,4285pt` no vanilla e `156,784 × 69,0657pt` no cristalino:
faltavam exatamente 2,3628pt no ascent declarado, embora a tinta coincidisse.

A causa era a classificação `is_text_like` de `layout_attach`: o cristalino
considerava qualquer base não-operador como texto, inclusive a base composta
`(m c²)`. No vanilla, essa base é um `FrameFragment`, cujo `text_like` é falso;
por isso o sobrescrito externo aplica o termo
`base_ascent − superscript_baseline_drop_max`. A classificação agora fica
verdadeira somente para fragmentos textuais atómicos sem glifo extensível.

Comparação final da secção 27:

| métrica | cristalino | vanilla | diferença |
|---|---:|---:|---:|
| página | 212,843 × 192,109pt | 212,843 × 192,110pt | arredondamento de 0,001pt no `pdfinfo` |
| baseline Γ | 84,985439 | 84,985460 | −0,000021pt |
| baseline E² | 124,427040 | 124,427059 | −0,000019pt |
| baseline Rμν | 156,096040 | 156,096060 | −0,000020pt |

O teste `p1132d_base_composta_com_sup_preserva_ascent_sec27`, com fonte real,
fixa o ascent vanilla de 11,4686pt para a equação E². A correção não move a
tinta interna; restaura apenas a morfologia vertical do frame.

## Adenda P1132e — centro interno da chave na secção 9

A correção inicial de P1132 acertou a extensão externa, mas alinhou grelha e
chave pelos topos. Como ambas as `MathBox` já partilham a baseline matemática,
isso deslocou somente o conteúdo dos `cases` `+1,866592pt` para baixo; a chave
já estava na posição vanilla. Removido o `dy`, mantendo `max(ascent/descent)`.

Na primeira chave, as baselines das linhas passaram de
57,61284/70,83264/83,98864pt para
55,74625/68,96605/82,12205pt, coincidentes com o vanilla dentro de
0,00001pt. A página permanece `256,660 × 154,663pt`. O teste
`p1132e_cases_conteudo_preserva_centro_da_chave_sec09` fixa simultaneamente a
posição interna da primeira linha e as dimensões externas.

## Adenda P1132f — secção 7, morfologia do `binom`

**Proveniência:** `HEAD 781b207b4a5d`, working tree não commitado,
2026-08-22 (medição após o build release de P1132f; estado acumulado dos
passos P1129–P1132).

A implementação anterior avaliava `binom(n, k)` como uma `MathMatrix` de duas
linhas. O vanilla avalia-o como a mesma pilha tipográfica da fracção, com a
barra desativada, envolvida por parênteses extensíveis. A aproximação por
matriz produzia um intervalo interno n→k de 13,156pt contra 14,993pt no
vanilla.

`MathFracElem` passa a transportar explicitamente `line`; `frac` público
mantém `line=true` por defeito e o construtor interno do `binom` usa
`line=false`. O layout sem barra decalca a fórmula do vanilla e seleciona os
seis valores `Stack*` da tabela OpenType MATH. A infraestrutura lê esses
valores da face ativa; os números da fonte de fixture não são usados como
ajuste de posição.

Após a correção, o PDF da secção 7 mede n em 162,427640pt e k em
177,420640pt no eixo `yMin`: intervalo **14,993000pt**, igual ao vanilla
(160,777665pt e 175,770660pt: **14,992995pt**). O conteúdo fica centrado nos
parênteses em ambos. Permanece uma translação externa de 1,649975pt nesta
linha e diferenças anteriores independentes na expressão de conjuntos e em
`cal(P)(A)`; portanto esta adenda fecha a morfologia interna do `binom`, não
declara ainda paridade integral da secção 7.

### Correções adicionais de espaçamento da secção 7

A comparação visual confirmou que o `binom` vanilla não desenha barra; a
barra pertence apenas à fração da última expressão. Foram corrigidas duas
divergências horizontais independentes:

- `|` volta a cumprir `MathItem::is_spaced`: o espaço dos dois lados é a
  largura real do espaço na fonte ativa. A expressão de conjuntos passou de
  `111,772050..179,973270pt` para `108,120050..183,625270pt`, coincidente
  com o vanilla dentro de 0,005pt por glifo;
- `cal(P)` é um glifo-base seguido por variation selector, não um run de dois
  glifos. A italics correction OpenType MATH de `𝒫` volta a integrar o
  avanço. `cal(P)(A)` passou para `133,189660..158,555660pt`, contra
  `133,189650..158,555660pt` no vanilla.

Ambas as correções são derivadas da fonte; não introduzem constantes de
posição.

### Fecho vertical e remoção da barra do `binom`

**Proveniência:** `HEAD 781b207b4a5d`, working tree não commitado,
2026-08-22T00:46:45-03:00; `git diff HEAD --stat`: 82 ficheiros alterados,
1442 inserções e 225 remoções (estado acumulado P1129–P1132).

Duas causas independentes explicavam o restante:

- `apply_math_default` e `apply_math_style` reconstruíam toda `MathFrac` pelo
  construtor com `line=true`, perdendo o modo sem barra antes do layout;
- `layout_delimited` aplicava sempre o alvo balanceado no eixo. No vanilla,
  `binom` passa `balanced=false`, portanto os parênteses selecionam variante
  pela altura real de sua pilha (`body.height()`).

O cristalino agora preserva `line` nas transformações e reconhece a forma
semântica interna `MathFrac(line=false)` para usar esse mesmo braço não
balanceado. Não há offset nem constante empírica: as posições derivam das
constantes `Stack*`, das métricas reais do corpo e da seleção de variante da
fonte.

Medição final (`pdftotext -bbox`, mesma fixture e fontes):

| glifo | cristalino `yMin` | vanilla `yMin` | diferença |
|---|---:|---:|---:|
| `n` do binômio | 160,777640pt | 160,777665pt | −0,000025pt |
| `k` do binômio | 175,770640pt | 175,770660pt | −0,000020pt |
| `n!` da linha seguinte | 197,319640pt | 197,319660pt | −0,000020pt |

O PDF cristalino contém somente uma linha desenhada, pertencente à fração
ordinária final; o binômio não contém barra. O teste
`p1132g_binom_sec07_sem_barra_e_com_posicao_vanilla` fixa tanto essa contagem
quanto as baselines de `n` e `k`.

## Adenda P1132h — espaço após `∇` na secção 11

O `δq` da equação termodinâmica já coincidia exatamente. A divergência
indicada como “delta” era o `∇` (`nabla`) antes de `⋅` e `×`: faltavam
2,444pt. U+2207 tem classe `Unary`, mas o cristalino possuía braços locais
de espaçamento para `Unary` que não existem em
`vanilla/math/ir/process.rs::spacing`; o braço `(Unary, _) → 0` interceptava
a regra posterior do operador binário.

Removidos esses braços, a classe `Binary` de `⋅`/`×` aplica dinamicamente o
gap `MEDIUM = 2/9 em` em ambos os lados. Medição final da segunda equação:
`∇` começa em 36,941060pt, `×` em 48,546060pt e `B` em 59,546060pt; vanilla:
36,941063pt, 48,548508pt e 59,550953pt (diferenças limitadas à quantização
PDF). A primeira equação também converge: `∇⋅E` começa em 69,259310pt contra
69,259310pt no vanilla.

Não foi introduzida constante empírica. O teste unitário
`p1132h_nabla_usa_classe_do_glifo_resolvido` fixa a classe Unicode e os dois
gaps `2/9 em`.

## Adenda P1132i — módulo nas secções 13, 14 e 15

A regra P1132 para o pipe solitário de conjunto tratava também os dois pipes
de módulo como `Fence` espaçado. Isso inseria a largura inteira do espaço da
fonte (3,652pt a 11pt) entre cada barra e seu conteúdo em `|xᵢ-yᵢ|`, `|z|` e
`|x|`.

`compute_gaps` agora distingue estruturalmente pipes pareados, alternando
`Opening/Closing`, de um pipe sem par, que permanece `Fence`. Medições finais:

- secção 13: o primeiro `|x` passa a começar em 130,105130pt, igual ao
  vanilla; a barra final começa em 165,745620pt, igual ao vanilla;
- secção 14: `|z|` ocupa 73,494860–85,055860pt, contra
  73,494860–85,055856pt no vanilla;
- secção 15: `|x|` ocupa 161,351800–173,759800pt, contra
  161,351790–173,759800pt no vanilla.

Os testes `p1132i_modulo_pareado_nao_recebe_espaco_interno` e
`p1132i_pipe_solitario_conserva_espaco_de_separador` fixam os dois contextos.
Não há constante posicional: módulo recebe zero pelas classes de abertura e
fecho; separador usa a largura real do espaço da fonte.

## Adenda P1132j — microdiferença do radical na secção 14

A barra e o radicando já coincidiam; somente o `√` estava 0,264pt abaixo.
`layout_root` ancorava o topo do frame do surd no centro da overline, enquanto
o vanilla (`radical.rs::sqrt_pos`) o ancora no topo da barra. A posição passa
a subtrair `radical_rule_thickness / 2`, valor OpenType MATH da fonte ativa.

Após o rebuild release, `√a` começa em `yMin=90,837440pt`; vanilla:
`90,837455pt` (diferença −0,000015pt). Os 94 PDFs cristalinos/oracle foram
regenerados com o binário release atualizado.

## Adenda P1132k/l — pequenas divergências da secção 12

Estado da medição: working tree não commitado em 2026-08-22; ficheiros
directamente alterados nesta adenda: `compiler/math/layout/attach.md`,
`compiler/math/layout/stretchy.md`, `compiler/math/layout/attach.rs`,
`compiler/math/layout/stretchy.rs` e `03_infra/src/integration_tests.rs`.
O estado cumulativo completo é o devolvido por `git diff HEAD --stat` no
fecho deste relatório.

A altura excedente de 1,507pt vinha de `(E[X])^2`: o cristalino tratava todo
delimitado como `text_like=false`; o vanilla agrega dinamicamente os
fragmentos e `(E[X])` é text-like, enquanto `(m c^2)` da secção 27 continua
false por conter um attachment. A classificação agora percorre o conteúdo e
distingue o gid-base das variantes estendidas da fonte. O teste conjunto
preserva 12 e 27.

A diferença horizontal de 0,066pt depois de `[` era a
`ItalicsCorrection=6du` da NewCMMath-Book. O gid-base emitido como `Glyph`
agora soma a correção lida da tabela MATH; variantes estendidas não somam.
Não foi introduzida constante empírica. O teste
`p1132k_sec12_limite_unilateral_preserva_altura_vanilla` fixa a altura da
página e a posição do primeiro `X` da segunda equação.

## Adenda P1132m — espaço fino antes de `dif t` na secção 12

Estado: working tree não commitado em 2026-08-22; ficheiros directamente
alterados: `compiler/stdlib/structural.md`, `compiler/stdlib/structural/math.md`,
`compiler/math/layout/_comum.md`, `compiler/stdlib/structural/math.rs`,
`compiler/math/layout/mod.rs`, `compiler/eval/tests.rs` e
`03_infra/src/integration_tests.rs`.

O gap medido entre `e` e o `d` era 15,302pt no cristalino e 17,135pt no
vanilla. A diferença de 1,833pt é exactamente o `THIN=1/6em` definido pelo
vanilla em `math/op.rs`, a 11pt. `dif`/`Dif` agora incluem o `HSpace` fraco
em unidade `em`; o layouter o conserva entre itens e o colapsa nas bordas do
run. Não foi usada constante posicional do corpus. A regressão estrutural
fixa `1/6em + weak + Unary`; a integração fixa o `d` em 179,11551pt.
## P1132n — secção 18, chaves aninhadas

Medição em 2026-08-22, working tree não commitado. Estado relevante:
`underover.rs`, `glyph_variants.rs`, `font_metrics.rs`, respectivos L0 e teste
de integração alterados. O teste RED observou ambas as variantes com
27.478pt e a chave superior em x=100.7374567. Após propagar a métrica MATH
da variante e a largura exacta do spreader, o teste GREEN observa a chave
superior em x=100.58346/27.478pt e a inferior em x=97.82246/33.000pt, iguais
ao vanilla ratificado. A seleção continua sendo “primeira variante cujo
AdvanceMeasurement cobre o alvo”; nenhuma constante do PDF foi usada no
algoritmo.
## P1132o — secção 18, fração contínua

Medição em 2026-08-22, working tree não commitado. O cristalino aplicava
espaços Binary nos três níveis de `1/(1+1/(1+1/(1+x)))`; o vanilla os mantém
somente no nível Text e os suprime em Script/ScriptScript. O teste RED mediu
os `+` em x=96.76792/114.96278/128.40221; após usar o `MathSize` discreto no
gate de spacing, o GREEN mede x=99.701259/116.185/126.6911, igual ao vanilla.
Arquivos relevantes: `math/layout/mod.rs`, `spacing.md`, `_comum.md` e o teste
de integração. Estado: working tree não commitado.

## P1132p — secção 10, brackets e braces

Medição em 2026-08-22, working tree não commitado. Depois de P1132n, as
assemblies horizontais de `underbracket`/`overbracket` tinham a largura certa,
42,6238pt, mas começavam em x=104,40351pt; o vanilla começa em x=94,81457pt.
A causa era aplicar a métrica do primeiro glifo como se toda a assembly fosse
uma variante única. O attachment de uma assembly passa a ser o seu centro
geométrico, conforme o frame horizontal do vanilla.

Na revalidação GREEN, `⎵` e `⎴` começam em x=94,81457pt, e `⏟` e `⏞` em
x=93,93945pt, iguais ao vanilla ratificado. A compensação de largura das
chaves simples é derivada de `TopAccentAttachment` e aplicada somente no
nível que contém a peça; wrappers não a duplicam. Não foi introduzida
constante empírica. Arquivos relevantes: `underover.rs`, seu L0 e o teste
`p1132p_sec10_assembly_de_bracket_usa_attachment_central` em
`03_infra/src/integration_tests.rs`. Estado cumulativo: working tree não
commitado; a lista exata de alterações é a de `git diff HEAD --stat` no
fecho deste relatório.

## P1132q — secção 10, `cancel` e `std.strike`

Medição em 2026-08-22, working tree não commitado. No stream PDF, o
`cancel(a+b)` cristalino tinha traço x=103,723–127,785pt e 0,500pt de
espessura; o vanilla tinha x=102,502–129,747pt e 0,550pt. A implementação
passa a aplicar os defaults da linguagem `length: 100% + 0.3em` e
`stroke.thickness: 0.05em`, projetados dinamicamente pela diagonal da caixa.

A inspeção do stream também refutou a interpretação anterior de
`std.strike(a+b)`: o vanilla contém os três glifos matemáticos e nenhuma
operação de linha; somente o cristalino desenhava um risco horizontal. O
braço math agora preserva o corpo itálico sem materializar a decoração
textual. O comportamento de strike em texto comum permanece inalterado.
Não foram usadas coordenadas do corpus no algoritmo. Arquivos relevantes:
`math/layout/cancel.rs`, `math/layout/mod.rs`, `math/layout/tests.rs`,
`cancel.md` e `_comum.md`; estado cumulativo é o `git diff HEAD --stat` no
fecho.

## P1132r — secção 22, brackets invertidos de `lr`

Medição em 2026-08-22, working tree não commitado. Os sete primeiros casos
da secção 22 coincidiam; em `lr(\]a/b\[)`, o cristalino deixava os brackets
como glifos comuns de 3,058pt e produzia página de 333,895pt, enquanto o
vanilla usava as variantes escaláveis de 5,192pt e página de 334,236pt.

A causa estava no eval: a guarda exigia Opening à esquerda e Closing à
direita, mas o vanilla aceita qualquer classe delimitadora nas extremidades
e redefine o papel pela posição. Além disso, os escapes chegam como
`Content::Text`. A projeção agora aceita `Text`/`MathText`/`MathIdent` de um
único caractere, ainda condicionada a Opening/Closing/Fence, e produz
`MathDelimited(']', corpo, '[')`. A integração fixa que o par invertido
seleciona exatamente os mesmos gids e advances do par normal. Nenhum
mapeamento específico da fixture foi introduzido. Arquivos relevantes:
`compiler/eval.md`, `compiler/eval/math.rs`, `compiler/eval/tests.rs` e
`03_infra/src/integration_tests.rs`; estado cumulativo é o
`git diff HEAD --stat` no fecho.

## P1132s — espaçamentos da secção 25

Medição em 2026-08-22T09:55:18-03:00, HEAD `781b207b4`, working tree não
commitado (`git diff HEAD --stat`: 213 ficheiros, 8437 inserções e 4305
remoções; estado acumulado P1129–P1132). A página cristalina media
222.175×271.666pt contra 218.523×271.666pt no vanilla. O diagnóstico separou
três causas estruturais: o vetor de gaps perdia a fronteira posterior a
`HSpace` e deslocava o espaço de `z =` para `d z`; itens textuais `spaced`
perdiam o espaço real da fonte em scripts; e o avanço terminal do frame inline
de `"Res"` era contado antes de `(`.

Após a correção, os dois PDFs medem 218.523×271.666pt. `dif z` volta a formar
`dz`; `"Res"(f, a_k)` encosta à abertura; e o limite `p "prime"` conserva
2.5564pt entre as caixas visíveis no tamanho Script e fica centrado sob `∏`.
Os valores de produção vêm de `THIN`/classes matemáticas e das métricas da
fonte ativa (`advance(" ")`), sem coordenada nem constante da fixture.
## P1132t — regressão da secção 26 após P1132s

Medição em 2026-08-22T15:27:31-03:00, HEAD `781b207b4`, working tree não
commitado (`git diff HEAD --stat`: 213 ficheiros, 8559 inserções e 4317
remoções; estado acumulado P1129–P1132). O achatamento indiscriminado de
`Content::Sequence` separava os fragmentos internos de `bra`/`ket`; depois da
regra `spaced` de P1132s, surgiram espaços dentro de `⟨φ|` e `|ψ⟩`. Preservar
toda `Sequence`, por outro lado, achatava semanticamente `hat` em texto e
duplicava/perdia acentos.

A correção preserva como caixa atômica apenas `Sequence` cujos filhos são
folhas textuais, mantém essa caixa como item `spaced` externo e continua
abrindo sequências técnicas (`HSpace`, como `dif`) ou compostas (`hat`, attach,
delimitados). A secção 26 voltou a 207.925×220.758pt, igual ao vanilla; a 25
permaneceu em 218.523×271.666pt. Não há lista de caracteres nem ajuste de
coordenadas na produção.
## P1132u — espaço externo do módulo na secção 28

Medição em 2026-08-22T15:44:07-03:00, HEAD `781b207b4`, working tree não
commitado (`git diff HEAD --stat`: 213 ficheiros, 8608 inserções e 4317
remoções; estado acumulado P1129–P1132). Em `2 |E(G)|`, o cristalino fundia
`2|E(G)|`; o vanilla deixava 3.652pt, a largura real do espaço a 11pt.

O módulo pareado passou a usar classes laterais assimétricas: a barra inicial
é `Fence→Opening` e a final `Closing→Fence`. Isso mantém o interior compacto e
ativa `spaced` externamente em Text/Display. Em Script a face externa do par
não ativa `spaced`, preservando `|z|=R` da secção 25. Resultado: secção 28 em
198.738×171.009pt, secção 25 em 218.523×271.666pt e secção 26 em
207.925×220.758pt, todas iguais às dimensões vanilla. Nenhuma coordenada da
fixture entrou na produção.
## P1132v — scripts da soma inline na secção 31

Medição em 2026-08-22T15:50:55-03:00, HEAD `781b207b4`, working tree não
commitado (`git diff HEAD --stat`: 213 ficheiros, 8666 inserções e 4313
remoções; estado acumulado P1129–P1132). No `sum_(k=1)^n` inline, o topo dos
scripts cristalinos estava 1.272pt abaixo e o fundo 1.998pt acima do vanilla;
a soma em bloco já coincidia.

O diagnóstico antigo P1123 estava correto: o par era tratado como se a base
fosse texto. A correção reproduz `GlyphFragment.extended_shape`: operadores
grandes de um carácter são não-text-like mesmo quando o inline usa o glifo
base, fazendo `compute_script_shifts` considerar a altura/descida da base e o
gap OpenType normal. Depois da alteração, a bbox combinada dos scripts inline
é 41.45844–59.60844pt no cristalino contra 41.45845–59.60844pt no vanilla.
Nenhum delta medido foi codificado.
