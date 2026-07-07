# Paridade de Produção — P599

**Data do relatório:** 2026-07-07
**Passo:** 599
**Foco:** Completar a validação de P598 com `cargo test --workspace` e `crystalline-lint .`.

---

## Resumo executivo

P598 validou apenas `cargo test -p typst-core`. Este passo correu a validação completa do workspace.

Resultado: `cargo test --workspace` passou após actualização de 4 snapshots binários do crate `typst-infra` (P307b). As falhas iniciais foram causadas pela mudança de margem de `70.87 pt` (valor hard-coded anterior) para `70.8666... pt` (valor calculado pela fórmula do vanilla). A diferença é puramente mecânica (bytes do PDF) e imperceptível visualmente.

`crystalline-lint .` passou sem violações.

---

## Proveniência

- **Hash base:** `79a29bdde4fda3a9a3f20d087c1a952c7e2f946c`
- **Data/hora:** 2026-07-07T20:34:13-03:00
- **Estado da working tree:** apenas os snapshots PDF de `03_infra/fixtures/p307b/reference/` foram modificados em resultado da validação.

---

## Validação completa

### `cargo test --workspace`

Corridos todos os crates do workspace. Resultados por crate:

| Crate | Resultado |
|-------|-----------|
| `typst-core` | 3581 passed; 0 failed |
| `typst-infra` | 598 passed; 0 failed; 5 ignored |
| `typst-shell` | 24 passed; 0 failed |
| `typst-wiring` (lib) | 21 passed; 0 failed |
| `typst-wiring` (crystalline_lint) | 2 passed; 0 failed |
| Doc-tests | 0 passed; 0 failed; 3 ignored |

**Total:** 0 falhas em todo o workspace.

### Falhas iniciais e correção

Na primeira corrida, 4 snapshot tests de `typst-infra` falharam:

- `p307b_04_shapes`
- `p307b_06_gradient_conic`
- `p307b_08_image_jpeg`
- `p307b_09_cidfont`

Mensagem de erro comum:

```text
PDF binário regrediu: 04-shapes | actual=1149B expected=1149B
```

Causa: a margem A4 passou de `70.87 pt` para `70.8666... pt`, o que deslocou coordenadas dos elementos nos PDFs em ~0.003 pt. Comparação byte-a-byte dos snapshots detectou a mudança.

Confirmação extraída dos floats dos PDFs (exemplo do fixture `04-shapes`):

```diff
-100.870
+100.867
-130.870
+130.867
-70.870
+70.867
```

A única diferença é a margem e os valores derivados dela.

Correcção: os snapshots foram regenerados com `UPDATE_P307B_SNAPSHOTS=1`, mecanismo previsto no próprio teste (P520) para alterações intencionais do output PDF.

### `crystalline-lint .`

```text
✓ No violations found
```

---

## Decisão

P598 está validado e pode ser considerado fechado. A fórmula de margem automática permanece inalterada; os snapshots binários foram actualizados para reflectir a nova margem calculada.

---

## Critérios de fecho do passo

- [x] `cargo test --workspace` completo: 0 falhas.
- [x] `crystalline-lint .`: 0 violações.
- [x] Falhas iniciais investigadas e corrigidas (regeneração de snapshots).
- [x] Relatório escrito com hash do commit e proveniência.

---

## Ligações

- `00_nucleo/materialization/typst-passo-599.md` — passo que originou esta verificação.
- `00_nucleo/diagnosticos/paridade-producao-p598.md` — relatório do passo validado.
- `03_infra/src/p307b_snapshot_tests.rs` — testes de snapshot com mecanismo de regeneração.
