# Passo 1034 — três defaults da linguagem, e a via óbvia que a medição refutou

**Data**: 2026-08-13
**Estado**: **fechado**. Os três achados corrigidos e verificados contra o vanilla; um quarto
sítio de língua encontrado e corrigido; uma via de implementação testada e **rejeitada por
medição**, com o âmbito diferido nomeado.

---

## Ponto de partida — o que estava feito e o que faltava

O passo foi retomado com trabalho já na árvore, não commitado. Estado encontrado:

| | estado |
|---|---|
| `bibliography.style` → `"ieee"` | código escrito |
| `figure.numbering` → `"1"` | código escrito (leitura + gate do `introspect`) |
| default de língua | tentativa via `StyleChain::lang()` |
| L0 | **nenhum** — o gate ADR-0127 do passo não chegou a ser aberto |
| testes | **6 a falhar**; nenhum teste novo para os três defaults |
| resíduos | dois `eprintln!("DEBUG …")` em código de produção **L1** |

Ou seja: a direcção estava certa e a verificação não tinha sido feita. Este relatório cobre
o que faltava.

---

## Proveniência

Árvore não commitada sobre `HEAD = 29a75805c` (Passo 1031), com trabalho dos Passos 1032 e
1033 também presente e por commitar. Binários: cristalino `./target/release/typst`
reconstruído após cada alteração; vanilla `lab/typst-original/target/release/typst`, baseline
ratificado `a51e02804`. Medições de 2026-08-13, 12:10–13:05. Documentos em `/tmp/p1034`.

---

## O que a medição mostrou

### Achado 2 — `bibliography.style` (fechado)

Mesmo documento nos dois binários, texto extraído do PDF:

```
vanilla   : A [1] B [1] C [2] D [1] Bibliography [1] J. Doe, At what cost. Fake Press, 2020. …
cristalino: A [1] B [1] C [2] D [1] Bibliography [1] J. Doe, At what cost. Fake Press, 2020. …
```

Idêntico. O `ibid.`/`op. cit.` que o P1031 mediu desapareceu — era o fallback CSL local a
actuar por ausência de `style`.

### Achado 5 — `figure.numbering` (fechado)

`Figure 1: Uma coisa` e `Table 1: Uma tabela` nos dois binários. A correcção tem de estar em
**dois** sítios que têm de concordar: a leitura do padrão (`layout/figure.rs`) e o gate de
contagem no `introspect` (`walk`, arm `Figure`) — sem o segundo, o contador não avança e a
legenda numerada sai sem número.

### Achado 6 — língua por defeito: a causa não era acidente de ambiente

O plano do passo pedia para confirmar se o português vinha do ambiente de build ou estava
codificado. **Está codificado, e a razão estava escrita no próprio código** (P158B §2/§8.2):

> `DEFAULT_SUPPLEMENTS_PT` … *"usa PT (não EN) para preservar backwards compat com tests
> pré-existentes que esperam 'Figura'"*

Isto é: o fallback estava alinhado com os testes, não com a linguagem.

**Quarto sítio, não previsto no plano**: `layout/outline.rs` tinha `Content::text("Índice")`
**fixo**, sem consultar língua nenhuma. E o vanilla, mesmo em `pt`, não diz "Índice" — diz
"Sumário". Tabela medida (`#set text(lang: X)` + `#outline()`):

| lang | vanilla | | lang | vanilla |
|---|---|---|---|---|
| en | Contents | | es | Índice |
| pt | Sumário | | it | Indice |
| de | Inhaltsverzeichnis | | zh | 目录 |
| fr | Table des matières | | | |

Correcção: fallback `en` nos dois sítios de geração de texto — `figure_supplement` e o módulo
novo `lang/outline_title.rs`. Verificado que o vanilla manda uma língua **sem localização**
(`lang: "jp"`) para inglês, não para a língua do ambiente — medido para o supplement
(`Figure 1`) e para o título do outline (`Contents`).

---

## A via óbvia, refutada por medição

A tentativa encontrada na árvore punha o default em `StyleChain::lang()`: devolver
`Some(Lang::ENGLISH)` em vez de `None`. É a leitura semanticamente natural — o default da
linguagem **é** `en`. Foi mantida, medida, e depois **revertida**:

`compiler/layout/cursor.rs:137` decide hifenizar **só** por `style.lang` ser `Some`. Não há
porta de `justify` nem de `hyphenate`. Com o default na chain, toda a hifenização liga.

Medição, mesmo documento (`The extraordinary characteristics of this remarkable phenomenon.`
numa coluna de 100pt, sem `#set text(lang:)`):

| | hífenes de quebra |
|---|---|
| vanilla | **0** |
| cristalino com o default na chain | **3** |
| cristalino com o default nos geradores de texto | **0** |

O teste de integração `lang_hyphenation_sem_set_lang_comportamento_inalterado` apanhou-o. Se
o default tivesse ficado na chain, o passo teria corrigido três defaults e introduzido uma
divergência nova, mais difícil de ver.

**Decisão**: `StyleChain::lang()` mantém `None` = "não definido", que é o estado verdadeiro; o
default de língua vive em quem **gera texto**. **Âmbito diferido, nomeado**: mover o default
para a chain exige primeiro corrigir a porta de hifenização (medir o par `justify`/`hyphenate`
do vanilla) — passo próprio.

---

## Os testes que falhavam

Os 6 iniciais, mais 4 que a correcção do fallback destapou. Nenhum era regressão de
comportamento; todos fixavam o comportamento antigo. Um caso merece registo:

**`ref_supplement_explicit_overrides_default` passava por acidente.** Asseria
`text.contains("Figura 1")` — e "Figura 1" existia na **legenda** (o supplement por defeito
era PT), não na referência que o teste dizia estar a verificar. Com a legenda em inglês o
acidente desfez-se. Reescrito para asserir as duas coisas separadamente. O separador entre
supplement e número é um **espaço não-quebrável** (U+00A0), não um espaço normal — a
tentativa anterior na árvore tinha ajustado a asserção para "dois espaços", por suposição.

Resíduos removidos: dois `eprintln!("DEBUG …")` em `introspect.rs` e
`introspect/labelled.rs` — código de produção L1.

---

## O que ficou escrito

L0 (o gate do passo não tinha sido aberto; as secções registam a decisão e a medição):

- **`compiler/lang.md` §P1034** — dono da decisão: fallback `en`, tabela do outline medida, e
  a refutação da via `StyleChain::lang()` com os números da hifenização.
- **`compiler/layout_figure.md` §P1034** — os três casos de `figure.numbering`
  (padrão / `none` / ausente) e a exigência de as duas leituras concordarem.
- **`compiler/stdlib/structural/bibliography.md` §P1034** — os três casos de `style`.
- **`compiler/layout_outline.md` §P1034** — título localizado, a apontar para `lang.md`.

Código: `lang/outline_title.rs` (novo, tabela medida + 3 testes), `lang/figure_supplement.rs`,
`layout/outline.rs`, `layout/figure.rs`, `stdlib/structural/bibliography.rs`,
`entities/style_chain.rs` (revertido, com a medição no comentário), `introspect.rs`,
`introspect/labelled.rs`.

---

## Divergências que **não** foram tocadas (medidas, fora de âmbito)

- **Dot leaders e número de página no `outline()`**: o vanilla produz
  `Uma secção . . . . . 1`; o cristalino lista só o título. Pré-existente, sem relação com
  os defaults deste passo.
- **Porta de hifenização** (acima) — passo próprio.

---

## Validação

```
crystalline-lint .       → 0 erros; 3 avisos V7 pré-existentes
cargo test --workspace   → 5858 passed; 0 failed  (4987 + 789 + 41 + 2 + 37 + 2)
```

Um erro V14 apareceu no caminho: `ecow::EcoString::from` não está na whitelist de L1
(`EcoString` está, o path `::from` não). Resolvido com `"ieee".into()`, sem alargar a
whitelist.
