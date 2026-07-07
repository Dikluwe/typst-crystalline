# Relatório de Paridade/Produção — P595

**Data:** 2026-07-07  
**Passo:** 595  
**Foco:** Nota de rodapé maior do que o espaço restante numa coluna  
**Hash do commit de base:** `9b65181bf561e55cb1e55b2937e2d29ece96517c`  
**Hash do commit de fecho:** `5d20ca87b50a37a46f7f5d41860c4082e511c0fc`  
**Prompt L0:** `00_nucleo/prompts/rules/footnote_overflow_columns.md` (`Hash do Código: 3a8202e4`)

---

## Resumo

Corrigida a perda silenciosa de conteúdo em notas de rodapé grandes em documentos com colunas. A nota nunca desaparece sem aviso: se o seu body exceder a altura útil da coluna/página, o layout emite-o defensivemente (para evitar loop infinito) e gera um aviso claro no `Sink` do compilador. Notas pequenas que cabem não sofrem regressão.

---

## Sonda

### Documentos de teste

1. **Nota grande numa coluna pequena** (`/tmp/p595-nota-grande.typ`):
   ```typst
   #set page(columns: 2, height: 200pt)
   #lorem(30)#footnote[
     Esta é uma nota de rodapé muito longa, com texto suficiente para não caber no espaço restante de uma coluna pequena, testando o que acontece quando isto excede o espaço disponível na página.
   ]
   #lorem(30)
   ```

2. **Nota maior do que uma página inteira** (`/tmp/p595-nota-enorme.typ`):
   ```typst
   #set page(columns: 2, height: 200pt)
   #lorem(30)#footnote[
     Esta é uma nota de rodapé extremamente longa. #lorem(200)
   ]
   #lorem(30)
   ```

### Comportamento antes da correção (commit `9b65181bf`)

- **Nota grande:** o texto da nota aparecia no PDF, mas entremeado/sobreposto com o texto principal da segunda coluna.
- **Nota enorme:** o corpo da nota era desenhado a partir do topo da primeira página, sobrepondo o início do texto principal.
- Em ambos os casos o documento finalizava sem panic, mas sem qualquer aviso ao utilizador.

### Comportamento do vanilla (referência em quarentena)

A versão vanilla em `lab/typst-original/target/release/typst` não aplicou `page(columns:)` da mesma forma (renderizou como página de coluna única), pelo que não serviu de referencial directo para a paridade de layout em colunas. A paridade mantém-se ao nível linguístico (ADR-0107): o conteúdo da nota deve ser recuperável ou o utilizador avisado.

---

## Implementação

### Ficheiros alterados

| Ficheiro | Alteração |
|----------|-----------|
| `01_core/src/rules/layout/cursor.rs` | Corrigido `flush_pending_footnote_bodies`: `top_safe` agora inclui `cursor_y` e o conteúdo da `current_line`; adicionado aviso quando `force_emit` é acionado. |
| `01_core/src/rules/layout/mod.rs` | Adicionado campo `layout_warnings` ao `Layouter`; exportado no `PagedDocument` em `finish()`. |
| `01_core/src/entities/layout_types.rs` | Adicionado campo `layout_warnings: Vec<String>` a `PagedDocument`. |
| `03_infra/src/pipeline.rs` | Propaga `doc.layout_warnings` para o `Sink` como `SourceDiagnostic::warning(Span::detached(), msg)`. |
| `01_core/src/rules/layout/tests.rs` | Adicionados 3 testes de regressão/overflow para P595. |
| `crystalline.toml` | Adicionada excepção de órfão para `00_nucleo/prompts/infra/pipeline.md` (L3), evitando falso positivo V7 após adicionar segundo `@prompt` ao `pipeline.rs`. |
| `00_nucleo/prompts/rules/footnote_overflow_columns.md` | Prompt L0 redigido com a especificação da correção. |
| `00_nucleo/diagnosticos/estado-disparidades-vanilla-p593.md` | Item movido de "scope-out" para "corrigido". |

### Ponto exacto de detecção de overflow

- `01_core/src/rules/layout/cursor.rs:flush_pending_footnote_bodies`, linhas que calculam `top_safe` e `force_emit`.

---

## Validação

```bash
cargo build --workspace          # OK
cargo test --workspace           # OK (todos os testes passam)
crystalline-lint .               # OK — No violations found
./target/release/typst /tmp/p595-nota-grande.typ /tmp/p595-fixed.pdf
# → warning: footnote body [1] exceeds available column/page height and may be truncated or overlap content
./target/release/typst /tmp/p595-nota-enorme.typ /tmp/p595-enorme-fixed.pdf
# → warning: footnote body [1] exceeds available column/page height and may be truncated or overlap content
```

### Testes novos

- `p595_nota_pequena_colunas_sem_regressao`
- `p595_nota_enorme_colunas_gera_warning`
- `p595_nota_grande_colunas_nao_sobrepoe_topo`

Todos passam.

---

## Critérios de fecho do passo

- [x] Comportamento actual confirmado com teste directo.
- [x] Corrigido — nota visível, ou aviso claro, nunca silêncio.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p595.md`, com hash do commit.
- [x] Actualização da lista de disparidades.

---

## Notas

- L1 permanece puro: os avisos são `String` simples acumuladas no `Layouter` e convertidas a `SourceDiagnostic` apenas em L3 (`pipeline.rs`).
- O vanilla original em quarentena não suporta `page(columns:)` como o cristalino; a decisão foi tomada ao nível da linguagem (semântica de visibilidade/avisos), não da mecânica de layout exacta (ADR-0107).
