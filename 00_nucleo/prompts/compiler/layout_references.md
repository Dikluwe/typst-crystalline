# L0 — Layout: Referências e Labels
Hash do Código: 3eb508e5

## Módulo
`01_core/src/compiler/layout/references.rs`

## Propósito
Encapsula os braços `Ref` e `Labelled`. Resolve `@nome` (`Content::Ref`) usando
o `Introspector` fornecido ao Layouter. O caminho numérico P462 consulta
`Introspector::counter_key_for_label` + `query_by_label` e formata o número do
counter correspondente. O caminho legacy (`Content::Labelled`) continua a usar
`resolved_labels` / `figure_label_numbers` injectados pela introspecção
(Passagem 1). P463 envolve todo `Ref` num `FrameItem::Link` com destino
interno (`LinkTarget::Destination`), tornando a referência clicável no PDF.

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
- `Ref` a figure numerada via `Content::Label` → `"Figure 1"` (`en`) / `"Figura 1"` (`pt`) (P1073).
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
6. **Suplemento default de heading por língua** (vanilla: "Section 1" em
   docs `en` — medido; "Secção" hardcoded antes divergia): `en` →
   `Section `, `pt` → `Secção `, outras línguas → fallback `en`
   (limitação registada). Língua lida de `layouter.style.lang`.
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
