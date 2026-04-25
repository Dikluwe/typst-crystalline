# Prompt L0 — layout
Hash do Código: 41f76748

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
