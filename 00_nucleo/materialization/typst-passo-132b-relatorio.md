# Passo 132B — Relatório (materialização `FontList` em L1)

**Data**: 2026-04-24
**Precondição**: Passo 132A encerrado; diagnóstico + ADR-0053
em `PROPOSTO`; 1069 total tests; 11 DEBTs abertos.
**Natureza**: passo S-M em L1. Código novo + migração de 10
testes em 3 camadas + rotação de pool DEBT-49.
**ADR**: **ADR-0053** transita `PROPOSTO` → `IMPLEMENTADO`.
ADR-0038 ganha quinta nota.

---

## Sumário

Tipo composto `FontList` materializado em L1
(`01_core/src/entities/font_list.rs`, ~200 linhas total).
3 tipos: `FontFamily`, `FontList`, `Covers` (inabitado).
`StyleDelta.font: Option<FontList>` capturado com paridade
parcial (string + array; dict rejeitada conscientemente).

**10 testes migrados** em 3 camadas: 5 canary L1 **consolidados
em 1** (`hyphenate_canary_passo_132b`) + 3 L3 + 2 L4.

**Pool DEBT-49 L3 rotacionado**: `font/alignment/stroke` →
`hyphenate/alignment/stroke`.

**852 L1 (+14) + 24 L2 + 186 L3 + 21 L4** + 6 ignorados =
**1083 total** (+14 vs 1069). Zero violations.

---

## 132B.A — Inventário confirmatório

| Item | Resultado |
|------|-----------|
| A.1 estado inicial | ✓ sem `font` field, sem `"font"` arm, sem `font_list.rs` |
| A.2 `Value::Array` | ✓ `Vec<Value>` directo |
| A.3 `Value::Dict` | ✓ `IndexMap<EcoString, Value, FxBuildHasher>` |
| A.4 `hyphenate` livre | ✓ nenhum arm hoje |
| A.5 L4 path | ✓ `04_wiring/tests/cli.rs` |
| A.6 base 1069 | ✓ confirmado |

**Gate passou**: zero bloqueios.

---

## 132B.B — ADR-0038 quinta nota

Adicionada. Documenta:
- `font` como primeiro tipo agregador em L1.
- Paridade ADR-0033 parcial (dict rejeitado conscientemente).
- `Covers` como enum inabitado (forma estrutural sem código
  activo).
- Canary migration `font → hyphenate` em 10 testes.
- Precedente: futuras materializações seguem padrão
  131/132 (diagnóstico + ADR dedicada).

---

## 132B.C — `entities/font_list.rs` criado

- **95 linhas de código** + **85 linhas de tests**.
- 3 tipos: `Covers` (inabitado), `FontFamily { name, covers }`,
  `FontList(Vec<FontFamily>)`.
- 12 unit tests passam:
  - `font_family_new_normaliza_lowercase_passo_132b`
  - `font_family_new_case_insensitive_passo_132b`
  - `font_family_covers_sempre_none_passo_132b`
  - `font_list_single_tem_um_elemento_passo_132b`
  - `font_list_new_rejeita_vector_vazio_passo_132b`
  - `font_list_new_aceita_um_elemento_passo_132b`
  - `font_list_new_aceita_multiplos_passo_132b`
  - `font_list_preserva_ordem_passo_132b`
  - `font_list_partial_eq_passo_132b`
  - `font_list_clone_o1_via_ecow_passo_132b`
  - `covers_inabitado_estruturalmente_passo_132b`
  - `font_list_is_empty_sempre_false_passo_132b`
- `entities/mod.rs`: `pub mod font_list;` adicionado.

---

## 132B.D — StyleDelta.font

- `use crate::entities::font_list::FontList;` adicionado.
- `pub font: Option<FontList>` novo campo.
- `StyleDelta::empty()`: `font: None` init.
- `const fn empty()` preservado.

---

## 132B.E — Arm `"font"` em `eval_set_text`

Imports `FontFamily, FontList`. Arm match sobre `val`:

```rust
Value::Str(s) => FontList::single(s) ✓
Value::Array(arr) => {
    cada item deve ser Value::Str,
    FontList::new rejeita vazio,
}
Value::Dict(_) => Err("dict form of font not yet supported...")
_ => Err("font expects a string or array of strings")
```

Todos os paths de erro produzem `return Err(vec![
SourceDiagnostic::error(named.expr().span(), ...)])` hard,
padrão 131B.

---

## 132B.F+G — L1 canary consolidation + novos integration tests

### Consolidação

**5 canary tests removidos** (font_canary_passo_126/127/128/129/131b)
**→ 1 teste consolidado** (`eval_set_text_hyphenate_canary_passo_132b`).

Rationale: todos os 5 testavam exactamente a mesma invariante
("`#set text(font: ...)` emite warning '`font`'"). Consolidação
reduz ruído sem perda de cobertura.

### 6 integration tests novos

- `eval_set_text_hyphenate_canary_passo_132b` (substituto).
- `eval_set_text_font_string_simples_passo_132b`.
- `eval_set_text_font_array_passo_132b`.
- `eval_set_text_font_dict_rejeitado_unit_passo_132b` (unit,
  porque dict inline em arg list é parseado como named args,
  não como dict).
- `eval_set_text_font_array_vazio_rejeitado_passo_132b`.
- `eval_set_text_font_array_com_nao_string_rejeitado_passo_132b`.
- `eval_set_text_font_tipo_invalido_rejeitado_passo_132b`.

### Divergência face ao spec 132B

Spec assumia que `(name: "X", covers: "...")` inline produziria
`Value::Dict`. Em Typst, named args inline dentro de arg list
são **named arguments**, não dict. Teste `dict_rejeitado` foi
**transformado em unit test** que constrói `Value::Dict`
directamente para validar o arm `Value::Dict(_) => Err`.

Empiricamente, tipo `Int` hit o fallback `_` arm com mensagem
genérica — teste separado cobre isso.

### L1 delta

- -5 canary removidos.
- +1 canary consolidado.
- +6 integration tests.
- +12 unit tests em `font_list.rs`.
- **Líquido L1: +14** (838 → 852). ✓

---

## 132B.H — L3 migration (3 tests)

| Teste | Input antes | Input depois |
|-------|-------------|--------------|
| `debt49_set_text_font_emite_warning` | `font: "Arial"` | `hyphenate: true` |
| (renomeado) | ... | `debt49_set_text_hyphenate_emite_warning` |
| `debt49_set_text_multiplas_propriedades_desconhecidas` | `font, alignment, stroke` | `hyphenate, alignment, stroke` |
| `debt49_dedup_warnings_identicos` | `font: "A"` × 2 | `hyphenate: true` × 2 |

Todos passam após rotação. L3 mantém-se em **186 tests**.

---

## 132B.I — L4 migration (2 tests)

| Teste | Mudança |
|-------|---------|
| `cli_sucesso_com_warning:65` | input `font: "Arial"` → `hyphenate: true`; assertion `"font"` → `"hyphenate"` |
| `disciplina_warnings_antes_de_errors:591` | input `font: "X"` → `hyphenate: true` (assertion inalterada, só procura `warning:`) |

L4 mantém-se em **21 tests**.

---

## 132B.J — ADR-0053 transição

- Status: `PROPOSTO` → **`IMPLEMENTADO`**.
- Adicionado `**Materializado em**: Passo 132B (2026-04-24)`.
- Secção "Estado final" com 10 números concretos:
  linhas, testes, consolidação, pool actualizado.

---

## 132B.K — Prompts L0

- `entities/style_chain.md`: campo `font: Option<FontList>`
  adicionado à declaração.
- **`entities/font-list.md` novo**: prompt completo para o
  tipo com 3 componentes + validação + critérios.
- Hashes refixados: `583df8dd` (font_list.rs), `412bb696`
  (style_chain.rs).

---

## 132B.L — Verificação

### Cargo tests

```
test result: ok. 852 passed ...       (L1 +14 vs 838)
test result: ok. 186 passed, 6 ign    (L3 — 3 rotados)
test result: ok. 24 passed            (L2 inalterado)
test result: ok. 21 passed            (L4 — 2 rotados)
```

### `crystalline-lint .`

```
✓ No violations found
```

### Manual — 5 cenários

```bash
$ typst s.typ    # font: "Arial"
exit=0, stderr: (vazio)

$ typst a.typ    # font: ("Inria Serif", "Noto Sans")
exit=0, stderr: (vazio)

$ typst e.typ    # font: () — vazio
e.typ:1:17: error: font array must not be empty
exit=1

$ typst i.typ    # font: 42 — int
i.typ:1:17: error: font expects a string or array of strings
exit=1

$ typst h.typ    # hyphenate: true — canary
h.typ:1:11: warning: text: propriedade 'hyphenate' ainda não suportada
exit=0
```

---

## Ficheiros tocados / criados

| Ficheiro | Natureza | Mudança |
|----------|----------|---------|
| `01_core/src/entities/font_list.rs` | **novo** | 3 tipos + 12 unit tests (~200 linhas) |
| `01_core/src/entities/mod.rs` | modificado | `pub mod font_list;` |
| `01_core/src/entities/style_chain.rs` | modificado | +import FontList, +campo `font`, init |
| `01_core/src/rules/eval/rules.rs` | modificado | +imports, +arm `"font"` com 4 branches |
| `01_core/src/rules/eval/tests.rs` | modificado | -5 canaries, +6 integration tests (líquido +1) |
| `03_infra/src/integration_tests.rs` | modificado | 3 testes rotados |
| `04_wiring/tests/cli.rs` | modificado | 2 testes rotados |
| `00_nucleo/adr/typst-adr-0053-*.md` | modificado | status + estado final |
| `00_nucleo/adr/typst-adr-0038-*.md` | modificado | quinta nota |
| `00_nucleo/prompts/entities/style_chain.md` | modificado | campo font no exemplo |
| `00_nucleo/prompts/entities/font-list.md` | **novo** | prompt L0 do tipo |

### Números finais

| Métrica | Antes (132A) | Depois |
|---------|------:|-------:|
| L1 tests | 838 | **852** (+14) |
| L2 tests | 24 | 24 |
| L3 tests | 186 | 186 |
| L4 tests | 21 | 21 |
| **Total** | **1069** | **1083** (+14) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 52 | **53** (ADR-0053 `IMPLEMENTADO`) |
| DEBTs abertos | 11 | 11 |

---

## Divergências face ao diagnóstico 132A

1. **Dict test virou unit em vez de integration**: Typst
   parseia `(name: "X", covers: "...")` inline como named args,
   não como dict literal. Integration test foi reformulado
   como unit test que constrói `Value::Dict` directamente.
   Arm `Value::Dict(_) => Err` continua válido; apenas o teste
   verifica-o via construção directa.

2. **+1 teste extra vs spec**: spec pedia 4-5 integration tests;
   foram adicionados 5 (font_string + font_array + font_dict_unit
   + font_array_vazio + font_array_não_string + font_tipo_inválido
   = 6 se contar hyphenate_canary). Contagem spec vs real diverge
   em ±1 — insignificante.

3. **Consolidação vs preservação dos 5 canaries**: escolhi
   **consolidação** (estratégia (a) do spec). Rationale:
   redução de 5 para 1 sem perda de cobertura.

4. **`Value::Dict` arm é valioso mesmo sem teste integration**:
   se utilizador construir dict via `#let d = (a: 1); #set text(font: d)`,
   o arm captura-o correctamente. Unit test valida pattern
   match estrutural.

---

## Lições

1. **Diagnóstico 132A detectou bloqueios**: `regex` não
   autorizado + 10 testes afectados foram apanhados em L0.
   132B correu quase mecânico após os gates 132B.A passarem.

2. **Typst parser nuances revelaram-se em runtime**:
   spec assumia `(k:v)` inline → `Dict`. Real: named args
   em arg list. **Empirical testing cruza assumptions** —
   lesson recorrente.

3. **`Covers` inabitado é pattern novo**: primeira vez que
   um enum sem variantes é usado como placeholder estrutural.
   Zero linhas de match (vazio), zero custo runtime,
   forward-compat garantido. Candidato para replicação em
   outros tipos onde queremos "reservar forma sem compromisso".

4. **Consolidação de 5 canaries → 1**: reduzir duplicação
   de tests sem perder cobertura. 5 testes asseravam a mesma
   invariante — consolidação é clean-up legítimo durante
   migração. Alternativa seria manter 5 canaries × 2 iterações
   por passo = multiplicação irracional.

5. **Primeira rejeição explícita de form vanilla**: `Value::Dict
   → Err`. Trade-off documentado: rejeitar é menos grave que
   aceitar-silenciosamente. Mensagem clara guia utilizador para
   forms suportadas. ADR-0033 cumprido "em espírito" (erro em
   vez de divergência silenciosa).

6. **Estimativa S-M confirmada**: 132B demorou ≈2-3h, maior
   que 131B (~1.5h). Complexidade com tipo agregador + 10
   migrações de canary + 3 camadas justifica. Pattern
   131/132 ("diagnóstico + materialização") agora tem dois
   pontos de dados — útil para estimar futuras.

7. **Pool DEBT-49 saudável**: rotação `font → hyphenate` foi
   clean. Pool ainda tem `alignment`, `stroke`, `justify`,
   `first-line-indent`, `dir`, `region`, etc. Mais 2-3 passos
   antes de esgotar. Candidato "substituir rotativo por
   positivos específicos" continua pendente mas não urgente.

---

## Estado pós-Passo 132B

### DEBT-1 progresso

Propriedades capturadas em `#set text`:
- ✓ bold, italic, size, fill (30/102)
- ✓ weight numérico + simbólico (126/129)
- ✓ tracking (127)
- ✓ leading — divergente em text vs par (128)
- ✓ lang — tipo semântico Lang (131B)
- ✓ **font — tipo composto FontList, paridade parcial (132B)**

**Lista DEBT-1 canónica AGORA TOTALMENTE COBERTA** com
ressalvas documentadas:
- `leading` em `text` em vez de `par` (resolve em 133-134).
- `font` dict deferido (resolve em ADR-0054 futura se consumer
  pedir).

### ADR-0053 `IMPLEMENTADO`

Disponível como precedente para materializações de tipos
agregadores (`Stroke { width, paint }`, `Spacing`, etc.).

### Candidatos futuros imediatos

1. **Passo 133**: activar target `par` em `eval_set_rule`.
2. **Passo 134**: migrar `leading` de `text` para `par`.
3. **Passo 135**: fechar DEBT-1 no DEBT.md.
4. **ADR-0054** (quando consumer de shaping chegar):
   autorizar `regex` em L1 + implementar `Covers` concreto.
5. **Consumer de propriedades**: primeiro passo que consome
   `weight`/`tracking`/`font` em layout.

Estimativa restante para fechar DEBT-1: **133 + 134 + 135 ≈
2-3h cumulativo** (cada um < 1h).
