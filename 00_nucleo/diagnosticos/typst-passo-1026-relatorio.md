# Passo 1026 — o gate de delimitadores dissolve-se: a folga de 10% é do vanilla

**Data**: 2026-08-13
**Estado**: **fechado**. Fases A, B e C feitas. O gate ADR-0127 que bloqueava C/D deixou de
ter objecto — não há divergência para decidir. **Zero alteração de código de produção.**

---

## Resumo

O passo foi aberto para decidir o que fazer com o factor `* 1.1` no alvo dos delimitadores
esticáveis de `mat`/`cases`, que dois relatórios anteriores tinham classificado como
divergência face ao vanilla. A Fase C mediu o vanilla no call site do construto certo e a
premissa caiu:

```rust
// lab/typst-original/crates/typst-library/src/math/ir/resolve.rs:1173
// fn resolve_delimiters — "Resolves the delimiters around the body of a vector, matrix, or cases"
let target = Rel::new(Ratio::new(1.1), Abs::zero());
let stretch = Stretch::new().with_y(StretchInfo::new(target, DELIM_SHORT_FALL));
```

**O vanilla também usa 110%**, com o mesmo `DELIM_SHORT_FALL`. O `* 1.1` do cristalino é
paridade. A Opção A do plano (alinhar o alvo a 100%) **introduziria** a divergência que se
julgava estar a remover.

---

## Proveniência

`HEAD = 0f8487b9d`, árvore de trabalho **limpa** (`git status --porcelain` → 0 ficheiros),
medições de 2026-08-13 07:46–08:20. Binário cristalino **reconstruído no HEAD actual** antes
de medir (`cargo build --release --bin typst` → `typst 0.15.0 (0f8487b9)`); referência
vanilla `lab/typst-original/target/release/typst` → `typst 0.15.1 (e0e8ca4d)`, baseline
ratificado `a51e02804`.

Documentos, scripts e PDFs em `temp/p1026/` (`varredura_fronteira.py`, `medir_delim.py`).

**Controlo de confound — os dois binários trazem ficheiros de fonte diferentes.** O
cristalino fixa `typst-assets` em `c0ae970` (0.15.0), o lab em `94dcb99` (0.15.1); os 31
ficheiros de fonte diferem em md5. Verificado antes de comparar píxeis, sobre
`NewCMMath-Regular.otf`: tabela `MATH` **byte-a-byte idêntica**; contornos e avanços de
`parenleft`, `parenleft.v1…v7`, `uni239B/C/D` idênticos; `cmap` funcionalmente idêntico
(4887 codepoints, zero diferenças de mapeamento) apesar de diferir em bytes; ambas
`NewCMMath 4.0`, 8603 glifos. **Não-achado, registado para não voltar a ser perseguido.**

---

## Fase A — o que ficou de pé e o que caiu

A Fase A corrigiu um erro do Passo 1024 e introduziu outro.

**Ficou de pé (confirmado na Fase C):** o short fall do vanilla **está portado**.
`stretchy.rs:57-61` subtrai `DELIM_SHORT_FALL = 0.1em` antes de escolher a variante, com a
distinção fina do vanilla — só delimitadores; o radical usa `short_fall = 0`
(`resolve.rs:1248`). O `×1.1` e o `−0.1em` aplicam-se **em sequência**, não em alternativa.
A afirmação do Passo 1024 ("multiplica em vez de subtrair") continua refutada.

**Caiu:** a tabela da Fase A dava o alvo do vanilla como `Rel::one()` citando
`math/ir/resolve.rs:843`. Esse call site é `resolve_skewed_frac` — **a barra da fracção
inclinada**, não o delimitador de grelha. A conclusão foi tirada de um call site vizinho, do
construto errado.

**Caiu também o achado lateral:** a Fase A registou, a partir de extracção de texto, que "o
vanilla estica um glifo variante e o cristalino monta a partir de `U+239B…U+23A0`". Falso —
ver Fase C.

---

## Fase C — medição

### 1. O call site certo, e a verificação de âmbito

O `1.1` tem de aparecer neste caminho e em mais nenhum, dos dois lados. Confere:

| construto | cristalino | vanilla ratificado |
|---|---|---|
| grelha (vector/matriz/cases) | `math/layout/mod.rs:390`, chamado **só** por `cases.rs:41` e `matrix.rs:113` | `resolve.rs:1173` — `Ratio::new(1.1)` |
| `binom` | não usa `grid_delim_target_du` | `resolve.rs:754` — `Rel::one()` |
| fracção inclinada | idem | `resolve.rs:843` — `Rel::one()` |
| `lr` / `mid` | idem | `resolve.rs:881`, `:917` — tamanho pedido; `Rel::one()` por defeito (`lr.rs:26`) |
| radical | `layout_radical_symbol`, `short_fall = 0` | `resolve.rs:1248` — `Rel::one()`, `Em::zero()` |
| acento | eixo horizontal | `resolve.rs:1432` — `Rel::one()` |

`grep` de `1.1` em todo o `01_core/src/compiler/math/layout/` devolve **uma** ocorrência
(`mod.rs:390`). Âmbito idêntico.

**A montagem por peças recebe o alvo cru nos dois.** Vanilla: `glyph.rs:271` calcula
`short_target = target − short_fall` e usa-o só na busca de variante; `glyph.rs:312` passa
**`target`** a `assemble`. Cristalino: `layout_stretchy_delimiter_impl` passa `min_height_du`
(pré-short-fall) a `layout_assembly`. Igual.

### 2. Output — os dois produzem o mesmo delimitador

Método: mesmo documento nos dois binários → PDF → PGM 300dpi (`pdftoppm -gray`); banda
horizontal do delimitador esquerdo **auto-calibrada** (da coluna de tinta mais à esquerda até
um intervalo de ≥20 colunas vazias — a banda fixa de 5pt da Fase A cortava a ponta do glifo,
que curva para a direita); contagem de glifos desenhados por `mutool trace`.

| `mat` (11pt) | vanilla | cristalino | Δ | Δ px@300dpi | glifos v / c |
|---|---:|---:|---:|---:|---|
| 2 linhas | 26,160pt | 26,160pt | 0,000 | 0,0 | 4 / 4 |
| 3 linhas | 40,800pt | 40,800pt | 0,000 | 0,0 | 11 / 11 |
| 4 linhas | 55,200pt | 55,440pt | +0,240 | +1,0 | 18 / 18 |
| 5 linhas | 69,840pt | 69,840pt | 0,000 | 0,0 | 25 / 25 |
| 6 linhas | 84,240pt | 84,240pt | 0,000 | 0,0 | 30 / 30 |
| 8 linhas | 113,040pt | 113,280pt | +0,240 | +1,0 | 44 / 44 |
| 10 linhas | 142,080pt | 142,080pt | 0,000 | 0,0 | 57 / 57 |
| 12 linhas | 171,120pt | 171,120pt | 0,000 | 0,0 | 73 / 73 |

0,240pt é exactamente 1 pixel a 300dpi. Com as 30 configurações da Fase A (2-12 linhas a
11pt; 3 e 5 linhas a 8/9/10/11/12/14/16/18/20/24pt), são **38 configurações, nenhuma acima de
1 pixel**. A explicação correcta não é "a quantização absorve a inflação" — é que **não há
inflação a absorver**.

### 3. O achado lateral da Fase A, refutado

`mutool trace` conta os glifos realmente desenhados:

| `mat` | vanilla | cristalino |
|---|---|---|
| 5 linhas | 1×`(` + 1×`)` + 18 peças sem ToUnicode = **20** | (1×⎛ + 8×⎜ + 1×⎝) × 2 = **20** |
| 8 linhas | 2 + 34 = **36** | (1+16+1) × 2 = **36** |
| 12 linhas | 2 + 56 = **58** | (1+27+1) × 2 = **58** |

**Contagem de peças igual em todas as configurações.** Os dois usam a `GlyphAssembly` da
fonte; o que difere é só o `ToUnicode` do PDF — o vanilla mapeia a primeira peça a `(` e
deixa as outras sem mapeamento, o cristalino mapeia cada peça ao seu codepoint
`U+239B…U+23A0`. Mecânica de codificação do PDF, livre por ADR-0107. Não é diferença de
composição.

### 4. A varredura contínua não chegou a ser possível — e porquê

O plano da Opção C pedia altura de grelha contínua via `#box(height: …)` dentro da matriz,
para caçar a fronteira de variante. As três réguas contínuas disponíveis falham no
cristalino: `#box(height:)` e `#rect(height:)` não propagam dimensão para a medição do math,
`#text(size:)` embutido é ignorado, e `#set math.mat(row-gap:)` não é suportado (`warning:
set: target '' ainda não suportado`). Sem régua contínua, a varredura fica com a mesma
granularidade discreta da Fase A.

Isto deixou de importar para a decisão: com o mesmo alvo, a mesma tabela de variantes e o
mesmo `select_variant`, os dois escolhem a mesma variante em **toda** a gama, por construção
— não há fronteira onde divirjam. A tentativa produziu, em vez disso, dois achados novos
(abaixo).

---

## O que ficou escrito

Correcções (L0, fluxo contínuo ADR-0127 — correcção de registo de paridade, sem mudança de
contrato nem de comportamento):

1. **`math/layout/_comum.md` §P912-folga** — reescrita. A folga passa a estar registada como
   paridade, com o call site certo, a verificação de âmbito, a medição de output e as duas
   redacções anteriores marcadas como refutadas. **Item fechado.**
2. **`matrix.md` §P912** e **`cases.md` §P912** — as notas de correcção passam a dizer que a
   folga é do vanilla.
3. **`math/layout/tests.rs`** — comentário do teste-guarda
   `p945_grid_delim_target_du_e_altura_vezes_1_1` reforçado com o call site exacto e o
   registo de que dois passos o contradisseram sem o consultar. Só comentário.

Achados novos (registados como item aberto com dono, sem código):

4. **`math/layout/_comum.md` §P994 adenda 4** — o custo da deny-list de
   `needs_external_layout`, medido.
5. **`compiler/layout.md`** — `BoxedElem.width` não chega à largura desenhada, medido.

---

## Achado meta — a resposta certa já estava no repositório

O comentário do teste `p945_grid_delim_target_du_e_altura_vezes_1_1`
(`math/layout/tests.rs:4748`), escrito em P945, já dizia:

> o alvo do delimitador de matrizes/casos é `(ascent+descent) × 1.1` … **CONFIRMADO contra o
> vanilla** (`resolve.rs:1168-1186`) … Este teste trava qualquer "correcção" futura que troque
> as fórmulas.

O intervalo `1168-1186` contém a linha 1173. P1024 e P1026 Fase A concluíram o contrário,
cada um por sua via, e **nenhum consultou o teste que existia exactamente para os travar** —
apesar de o teste estar verde nos dois passos. O gate ADR-0127 fez o seu trabalho: a paragem
obrigatória impediu que uma "correcção" de uma linha quebrasse a paridade. Mas o freio que
falhou primeiro foi mais barato: ler a guarda antes de propor a mudança.

Consequência prática, sem regra nova: quando um passo propõe mudar uma fórmula, o `grep` do
nome da função nos testes vem **antes** da leitura do vanilla — a guarda, se existir, já traz
o call site medido.

---

## Achados novos, fora do eixo do gate

Encontrados ao tentar construir a régua contínua. Nenhum é tocado neste passo.

### A. `#text(size:)` embutido em math é ignorado (deny-list de P994)

`needs_external_layout` (`math/layout/mod.rs:249-262`) sobe para `layout_external` só
conteúdo que contém `Equation`/`Boxed`/`Align`/`Pad`/`Block`. `Styled(Text, …)` não está na
lista → fica no caminho de texto de math, que descarta o override de tamanho.

| documento | vanilla | cristalino |
|---|---|---|
| `#text(size: 40pt)[x]` **fora** de math | 18,24 × 17,28pt | 18,24 × 17,04pt ✔ |
| `$ #text(size: 40pt)[x] $` | 20,16 × 17,28pt | **5,52 × 4,80pt** ✘ |
| `$ mat(#text(size: 40pt)[x]) $` | 31,20 × 22,80pt | **14,40 × 10,80pt** ✘ |
| `$ lr(( #text(size: 40pt)[x] )) $` | 30,24 × 24,48pt | **12,24 × 10,80pt** ✘ |
| `$ cases(#text(size: 40pt)[x]) $` | 27,12 × 23,28pt | **11,28 × 10,80pt** ✘ |

Não é regressão: é o que a regra prescreve. Os testes-guarda de P994 passam porque usam
`#text(size: …)[$…$]` — com equação aninhada, que entra na allow-list. O critério original de
P994 ("tamanhos preservados em `text()`") deixou de valer para markup simples quando a adenda
2 estreitou a regra, e isso não estava registado. Registado em `_comum.md` §P994 adenda 4,
com as três perguntas que o passo dono tem de responder.

### B. `BoxedElem.width` não chega à largura desenhada — e é fora de math

| documento (11pt) | vanilla | cristalino |
|---|---|---|
| `#box(height: 40pt, width: 40pt, stroke: 1pt)` (vazio) | 40,80 × 40,80pt | **1,92** × 38,40pt |
| `#box(height: 40pt, width: 40pt, stroke: 1pt)[a]` | 40,80 × 40,80pt | **6,96** × 42,00pt |
| `#box(width: 40pt, stroke: 1pt)[a]` | 40,80 × 8,16pt | **6,96** × 14,64pt |
| `#box(height: 40pt, stroke: 1pt)[a]` (controlo, sem `width`) | 6,00 × 40,80pt | 6,96 × 42,00pt |

A `height` chega; a `width` não — 6,96pt é a largura do conteúdo mais inset. O controlo sem
`width` dá a mesma largura, ou seja o campo não participa. Reproduz-se em texto corrido, logo
não é `layout_external`. Registado em `compiler/layout.md`, com o ponto de partida
(`boxed.rs:133` e `:225`).

---

## Validação

```
crystalline-lint --fix-hashes .  → 17 ficheiros .rs resselados
crystalline-lint .               → 0 erros; 3 avisos V7 pré-existentes (prompts órfãos)
cargo test --workspace           → 5842 passed; 0 failed  (4971 + 789 + 41 + 2 + 37 + 2)
```

Igual ao baseline das Fases A/B — como esperado, já que a única alteração a ficheiro `.rs`
neste passo é um comentário de doc no teste-guarda (`math/layout/tests.rs`).

Os 17 ficheiros resselados são **todos** de uma linha (`@prompt-hash`), por edição de L0 e
não por mudança de lógica. São muitos porque `compiler/layout.md` é o L0 de todo o módulo
`compiler/layout/` (13 ficheiros: `cursor`, `dynamic`, `grid`, `grid_placement`, `helpers`,
`hyphenation`, `metrics`, `mod`, `placement`, `sequence`, `slicing`, `sub_frame`, `tests`) e
`math/layout/_comum.md` é o dos 4 de `math/layout/` (`cases`, `matrix`, `mod`, `tests`).
