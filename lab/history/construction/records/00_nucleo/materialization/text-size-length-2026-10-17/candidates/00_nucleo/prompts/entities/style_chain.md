# Prompt L0 — StyleChain
Hash do Código: cded0a19

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
    pub size:   Option<Length>,    // tamanho declarado abs+em
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

## Default da proveniência `TextItem`

O bridge `StyleChain::to_text_style` define `math_text_item=false`. O
StyleChain não origina essa proveniência: somente o layout matemático marca um
`Content::Text` direto como TextItem ao emitir o item correspondente.

Todos os demais campos continuam resolvidos da cadeia. O bit não entra em
`StyleDelta`, parsing, eval ou defaults configuráveis pelo utilizador.

## Fronteira de instrumentação

`StyleChain` não expõe identidade de alocação ou método condicionado por
`p1339_observation`. Estrutura persistente, defaults, `Arc`, clonagem O(1),
resolução e APIs produtivas permanecem independentes de telemetria.

## P1481 — origem morfológica no TextStyle (IMPLEMENTADO; confirmado em 2026-09-23)

Medição: `StyleDelta.bold_from_strong` e `italic_from_emph` já distinguem
`*strong*`/`_emph_` de `#set text(bold/italic: true)`; o
`From<&StyleChain> for TextStyle` atual copia somente bold/italic. O
vanilla QDF do corpus P1481 contém nós `/S /Strong` e `/S /Em`.

Contrato confirmado: resolver os dois bits de proveniência na conversão
única para `TextStyle.semantic_strong/semantic_emph` usando a regra
top-wins da chain e valor falso por default. A resolução não altera
bold/italic de render. Não criar semântica a partir do peso ou aparência.
Os campos públicos são definidos pelo owner `entities/layout_types.md`
e foram confirmados em 2026-09-23 sob ADR-0127.

## Leitura direta da proveniência no bridge — 2026-10-09

Medição antes da decisão: no release SHA-256
`a01e3854b67abf7ec7902e14e0a695a3ea425a91ad13aab464247aade98b8491`,
`stacks-after.json.gz` do diagnóstico `table-local-scaling-followup-2026-10-09`
registra 801 amostras de alocação em `StyleChain::collapse` sob
`layout_styled` no caso `local-set-1600`; cada amostra representa 64 pedidos.
`01_core/src/entities/style_chain.rs:779-784` chama `collapse()` somente para
ler `bold_from_strong` e `italic_from_emph`, mas `collapse()` copia também o
canal custom e outras propriedades. O gate externo
`semantic-red-gate.json.gz` registra 1.067.883 pedidos para esse caso no
binário restaurado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`,
working tree não commitado e UTC da medição. Inferência refutável: resolver
somente esses dois campos deve poupar clones sem mudar semântica; o gate de
alocações, os testes de precedência e a comparação bilateral de saída e tempo
podem refutá-la.

Decisão interna: no bridge `From<&StyleChain> for TextStyle`, percorrer os nós
do topo para a raiz e capturar a primeira definição de cada bit de origem.
Parar quando ambos estiverem definidos ou acabar a cadeia; ausência equivale
a `false`. Não chamar `collapse()` nesse bridge. O método público `collapse()`
continua integral para captura de show-set. Não mudar representação, API,
defaults, fase de pipeline ou interpretação de `bold`/`italic`. Aceitação:
`semantic_strong` e `semantic_emph` mantêm top-wins, inclusive `Some(false)`
interno sobre `Some(true)` externo; saída de linguagem/raster igual e redução
mensurável de alocações e tempo no caso com escopos locais. Correção interna
em fluxo contínuo conforme ADR-0127.


## C2-R1 — correção interna de paridade

Medição anterior à decisão: 00_nucleo/diagnosticos/performance-c2-2026-10-07/table-local-regression.json, três samples/r*-table-local/samples.json com fontes/argv/UTC/estado/hash do release ac889581 e vanilla pin9f2b6e871. Tamanho cresce10%, segunda coluna x60.880→38.999 com sobreposição. RED semântico em 00_nucleo/diagnosticos/table-local-c2r1-2026-10-07/red.log: tamanho10pt e separação Auto falham, tamanho local explícito11pt já passa.

Fonte upstream typst-eval/src/code.rs:32–43 aplica somente styles da declaração set à cauda. Tekt expression_dispatch.rs:132 colapsa raiz/defaults e não só declarações locais; style_chain.rs:304–315 contém size11/boldfalse/italicfalse na raiz. tracks.rs:328 reconhece somente Cell com resolved, deixando a Cell standalone Complete na projeção; measurement.rs não mede TableCell. É semântica/estilo/posição observável, não obrigação de reproduzir árvore ou algoritmo upstream. A causa conjunta é inferência refutável por fonte/tamanho/posição ainda divergentes após isolar estilos locais e medir o visual completo.

Classificação ADR-0127: reparo interno de paridade, sem API pública/trait/entidade/flag/fase nova. Fluxo contínuo autorizado por pedido de corrigir prioridade1; L0 primeiro, RED→GREEN, resselo restrito, builds/suítes/lint e recoleta física/temporal.

Responsabilidade: adicionar helper crate-private local_delta_since que retorna somente o prefixo de nós empilhados após um ancestral compartilhado, ou None se o ancestral não pertence à cadeia. Usar identidade interna Arc exclusivamente para delimitar fronteira lexical comprovada, sem expor identidade a caller/telemetria e sem comparação dos valores como prova de declaração. Nó explicitamente size11/boldfalse deve sobreviver mesmo se igual ao valor herdado. Dobra top-wins existente dos deltas permanece; cadeia ancestral/defaults não são copiados ao resultado. Chain==ancestral retorna delta vazio. Pureza, representação pública, O(1) clone e APIs vigentes permanecem. Testar ancestral compartilhado, declaração igual ao default e não ancestral.

## Capacidade exata na conversão de fontes custom — 2026-10-10

### Medição antes da decisão

`00_nucleo/diagnosticos/style-font-capacity-2026-10-10/before.json.gz`
registra HEAD, working tree, arquivos/diffstat, fontes e o binário baseline
bb138eb8d2f8a5762ed693bfe2dda526c092780a47e0dc4c5e8e4f7af35b81a8.
O memo tentado em style-font-resolution-reuse-2026-10-10 foi descartado:
menos pedidos não trouxeram aceleração consistente e o RSS dos alvos subiu.
Não há cache desse experimento neste consumer.

`00_nucleo/diagnosticos/style-scope-transfer-2026-10-10/stacks-current.json.gz`
do mesmo binário inclui 400 amostras de pedidos480bytes no vetor sob
StyleChain::font/local-set1600. `capacity-probe.json` desta rodada mede
FontFamily=120bytes e reproduz filter_map/collect de uma família com
capacidade4, len1 e buffer480bytes. A amostragem é periódica inclusiva,
não custo exclusivo; a sonda sintética valida o tipo/iterator, não tempo.
`entities/style_chain.rs:641–653` usa collect com limite inferior zero.
Inferência: reservar apenas as strings efetivas evita capacidade excedente;
bytes acumulados, RSS e tempo podem refutá-la. Não inferir intenção do
vanilla nem impor igualdade de bytes PDF; capacidade é mecânica.

### Decisão interna, fluxo contínuo ADR-0127

Ao encontrar text.font como Array, contar somente Value::Str desse array
e reservar esse número para o Vec<FontFamily>. Preencher na mesma ordem
com FontFamily::new e normalização vigentes. Array sem strings não aloca
buffer e continua retornando None, interrompendo a herança como antes.
Typed delta.font continua antes do custom no mesmo nó; custom usa primeira
ocorrência; tipo diferente de Array continua ignorado. Não alterar dict
custom legado/regex, defaults, StyleDelta, public APIs, fase ou cache.
A leitura faz duas travessias locais do array; push, nós e chain não mudam.

### Aceitação e limites

RED→GREEN do gate externo: bytes solicitados estritamente menores nos
controles de fontes, sem crescimento da contagem de alocações. É gate
mecânico, separado da linguagem. Testes de ordenação, mixed/empty/invalid,
precedência, typed regex e declarações originais, controles de texto/
markup/query/fontes/posição/páginas/raster e suites preservam os observáveis.
Comparar tempo e RSS com argv equivalentes, rodadas pareadas sem
interposer e controles curtos/sem declarações consumidas. Regressão
persistente nesses custos ou mudança observável refuta o candidato e
requer restauração exata do source e deste L0. Menos bytes acumulados não
certifica memória residente menor.


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/text-size-affine.toml sha256:8a628c506765c5d67144248103049ab49d2f1402312cb210c25ac9902b0f2682


## TS1 — tamanho completo de texto (PROPOSTO; gate ADR-0127)

### Medição anterior à decisão

`00_nucleo/diagnosticos/text-argumentos-2026-10-17/before.json.gz`, `before-measurement.json.gz`,
`morph-before-measurement.json.gz` e `audit.json` conservam HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado, UTC, status/diffstat,
fontes/L0/binários/argv e o pin9f2b6e8715237cb086899a42873660fe744622e8.
Em named-em, A mede0pt contra24pt; em affine,3pt contra27pt; nested sobre18pt
mede0pt contra36pt. As seis permutações Size/Color/body são aceitas pelo pin
mas rejeitadas pelo Tekt. Set posicional18pt é ignorado (12 contra18pt).
Fonte pinada text/mod.rs:290,303,924–930,1136–1142 e args.rs:178–247
estabelece busca tipada, preferência nomeada, body restante e composição.
style_chain.rs:513–517 e constructor.rs:123 descartam em; collapse em
style_chain.rs:394–395,419–422 aplica top-wins onde tamanho precisa fold.
É tamanho/escopo observável da linguagem; a representação Rust é escolha
interna. Não inferir intenção histórica. Refutar por restauração, Raw,
heading, math, capture/show, dupes/spans, zero/negativos ou métricas divergentes.
A sonda de igualdade refutou a hipótese de comparar os valores declarados:
no pin, text(size:2em)[A] == text(size:24pt)[A] é true. StyledElem::eq no pin
content/mod.rs:779–782 compara child. Esse defeito de igualdade requer sua
própria correção; o tipo completo não é prova de fechamento desse eixo.

### Ratificação obrigatória

Este candidato altera dois contratos públicos, descritos em api.md. Não é
L0 vigente até aprovação explícita. Hash do Código antigo é apenas contexto,
não resselo de implementação inexistente. Materializar somente após ratificar
TS1; sem release/merge/push. Testes S1–S16 e gates sob12GiB, swap zero.

### Responsabilidade exclusiva adicional

StyleDelta.size passa a Option<Length>. Default tipado11pt; size() continua f64 de pontos resolvidos. Dobrar contribuições typed/custom text.size na ordem lexical com precedência typed no mesmo nó. collapse e local_delta_since conservam a função afim, sem completar com default quando o ancestral foi excluído. Demais propriedades mantêm suas regras. Diff/bridge leem o comprimento completo; Pt existe só no render resolvido.
