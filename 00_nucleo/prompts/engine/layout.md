# Prompt L0 — layout
Hash do Código: c4936ac9

## Módulo
`01_core/src/engine/layout/mod.rs` e sub-módulos (`metrics.rs`, etc.)

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
- `Content::Parbreak` → `flush_line()` no ponto onde ocorre, separando os
  parágrafos visualmente (avanço vertical por `line_height + leading`).
- Word-wrap: quebra quando palavra ultrapassa `page_width - MARGIN`
- Paginação: nova página quando `cursor_y > page_height - MARGIN`
- `flush_line()` move `current_line` para o frame actual
- `finish()` faz flush final e descarta página vazia

### P867 — `#set page(height: auto)` / `width: auto`

`#set page` pode especificar `auto` para `width` e/ou `height`. Neste caso,
a página cresce ao longo do eixo correspondente para acomodar o conteúdo,
desactivando a quebra automática nesse eixo:

- `height: auto` — `page_bottom_limit()` retorna `f64::INFINITY`, pelo que
  `flush_line()` nunca dispara `new_page()` por overflow vertical. A altura
  final da página é calculada em `finish()`/`new_page()` como
  `cursor_y + margin` (aproximação do fundo do conteúdo).
- `width: auto` — `available_width()` e o limite direito da linha são
  `f64::INFINITY`, pelo que `layout_word()`/`layout_chunk()` nunca quebram
  linha. A largura final é calculada em `finish()`/`new_page()` a partir da
  extensão horizontal real dos items (`line_content_right + margin`).
- `width: auto` / `height: auto` combinados — página única que cresce nos
  dois eixos.
- Dimensões `auto` são representadas internamente por `f64::INFINITY` em
  `PageConfig.width`/`height`. `PageConfig::auto_margin()` usa a dimensão
  finita restante (ou largura A4 como fallback se ambas forem infinitas).
- Casos limite defensivos:
  - `h(Nfr)` e alinhamento RTL não expandem quando `width: auto` (faltam
    referencial de espaço restante).
  - Floats bottom-aligned decaem para top-aligned quando `height: auto`.
  - Footnotes e numeração de página usam as dimensões reais calculadas
    antes do flush final.

### Avanço vertical e cálculo de line advance (Passo 579 / P762)

Para suportar parágrafos com fontes de tamanhos mistos ou tamanhos diferentes do padrão do documento, o Layouter deve calcular o avanço do cursor vertical (`cursor_y`) de forma dinâmica. A partir de **P762**, o avanço de linha não usa o antigo `line_height = ascender + descender + lineGap`; em vez disso, usa o modelo do vanilla:

```text
line_advance = top_edge + |bottom_edge| + leading
```

onde `top_edge` e `bottom_edge` são offsets medidos a partir da baseline
(defaults vanilla: `top-edge: "cap-height"`, `bottom-edge: "baseline"`), e
`leading` default é `0.65em` quando não está explicitamente definido.

1. **Cálculo em `flush_line()`**:
   - O estilo que governa o avanço (`max_style`) é o do item de texto com o
     maior `style.size` na `current_line` a ser drenada.
   - O avanço vertical é `FontMetrics::text_edges(max_font_size, &max_style)`,
     que devolve `(top, bottom)`, somado a `leading` resolvido em pontos do
     próprio `max_style.size`.
   - Se `current_line` estiver vazia, `flush_line` é um no-op vertical.
   - **P813** — quando há items, `flush_line` regista o avanço aplicado no
     campo `last_flush_advance` do Layouter (reset em `new_page`;
     save/restore em `layout_sub_frame`). Consumidor actual: o layout de
     equações de bloco (`engine/layout/equation.rs`), que recupera a
     baseline da linha anterior como `cursor_y - last_flush_advance` para
     posicionar a baseline da equação por `prev_baseline + spacing +
     ascent_ink` (ver `engine/layout/equation.md`).
   - O `line_leading_pt` de cada elemento de texto na linha deve ser resolvido
     usando o tamanho de fonte do próprio elemento (`style.size`) em vez da
     constante base do documento.

2. **Inicialização do Cursor (Página e Coluna)**:
   - Ao iniciar uma nova página ou coluna, o deslocamento inicial do cursor
     (`top_edge`) deve ser calculado usando o tamanho de fonte activo
     (`self.style.size`) do Layouter e `FontMetrics::text_edges`, garantindo
     que a primeira baseline do texto fique a `margem + top_edge`. Isto
     alinha-se com o vanilla, onde `top-edge: "cap-height"` é o default do
     texto.

3. **`text(top-edge: ..., bottom-edge: ...)`**:
   - Os campos `top_edge` e `bottom_edge` fazem parte de `TextStyle` e
     propagam-se pelo `StyleChain`/`StyleDelta`.
   - **P837** — o tipo dos campos é `Option<TextEdge>`
     (`entities/layout_types.rs`), espelho dos enums `TopEdge`/`BottomEdge`
     do vanilla (`text/mod.rs:1161-1248`): `TextEdge::Metric(EcoString)`
     (métrica nomeada) ou `TextEdge::Length(Length)` (comprimento explícito
     a partir da baseline, resolvido no font-size).
   - O eval de `#set text(top-edge: ...)` / `#set text(bottom-edge: ...)`
     valida o domínio enumerado e rejeita o resto com erro hard verbatim
     do vanilla (contrato de erros em `engine/eval.md` §P837). Domínios:
     top = `"ascender"`, `"cap-height"`, `"x-height"`, `"baseline"`,
     `"bounds"`; bottom = `"baseline"`, `"descender"`, `"bounds"`.
   - Implementações de `FontMetrics` devem mapear as métricas nomeadas para
     offsets `(top, bottom)` medidos a partir da baseline; `TextEdge::Length`
     resolve directamente (`top = length.resolve_pt(size)`; bottom com sinal
     negativo = abaixo da baseline, logo `bottom = length.resolve_pt(size)`
     — paridade `FontInstance::edges`, vanilla `text/font/mod.rs:276-289`).


### Decoração textual wrap-aware (Passo 286 — fecha cluster P284-P285-P286)

O consumer Layouter de `Content::Underline`/`Strike`/`Overline`
(P284 §2.3, estendido em P285 §3.3 herança stroke, **finalizado em
P286 §3.6 wrap**) emite **N `FrameItem::Line` por decoração** quando
o body faz wrap em N linhas visuais. Mecanismo per diagnóstico P286:

1. **Campo opcional no Layouter** —
   `decoration_lines_collector: Option<Vec<DecoSegment>>` (None por
   default — zero overhead nos outros call-sites).
2. **Hook em `flush_line`** — se o collector está activo e há items
   pendentes, regista `DecoSegment { start_x: line_start_x,
   end_x: cursor_x, baseline_y: cursor_y }` **antes** do drain.
3. **Consumer P284** — snapshot inicial + activa collector + recurse
   no body + drena vec + acrescenta segment "final não-flushed" +
   emite 1 `FrameItem::Line` por segment (cor uniforme via
   `stroke.or(style.fill)`; extent simétrico em ambos os lados de
   cada linha per P286 §A.3 opção α).
4. **Fallback single-line bit-exact** — body que não causa flush
   produz 1 segment ("final não-flushed"); coerente com algoritmo
   P284 original.

Cluster decorações **COMPLETO**:
- P284: variants + native_* + Layouter consumer single-line + emit reusa `FrameItem::Line`.
- P285: `stroke` activado (emit `RG` + herança `style.fill`).
- P286: wrap-aware (N segments → N Lines visuais).

Restrição graded P284 §5.3 **RESOLVIDA**. ADR-0054 graded preservado
para `evade`/`background`/objecto Stroke rico.

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

### Segmentação de linha para scripts sem espaços (P756)

Texto em chinês, japonês, tailandês, laosiano, birmanês e khmer não
usa espaços entre palavras. Tratá-lo como uma única palavra
indivisível resulta em quebras incorrectas ou truncamento na margem.
O Layouter deve segmentar esses runs antes de os passar a
`layout_word`.

#### Algoritmo

1. **Detectar o script/idioma do run** a partir de
   `self.style.lang` (código ISO) ou, em ausência de `lang`, pela
   análise dos codepoints do run (scripts CJK, Thai, Lao, Myanmar,
   Khmer).

2. **Obter oportunidades de quebra** com `icu_segmenter`
   (`LineSegmenter::new_lstm` ou `LineSegmenter::new_auto`), usando os
   dados compilados em build-time (`compiled_data`). Esta crate é
   I/O-livre em tempo de execução: os dados são constantes Rust
   embutidas no binário. Por isso pode residir em L1, declarada em
   `[l1_allowed_external]`.

3. **Tailoring de aspas para chinês/japonês**: replicar o
   `CJ_SEGMENTER` do vanilla, que sobrescreve as propriedades
   `LineBreak` de `U+201C` (`“`) para `OP` e `U+201D` (`”`) para `CP`,
   impedindo que aspas de abertura fiquem no início de linha e aspas
   de fecho no fim de linha. Pode ser feito ou por um blob de dados
   customizado estático, ou por pós-processamento dos breakpoints do
   segmentador geral.

4. **Fragmentar o run** nos breakpoints permitidos e chamar
   `layout_word`/`layout_chunk` para cada fragmento, preservando o
   tratamento existente de espaços, hyphenation e decorações.

5. **Fallback**: se `icu_segmenter` não estiver disponível ou falhar
   para um run específico, o comportamento pré-P756 é preservado
   (palavra inteira), sem panic.

#### Restrições

- A segmentação é pura: nenhum I/O, nenhum estado global, nenhuma
  chamada a `std::env` ou `SystemTime`.
- A dependência `icu_segmenter` fica em L1 (computação pura sobre
  Unicode). `icu_provider_blob` só é usada se o blob customizado for
  carregado via L3; caso contrário, o tailoring pode ser feito em L1
  por pós-processamento dos breakpoints.
- O texto latino e outros scripts com espaços continuam a usar o
  mecanismo `split(' ')` + `layout_word` sem regressão.

## Critérios de verificação
- `layout(&Content::Empty).pages.is_empty()`
- `layout(&Content::text("Hello world")).plain_text()` contém "Hello" e "world"
- 100 palavras → todos os items dentro dos limites da página (x<595, y<842)
- 50 palavras → múltiplas linhas (y_values.len() > 1)
- Dois parágrafos separados por `Content::Parbreak` produzem pelo menos 2
  linhas visuais distintas (y diferentes)
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

## Secção: Figure-arm consome Introspector (P184D)

Figure-arm de `Content::Figure { body, caption, kind, numbering }` em
`layout/mod.rs:435–439` consulta
`Introspector::figure_number_at_index(kind_key, idx)` primeiro com
fallback **substitution-with-fallback** a
`self.counter.figure_numbers.get(kind_key).and_then(|v| v.get(idx)).copied()`
legacy + `unwrap_or(idx + 1)` defensivo final (padrão P168/P181G/P182D
estendido com camada extra dado o fallback heurístico pré-existente):

```rust
let figure_number = self.introspector
    .figure_number_at_index(kind_key, idx)
    .or_else(|| self.counter.figure_numbers
        .get(kind_key).and_then(|v| v.get(idx)).copied())
    .unwrap_or(idx + 1);
```

**Convenção de chave**: Introspector resolve internamente
`format!("figure:{}", kind_key)` (P184B); Layouter passa `kind_key`
sem prefixo (`"image"`, `"table"`). Default `kind_key = "image"`
quando `kind: None` é responsabilidade do caller (Layouter, linha 431
`kind.as_deref().unwrap_or("image")`).

**Idx 0-indexed em ambos paths**: `figure_progress` no Layouter
inicializa em 0, incrementa após cada figure numerada; legacy
`figure_numbers[kind][idx]` faz acesso `Vec::get` 0-indexed;
Introspector `value_at_index(key, idx)` faz `history.get(key)?.get(idx)`
0-indexed. Sem deslocamento entre paths.

**Comportamento por path**:
- **`layout()` legacy**: `layout()` re-corre `introspect_with_introspector`
  internamente (cf. secção P181H) — Introspector populado via P184B arm
  Figure (`apply_at("figure:{kind}", Step, loc)` para cada figure).
  Fallback legacy: `state.figure_numbers` é populado em walk
  (`introspect.rs:391–399`) mas **nunca copiado ao Layouter** (achado
  P184A §3.6 — copy-sites `mod.rs:1414, 1442` não copiam o campo).
  Em produção, fallback legacy retorna sempre `None` → Introspector
  path activo é o caminho real após P184D.
- **`layout_with_introspector(content, state, introspector)`**: caller
  passa Introspector populado; mesmo comportamento.

**Paridade output**: counter flat é incrementado no walk legacy só
para figures `is_counted` (numbering+caption, `introspect.rs:387`)
enquanto Introspector P184B incrementa para **toda** figure
(`extract_payload.rs:33` define `counter_update: Step` incondicional).
Layouter idx conta figures `numbering.is_some()` (sem exigir caption,
`mod.rs:430`). Em produção típica (numbering+caption juntos) o offset
coincide e ambos paths retornam `idx + 1`. Casos limite (numbering sem
caption ou vice-versa) convergem na heurística `unwrap_or(idx + 1)`
final que ambos paths originam.

Janela compat eliminada em **M6** quando F1 retomar
(`CounterStateLegacy.figure_numbers`/`local_figure_counters` removidos
+ walk arm canonical legacy + chave global `"figure"` paralela em
`from_tags` arm Figure desaparecem + fallback removido).

## P185C — Locator + current_location (mecanismo M3 de ADR-0068)

`Layouter` ganha dois fields para suportar consumers
location-aware (P187 C1, P188 C2):

```rust
locator:          Locator,
current_location: Option<Location>,
```

- `Locator::new()` em `Layouter::new()` — determinismo
  (provado em P185A §3.3) garante sincronização-por-construção
  com o `Locator` do walk de introspect, sem partilha por
  referência.
- `current_location: None` antes de processar qualquer
  conteúdo locatable. Após o primeiro `is_locatable(content)`
  arm, `Some(loc)` reflecte a `Location` actual.
- Avanço monotónico (sem save/restore) — alinha com walk de
  introspect, que avança cumulativamente. Caller que precise
  de scoping léxico salva/restaura no seu próprio nível.

### Gating em `layout_content`

Padrão atómico, no topo do método antes do match:

```rust
pub fn layout_content(&mut self, content: &Content) {
    self.advance_locator_if_locatable(content);
    match content { /* ... */ }
}

fn advance_locator_if_locatable(&mut self, content: &Content) {
    if is_locatable(content) {
        self.current_location = Some(self.locator.next());
    }
}
```

Invariante: `is_locatable(c) == extract_payload(c).is_some()`
(garantida em `locatable.rs:11`) torna o gating do Layouter
isomorfo ao do walk de introspect (`introspect.rs:329`).
Consequência: `Locator::next()` é chamado nas mesmas posições
em ambos walks, produzindo a mesma sequência de `Location`s.

### Consumers (em P187/P188)

- **P187 (C1 heading prefix)**: `is_numbering_active_at(key,
  current_location)` em vez de `is_numbering_active(key)`
  (snapshot final).
- **P188 (C2 equation counter)**: `flat_counter_at("equation",
  current_location)` em vez de `state.get_flat("equation")`
  legacy.

Em P185C **nenhum consumer** migra — Layouter ganha apenas
infra. Output observable inalterado.

### `Locator` não-`Clone`

`Locator` é deliberadamente não-`Clone` (per `locator.rs:23`)
para preservar invariante de unicidade. `Layouter` por
consequência também não pode derivar `Clone`. Confirmado:
`Layouter` actual não deriva `Clone` (verificado em `.A`),
não há regressão de API.

## Secção: C1 heading prefix migrado (P187B)

Heading-arm em `layout/mod.rs:Content::Heading` consulta
`Introspector::formatted_counter_at("heading", current_location)`
primeiro para obter o prefixo numérico, com fallback
`substitution-with-fallback` a `self.counter.format_hierarchical("heading")`
legacy:

```rust
let num_str = self.current_location
    .and_then(|loc| self.introspector
        .formatted_counter_at("heading", loc))
    .or_else(|| self.counter.format_hierarchical("heading"));
if let Some(num_str) = num_str {
    let prefix = Content::text(format!("{}. ", num_str));
    self.layout_content(&prefix);
}
```

**P183B aprendizado retroactivamente validado**: P183B
falhou tentando substituir por `formatted_counter("heading")`
(snapshot-final P170) que pré-emptava fallback em sequências
re-update (`H1, H2, H1` produzia `"2.", "2.", "2."` em vez
de `"1.", "1.1", "2."`). P185 introduziu primitiva
location-aware `formatted_counter_at(key, location)` (P177)
e field `current_location: Option<Location>` no Layouter
(P185C). P187B finalmente fecha C1 com a primitiva correcta
— snapshot por Location é exactamente o valor que walk-during
legacy retornaria.

**Inversão observable**: P187 é o segundo caso da série
M4-residual onde Introspector é caminho funcional
(depois de P184D Figure). Diferente de P186 (Equation
dormente em produção até `Content::SetEquationNumbering`
materializar).

**Caminho funcional após P187B**:
- `layout()` legacy: re-corre `introspect_with_introspector`
  internamente (P181H); Introspector populado; consulta
  `formatted_counter_at` retorna `Some("1.2.3")` para
  headings esperados.
- `layout_with_introspector(content, state, intr)`: caller
  passa Introspector populado; mesmo comportamento.
- Fallback legacy `format_hierarchical` activo apenas se
  Introspector vazio ou `current_location` `None` (raro
  em prática — heading-arm é sempre invocado após gating
  `advance_locator_if_locatable` per P185C).

Janela compat eliminada em **M6** quando F1 retomar
(`CounterStateLegacy.hierarchical` removido + walk arm
canonical legacy + fallback removido).

## Secção: C2 equation counter migrado (P188B)

Equation-arm em `layout/equation.rs:97` consulta
`Introspector::flat_counter_at("equation", current_location)`
primeiro para obter o número da equação, com fallback
**substitution-with-fallback** a `self.counter.get_flat("equation")`
legacy:

```rust
use crate::entities::introspector::Introspector;
let n = self.current_location
    .and_then(|loc| self.introspector
        .flat_counter_at("equation", loc))
    .unwrap_or_else(|| self.counter.get_flat("equation"));
```

**Diferença sintáctica face a P187B (C1)**: `unwrap_or_else`
em vez de `or_else` porque `get_flat` legacy retorna `usize`
directamente (não `Option<usize>` como `format_hierarchical`).

### Estado dormente em produção (honestidade documental)

P188 é o **primeiro consumer da série M4-residual onde
migração estrutural não traduz em mudança funcional em
produção**. Comparação:

| Caso | Introspector em produção | Caminho funcional |
|------|---------------------------|-------------------|
| C3 Figure (P184D) | activo | Introspector |
| C1 Heading prefix (P187B) | activo | Introspector |
| **C2 Equation counter (P188B)** | **dormente** | **fallback legacy permanente** |

**Razão**: `Content::SetEquationNumbering` não existe em
cristalino (descoberta P186A §11.2). State
`numbering_active:equation` nunca é populado em walk real.
Gate em `from_tags` arm Equation (P186E) bloqueia →
counter introspector permanece vazio → `flat_counter_at`
retorna sempre `None` em produção → `unwrap_or_else` cai
sempre no fallback legacy `get_flat`.

**Trabalho identificado fora série**: materializar
`Content::SetEquationNumbering` (passo dedicado, fora série
P186-P188). Após esse passo:
- State é populado via tag StateUpdate.
- Gate em P186E dispara → counter introspector populado.
- `flat_counter_at` retorna `Some(n)` → caminho Introspector
  activa-se em produção.
- Janela compat M6 pode abrir para Equation
  (`CounterStateLegacy.flat["equation"]` removido +
  fallback removido).

Cross-references: P186A §11.2 (descoberta inicial), P186E
(gate location-aware), P188A (decisões), P188B (migração).

Janela compat M6 para C2 **não fechará** até
`Content::SetEquationNumbering` materializar — diferente
de C1 (que pode fechar imediatamente quando F1 retomar).

## Secção: C4 resolved label migrado (P194B)

Consumer C4 em `layout/references.rs:53-67::layout_ref`
consulta `Introspector::resolved_label_for(target)`
primeiro para obter texto resolvido de cross-references,
com fallback **substitution-with-fallback** a
`counter.resolved_labels.get(target)` legacy:

```rust
let display_text = match layouter.introspector
    .resolved_label_for(target)
    .or_else(|| layouter.counter.resolved_labels.get(target).map(String::as_str))
{
    Some(text) => text.to_string(),
    None       => format!("@{}", target.0),
};
```

### Estado temporário em produção (não permanente)

P194 é **distinto de P188B** (C2 Equation):

| | P188B (C2) | P194B (C4) |
|---|---|---|
| Estado dormente | **Permanente** | **Temporário** |
| Razão | `SetEquationNumbering` ausente | Walks Labelled/Heading não migrados (E2/E4 P189B) |
| Activação | Passo dedicado SetEquationNumbering | P195 + P196 (sequência §9 P189) |
| Documentação | 4 pontos obrigatórios | Comentário inline curto + secção L0 |

**Em produção até P195+**: sub-store
`intr.resolved_labels` (P193B) está vazio →
`resolved_label_for` retorna `None` → `or_else` cai em
fallback legacy → output idêntico ao actual. Paridade
observable preservada por construção.

**Após P195** (walk arm `Labelled` migrated): Tag emitida;
`from_tags` arm popula sub-store → caminho Introspector
activa parcialmente (Labelled explicit cobertos).

**Após P196** (walk arm `Heading` migrated): auto-toc
populated → caminho Introspector activa universalmente.

**Após P200** (M6 cleanup): `CounterStateLegacy.resolved_labels`
removido + fallback legacy removido. Forma final apenas
Introspector path.

### Forma Opção C — `Option<&str>` propagado

Variante idiomática vs P184D/P187B/P188B:
- API trait `resolved_label_for` retorna `Option<&str>`.
- Legacy `resolved_labels.get` retorna `Option<&String>`,
  convertido a `Option<&str>` via `.map(String::as_str)`.
- `or_else` chain propaga `Option<&str>` sem clones
  intermediários.
- Único `to_string()` no `Some` arm para satisfazer tipo
  final `String`.

Cross-references: P193A (decisões), P193B (sub-store
aberto), P194A §11 (achados), P189 §9 sequência.

Excepções E2-E6 (P189B walk arms) continuam activas após
P194 — só fecham com P195+ que materializam o populate
do sub-store via Tag.

---

## Secção: `engine/layout/metrics.rs` — Métricas de Fonte e Shaping

O sub-módulo `metrics.rs` (`01_core/src/engine/layout/metrics.rs`) hospeda a
interface `FontMetrics` e a implementação `FixedMetrics`, extraído de
`layout/mod.rs` no Passo 96.7 (ADR-0037).

### `FontMetrics` trait

```rust
pub trait FontMetrics: Send + Sync {
    fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt;
    fn vertical_metrics(&self, size: Pt, style: &TextStyle) -> (Pt, Pt);
    fn cap_height(&self, size: Pt, style: &TextStyle) -> Pt;
    fn text_edges(&self, size: Pt, style: &TextStyle) -> (Pt, Pt);
    // P813 — limites de tinta (ink) do texto; tem default, ver nota abaixo.
    fn text_ink_bounds(&self, text: &str, size: Pt, style: &TextStyle) -> (Pt, Pt);
}
```

- `advance`: largura horizontal de uma string em pontos tipográficos.
- `vertical_metrics`: `(ascender, line_height)` em pontos tipográficos. Recebe
  o `style` activo para que implementações com resolução de fonte possam usar
  a mesma face que o shaper/PDF efectivamente renderizará (P760). **Nota P762:**
  o avanço de linha do Layouter já não usa `line_height` directamente; usa
  `text_edges` + `leading`.
- `cap_height`: distância da baseline ao topo das maiúsculas (`H`, `X`).
  Mantido para compatibilidade; o posicionamento da primeira baseline passou
  a usar `text_edges` (P762).
- `text_edges`: devolve `(top, bottom)` offsets a partir da baseline conforme
  `TextStyle::top_edge` / `TextStyle::bottom_edge`. Valores positivos indicam
  distância para cima (`top`) ou para baixo (`bottom`). Deve suportar pelo
  menos `"baseline"`, `"x-height"`, `"cap-height"`, `"ascender"` e
  `"descender"`. **P837**: os campos são `Option<TextEdge>` — além das
  métricas nomeadas, `TextEdge::Length` resolve directamente a partir da
  baseline (`length.resolve_pt(size)`; bottom negativo = abaixo da
  baseline). `"bounds"` é aceite no eval (domínio vanilla) mas cai no
  fallback defensivo — ver `engine/eval.md` §P837. Implementações sem
  métrica real devem fazer fallback proporcional consistente com a face
  (ex: `cap-height ≈ size * 0.7`).
- `text_ink_bounds` (**P813**): devolve `(ascent, descent)` em pontos, ambos
  ≥ 0, medidos da **união das bounding boxes reais dos glyphs** do texto
  (paridade vanilla: o ascent/descent de um frame math vem das bboxes dos
  glyphs — `lab/typst-original/crates/typst-layout/src/math/fragment/glyph.rs`
  — não das métricas globais da fonte). Tem implementação default
  conservadora `(cap_height(size, style), Pt(0.0))` para métricas sem
  acesso a bboxes (`FixedMetrics`, stubs de teste); a implementação L3 com
  fonte real sobrescreve com `glyph_index` + `glyph_bounding_box`.
  Consumidor actual: `MathLayouter::layout_equation_measured` (extent da
  equação para centragem/espaçamento de bloco — ver
  `engine/layout/equation.md`).

### `FixedMetrics`

Implementação monoespaçada pura de L1:

- `advance(text, size, _)` → `size * (chars.count() * 0.6)`.
- `vertical_metrics(size)` → `(size * 0.8, size * 1.2)`.
- `cap_height(size)` → `size * 0.7`.
- `text_edges(size, style)` → `TextEdge::Length(l)` resolve para
  `Pt(l.resolve_pt(size))` em ambos os edges (P837); `TextEdge::Metric`
  mapeia `top_edge` para `"baseline"=0`, `"x-height"≈size*0.5`,
  `"cap-height"/default≈size*0.7`, `"ascender"≈size*0.8`; `bottom_edge`
  para `"descender"≈size*-0.2`, `"baseline"/default=0`.

### `needs_shaped_width` — detecção de scripts contextuais

```rust
pub fn needs_shaped_width(text: &str) -> bool
```

Função pura de L1 que decide se um trecho de texto deve ser medido com
shaping aplicado (formas contextuais / ligaduras) antes da decisão de
quebra de linha. Usada por `FallbackFontMetrics::advance_shaped` em L3
(`03_infra/src/font_metrics.rs`).

A lista de scripts é determinada empiricamente: inclui scripts onde as
formas contextuais reduzem a largura total de forma significativa e
observável em comparação com a soma de advances isolados.

Scripts activos (actualizados em P623):

- `Arabic`
- `Syriac`
- `Mongolian`
- `Nko`
- `Mandaic`
- `Devanagari`

**Critério de inclusão (ADR-0108):** um script só é adicionado à lista
depois de medição directa que mostre diferença de largura shaped vs
não-shaped suficiente para alterar a decisão de quebra de linha em
documentos realistas.

### Invariantes

- `needs_shaped_width` não acede a ficheiros de fonte nem faz I/O — é uma
  análise Unicode pura.
- O shaping real continua a ser responsabilidade de L3 (`shaper.rs`).
- L1 permanece independente de `rustybuzz` / `ttf-parser`.

### `measure_content_constrained`

Função auxiliar em `layout/mod.rs` usada por `Grid`, `Box`, `Block`,
`Pad`, `Stack` e `Sequence` para estimar dimensões de conteúdo antes do
layout real.

- Para `Content::Text`, deve usar `FontMetrics::text_width` (P593) em vez
  de `FontMetrics::advance` directo, para que medições antecipadas de
  texto em scripts contextuais (árabe, devanágari, etc.) tenham a mesma
  largura que o layout final.
- Para outros tipos de `Content`, mantém a lógica específica (Shape,
  Curve, Pad, etc.).

## Secção: Sub-layout isolado (`layout_sub_frame`, Passo 629)

O helper `layout_sub_frame` vive em `01_core/src/engine/layout/sub_frame.rs`
e é o ponto comum para executar layout de conteúdo numa região isolada,
salvando e restaurando o estado do `Layouter`.

### `SubLayoutRegion`

```rust
pub(super) struct SubLayoutRegion {
    pub origin_x: f64,
    pub width: f64,
    pub height: Option<f64>,
    pub align_rtl: bool,
    pub unconstrained_height: bool,
}
```

- `origin_x`: origem horizontal da região dentro do frame pai.
- `width`: largura útil disponível para o conteúdo.
- `height`: altura útil disponível; `None` significa "sem limite".
- `align_rtl`: se a última linha deve ser alinhada à direita quando o
  estilo de texto for RTL.
- `unconstrained_height`: se a altura é ilimitada; afecta o ancoramento
  de `Content::Align` (decai `VAlign::Bottom`/`VAlign::Horizon` para `Top`).

### `layout_sub_frame`

```rust
pub(super) fn layout_sub_frame(
    &mut self,
    content: &Content,
    region: SubLayoutRegion,
) -> (f64, Vec<FrameItem>, Vec<DecoSegment>)
```

- Salva o estado completo do `Layouter` (`current_items`, `current_line`,
  cursor, dimensões da região, `is_height_unconstrained`).
- **P772x** — faz swap do `decoration_lines_collector` ambiente por um
  collector LOCAL (`Some(Vec::new())` se havia um ambiente activo, `None`
  caso contrário — sem overhead quando não há decoração em curso).
- Inicializa um frame temporário com origem em (`origin_x`, ascender).
- Executa `layout_content(content)`.
- Alinha a última linha RTL se `align_rtl` for `true`.
- Faz flush dos itens pendentes, calculando a altura real do sub-frame —
  **P772x**: este flush manual regista o segmento da última linha no
  collector local (mesmo hook usado por `flush_line()`, `cursor.rs`), algo
  que faltava antes desta correcção (ver secção "Decoração através de
  `layout_sub_frame`" abaixo).
- Restaura o collector ambiente (LIFO) e o resto do estado; devolve
  `(height, items, deco_segments)` em coordenadas locais ao frame temporário
  — `deco_segments` só é não-vazio se havia um collector ambiente activo.

### Decoração através de `layout_sub_frame` (P772x)

**Bug corrigido** (achado por P772w, corrigido por P772x): antes desta
correcção, `layout_sub_frame` fazia o seu flush manual da última linha
**sem** passar por `flush_line()` — o único ponto onde o mecanismo de
decoração wrap-aware (P284/P286, `decorations.rs`) regista segmentos em
`decoration_lines_collector`. Qualquer conteúdo decorado
(`underline`/`strike`/`overline`) que passasse por um sub-frame (via
`place()`, células de grid, `align()`, footnotes) perdia a decoração
silenciosamente — repro: `#underline[Some text #place(top+left)
[explanation].]` só sublinhava "Some text", nunca "explanation".

**Mecanismo da correcção**: `layout_sub_frame` colecciona segmentos **em
coordenadas locais ao sub-frame** (mesmo referencial de `items` devolvidos)
num collector local (swap-in/out do collector ambiente, disciplina LIFO
idêntica a `decorations.rs`). Cada um dos **7 call-sites** de
`layout_sub_frame` (`grid.rs` ×2, `mod.rs`, `place.rs`, `placement.rs` ×2,
`cursor.rs`) já aplica uma translação própria (`offset_x`/`offset_y`, por
vezes com correcção de baseline) aos `FrameItem`s devolvidos — **a mesma
translação, aplicada aos campos `start_x`/`end_x`/`baseline_y` de cada
`DecoSegment`**, produz os segmentos no referencial do frame pai, prontos a
reinserir em `self.decoration_lines_collector` (se `Some`).

**Classificação dos 7 call-sites**:

| Call-site | Papel | Tratamento |
|---|---|---|
| `placement.rs::layout_align` (`Content::Align`) | Emissão real | Traduz e reinsere (`delta_x`, `target_y - sub_origin_y`) |
| `placement.rs::layout_place` (`Content::Place`, `float: false`) | Emissão real | Traduz e reinsere (`target_x + ix`, `target_y - y_offset`) — **caso motivador de P772w** |
| `grid.rs` (Fase 2, emissão de célula) | Emissão real | Traduz e reinsere (x já absoluto; `body_y + (y - local_start_y)`) |
| `cursor.rs` (footnote body, Pass 2) | Emissão real (2 passes) | `measured` carrega `(h, items, deco)` por footnote; Pass 2 traduz e reinsere por iteração |
| `place.rs` (`Content::Place`, `float: true`) | Emissão **diferida** (`DeferredFloat`) | `deco_segments` capturado no `DeferredFloat`, traduzido em `emit_deferred_float` (flush da página) — **best-effort**: só produz `FrameItem::Line` se o collector ainda estiver `Some` nesse momento (não garantido para floats que só flusham muito depois do consumer decorador ter retornado — ver `DeferredFloat::deco_segments`, `mod.rs`) |
| `grid.rs` (Fase 1, medição de altura de linha) | Medição pura | `_sub_items`/`_deco` descartados — emissão real acontece na Fase 2 |
| `mod.rs::measure_content_real` (`measure()`) | Medição pura, `Layouter` isolado e efémero | Sem collector ambiente possível — `_deco` sempre vazio |

**Paridade vanilla (ADR-0107)**: a contagem exacta de operadores `S` (stroke)
que o vanilla emite para uma mesma decoração pode diferir da do cristalino
(ex.: vanilla segmenta por `MCID`/span de texto, produzindo mais segmentos
contíguos para a mesma linha visual) — isto é mecanismo de exportação PDF,
não língua; confirmado que a contagem difere mesmo em texto simples sem
nenhum sub-frame envolvido (`#underline[texto simples]`: vanilla 2 stroke,
cristalino 1). O observável de língua é a decoração aparecer, visualmente
contínua, sobre o texto correcto — confirmado por render (`mutool draw`),
não por paridade byte-a-byte de operadores PDF.

### `layout_sub_frame_inline` (Passo 631)

Variante do helper para **sub-layouts inline**, onde o conteúdo
continua na linha horizontal do pai (avança `cursor_x`, não
`cursor_y`) e os itens produzidos devem ser devolvidos ao caller em
vez de serem injectados directamente no frame pai. Usada por
`Boxed` (`boxed.rs`) para isolar o corpo da caixa, aplicar
alinhamento RTL apenas aos itens do body e reduzir a duplicação de
save/restore de estado.

```rust
pub(super) fn layout_sub_frame_inline(
    &mut self,
    content: &Content,
    region: SubLayoutRegion,
) -> (f64, Vec<FrameItem>)
```

Semântica:

- **Não** altera `regions.current.width` nem `line_start_x` — o
  caller (por exemplo `boxed.rs`) é responsável por configurar a
  largura disponível antes de chamar, exactamente como faz hoje.
  Isto evita qualquer divergência de comportamento entre
  `width: Some(w)` e `width: None`.
- Executa `layout_content(content)` sem esvaziar a `current_line` do
  pai — o body do box pode continuar a linha do pai, tal como hoje.
- Isola os itens que o body adicionou à `current_line` do pai
  (`drain` a partir do comprimento anterior), aplica
  `align_current_line_rtl()` apenas a esses itens se
  `region.align_rtl` for `true`, e restaura a `current_line` do pai.
- Devolve `(height, items)` em coordenadas locais à linha inline. O
  `height` é a altura da linha resultante (fallback ao
  `line_height` do estilo activo quando o body não produz texto);
  `items` **não** são adicionados a `current_items` — o caller
  decide onde e como posicioná-los (por exemplo, para aplicar
  `clip` antes de os emitir).

Campos de `SubLayoutRegion` usados:

- `align_rtl`: se a última linha do body deve ser alinhada à
  direita quando o estilo for RTL.
- `origin_x`, `width`, `height`, `unconstrained_height`: reservados
  para extensões futuras; nesta variante inline são ignorados.

### `measure_content_real` (P712) — consumer standalone, sem `Layouter` do chamador

Diferente dos consumers acima (que já correm dentro de um `Layouter`
existente, do documento principal), `measure_content_real`
(`layout/mod.rs`) constrói o **seu próprio** `Layouter` isolado — é o
motor de medição real por trás da stdlib `measure()` (P712, substitui
a antiga aproximação manual por tipo de `Content` em
`layout/helpers.rs::measure_content` **apenas para este consumer**; os
outros consumers de `measure_content` — `Content::Transform`,
`Content::Place` — ficam inalterados).

```rust
pub fn measure_content_real(content: &Content, chain: &StyleChain) -> (f64, f64)
```

- Constrói `Layouter::new(FixedMetrics, NullImageSizer, chain.size(),
  <TagIntrospector::empty().track()>)` — L1 não tem métricas de fonte
  reais (`FallbackFontMetrics` é L3); divergência mecânica documentada
  (ADR-0107), não de língua.
- Substitui `layouter.chain`/`layouter.style` pela `StyleChain` do
  chamador (`chain.clone()`/`TextStyle::from(chain)`) — o tamanho
  medido depende do `#set text(size:)` activo no ponto de chamada,
  paridade com o vanilla `context.styles()`.
- Chama `layout_sub_frame` com `SubLayoutRegion { origin_x: 0.0,
  width: f64::INFINITY, height: None, align_rtl: false,
  unconstrained_height: true }` — região efectivamente sem limites,
  paridade com o vanilla `Region::new(.., Abs::inf())` para `measure()`
  sem `width`/`height` explícitos (scope-out, ADR-0054 — só a forma
  sem overrides é medida/implementada).
- **Largura**: `FixedMetrics.line_content_right(&items)` sobre os itens
  devolvidos — generaliza correctamente para conteúdo multi-linha,
  já que `line_start_x` reinicia a 0 a cada linha (o máximo de `x +
  largura` across todos os itens de todas as linhas dá a linha mais
  larga).
- **Altura**: a `height` já devolvida por `layout_sub_frame` (calculada
  a partir do cursor real avançado durante o layout, não uma
  aproximação por tipo de `Content`).
- Caller real: intercepção de `measure()`/`std.measure()` em
  `eval_func_call` (`eval/closures.rs` §P712, `rules/eval.md` §P712) —
  ver `rules/stdlib/layout.md` §`measure(body)` para o contrato
  observável completo (incluindo o gate `ctx.in_context`).

### Invariantes

- O helper não deve introduzir comportamento específico de nenhum
  container (`grid`, `place`, `box`, etc.). Essa lógica permanece nos
  call-sites.
- Refactors deste helper são passivos: alterações na assinatura só
  acontecem quando há ganho de clareza e todos os call-sites são
  actualizados.
- Passos que tocam `layout_sub_frame` devem manter zero regressão nos
  testes de `grid`, `placement`, `p624`, `p625`, `p626` e `p627`.
- `layout_sub_frame_inline` deve manter zero regressão nos testes de
  `boxed`, `p624` e `p625`.

## Secção: Contrato de composição de coordenadas entre `layout_sub_frame` e
`Content::Place` (P772g, encerra achado B de P772f)

### O problema (medido em P772f, `00_nucleo/diagnosticos/paridade-producao-p772f.md` §2.4)

`layout_place` (`placement.rs`), quando resolve `PlaceScope::Column` com
`regions.cell`/`cell_origin_x` `Some` (dentro de uma célula de grid), calcula e
**emite coordenadas finais absolutas** para os itens do seu corpo (`target_x = cx +
alinhamento + dx`, onde `cx` já é a posição absoluta da célula). Isto está correcto
quando quem consome esses itens **confia neles como já finais** — mas está errado
quando o chamador imediato de `layout_place` (via `layout_content`) é, por sua vez,
outro wrapper que **também** recompôe uma posição absoluta a partir do seu próprio
estado e a soma a todos os itens que recebe de volta. Nesse caso a origem da célula
é somada duas vezes.

### As duas famílias de chamadores de `layout_sub_frame`

Todo o código que chama `layout_sub_frame`/`layout_sub_frame_inline` (ou manipula
`cursor_x`/`line_start_x` directamente para simular o mesmo efeito) pertence a uma de
duas famílias, distinguidas por **como tratam os itens devolvidos**, não por nenhuma
flag no `Layouter`:

- **Consumidor absoluto** — passa a `origin_x` **real/absoluta** da região (a
  posição onde o conteúdo vai mesmo ficar), e **não soma nenhum deslocamento extra**
  aos itens devolvidos; trata-os como já finais. Exemplos actuais, confirmados por
  leitura de código em P772g: `grid.rs` (chamada per-célula de `layout_sub_frame`
  com `origin_x: body_x`, tradução final usa `x: Pt(lx)` sem re-somar `body_x`);
  `box.rs`/`layout_sub_frame_inline` (não reinicia `line_start_x`, opera no cursor
  real do pai); `pad.rs`/`stack.rs` (não usam `layout_sub_frame` — deslocam
  `cursor_x`/`line_start_x` directamente no frame real, sem sub-frame local).
- **Consumidor relativo (recompositor)** — passa `origin_x: 0.0` (uma origem local
  fictícia), mede o conteúdo à parte, calcula depois um `target_x` absoluto a partir
  do seu **próprio** estado (`line_start_x`/`cursor_y` reais), e **soma esse
  `target_x` inteiro** a cada item devolvido do sub-frame. Exemplos actuais,
  confirmados por leitura de código em P772g: `layout_align` (`placement.rs`,
  `origin_x: 0.0` + `new_x = target_x + ix`); a emissão de footnotes em `cursor.rs`
  (mesmo padrão, `target_x = left_x` somado a cada item); `columns.rs::layout_segmented`
  (usa `margin` como origem local do buffer da coluna, depois traduz por
  `dx = column_x_offsets[idx] - margin`).

`Content::Place` (`layout_place`) **sempre** emite coordenadas absolutas quando tem
`regions.cell`/`cell_origin_x` `Some` — isto é, o seu contrato é o de um
**consumidor absoluto** a jusante. Por construção, isto só compõe correctamente
quando o chamador imediato também é um consumidor absoluto. Um `place()` aninhado
dentro de um wrapper "recompositor" (align, footnote, columns) soma a origem da
célula duas vezes.

### Correcção (Decisão do humano, P772g — Opção "corrigir os wrappers"): tornar os
recompositores consistentes com os consumidores absolutos

Em vez de ensinar `layout_place` a distinguir dinamicamente em que família está
(exigiria novo estado no `Layouter`, propagado e restaurado correctamente em todos
os pontos de aninhamento — mais estado partilhado, mais superfície de regressão),
**os wrappers recompositores passam a comportar-se como consumidores absolutos**:

1. Chamar `layout_sub_frame`/equivalente com a **origem real absoluta** (o
   `line_start_x`/`cursor_x`/`margin+offset` já em vigor no momento da chamada), em
   vez de `0.0` (ou, no caso de `columns.rs`, em vez de `margin`).
2. Na recomposição, somar apenas o **deslocamento incremental** induzido pelo
   alinhamento — `delta = target_x - origin_x_absoluta_usada_no_passo_1` — em vez do
   `target_x` inteiro.

Isto preserva bit-a-bit o resultado para conteúdo "normal" (texto, formas): como
`layout_content` avança posições aditivamente a partir do `cursor_x`/`line_start_x`
inicial, `item.x_absoluto = origin_x_absoluta + item.x_relativo_a_0` sempre que a
região é inicializada com essa origem; logo
`item.x_absoluto + delta = origin_x_absoluta + item.x_relativo_a_0 + (target_x -
origin_x_absoluta) = target_x + item.x_relativo_a_0`, exactamente o valor já
produzido hoje. Para `Content::Place` aninhado, o item já vem absoluto
(`item.x_absoluto = target_x_place`, sem termo relativo-a-0 a somar); com o wrapper
a somar só `delta` (não o `target_x` do wrapper inteiro), a origem da célula deixa
de ser somada duas vezes.

### Âmbito confirmado nesta correcção (P772g)

- `layout_align` (`placement.rs::layout_align`) — corrigido.
- Emissão de footnotes (`cursor.rs`, bloco de posicionamento pass 2) — corrigido,
  mesmo padrão que `layout_align`.
- `columns.rs::layout_segmented` — corrigido; a origem local passa de `margin` para
  `column_x_offsets[idx]` (a posição real da coluna), e a tradução final usa o
  incremento correspondente.
- `layout_place` (`placement.rs::layout_place`) — **inalterado**; mantém-se como
  consumidor absoluto. Não deve ganhar lógica condicional a distinguir famílias de
  chamador — essa distinção vive nos wrappers, não em `Content::Place`.
- `grid.rs`, `box.rs`, `pad.rs`, `stack.rs` — já consumidores absolutos por
  construção; revistos e confirmados sem alteração necessária.

### Invariante para wrappers futuros

Qualquer novo wrapper que chame `layout_sub_frame` e depois reposicione os itens
devolvidos deve ser um **consumidor absoluto**: passar a origem real da região a
`layout_sub_frame`, e somar apenas o deslocamento incremental de alinhamento (nunca
uma origem local fictícia recomposta à parte). Isto evita reintroduzir a classe de
bug medida em P772f/corrigida em P772g sempre que `Content::Place` (ou qualquer
conteúdo futuro que também emita coordenadas absolutas) for aninhado dentro desse
wrapper. Validar com um `place(scope: column, ...)` aninhado dentro do novo wrapper,
dentro de uma célula de grid, contra o vanilla via `mutool trace` (tolerância
sub-pt), antes de considerar o wrapper completo.

## Secção: Alinhamento efectivo per-célula em `grid()`/`table()` (P772j)

### Contexto

P772f encontrou, no working tree, um bloco de código não commitado (nunca finalizado,
sem L0, sem testes — arqueologia completa em
`00_nucleo/diagnosticos/paridade-producao-p772h.md`) que envolvia o corpo da célula
num `Content::Place { scope: Column, .. }` quando havia alinhamento efectivo. Medido
contra o vanilla: `#grid(align: center, [Hello], [World])` divergia em ~13.5pt. P772j
reverte esse código (`grid.rs`, ver commit deste passo) e substitui por um mecanismo
verificado directamente contra o código-fonte do vanilla — não uma tentativa nova
adivinhada.

### Mecanismo vanilla confirmado (leitura directa, não assumido do nome da propriedade)

`lab/typst-original/crates/typst-layout/src/engine.rs`:

```rust
const GRID_CELL_RULE: ShowFn<GridCell> = |elem, _, styles| {
    show_cell(elem.body.clone(), elem.inset.get(styles), elem.align.get(styles))
};

fn show_cell(mut body: Content, inset: .., align: Smart<Alignment>) -> SourceResult<Content> {
    if inset != Sides::default() {
        body = body.padded(inset);
    }
    if let Smart::Custom(alignment) = align {
        body = body.aligned(alignment);
    }
    Ok(body)
}
```

Isto é um **show-rule** — corre na realização da célula (eval-time, antes do
layout), não durante o layout do grid. `typst-layout/src/grid/layouter.rs` (a camada
de LAYOUT) **não referencia `.align` de todo** (confirmado por grep) — quando o
layouter do grid recebe o corpo da célula, o alinhamento já foi aplicado por
`show_cell`, envolvendo o corpo num `Align` (nunca num `Place`). A ordem importa:
padding primeiro (mais interno), alinhamento depois (mais externo) —
`align(alignment, pad(inset, body))`.

### Precedência confirmada por medição (não assumida)

Repro `#grid(rows: 2cm, align: horizon, grid.cell(align: left)[Hello])` no vanilla
(`mutool trace`): a posição Y do texto corresponde a alinhamento vertical
**Horizon** (herdado do grid), não Top (o default se a célula substituísse o
`Align2D` inteiro por `left` sozinho). Confirma: a resolução é um **fold por eixo**
— cada eixo (H/V) resolvido independentemente; o eixo que a célula especifica vence,
o eixo que a célula não especifica herda do grid. Não é "célula vence inteiramente
se especificar qualquer eixo".

### Mecanismo cristalino (implementado neste passo)

`effective_align` deixa de ser `cell_align.or(self.cell_align)` (que descartava
inteiramente o align do grid se a célula especificasse qualquer eixo) e passa a
fold por eixo:

```rust
let effective_align = match (cell_align, self.cell_align) {
    (None, None) => None,
    (Some(c), None) => Some(c),
    (None, Some(g)) => Some(g),
    (Some(c), Some(g)) => Some(Align2D { h: c.h.or(g.h), v: c.v.or(g.v) }),
};
```

Quando `effective_align` é `Some`, o corpo da célula é envolvido em
`Content::Align { alignment: effective_align, body: cell.clone() }` antes de
`layout_sub_frame` — **nunca em `Content::Place`** (essa foi a causa arquitectural
da divergência do código órfão: `Place` tem semântica de posicionamento absoluto
fora do fluxo; `Align` tem semântica de reposicionamento dentro do espaço
disponível — não são intercambiáveis, mesmo quando numericamente próximos nalguns
casos). O inset continua aplicado por aritmética directa de bounds (`body_x`/`body_w`
reduzidos antes de layoutar), não por um wrapper `Pad` explícito — divergência
mecânica aceite (ADR-0107): o resultado observável é idêntico, só a mecânica interna
difere.

Este mecanismo **reutiliza directamente** a disciplina de "consumidor absoluto"
estabelecida na secção anterior (P772g): quando `grid.rs` chama
`layout_sub_frame(&Content::Align{..}, SubLayoutRegion{origin_x: body_x, width:
body_w, ..})`, `layout_align` lê `self.regions.current.line_start_x` (=`body_x`,
correctamente definido por essa chamada) e `self.regions.cell` (`Some`, com
`width`/`height` do corpo já reduzido por inset) — **nenhuma alteração adicional a
`placement.rs` é necessária**; a correcção de P772g já compõe correctamente quando
`layout_align` é invocado nesta posição, porque não distingue "quem construiu o
`Content::Align`" — só lê o estado do `Layouter` no momento da chamada.

### Invariante

Qualquer alinhamento efectivo de célula (grid ou table) usa `Content::Align`, nunca
`Content::Place`. A precedência grid-level vs per-célula é sempre um fold por eixo
(H e V resolvidos independentemente), nunca uma substituição total do `Align2D`.
Validar qualquer alteração a este mecanismo com um repro que combine align
per-célula num eixo com align a nível de grid no outro eixo, confirmando por
coordenadas (`mutool trace`) que ambos os eixos compõem correctamente — não apenas
o caso em que a célula especifica os dois eixos ou nenhum.

## Secção: `grid.header(...)`/`grid.footer(...)` como row-groups (P772i)

### Contexto

P772f (achado incidental) e P772h (arqueologia) confirmaram que `header:`/`footer:`
tinham sido implementados em P224 (maio 2026) como **argumentos nomeados** de
`grid()`, sem equivalente no vanilla (que usa `grid.header(...)`/`grid.footer(...)`
como **elementos-filho** posicionais — `#[elem(name = "header")]` em
`lab/typst-original/crates/typst-library/src/layout/grid/mod.rs:580`), com o
conteúdo passado por esse caminho nunca lido pelo motor de layout
(`_header`/`_footer` ignorados, documentado como "graded" no relatório de fecho de
P224). Em paralelo, `Content::GridHeader`/`GridFooter` passados como children de
`grid()` caíam no braço genérico do loop de resolução, tratados como uma célula
normal — perdendo a semântica de row-group.

### Bug adicional confirmado durante a implementação (P772i)

`native_grid_header`/`native_grid_footer` (e os pares `table_*`) só guardavam
`args.items.first()` — `grid.header[Nome][Idade]` (sintaxe de vários blocos de
conteúdo trailing, paridade vanilla) perdia silenciosamente todas as células a
partir da segunda. Confirmado por repro directo antes da correcção. Corrigido para
colectar **todos** os argumentos posicionais num `Content::sequence(..)`.

### Mecanismo implementado

1. `native_grid` já não aceita `header:`/`footer:` como argumentos nomeados —
   `#grid(header: ..)` erra com "argumento nomeado inesperado", paridade vanilla
   (que rejeitaria da mesma forma).
2. O loop de resolução de `grid()` (`stdlib/layout.rs`) distingue
   `Content::GridHeader`/`Content::GridFooter` de células normais — não
   incrementam `col`/`row`; são guardados em `header`/`footer: Option<Content>` e
   passados ao `GridElem` (campos já existentes, agora alimentados por children em
   vez de named args). Só um header e um footer são aceites (erro explícito em
   caso de duplicado — múltiplos headers por `level` são scope-out, ver abaixo).
3. `layout_grid` (`grid.rs`) extrai as células do corpo do header/footer
   (`Content::Sequence` se houver mais que uma célula; o próprio `Content` se só
   houver uma — `Content::sequence` colapsa um Vec de 1 elemento) e cola-as antes/
   depois das células normais, preenchendo com `Content::Empty` até múltiplo de
   `num_cols` (headers/footers ocupam linhas inteiras, paridade vanilla). A partir
   daí, o algoritmo de posicionamento/layout existente trata-as como quaisquer
   outras células — sem mecanismo novo de renderização.

### Scope-out explícito (não silencioso): repeat-across-páginas

Header/footer renderizam **uma única vez**, na posição onde foram colados (header
sempre no topo, footer sempre no fim das células). **Não repetem em quebras de
página** — o equivalente vanilla de `Header`/`Footer`/`Repeatable<T>`
(`range`/`level`/`short_lived`, resolve.rs) não está implementado. Isto é
suficiente para o caso comum (tabela cabe numa página); para tabelas que quebram
página, o header não reaparece na página seguinte, e o footer aparece logo a
seguir aos dados (não necessariamente ancorado ao fundo da última página).
Decisão registada, não descoberta por acidente — se uma futura necessidade exigir
repeat-across-páginas, tratar como passo dedicado (implica estruturas novas de
`range`/`level` e lógica de re-emissão consciente de paginação no motor de grid).

### `table()` — extensão do mecanismo (P772v)

**Fechado em P772v.** `table()` nunca teve `header:`/`footer:` como argumentos
nomeados (não é o mesmo bug de P224 acima), mas `TableElem` não tinha os campos
`header`/`footer` e o loop de resolução de `table()` (`stdlib/structural.rs`)
tratava `Content::TableHeader`/`TableFooter` como célula normal — mesmo braço
genérico, mesmo sintoma. P772v confirmou por leitura directa do vanilla
(`lab/typst-original/crates/typst-library/src/model/table.rs:495,525`) que
`TableHeader`/`TableFooter` têm **exactamente** a mesma forma
(`repeat: bool` default `true`, `level: NonZeroU32` default `1` no header,
`children` variádico) que `GridHeader`/`GridFooter` — e que
`typst-layout::layout_table`/`layout_grid` (vanilla) são wrappers idênticos
sobre o mesmo `GridLayouter`, sem estilo visual por omissão específico de
`table()` para header/footer (confirmado por leitura do código, não assumido).

Extensão directa do mecanismo de P772i, sem redesenho:

1. `TableElem` ganhou `header: Option<Content>`/`footer: Option<Content>`
   (mesmos campos de `GridElem`).
2. `native_table` (`stdlib/structural.rs`) distingue
   `Content::TableHeader`/`Content::TableFooter` no loop de resolução — mesma
   lógica de `native_grid` (não incrementam col/row; só um de cada; erro
   explícito em caso de duplicado).
3. `layout_grid` (`grid.rs`, motor partilhado por `Content::Grid` e
   `Content::Table`) estende os braços de `row_group_cells` para também
   reconhecer `Content::TableHeader`/`Content::TableFooter` (extrai `.body`,
   mesmo tratamento que `GridHeader`/`GridFooter` — o motor já era genérico o
   suficiente, só faltava o braço de match).
4. `table.rs` (layout) passa `e.header.as_ref()`/`e.footer.as_ref()` a
   `layout_grid` em vez de `None, None`.

`native_table_header`/`native_table_footer` **já** colectavam todos os
argumentos posicionais (não só `.first()`) desde P772i — o bug do `.first()`
tinha sido corrigido em paralelo para os pares `table_*` na altura, mesmo sem
o wiring do `TableElem` existir ainda. P772v não repetiu esse bug (nada a
corrigir aí).

Mesmo scope-out explícito de P772i aplica-se a `table()`: sem
repeat-across-páginas (ver secção acima) — decisão herdada, não redecidida.

### Detecção de conflito célula↔header (P789)

Paridade vanilla `check_for_conflicting_cell_row`
(`lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:2112`):
uma célula do **corpo** com `y` explícito cujo range `y..y+rowspan` intersecta
as linhas do header (`0..header_rows`, onde `header_rows =
header_cells.len() / num_cols` após o preenchimento a múltiplo de `num_cols`)
é **erro**, não sobreposição silenciosa. Mensagem e hint idênticos ao vanilla
(observável ao nível da língua — ADR-0107):

```text
error: cell would conflict with header also spanning row {row}
hint: try moving the cell or the header
```

`{row}` é a primeira linha do range da célula que cai dentro do header
(vanilla: primeiro `row` de `cell_y..cell_y+rowspan` contido em
`header_rows`). No modelo splice as linhas do header são contíguas a partir
de 0, logo `{row} == y` quando `y < header_rows`.

Mecanismo: verificação em `layout_grid` (`grid.rs`) **antes** do splice,
iterando só as células do corpo (células dentro do corpo do header/footer não
são verificadas — equivalente ao `!in_row_group` do vanilla). Cobre
`Content::GridCell` e `Content::TableCell` com `y: Some(_)` (células com
`y: None` são auto-posicionadas e contornam o header — paridade vanilla, que
só verifica os braços `(Custom, Custom)` e `(Auto, Custom)` de
`resolve_cell_position`). Emissão via `layout_errors` (mesmo caminho do erro
de conflito explicit/explicit de P647), com `Span::detached()` (elementos não
carregam span — mesmo trade-off de P647).

**Scope-out explícito (não silencioso): conflito célula↔footer.** O vanilla
verifica também overlap com o range absoluto do footer
(`footer.start..footer.end`). No modelo splice o footer é colado **depois**
das células do corpo, logo o seu range absoluto só existe pós-placement — e
`PlacedCell` não carrega identidade da célula de origem para distinguir corpo
de footer nessa altura. Um port fiel exige estrutura nova (rasto de
identidade no placement ou verificação pós-placement com ranges derivados);
registado como débito, candidato a passo futuro. O achado de P786 é só
header; o caso footer não foi observado em divergência real.

**Divergência registada em P789 (não corrigida neste passo):**
`table.cell(x:, y:, colspan:, rowspan:)` explícitos são **ignorados** pelo
placement — `extract_cell_fields` (`grid_placement.rs`) só faz match de
`Content::GridCell`; `Content::TableCell` cai no braço `other` e é tratada
como célula auto `1×1`. Medido em P789 por bbox (`table.cell(x: 0, y: 0,
rowspan: 2)` cai na primeira linha livre em vez de (0,0)). A verificação de
conflito acima cobre `TableCell` com `y` explícito (paridade do **erro**);
honrar as posições explícitas de `TableCell` no placement é item separado,
candidato a passo futuro.

---

## §P788 — Refs: expectativas actualizadas para paridade vanilla

Os testes legacy de refs (`Ref para trás/frente ... 'Secção 1'`) usavam
headings **sem** `numbering` — o vanilla 0.15.0 **erra** nesse caso
(`cannot reference heading without numbering`, medido por execução). As
expectativas foram actualizadas: sem numbering → erro de layout; com
numbering (doc `en`) → `Section 1` (não `Secção 1`, que era hardcoded
legacy; em docs `pt` → `Secção 1`). Ver layout_references.md §P788.

---

## §P842 — `h(Nfr)`: expansão de spacings fracionários na linha (achado #38 de P831)

Paridade vanilla (`layout/spacing.rs`, `Spacing::Fractional`): `h(1fr)`
consome o espaço restante da linha; vários fr partilham na razão dos
valores; length fixo e fr combinam (`A#h(10pt)B#h(1fr)C`). Medido nos dois
binários (`temp/p842/l7_h_*.typ`).

Mecanismo:

1. **Registo** — o layout de `HSpace` com `Spacing::Fractional(fr)` (ver
   `entities/elements/h_space.md`) **não** avança o cursor; regista
   `(current_line.len(), fr)` em `Region::pending_fr`.
2. **Expansão** — `Layouter::expand_fr_spacings` (`cursor.rs`), chamada no
   início de `flush_line` e de `finish` (a última linha do documento
   também expande — medido):
   - `remaining = (width - margin) - line_content_right(current_line)`,
     truncado a `>= 0` (linha overfull → fr = 0, sem translação);
   - cada fr, por ordem de índice de inserção, translada os items da
     linha a partir do seu índice **apenas pelo seu próprio share**
     (`remaining * fr / total_fr`) — o item à direita de vários fr recebe
     a soma dos shares, uma parcela por iteração;
   - após a expansão, `cursor_x = right_margin` (a linha consumiu todo o
     espaço; decorações e medidas subsequentes vêem o fim real).
3. **Medição** — em `measure_content_constrained` (grid measurement),
   `Fractional` mede `(0, 0)`: o fr só expande contra o espaço restante
   de uma linha real; numa medição isolada não há restante definido.

Interações registadas: a expansão corre **antes** do collector de
decorações, do cálculo de leading e do alinhamento RTL em `flush_line`
(todos veem a linha já expandida); `weak` em frações fica diferido como
nos comprimentos (perfil ADR-0054 graded). `v(1fr)` fica fora de escopo
(distribuição vertical é outro mecanismo) — rejeição pré-P842 preservada
verbatim.

## Secção: Agrupamento por Parbreak entre itens estruturais (P864)

Itens de lista, enum e termos (`Content::ListItem`, `Content::EnumItem`,
`Content::TermItem`) separados por `Content::Parbreak` numa mesma
`Sequence` devem ser tratados como **grupos distintos** em layout,
introduzindo o espaçamento de parágrafo entre grupos e reiniciando
contadores quando aplicável — paridade vanilla com listas/enums/termos
separados por linha em branco no markup.

### Estado no `Layouter`

- `ItemGroup` enum (`List`, `Enum`, `Terms`) identifica o tipo do último
  item estrutural visto.
- `last_seen_item_group: Option<ItemGroup>` — grupo do último item
  estrutural processado numa `Sequence`.
- `parbreak_since_last_item: bool` — `true` quando um `Content::Parbreak`
  ocorreu desde o último item estrutural sem conteúdo que quebre a
  consecutividade no meio.

Inicialização: ambos os campos começam a `None` / `false`.

### Lógica em `engine/layout/sequence.rs`

Durante a iteração de uma `Sequence`:

1. Se o conteúdo actual for um item estrutural:
   - Se `parbreak_since_last_item == true` e
     `last_seen_item_group == Some(group_do_item_atual)`:
     - Avança `cursor_y` por um `paragraph_advance` (espaçamento de
       parágrafo) **antes** de layoutar o item.
     - Reseta `last_was_loose_item = false` (P505 — não acumula com o
       espaçamento de itens soltos do mesmo grupo).
     - Reseta `enum_counter = None` (o próximo `EnumItem` sem número
       reinicia em 1).
   - Actualiza `last_seen_item_group = Some(group_do_item_atual)`.
   - Reseta `parbreak_since_last_item = false`.

2. Se o conteúdo for `Content::Parbreak` e `last_seen_item_group` for
   `Some(_)`:
   - Marca `parbreak_since_last_item = true`.

3. Se o conteúdo for outro conteúdo real (não `Space`, `Empty`,
   `Styled`, nem item estrutural):
   - Reseta `last_seen_item_group = None` e
     `parbreak_since_last_item = false`.

### Avanço de parágrafo

O avanço aplicado entre grupos usa o mesmo modelo P762 do avanço de
linha:

```text
paragraph_advance = top_edge + |bottom_edge| + leading
```

Calculado a partir do `TextStyle` activo (`style.size`, `leading`,
`top_edge`/`bottom_edge`) e de `FontMetrics::text_edges`. O `leading`
default é `0.65em` quando não está explicitamente definido.

### Reset de contador de enum

A separação por Parbreak reinicia a numeração de enums: o campo
`enum_counter` do `Layouter` é posto a `None`, pelo que o próximo
`EnumItem` sem número explícito começa novamente em `1.`.

### Não-acumulação entre tipos diferentes

Itens de tipos diferentes separados por Parbreak (ex: `ListItem`
seguido de `EnumItem`) não recebem o avanço de parágrafo extra — o
Parbreak comporta-se como uma quebra de parágrafo normal entre
conteúdos de tipos distintos.

### Validação

- Dois `ListItem` separados por `Parbreak` produzem marcadores cujo gap
  vertical é `2 * line_advance`; sem `Parbreak`, o gap é `1 * line_advance`.
- Dois `EnumItem` separados por `Parbreak` reiniciam a numeração (ambos
  `1.`) e têm gap `2 * line_advance`; sem `Parbreak`, a numeração continua
  (`1.`, `2.`) e o gap é `1 * line_advance`.
- Dois `TermItem` separados por `Parbreak` têm gap `2 * line_advance`;
  sem `Parbreak`, o gap é `1 * line_advance`.
- `ListItem` seguido de `EnumItem` por Parbreak não adiciona espaço extra
  além do avanço normal do Parbreak (gap `1 * line_advance`).

---

## P887 (achado 3 de P885, extensão) — `ShapeKind::Line` em `layout_grid` com `width`/`height` zerados produz linhas degeneradas (comprimento zero)

**Contexto**: a Fase A de P887 (`00_nucleo/diagnosticos/typst-passo-887-relatorio.md`) diagnosticou
e corrigiu a ausência do stroke default de `table()` (`stdlib/structural.md`, secção P887). Depois
dessa correcção, a confirmação visual exigida pela Fase B (`typst-passo-887.md`) revelou uma
**segunda causa, independente**, sem a qual a grelha continua invisível mesmo com o stroke
correctamente resolvido: as bordas de célula e as `hline`/`vline` explícitas emitem `FrameItem::
Shape { kind: ShapeKind::Line { dx, dy }, width: 0.0, height: 0.0, ... }` — **`width`/`height`
sempre `0.0`, independentemente de `dx`/`dy`** (`layout/grid.rs`, seis pontos: bordas de célula
top/bottom/left/right em torno da linha ~828-866; `hlines`/`vlines` explícitos em torno da linha
~905-913 e ~933-941).

**Contrato documentado, violado**: `FrameItem::Shape` (`entities/layout_types.rs`, campo `width`/
`height` da variante `Shape`) documenta "`pos`: canto superior esquerdo da bounding box" — para
`ShapeKind::Line`, a bounding box é `(dx.abs(), dy.abs())`, convenção já usada consistentemente
noutros consumidores do mesmo campo (`layout/shape.rs::medir` e `layout/helpers.rs`, ambos com
`ShapeKind::Line { dx, dy } => (dx.abs(), dy.abs())`; `layout/divider.rs`, que define `width:
width_pt` igual ao `dx` da própria linha). O exportador PDF (`03_infra/src/export/stream.rs:703-
714`, função que emite `m`/`l` para `ShapeKind::Line`) **implementa correctamente** esse contrato —
usa `width`/`height` (não `dx`/`dy`) para calcular os pontos inicial/final do segmento:
```rust
let start_offset_x = if *dx < 0.0 { *width } else { 0.0 };
let end_offset_x = if *dx < 0.0 { 0.0 } else { *width };
```
Com `width == 0.0` (o que `grid.rs` emite), `start_offset_x` e `end_offset_x` colapsam ambos a
`0.0` — o segmento `m`/`l` tem início e fim no mesmo ponto, um traço de comprimento zero. **O bug
está em `grid.rs`, não no exportador** — o exportador só precisa que `width`/`height` reflictam
`dx.abs()`/`dy.abs()`, contrato que `divider.rs` já respeita para a mesma variante de shape.

**Medido**: `table(columns: 5, rows: 10, ..range(50).map(str))` (`05-tables.typ`) com o stroke
default já corrigido — 4003 operadores `S` no content stream (mais que os 371 do vanilla, não menos:
`layout_grid` desenha 4 segmentos por célula em vez de 1 segmento por linha de grelha partilhada —
divergência de mecânica aceitável per ADR-0107, não tratada aqui), mas **render a 150dpi não mostra
nenhuma linha visível** — confirma que os `S` emitidos são, de facto, segmentos degenerados.

**Correcção**: nos seis pontos de `grid.rs` que constroem `FrameItem::Shape { kind: ShapeKind::
Line { dx, dy }, .. }` para bordas de célula e `hlines`/`vlines`, `width`/`height` passam a
`dx.abs()`/`dy.abs()` (mesma convenção de `divider.rs`), em vez do literal `0.0`.

**Fora de escopo desta correcção** (registar para não se perder, não abrir achado novo sem medição
própria): a discrepância 4003 vs 371 operadores `S` (mecânica de desenho por-célula vs por-linha-
partilhada) é uma divergência de implementação, não de linguagem (ADR-0107) — o resultado visual,
uma vez corrigido o comprimento das linhas, deve bater com o vanilla independentemente de quantos
operadores o produzem. Não medido/confirmado neste passo se o resultado visual final bate
pixel-a-pixel; a confirmação da Fase B é visual/qualitativa (linhas presentes e no lugar certo),
não bit-exact (ADR-0107, paridade é com a linguagem, não com a mecânica).

---

## P888 — `layout_grid`: fusão de segmentos de borda partilhados (substitui "Opção β" de P227)

**Nota de proveniência**: esta secção foi escrita **depois** da implementação, não antes — o
desenho do algoritmo (secção "Vertical", abaixo) só se consolidou durante a própria escrita do
código, tentativa que se mostrou impraticável planear em abstracto sem experimentar a interacção
com paginação. Registado explicitamente (`typst-passo-888-relatorio.md`) — desvio da ordem normal
do Protocolo de Nucleação, não escondido.

**Contexto**: `typst-passo-888-relatorio.md` (Fase A) mediu 4003 operadores `S` no cristalino contra
371 no vanilla para `05-tables.typ` (mesma grelha, 4 páginas, 20 tabelas 5×10) — a "Opção β" de P227
desenha 4 segmentos por célula, sempre, sem fundir nada. Confirmado como seguro fundir SÓ quando os
dois lados de uma fronteira concordam no `effective_stroke` (`table.cell(stroke:)`/`grid.cell(
stroke:)` permitem strokes divergentes por célula — não presumir sempre uniforme).

### Horizontal — fundido dentro da linha, sem estado entre linhas

Para cada linha (`row_idx`), percorre as células ordenadas por coluna e funde runs contíguos com o
mesmo `effective_stroke` num único segmento de topo e um de fundo. **Não** funde a borda de fundo
da linha `r` com a borda de topo da linha `r+1` mesmo quando concordam (decisão consciente — ver
"Fora de escopo" abaixo) — o ganho principal vem da fusão vertical, não horizontal.

### Vertical — fundido entre linhas via estado acumulado (`open_vsegments`)

Para cada fronteira de coluna (`x_idx` em `0..=num_cols`), mantém um "segmento aberto"
`Option<(start_y, end_y, Stroke)>` que persiste ao longo de toda a chamada a `layout_grid` (fora do
loop de linhas). Em cada linha, resolve o stroke da fronteira olhando para a célula "dona" de cada
lado (via `grid_owner: Vec<Vec<Option<usize>>>`, um mapa (linha, coluna) → índice em `placed_cells`
construído uma vez, cobrindo colspan/rowspan — necessário porque uma célula com rowspan > 1 não
aparece em `cells_per_row` nas linhas que só atravessa, só na linha onde começa):

- Ambos os lados com o mesmo `effective_stroke` (ou só um lado tem célula — bordas externas):
  continua o segmento aberto (actualiza `end_y`) se o stroke bate com o que já estava aberto;
  senão descarrega o antigo e abre um novo.
- Lados com `effective_stroke` **diferente**: não funde — emite os dois segmentos directamente para
  esta linha (comportamento idêntico ao pré-P888 nessa posição específica) e fecha/reabre o estado
  acumulado em conformidade (sem continuidade forçada por cima de uma divergência).
- Fronteira dentro de um colspan (as duas posições resolvem para a mesma célula): não desenha nada
  (paridade vanilla, `lines.rs::vline_stroke_at_row`, "returns None" quando cruza colspan).

**Quebra de página**: uma linha vertical não pode atravessar páginas (são content streams
separados). `flush_all_vsegments` é chamado explicitamente antes de cada `self.new_page()` dentro do
loop de linhas (descarrega tudo o que está aberto para a página que está a fechar) e outra vez no
fim da função (para o que sobrar aberto na última página). Isto significa que a fusão vertical
**reinicia** a cada quebra de página dentro da mesma tabela — correcto, não uma limitação: não há
como desenhar fisicamente uma linha contínua atravessando duas páginas.

### Fora de escopo desta implementação (decisões explícitas, não omissões)

- **Sem dedup horizontal entre linhas** (fundo da linha `r` vs topo da linha `r+1`, mesmo quando
  concordam): manter os dois, tal como pré-P888, para essa fronteira específica. Medido que a fusão
  vertical sozinha já traz a contagem para a mesma ordem de grandeza do vanilla (523 vs 371 para
  `05-tables.typ`, secção seguinte) — o ganho adicional de deduplicar horizontal não compensa a
  complexidade extra (precisaria comparar runs completos entre linhas, não só stroke por stroke).
- **Sem sistema de prioridade de 3 níveis** (vanilla `StrokePriority`: `ExplicitLine` >
  `CellStroke` > `GridStroke`, `lines.rs:10-26`): quando os dois lados de uma fronteira divergem,
  este código não tenta decidir um vencedor — desenha os dois. Resultado visual idêntico ao
  pré-P888 nesse caso (sem regressão), só não ganha a optimização de contagem que o vanilla ganharia
  ali. Aceitável per ADR-0107 (mecânica pode divergir).
- **`hlines`/`vlines` explícitos continuam numa passada separada** (inalterada, código pré-existente
  logo depois do loop principal) — não entram no mesmo sistema de fusão das bordas de célula. Podem
  sobrepor-se com bordas de célula na mesma posição — comportamento pré-existente, não introduzido
  por P888.

### Medição

`05-tables.typ` (mesmo documento de referência): 4003 → **523** operadores `S` (vanilla: 371) — 87%
de redução, mesma ordem de grandeza do vanilla (contra 10.8× antes). Confirmado visualmente (render
150dpi) idêntico ao pré-P888 — nenhuma linha em falta ou deslocada.

### Testes

- `p888_grid_5x10_stroke_uniforme_funde_verticais_entre_10_linhas`
  (`01_core/src/engine/layout/tests.rs`): grelha do mesmo tamanho do cenário de benchmark, stroke
  uniforme — confirma a contagem exacta esperada (26 = 6 verticais fundidos ao longo de 10 linhas +
  20 horizontais).
- `p888_stroke_divergente_por_celula_nao_funde_e_preserva_os_dois_lados`: `grid.cell(stroke:
  vermelho)` ao lado de uma célula sem override (herda azul do grid) — confirma que a fronteira
  entre elas mantém os dois segmentos separados (não funde), e que as fronteiras externas (sem
  divergência) continuam com 1 segmento cada.
- Testes pré-existentes de P227/P234/P887 (`p227_grid_stroke_renderiza_4_lines_per_cell`,
  `p227_table_stroke_paridade_grid`, `p234_grid_stroke_baseline_p227_preservado`,
  `p887_grid_stroke_lines_bounding_box_bate_com_dx_dy`) tinham asserções codificadas para o
  comportamento antigo ("4 por célula, sem fusão") — actualizados para os valores fundidos correctos
  (7, 5, 5, 7 respectivamente, computados à mão e confirmados por execução), não removidos.

---

## P891 — `FontMetrics::math_kern` ganha parâmetro `style: &TextStyle`

Assinatura (`01_core/src/engine/layout/metrics.rs`) passa de `fn math_kern(&self, c: char) ->
MathGlyphKern` para `fn math_kern(&self, c: char, style: &TextStyle) -> MathGlyphKern` (default
inalterado — `MathGlyphKern::default()`, ignora os dois parâmetros). Motivo: `FallbackFontMetrics`
(`03_infra/src/font_metrics.rs`, ver `infra/font_metrics.md` §P891) precisa de `style` para resolver
qual face activa (entre várias candidatas, cadeia de fallback) cobre `c`, antes de ler a tabela MATH
dessa face — sem `style`, não tinha essa informação e nunca implementou o método (herdava o default,
kern sempre zero — causa confirmada do gap indevido em expoentes, `typst-passo-891-relatorio.md`).
`FontBookMetrics` (face única) ignora o novo parâmetro. `attach.rs` (único consumidor de produção,
`math/layout/attach.md`) passa `style` no call site. `impl FontMetrics for &dyn FontMetrics`
(`metrics.rs`, wrapper P858) **não foi actualizado** para reencaminhar `math_kern` — já não
reencaminhava antes de P891 (só reencaminha os 4 métodos obrigatórios sem default); mantém-se assim,
fora de âmbito deste passo (o caminho de produção usa o tipo concreto, não este wrapper, confirmado
em P890).

## P893 — `FontMetrics::math_constants` ganha parâmetro `style: &TextStyle`

Mesmo motivo e mesmo padrão de P891 (`math_kern`), primeiro dos 3 achados colaterais aí registados.
Assinatura passa de `fn math_constants(&self) -> MathConstants` para `fn math_constants(&self,
style: &TextStyle) -> MathConstants` (default inalterado — `MathConstants::fallback()`, ignora o
parâmetro). `FallbackFontMetrics` (ver `infra/font_metrics.md` §P893) precisa de `style` para
resolver, entre as faces candidatas da cadeia de fallback, qual delas tem tabela MATH (a diferença
para `math_kern` é que aqui não há `char` — a escolha é "primeira face com tabela MATH", não
"primeira que cobre o glifo `c`"). `FontBookMetrics` (face única) ignora o novo parâmetro. Único
consumidor: `MathLayouter::new` (`math/layout/_comum.md` §P893), que passa a receber `style` também
— propagado do único call site de produção, `engine/layout/equation.rs` (ver `engine/layout/
equation.md` §P893). `impl FontMetrics for &dyn FontMetrics` **não reencaminha** `math_constants`
(mesma situação de `math_kern`, P891 — não é um dos 4 métodos obrigatórios sem default; caminho de
produção usa o tipo concreto).

## P896 — correcção adiada de centragem/numeração de equação sob `width: auto`

**Achado** (`typst-passo-896-relatorio.md`): `page_config.width` só é resolvido para um valor finito
em `finish()`/`new_page()` (P867), **depois** de todo o conteúdo da página já posicionado — para
página com múltiplas equações de bloco de larguras diferentes, o valor usado para centrar (P813,
`equation.rs`) ainda estava infinito no momento da centragem; P895 evitava propagar o infinito (fica
na margem) mas não centrava de facto contra a largura final real da página.

**Mecanismo** (lido do vanilla real, `typst-layout/src/flow/distribute.rs` — não é "duas passagens
completas", é posicionamento diferido dentro do mesmo flow; escopo mínimo confirmado pelo dono:
só equação, não generalizado a `Content::Align`):

- **`Layouter::pending_equation_centering: Vec<(usize, usize, f64, f64)>`** — `(índice inicial em
  current_items, nº de items, largura própria da equação, offset x já aplicado)`. Populado por
  `equation.rs::layout_equation` quando `regions.current.width` está infinito no momento da
  centragem P813, **antes** do `flush_line()` que move os items de `current_line` para
  `current_items` (o índice inicial é calculado a partir de `current_items.len()` + o que já estava
  em `current_line` antes desta equação começar a empurrar os seus próprios items — preserva ordem).
- **`Layouter::pending_equation_numbering: Vec<(f64, EcoString, TextStyle, f64)>`** — `(y da
  baseline, texto formatado, estilo, largura do texto)`. Populado quando a numeração (P456) não tem
  margem direita bem definida ainda.
- **`Layouter::apply_pending_equation_fixups(&mut self, items: &mut Vec<FrameItem>, page_width: f64)`**
  — chamado por `finish()` e `new_page()` (`cursor.rs`), logo depois de `page_width`/`page_height`
  serem resolvidos e de `items` ser extraído de `current_items` (`std::mem::take`, não move parcial
  de `self` — `finish()` foi alinhado com o padrão que `new_page()` já usava), **antes** da
  numeração de página (P532) e de `items` ser movido para a `Page` final. Esvazia os dois `Vec`
  pendentes por completo (uma página nunca fecha parcialmente). Centragem: desloca os items pela
  diferença entre o offset correcto (agora com `page_width` finito) e o offset já aplicado.
  Numeração: constrói o `FrameItem::Text` do número só agora e acrescenta-o a `items`.
- **`helpers::shift_frame_item_x(item: &mut FrameItem, dx: f64)`** — desloca a coordenada x de um
  `FrameItem` **in-place** (mantém y), todas as variantes (incluindo `Link`, recursivo nos seus
  `items` internos). Distinto de `translate_frame_item` (que substitui por posição absoluta e exige
  mover o item por valor) — aqui só é preciso um delta relativo sobre um item já existente no `Vec`
  da página.

**Ainda fora de âmbito** (decisão explícita do dono): `Content::Align`/`resolve_alignment`
(`placement.rs`) sofrem a mesma classe de bug (`available_width()` também devolve `f64::INFINITY`
sob `width: auto`) — confirmado na Fase A, **não corrigido**. Candidato a um passo dedicado futuro
que estenda o mesmo mecanismo de diferimento (campos `pending_*` + correcção em `finish()`/
`new_page()`) a `resolve_alignment` em geral, não só equações. **Feito em P897** (secção seguinte).

## P897 — mesmo mecanismo, generalizado a `Content::Align`/`Content::Place` (eixo horizontal)

**Achado** (`typst-passo-897-relatorio.md`): grep exaustivo de `resolve_alignment(` confirma
exactamente 2 chamadas, ambas em `placement.rs` — `layout_align` (`Content::Align`) e
`layout_place` (`Content::Place`, só nos ramos sem célula de grid activa: `PlaceScope::Parent`
sempre, `PlaceScope::Column` quando `regions.cell` é `None`; os ramos com célula usam `cell.width`,
sempre finito, não tocados). Ambos sofrem a mesma classe de bug de P896: `avail_w`/`avail_w_page`
vêm de `available_width()`, que devolve `f64::INFINITY` sob `width: auto`.

**Mecanismo — reaproveita directamente `resolve_alignment` como função de correcção**, em vez de
reimplementar a fórmula de centragem/alinhamento (diferente da abordagem de P896, que recalculava
o offset inline em `equation.rs`):

- **`Layouter::pending_align_centering: Vec<(usize, usize, Align2D, f64, f64, f64)>`** —
  `(índice inicial em current_items, nº de items, alinhamento pedido, largura própria do conteúdo,
  origin_x usado na chamada de fallback, x aplicado nesses items)`. Populado por
  `layout_align`/`layout_place` quando `avail_w` está infinito no momento do posicionamento.
  Fallback aplicado nesse momento: `avail_w` é substituído por `content_w` na chamada a
  `resolve_alignment` — a fórmula degenera para `target_x == origin_x` (equivalente a alinhamento
  `Left`/`Start`, mesmo efeito de fallback que P896 usava para a centragem de equação). Em
  `layout_place`, `origin_x` gravado inclui `dx` somado (`origin_x + dx`, não `origin_x` sozinho) —
  `resolve_alignment` é linear em `origin_x` (soma-o directamente ao resultado, sem interagir com
  `avail_w`/`content_w`), logo gravar `origin_x + dx` faz a chamada de correcção recompor
  directamente o `target_x` final (que já inclui `dx`) sem somar `dx` uma segunda vez fora de
  `resolve_alignment`.
- **`Layouter::apply_pending_align_fixups(&mut self, items: &mut Vec<FrameItem>, page_width: f64)`**
  — chamado por `finish()` e `new_page()` (`cursor.rs`), logo a seguir a
  `apply_pending_equation_fixups`. Para cada entrada pendente, chama `resolve_alignment` de novo,
  agora com `available_w = page_width - 2*margin` (finito), `content_h`/`available_h`/`origin_y`
  como dummies (`0.0`, só o componente x é usado); a diferença entre o `x` corrigido e o `x` já
  aplicado é o delta passado a `helpers::shift_frame_item_x` sobre a gama de items gravada. Esvazia
  `pending_align_centering` por completo (mesma disciplina "página nunca fecha parcialmente" de
  P896).

**Confirmado que `Content::Align`/`Content::Place` também contribuem para a largura final da
página sob `width: auto`** (Fase A ponto 2, `typst-passo-897-relatorio.md`) — o mecanismo de
posicionamento diferido cobre isto automaticamente: o conteúdo alinhado é medido e emitido antes de
`page_width` ser conhecido (mesma ordem que as equações), `compute_page_width()` (P867) já soma
todos os items emitidos (incluindo os de align/place, na sua posição de fallback) para determinar a
largura final — não foi preciso nenhuma mudança adicional em `compute_page_width()`.

**Ainda fora de âmbito** (registado, não corrigido neste passo): o mesmo bug existe no **eixo
vertical** sob `height: auto` — `available_height()` e `page_bottom_limit()` (`mod.rs`) também
devolvem `f64::INFINITY` nesse caso, afectando `VAlign::Horizon`/`VAlign::Bottom`. Confirmado por
leitura de código (Fase A ponto 1), não medido/testado neste passo — candidato a passo futuro
dedicado, mesmo padrão de diferimento generalizado ao eixo y. **Feito em P898** (secção seguinte).

## P898 — mesmo mecanismo, eixo vertical (`height: auto`); protocolo de dois agentes

**Achado** (`typst-passo-898-relatorio.md`): mesma classe de bug de P896/P897, eixo Y —
`VAlign::Horizon`/`VAlign::Bottom` em `resolve_alignment` usam `available_h`/`origin_y`; sob
`height: auto`, `available_height()` e `page_bottom_limit()` devolvem `f64::INFINITY`, produzindo
`target_y` infinito. Confirmado (não `NaN` — só ocorreria se `content_h` também fosse infinito, o
que não acontece em conteúdo real).

Executado com um **protocolo de TDD em dois agentes separados** (materialização
`typst-passo-898.md`, secção "Protocolo de TDD em dois agentes"): um agente (A) recebeu só o bug e a
Fase A, investigou e escreveu os testes falhos sem ver o mecanismo de correcção horizontal em
detalhe; um segundo agente (B), sem ver o processo do primeiro, implementou até os testes passarem
sem os poder editar. Achado extra do Agente A, fora do pedido pela Fase A mas registado: em
`layout_align`, o ramo `(sem célula, Horizon|Bottom)` fazia
`cursor_y = Pt(self.page_bottom_limit())` — sob `height: auto` isto grava `cursor_y = INFINITY` no
próprio estado do `Layouter`, corrompendo `compute_page_height()` (não só a posição de um item).
`Content::Equation` confirmado **não vulnerável** no eixo vertical (nunca chama
`available_height()`/`page_bottom_limit()`). `grid.rs:460` (distribuição de `fr` entre linhas de
grid) também vulnerável mas **fora de âmbito** deste passo (achado extra do Agente A, não coberto
pelos testes, não corrigido).

**Mecanismo — simétrico ao de P897, com uma diferença estrutural relevante**:

- **`Layouter::pending_align_v_centering: Vec<(usize, usize, Align2D, f64, f64, f64, f64)>`** —
  `(índice inicial, nº de items, alinhamento, altura própria do conteúdo, origin_y **puro** (sem
  `dy`), `dy` — `0.0` para `layout_align`, que não tem `dy` —, y aplicado)`.
- **`Layouter::apply_pending_align_v_fixups(&mut self, items, page_height)`** — reaproveita
  `resolve_alignment` com `final_avail_h = page_height - margin - origin_y`, **recalculado por
  entrada** (ao contrário do eixo X, onde `final_avail_w` é uma constante da página inteira,
  independente de `origin_x` — no eixo Y, `final_avail_h` depende de onde no fluxo o item estava,
  por isso não há uma constante partilhada). Chamado por `finish()`/`new_page()`, logo a seguir a
  `apply_pending_align_fixups`.
- **`helpers::shift_frame_item_y`** — simétrico de `shift_frame_item_x`.

**Bug de implementação encontrado e corrigido na revisão (não capturado pelos 2 testes do Agente
A, que só cobrem `Content::Align`)**: a 1ª versão do Agente B reaproveitava o truque de linearidade
de P897 (`origin_x + dx` gravado directamente no campo `origin_x`) também para `dy`, gravando
`origin_y + dy`. Isto é seguro no eixo X (`final_avail_w` não depende de `origin_x`) mas **não** no
eixo Y: como `final_avail_h` depende de `origin_y`, gravar `origin_y + dy` faz `dy` contaminar esse
cálculo — para `Bottom`, `dy` era descartado por completo; para `Horizon`, aplicado a metade do
valor. Confirmado por teste exploratório (`Content::Place(bottom)` com `dy=30` sob `height: auto`
devolvia a posição sem nenhum `dy` aplicado) antes de corrigir. Corrigido separando `origin_y`
puro e `dy` em campos distintos do tuplo — `apply_pending_align_v_fixups` só soma `dy` **depois**
de `resolve_alignment` já ter usado `origin_y` puro. Teste de regressão:
`p898_place_bottom_com_dy_sob_height_auto_aplica_dy_correctamente`
(`01_core/src/engine/layout/tests.rs`).

**Guarda adicional em `layout_align`** para o achado extra do Agente A (corrupção de `cursor_y`):
o ramo `(sem célula, Horizon|Bottom)` só grava `cursor_y = Pt(page_bottom_limit())` quando esse
valor é finito; caso contrário cai no ramo `_` (`cursor_y = Pt(target_y + sub_h)`, já finito porque
`target_y` foi resolvido com o fallback `remaining_h_for_resolve`).

**Ainda fora de âmbito** (registado, não corrigido): `grid.rs:460` (unidades `fr` em linhas de grid
sob `height: auto`, diverge do vanilla onde `fr` numa página `auto` degenera a 0); nested
`Content::Place` dentro de sub-frame (`in_sub_frame`) usa `origin_y = 0.0` local, enquanto
`apply_pending_align_v_fixups` corrige contra a altura final da página **raiz** — limitação
pré-existente idêntica já presente no eixo X de P897 para o mesmo caso aninhado, não expandida
aqui.
