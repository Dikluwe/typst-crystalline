# Prompt L0 — `rules/footnote_overflow_columns` — Notas de rodapé grandes em colunas

Hash do Código: 80a187ae

**Camada**: L1 (com wiring de warnings em L3/L4)  
**Ficheiros alvo**: `01_core/src/engine/layout/cursor.rs`, `01_core/src/engine/layout/columns.rs`, `01_core/src/entities/layout_types.rs`  
**Ficheiros adjacentes**: `01_core/src/engine/layout/mod.rs`, `03_infra/src/pipeline.rs`  
**Origem**: P595 — nota de rodapé maior do que o espaço restante numa coluna  
**ADRs**: ADR-0107 (paridade linguagem), ADR-0108 (medir antes de decidir), ADR-0109 (atomização forma B)

---

## 1. Contexto

Em documentos com `#set page(columns: N)`, uma nota de rodapé cujo corpo excede o espaço restante no fundo da coluna onde é referenciada pode ser posicionada de forma a sobrepor o texto principal da coluna seguinte ou da mesma página. Medições de P595 mostram que o texto da nota aparece no documento, mas misturado com o conteúdo principal, o que constitui perda de legibilidade e, em casos extremos, pode fazer parte do conteúdo parecer desaparecido para o utilizador.

O Prompt L0 `columns.md` (P552/P553) regula a semântica de posicionamento de footnotes em colunas, mas não trata do overflow de bodies muito grandes. Este L0 acrescenta essa regra.

---

## 2. Medições que sustentam a decisão

### 2.1 Nota grande numa coluna pequena

Documento de teste (`/tmp/p595-nota-grande.typ`):

```typst
#set page(columns: 2, height: 200pt)
#lorem(30)#footnote[
  Esta é uma nota de rodapé muito longa, com texto suficiente para não caber no espaço restante de uma coluna pequena, testando o que acontece quando isto excede o espaço disponível na página.
]
#lorem(30)
```

Cristalino (commit `9b65181bf`, working tree com alterações pendentes de P594):
- O texto da nota aparece no PDF, mas sobreposto/entremeado com o texto principal da segunda coluna da primeira página.
- A nota não desaparece por completo, mas o layout está corrompido.

Vanilla `lab/typst-original/target/release/typst` (mesmo documento):
- Renderiza o documento como página de coluna única (a versão vanilla em quarentena parece não aplicar `page(columns:)` da mesma forma), colocando a nota no rodapé e continuando-a sem sobreposição.

### 2.2 Nota maior do que uma página inteira

Documento de teste (`/tmp/p595-nota-enorme.typ`):

```typst
#set page(columns: 2, height: 200pt)
#lorem(30)#footnote[
  Esta é uma nota de rodapé extremamente longa. #lorem(200)
]
#lorem(30)
```

Cristalino:
- O corpo da nota é desenhado a partir do topo da primeira página, sobrepondo o início do texto principal.
- O documento finaliza sem panic nem loop infinito, mas o conteúdo principal fique ilegível.

---

## 3. Decisão

### 3.1 Nunca desaparecer em silêncio

O critério de aceitação primário é **linguístico** (ADR-0107): o utilizador deve conseguir recuperar o texto da nota, ou deve ser informado de que parte dele não pôde ser colocada. Não basta que os bytes estejam presentes no PDF se estiverem ilegíveis por sobreposição.

### 3.2 Correção de `top_safe`

`flush_pending_footnote_bodies` em `01_core/src/engine/layout/cursor.rs` calcula `top_safe` a partir dos `current_items` já posicionados. Isso é insuficiente quando:

- Ainda há texto na `current_line` que não foi `flush`ado para `current_items`.
- O `cursor_y` já avançou para além do `top_safe` calculado.

A correção é considerar `cursor_y` e o conteúdo da `current_line` no cálculo de `top_safe`, garantindo que a zona de footnotes começa **abaixo** de todo o conteúdo já layoutado (quer em `current_items`, quer em `current_line`).

### 3.3 Aviso quando o body excede a página/coluna

Quando um body individual é maior do que a área útil total da página/coluna (`full_avail`), o algoritmo actual emite-o mesmo assim via `force_emit` para evitar loop infinito. Essa emissão forçada é aceitável como fallback defensivo, mas deve ser **reportada** ao utilizador como aviso.

### 3.4 Mecanismo de warnings vinda do layout

L1 permanece puro: não acede a `Sink`, `std::env`, relógio nem I/O. Os avisos produzidos durante o layout são acumulados num campo do `Layouter` e exportados no `PagedDocument` via novo campo `layout_warnings: Vec<String>`.

A pipeline em L3 (`03_infra/src/pipeline.rs`) é responsável por converter cada string num `SourceDiagnostic::warning(Span::detached(), msg)` e adicioná-lo ao `Sink` existente.

---

## 4. Alterações autorizadas

### 4.1 `flush_pending_footnote_bodies` — cálculo de `top_safe`

Em `01_core/src/engine/layout/cursor.rs`:

1. Calcular `top_safe` como o máximo entre:
   - `margin` (fallback mínimo);
   - máximo Y de `current_items` (já existente);
   - `cursor_y` actual;
   - máximo Y dos items que ainda estão em `current_line`.
2. Manter o clamp `y_cursor = (area_bot - acc_h).max(top_safe)`.

### 4.2 `flush_pending_footnote_bodies` — aviso em `force_emit`

1. Adicionar campo `layout_warnings: Vec<String>` ao `Layouter`.
2. Quando `force_emit` for verdadeiro, registar aviso do tipo:
   `"footnote body [N] exceeds available column/page height and may be truncated or overlap content"`.

### 4.3 `PagedDocument` — exportar warnings

Em `01_core/src/entities/layout_types.rs`:

1. Adicionar campo `pub layout_warnings: Vec<String>` a `PagedDocument`.
2. Inicializar campo vazio em `PagedDocument::new`.

### 4.4 `Layouter::finish` — propagar warnings para o documento

Em `01_core/src/engine/layout/mod.rs`:

1. Ao construir o `PagedDocument` final, mover `self.layout_warnings` para `doc.layout_warnings`.

### 4.5 Pipeline — propagar warnings para o Sink

Em `03_infra/src/pipeline.rs`:

1. Após `layout_with_introspector_and_metrics`, para cada `doc.layout_warnings`, criar `SourceDiagnostic::warning(Span::detached(), msg)` e adicionar ao `Vec` de `warnings` que a pipeline já retorna.

---

## 5. Restrições

- Não se remove o `match` exaustivo em `layout_content`.
- Não se introduz despacho dinâmico (`dyn`/vtable/PropMap).
- L1 permanece sem I/O: `layout_warnings` são `String` puras, sem construção de `SourceDiagnostic` em L1.
- Não se altera a semântica de footnotes pequenas que cabem — deve haver regressão zero nos testes existentes.
- Não se muda a política de paridade com o vanilla para casos em que o vanilla original em quarentena não suporta `page(columns:)` da mesma forma.

---

## 6. Critérios de verificação

- `cargo test --workspace` limpo.
- `crystalline-lint .` limpo.
- Teste novo: nota grande em `#set page(columns: 2)` produz documento sem sobreposição de texto principal (verificar que o Y mínimo do corpo da nota está abaixo do Y máximo do texto principal da coluna onde é colocada, ou noutra página).
- Teste novo: nota maior do que uma página inteira não causa panic nem loop infinito, e o documento resultante contém um aviso em `layout_warnings`.
- Regressão: notas pequenas que cabem continuam a renderizar no fundo da coluna/página como antes.
- Relatório em `00_nucleo/diagnosticos/paridade-producao-p595.md` com hash do commit de fecho.
