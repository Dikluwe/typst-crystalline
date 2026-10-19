# Paridade de Produção — P598

**Data do relatório:** 2026-07-07
**Passo:** 598
**Foco:** Confirmar a fórmula exacta de margem automática do vanilla e implementar a correção correspondente no cristalino.

---

## Resumo executivo

A fórmula de margem automática do vanilla 0.15.0 foi confirmada por leitura directa do código fonte e por medições em múltiplos tamanhos de página:

```text
margin = min(width, height) * (2.5 / 21)
```

Equivalente a `≈ 11.90476 %` da menor dimensão. A coincidência numérica observada em P597 (`70.87 pt ≈ 595.28 pt × 2.5/21`) confirma-se como a fórmula real.

A correção foi implementada no cristalino:

- `PageConfig::default()` passa a calcular a margem automaticamente.
- Adicionado campo `margin_is_auto` a `PageConfig` para distinguir margem automática de margem fixa.
- `SetPage` recalcula a margem quando `width`/`height` mudam e a margem está em modo automático; preserva uma margem fixa definida pelo utilizador.

O caso de P597 (`#set page(columns: 2, height: 200pt)` + `#lorem(n)`) passa a bater com o vanilla para `n ≤ 150`. Para `n = 500` ainda há divergência residual (4 vs 3 páginas), provavelmente por diferenças no algoritmo de colunas/gutter; fica fora do escopo deste passo.

---

## Proveniência das medições

- **Hash base das sondas iniciais:** `9150389fecb44e43affe1120edb816a692bc3e1b`
- **Hash após implementação (working tree):** alterações não commitadas nos ficheiros:
  - `00_nucleo/prompts/entities/layout_types.md`
  - `01_core/src/entities/layout_types.rs`
  - `01_core/src/engine/layout/set_page.rs`
  - `01_core/src/engine/layout/tests.rs`
- **Data/hora das medições:** 2026-07-07T20:24:15-03:00
- **Binários usados:**
  - Cristalino: `./target/release/typst` (crate `typst-wiring`, release)
  - Vanilla 0.15.0: `lab/typst-original/target/release/typst compile`
- **Ferramentas auxiliares:** `mutool draw -F stext`, `pdftotext -layout`, `pdfinfo`

---

## Sonda — confirmação da fórmula

### Código fonte do vanilla

`lab/typst-original/crates/typst-library/src/layout/page.rs:127-131`:

```rust
/// The page's margins.
///
/// - `{auto}`: The margins are set automatically to 2.5/21 times the
///   smaller dimension of the page. This results in 2.5 cm margins for an
///   A4 page.
```

### Medições no vanilla

Documento de teste:

```typst
#set page(height: ${altura}pt)
teste
```

| Altura (pt) | Margem esquerda medida (pt) | Proporção `margem ÷ min(width, height)` |
|------------:|----------------------------:|----------------------------------------:|
| 100         | 11.904762                   | 0.1190476                               |
| 150         | 17.857144                   | 0.1190476                               |
| 200         | 23.809525                   | 0.1190476                               |
| 300         | 35.714288                   | 0.1190476                               |
| 500         | 59.52381                    | 0.1190476                               |
| 841.89      | 70.86614                    | 0.1190476 (limitado pela largura)       |
| 1200        | 70.86614                    | 0.1190476 (limitado pela largura)       |

Documento de teste:

```typst
#set page(width: ${largura}pt, height: 841.89pt)
teste
```

| Largura (pt) | Margem esquerda medida (pt) | Proporção `margem ÷ min(width, height)` |
|-------------:|----------------------------:|----------------------------------------:|
| 200          | 23.809525                   | 0.1190476                               |
| 300          | 35.714288                   | 0.1190476                               |
| 400          | 47.61905                    | 0.1190476                               |
| 595.28       | 70.86667                    | 0.1190476                               |
| 800          | 95.2381                     | 0.1190476                               |

Conclusão: a fórmula usa sempre a **menor dimensão** da página, não a altura nem a largura especificamente.

---

## Implementação

### `01_core/src/entities/layout_types.rs`

- `PageConfig` ganha campo `margin_is_auto: bool`.
- `PageConfig::default()` calcula `margin = width.min(height) * 2.5 / 21.0`.
- Adicionado método auxiliar `PageConfig::auto_margin()`.
- Header `@prompt-hash` actualizado pelo linter (`269ba6e5`).

### `01_core/src/engine/layout/set_page.rs`

- Quando `SetPage` recebe `margin` explícito: `margin_is_auto = false`.
- Quando `SetPage` altera `width`/`height` e `margin_is_auto == true`: recalcula a margem.
- Margem ausente não altera `margin_is_auto` (preserva estado anterior).

### `00_nucleo/prompts/entities/layout_types.md`

- Actualizado com a secção `PageConfig` documentando a fórmula e a semântica `margin_is_auto`.

---

## Testes

### Novos testes

- `p598_page_config_default_margin_a4_bate_vanilla`
- `p598_page_config_margin_formula_uses_smaller_dimension`
- `p598_page_config_auto_margin_follows_dimension_changes`
- `p598_colunas_pagina_pequena_texto_curto_uma_pagina`
- `p598_colunas_pagina_pequena_texto_medio_uma_pagina`
- `p598_margem_explicita_nao_recalculada_ao_mudar_altura`

### Resultados

```text
cargo test -p typst-core p598
=> 6 passed; 0 failed

cargo test -p typst-core
=> 3581 passed; 0 failed
```

### Verificação end-to-end

Documento:

```typst
#set page(columns: 2, height: 200pt)
#lorem($n)
```

| `$n` | Páginas cristalino | Páginas vanilla |
|------|-------------------:|----------------:|
| 30   | 1                  | 1               |
| 50   | 1                  | 1               |
| 80   | 1                  | 1               |
| 100  | 1                  | 1               |
| 120  | 1                  | 1               |
| 150  | 1                  | 1               |
| 200  | 2                  | 2               |
| 300  | 2                  | 2               |
| 500  | 4                  | 3               |

O caso de P597 fica resolvido até `n = 150`. A divergência em `n = 500` indica que, para textos muito longos, outros factores (gutter, quebra de linha, algoritmo de enchimento de colunas) ainda diferem do vanilla. Fica registado como scope-out de P598.

---

## Decisão

A fórmula é simples e consistente, mas a implementação completa exigiu uma pequena extensão semântica (`margin_is_auto`) para preservar margens explicitamente definidas pelo utilizador. A correção foi implementada neste passo, com testes de regressão para A4 e para o caso de P597.

**Scope-outs conscientes:**

1. Divergência residual para documentos muito longos (`n = 500` no teste acima).
2. `margin: auto` explícito após uma margem fixa não é distingível de `margin` ausente nesta fase (o eval representa ambos por `None`). Uma futura revisão do eval pode introduzir essa distinção.

---

## Critérios de fecho do passo

- [x] Proporção confirmada com 12 tamanhos de página diferentes.
- [x] Confirmado que a fórmula usa a menor dimensão da página.
- [x] Fórmula exacta confirmada por leitura directa do código fonte do vanilla.
- [x] Correção implementada e testada.
- [x] `cargo test -p typst-core` limpo.
- [x] Relatório escrito com hash do commit e proveniência.

---

## Ligações

- `00_nucleo/materialization/typst-passo-598.md` — passo que originou esta sonda.
- `00_nucleo/diagnosticos/paridade-producao-p597.md` — disparidade documentada em P597.
- `01_core/src/entities/layout_types.rs` — implementação de `PageConfig`.
- `01_core/src/engine/layout/set_page.rs` — recálculo de margem em `SetPage`.
- `lab/typst-original/crates/typst-library/src/layout/page.rs:127-131` — fórmula do vanilla.
- ADR-0108 — *Disciplina anti-deriva: medir antes de decidir*.
