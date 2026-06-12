# Medição pré-F (P318 Parte 2) — números e comandos, sem recomendação

> **⚠️ Baseline de performance SUPERSEDED (P330).** O baseline M3 abaixo está
> **quantizado** (~15%, caveat C1/P319). O baseline de referência do F é agora
> **`medicao-pre-f-passo-330.md`** — tirado com o hub no estado final pós-lotes
> (Block/L15), corpus 10× (σ útil). Use **esse** como "antes" do F. As outras
> medições deste documento (M1/M2/M4 — larguras, superfície) continuam válidas.

**Propósito**: alimentar o diagnóstico de decisão do candidato F (StyleChain
real / PropMap, DEBT 99.E `debt-stylechain-nao-materializada.md`). **Este
documento mede; não recomenda nem decide.** Quatro medições (M1–M4).

**Data**: 2026-06-11 · **Máquina**: AMD Ryzen 7 5800H, 16 cores.
**Imprecisão geral**: contagens por `grep`/`awk` (sintáticas), não por parse
semântico — cada secção regista o seu método e limite.

---

## M1 — Tamanho da superfície de propriedades (teto vs piso)

### Teto — vanilla (`lab/typst-original`, leitura autorizada Parte 2)

```sh
grep -rE "^\s*#\[elem"    lab/typst-original/crates --include='*.rs' | wc -l   # 163
grep -rE "^\s*#\[default" lab/typst-original/crates --include='*.rs' | wc -l   # 283
```

- **163** structs `#[elem]`.
- **283** campos `#[default(...)]` (settable com default) — proxy da superfície
  que a StyleChain de referência resolve.
- Outros atributos de campo (amostra): `#[required]` 128, `#[ghost]` 75,
  `#[fold]` 59, `#[positional]` 25, `#[synthesized]` 20, `#[resolve]`/`#[borrowed]` 0.
- **Método/imprecisão**: conta atributos do proc-macro `#[elem]`; **não**
  distingue settable de internal sem parse semântico. `283 (#[default])` é o
  piso defensável da superfície settable; o total de campos é maior.

### Piso — cristalino hoje

```sh
# Style enum (entities/style.rs) e StyleDelta (entities/style_chain.rs)
awk '/pub enum Style/{f=1}f{print}f&&/^}/{exit}' 01_core/src/entities/style.rs | grep -cE "^    [A-Z]"     # 10
awk '/pub struct StyleDelta/{f=1}f{print}f&&/^}/{exit}' 01_core/src/entities/style_chain.rs | grep -cE "Option<"  # 10
```

- **`Style`**: 10 variantes hardcoded — `Bold, Italic, Size, Fill, HeadingLevel,
  Lang, Weight, Tracking, Leading, Font` (confirma 313 §4).
- **`StyleDelta`**: 10 campos `Option<T>` (os mesmos 10).
- **Variantes `Set*` do `Content`** (largura de uso, tabela P317; comando
  `grep -rnE "Content::<V>([^A-Za-z0-9]|$)" 01_core 02_shell 03_infra 04_wiring --include='*.rs' | grep -v entities/content.rs | wc -l`):
  `SetHeadingNumbering` **62** · `SetEquationNumbering` **16** · `SetPage` **8** ·
  `SetFigureNumbering` **5** (= 4 variantes, 91 sites).
- **`Content::Styled`** (wrapper de styling): **58** sites — **57 em L1**, **1 em
  L3**, 0 em L2/L4.

### Delta teto − piso

**≈ 283 − 10 = ~273** propriedades settable que o F/99.E teria de carregar numa
PropMap genérica `(elemento+campo → valor resolvível)`, contra as 10 reificadas
à mão hoje. Esse é o tamanho honesto da obra de propriedades.

---

## M2 — Sites que o F converteria (custo da trava de verificação)

### Leitura de propriedade de estilo (produção, fora de `*tests.rs`)

```sh
prod=$(find 01_core/src/rules -name '*.rs' ! -name 'tests.rs' ! -name '*_tests.rs')
echo "$prod" | xargs grep -hoE "Style::[A-Z][A-Za-z]*" | wc -l   # 5
echo "$prod" | xargs grep -hcE "StyleDelta" | awk '{s+=$1}END{print s}' # 10
```

- `Style::<Variant>` reads em produção: **5** · `StyleDelta` refs: **10**.
- Superfície de leitura de propriedade **pequena** — o sistema de estilo é jovem
  (a maioria dos `Style::`/`StyleDelta` está em fixtures de teste, não produção).

### Exaustividade de elemento que o F perde (a dimensão da trava)

```sh
pat="Content::(Heading|Divider|MathStyled|MathFrac|MathAttach|MathRoot|MathDelimited|MathMatrix|MathCases|MathAccent|MathCancel|MathUnderover|MathOp|MathAlignPoint)\("
echo "$prod" | xargs grep -cE "$pat" | awk -F: '{s+=$2}END{print s}'   # 104
```

- **104** sites de dispatch/match exaustivo dos 14 elementos migrados (D), em
  produção: `introspect.rs` 29 · `math/layout/mod.rs` 22 · `stdlib/mod.rs` 21 ·
  `layout/mod.rs` 14 · `introspect/locatable.rs` 14 · outros 4.
- Estes são a **exaustividade que o compilador garante hoje** (match sem `_`).
  O F (lookup em PropMap) **perde** essa garantia → este número dimensiona a
  **regra de lint / teste-varre-tabela** que a trava da ADR-0105 exige antes do F.
- **52** variantes `Content` ainda em struct-form `{ … }` (campos inline lidos
  direto) — alvos de D nos lotes seguintes, antes do F
  (`grep -cE "^\s+[A-Z][A-Za-z]+ \{" 01_core/src/entities/content.rs`).

---

## M3 — Baseline de performance do caminho quente (o "antes" do F)

- **Corpus fixo** (guardado para o "depois" ser comparável):
  `00_nucleo/diagnosticos/medicao-pre-f-passo-318-corpus.typ` — **7003 linhas**,
  500 secções (texto + `*negrito*`/`_itálico_` + listas `-`/`+` + termos `/` +
  math inline e display: `frac`/`root`/`mat`/`attach`/`op`).
- **Binário**: `target/release/typst` (release, `cargo build --release -p typst-wiring`).
- **Comando** (após warmup):
  ```sh
  /usr/bin/time -f "%e" ./target/release/typst \
    00_nucleo/diagnosticos/medicao-pre-f-passo-318-corpus.typ -o /tmp/out.pdf
  ```
- **5 execuções (s)**: `0.07 0.07 0.07 0.07 0.07` → **mediana 0.07 s (~70 ms)**.
- PDF de saída ~170 KB.
- **Imprecisão**: o binário mede o pipeline **completo** eval→layout→**export PDF**
  (não só eval+layout). É o caminho quente honesto que o F afeta (leituras de
  propriedade em eval/layout). O "depois" do F mede-se com **o mesmo corpus e
  comando**. **Nada foi otimizado** (medir ≠ mexer).
- **Caveat de resolução (P319 C1)**: as 5 execuções deram **exatamente** 0.07 s —
  a granularidade de 0.01 s do `/usr/bin/time` ⇒ quantização **~15%**. Este
  baseline só deteta **regressões grosseiras**. **A medição do "depois" do F
  deve refazer o "antes" e o "depois" no par de commits**, com `hyperfine`
  (média ± σ) ou corpus ~10× maior, **mesmo corpus e comando**. (O hash de
  linhagem não se aplica: diagnósticos não são L0.)

---

## M4 — Censo da necessidade de scoping (o valor do F)

**Features que hoje divergem do vanilla por falta de scoping léxico** (fonte:
`00_nucleo/debt-stylechain-nao-materializada.md`):

1. **`numbering_active` (heading + equation)** — `HashMap` global em
   `CounterStateLegacy` → `StateRegistry` (P171/P182). Vanilla resolve via
   `Option<Numbering>` em `HeadingElem`/`EquationElem` + StyleChain.
2. **Set rules de styling** (`Bold`/`Italic`/…) — cristalino exige **wrapping
   manual** (`Content::Styled`) onde vanilla permite set global (ADR-0026/0038).
3. **Marcadores `Set*`** (`SetHeadingNumbering`/`SetEquationNumbering`/
   `SetFigureNumbering`/`SetPage`) — set-rule global a partir do ponto onde
   aparece; **não respeita escopo léxico de container**.

**Censo de testes que fixam o comportamento global atual** (método: `awk` conta
funções `#[test]` cujo corpo referencia o termo; pode haver sobreposição entre
categorias; imprecisão registada):

| comportamento divergente | testes `#[test]` | refs totais |
|---|--:|--:|
| `numbering_active` (numbering global) | **38** | 166 |
| `Set*` markers (set-rule global) | **41** | 35 em tests + 91 sites prod |
| `Content::Styled` (wrapping manual de styling) | **21** | 58 sites |

Estes testes fixam o comportamento **global** atual: são o custo de migração
**comportamental** que o F/99.E paga ao introduzir scoping léxico (cada um pode
precisar de revisão quando o set rule passar a respeitar container). **Sem juízo
de valor — contagem e lista.**

---

## Síntese (4 números, sem recomendação)

- **M1**: superfície settable **teto ~283 / piso 10** (delta ~273).
- **M2**: **104** sites de exaustividade a cobrir pela trava; superfície de
  leitura de propriedade em produção pequena (5+10).
- **M3**: baseline **0.07 s** (mediana, 5 runs) no corpus de 7003 linhas.
- **M4**: **~100** testes fixam o comportamento global (38 numbering + 41 Set\*
  + 21 Styled, com sobreposição).

A decisão do F (migrar/forma/sequência) é de passo futuro junto do DEBT 99.E,
que consome este documento.
