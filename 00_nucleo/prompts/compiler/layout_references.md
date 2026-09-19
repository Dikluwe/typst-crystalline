# L0 — Layout: Referências e Labels
Hash do Código: 790cd40e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/references/unreferencable-label.toml sha256:5dde6e7f84150a121e671dab0d174798e8b56f24355147c1bcc471d6babd69ed

## Módulo
`01_core/src/compiler/layout/references.rs`

## Propósito
Encapsula os braços `Ref` e `Labelled`. Resolve `@nome` (`Content::Ref`) usando
o `Introspector` fornecido ao Layouter. O caminho numérico P462 consulta
`Introspector::counter_key_for_label` + `query_by_label` e formata o número do
counter correspondente. O caminho legacy (`Content::Labelled`) continua a usar
`resolved_labels` / `figure_label_numbers` injectados pela introspecção
(Passagem 1). Supplements de figure, tanto no caminho numérico como no
fallback legacy, são obtidos do owner linguístico
`compiler/lang/figure_supplement.rs`; este módulo não mantém uma segunda
tabela ou bifurcação por língua. P463 envolve todo `Ref` num
`FrameItem::Link` com destino interno (`LinkTarget::Destination`), tornando a
referência clicável no PDF.

## Regras de negócio
- `Labelled { target, label }` → layout transparente do target, depois registo
  da página em `counter.label_pages`. O registo ocorre **depois** do layout
  porque o target pode forçar uma quebra de página.
- `Ref { target }`:
  1. Caminho P462 (`Content::Label`): se `counter_key_for_label` retornar uma
     chave, formata o número na `Location` da label usando o counter
     correspondente e prefixa `supplement` (explícito ou default do tipo).
  2. Fallback legacy: `figure_number_for_label(&label)` → `"Fig. {n}"`, ou
     `resolved_label_for(&label)` → texto resolvido (ex: `"Secção 1"`).
  3. Label não encontrada → renderizar `"?"`.
- `current_page_number()` no Layouter: `self.pages.len() + 1` (Abordagem A).

## Critérios de verificação
- `Ref` a heading numerado via `Content::Label` → número (ex: `"1"`) no
  plain_text.
- `Ref` a figure numerada via `Content::Label` usa o mesmo supplement
  localizado da legenda (`Figure`, `Figura`, `Abbildung`, `Tableau`, etc.) e
  junta-o ao número com NBSP.
- `Ref` a equation numerada → `"Equation 1"` (`en`) / `"Equação 1"` (`pt`) (P1073).
- `Ref` a table numerada → `"Table 1"`.
- Supplement explícito sobrepõe default.
- Label inexistente → `"?"`, sem panic.
- `Ref` produz `FrameItem::Link { target: LinkTarget::Destination(label), .. }`.
- `Content::Link` continua a produzir `LinkTarget::Url` (sem regressão).
- Label registada → `counter.label_pages` contém a chave após layout.
- Layout de label num elemento que força quebra de página → página registada
  é a do elemento, não a anterior.

---

## §P788 — Validação de refs (erros do vanilla) + precedência numérica + suplemento por língua

**Decisão (mensagens medidas no vanilla 0.15.0 por execução, 2026-07-20):**

1. **Label inexistente** → erro fatal de layout (via `layout_errors`):
   `` label `<{name}>` does not exist in the document `` — antes renderizava
   "?" em silêncio (P786 A10). Existência = qualquer caminho conhece o label
   (counter key / figure number / resolved legacy / query_by_label / unreferencable label registry).
2. **Ref a heading sem `numbering`** → erro:
   `cannot reference heading without numbering` + hint
   `` you can enable heading numbering with `#set heading(numbering: "1.")` ``
   — antes renderizava vazio em silêncio (P786 A8). A flag por Location vem
   do introspector (`heading_numbering`, ver introspector.md/introspect.md §P788).
3. **Ref a equation sem `numbering`** → erro:
   `cannot reference equation without numbering` + hint
   `` you can enable equation numbering with `#set math.equation(numbering: "1.")` ``
   (paridade vanilla 0.15.0). A flag por Location vem do introspector
   (`equation_numbering`, análoga a `heading_numbering`).
4. **Ref a label existente mas não referenciável** (texto simples, lista, raw,
   ou outro elemento sem counter/figure/resolução) → erro específico do tipo:
   - Texto / lista / enum / termo / conteúdo genérico → `cannot reference text`.
   - Bloco raw → `cannot reference raw directly, try putting it into a figure`.
   - Outros elementos não numeráveis → mensagem genérica equivalente ao vanilla.
   
   Estes labels são registados no `Introspector` como *unreferencable labels*
   durante o walk (ver §P856 abaixo).
5. **Precedência numérica sobre o legacy `Labelled`:** os dois
   `label_to_counter_key.remove()` (walk `Content::Label` + populate arm
   `Labelled`) são **removidos** — refs a heading/equation/table/figure
   numerados usam o caminho do counter (formatado), ficando o
   `resolved_labels` legacy como fallback de última linha.
6. **Suplemento default de heading por língua** (decisão original P788,
   supersedida por P1459): `en` → `Section `, `pt` → `Seção ` e outras
   línguas caíam em inglês, lendo `layouter.style.lang`. P1459 remove essa
   limitação e a segunda decisão linguística; ver secção própria abaixo.
7. **Spans:** `RefElem` não carrega span — os erros seguem com
   `Span::detached()` (limitação registada; vanilla aponta o `@ref`).

---

## §P856 — Registo de labels não referenciáveis (unreferencable labels)

**Motivação:** P856 valida que `#ref(<lbl>)` sobre uma label existente mas
associada a conteúdo não numerável (texto, raw, etc.) produz a mensagem de
erro do vanilla (`cannot reference text`, `cannot reference raw directly, ...`)
em vez de `label <lbl> does not exist in the document`.

**Decisão arquitetural:**

1. Durante o walk de introspecção (`compiler/introspect.rs`), quando o braço
   `Content::Label` não-auto encontra um `body` que **não** produz um Tag
   locatable (heading, figure, equation, table), registar o par
   `(Label, UnreferencableKind)` num novo sub-store do `TagIntrospector`:
   `unreferencable_labels: HashMap<Label, UnreferencableKind>`.
2. O enum `UnreferencableKind` (em `entities/label_kind.rs` ou dentro de
   `element_payload.rs`) tem pelo menos:
   - `Text` — texto simples, parágrafos, listas, enums, terms, etc.
   - `Raw` — blocos de código/raw.
   - `Other` — fallback para tipos não mapeados.
3. O trait `Introspector` expõe:
   ```rust
   fn unreferencable_label_kind(&self, label: &Label) -> Option<UnreferencableKind>;
   fn equation_has_numbering(&self, location: Location) -> Option<bool>;
   ```
4. `layout_ref` consulta `unreferencable_label_kind` **depois** de confirmar
   que a label não é referenciável pelos caminhos numéricos/legacy. Se
   `Some(kind)`, emite o erro correspondente em vez de
   `label <name> does not exist in the document`.
5. Labels em elementos numeráveis mas com numbering desactivado **não** entram
   em `unreferencable_labels`; são tratados pelas verificações específicas de
   `heading_has_numbering` e `equation_has_numbering`.

**Critérios de verificação:**
- `#ref(<lbl>)` sobre parágrafo `<lbl>` → `cannot reference text`.
- `#ref(<lbl>)` sobre lista `<lbl>` → `cannot reference text`.
- `#ref(<lbl>)` sobre raw `<lbl>` → `cannot reference raw directly, try putting it into a figure`.
- `#ref(<eq>)` sobre equation sem numbering → `cannot reference equation without numbering` + hint.
- `#ref(<h>)` sobre heading sem numbering continua a funcionar (sem regressão).
- `#ref(<h>)` sobre heading com numbering continua a funcionar (sem regressão).
- Label genuinamente inexistente continua a dar `label <x> does not exist in the document`.

## P1140.4-C — suplemento próprio da equação

### Medição antes da decisão

No vanilla, `reference.rs:341-355` dá precedência ao suplemento explícito da
ref; `none`/vazio omite prefixo; não vazio junta com NBSP. Probes produziram
`Equation 1`, `2`, `3`, `Eq. 4` e `FUN 5`; `@eq[Ref]` sobre elemento `Elem`
produziu `Ref 1`. O cristalino hoje ignora suplemento do alvo e deriva default
da língua corrente (`references.rs:235-249`).

### Decisão

Para Equation, `resolve_ref_text` consulta primeiro o suplemento explícito do
`RefElem`; se ausente, consulta `equation_supplement_content(loc)`. Conteúdo
vazio gera apenas número; conteúdo não vazio usa `plain_text`, U+00A0 e o
número formatado. O fallback genérico por língua deixa de ser fonte quando o
sub-store tem entrada. Referência sem numbering falha antes. O layout não
chama callbacks nem relê locale.
## P1140.25 — `ref(form: "page")`

A forma Page consulta numbering/supplement selados da página do alvo, aplica a
precedência explícita, compõe NBSP somente com supplement não vazio e mantém o
link interno. Ver `entities/page_supplement.md`.

## P1157 — referência de página usa vista unária

### Medição antes da decisão

Com footer explícito, callback variádico produziu `R1` na referência; callback
binário falhou por falta de `total`. Assim referência chama exatamente com o
número lógico corrente.

### Decisão

`ref(form: "page")` consome `reference_numbering` já realizado no PageStore e
nunca chama Func durante layout. Pattern usa o mesmo slot, realizado pelo owner
compartilhado com `[current]`. Supplement explícito mantém precedência e NBSP;
conteúdo vazio omite só o número, não o link.

## P1458 — remover o hardcode bilingue de referências de figure

### Medição anterior à decisão

No commit `f6463768d8bb093a59577ff6f3262a9c5f8eb448`, a fixture pública com
`#set text(lang: "de")`, uma figure `image` numerada e `@abbildung` produziu
`Abbildung 1: Bild` na legenda nos dois produtos. A referência produziu
`Abbildung 1` no vanilla ratificado `586e1bd43`, mas `Figure 1` no cristalino.
O consumer medido em `references.rs:391-411` continha uma segunda decisão
`is_pt`: português recebia tradução e qualquer outra língua recebia inglês,
apesar de `figure_supplement_for_lang` já ser o owner das 18 combinações
legitimadas por `compiler/lang/figure_supplement.md`.

### Decisão

Para toda `CounterKey::Str("figure:{kind}")`, o supplement default é
`figure_supplement_for_lang(kind, lang)`. O fallback legacy de figure usa o
mesmo owner com kind `image`. Supplement explícito conserva precedência e o
join conserva U+00A0. É proibido manter ou recriar neste módulo catálogo,
`is_pt`, match de kind, texto Figure/Figura/Table/Listing ou fallback próprio
para figures.

### Aceitação

- em `de`, legenda e referência de image dizem `Abbildung 1`;
- `figure:table` e `figure:raw` observam as traduções do owner para as línguas
  já cobertas;
- `en`, `pt`, supplement explícito, número, NBSP, link e erros permanecem;
- língua/kind não cobertos preservam o fallback definido exclusivamente pelo
  owner linguístico.

## P1459 — remover o hardcode bilingue de referências de heading

### Medição anterior à decisão

No commit `c7577a031847ff38e4780044bc4132c1d92a14b9`, um heading alemão
referenciado sob texto francês produziu `Abschnitt 1` no vanilla ratificado
`586e1bd43` e `Section 1` no cristalino. Trocar apenas o branch por uma chamada
com `layouter.style.lang` continuaria errado: o vanilla conserva a língua do
alvo. A introspecção já guarda em `resolved_labels` a forma construída quando
a língua causal do alvo existe.

### Decisão

Para `CounterKey::Heading` com supplement implícito, recuperar de
`resolved_label_for(label)` o prefixo causal somente quando a representação
termina exatamente em espaço ASCII + o mesmo counter formatado. O número
continua vindo do caminho numérico; a composição pública continua no join
único com U+00A0. Supplement explícito, inclusive conteúdo vazio, tem
precedência absoluta e não consulta o store.

Se um introspector sintético ou caminho legacy não possuir transporte causal,
usar defensivamente `heading_supplement_for_lang(layouter.style.lang)`; essa
queda não autoriza alegar língua causal para um alvo cuja cadeia não foi
armazenada. É proibido manter ou recriar neste módulo tabela, `is_pt`, branch
por língua ou literais `Section`/`Seção`/`Abschnitt`.

### Aceitação

- alvo `de` sob referência `fr` → `Abschnitt 1`;
- alvo `fr` sob referência `de` → `Section`, nunca `Chapitre`;
- alvos `es` e `it` preservam `Sección` e `Sezione` sob outras línguas;
- forward e backward refs coincidem;
- supplement explícito e vazio conservam precedência;
- o separador público continua U+00A0;
- ausência do store mantém fallback determinístico sem panic;
- snapshot/query, transporte labelled e layout consomem um único owner.
