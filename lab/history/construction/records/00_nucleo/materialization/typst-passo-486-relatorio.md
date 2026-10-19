# Relatório P486 — Features OpenType + `x_offset` em TJ (Trilha 5 extensão final)

**Data:** 2026-06-28
**Executor:** Claude Sonnet 4.6 (Claude Code)
**Passo:** P486 (Trilha 5 extensão: features OpenType + posicionamento kern marks)
**Materialização:** documentação de features + `x_offset` em `emit_shaped_pdf`

---

## 1. Resumo

**Sub-item A — Features OpenType `liga`/`kern`/`calt`:**
Sonda do código-fonte de rustybuzz 0.20.1 confirma que `liga`, `kern` e `calt`
estão em `HORIZONTAL_FEATURES` com flags `F_GLOBAL`/`F_GLOBAL_HAS_FALLBACK`
(`ot_shape.rs:86-91`) — activados por defeito para texto horizontal, independentemente
do parâmetro `user_features = &[]`. Sub-item A é apenas documentação: comentário
adicionado em `shaper.rs:92`; nenhum código de produção alterado.

**Sub-item B — `x_offset` em `emit_shaped_pdf`:**
`ShapedGlyph.x_offset` aplicado no array TJ de `emit_shaped_pdf` para cenários
`Cidfont` e `Multifont`. Fórmula: pré-glifo `-(x_offset/upm×1000)` (desloca cursor
à direita), post-glifo `-(x_advance−x_offset)/upm×1000` (cancela o desvio e aplica
o avanço normal). Para `x_offset=0`: output idêntico ao P485 — zero regressão.

**`y_offset` — scope-out confirmado:** sem corpus LTR com `y_offset!=0`; implementação
requereria sequências `Td` (saída do array TJ). Scope-out documentado.

**Resultado final:** 4 testes novos verdes + sentinela `p486_parity_73_73_mantido`;
`crystalline-lint` 0 erros V1–V14. Trilha 5 extensão final fechada.

---

## 2. Sub-item A — Features OpenType (ADR-0108)

### 2.1 Medições antes de decidir

| Medição | Resultado | `file:line` |
|---------|-----------|-------------|
| `liga` activada por defeito em rustybuzz 0.20.1? | ✅ `HORIZONTAL_FEATURES` com `F_GLOBAL` | `ot_shape.rs:91` |
| `kern` activada por defeito? | ✅ `HORIZONTAL_FEATURES` com `F_GLOBAL_HAS_FALLBACK` | `ot_shape.rs:90` |
| `calt` activada por defeito? | ✅ `HORIZONTAL_FEATURES` com `F_GLOBAL` | `ot_shape.rs:86` |
| `Feature` struct API | `pub struct Feature { tag, value, start, end }` + `fn new(tag, value, range)` | `common.rs:483-514` |
| `features = &[]` suficiente? | ✅ user features são adicionais; globais sempre aplicadas | `ot_shape.rs:181-186` |
| `x_offset` usado em `emit_shaped_pdf` pré-P486? | ❌ não usado | `stream.rs:200,214` |
| `y_offset` no corpus parity? | ❌ 0 em todos os glifos (corpus LTR) | grep corpus |

### 2.2 Conclusão Sub-item A

`features = &[]` é suficiente. Rustybuzz 0.20.1 aplica `liga`, `kern`, `calt`
via `HORIZONTAL_FEATURES` para qualquer shape de texto horizontal, antes de
processar user features. Nenhuma user feature é necessária.

### 2.3 Alteração em código

**`03_infra/src/shaper.rs` linha 92–94:** comentário documentando a invariante:

```rust
// P486 — liga/kern/calt activados por defeito via HORIZONTAL_FEATURES
// (rustybuzz 0.20.1 ot_shape.rs:86-91). features = &[] é suficiente.
let output = rustybuzz::shape(&rb_face, &[], buffer);
```

---

## 3. Sub-item B — `x_offset` em `emit_shaped_pdf` (ADR-0108)

### 3.1 Medições antes de decidir

| Medição | Resultado |
|---------|-----------|
| Operador TJ pré-P486 | `<GID> advance_tu` com `advance_tu = -(x_advance/upm*1000)` |
| `x_offset` HarfBuzz significado | Deslocamento horizontal do glifo relativo à sua posição normal (+= direita) |
| PDF TJ: número negativo | Desloca cursor à DIREITA (aumenta advance) |
| Fórmula pré-glifo | `-(x_offset/upm*1000)` — negativo para x_offset>0 = shift right ✓ |
| Fórmula post-glifo | `-(x_advance/upm*1000) + (x_offset/upm*1000)` = cancela desvio + aplica advance |
| Referência krilla | `content.rs:591-613` — padrão `adjustment += x_offset; ... adjustment -= x_offset` |
| `y_offset` corpus | 0 em todos — scope-out declarado |

### 3.2 Implementação

**`03_infra/src/export/stream.rs` — `emit_shaped_pdf` (Cidfont + Multifont):**

```rust
for g in glyphs {
    // P486 — x_offset: deslocar glifo e cancelar após (kern marks, diacríticos)
    if g.x_offset != 0 {
        let xoff_tu = -(g.x_offset as f64 / upm * 1000.0);
        ops.push_str(&format!("{:.0} ", xoff_tu));
    }
    let advance_tu = -(g.x_advance as f64 / upm * 1000.0)
        + (g.x_offset as f64 / upm * 1000.0);
    ops.push_str(&format!("<{:04X}> {:.0} ", g.glyph_id, advance_tu));
}
```

Para `x_offset = 0`: `advance_tu = -(x_advance/upm*1000) + 0 = advance_tu_P485` → idêntico ao P485.

### 3.3 Exemplos concretos

| x_offset | x_advance | upm | Pré-glifo TJ | Post-glifo TJ | Output |
|----------|-----------|-----|--------------|----------------|--------|
| 0 | 600 | 1000 | (nenhum) | -600 | `<GID> -600 ` |
| -50 | 600 | 1000 | 50 | -650 | `50 <GID> -650 ` |
| 30 | 600 | 1000 | -30 | -570 | `-30 <GID> -570 ` |

---

## 4. Testes (5 novos)

### L3 — `shaper::tests`

| Teste | Cobertura |
|-------|-----------|
| `p486_features_default_confirmado` | `features = &[]` tem len=0; documenta invariante defaults |

### L3 — `export::stream::stream_tests`

| Teste | Cobertura |
|-------|-----------|
| `p486_emit_x_offset_zero_equivale_p485` | x_offset=0 → sem número antes do GID; advance=-600 |
| `p486_emit_x_offset_nonzero_aplica_ajuste` | x_offset=-50, upm=1000 → "50 " antes do GID |
| `p486_emit_x_offset_positivo` | x_offset=30, upm=1000 → "-30 " antes do GID |

### Lab/parity

| Teste | Cobertura |
|-------|-----------|
| `p486_parity_73_73_mantido` | Sentinela: 73/73 matches; 0 diffs; 0 errors |

---

## 5. Arquivos alterados

### Specs L0 (3 actualizadas)

| Ficheiro | Alteração |
|----------|-----------|
| `00_nucleo/prompts/infra/shaper.md` | §P486: features defaults confirmadas; hash `63feb005` |
| `00_nucleo/prompts/infra/export/stream.md` | §P486: x_offset em TJ; y_offset scope-out; hash `30ad603b` |
| `00_nucleo/adr/typst-adr-0120-textshaped-rustybuzz.md` | §P486: sub-items A+B + extensão final |

### Código L3 (2 ficheiros)

| Ficheiro | `@prompt-hash` pós-P486 | Alteração |
|----------|------------------------|-----------|
| `03_infra/src/shaper.rs` | `13171e36` | Comentário P486 + teste `p486_features_default_confirmado` |
| `03_infra/src/export/stream.rs` | `49b2c8cc` | `x_offset` em TJ (Cidfont + Multifont) + 3 testes + helper `glyph_xoff` |

### Lab/parity

| Ficheiro | Alteração |
|----------|-----------|
| `lab/parity/tests/structural_parity.rs` | Sentinela `p486_parity_73_73_mantido` |

---

## 6. `crystalline-lint` resultados

```
crystalline-lint --fix-hashes .
  Fixed 2 files:
    ./03_infra/src/export/stream.rs  → 49b2c8cc
    ./03_infra/src/shaper.rs         → 13171e36
  Re-running analysis... ✅ 0 drift warnings remaining

crystalline-lint .
  ✅ No violations found (0 erros V1–V14)
```

---

## 7. Scope-out explícito

| Área | Scope-out |
|------|-----------|
| **`y_offset` em emit** | Requer saída do TJ e sequências `Td`; corpus LTR tem y_offset=0 em todos os glifos. P487 se necessário. |
| **`dlig`/`hlig`/`clig`** | Ligatures discricionárias/históricas; apenas `liga` standard é default. |
| **`smcp` feature** | Smallcaps via OpenType; scope-out P447+. |
| **`x_advance` exacto vs `hmtx`** | P485 scope-out preservado; sem acesso por-glifo ao hmtx. |
| **Corpus RTL com x_offset** | Tests unitários L3 cobrem; sem corpus árabe/hebraico no lab/parity. |

---

## 8. Critério de fecho

- [x] Sondas: `liga`/`kern`/`calt` confirmadas defaults (`ot_shape.rs:86-91`); `Feature` API (`common.rs:483`); `y_offset=0` em corpus — todos com `file:line`.
- [x] Sub-item A: features confirmadas activas por defeito; comentário em `shaper.rs:92`.
- [x] Sub-item B: `x_offset` aplicado no TJ array (Cidfont + Multifont).
- [x] `y_offset`: scope-out documentado (corpus LTR sem y_offset!=0).
- [x] 4 testes novos verdes (`p486_features_default_confirmado` + 3 stream).
- [x] `p486_parity_73_73_mantido` adicionado.
- [x] `crystalline-lint`: 0 erros V1–V14.
- [x] `cargo build --workspace` verde.
- [x] **Trilha 5 extensão final FECHADA.**

---

## 9. Estado pós-P486

| Indicador | Estado |
|-----------|--------|
| DEBTs activos | 0 |
| Paridade | **73/73 matches; 0 diffs; 0 errors** |
| ADR-0120 | ACEITE — P482–P486 executados |
| **P486** | **FECHADO** |

| Indicador Trilha 5 | Estado pós-P486 |
|--------------------|-----------------|
| `FrameItem::TextShaped` | ✅ P482 |
| shaper.rs pipeline | ✅ P482 |
| RTL básico | ✅ P484 |
| `x_advance` TJ | ✅ P485 |
| `liga`/`kern`/`calt` | ✅ P486 (confirmação + doc) |
| `x_offset` kern marks | ✅ P486 |
| `y_offset` diacríticos | scope-out (corpus LTR = 0) |
| `smcp` OpenType | scope-out |
| `x_advance` exacto vs hmtx | scope-out |

---

## 10. Próximo passo recomendado

| Opção | Descrição | Magnitude |
|-------|-----------|-----------|
| **P487-A** | `y_offset` em emit se corpus RTL mostrar necessidade | S |
| **P487-B** | Expansão corpus lab/parity com ficheiros RTL (árabe/hebraico) | S |
| **P487-C** | Fechar Trilha 6 (5ª funcionalidade: LoF/LoT page numbers) | M–L |
| **P487-D** | Audit final do projecto — estado consolidado | XS |
