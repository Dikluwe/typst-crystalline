# typst-passo-282 — Auditoria paridade emit local + estado percentual do projecto

**Objectivo**: duas auditorias diagnósticas em paralelo:

1. **Verificar paridade bit-exact** entre emit local (em `draw_item_local`) e emit top-level (em `build_page_stream`) para Text/Glyph/Line nos 3 font scenarios. Detectar simplificações inadvertidas que P281 possa ter introduzido.

2. **Levantar estado percentual do projecto** — cobertura empírica vs vanilla por categoria (Model / Layout / Visualize / Math / Text / Introspection / Export). Identificar onde estão as próximas frentes de trabalho concretas.

Passo administrativo zero-código.

---

## §1 — Sub-passo P282.A — Fase A1: auditoria paridade emit

Produz `00_nucleo/diagnosticos/diagnostico-paridade-emit-passo-282.md`.

### §A1.1 — Text emit: comparação top-level vs local

Para cada scenario (Type1, CidFont, Multifont):

```bash
# Localizar o emit top-level de Text em build_page_stream
rg -n -B 2 -A 30 "FrameItem::Text" 03_infra/src/export.rs

# Localizar emit_text_pdf (helper criado em P281)
rg -n -B 2 -A 30 "fn emit_text_pdf" 03_infra/src/export.rs

# Comparar literal o que cada um emite
```

**Verificar para cada scenario**:

| Aspecto | Top-level emite? | Local emite? | Diferença legítima? |
|---|---|---|---|
| `BT ... ET` envelope | (sim/não) | (sim/não) | (`cm` do Group substitui? ou ambos têm?) |
| `Tf` (font select + size) | (sim/não) | (sim/não) | — |
| `Td` ou `Tm` (posicionamento) | (sim/não) | (sim/não) | (top-level usa `page_height - y`; local usa `y` directo — paridade pós-`cm`) |
| `Tj` (text operator) | (sim/não) | (sim/não) | — |
| **Fill colour (`rg`)** | (sim/não) | (sim/não) | **suspeita: pode estar em falta local** |
| **Faux-bold (`Tr 2` + stroke width)** | (sim) Type1 | (?) | **suspeita: pode estar simplificado** |
| **Tracking (`Tc`)** | (sim) | (?) | **suspeita: pode estar simplificado** |
| **Style.bold/italic seleciona /F2/F3** (Type1) | (sim) | (?) | **suspeita: pode estar /F1 hardcoded** |

Output §A1.1 — confirmar ponto a ponto. Cada **suspeita** ou é refutada (paridade) ou confirmada (correção pendente).

### §A1.2 — Glyph emit: comparação

Análoga §A1.1 mas para `FrameItem::Glyph`:

- **Type1**: top-level e local ambos ignoram silenciosamente (sem fonte TrueType)? Confirmar paridade.
- **CidFont**: top-level emite `<XXXX> Tj`; local idem?
- **Multifont**: top-level seleciona `/F{i+1}` baseado em algum critério (relatório P281 §2.2 diz "Glyph emite `<{:04X}>` em /F1 (math fonts default)" — verificar se top-level também usa /F1 ou se selecciona por outro critério).

### §A1.3 — Line emit: comparação

`FrameItem::Line { start, end, thickness }`:

- **Top-level emite**:
  - `q` (push state)?
  - `RG` (stroke colour set)?
  - `w` (line width)?
  - `m`/`l`/`S` (path ops + stroke)?
  - `Q` (pop state)?
- **Local emite** (P281 §3.4 mostra snippet):
  ```rust
  "q {:.3} w {:.1} {:.1} m {:.1} {:.1} l S Q\n"
  ```
  **Falta `RG` literal**. Top-level provavelmente tem cor explícita.

Confirmar empíricamente:

```bash
rg -n -B 2 -A 10 "FrameItem::Line" 03_infra/src/export.rs
```

Output §A1.3 — comparação literal. Se `RG` estiver em top-level e falta em local, **bug confirmado**.

### §A1.4 — Síntese paridade

Tabela final:

| Variante | Scenario | Paridade bit-exact? | Acção |
|---|---|---|---|
| Text | Type1 | (sim/não) | — / fix |
| Text | CidFont | (sim/não) | — / fix |
| Text | Multifont | (sim/não) | — / fix |
| Glyph | Type1 | (sim/não — ambos ignored) | — |
| Glyph | CidFont | (sim/não) | — / fix |
| Glyph | Multifont | (sim/não) | — / fix |
| Line | (scenario-independent) | (sim/não) | — / fix |

Output §A1.4 — lista concreta de fixes pendentes (se algum).

---

## §2 — Sub-passo P282.B — Fase A2: estado percentual do projecto

Produz `00_nucleo/diagnosticos/estado-percentual-projecto-passo-282.md`.

Auditoria empírica por categoria. Para cada uma:

- **Cobertura empírica actual** (% features implementadas vs vanilla, baseado em inventário factual; não em declarações herdadas).
- **Próximos itens accionáveis** (sem bloqueador).
- **Pendências bloqueadas** (com motivo).

### §A2.1 — Categorias a auditar

1. **Model** (`Content` enum + variants estruturais).
2. **Layout** (engine/layout + page model).
3. **Visualize** (geometry + shapes + paint).
4. **Math** (rules/math).
5. **Text** (style + font + lang).
6. **Introspection** (counters + locations + queries).
7. **Export** (PDF infra).
8. **Stdlib** (funções nativas user-facing).
9. **Eval** (eval + show + set).
10. **CLI / Wiring**.

### §A2.2 — Método empírico

Para cada categoria:

```bash
# 1. Listar features cristalinas existentes
rg "Content::\w+|native_\w+|fn .*(?:rule|stdlib)" --type rust 01_core/src/ | sort -u

# 2. Listar features vanilla correspondentes (se disponível em lab/)
ls lab/typst-original/crates/typst-library/src/<categoria>/

# 3. Cross-reference contra inventários históricos
# (referenciar diagnósticos prévios P148, P156B, P259, etc. que já fizeram inventários)
```

### §A2.3 — Outputs por categoria

Tabela:

| Feature | Vanilla tem | Cristalino tem | Estado | Próximo passo plausível |
|---|---|---|---|---|
| (e.g. Model) `Content::Block` | sim | sim (P156G) | ✓ | — |
| (e.g. Model) `Content::Footnote` | sim | não | pendência | P-Footnote-N (M) |
| (e.g. Layout) `pad()` | sim | sim (P156C) | ✓ | — |
| (e.g. Layout) `columns` | sim | não | bloqueada | DEBT-56 (encerrado P221 com nota arquitectural) |
| (e.g. Visualize) `Curve` | sim | não | pendência | passo dedicado |
| ... | ... | ... | ... | ... |

### §A2.4 — Síntese percentual

Por categoria:

| Categoria | Cobertura empírica | Direção principal |
|---|---|---|
| Model | (%) | (item principal pendente) |
| Layout | (%) | ... |
| Visualize | (%) | ... |
| Math | (%) | ... |
| Text | (%) | ... |
| Introspection | (%) | ... |
| Export | (%) | ... |
| Stdlib | (%) | ... |
| Eval | (%) | ... |
| CLI | (%) | ... |

### §A2.5 — Lista de próximos passos sugeridos

Top 5-10 frentes de trabalho concretas, ordenadas por:
- Valor user-facing.
- Ausência de bloqueador.
- Continuidade com trabalho recente (cluster Gradient + arquitectura P281).

Cada item:
- Descrição factual (não declarativa).
- Pré-requisitos.
- Estimativa qualitativa de complexidade (sem cap numérico).

---

## §3 — Sub-passo P282.C — Materialização

### §C.1 — Fix paridade emit (condicional)

**Só dispara** se §A1.4 detectar diferenças bit-exact não-legítimas:

- Adicionar emit em falta (e.g. `RG` em Line; `Tc` em Text; faux-bold em Type1 Text).
- Testes regressão: comparar PDF byte-byte entre Text em Group vs Text top-level com mesmo `style`.
- Confirmar tests workspace continuam verdes.

Se §A1.4 retornar zero fixes pendentes, §C.1 não dispara.

### §C.2 — Relatório consolidado

Produz `/mnt/user-data/outputs/typst-passo-282-relatorio.md`:

- §1 — Achados auditoria paridade emit (lista factual; fixes aplicados ou refutados).
- §2 — Estado percentual por categoria (tabelas §A2.4).
- §3 — Próximos passos sugeridos (top 5-10).
- §4 — Decisão humana sobre P283+.

---

## §4 — Critério de fecho

P282 fecha quando:

- Fase A1 produzida; paridade emit auditada para Text/Glyph/Line × 3 scenarios.
- Fase A2 produzida; cobertura empírica por categoria.
- Fixes paridade aplicados se necessário (testes verdes; lint zero).
- Relatório com tabelas de estado + lista de próximos passos.

P282 não fecha se:

- Auditoria revela regressão pré-existente em tests baseline.
- Fix paridade introduz bug nova (regressão tests).
- Lint não-zero.

---

## §5 — Workflow operacional

1. Utilizador upload `03_infra/src/export.rs` actual (pós-P281) — necessário para Fase A1.
2. Claude Code executa Fase A1:
   - Compara emit top-level vs local literal.
   - Produz `typst-passo-282A1-paridade.md`.
3. Claude Code executa Fase A2:
   - Inventário empírico por categoria.
   - Produz `typst-passo-282A2-estado.md`.
4. Se A1 detecta fixes, executar §C.1; senão saltar.
5. Produz relatório consolidado §C.2.
6. Utilizador decide P283+ com mapa completo.

---

## §6 — Notas de execução

- **Auditoria primeiro; fix só se necessário.** Honestidade epistémica.
- **Cobertura empírica vs declarada**: inventários prévios (P148, P156B, P259) podem ter declarações desactualizadas. P282 verifica empíricamente.
- **Não criar ADRs novas.**
- **Outputs**: 3 ficheiros em `/mnt/user-data/outputs/`:
  - `typst-passo-282A1-paridade.md`.
  - `typst-passo-282A2-estado.md`.
  - `typst-passo-282-relatorio.md`.

---

*Spec produzida em 2026-05-XX. Auditoria paralela: paridade emit P281 (verifica suspeitas faux-bold/tracking/RG/multifont Glyph) + estado percentual do projecto (cobertura empírica por categoria; lista de próximos passos accionáveis).*
