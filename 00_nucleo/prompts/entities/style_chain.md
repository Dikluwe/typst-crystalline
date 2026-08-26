# Prompt L0 — StyleChain
Hash do Código: 2daa3854

## Módulo
`01_core/src/entities/style_chain.rs`

## Propósito
`StyleChain` é uma lista ligada imutável de deltas de estilo.
Permite que `#set text(bold: true)` afecte apenas o conteúdo subsequente,
com herança: um nó filho herda todas as propriedades não definidas do pai.

## Motivação (DEBT-1)
`TextStyle { bold, italic, size }` era uma struct plana que não suportava
`#set` rules. `StyleChain` substitui essa representação.

## Representação

```rust
pub struct StyleDelta {
    pub bold:   Option<bool>,
    pub italic: Option<bool>,
    pub size:   Option<f64>,       // pontos tipográficos
    pub fill:   Option<Color>,     // Passo 99 (ADR-0038)
    pub heading_level: Option<u8>, // Passo 99 (ADR-0038) forward-compat
    pub weight:   Option<u16>,     // Passo 126 — inerte
    pub tracking: Option<Length>,  // Passo 127 — inerte; preserva abs+em
    pub leading:  Option<Length>,  // Passo 128 — capturado em text; migra p/ par
    pub lang:     Option<Lang>,    // Passo 131B (ADR-0052) — tipo semântico validado
    pub font:     Option<FontList>,// Passo 132B (ADR-0053) — tipo agregador; covers deferido
    pub subscript:   Option<bool>, // Passo 448 — subscrito
    pub superscript: Option<bool>, // Passo 448 — sobrescrito
    pub highlight:   Option<Option<Color>>, // Passo 449 — fundo colorido
}

pub struct StyleChain(Option<Arc<StyleNode>>);
```

`StyleDelta::empty()` — nenhuma propriedade definida.
`StyleChain::default_chain()` — bold=false, italic=false, size=11.0pt.
`StyleChain::push(delta)` — nova cadeia herdando desta, com delta por cima. O(1).
`StyleChain::collapse() -> StyleDelta` (P352, show-set) — dobra **todos** os nós da
cadeia num único `StyleDelta` preservando a semântica `Option` (top-wins por campo;
`custom` mantém a primeira ocorrência por chave). Read-only. Usado pela captura do
show-set para extrair, de uma cadeia construída sobre `StyleChain::empty()`, o efeito
exacto do `#set` (sem os defaults de `default_chain`). Sobre `empty()` o resultado é
apenas o que o `set` definiu — não os defaults.

## Resolução de propriedades

Percorre a cadeia do topo para a raiz, retorna o primeiro valor encontrado.
Se nenhum nó define a propriedade, usa o valor por defeito do accessor.

## Bridge para layout/export

`impl From<&StyleChain> for TextStyle` — converte para `TextStyle` plano,
compatível com o layout e export actuais durante a migração.

### Fonte por defeito (P753)

Quando nenhum nó da cadeia define `font`, o bridge `From<&StyleChain> for TextStyle`
deve fornecer uma fonte por defeito para que o shaper tenha sempre uma família primária.

- O vanilla usa `Libertinus Serif` como fonte por defeito.

> **Fonte de paridade (P1031)** — **citação literal** do vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/text/mod.rs:180-182`:
>
> ```rust
> #[default(FontList(vec![FontFamily::new("Libertinus Serif")]))]
> #[ghost]
> pub font: FontList,
> ```
>
> Publicado em `typst.app/docs/reference/text/text/#parameters-font`. O mesmo doc comment
> fixa a prioridade de descoberta (`text/mod.rs:141-144`): *"The priority is: `--font-path`
> > system fonts > embedded fonts."* — o que sustenta a decisão de P753 de embutir
> `Libertinus Serif` em vez de depender do sistema. Ver
> `00_nucleo/prompts/infra/embedded_fonts.md` §"Fonte de paridade (P1031)" para a citação
> do lado das fontes embutidas (`crates/typst-kit/src/fonts.rs:129-142`).
>
> Nota de rigor: a versão ratificada é **`e0e8ca4d`**, não a tag "0.15.0" — a referência
> solta a "vanilla 0.15.0" foi removida da frase acima (ver `CLAUDE.md`, §"Referência de
> paridade").
- O cristalino, a partir de **P753**, carrega as mesmas fontes embutidas que o
  vanilla CLI (`Libertinus Serif`, `New Computer Modern`, `New Computer Modern Math`,
  `DejaVu Sans Mono`) através de `typst-assets` (ver Prompt L0
  `00_nucleo/prompts/infra/embedded_fonts.md`). `Libertinus Serif` está portanto
  sempre disponível.
- **Decisão P753**: a fonte por defeito do cristalino passa a ser `Libertinus Serif`,
  batendo com o vanilla na família da fonte. Isto elimina a diferença visual e o resíduo de
  paginação causado pelo uso anterior de `Liberation Serif`.

> **Medição de confirmação (P1031, 2026-08-13)** — a alegação *"batendo exactamente com o
> vanilla"* estava sem medição. Medida agora, documento de uma linha (`Ola mundo`, sem
> nenhum `#set text`), lida com `pdffonts`:
>
> | Binário | Fonte no PDF |
> |---|---|
> | Vanilla `/usr/local/bin/typst` (`typst 0.15.1 (e0e8ca4d)`) | `SGDIWK+LibertinusSerif-Regular-Identity-H`, CID Type 0C, embutida e subsetted |
> | Cristalino `target/release/typst` (fonte em HEAD `4f64e4e69`, árvore só com edições em `00_nucleo/prompts/**`) | `AAAAAA+LibertinusSerif-Regular`, CID Type 0C, embutida e subsetted |
>
> **A família, o corte e o tipo de embutimento coincidem** — a decisão de P753 confirma-se
> ao nível que interessa. **"Exactamente" era forte de mais** e foi corrigido para "na
> família da fonte": o prefixo de subset difere (`SGDIWK+` vs `AAAAAA+`) e o sufixo
> `-Identity-H` só aparece no vanilla. Ambos são mecânica de escrita do PDF (geração do tag
> de subset e nomenclatura do descendente CID), não superfície de linguagem, logo divergem
> de propósito (ADR-0107) — mas não devem ser tapados por uma alegação de igualdade exacta.
- Se, por qualquer razão, `Libertinus Serif` não resolver (ex: `FontBook` vazio por
  configuração especial), o shaper faz fallback pelas fontes serif definidas em
  `DEFAULT_FALLBACK_FONTS_SERIF`, ordenadas para preferir fontes sem o bug de
  decomposição de acentos (Liberation Serif, DejaVu Serif, Bitstream Vera Serif,
  FreeSerif como último recurso). Esta salvaguarda preserva o comportamento de P558.

### Histórico de decisões anteriores (P554/P558)

- **P554**: a fonte por defeito inicial do cristalino era `FreeSerif`, uma serif
  amplamente disponível em sistemas Linux. Mantinha a mesma classe visual do vanilla
  e atingia paridade de paginação no caso de teste de P553.
- **P558**: `FreeSerif` descompõe caracteres acentuados em base + mark durante o
  shaping, e o subsetter CFF do cristalino não reconstrói esses glifos correctamente.
  A fonte por defeito passou para `Liberation Serif`, igualmente disponível e sem o
  problema de acentos.
- **P753**: com a adopção de fontes embutidas via `typst-assets`, o cristalino pode
  finalmente usar `Libertinus Serif`, a fonte por defeito do vanilla.

### `math: false` sempre (P784)

`From<&StyleChain> for TextStyle` sempre define `math: false` — `StyleChain`
não carrega contexto matemático (é uma propriedade do *ponto de entrada* do
layout math, não da chain de estilos). `layout/equation.rs::layout_equation`
sobrepõe `math: true` explicitamente no `TextStyle` que passa ao motor de
layout matemático, **depois** desta conversão. Ver `entities/layout_types.md`
§P784 e `compiler/layout/equation.md` §P784.

### `math_script: false` sempre (P891)

Mesmo motivo de `math: false` acima: `From<&StyleChain> for TextStyle` sempre
define `math_script: false` — `StyleChain` não carrega contexto de script
(sub/super-índice), é uma propriedade do *ponto de construção* do
`script_style` em `math/layout/attach.rs`, não da chain de estilos.
`attach.rs::layout_attach` sobrepõe `math_script: true` explicitamente ao
construir `script_style`. Ver `entities/layout_types.md` §P891.

## Camada
L1 — pura. Sem I/O de sistema. Usa apenas `Arc` (RAM).

## Critérios de Verificação

- `StyleChain::default_chain()` retorna bold=false, italic=false, size=11.0
- `push(StyleDelta { bold: Some(true), .. })` propaga bold para filhos
- Herança: filho com `bold: None` herda bold do pai
- Clone de `StyleChain` é O(1) (só clona o Arc do topo)
- `From<&StyleChain> for TextStyle` converte correctamente
- P753: `TextStyle.font` de uma chain sem `font` definido é `Some(Libertinus Serif)`

---

## Anotação cumulativa P266 — Cobertura empírica StyleChain confirmada (Fase A)

**Data**: 2026-05-15.

P266 audit Fase A confirmou estado real cumulativo `StyleDelta`
+ resolvers + consumers. Anotações "inerte" originais em
linhas 25-27 são **factualmente desactualizadas** — os campos
têm consumers reais materializados em passos subsequentes:

### Promoções implementado → implementado⁺ confirmadas

| Campo | Consumer real | Passo | Status |
|-------|---------------|-------|--------|
| `tracking: Option<Length>` | PDF `Tc` operator + Cursor advance | P137 | implementado⁺ |
| `leading: Option<Length>` | `line_height = default + leading` em cursor | P128 | implementado⁺ |
| `lang: Option<Lang>` | Hyphenation hypher + smart-quotes localize | P144 + P155 | implementado⁺ |
| `weight: Option<u16>` | Faux-bold `faux_bold_stroke_pt` + PDF `2 Tr` | P139 | implementado⁺ |

### Estado actual 13 fields StyleDelta (P449)

- 13 fields (10 pré-P448 + `subscript`/`superscript`/`highlight`).
- 13 resolvers em `impl StyleChain` (bold/italic/size/fill/
  heading_level/weight/tracking/leading/lang/font/subscript/superscript/highlight).
- `impl From<&StyleChain> for TextStyle` materializado (inclui
  `subscript`/`superscript`/`highlight` e `baseline_offset`).

### Cobertura Text agregada empírica P266

- 13/13 entradas StyleChain (A.1-A.13) fechadas (implementado
  ou implementado⁺).
- Cobertura StyleChain subset = **100%** estrutural.
- StyleChain subsistema A: 4/12 promoções implementado⁺
  (A.8 tracking + A.9 leading + A.12 lang + via E.1 faux-bold
  consumer).

### Cross-references

- ADR-0038 — Content::Styled (consumer StyleChain via
  `push_styles`).
- ADR-0039 — TextStyle SR (bridge preservado).
- ADR-0052 — Lang tipo (consumer StyleDelta.lang).
- ADR-0053 — FontList (consumer StyleDelta.font; covers
  inabitado).
- ADR-0054 — Perfil graded (cobertura subsystem).
- ADR-0080 — L0 minimal para refactors aditivos (esta anotação
  preserva representação base).
- ADR-0084 + 0085 — Auditoria condicional + diagnóstico
  imutável (primeiro consumo directo P266).
- `00_nucleo/diagnosticos/diagnostico-text-fase-a-passo-266.md`
  — diagnóstico imutável Fase A (cobertura agregada ~86%).
- P753 — `00_nucleo/diagnosticos/paridade-producao-p753.md`:
  mudança da fonte por defeito para `Libertinus Serif` e
  adopção de fontes embutidas via `typst-assets`.

## P836 — resolver `variations()`

Novo resolver `StyleChain::variations() -> Option<FontVariations>`:
percorre a cadeia do nó interno para o externo, lê o canal custom
`"text.variations"` (`Value::Dict` já validado no eval) e dobra os
níveis com `FontVariations::fold` (interno vence por tag; tags de
níveis externos sobrevivem) — paridade do `#[fold]` do campo ghost
`TextElem::variations` no vanilla (`text/mod.rs:846-850`).

Consumido pelo `From<&StyleChain> for TextStyle` (campo
`TextStyle::variations`). Não há campo tipado em `StyleDelta`: o canal
custom é a única via (constructor `text()` e set rule convergem nele),
seguindo o precedente F-5b (P373) para `text.<campo>`.
