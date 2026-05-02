# Prompt L0 — layout
Hash do Código: 10004310

## Módulo
`01_core/src/rules/layout.rs`

## Propósito
Converte `Content` em `PagedDocument` com word-wrap e paginação básica.
Usa métricas monoespaçadas fixas (`FixedMetrics`) injectáveis via trait
`FontMetrics` — substituíveis por `FontBookMetrics` no Passo 20.

## Restrição arquitectural
Não depende de L3. Métricas de fonte reais (FontBook) são injectadas
por trait, não importadas directamente. `layout()` compila e testa em L1.

## Tipos e interface

### `FontMetrics` trait
```rust
pub trait FontMetrics {
    fn char_width(&self, c: char) -> Pt;
    fn line_height(&self) -> Pt;
    fn font_size(&self) -> Pt;
}
```

### `FixedMetrics`
Monoespaçado: `char_width = size * 0.6`, `line_height = size * 1.2`.

### `Layouter<M: FontMetrics>`
Máquina de estado: cursor_x, cursor_y, current_line buffer, paginação.

### `layout(content: &Content) -> PagedDocument`
API pública — usa `FixedMetrics::new(12.0)`.

## Comportamento
- `Content::Empty` → zero páginas
- Word-wrap: quebra quando palavra ultrapassa `page_width - MARGIN`
- Paginação: nova página quando `cursor_y > page_height - MARGIN`
- `flush_line()` move `current_line` para o frame actual
- `finish()` faz flush final e descarta página vazia

### Hyphenation (Passo 144, ADR-0057)

Quando uma palavra não cabe na linha actual e `style.lang` é
`Some(lang)`, `layout_word` invoca o helper puro
`hyphenation::hyphenate(word, &lang)` antes de fazer flush. O
helper devolve `Vec<usize>` de pontos de quebra (em chars) na
palavra. O algoritmo greedy tenta cada ponto da maior para a
menor; o primeiro prefixo (com hífen literal `-`) que cabe no
espaço disponível é emitido, seguido de `flush_line` e
recursão com o sufixo restante. Se nenhum ponto cabe ou
`style.lang` é `None`, comportamento pré-144 preservado
(palavra inteira para a linha seguinte).

`hyphenation::hyphenate` é wrap puro sobre `hypher::hyphenate`
(crate autorizada em `[l1_allowed_external]` por ADR-0057;
padrões TeX embebidos em compile-time, sem I/O). Política de
fallback:
- Idioma com código ISO de 3 letras (ISO 639-2/3) → vazio.
- Idioma não suportado pelo `hypher` → vazio.
- Palavra sem pontos de quebra (uma sílaba) → vazio.

Em todos os casos de fallback, a palavra inteira passa para a
linha seguinte como antes — silent skip por consistência com
a política de fallback de fonts (ADR-0055 decisão 5).

`lang` continua **parcialmente** scope-out per ADR-0054
(perfil observacional graded): hyphenation existe; shaping
features (ligatures, kern, bidi via rustybuzz) permanecem
ausentes. DEBT-53 candidato XL futuro endereça shaping.

## Critérios de verificação
- `layout(&Content::Empty).pages.is_empty()`
- `layout(&Content::text("Hello world")).plain_text()` contém "Hello" e "world"
- 100 palavras → todos os items dentro dos limites da página (x<595, y<842)
- 50 palavras → múltiplas linhas (y_values.len() > 1)
- Pipeline parse→eval→layout sem crash

## Secção: Referências e Contadores Automáticos (Passo 59)

### Resolução Single-Pass
O Layouter executa numa única passagem. O `CounterState` acumula
`resolved_labels: HashMap<Label, String>` à medida que avança.

- **Labelled**: não tem presença visual. Side-effect: insere no dicionário
  o texto formatado do contador actual (ex: `Label("intro") → "Secção 1.1"`).
  O registo acontece **depois** de `layout_content(target)` para garantir que
  o contador do alvo (ex: Heading) já avançou antes de ser lido.
- **Ref**: consulta o dicionário. Encontrou → desenha o texto resolvido.
  Não encontrou → fallback literal `@nome` (DEBT-10: referências para a frente).

### Auto-numeração
- `Equation { block: true, .. }`: se `numbering_active["equation"]` for
  verdadeiro, avança `step_flat("equation")` antes de desenhar e adiciona
  o número formatado `(N)` à direita da equação.
- `Figure`: variante não existe em `Content` — auto-numeração de Figure
  registada em DEBT-10, será adicionada nos Passos 60+.

### Limitação conhecida (DEBT-10)
Referências para a frente (a label aparece depois da Ref no documento)
não são resolvidas nesta passagem — exigem o motor de introspecção de
duas passagens (Passos 60+).

### Critérios adicionais de verificação (Passo 59)
- `Labelled(Heading, label)` → `resolved_labels` contém a chave após layout.
- `Ref(label)` para trás → plain_text contém o texto resolvido.
- `Ref(label)` para a frente → plain_text contém `@nome` (não panic).
- `Equation { block: true }` numerada → número aparece no documento.

## Secção: Cite-arm consome Introspector (P181G)

Cite-arm de `Content::Cite { key, supplement, form }` em
`layout/mod.rs:584-597` consulta `Introspector` primeiro
(`bib_entry_for_key`, `bib_number_for_key`) com fallback
**substitution-with-fallback** a `self.counter.bib_*` legacy
(padrão P168 figure-ref):

```rust
let entry = self.introspector
    .bib_entry_for_key(key)
    .or_else(|| self.counter.bib_entries.iter().find(|e| e.key == *key));

let number = self.introspector
    .bib_number_for_key(key)
    .or_else(|| self.counter.bib_numbers.get(key).copied());
```

Comportamento por path:
- **`layout()` legacy** invoca `layout_with_introspector(_, _,
  TagIntrospector::empty())` — Introspector vazio, fallback a state
  legacy serve as 4 cite forms (Normal/Prose/Author/Year). Backward
  compat preservado.
- **`layout_with_introspector(content, state, introspector)`** usa
  Introspector populado por `from_tags` (P181E). State legacy
  preservado paralelamente durante janela compat.

Paridade `BibStore` ↔ `state.bib_*` garantida por construção
(P181E §6 — mesma lógica replicada). Output observable inalterado.

Janela compat eliminada em **M6** quando F1 retomar
(`CounterStateLegacy.bib_entries`/`bib_numbers` removidos +
copy-sites em `pub fn layout`/`pub fn layout_with_introspector`
desaparecem + fallback removido).

## Secção: `layout()` legacy injecta Introspector populado (P181H)

Pré-P181H, `layout()` era thin wrapper sobre `layout_with_introspector`
com `TagIntrospector::empty()` — funcionava porque cite-arm consumia
`state.bib_*` legacy (populado por walk arm `Content::Bibliography`).

Pós-P181H, walk arm `Content::Bibliography` ficou puro (P163 invariante
restaurada). `state.bib_*` é vazio em produção. Para preservar
funcionalidade bib em path `layout()` legacy, `layout()` re-corre
`introspect_with_introspector(content, None, None)` internamente para
obter `Introspector` populado; descarta o `state` retornado e usa o
`initial_state` passado pelo caller (mantém backward compat de
fields não-bib que walk continua a popular):

```rust
pub fn layout(content: &Content, initial_state: CounterStateLegacy) -> PagedDocument {
    let (_, intr) = introspect_with_introspector(content, None, None);
    layout_with_introspector(content, initial_state, intr)
}
```

**Custo**: walk extra (caller já fez 1 walk via `introspect()`).
Aceitável — bib feature é raramente usada e o custo extra é
trivial para documentos sem `Content::Bibliography`. Trabalho extra
é cobrado só quando bib está activa.

**Outros sub-stores do introspector** (figure_label_numbers, metadata,
state) ficam também populados — `layout()` legacy ganha acesso
implícito a queries Introspector que outros consumers M5+ podem
adoptar quando migrarem.

**M6** elimina este re-walk: quando callers adoptarem
`introspect_with_introspector + layout_with_introspector` directamente,
`layout()` legacy desaparece (pode passar a wrapper trivial sobre o
entry point novo, ou ser removido).

## Secção: Heading-arm + equation-arm consomem Introspector (P182D)

Heading-arm de `Content::Heading { level, body }` em `layout/mod.rs:301`
e equation-arm de `Content::Equation { body, block }` em
`layout/equation.rs:24` consultam `Introspector::is_numbering_active`
primeiro com fallback **substitution-with-fallback** a
`self.counter.is_numbering_active(legacy_key)` (padrão P168/P181G).

```rust
// Heading prefix (mod.rs:301)
let on = self.introspector
    .is_numbering_active("numbering_active:heading")
    || self.counter.is_numbering_active("heading");
if on {
    if let Some(num_str) = self.counter.format_hierarchical("heading") { ... }
}

// Equation auto-numeração (equation.rs:24)
let is_numbered = block
    && (self.introspector.is_numbering_active("numbering_active:equation")
        || self.counter.is_numbering_active("equation"));
```

**Convenção de chave**: Introspector usa `numbering_active:<feature>`
(prefixo namespace, P182B); legacy `CounterStateLegacy.is_numbering_active`
usa key sem prefixo (`"heading"`, `"equation"`).

**Estado dos emitters** (P182C):
- `numbering_active:heading` é populado em `StateRegistry` via
  `extract_payload` arm `Content::SetHeadingNumbering` →
  `from_tags::StateUpdate` com auto-init.
- `numbering_active:equation` **não tem emitter em P182** (cristalino
  não tem `Content::SetEquationNumbering` variant). Introspector
  retorna sempre `false` para esta chave; fallback legacy é o caminho
  real até passo dedicado equation-set-rule (fora P182).

Comportamento por path (heading):
- **`layout()` legacy**: `layout()` re-corre `introspect_with_introspector`
  internamente (cf. secção P181H) — Introspector populado via P182C.
  Fallback legacy também populado via walk canonical
  (`introspect.rs:455–457`); paridade preservada por construção.
- **`layout_with_introspector(content, state, introspector)`**: caller
  passa Introspector populado; fallback continua disponível como rede
  de segurança.

Output observable preservado: para heading, ambos caminhos devolvem
mesmo bool (StateRegistry e legacy populados pelo mesmo `Content::SetHeadingNumbering`);
fallback é redundante mas inofensivo. Para equation, fallback é o
único path activo.

Janela compat eliminada em **M6** quando F1 retomar
(`CounterStateLegacy.numbering_active` removido + walk arm canonical
+ write paralelo `layout/counters.rs:11–13` + copy-sites em
`mod.rs:1414, 1442` desaparecem + fallback removido).
