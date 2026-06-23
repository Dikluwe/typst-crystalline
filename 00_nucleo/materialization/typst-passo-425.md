# P425 — Varredura Mecânica: gaps parciais/ausentes em layout, infra e warnings

**Título**: Varredura mecânica autônoma — A5/A6/A7/A3/A4
**Tipo**: Manutenção mecânica (M) — hash sync, warning hunter, PDF link consumer, atomização forma B, test gap filler
**Bloqueadores**: Nenhum externo; `FrameItem::Link` sem `pos/size` impede annotation URI completa (scope-out arquitetural)
**Referências**: ADR-0107 (paridade linguagem vs mecânica), ADR-0108 (medir antes de decidir), ADR-0109 (atomização forma B)

---

## FASE A.0 — Sonda do substrato

Executada antes de qualquer código. Resultados:

- **A1 (layout stubs)**: todos os variants de `Content` possuem arm em `layout_content` → vazio.
- **A2 (repr gaps)**: todos os variants de `Value`/`Content` possuem arm em `repr.rs` → vazio.
- **A3 (arms monolíticos)**: `Content::Dynamic`, `Content::Sequence` e `Content::SetPage` ainda estão inline no `layout_content` monólito (>20 LOC cada) → gap.
- **A4 (test gaps)**: `repr.rs` não cobre variants complexos de `Value` (Length, Ratio, Angle, Color, Gradient, etc.); `Divider`, `Link` e `Transform` não têm testes de layout direto → gap.
- **A5 (hash drift)**: `crystalline-lint --fix-hashes` não reporta drift → vazio.
- **A6 (warnings)**: `cargo check -p typst-core` reporta 23 warnings mecânicos (unused imports/variables/mut, unreachable patterns) → gap.
- **A7 (PDF link consumer)**: `cargo check -p typst-infra` falha porque `FrameItem::Link` (introduzido em P422) não é tratado em walkers recursivos de `typst-infra` → gap.

---

## FASE A.1 — L0 (hash obrigatório)

Criado `00_nucleo/prompts/meta/p425-varredura-mecanica.md` com:

- Definição dos agentes A1–A7, critérios de aceitação e escopo máximo.
- Distinção mecânico vs linguagem (ADR-0107).
- Protocolo de execução, critério de parada e scope-out explícito.
- Ordem recomendada: A5 → A6 → A7 → A3 → A4.

O prompt é de processo/meta, portanto não exige `@prompt-hash` em si, mas define que cada agente deve manter hashes sincronizados nos arquivos L1–L4 que tocar.

---

## FASE B — Execução dos agentes

### B.1 — A5 Hash sync

**Estado**: nada a fazer.

`crystalline-lint --fix-hashes` não detectou drift nos prompts já materializados. Nenhum commit gerado.

### B.2 — A6 Warning hunter

**Commit**: `a0b802d5c P425-A6: warning hunter — corrige 23 warnings em typst-core`

Ações mecânicas em `typst-core`:

- Remoção/adaptação de imports não usados.
- Prefixo `_` em variáveis não usadas.
- Remoção de `mut` desnecessários.
- Ajuste de patterns unreachable.

Critério: `cargo check -p typst-core` passa (apenas warnings preexistentes não relacionados ao escopo).

### B.3 — A7 PDF link consumer

**Commit**: `1c497bd49 P425-A7: PDF link consumer — adiciona FrameItem::Link em typst-infra`

`FrameItem::Link { url, items }` foi introduzido em P422 no `typst-core`, mas `typst-infra` não compilava porque os walkers recursivos não tinham braço para o novo variant.

Ações:

- `03_infra/src/export/stream.rs`: adicionado arm `FrameItem::Link { items, .. }` em `draw_item_local`, renderizando o conteúdo interno recursivamente.
- `03_infra/src/export/pdf/pipeline.rs`: adicionado arm para transportar/recursar `FrameItem::Link`.
- `03_infra/src/export/pdf/image.rs`: adicionado arm `FrameItem::Link { items, .. }` em `collect_images_from_items`.
- `03_infra/src/export/pdf/font.rs`: adicionado arm `FrameItem::Link { items, .. }` em `collect_fonts_from_items`.
- `03_infra/src/export/pdf/gradient.rs`: adicionado arm `FrameItem::Link { items, .. }` em `collect_gradients_from_items`.

**Scope-out**: emitir annotation URI de link no PDF exigiria adicionar `pos`/`size` a `FrameItem::Link` ou calcular bbox acumulado. Isso é decisão arquitetural (ADR-0107) e ficou documentado como bloqueador para passo futuro.

Critério: `cargo check -p typst-infra` passa.

### B.4 — A3 Atomizador mecânico

**Commit**: `1502ba90e P425-A3: atomizador mecânico — extrai arms de layout_content`

Três arms monolíticos de `layout_content` foram extraídos para free functions (forma B, ADR-0109):

- `Content::Dynamic` → `01_core/src/rules/layout/dynamic.rs`.
  - Free function `layout()` resolve propriedades dinâmicas via chain e delega ao body.
- `Content::Sequence` → `01_core/src/rules/layout/sequence.rs`.
  - Free function `layout()` itera com `peekable()` e aplica sticky lookahead de `below` spacing.
- `Content::SetPage` → `01_core/src/rules/layout/set_page.rs`.
  - Free function `layout()` atualiza `page_config` e sincroniza `regions.current`.

O match principal em `layout_content` ficou magro, delegando às novas funções. Nenhuma semântica alterada.

Critério: `cargo check -p typst-core` passa; lint sem erros.

### B.5 — A4 Test gap filler

**Commit**: `4b6772bf6 P425-A4: test gap filler — cobre repr complexo e layout de Divider/Link/Transform`

Ações:

- `01_core/src/rules/eval/repr.rs`: adicionado teste `repr_value_complex_types` cobrindo `Value::Length`, `Ratio`, `Angle`, `Color`, `Fraction`, `Location`, `Gradient`, `Regex`, `Tiling`, `Bytes`, `Decimal`, `Duration` e `Version`.
- `01_core/src/rules/layout/tests.rs`:
  - `layout_divider_emite_shape_line`: valida que `Content::divider()` emite `FrameItem::Shape` com `ShapeKind::Line`.
  - `layout_link_preserva_url_e_texto`: valida que `Content::link(url, body)` emite `FrameItem::Link` com a URL correta e preserva texto plano.
  - `layout_transform_preserva_shape`: valida que `Content::transform(identity, shape)` preserva o shape dentro do `FrameItem::Group` resultante.

Critério: todos os novos tests passam.

---

## FASE C — Validação

```bash
# Checks
cargo check -p typst-core   # → ok (warnings preexistentes fora do escopo)
cargo check -p typst-infra  # → ok (28 warnings preexistentes)

# Lint
crystalline-lint .          # → 0 errors; apenas warnings de prompts órfãos

# Tests novos
cargo test -p typst-core --lib repr_value_complex_types          # → 1 passed
cargo test -p typst-core --lib layout_divider_emite_shape_line   # → 1 passed
cargo test -p typst-core --lib layout_link_preserva_url_e_texto  # → 1 passed
cargo test -p typst-core --lib layout_transform_preserva_shape   # → 1 passed

# Full lib test suite
cargo test -p typst-core --lib
# → falha em `p350c_flag_on_nao_convergente_classifica` com stack overflow.
# → VERIFICADO: reproduzível no commit base `fac1f4940`, portanto preexistente e
#    não causado por este passo.
```

**Critério de fecho**:
- [x] L0 meta criado e materializado.
- [x] A5 executado (0 drift, sem commit).
- [x] A6 corrige warnings mecânicos; `cargo check -p typst-core` compila.
- [x] A7 adiciona braços `FrameItem::Link` em `typst-infra`; `cargo check -p typst-infra` compila.
- [x] A3 extrai arms monolíticos para free functions sem alterar semântica.
- [x] A4 adiciona 4 novos tests (1 repr + 3 layout), todos verdes.
- [x] Nenhum variant/API pública novo introduzido.
- [x] Nenhum vtable/`dyn` novo introduzido.
- [x] `match` exaustivo preservado.

---

## Relatório de Execução — P425

**Data**: 2026-06-23
**Executor**: assistente IA (Kimi Code CLI)
**Branch**: `Tekt`

**Sonda A.0**:
- A1/A2/A5 vazios ✅
- A3: 3 arms monolíticos identificados ✅
- A4: gaps de teste em repr e layout identificados ✅
- A6: 23 warnings mecânicos identificados ✅
- A7: `typst-infra` não compilava por `FrameItem::Link` ausente ✅

**L0**: `00_nucleo/prompts/meta/p425-varredura-mecanica.md` criado com definição de agentes, protocolo e scope-out.

**Implementação**:
- `a0b802d5c` — A6: ~23 warnings mecânicos corrigidos em `typst-core`.
- `1c497bd49` — A7: braços `FrameItem::Link` recursados em stream, pipeline, image, font e gradient de `typst-infra`.
- `1502ba90e` — A3: extração de `Dynamic`, `Sequence` e `SetPage` para free functions em `rules/layout/`.
- `4b6772bf6` — A4: testes `repr_value_complex_types`, `layout_divider_emite_shape_line`, `layout_link_preserva_url_e_texto`, `layout_transform_preserva_shape`.

**Validação**:
- `cargo check -p typst-core` e `cargo check -p typst-infra` passam.
- `crystalline-lint .` → 0 errors.
- Tests novos passam.
- Suite completa `cargo test -p typst-core --lib` falha apenas em `p350c_flag_on_nao_convergente_classifica` por stack overflow preexistente.

**Scope-out / bloqueadores**:
- A7: annotation URI completa para links PDF requer `pos`/`size` em `FrameItem::Link` — decisão arquitetural, não forçada.
- A4: metadata/state deixados de fora por baixo valor/demanda linguística.

**Notas epistêmicas**:
- **ADR-0107**: todas as alterações são mecânicas (transporte de dado, remoção de warnings, extração de função, testes). Nenhuma decisão de semântica linguística foi tomada.
- **ADR-0108**: cada agente começou com medição (grep/cargo check) antes da implementação.
- **ADR-0109**: A3 usou forma B (free functions na camada de regras) em vez de adicionar métodos aos elementos.
- **Honestidade**: teste `p350c_flag_on_nao_convergente_classifica` continua falhando, mas foi isolado como preexistente.

**Próximo passo sugerido**: P426 pode atacar o bloqueador de annotation URI (S-M), ou continuar varredura mecânica em outras camadas (`typst-shell`, `typst-wiring`).
