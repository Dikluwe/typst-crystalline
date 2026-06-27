# Relatório P475 — Trilha 8: inset/outset Sides dict + Value::Relative em block/box

**Data:** 2026-06-27
**Executor:** Claude Sonnet 4.6 (Claude Code)
**Passo:** P475 (Trilha 8 — itens 7/8 e 8/8)
**Materialização:** Implementação + testes + atualização de spec L0

---

## 1. Resumo

P474 identificou dois itens remanescentes de Trilha 8:

- **(7/8) `inset`/`outset` em `block`/`box` com `Sides` dict** — aceitava apenas `Length` uniforme; agora aceita `Dict {left?, right?, top?, bottom?, x?, y?, rest?}` per-side com precedência específico > eixo > rest.
- **(8/8) `Value::Relative` em `extract_length`** — `50%` (ou `Rel { rel: f64, abs: Length }`) não era tratado; agora aceita e usa a parte `abs`, truncando `rel` (scope-out documentado).

Ambos os itens implementados. **Trilha 8: 8/8 completo (fechado).**

---

## 2. Sondas pré-implementação (ADR-0108)

| Sonda | Resultado | file:line |
|-------|-----------|-----------|
| `native_block` aceitava inset como `Sides` dict? | Não — só `extract_length` uniforme | `stdlib/layout.rs:780` (pre-P475) |
| `native_box` aceitava inset como `Sides` dict? | Não — mesma limitação | `stdlib/layout.rs:1017` (pre-P475) |
| `extract_length` tratava `Value::Relative`? | Não — braço ausente no `match` | `stdlib/layout.rs:~310` (pre-P475) |
| `Sides<Length>` tipo de domínio existia? | Sim — P156 | `entities/layout_types.rs` (Sides struct) |
| `Value::Relative(Rel<Length>)` existia? | Sim — P469 | `entities/value.rs` (variant) |
| `Value::Dict` = `IndexMap<EcoString, Value, FxBuildHasher>` confirmado? | Sim | `entities/value.rs` + `mod.rs:16-17` |
| Negativos rejeitados na cadeia `extract_sides_from_value`? | Não por omissão — adicionado | `stdlib/layout.rs:326–336` |
| Testes de regressão negativo pré-existentes? | Sim — 3 (block inset, block outset, box inset) | `stdlib/mod.rs:5763, 5961, 7438` |

**Conclusão:** Trabalho real, não Caso A. Implementação necessária em `layout.rs`.

---

## 3. Implementação

### 3.1 `extract_length` — suporte a `Value::Relative`

```rust
// P475 — Rel<Length> aceite: parte rel truncada a zero (scope-out).
Value::Relative(r) => Some(r.abs),
```

Localização: `stdlib/layout.rs:~318` (dentro de `fn extract_length`).
Semântica: `50% + 2pt` → `2pt`; `50%` → `Length::ZERO`. A parte relativa (`rel: f64`) é descartada — scope-out explícito para layout de comprimento fixo.

### 3.2 `extract_sides_from_value` — novo helper centralizado

```rust
fn extract_sides_from_value(val: &Value, fn_name: &str, field: &str) -> SourceResult<Sides<Length>> {
    // Rejeita negativos em qualquer caminho.
    // Uniforme: Length | Float | Int | Relative → Sides::uniform(l).
    // Dict: {left?, right?, top?, bottom?, x?, y?, rest?}
    //   precedência: específico > eixo (x/y) > rest.
    //   chave desconhecida → Err.
    // Outros tipos → Err.
}
```

Localização: `stdlib/layout.rs:323–380`.

Precedência per-side (espelha `native_pad` P156L):

| Chave específica | Eixo | Rest | Resultado |
|-----------------|------|------|-----------|
| `left: L` | — | — | `left = L` |
| — | `x: X` | — | `left = X`, `right = X` |
| — | — | `rest: R` | todos os lados = `R` |
| `left: L` | `x: X` | `rest: R` | `left = L` (específico ganha) |

### 3.3 `native_block` e `native_box` — integração

Antes (pre-P475):
```rust
// inset_uniform: Option<Length> = args.named.get("inset").and_then(extract_length)
// ... validação negativa inline ...
// Sides::uniform(inset_uniform.unwrap_or(Length::ZERO))
```

Depois (P475):
```rust
let inset = match inset_val {
    Some(val) => extract_sides_from_value(val, "block", "inset")?,
    None      => Sides::uniform(Length::ZERO),
};
```

Mesmo padrão aplicado a `outset` em `block` e a `inset`/`outset` em `box`.

---

## 4. Testes (6 novos)

| Teste | Cobertura |
|-------|-----------|
| `p475_block_inset_dict_per_side` | `{left: 3pt, right: 5pt}` → inset.left=3, right=5, top=0, bottom=0 |
| `p475_block_outset_dict_x_axis` | `{x: 2pt}` → outset.left=2, right=2, top=0 |
| `p475_box_inset_dict_per_side` | `{left: 3pt, top: 5pt}` → inset.left=3, top=5, right=0, bottom=0 |
| `p475_block_inset_uniforme_ainda_funciona` | `4pt` uniforme → todos os lados = 4pt (regressão) |
| `p475_relative_aceite_em_extract_length_parte_abs` | `Rel{0.5, 2pt}` → inset.left = 2pt |
| `p475_block_inset_dict_chave_invalida_erro` | `{diagonal: 1pt}` → Err |

Localização: `stdlib/mod.rs:11533–11629`.

---

## 5. Arquivos alterados

### Spec L0 (atualizada)

- `00_nucleo/prompts/rules/stdlib/layout.md` — inset/outset de `block` e `box`: `"Length uniforme"` → `"Length uniforme, Relative (parte abs), ou Dict {left?, right?, top?, bottom?, x?, y?, rest?} per-side (P475)"`. Testes canónicos adicionados.

### Código L1 (implementado)

- `01_core/src/rules/stdlib/layout.rs`:
  - `extract_length`: +braço `Value::Relative(r) => Some(r.abs)`.
  - `extract_sides_from_value`: novo helper (linhas 323–380) — uniforme + dict + rejeição de negativos.
  - `native_block`: `inset`/`outset` migrados para `extract_sides_from_value`.
  - `native_box`: `inset`/`outset` migrados para `extract_sides_from_value`.

### Testes (adicionados)

- `01_core/src/rules/stdlib/mod.rs` — 6 testes P475 (linhas 11533–11629).

---

## 6. Resultados dos testes

### Testes específicos P475 (6 novos)

```
rules::stdlib::tests::p475_block_inset_dict_per_side             ok
rules::stdlib::tests::p475_block_outset_dict_x_axis              ok
rules::stdlib::tests::p475_box_inset_dict_per_side               ok
rules::stdlib::tests::p475_block_inset_uniforme_ainda_funciona   ok
rules::stdlib::tests::p475_relative_aceite_em_extract_length_parte_abs  ok
rules::stdlib::tests::p475_block_inset_dict_chave_invalida_erro  ok
```

### Testes de regressão (negativos — 3 pré-existentes)

```
rules::stdlib::tests::native_block_rejeita_inset_negativo        ok
rules::stdlib::tests::native_box_rejeita_inset_negativo          ok
rules::stdlib::tests::p231_native_block_outset_negativo_rejeita  ok
```

### Suite completa

```
test result: ok. 3390 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

(3384 → 3390: +6 testes P475)

### `crystalline-lint --fix-hashes .`

```
Fixed 1 file:
  ./01_core/src/rules/stdlib/layout.rs → 284672b8
Re-running analysis... ✅ 0 drift warnings remaining
```

---

## 7. Scope-out explícito

| Área | Scope-out |
|------|-----------|
| **`Value::Relative` parte `rel`** | A componente relativa (`rel: f64`) é truncada a zero em `extract_length`. `block(inset: 50%)` equivale a `block(inset: 0pt)`. Para suporte completo precisa de contexto de layout (largura do container) — indisponível em L1. |
| **`pad` com `Value::Relative`** | `native_pad` usa `extract_sides_lengths` distinto; não migrado neste passo. |
| **Negativo em `Relative`** | `Rel { rel: -0.5, abs: 2pt }` → `Some(2pt)` (abs positivo); o `rel` negativo é descartado mas não rejeitado (a parte validada é só a `abs`). |
| **`inset`/`outset` em `table`/`grid` cells** | Não abrangido. |

---

## 8. Critério de fecho

- [x] `extract_length` aceita `Value::Relative` (parte `abs`; `rel` truncado; scope-out documentado).
- [x] `extract_sides_from_value` criado: uniforme + dict per-side com precedência específico > eixo > rest.
- [x] Negativos rejeitados em todos os caminhos de `extract_sides_from_value`.
- [x] `native_block`: `inset` e `outset` migrados para `extract_sides_from_value`.
- [x] `native_box`: `inset` e `outset` migrados para `extract_sides_from_value`.
- [x] 6 testes P475 verdes.
- [x] 3 testes de regressão de negativos continuam verdes.
- [x] `layout.md` L0 atualizado (inset/outset block/box com dict per-side).
- [x] `crystalline-lint --fix-hashes`: `layout.rs` → `284672b8`; 0 drift warnings.
- [x] Suite completa: 3390 passed; 0 failed.
- [x] **Trilha 8: 8/8 completo (fechado).**

---

## 9. Próximo passo recomendado

**Trilha 8 fechada.** Trilhas abertas:

| Trilha | Estado | Próximo item |
|--------|--------|--------------|
| **Trilha 6** | 3/4 | `LoF/LoT com page numbers` — requer 2-pass convergente (DEBT longo prazo) |
| **Trilha 4** | 0/N | `Value::Gradient` tipo real (M, ~35 min) |
| **Trilha 7** | 0/N | `columns`/`colbreak` (L, ~60+ min) |

Sugestão para P476:
- **Trilha 4** — `Value::Gradient` tipo real (menor dependência; M).
- **Trilha 7** — `columns`/`colbreak` (maior impacto layout; L).
