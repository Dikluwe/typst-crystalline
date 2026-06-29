---

# P489 — Verificação de estado: audit empírico pós-P488

> **Passo:** 489
> **Data:** 2026-06-29
> **Foco:** Verificar empiricamente o estado real do projecto após P488. Confirmar via `grep`, `cargo test`, e `crystalline-lint` o que está efectivamente implementado, o que é scope-out declarado, e o que pode ter ficado desalinhado entre L0 e L1 durante os passos P465–P488. Zero código de produção — apenas sondas e documentação.
> **Tipo:** Audit empírico.
> **Tamanho:** S (~25 min).

---

## Contexto

Passaram 24 passos desde o roteiro P463 (P465–P488). A maioria foi de materialização rápida com sonda-first — mas em passos rápidos, L0 e L1 podem ter ficado desalinhados (prompts desactualizados, scope-outs não documentados, itens marcados como completos que podem ter regressões silenciosas).

Este passo faz um audit empírico do estado real antes de declarar o projecto concluído.

---

## Grupo 1 — Build e testes

```bash
# 1. Build limpo
cargo build --workspace 2>&1 | grep -E "^error"

# 2. Testes completos (com stack suficiente)
RUST_MIN_STACK=33554432 cargo test --workspace 2>&1 | tail -20

# 3. Crystalline-lint
crystalline-lint . 2>&1 | grep -E "^V[0-9]|violation"
```

**Critério:** zero erros de build; zero falhas de teste (excluindo stack overflow pré-existentes documentados em P487-D); zero violations V1–V14.

---

## Grupo 2 — Paridade corpus

```bash
# Suite de paridade
cd lab/parity
RUST_MIN_STACK=33554432 cargo test --test structural_parity -- --nocapture 2>&1 | grep -E "matches|diffs|errors|INCLUDE|SKIP"
```

**Critério:** ≥73 matches; 0 diffs; 0 errors (excluindo RTL que é SKIP-feature).

---

## Grupo 3 — Inventário de DEBTs activos

```bash
grep -n "EM ABERTO\|⚠\|BLOCKED\|\[ \]" 00_nucleo/DEBT.md | grep -v "^--$" | head -30
```

**Critério:** zero DEBTs com critério de fecho activo. DEBT-9 é instrumento contínuo — não é item a fechar.

---

## Grupo 4 — L0 drift: prompts desalinhados

```bash
crystalline-lint . 2>&1 | grep "V5\|V7\|drift\|orphan\|mismatch" | head -20
```

**Critério:** zero warnings V5 (hash drift entre L0 e L1). Warnings V7 (prompts órfãos) são aceitáveis se referenciados em scope-outs.

---

## Grupo 5 — Verificação de funcionalidades chave

Confirmar via `grep` que as funcionalidades materializadas em P465–P488 estão presentes:

```bash
# Trilha 5 — shaping
grep -rn "FrameItem::TextShaped\|ShapedGlyph\|shape_document" 01_core/src/ 03_infra/src/ | wc -l

# Trilha 6 — LoF/LoT page numbers
grep -rn "figure_page_numbers\|table_page_numbers\|known_figure" 01_core/src/ | wc -l

# Trilha 8 — marcadores list/enum
grep -rn "ListMarker\|EnumNumbering" 01_core/src/ | wc -l

# Trilha 4 — operadores cor
grep -rn "fn lighten\|fn darken\|fn saturate\|fn mix\|fn negate" 01_core/src/ | wc -l

# Symbol + sym module
grep -rn "build_sym_dict\|SYM_TABLE\|Value::Symbol" 01_core/src/ | wc -l

# unicode-bidi
grep -rn "unicode_bidi\|BidiInfo\|bidi_runs" 03_infra/src/ | wc -l
```

**Critério:** todas as contagens > 0 (funcionalidades presentes). Se alguma retorna 0, investigar e documentar.

---

## Grupo 6 — Scope-outs permanentes confirmados

Verificar que os scope-outs declarados são de facto scope-outs (não estão parcialmente implementados de forma inconsistente):

| Scope-out | Verificação | Critério |
|-----------|-------------|---------|
| `FrameItem::Text` remoção | `grep -rn "FrameItem::Text {" 01_core/src/ \| grep -v "deprecated\|allow"` | Deve existir (é tipo pré-shaping permanente) |
| `y_offset` em emit | `grep -rn "y_offset" 03_infra/src/export/stream.rs` | Deve retornar 0 (não implementado) |
| RTL stdlib `dir:` | `grep -rn "dir.*rtl\|rtl.*dir\|Direction::RTL" 01_core/src/rules/` | Deve retornar 0 (não implementado) |
| ColorSpace runtime user-facing | `grep -rn "ColorSpace\|color_space" 01_core/src/rules/stdlib/` | Verificar que não há funcs user-facing |
| `saturate`/`desaturate` em color module | `grep -rn "native_color_saturate" 01_core/src/` | Deve existir (foi implementado em P477) |

---

## Grupo 7 — Sentinelas de testes chave

Verificar que os testes sentinel dos passos críticos estão presentes e passam:

```bash
RUST_MIN_STACK=33554432 cargo test -p typst-core -- \
  p468 p469 p470 p471 p472 p473 p474 p475 p476 p477 \
  p480 p482 p483 p484 p485 p486 p488 \
  --nocapture 2>&1 | grep -E "PASSED|FAILED|ok|FAILED"

RUST_MIN_STACK=33554432 cargo test -p typst-infra -- \
  p482 p483 p484 p485 p486 p488 \
  --nocapture 2>&1 | grep -E "PASSED|FAILED|ok|FAILED"
```

**Critério:** todos os testes com prefix `p4xx` que existem passam.

---

## Grupo 8 — ADR-0120 estado final

```bash
grep -n "Estado\|PROPOSTO\|ACEITE\|Fase" 00_nucleo/adr/typst-adr-0120-textshaped-rustybuzz.md | head -20
```

**Critério:** ADR-0120 em estado ACEITE; todas as Fases 1–4 documentadas; scope-outs permanentes listados.

---

## Output esperado deste passo

### Tabela de estado final

| Área | Estado confirmado | Notas |
|------|-------------------|-------|
| Build | ✅ / ❌ | — |
| Testes | ✅ / ❌ | N passed, M failed |
| crystalline-lint | ✅ / ❌ | 0 violations / N violations |
| Paridade corpus | ✅ / ❌ | X/73 matches |
| DEBTs activos | ✅ / ❌ | N activos |
| L0 drift | ✅ / ❌ | N warnings V5 |
| Funcionalidades chave | ✅ / ❌ | Ver contagens Grupo 5 |
| Scope-outs confirmados | ✅ / ❌ | Ver Grupo 6 |
| Sentinelas p4xx | ✅ / ❌ | N/M passam |
| ADR-0120 | ✅ / ❌ | ACEITE / PROPOSTO |

### Se algum item é ❌

Documentar a causa e propor o fix mínimo. Se o fix é XS (< 5 min), executar inline neste passo. Se é S+, abrir como P490.

---

## Critério de fecho

- [ ] Todos os 8 grupos de sondas executados com output registado.
- [ ] Tabela de estado final preenchida com resultados empíricos reais.
- [ ] Zero itens ❌ sem plano de resolução documentado.
- [ ] Se algum ❌ é fix XS: fix executado e confirmado verde.
- [ ] `crystalline-lint .` zero violations ao fechar.
- [ ] Relatório de estado final produzido como `00_nucleo/materialization/estado-final-p489.md`.

---

## Próximo passo (P490+)

Dependendo do resultado:

- **Se tudo ✅:** P490 é o passo de documentação final — actualizar roteiro, produzir changelog de P465–P489, declarar projecto concluído.
- **Se há ❌ XS:** corrigir neste passo; P490 é a documentação final.
- **Se há ❌ S+:** P490 é o fix; P491 é a documentação final.

---

## Estado pós-P488 (declarado nos relatórios)

| Indicador | Estado declarado |
|-----------|-----------------|
| `cargo test --workspace` | ✅ 3435+526+... verde |
| `crystalline-lint .` | ✅ 0 violations |
| Paridade | 73/73 matches |
| DEBTs activos | 0 |
| Trilha 5 | COMPLETA (P482–P486) |
| Trilha 6 | 5/5 COMPLETA (P488) |
| Todas as trilhas | COMPLETAS |

**P489 confirma empiricamente este estado.**
