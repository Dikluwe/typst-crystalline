# Prompt L0 — layout
Hash do Código: 92312c9e

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
