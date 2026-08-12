# Passo 1006 — Relatório: `layout::metrics` não é o que o Passo 1003 mediu

**Resultado**: a **Fase A refuta a premissa do passo**. Não houve fatiamento. Nenhum
ficheiro de código foi alterado por este passo.
**Estado**: Fase A fechada; Fase B **não abre** sem gate ADR-0127 do dono (ver §7).

**Proveniência das medições** (regra de proveniência, CLAUDE.md): todos os números
abaixo foram medidos em `HEAD = 0f5575cd05087e7be69d8575f2ce57163461ede9` (Passo 1005),
em 2026-08-12 ~14:10 −03, com `git diff HEAD --stat` a mostrar exactamente
`00_nucleo/prompts/template-prompts.md | 6 ++++++` (alteração alheia, do template de
prompts) e os untracked habituais de documentação. O ficheiro analisado é
`01_core/src/compiler/layout/metrics.rs` (era `01_core/src/engine/layout/metrics.rs`
antes do Passo 1005; a análise histórica usa `git log --follow` sobre o caminho antigo,
que atravessa este rename e o `rules/`→`engine/` de `6636c5ea6`).

---

## 1. O que o passo assumia

O Passo 1006 herda a caracterização do Passo 1003, que classificou
`engine::layout::metrics` como:

> `| engine::layout::metrics | 68 | 73 | 3 | 16 | Hub de tipos de medida — quase todo `engine/` o toca. |`
> `| engine::layout::metrics | in 68 / out 3 | typst_library::layout::{length, abs, rel, em, axes, ratio} | … | Monolito cristalino vs família de tipos no vanilla. |`
> `| 5 | engine::layout::metrics | Fan-in 68 … | Corte relativamente mecânico: separar tipos de medida. |`

Três afirmações: (a) é um **monólito**; (b) contém **tipos de medida**; (c) o
equivalente vanilla é a família `layout::{length, abs, rel, em, axes, ratio}`, logo o
corte é "separar tipos de medida".

O próprio Passo 1006 já mandava desconfiar disto — *"confirmar, não presumir a partir do
nome"*. Confirmou-se, e as três afirmações estão erradas.

---

## 2. Critério 1 — inventário completo (incl. `pub(crate)`, lição do P1000)

```
grep -nE '^(pub |pub\(crate\) )?fn ' 01_core/src/compiler/layout/metrics.rs
grep -nE '^(pub |pub\(crate\) )?(struct|enum|trait|impl|type|const|static) ' …
```

443 linhas. O ficheiro inteiro contém:

| Item | Linhas | Natureza |
|------|--------|----------|
| `pub trait FontMetrics: Send + Sync` (:22–293) | 271 | **19 métodos**: 4 obrigatórios (`advance`, `vertical_metrics`, `cap_height`, `text_edges`) + 15 com corpo default |
| `pub fn needs_shaped_width(&str) -> bool` (:299) | 13 | a **única** free function do ficheiro |
| `pub struct FixedMetrics` + `impl` (:319–363) | 45 | unit struct; implementa os 4 obrigatórios |
| `impl FontMetrics for &dyn FontMetrics` (:369–412) | 44 | delegação para trait object (P858) |
| `mod smoke` (:414–443) | 30 | 4 testes, todos de `needs_shaped_width` |

Zero `pub(crate) fn` — a lacuna que escondeu 4 de 13 funções em `operators.rs` no P1000
foi procurada e não se repete aqui.

**Não existe um único tipo de medida neste ficheiro.** A afirmação (b) do P1003 é falsa
por inspecção directa.

### Onde vivem realmente os tipos de medida

| Vanilla | Cristalino |
|---------|-----------|
| `typst_library::layout::abs` / `length` / `ratio` | `01_core/src/entities/layout_types.rs` (`Pt:36`, `Length:926`, `Ratio:1010`) |
| `typst_library::layout::rel` | `01_core/src/entities/rel.rs` (`Rel<T>:18`) |
| `typst_library::layout::axes` | `01_core/src/entities/axes.rs` (`Axes<T>:18`) |
| `typst_library::layout::em` | **sem tipo dedicado** — absorvido por `Length` |

Tamanhos: vanilla `layout/{abs,em,length,ratio,rel,axes}.rs` = **1600 linhas**;
cristalino `entities/{layout_types,rel,axes}.rs` = **1834 linhas**. A família mapeia para
**`entities/` (L1 dados)**, não para `compiler/layout/metrics.rs`. A correspondência do
P1003 foi feita por colisão do nome ("metrics" ≈ "medida"), sem ler o conteúdo dos dois
lados.

---

## 3. Critério 4 — o equivalente vanilla real

O correspondente de `metrics.rs` no vanilla é
`lab/typst-original/crates/typst-library/src/text/font/metrics.rs`:

- **430 linhas** — praticamente o mesmo tamanho que as nossas 443.
- Vive na família **`text/font/`**, não em `layout/`.
- **O vanilla também não o fatia.** A família `text/font/` tem 9 ficheiros
  (`book, color, exceptions, info, metrics, mod, tag, variant, variations`) e `metrics.rs`
  é um deles, inteiro.

Diferença de forma, deliberada e explicável: no vanilla `FontMetrics` é uma **struct de
dados** (`units_per_em`, `ascender`, `cap_height`, `x_height`, `descender`,
`strikethrough`, `underline`, `overline`, `subscript`, `superscript`, `math`), lida
directamente de `ttf-parser`. No cristalino é um **trait** — porque L1 não pode tocar em
I/O nem em `ttf-parser`, e a leitura de fontes vive em L3
(`03_infra/src/font_metrics.rs`). É divergência de **mecânica**, não de língua
(ADR-0107): não há construção da linguagem Typst por trás dela, só a topologia de
camadas.

**Conclusão do critério 4: não há divisão vanilla a copiar.** O candidato de 6 nós que o
passo trazia como ponto de partida aponta para o ficheiro errado.

---

## 4. Porque é que o fan-in é 68 — a resposta que o passo pedia explicitamente

O P1003 mediu fan-in ao nível de **símbolo**, não de módulo. Ao nível de módulo,
`metrics` quase não é importado:

- `01_core/src/compiler/layout/mod.rs:34` declara **`mod metrics;` — privado**.
- Linha `:35`: `pub use crate::compiler::layout::metrics::{needs_shaped_width, FixedMetrics, FontMetrics};`
- Só **9 ficheiros** referenciam o caminho do módulo (`super::metrics` ×8 + 1 caminho
  completo): `cursor, dynamic, equation, footnote_flush, grid, mod, placement, sequence,
  set_page`.

O que tem fan-in alto é o **símbolo** `FontMetrics`, através do re-export:

| Âmbito | Ficheiros que referenciam `FontMetrics`/`FixedMetrics`/`needs_shaped_width` |
|--------|---:|
| `01_core/src/compiler` | 76 |
| `01_core/src/entities` | 3 |
| `03_infra` | 6 |
| `02_shell`, `04_wiring` | 0 |

Os 76 de `compiler/` são, na esmagadora maioria, o **bound genérico `M: FontMetrics`**
nas free functions de layout — exactamente a forma B da ADR-0109, em que cada
`compiler/layout/<elem>.rs` recebe o `Layouter<M>`.

**O fan-in 68 é a assinatura de um _port_, não de um monólito.** Um port com muitos
consumidores é o comportamento normal e desejado de uma interface; não é sintoma de
acumulação. Este é o eixo que o passo pedia para verificar ("por tipo de medida" vs "por
quem consome") — a resposta é: **nenhum dos dois**, porque não há aglomerado de
responsabilidades para separar.

---

## 5. Critérios 1 e 2 — vácuos pela segunda vez

**Critério 2 (pureza vs estado):** `grep -c 'EvalContext\|Engine<'` no ficheiro → **0**.
Os 19 métodos são todos `(&self, medida…) -> medida`. `needs_shaped_width` é
`&str -> bool`. Nenhum toca `EvalContext`, `Engine<'a>`, scope ou estado.

O Passo 1006 avisava para *"não presumir que é tudo declarativo como `operators.rs`"*.
Mediu-se: **é**. O critério não discrimina.

**Critério 1 (isolamento de teste):** os 4 testes locais cobrem só
`needs_shaped_width`; o próprio ficheiro declara em `:421` *"A cobertura funcional vive
em `layout/tests.rs`"*. Não há partição de testes que sugira fronteira.

---

## 6. Critério 3 — co-mudança histórica (o critério que devia dominar)

Método: para cada um dos 51 commits que tocaram o ficheiro (`git log --follow`, a
atravessar `rules/`→`engine/`), mapeou-se cada linha adicionada ao item que a contém na
pós-imagem desse commit. Script guardado em `tools/analysis/cochange_metrics.py`
(reprodutível a partir de `0f5575cd0`: `python3 tools/analysis/cochange_metrics.py`).

### 6a. Primeiro achado: 30 dos 51 commits não tocaram em lógica nenhuma

**30 de 51 commits (59%)** alteraram exclusivamente o cabeçalho de linhagem
(`@prompt-hash` / `@updated`). O ficheiro partilha o L0 `compiler/layout.md` com **13
ficheiros `.rs`**; sempre que esse L0 é resselado, os 13 mudam. A maior parte da
"turbulência" de `metrics.rs` no histórico é resselo de hash, não evolução do módulo.

### 6b. Os clusters reais

Nos 21 commits com mudança de corpo, os itens agrupam-se assim:

| Cluster | Métodos | Commits que os movem juntos |
|---------|---------|------------------------------|
| **A — largura / avanço de texto** | `advance`, `advance_shaped`, `text_width`, `line_content_right`, `needs_shaped_width` | P544, P591, P593, P623 |
| **B — métricas verticais / edges** | `vertical_metrics`, `cap_height`, `text_edges`, `text_ink_bounds`, `text_ink_bounds_signed` | P750, P752, P762, P813-827, P858-860 |
| **C — glifos / tabela OpenType MATH** | `math_constants`, `math_kern`, `vertical_glyph_variants`, `vertical_glyph_assembly`, `horizontal_glyph_variants`, `horizontal_glyph_assembly`, `glyph_to_char`, `glyph_ink_bounds`, `italics_correction`, `top_accent_attach` | P906, P917, P952, P970/971, P985, P988-B |

Travessias entre clusters (o que enfraquece a fronteira): P544 (`advance` + `math_kern`,
A×C), P591 (`advance_shaped` + `math_kern`, A×C), P760 (`advance` + `vertical_metrics`,
A×B), P906 (`text_edges` + 6 métodos de glifo, B×C), P917 (`math_constants` +
`text_ink_bounds`, C×B), P858-860 (4 métodos de A e B ao mesmo tempo — é o
`impl for &dyn`). Os commits `b0e081e64` (criação, P96.7) e `6636c5ea6` (rename
`rules/`→`engine/`) tocam tudo e são ruído.

**Veredicto do critério 3: os clusters existem e são legíveis, mas não são disjuntos.**
`text_ink_bounds`/`text_ink_bounds_signed` oscilam entre B e C, e o eixo texto↔math
cruza-se em 5 commits.

### 6c. E, sobretudo: os clusters não são fatiáveis

Os métodos de A, B e C são **métodos do mesmo trait Rust**. Rust não permite distribuir
os métodos de um `trait` por vários ficheiros. Fatiar por co-mudança exigiria partir
`FontMetrics` em supertraits (`FontMetrics: TextAdvance + VerticalMetrics + MathGlyph`).
Ver §7 — é mudança de contrato público.

---

## 7. Onde isto encalha: ADR-0127

Qualquer corte de `metrics.rs` pelas fronteiras que o critério 3 sugere implica
**redefinir o trait `FontMetrics`**, e ADR-0127 lista explicitamente *"método em trait"*
na categoria (1) — contrato público → **paragem obrigatória para confirmação do dono**.
Não é fluxo contínuo.

Raio de impacto medido, para o dono avaliar:

| Superfície | Contagem | Onde |
|------------|---------:|------|
| Implementadores de `FontMetrics` | 13 | 2 reais em L3 (`FontBookMetrics`, `FallbackFontMetrics` em `03_infra/src/font_metrics.rs`), 1 em L1 (`FixedMetrics`), 1 blanket (`&dyn FontMetrics`), 9 stubs de teste em `compiler/{math/layout,layout}/tests.rs` |
| Ficheiros com bound `M: FontMetrics` ou uso do símbolo | 76 em `compiler/`, 3 em `entities/`, 6 em `03_infra` | — |
| L0 a carvar | 1 → N | `metrics.rs` não tem L0 próprio; partilha `compiler/layout.md` com 13 `.rs`. V15 exige `@prompt` por ficheiro, logo cada nó exigiria um L0 novo |

---

## 8. E se o gate for dado: o que sobra para mover

Mesmo com autorização, o ganho é pequeno. Medindo linhas de **código** (excluindo
doc-comments, comentários e linhas vazias):

```
total 443 | doc-comments 175 (39,5%) | comentários 17 | vazias 40 | código 211
```

Corpo real por item, os únicos acima de 8 linhas:

| Item | Linhas de código |
|------|---:|
| `line_content_right` | 36 |
| `FixedMetrics::text_edges` | 26 |
| `text_width` | 15 |
| `needs_shaped_width` | 13 |
| *todos os outros 20 itens* | ≤ 8 cada, a maioria 4 (`let _ = …; Default::default()`) |

**90 das 211 linhas de código estão em 4 itens; as restantes 121 são assinaturas, stubs
de default e delegação.** E 175 linhas — 40% do ficheiro — são documentação: cada método
traz a justificação do passo que o introduziu (P544, P591, P750, P891, P906, P971,
P985, P988-B…).

`metrics.rs` não é um monólito de lógica. É **uma interface documentada**. Fatiá-la pelo
critério 3 dispersaria 90 linhas de lógica por 3 ficheiros e deixaria o hub em ~350
linhas de assinatura e prosa — que continuaria a ter fan-in 68, porque o fan-in vem do
trait, não da lógica.

---

## 9. Fase D — avaliação do método (exigência do P1002)

| Critério | P1002 (`operators.rs`) | P1006 (`metrics.rs`) |
|----------|------------------------|----------------------|
| 1 — isolamento de teste | vácuo | **vácuo** (testes vivem em `layout/tests.rs`) |
| 2 — pureza vs estado | vácuo | **vácuo** (0 ocorrências de `EvalContext`/`Engine<'a>`) |
| 3 — co-mudança histórica | o mais informativo; corrigiu a divisão por inspecção | **informativo, mas inaplicável** — os itens são métodos de um trait |
| 4 — vanilla | útil | **enganador** — o mapeamento do P1003 apontava para o ficheiro errado |

### Lacunas novas do método (equivalentes à do `pub(crate)` no P1000)

1. **Falta um critério-zero: "isto é um agregado ou uma interface?"** Os 4 critérios
   assumem que o alvo é um conjunto de unidades independentes. Aplicados a um `trait`,
   produzem clusters bonitos e inúteis, porque a linguagem não permite o corte que
   sugerem. Este teste tem de vir **antes** dos outros quatro.
2. **O fan-in do DSM é ao nível de símbolo e o candidato a corte é ao nível de módulo.**
   Os 68 do P1003 e os 9 imports de módulo medem coisas diferentes. Usar o primeiro para
   priorizar fatiamento sobrestima alvos que são ports.
3. **O ruído de resselo de L0 contamina o critério 3.** 59% dos commits deste ficheiro
   são só mudança de `@prompt-hash`. Qualquer futura análise de co-mudança tem de
   descontar linhas de cabeçalho de linhagem antes de contar (foi feito aqui; deve ser
   regra).
4. **O mapeamento vanilla do P1003 precisa de verificação de conteúdo, não de nome.**
   Este caso mostra que um par cristalino↔vanilla pode estar errado e ainda assim
   parecer plausível na tabela.

---

## 10. Recomendação

**Não fatiar `compiler::layout::metrics`.** As três razões, por ordem de força:

1. Não há monólito: 90 linhas de lógica em 4 itens, 175 de documentação, 19 métodos de
   um trait.
2. O corte que o critério 3 sugere é uma mudança de contrato público (ADR-0127 §1) com
   13 implementadores e 85 ficheiros consumidores, para mover 90 linhas.
3. O vanilla — que é a referência invocada para justificar o corte — **também não fatia**
   o seu `text/font/metrics.rs` (430 linhas).

**Correcção a propagar**: a linha do Passo 1003 que mapeia `engine::layout::metrics` →
`typst_library::layout::{length, abs, rel, em, axes, ratio}` está errada. O mapeamento
correcto é:

- `compiler::layout::metrics` → `typst-library/src/text/font/metrics.rs` (+ a
  implementação em `03_infra/src/font_metrics.rs`, que é o lado que no vanilla é acesso
  directo a `ttf-parser`);
- `typst_library::layout::{length, abs, rel, em, axes, ratio}` →
  `entities/{layout_types.rs, rel.rs, axes.rs}`.

O relatório do P1003 fica **inalterado** (registo histórico do que foi medido nessa
altura); esta correcção vive aqui e deve ser lida com ele.

**Antes de aplicar o método a `eval.md`/`bindings.rs`** (os alvos maiores que o P1006
antecipava), correr o critério-zero de §9.1: `bindings.rs` é um conjunto de funções ou
uma interface? Se for o primeiro, o método do P1002 aplica-se; se for o segundo, o P1006
repete-se.

---

## Ficheiros alterados por este passo

Nenhum, além deste relatório. `cargo test --workspace` e `crystalline-lint .` não foram
re-corridos porque nada mudou — o estado de validação em vigor é o do Passo 1005
(`0f5575cd0`): 5828 passed / 0 failed / 3 ignored; lint com 2 V7 pré-existentes.
