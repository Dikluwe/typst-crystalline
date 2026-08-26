# Prompt L0 — `compiler/layout/heading`
Hash do Código: 2d297d4f

**Camada**: L1 · **Alvo**: `01_core/src/compiler/layout/heading.rs`
**Criado em**: 2026-06-24 (P451 — heading numbering patterns)
**ADRs relevantes**: ADR-0109 (atomização), ADR-0033 (paridade vanilla)

---

## Contexto

Layout de `Content::Heading` (via `HeadingElem`). Responsável por aplicar estilo bold escalado por nível e, opcionalmente, prefixar o body com o número do counter hierárquico.

---

## Comportamento

1. Aplicar `TextStyle` com `bold=true`, `italic=false`, tamanho escalado por `heading_scale(level)`.

> **Fonte de paridade (P1031)** — vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/model/heading.rs:288-308`, `impl ShowSet for Packed<HeadingElem>`:
>
> ```rust
> let scale = match level { 1 => 1.4, 2 => 1.2, _ => 1.0 };
> …
> out.set(TextElem::size, TextSize(size.into()));
> out.set(TextElem::weight, FontWeight::BOLD);
> ```
>
> - **`bold=true`** — citação literal (`out.set(TextElem::weight, FontWeight::BOLD)`).
> - **Escala por nível** — `heading_scale` do cristalino
>   (`01_core/src/compiler/layout/helpers.rs:260-266`) devolve `1 => 1.4, 2 => 1.2, _ => 1.0`,
>   os mesmos valores do vanilla. Citação literal.
> - **`italic=false` — não é citação literal.** O `show_set` do vanilla **não** toca em
>   `TextElem::style`; o itálico simplesmente herda o valor da cadeia, cujo default é
>   `FontStyle::Normal`. O *efeito observável* coincide (um heading não sai em itálico por
>   defeito), mas o mecanismo difere: no vanilla é herança, no cristalino é imposição.
>   **ACHADO ESCALADO — medido (2026-08-13).** Documento
>   `#set text(style: "italic")` + `= Cabecalho`:
>
>   | Binário | Fonte usada no heading |
>   |---|---|
>   | Vanilla `/usr/local/bin/typst` (`typst 0.15.1 (e0e8ca4d)`) | `LibertinusSerif-BoldItalic` (`pdffonts`) — **herda o itálico** |
>   | Cristalino `target/release/typst` (fonte em HEAD `4f64e4e69`) | operador de fonte `/F2 15.4 Tf`, **idêntico** ao do mesmo documento sem `#set text(style: "italic")` — o itálico envolvente não tem efeito |
>
>   **Controlo de confound**: os dois binários não resolvem para a mesma família (o vanilla
>   embute Libertinus Serif; o cristalino caiu nas base-14 não embutidas), pelo que **não**
>   se comparam nomes de fonte entre binários. A conclusão vem de duas observações
>   independentes de confound: (i) no vanilla, o nome da fonte contém `Italic`; (ii) no
>   cristalino, o operador `Tf` do heading é bit-a-bit o mesmo com e sem o `set text`
>   envolvente — isto é, comparação do cristalino **consigo próprio**, onde a fonte é a
>   mesma dos dois lados.
>
>   Corrigir isto é mudança de comportamento por defeito → gate ADR-0127 e passo próprio.
>   **Não implementado aqui.**
2. Se `cursor_x` já passou da margem, fazer `flush_line()`.
3. Se `heading.numbering` na `StyleChain` for `Bool(true)`:
   - Ler `heading.numbering.pattern` (opcional, `EcoString`).
   - Obter os valores brutos do counter via `Introspector::counter_values_at("heading", loc)`.
   - Se houver pattern, formatar com `format_counter(values, pattern)`. O resultado é
     usado **verbatim** — o pattern já transporta toda a pontuação — e o separador para o
     body é um espaço simples (P1036).
   - Se não houver pattern, usar `Introspector::formatted_counter_at("heading", loc)` (forma legada `"1.2.3"`), com o separador histórico `". "` (P1036).
   - Renderizar o prefixo numérico como `Content::text("{num_str}{sep}")` antes do body.
4. Renderizar o `body`.
5. Restaurar estilo anterior.

## Gate de numeração

- A numeração é activa quando `chain.custom("heading.numbering") == Some(Value::Bool(true))`.
- O pattern é transportado em `chain.custom("heading.numbering.pattern")` como `Value::Str`.

## Patterns suportados

Ver prompt `entities/counter_format.md`. Subset: `"1."`, `"1.1"`, `"I."`, `"(a)"`, `"A."`.

## Scope-outs

- Alinhamento customizado do número (left/center/right/hanging indent).
- Formatação de counter via `counter.display` callback (P241) — este layout usa formatação directa.

## Tests canónicos

- Heading sem numbering: sem prefixo.
- Heading numerado com pattern `"1."`.
- Heading nível 2 com pattern `"1.1"` produz `"1.1"`.
- Reset inferior: heading nível 1 após nível 2 volta a incrementar nível 1.
- Pattern romano `"I.I"` hierárquico.

---

## Resultado esperado

- `01_core/src/compiler/layout/heading.rs` — free function `layout` + tests em `01_core/src/compiler/layout/tests.rs`.

## P978 — escala de heading do vanilla (1.4/1.2/1.0 em relativo) e composição com `#set text`

**Data:** 2026-08-05

**Medição que motiva** (achado incidental de P975 — as letras de maior
|dx| nas secções 4/25/28 eram de headings): duas divergências —

1. **Factores**: `heading_scale` (`layout/helpers.rs`) usava
   2.0/1.667/1.333/1.167 (escala tipo HTML). O vanilla
   (`lab/typst-original/crates/typst-library/src/model/heading.rs:281-285`)
   usa **1.4 para nível 1, 1.2 para nível 2, 1.0 para nível 3+** — medido:
   15.4/13.2/11.0pt sobre corpo de 11pt.
2. **Composição**: o vanilla define o tamanho do heading como estilo de
   chain em **em relativo** (`TextElem::size = Em(scale)`), que compõe com
   `#set text(size:)` (multiplica o tamanho corrente — medido: 9pt set →
   heading L2 a 10.8pt). O cristalino calculava o tamanho em
   `heading.rs` (`layouter.style.size × scale`) mas o merge de
   `layout/text.rs` (`ns_size.unwrap_or(layouter.style.size)`) deixava o
   canal custom do `#set` **sobrepor-se** ao tamanho deliberado do heading
   — com `#set text(size:)` o heading saía ao tamanho do corpo (medido:
   11.0pt em vez de 13.2pt com set de 11pt; 9.0 em vez de 10.8 com set de
   9pt). A regra documentada do merge é "a chain tipada (heading) vence" —
   estava invertida para `size`.

**Decisão**: `heading_scale` passa a 1.4/1.2/1.0 (nível 1/2/3+) e o merge
de tamanho em `text.rs` passa a usar `layouter.style.size` (que já inclui
o `#set` via sync da chain — verificado por instrumentação) — o tamanho
deliberado de features de layout (heading, super/subscrito) volta a
vencer, como a regra documentada manda. **Nota**: o merge também afecta
super/subscritos de texto (`text.rs` escala `layouter.style.size`
directamente) — ficam igualmente corrigidos pela inversão.

**Residual registado** (fora do escopo deste passo): o espaçamento
acima/abaixo do heading no vanilla é `1.8em` (L1) / `1.44em` (L2+) ÷
escala acima e `0.75em` ÷ escala abaixo (mesmo sítio, heading.rs:288-289)
— não medido nem portado aqui.

## P1036 — o número do corpo perdia os níveis; e a premissa sobre o outline, refutada

**Data:** 2026-08-13 · **Proveniência:** `HEAD = 0c8b64a41` (P1033), árvore com edições só
em `00_nucleo/prompts/**`; vanilla `/usr/local/bin/typst`
(md5 `36da18895eeb5e0136c068a7634e3f82`); cristalino `target/release/typst` reconstruído de
`0c8b64a41`. Medições 17:30–17:45 -03:00. Tabela por pattern em
`entities/counter_format.md` §P1036.

**Causa** — a formatação hierárquica em si (`format_counter`); corrigida nesse nó. Este
ficheiro tinha uma segunda metade do defeito, independente: o sufixo era escolhido por uma
heurística (`used_pattern`) que recomputava `values.len() >= nº de tokens` e emitia `". "`
quando falhava. Com pattern `"1.1"` e um só valor, isso produzia `1. Alpha` onde o vanilla
produz `1 Alpha` — o pattern já transporta a pontuação, e acrescentar `". "` duplica-a. A
heurística desaparece: **há pattern → verbatim + `" "`; não há pattern → legado + `". "`**.

> **ACHADO ESCALADO — o outline ignora o pattern.** A premissa do enunciado de P1036 ("o
> outline acerta, só o corpo perde os níveis") é **verdadeira apenas para o pattern `"1."`**,
> e por coincidência: o outline não passa por `format_counter`, usa
> `Introspector::formatted_counter_at` (`compiler/introspect/heading.rs:34,56`), que junta os
> níveis com `"."` em arábico e acrescenta `"."` — sem ler o pattern. Medido no mesmo
> documento, `= / == / === / =`:
>
> | pattern | vanilla (outline) | cristalino (outline) |
> |---|---|---|
> | `1.` | `1.` / `1.1.` / `1.1.1.` | `1.` / `1.1.` / `1.1.1.` ✅ coincide |
> | `I.` | `I.` / `I.I.` / `I.I.I.` | `1.` / `1.1.` / `1.1.1.` ❌ |
> | `1.1` | `1` / `1.1` / `1.1.1` | `1.` / `1.1.` / `1.1.1.` ❌ |
> | `A.1.a` | `A` / `A.1` / `A.1.a` | `1.` / `1.1.` / `1.1.1.` ❌ |
>
> É superfície de linguagem (ADR-0107) → paridade. **Não corrigido em P1036**: o pattern não
> chega ao walk de introspecção — `ElementPayload::Heading` transporta `numbering_active:
> bool` mas não o pattern, e o bake da chain (`compiler/introspect.rs:1105-1108`) só lê o
> `Bool`. Levar o pattern até lá é **campo novo em `entities/element_payload.rs`** → contrato
> público, gate ADR-0127 ponto 1, passo próprio. **A guarda de não-regressão de P1036 é o
> pattern `"1."`**, o único onde o outline já batia com o vanilla — e continua a bater, por
> o outline não partilhar o caminho corrigido.

**Residual medido, pré-existente, não tocado**: o separador entre o número e o título é um
**espaço literal** no cristalino e **`0.3em` fraco** no vanilla. Medido por `pdftotext -bbox`
sobre `#set heading(numbering: "1.")` + `= Alpha` / `== Beta` (mesmo documento nos dois
binários):

| nível | corpo | vanilla (gap) | cristalino (gap) |
|---|---|---|---|
| 1 | 15.4pt | 4.62pt = `0.3 × 15.4` | 9.08pt |
| 2 | 13.2pt | 3.96pt = `0.3 × 13.2` | 10.98pt |

A coincidência exacta com `0.3 × tamanho` nos dois níveis identifica a regra do vanilla. É
**anterior a P1036** — o caminho de nível 1 com pattern `"1."` é idêntico antes e depois
deste passo (a heurística `used_pattern` dava `true`, logo `" "`, o mesmo separador que
agora) — e é geometria, não numeração: **passo próprio**.
