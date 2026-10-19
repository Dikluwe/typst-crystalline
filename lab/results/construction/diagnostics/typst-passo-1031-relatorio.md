# Passo 1031 — Relatório de fecho dos 68 achados graves de Bloco 3 (fora da família math)

**Data:** 2026-08-13
**HEAD à partida:** `4f64e4e69` (árvore limpa, conforme pré-condição)
**HEAD no fecho:** `d5ed41433` — ver §"Nota sobre concorrência com o Passo 1030"
**Vanilla ratificado:** `/usr/local/bin/typst` e `lab/typst-original/target/release/typst`, ambos `typst 0.15.1 (e0e8ca4d)` (confirmado com `--version` no início do passo)
**Cristalino:** `target/release/typst`, reconstruído da fonte em HEAD `4f64e4e69` no início do passo; todas as medições correram com a árvore de trabalho a conter **apenas** edições em `00_nucleo/prompts/**` (zero `.rs` alterados)
**Base:** catálogo do Passo 1021, secção "Restantes (68)" de `temp/p1024/bloco3_graves.md`
**Código L1–L4 alterado por este passo:** nenhum, para além do resselo de `@prompt-hash`

---

## Resumo executivo

| Classificação | Contagem |
|---|---|
| Citação acrescentada — afirmação correcta (literal ou contextual, marcada como tal) | 38 |
| L0 corrigido — a afirmação sobre a linguagem estava errada, sem mudar comportamento | 18 |
| Achado de código escalado (com dono e prioridade) | 12 |
| **Total** | **68** |

Um mesmo achado pode ter produzido citação **e** correcção **e** escalonamento; a coluna acima conta a classificação **primária** de cada um dos 68.

**Doze achados escalados** — nenhum implementado neste passo, todos com gate ADR-0127 por serem
contrato público ou comportamento por defeito. Quatro deles produzem **saída silenciosamente
errada** (não erro), que é o subconjunto mais grave.

---

## Método — a descoberta que estruturou o passo

O catálogo pedia, para cada achado, "citação `docs.typst.app/` ou medição directa". A primeira
coisa medida neste passo foi **onde vive o texto de `typst.app/docs`**. Resposta, do próprio
repositório do vanilla (`lab/typst-original/docs/README.md`):

> "This repository contains the sources for the official documentation that's available on
> https://typst.app/docs. […] `content`: Typst sources containing documentation content.
> **Supplemented by Typst markup in Rust doc comments throughout the codebase.** Note that not
> all doc comments are Typst markup, only those affected by the `#[elem]`, `#[ty]`, and
> `#[func]` macros."

Consequência prática: **um doc comment sobre um item `#[elem]`/`#[ty]`/`#[func]` do vanilla
ratificado *é* o texto publicado em `typst.app/docs`** — com a vantagem de ser citável por
`file:line` no hash pinado `e0e8ca4d`, logo reproduzível, enquanto um URL não o é. As páginas em
prosa (tutorial, `reference/language/*`) estão em `lab/typst-original/docs/content/`.

Foi essa a fonte primária usada nos 68 achados, com medição directa contra os dois binários
sempre que a afirmação era sobre comportamento observável. Todas as citações estão marcadas como
**literal** ou **contextual** (lição do P1029, Exemplo C), e as afirmações não provadas ficaram
marcadas como **inferência**, com indicação do que as refutaria.

### Erros de procedimento cometidos e corrigidos durante o passo

1. **Armadilha do binário (CLAUDE.md §"Referência de paridade", nº 2), cometida duas vezes.** O
   `cwd` do shell persiste entre chamadas; depois de um `cd lab/typst-original`, o caminho
   relativo `./target/release/typst` deixa de ser o cristalino e passa a ser o vanilla. Duas
   medições saíram assim e foram **refeitas com caminhos absolutos**. Regra adoptada para o resto
   do passo e recomendada para os seguintes: **invocar sempre os binários por caminho absoluto**.
2. **`--version` do cristalino não identifica o commit compilado.** O binário reconstruído em
   HEAD `4f64e4e69` imprimiu `typst 0.15.0 (0f8487b9)`. Causa: `02_shell/build.rs` só declara
   `rerun-if-env-changed=TYPST_COMMIT_SHA` — cópia fiel do vanilla
   (`lab/typst-original/crates/typst-utils/build.rs`), logo o cargo não reexecuta o script quando
   o HEAD muda e o carimbo fica preso ao primeiro build daquele `target/`. É **quarta armadilha**
   a juntar às três da CLAUDE.md. A proveniência das medições deste relatório usa por isso o HEAD
   real + o estado da árvore, nunca a string de `--version`.

---

## Achados escalados (12) — nenhum implementado aqui

Ordenados por prioridade sugerida. Os quatro primeiros produzem **saída errada sem erro**.

| # | Achado | Medição | L0 onde ficou registado | Prioridade |
|---|---|---|---|---|
| 1 | **Show rules não estabelecem contexto.** A documentação diz que *"Show rules provide context"*; no cristalino `counter.get()` dentro de show rule **erra**, e com `#context` explícito **compila e devolve vazio**. | vanilla `got:(1,)\|Um got:(2,)\|Dois`; cristalino erro / `got:\|Um got:\|Dois` | `compiler/eval/show_rule_termination.md` | Alta |
| 2 | **`bibliography.style` não aplica o default `"ieee"`.** Sem `style` explícito o cristalino cai num fallback local que inventa `ibid.`, `op. cit.` e retrolinks. | sem `style`: vanilla `A [1] B [1] C [2] D [1]`, cristalino `A [1] B ibid. C [2] D [1] Doe, op. cit.`; **com `style: "ieee"` os dois são byte-idênticos** | `compiler/layout/bibliography.md` | Alta |
| 3 | **Argumentos de tipo `label` são rejeitados onde a linguagem os exige** — padrão sistémico em três funções: `cite(<key>)`, `link(<label>)`, `footnote(<label>)`. | ver §"Amostra de medições", caso C | `entities/elements/cite.md`, `compiler/layout/link.md`, `entities/elements/footnote.md` | Alta |
| 4 | **Numeração hierárquica de headings perde os níveis no corpo.** O outline acerta; o corpo não. | `#set heading(numbering: "1.")` + `=`/`==`/`===`: vanilla `1.` / `1.1.` / `1.1.1.`; cristalino `1.` / `1.` / `1.` | `compiler/layout_counters.md` | Alta |
| 5 | **`figure.numbering` não aplica o default da linguagem** (`#[default(Some(NumberingPattern::from_str("1")))]`). | vanilla `Figure 1: Uma coisa`; cristalino `Uma coisa` | `compiler/layout_figure.md` (ACHADO 1) | Alta |
| 6 | **A língua por defeito não é `en`.** O vanilla usa `#[default(Lang::ENGLISH)]`. | vanilla `Figure`/`Contents`; cristalino `Figura`/`Índice`. Com `#set text(lang: "en")` coincidem | `compiler/layout_figure.md` (ACHADO 2) | Alta |
| 7 | **Supplements de `ref` divergem.** O vanilla resolve pelo `LocalName` do elemento. | `@f`: vanilla `Figure 1`, cristalino `Fig. 1`; `@eq`: vanilla `Equation 1`, cristalino `(1)`; `@h`: ambos `Section 1` ✅ | `entities/elements/ref.md` | Média |
| 8 | **Default de `body_indent` é `0pt` em vez de `0.5em`** — em `list` e `enum`. | vanilla `• Um`, cristalino `•Um`; com `body-indent: 0.5em` explícito coincidem | `compiler/layout/list_item.md`, `compiler/layout/enum_item.md`, `entities/elements/enum_item.md` | Média |
| 9 | **Heading força `italic=false` em vez de herdar.** O `show_set` do vanilla nunca toca em `TextElem::style`. | sob `#set text(style: "italic")`: vanilla usa `LibertinusSerif-**BoldItalic**`; cristalino emite o mesmo operador `Tf` com e sem o `set text` | `compiler/layout/heading.md` | Média |
| 10 | **Grupo de captura não participante devolve `""` em vez de `none`.** | `"ab".match(regex("a(x)?(b)")).captures`: vanilla `(none, "b")`, cristalino `("", "b")` | `entities/regex.md` | Média |
| 11 | **`mix` em sRGB difere numa unidade no verde.** | vanilla `#805a88`, cristalino `#805b88` | `entities/color.md` | Baixa |
| 12 | **`len` global é extensão do cristalino e conta codepoints**, enquanto o método `str.len()` da linguagem conta **bytes** — duas contagens para a mesma operação no mesmo compilador. | `#len("ação")`: vanilla `error: unknown variable \`len\``, cristalino `4`. `"ação".len()`: ambos `6` | `compiler/stdlib/foundations.md` | Baixa |

### Lacunas observadas de passagem (fora dos 68, registadas para não se perderem)

- `color.mix` não aceita a forma de peso da linguagem `c.mix((d, 50%), space: rgb)` — erro
  `argumento 'col2' deve ser Color, recebeu array`; o vanilla devolve `#aa526c`.
- As entradas do `outline` do cristalino não imprimem número de página; o vanilla imprime.
- O vanilla tem duas mensagens de erro de pacote que o L0 de `package_downloader` não regista
  (`failed to decompress package (…)`).

---

## Tabela dos 68 achados

Legenda de classificação: **C** = citação acrescentada (afirmação correcta); **X** = L0 corrigido
(afirmação errada, sem mudança de comportamento); **E** = achado de código escalado (número da
tabela acima).

### Bloco A — `compiler/eval` (4)

| # | L0 | Secção | Cls. | Prova / correcção |
|---|---|---|---|---|
| 1 | `eval/call_dispatch.md` | Contexto l.14 / §2 l.59 | C | Tabela de 10 métodos com doc comment `#[func]` do vanilla (`func.rs:412`, `selector.rs:177/188/275`, `length.rs:140`, `measure.rs:47`, `layout.rs:65`, `counter.rs:383/456/490/506`, `state.rs:274`) + guardas. **Literal** para existência/assinatura; a *ordem* de intercepção é decisão interna, provada só pelas guardas. |
| 2 | `eval/cast.md` | Propósito l.15 | C | `rel.rs:11-25` — *"A length in relation to some known length"*. **Contextual**: a fonte diz que a relação exige um comprimento de referência; a conclusão "o eval puro não o conhece" é do cristalino. |
| 3 | `eval/show_rule_termination.md` | §3 l.124 | X + **E1** | *"confirmado por teste empírico"* sem proveniência, e a premissa era **falsa**: `docs/content/reference/language/context.typ:13` diz *"Show rules provide context"*. Frase removida; argumento de terminação reancorado só nas mutações. |
| 4 | `eval/table.md` | §3 l.27-35 / l.60-62 | C | `model/table.rs:494/524/732` (`#[elem(since = "0.11.0")]`) e os campos `repeat`/`colspan`/`rowspan` com `#[default]`. **Literal**. Cobertura parcial declarada: 5 de 12 campos de `TableCell`. |

### Bloco B — bibliografia e citação (10)

| # | L0 | Secção | Cls. | Prova / correcção |
|---|---|---|---|---|
| 5 | `layout/bib_csl.md` | ~l.42 | C | `model/bibliography.rs:159-163` — `#[default({ … InstituteOfElectricalAndElectronicsEngineers … })]`. **Literal**. |
| 6 | `layout/bib_csl.md` | ~l.38 | X | O vanilla tem **cinco** forms (`cite.rs:133-147`), não quatro; a ausência de `Full` é scope-out declarado em `citation_form.md`. Redacção corrigida para "as 4 forms suportadas pelo cristalino". |
| 7 | `layout/bibliography.md` | Propósito l.17 | X + **E2** | Este fallback não tem correspondente na linguagem; omitir `style` significa `"ieee"`. Medição completa registada no L0. |
| 8 | `layout/bibliography.md` | l.74-75 (`ibid.`) | X | Nenhum estilo embutido substitui a citação repetida por `ibid.`; medido `[1]` no vanilla. Reclassificado de paridade para invenção do cristalino, a remover se E2 for corrigido. |
| 9 | `layout/bibliography.md` | l.113-119 | X | As formatações `Author (Year)` etc. são do fallback, não dos estilos CSL. Com `style` explícito o cristalino já bate byte-a-byte com o vanilla. |
| 10 | `layout/bibliography.md` | l.127 | X + **E2** | "Entry `None` → `[key]`" é **falha silenciosa**: o vanilla erra (`the document does not contain a bibliography`), o cristalino imprime texto. |
| 11 | `entities/elements/bibliography.md` | P418/P419/P420 | C + X | `bibliography.rs:111-124` (formatos e path/bytes) e `:146-154` (built-in vs `.csl`) — **literal**. Corrigido: **`.json` não é formato de bibliografia do Typst** (`bibliography.rs:428-441`); o código do cristalino já concorda, era o título de P419 que estava errado. |
| 12 | `entities/elements/cite.md` | P418 | C + **E3** | `cite.rs:39-42` sustenta `@key` como sintaxe. Escalado: `key: String` contra `as a label` (`cite.rs:44-46`). |
| 13 | `entities/citation_form.md` | l.17-33 | C | Bloco `CitationForm` do vanilla citado na íntegra (`cite.rs:133-147`). Confirma que o scope-out declarado (`Full`) está certo. Corrigida uma sobre-afirmação minha inicial: `form: none` **é** representável no cristalino (`cite.rs:26`, `Option<CitationForm>`). |
| 14 | `entities/citation_style.md` | l.21-34 | X | *"vanilla não tem default fixo"* — refutado: tem, e é `"ieee"`. Também: não existe tipo `CitationStyle` no vanilla; é construto do cristalino para o fallback. |

### Bloco C — layout de elementos model (11)

| # | L0 | Secção | Cls. | Prova / correcção |
|---|---|---|---|---|
| 15 | `layout_counters.md` | Regras de negócio l.12-17 | C + **E4** | `counter.rs:38-41` (níveis), `:481-497` (`step`), `:501-512` (`update`), `:374-381` (`display`) — **literal**. A repartição `step_flat`/`step_hierarchical` é mecânica. |
| 16 | `layout/enum_item.md` | Semântica / Validação | C + X + **E8** | Campos citados de `model/enum.rs`. Corrigido o critério de validação: dois itens separados por `Parbreak` **continuam** a numeração (`1.`, `2.`) — medido nos dois binários; o critério que exigia reinício estava errado desde P864. |
| 17 | `compiler/layout_figure.md` | Regras de negócio l.18-21 | C + **E5**, **E6** | `translations/{en,pt,de}.txt:1` — os três prefixos conferem (**literal**). Removido "lang `None` → `Figura`", que contradiz `#[default(Lang::ENGLISH)]`. |
| 18 | `layout/footnote.md` | l.30-31 | C | `model/footnote.rs:79-80` — `#[default(Numbering::Pattern(… "1" …))]`. **Literal**. |
| 19 | `layout/footnote.md` | l.37-38 | C | `footnote.rs:24-31` — *"realized as a normal superscript"*; `:324` `SuperElem::new(link)`. **Literal**; a documentação não menciona colchetes em lado nenhum. |
| 20 | `layout/heading.md` | Contexto l.12 / Comportamento l.18 | C + **E9** | `model/heading.rs:288-308` — `FontWeight::BOLD` e escala 1.4/1.2/1.0 (idênticas a `helpers.rs:260-266`). **`italic=false` não é citação**: o vanilla não toca em `style`. |
| 21 | `layout/link.md` | Documento como um todo | C + X | `model/link.rs:24` (propósito), `:41-49` (sintaxe `http://`), `:175-195` (três formas de destino) — **literal**. Corrigido: *"cor azul / sublinhado"* não era lacuna — `link.rs:26-27` diz *"links do not look any different from normal text"*; entrada removida dos scope-outs. |
| 22 | `layout/link.md` | Algoritmo, passo 4 | **E3** | Das três formas de destino documentadas, **nenhuma** é aceite; só a string de URL. |
| 23 | `layout/list_item.md` | Semântica pt. 4 e 9 | C + **E8** | `model/list.rs:97-102` e `:65-66`. `indent` confere; `body_indent` não. Registada também a definição de `tight` por *paragraph spacing/leading* e a regra de markup não-sobreponível por set rules. |
| 24 | `compiler/layout_outline.md` | Regras de negócio l.29-32 | C | **Medição**: nenhum dos binários prefixa supplement na entrada, e headings não numerados começam pelo título. As duas afirmações confirmam-se. Divergências laterais (título `Índice`, números de página) escaladas. |
| 25 | `layout/shape_block_behaviour.md` | §1 l.31-32 | C | A medição existia com proveniência completa — faltava a referência: `diagnosticos/paridade-producao-p767c.md` (2026-07-15, base `beb4d4e4f`, `mutool trace` + `compare -metric AE`), com as quatro coordenadas a ±0,005 pt do vanilla. |

### Bloco D — `entities/elements` (11)

| # | L0 | Secção | Cls. | Prova / correcção |
|---|---|---|---|---|
| 26 | `columns.md` | l.8, 21-23 | C | `layout/columns.rs:28-35` — *"use the `{page}` function's `columns` parameter instead […] rather than wrapping all of your content in a layout container. As a result, things like pagebreaks, footnotes, and line numbers will continue to work as expected."* **Literal**, e nomeia o mecanismo exacto que `page_columns` distingue. |
| 27 | `enum_item.md` | l.65-70 | C + **E8** | `model/enum.rs` declara os três campos no container — premissa **literal**. A divergência mecânica é legítima; o *valor* do default de `body_indent` não. |
| 28 | `equation.md` | l.15-19, 37-39 | C | `math/equation.rs:63-64` — *"How to number **block-level** equations"*. O gate `block &&` é **literal**, não inferência. Guardar o padrão só na chain é mecânica declarada. |
| 29 | `figure.md` | l.24-25, 45 | C + X | `figure.rs:203-204` (caption opcional) e `:296-299` (numbering) — **literal**. Corrigido: o gate do vanilla é só `numbering.is_some()` (`figure.rs:437-445`), sem caption. **Medição mostra o observável em paridade** (`Figure 3: C1` / `Figure 4: C2` nos dois), logo é achado de redacção, não de código. |
| 30 | `footnote.md` | l.7-9, 40 | C + X + **E3** | `footnote.rs:224-231` sustenta `counter(footnote)` — **literal**. Corrigido: "toda a nota conta" tem excepção — `(!self.is_ref())` (`footnote.rs:174-178`). Escalado: a forma de referência não existe no cristalino. |
| 31 | `metadata.md` | l.18-30 (Struct) | C | `introspection/metadata.rs:24-28`, `#[required] pub value: Value`. **Literal**. `Box<Value>` é mecânica. |
| 32 | `metadata.md` | l.36 (`plain_text`) | C | `metadata.rs:3` — *"without producing visible content"*. **Contextual**: a fonte afirma a ausência de saída visível; a string vazia é a representação escolhida. |
| 33 | `metadata.md` | l.43-48 (`eq`) | X | Não tem base na linguagem: o vanilla deriva `PartialEq` normalmente. Reclassificado de "paridade" para **quirk interno preservado** (a igualdade do Rust é terreno de divergência, ADR-0107). |
| 34 | `metadata.md` | l.54 (`value`) | C | `metadata.rs:14-22` — o exemplo do doc comment é literalmente `query(<note>).first().value`. **Literal**. |
| 35 | `ref.md` | l.40-41 | C | `model/reference.rs:148-159`. **Contextual**: a documentação fala do "número referenciado"; que venha do `Introspector` no layout é do cristalino. |
| 36 | `ref.md` | l.42-44 (supplements) | X + **E7** | `heading.rs:321-327` + `translations/en.txt:1,2,3,5`. A afirmação *"nenhum para heading/equation"* estava errada nos dois lados: a linguagem usa `Section`, e o cristalino também já o faz. |

### Bloco E — `infra` e passo-537b (5)

| # | L0 | Secção | Cls. | Prova / correcção |
|---|---|---|---|---|
| 37 | `infra/embedded_fonts.md` | Contexto | C | `typst-kit/src/fonts.rs:129-142` (bloco `embedded()` com a repartição texto/math/code) e `text/mod.rs:180-182` (`Libertinus Serif` como default). **Literal**, incluindo a prioridade *"`--font-path` > system fonts > embedded fonts"*. |
| 38 | `infra/image-sizer.md` | Fontes de DPI | C | `visualize/image/raster.rs:363-375` — a cadeia `exif_dpi().or_else(jpeg_dpi).or_else(png_dpi)` fixa **literalmente** a prioridade EXIF > JFIF > pHYs; `:393-405` e `:423` dão os predicados de cada formato. Fallback 72 DPI em `image/mod.rs:431` + `typst-layout/src/image.rs:47`. |
| 39 | `infra/package_downloader.md` | Decisões P763 | C | Tabela de 10 linhas com `file:line` de `typst-kit/src/packages.rs` (registo, mirror, namespace, URLs, cache dirs, `rename()` atómico, ausência de verificação de integridade). As mensagens de erro de §6 confirmam-se à letra em `diag.rs:714-731`. |
| 40 | `infra/package_version_resolution.md` | Decisões P764 | C | `typst-syntax/src/package.rs:353-362` (três componentes `u32`), `:354` (`Ord` derivado), `:450` (mensagem de quarto componente), `packages.rs:203-214`. **Medição** confirma que `import` exige versão: mesma mensagem e mesma coluna nos dois binários. |
| 41 | `passo-537b-set-page-columns.md` | §2 | C | A mesma citação de `columns.rs:28-35`. **Contextual**: sustenta que as duas formas diferem quanto a notas de rodapé e qual é a "normal"; **não** sustenta o layout exacto da forma-função, que fica marcado como **inferência** a medir. |

### Bloco F — tipos de `entities`, parte I (12)

| # | L0 | Secção | Cls. | Prova / correcção |
|---|---|---|---|---|
| 42 | `bytes.md` | l.13 | C + X | `foundations/bytes.rs:16-28` — **literal**. Corrigido: a documentação **não** chama `bytes` opaco; descreve-o como equivalente a um array de inteiros 0–255, iterável e convertível. |
| 43 | `color.md` | l.82 (`Hsl`/`Hsv`) | C | `visualize/color.rs:26-27` — o vanilla delega em `palette` sobre encoding sRGB; "algoritmo standard" ganha origem. |
| 44 | `color.md` | l.213 (Luma "Rec.709") | C | `color.rs:28` + `:1771-1774`. Marcado como **inferência**: o vanilla usa `palette` com encoding sRGB, e a atribuição "Rec.709" é dedução a partir do encoding, não citação. |
| 45 | `color.md` | l.215 (CMYK ICC) | C | `color.rs:30-38` — perfil **CGATS TR 001-1995** carregado de `typst_assets::icc::CMYK_TO_XYZ`. **Literal**; o scope-out ADR-0083 estava certo e agora tem `file:line`. |
| 46 | `color.md` | l.265-273 (P744) | X + **E11** | Medidos os sete critérios. Cinco em paridade exacta; **dois estavam errados no L0** (os dois `mix`) e foram corrigidos para os valores do vanilla. |
| 47 | `content.md` | l.15-26 | C | `content/mod.rs:84`, `content/element.rs:234`, `content/raw.rs:17-19` e `:63-66` — **literal** para as quatro afirmações sobre a estrutura interna do original. |
| 48 | `content.md` | várias secções | C | Inspecção qualifica o achado: as citações sem linha apontam para ficheiros **do cristalino** (consumidores), não do vanilla, e as menções a "vanilla" são maioritariamente divergências declaradas. Fixada **convenção de citação** de 3 regras no L0; revisão por variante distribuída pelos L0 dedicados. |
| 49 | `counter_format.md` | Contexto/Semântica/Tokens | C | `model/numbering.rs:19-21` e `:70-75` — **literal** para prefixo/sufixo. Registadas duas lacunas: os 24 símbolos do vanilla (o cristalino suporta 4) e a repetição do último símbolo. |
| 50 | `decimal.md` | §1 Contexto | C + X | `foundations/decimal.rs:15-18`, `:26-30`, `:72-79` — **literal**. Corrigido: é **vírgula fixa**, não precisão arbitrária, com 28 a 29 dígitos e máximo `79228162514264337593543950335`. |
| 51 | `dir.md` | Contexto/Semântica | C | `layout/dir.rs:6-20` — as quatro variantes e os nomes de superfície, **literal**. O default `TTB` contra `Smart<Dir>` fica marcado como **inferência** não medida. |
| 52 | `document_info.md` | Propósito/Semântica | C | `model/document.rs:14-34` — **literal** para `#set document(...)` e para "embedded into the output, but not visibly rendered". Lacuna declarada: 3 de 5 campos. |
| 53 | `duration.md` | §1 Contexto | C | `foundations/duration.rs:9-12` — *"Represents a **positive or negative** span of time"*. **Literal**, e é exactamente a afirmação-chave (representação com sinal). |

### Bloco G — tipos de `entities`, parte II (14)

| # | L0 | Secção | Cls. | Prova / correcção |
|---|---|---|---|---|
| 54 | `layout_types.md` | PageConfig l.101-131 | C | `layout/page.rs:127-131` — *"set automatically to 2.5/21 times the smaller dimension"* — **na documentação publicada**, não só no código; implementação em `typst-layout/src/pages/run.rs:121`. **Literal** em duas camadas. |
| 55 | `operators.md` | Tabela de precedências | C | `docs/content/reference/language/scripting.typ:256+` (tabela oficial, com *"higher binds stronger"*) e `typst-syntax/src/ast.rs:1810-1815` / `:1930-1952`. **Literal** em duas camadas independentes. |
| 56 | `package-spec.md` | Contexto / validação | C | `typst-syntax/src/package.rs:224-232`, `:314-317`, `:319-324`, `:329-338`, `:344-350` — **literal**. O vanilla usa o **mesmo** `is_ident` e o mesmo `Scanner`. Medição confirma mensagem e coluna idênticas. |
| 57 | `regex.md` | Semântica / API | C + **E10** | `foundations/str.rs:426-437` — **literal** para a forma do resultado e para o `skip(1)`. Offsets em bytes **confirmados por medição** (`start=8` para `"ação: "`). Escalado o `""` vs `none`. |
| 58 | `state_registry.md` | l.72 | X | *"segundo init é ignorado (paridade vanilla)"* — sem base: no Typst `state(key, init)` carrega o inicial por construção, não há "registo de init". Reclassificado como decisão defensiva. |
| 59 | `state_registry.md` | l.73 | X | *"vanilla geraria erro"* para update sem init — idem: a situação não é produzível na linguagem. |
| 60 | `state_registry.md` | l.124 | X | *"fixpoint comemo"* nomeia o mecanismo errado; o vanilla resolve por **iterações de layout**, e isso está **documentado** (`state.rs:339-349`, incl. *"will not converge within 5 attempts"*). Consequência: o single-pass é restrição de comportamento observável, não simplificação invisível. |
| 61 | `state_update.md` | l.75 | C + X | `state.rs:336-337` — **literal** para a forma-função. Corrigida a superfície (`state("key").update(fn)`, método) e removido o número "80%", que não tinha medição — a documentação até **recomenda** a forma que falta. |
| 62 | `style_chain.md` | l.63 | C | `text/mod.rs:180-182` — `#[default(FontList(vec![FontFamily::new("Libertinus Serif")]))]`. **Literal**. |
| 63 | `style_chain.md` | l.69 | C + X | **Medição**: família, corte e tipo de embutimento coincidem (`LibertinusSerif-Regular`, CID Type 0C, subsetted). *"Exactamente"* corrigido para *"na família da fonte"* — o tag de subset e o sufixo `-Identity-H` diferem (mecânica de escrita do PDF). |
| 64 | `style.md` | Contrato l.19-104 | X | **`text.bold` e `text.italic` não existem na linguagem.** O vanilla tem `text.style` (`FontStyle`) e `text.weight` (`FontWeight`). Medição: os dois binários rejeitam `#set text(bold: true)` com a mesma mensagem e coluna — a superfície do cristalino já estava certa, a etiqueta do L0 é que não. Tabela de 5 propriedades com defaults acrescentada. |
| 65 | `symbol.md` | §1 l.13 | C | `foundations/symbol.rs:19-37` — **literal**, incl. os exemplos `sym.arrow.r` e `sym.gt.eq.not`. Duas lacunas registadas: acesso sem prefixo em modo math e o módulo `emoji`. |
| 66 | `tag.md` | Semântica l.53 | X | O *facto* (hash no `End`) é literal; o *motivo* estava inventado. O vanilla diz o contrário: *"simply to make the two enum variants more balanced in size […] **There are no semantic reasons for this.**"* (`introspection/tag.rs:18-22`). Logo não é matéria de paridade. |
| 67 | `tiling.md` | Contexto §2-4 | C + X | `visualize/tiling.rs:15-24` — **literal**. Corrigido: `tiling` preenche **ou traceja** e o corpo de cada célula é conteúdo arbitrário; não é construtor "para gradientes e imagens". |

### Bloco H — `compiler/stdlib` (1)

| # | L0 | Secção | Cls. | Prova / correcção |
|---|---|---|---|---|
| 68 | `stdlib/foundations.md` | `native_len` l.246-251 | X + **E12** | Alegação de paridade falsa em dois pontos: (a) o vanilla **não tem** função global `len`; (b) `str.len()` conta **bytes** (`foundations/str.rs:184-186`), não caracteres. O caminho de método do cristalino já está correcto (`A3B6C2D2E1` nos dois binários); a incoerência é só na função global. |

---

## Amostra de medições lado a lado com a fonte

Quatro exemplos completos, com a frase do L0, a prova exacta, e a natureza da citação.

### Caso A — citação literal que **refuta** o L0 (o mais valioso do passo)

- **L0 (antes)**: `entities/tag.md` §Semântica: *"Paridade com vanilla, onde o hash é guardado no
  End para **optimização de queries**."*
- **Fonte**: vanilla `crates/typst-library/src/introspection/tag.rs:18-22`:
  ```rust
  /// The element with the given location and key hash ends here.
  ///
  /// Note: The key hash is stored here instead of in `Start` simply to make
  /// the two enum variants more balanced in size, keeping a `Tag`'s memory
  /// size down. There are no semantic reasons for this.
  ```
- **Avaliação**: **literal**, e refuta a parte que interessa. O facto está certo; o motivo estava
  inventado. E como o vanilla declara *"no semantic reasons"*, isto sai inteiramente do domínio da
  paridade de linguagem e passa para o domínio onde o cristalino diverge de propósito. Um L0 que
  justificasse uma escolha estrutural com esta frase estaria a construir sobre areia.

### Caso B — citação literal em prosa oficial, que reabre um gatilho dado como hipotético

- **L0 (antes)**: `compiler/eval/show_rule_termination.md` §3: *"Actualmente (Passo 1007,
  confirmado por teste empírico), `counter.get()`/`state.get()` estão bloqueados fora de `context`
  durante este loop"*, usado para justificar terminação antecipada.
- **Fonte**: página de referência da linguagem,
  `lab/typst-original/docs/content/reference/language/context.typ:13`:
  > "Aside from explicit context expressions, context is also established implicitly in some
  > places that are also aware of their location in the document: **Show rules provide context**
  > […] and numberings in the outline, for instance, also provide the proper context to resolve
  > counters."
- **Medição** (2026-08-13), `#let c = counter("x")` + `#show heading: it => [got:#c.get()|#it.body]`
  + `#c.step()` + `= Um` + `#c.step()` + `= Dois`:

  | Binário | Resultado |
  |---|---|
  | Vanilla `typst 0.15.1 (e0e8ca4d)` | `got:(1,)\|Um got:(2,)\|Dois` |
  | Cristalino (fonte em HEAD `4f64e4e69`) | **erro**: `counter.get() can only be used inside context` |

- **Avaliação**: **literal**. A frase citada não é doc comment mas prosa da página oficial de
  `context`, que é ainda mais directa. O "teste empírico" de P1007 nunca deixou proveniência e não
  é reproduzível; a premissa está refutada. O gatilho de reabertura do L0 **já disparou**.

### Caso C — medição que revela um padrão sistémico, não um caso isolado

- **L0 (antes)**: três documentos independentes descreviam a sua restrição como se fosse local —
  `cite.md` (`key: String`), `link.md` (*"`Content::Link` mapeia sempre para URL"*), `footnote.md`
  (*"toda a nota conta"*).
- **Fonte**: `cite.rs:44-46` — *"The citation key […], **as a label**"*; `link.rs:175-195` — *"`dest`
  can take one of three forms: - A label […] - A location […] - A dictionary with a `page` key"*;
  `footnote.rs:174-178` — `(!self.is_ref())`, com `FootnoteBody::Reference(Label)`.
- **Medição** (2026-08-13):

  | Entrada | Vanilla | Cristalino |
  |---|---|---|
  | `#cite("netwok")` | `error: expected label, found string` | aceita → `[netwok]` |
  | `#link(<intro>)[Ir para intro]` | compila | `error: link() espera URL como string, recebeu label` |
  | `A#footnote[Um] <fn> B#footnote(<fn>) C#footnote[Dois]` | `A1 B1 C2` | `error: footnote() espera content ou string, recebeu label` |

- **Avaliação**: cada citação é **literal**; o valor está na justaposição. Tratados um a um, seriam
  três lacunas menores. Juntos são **um** achado: o cristalino não aceita argumentos de tipo
  `label` onde a linguagem os exige — e no caso do `cite` a polaridade está invertida (aceita
  string onde o vanilla erra). Um passo de correcção deve tratá-los em conjunto.

### Caso D — medição que **confirma** o L0 e delimita a divergência a um único ponto

- **L0**: `compiler/layout/bibliography.md` descrevia o fallback local sem dizer que o caminho CSL
  real já funcionava.
- **Medição** (2026-08-13), `A @netwok B @netwok C @other D @netwok` + `#bibliography("works.bib")`:

  | Caso | Vanilla | Cristalino |
  |---|---|---|
  | sem `style` | `A [1] B [1] C [2] D [1]` + `[1] J. Doe, At what cost. Fake Press, 2020.` | `A [1] B ibid. C [2] D [1] Doe, op. cit.` + `[1] Doe, Jane. At what cost. Fake Press (2020). ↑[1][2][4]` |
  | `style: "ieee"` | idem | **byte-idêntico ao vanilla** |

- **Avaliação**: **medição**, sem citação equivalente possível (o vanilla não documenta "o que
  acontece sem fallback" porque não tem fallback). O valor está na segunda linha: sem ela, o
  diagnóstico natural seria "o render de bibliografia do cristalino está errado" e o passo de
  correcção seria enorme. Com ela, o defeito reduz-se a **uma linha**: aplicar o default `"ieee"`.
  Medir os dois casos em vez de um mudou a dimensão do trabalho a agendar.

---

## L0s alterados (51)

`compiler/eval/call_dispatch.md`, `compiler/eval/cast.md`, `compiler/eval/show_rule_termination.md`,
`compiler/eval/table.md`, `compiler/layout/bib_csl.md`, `compiler/layout/bibliography.md`,
`compiler/layout_counters.md`, `compiler/layout/enum_item.md`, `compiler/layout_figure.md`,
`compiler/layout/footnote.md`, `compiler/layout/heading.md`, `compiler/layout/link.md`,
`compiler/layout/list_item.md`, `compiler/layout_outline.md`,
`compiler/layout/shape_block_behaviour.md`, `compiler/stdlib/foundations.md`, `entities/bytes.md`,
`entities/citation_form.md`, `entities/citation_style.md`, `entities/color.md`,
`entities/content.md`, `entities/counter_format.md`, `entities/decimal.md`, `entities/dir.md`,
`entities/document_info.md`, `entities/duration.md`, `entities/elements/bibliography.md`,
`entities/elements/cite.md`, `entities/elements/columns.md`, `entities/elements/enum_item.md`,
`entities/elements/equation.md`, `entities/elements/figure.md`, `entities/elements/footnote.md`,
`entities/elements/metadata.md`, `entities/elements/ref.md`, `entities/layout_types.md`,
`entities/operators.md`, `entities/package-spec.md`, `entities/regex.md`,
`entities/state_registry.md`, `entities/state_update.md`, `entities/style_chain.md`,
`entities/style.md`, `entities/symbol.md`, `entities/tag.md`, `entities/tiling.md`,
`infra/embedded_fonts.md`, `infra/image-sizer.md`, `infra/package_downloader.md`,
`infra/package_version_resolution.md`, `passo-537b-set-page-columns.md`.

Hashes resselados com `crystalline-lint --fix-hashes .` — 43 ficheiros `.rs` tocados, **apenas** na
linha `@prompt-hash` (verificado: `git diff HEAD -- '*.rs'` tem zero linhas alteradas que não sejam
`@prompt-hash`).

---

## Nota sobre concorrência com o Passo 1030

O passo autorizava execução em paralelo com o 1030. Isso aconteceu de facto: durante a execução, o
HEAD avançou de `4f64e4e69` para `d5ed41433` (*"feat(eval): P1030 fatia 1 — `#set math.*` chega ao
construtor"*), e esse commit **arrastou consigo** as edições de L0 dos Blocos A e B deste passo
(9 ficheiros: `eval/call_dispatch.md`, `eval/cast.md`, `eval/show_rule_termination.md`,
`eval/table.md`, `layout/bib_csl.md`, `layout/bibliography.md`, `citation_form.md`,
`citation_style.md`, `elements/cite.md`). Conteúdo verificado íntegro em HEAD.

Consequência prática: a atribuição por commit fica misturada. Os restantes 42 ficheiros deste passo
estão por commitar no momento em que este relatório é escrito. **Recomendação para passos
paralelos futuros**: trabalhar em branch própria, ou combinar antecipadamente quem commita o quê.

A restrição de área foi respeitada: este passo não tocou `01_core/src/compiler/eval/rules.rs` nem
prompts de `compiler/math/` / `entities/elements/math_*`. Nenhum dos 68 achados caiu nessas áreas.

---

## Validação

```
crystalline-lint .
cargo test --workspace
```

**Resultado** (corrido após o resselo de hashes, com a árvore descrita acima):

- `crystalline-lint .`: **0 erros**, exit code 0. Restam 3 avisos V7 de prompts órfãos
  (`auditar-fatiamento.md`, `auditar-spec.md`, `infra/package_version_resolution.md`), todos
  pré-existentes e fora do âmbito deste passo.
- `cargo test --workspace`: **5855 testes passaram, 0 falharam**, 3 ignorados.
  - `typst-core`: 4984 · `typst-infra`: 789 · `typst-shell`: 41 · `typst-wiring`: 2
  - testes de integração: 37 + 2

Zero regressão. Nota de honestidade sobre esta medição: o `cargo test` corre sobre o estado
combinado deste passo **e** da fatia 1 do P1030 (que introduziu código), porque os dois partilham a
mesma árvore — ver §"Nota sobre concorrência". O contributo deste passo para o resultado é nulo por
construção: não alterou nenhuma linha de lógica.

---

## Resultado

Os 68 achados estão fechados: 38 por citação, 18 por correcção de L0, 12 escalados com dono e
prioridade. Com o Passo 1029 (22 achados da família math), **o catálogo de graves do Bloco 3 fica
processado por inteiro (90 de 90)**. Restam os 58 achados leves e o Bloco 4 (referências a passo,
com tratamento à parte já decidido).

O saldo mais relevante não é a contagem de citações: é que **12 divergências reais de
comportamento** estavam escondidas atrás de afirmações de paridade não verificadas, quatro delas a
produzir saída errada sem erro. O padrão que as gerou é reconhecível e vale como aviso para os
passos seguintes: *frases que descrevem correctamente o cristalino, escritas na forma "paridade
vanilla", sem que ninguém tenha ido ver.*
