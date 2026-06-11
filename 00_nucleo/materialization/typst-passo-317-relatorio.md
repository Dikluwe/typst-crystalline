# Relatório P317 — Zera V9 + Lote 2 (família math) Fase A + Modelo de Lote

**Estado**: Pre-1 ✅ · Pre-2 ✅ · Modelo ✅ · Lote 2 (11 variantes) **Fase A
(L0) + checkpoint + Fase B (implementação) ✅ completos**. `crystalline-lint .`
→ **0 violations**; `RUST_MIN_STACK=33554432 cargo test --workspace` →
**2506 passed, 0 failed** (typst-core 2473 → 2506, +33 = só os testes unitários
novos; nenhuma asserção existente alterada).

---

## Pre-1 — Zerar as 3 V9 ✅

**Diagnóstico**: as 3 V9 (`export/tests.rs:16`, `font_metrics.rs:10`,
`layout.rs:7`) vinham **todas** da forma agrupada `use typst_core::{entities::…,
rules::…}`. A forma de path único (`use typst_core::entities::content::Content;`)
**não** dispara V9 — prova: `measurements.rs`, `pipeline.rs`, `query_helpers.rs`
usam-na sem flag. Logo os símbolos já eram alcançáveis pelos **mesmos paths** via
portas `[l1_ports]` `entities`/`rules`.

**Conserto**: desmembrar a forma agrupada em `use` de path único (estilo
prevalecente em L3). **Mudança local em L3, sem L0 novo** — confirmado que os L0
de `03_infra` (`infra/layout.md`, `infra/font_metrics.md`, `infra/export/tests.md`)
**não especificam paths de import**. Nenhum símbolo precisou de nova exposição →
superfície de L1 intacta → Trava não acionada.

**Validação**: `crystalline-lint .` → **0 violations** (0 V7, 0 V9, 0 drift).
`RUST_MIN_STACK=33554432 cargo test --workspace` → **2992 passed, 0 failed**
(2473+472+24+2+21). Ressalva conhecida: stack default estoura em
`recursao_infinita_*` — **não é regressão** (correr com `RUST_MIN_STACK`).

**Commit**: `74d9e0b62 Passo 317 — zera V9`. A partir daqui "zero violations"
vale sem asterisco; qualquer violação futura é **regressão**, não herança.

---

## Pre-2 — Tabela de largura de uso (dimensionador dos lotes)

Largura = nº de sites `Content::Nome` (construção/match) fora de `content.rs`,
nas camadas cristalinas. Comando:

```sh
grep -rnE "Content::Nome([^A-Za-z0-9]|$)" 01_core 02_shell 03_infra 04_wiring \
  --include='*.rs' | grep -v "entities/content.rs" | wc -l
```

**77 variantes; 3 já em D** (Heading, MathStyled, Divider) → **74 restantes.**
Tabela completa (largura crescente; **M** = família math do Lote 2):

| # | largura | variante | M | # | largura | variante | M |
|--:|--:|---|:--:|--:|--:|---|:--:|
| 1 | 4 | EnumItem | | 38 | 18 | Repeat | |
| 2 | 4 | Link | | 39 | 19 | CounterDisplayCallback | |
| 3 | 5 | SetFigureNumbering | | 40 | 20 | Quote | |
| 4 | 6 | ListItem | | 41 | 21 | MathSequence | ● |
| 5 | 6 | MathCases | ● | 42 | 21 | State | |
| 6 | 6 | TermItem | | 43 | 21 | StateDisplay | |
| 7 | 7 | GridFooter | | 44 | 22 | Columns | |
| 8 | 7 | GridHeader | | 45 | 22 | Pagebreak | |
| 9 | 7 | MathMatrix | ● | 46 | 23 | MathFrac | ● |
| 10 | 8 | SetPage | | 47 | 23 | Ref | |
| 11 | 9 | Raw | | 48 | 24 | Outline | |
| 12 | 10 | MathAlignPoint | ● | 49 | 25 | StateUpdate | |
| 13 | 10 | Overline | | 50 | 28 | MathAttach | ● |
| 14 | 10 | Strike | | 51 | 28 | SmartQuote | |
| 15 | 10 | TableFooter | | 52 | 28 | Text | |
| 16 | 11 | Align | | 53 | 30 | Stack | |
| 17 | 11 | Divider ✓D | | 54 | 31 | Underline | |
| 18 | 11 | Linebreak | | 55 | 32 | Cite | |
| 19 | 11 | TableHeader | | 56 | 32 | TableCell | |
| 20 | 11 | Terms | | 57 | 32 | Transform | |
| 21 | 12 | Colbreak | | 58 | 34 | Place | |
| 22 | 12 | MathAccent | ● | 59 | 38 | MathOp | ● |
| 23 | 13 | Space | | 60 | 39 | CounterUpdate | |
| 24 | 14 | MathCancel | ● | 61 | 39 | Pad | |
| 25 | 14 | VSpace | | 62 | 40 | Bibliography | |
| 26 | 15 | CounterDisplay | | 63 | 41 | Table | |
| 27 | 15 | MathDelimited | ● | 64 | 45 | Equation | |
| 28 | 16 | Metadata | | 65 | 46 | Footnote | |
| 29 | 16 | SetEquationNumbering | | 66 | 47 | GridCell | |
| 30 | 17 | Heading ✓D | | 67 | 50 | MathText | ● |
| 31 | 17 | Image | | 68 | 55 | Labelled | |
| 32 | 17 | MathStyled ✓D | | 69 | 57 | Shape | |
| 33 | 18 | Hide | | 70 | 58 | Boxed | |
| 34 | 18 | HSpace | | 71 | 58 | Styled | |
| 35 | 18 | MathRoot | ● | 72 | 62 | SetHeadingNumbering | |
| 36 | 18 | MathUnderover | ● | 73 | 73 | Grid | |
| 37 | — | — | | 74 | 89 | Figure | |
| | | | | 75 | 101 | Block | |
| | | | | 76 | 108 | MathIdent | ● |
| | | | | 77 | 116 | Empty | |
| | | | | 78 | 208 | Sequence | |

(✓D = já migrada modelo D; ● = família math, Lote 2.)

**Achado operacionalizado** (custo ∝ largura, mensurável antes): as variantes
mais largas são **primitivos de AST** — `Sequence` (208), `Empty` (116),
`MathIdent` (108), `Block` (101), `Figure` (89) — não element-shaped no sentido
do trait. Registrado como **critério de elegibilidade** no modelo (guideline,
com override do dono por lote).

---

## Modelo de Lote ✅ gravado

`00_nucleo/modelo-lote-migracao-d.md` — receita reutilizável (parâmetros
`N_LOTE`/`LOTE`/`NOTAS_FAMÍLIA`; Fase A→checkpoint→Fase B; validação/medições
fixas; critério de elegibilidade). Instanciável pelos lotes 3+ sem retrabalho.

---

## Lote 2 — família math (Fase B em execução)

**Composição (decisão do dono no checkpoint, ajustada): 11 `Math*`
element-shaped**, ordem de migração por largura crescente:

`MathCases`(6) · `MathMatrix`(7) · `MathAlignPoint`(10) · `MathAccent`(12) ·
`MathCancel`(14) · `MathDelimited`(15) · `MathRoot`(18) · `MathUnderover`(18) ·
`MathFrac`(23) · `MathAttach`(28) · `MathOp`(38).

> **Ajuste de composição no checkpoint**: a proposta inicial (14 `Math*`, por
> "override" pré-tabela) **caiu**. Excluídos `MathSequence`(21), `MathText`(50),
> `MathIdent`(108) — **primitivos de AST** que a própria guideline de
> elegibilidade do P317 exclui (risco de alocação `Arc` em folha quente,
> ADR-0029/0030). Os 3 L0 já redigidos ficam **guardados, não materializados**,
> marcados ⏸️ "aguardando decisão de primitivos de AST".

### Decisão futura própria — primitivos de AST

Grupo `{MathSequence, MathText, MathIdent}` + `{Sequence, Empty, Block}` (os
contentores/folhas de maior largura do enum). Opções a decidir em passo próprio:
**(a)** migrar para modelo D **com medição de performance** que justifique o
`Arc` em folha quente; **(b)** manter inline **por design**, formalizado via
**nota na ADR-0105** (folhas/contentores ficam fora do D); **(c)** forma
terceira (ex.: wrapper sem `Arc`, ou `Box` em vez de `Arc`). Pré-requisito de
(a): benchmark de `eval()`/`map_*` no hot path math antes/depois.

**Não-locatável (confirmado)**: zero refs `Math*` em `ElementKind`/introspecção
→ `element_kind`/`to_payload` ficam no default `None` (nenhuma segue o caminho
Heading). **Math é terminal em `map_text`** (structural; não desce) e recurse em
`map_content` (contentores) — folhas clonam.

**L0 redigidos (Fase A)** — 14 prompts finos + atualização do hub:

- `entities/elements/math_{cases,matrix,align_point,accent,cancel,delimited,`
  `root,underover,sequence,frac,attach,op,text,ident}.md` (1 por variante,
  content-preserving; tabela `impl Element` com referência de linha ao braço
  atual de `content.rs`).
- `entities/content.md` (+ secção "Lote 2 P317": tabela variante→módulo→forma→
  shape; convenção desboxar `Box<Content>`→`Content`; construtores ergonómicos).
- `_comum.md` **não tocado** (o trait não muda em lote — regra permanente).

**Plano de toque (sites a editar em Fase B, fora de `content.rs`)** — por
variante (ficheiro × ocorrências), totais = largura da tabela:

| variante | total | concentração principal |
|---|--:|---|
| MathCases | 6 | introspect(2), eval/math, math/layout, layout, locatable |
| MathMatrix | 7 | eval/math(2), introspect(2), math/layout, layout, locatable |
| MathAlignPoint | 10 | math/layout(3), introspect(2), eval/math(2), locatable(2), layout |
| MathAccent | 12 | stdlib/mod(4), stdlib/structural(2), introspect(2), …, export/tests |
| MathCancel | 14 | stdlib/mod(6), introspect(2), stdlib/structural(2), …, export/tests |
| MathDelimited | 15 | math/layout/tests(6), math/layout(2), eval/math(2), introspect(2), … |
| MathRoot | 18 | math/layout/tests(6), eval/math(4), math/layout(2), introspect(2), … |
| MathUnderover | 18 | stdlib/mod(9), stdlib/structural(2), introspect(2), export/tests(2), … |
| MathSequence | 21 | eval/math(6), eval/tests(5), math/layout(4), math/layout/tests(2), … |
| MathFrac | 23 | math/layout/tests(9), math/layout(4), introspect(2), export/tests(2), … |
| MathAttach | 28 | math/layout/tests(17), math/layout(2), eval/tests(2), export/tests(2), … |
| MathOp | 38 | stdlib/mod(16), math/layout(5), stdlib/structural(4), eval/math(3), … |
| MathText | 50 | math/layout/tests(23), eval/math(7), export/tests(7), math/layout(3), … |
| MathIdent | 108 | math/layout/tests(56), stdlib/mod(23), math/layout(11), export/tests(7), … |

> A maioria dos sites de alta contagem está em **`*/tests.rs`** (construção de
> fixtures): a *sintaxe* de construção muda (`Content::MathIdent("x".into())` →
> `Content::math_ident("x")`); **nenhuma asserção existente é alterada** (regra
> do modelo). Os sites em `introspect.rs`/`locatable.rs` são braços de match
> exaustivo (não-locatável) que passam de `Content::Math…{..}` a
> `Content::Math…(_)`.

> **Plano de toque vs realidade (Fase B)**: as 11 migradas tocaram exatamente
> os ficheiros previstos. As construções `Content::MathX { … }` viraram os
> construtores `Content::math_x(…)` (57 transformadas por script balanceado nos
> ficheiros de teste + ~20 em código lib editadas à mão); os padrões de match
> `{ … }` viraram `MathX(e)` com acesso `e.campo` (e `.as_deref()` → `.as_ref()`
> nos campos `Option`, agora `Option<Content>`). **Nenhuma asserção alterada.**

### Fase B — medições finais (métrica ADR-0104)

| medição | valor | comando |
|---|---|---|
| `content.rs` antes/depois | **5785 → 5735 (−50)** | `wc -l`; diff `+158 / −208` |
| parte atómica (11 módulos novos, c/ testes) | **954 linhas** | `wc -l elements/math_*.rs` |
| suíte typst-core | **2473 → 2506 (+33)** | só os 33 testes unitários novos (3×11) |
| `crystalline-lint .` | **0 violations** | gate binário restaurado |

Custo-por-módulo (linhas, com testes): MathAttach 103 · MathUnderover 97 ·
MathMatrix 94 · MathCases 90 · MathDelimited 89 · MathRoot 88 · MathOp 86 ·
MathAccent 83 · MathCancel/MathFrac 81 · MathAlignPoint 62. **Preditor validado**:
o custo dominou ∝ largura de uso (os sites externos), não ∝ tamanho do módulo
(homogéneo ~80–100 linhas) — confirma o achado P316.

**Hub encolheu** (−50 linhas) apesar de +11 variantes migradas: a lógica
por-variante saiu dos 6 matches para os módulos; o dispatch de 1 linha
compensa-os com folga. `map_text` manteve os math no bloco terminal (paridade
`MathStyled`); `map_content`/`plain_text`/`eq` viraram dispatch.

**Caveat conhecida**: stack default estoura em `recursao_infinita_*` — **não é
regressão** (correr com `RUST_MIN_STACK=33554432`).

---

## Fora de escopo (confirmado)

Lotes 3+ (instanciados pelo modelo); F/PropMap/StyleChain (DEBT 99.E); DEBT-57;
fatiar `rules/{eval,parse,layout}.md` (não mordido neste lote); mudanças no
trait `Element`.

## Proposta do Lote 3 (derivada da tabela — decisão humana)

Esgotada a família math, os próximos baratos element-shaped sugerem-se por
largura crescente, p.ex. um lote de **itens de lista/termos** (`EnumItem` 4,
`ListItem` 6, `TermItem` 6, `Terms` 11) ou **breaks/espaços**
(`Linebreak` 11, `Colbreak` 12, `Space` 13, `VSpace` 14, `HSpace` 18). Decisão
do dono no fecho do Lote 2.
