# Relatório de Execução — Passo 1072: Default de `body_indent` (`0pt` → `0.5em`) em `list` e `enum`

**Data**: 2026-08-18
**Passo**: 1072 — Default de `body_indent` (`0pt` → `0.5em`) em `list`/`enum` (Achado #8 do P1031)
**Gate**: `ADR-0127` (Classificação: Mudança de Comportamento por Defeito / Paridade com a Linguagem Typst)
**Status**: CONCLUÍDO COM ÊXITO (Paridade sub-pixel comprovada $\Delta\text{gap} = 0.0000\text{ pt}$, $\Delta x = +0.0006\text{ pt}$ offset de baseline, 100% PASS na suíte de testes, L0s selados)

---

## 1. Contexto e Motivação (Achado #8 do P1031)

No compilador Typst oficial (Vanilla `crates/typst-library/src/model/list.rs:100-102` e `model/enum.rs:152-154`), o espaçamento padrão entre o marcador/numeração e o corpo do item (`body_indent`) é definido com `#[default(Em::new(0.5).into())]` ($0.5\text{em}$).

No Crystalline, o valor de fallback na ausência de especificação explícita estava definido como `Length::pt(0.0)`, fazendo com que listas e enumerações sem `body-indent` explícito colapsassem o texto diretamente contra o marcador (`•Um` / `1.Um`).

---

## 2. Auditoria e Comprovação Factual de `TermItem` (§2)

Para comprovar formalmente a isenção de `TermItem`, foi inspecionada a definição normativa de `TermItem` no Typst Vanilla (`lab/typst-original/crates/typst-library/src/model/terms.rs:114-123`):

```rust
/// A term list item.
#[elem(name = "item", title = "Term List Item", since = "0.4.0", Tagged)]
pub struct TermItem {
    /// The term described by the list item.
    #[required]
    pub term: Content,

    /// The description of the term.
    #[required]
    pub description: Content,
}
```

* **Conformidade de Campos**: A struct `TermItem` no Vanilla contém **exclusivamente** os campos `term: Content` e `description: Content`.
* Ao contrário de `ListItem` (`list.rs:100`) e `EnumItem` (`enum.rs:152`), `TermItem` não modela `body_indent` no item — o container `TermsElem` gerencia a separação via `separator: Content` (default `h(0.6em, weak: true)`).
* **Conclusão Factual**: A ausência de `body_indent` em `TermItem` é canônica da especificação do Typst. O ajuste aplica-se estritamente a `list` e `enum`.

---

## 3. Medição Diferencial e Paridade Sub-Pixel (§3 e §5)

### 3.1 Lista Não-Ordenada (`- Um \n - Dois`) — `font-size: 11pt` ($0.5\text{em} = 5.5000\text{ pt}$)

| Métrica / Elemento | Posição Vanilla (`/usr/local/bin/typst`) | Posição Crystalline (Antes) | Posição Crystalline (P1072) | Diferença ($\Delta$) |
| :--- | :--- | :--- | :--- | :---: |
| **Marcador (`•`) xMin** | `70.8661 pt` | `70.8667 pt` | `70.8667 pt` | **+0.0006 pt** (ruído de baseline) |
| **Marcador (`•`) xMax** | `74.7271 pt` | `74.7277 pt` | `74.7277 pt` | **+0.0006 pt** (ruído de baseline) |
| **Corpo (`Um`) xMin** | `80.2271 pt` | `74.7277 pt` (colapsado) | `80.2277 pt` | **+0.0006 pt** (ruído de baseline) |
| **Corpo (`Um`) xMax** | `96.1881 pt` | `90.6887 pt` | `96.1887 pt` | **+0.0006 pt** (ruído de baseline) |
| **Gap Marcador $\to$ Corpo** | $\mathbf{5.5000\text{ pt}}$ ($0.5\text{em}$) | $0.0000\text{ pt}$ ($0\text{pt}$) | $\mathbf{5.5000\text{ pt}}$ ($0.5\text{em}$) | $\mathbf{\Delta\text{gap} = 0.0000\text{ pt}}$ (exato) |
| **Saída `pdftotext -layout`** | `• Um` / `• Dois` | `•Um` / `•Dois` | `• Um` / `• Dois` | **Idêntico** |

### 3.2 Lista Ordenada (`+ Um \n + Dois`) — `font-size: 11pt` ($0.5\text{em} = 5.5000\text{ pt}$)

| Métrica / Elemento | Posição Vanilla (`/usr/local/bin/typst`) | Posição Crystalline (Antes) | Posição Crystalline (P1072) | Diferença ($\Delta$) |
| :--- | :--- | :--- | :--- | :---: |
| **Rótulo (`1.`) xMin** | `70.8661 pt` | `70.8667 pt` | `70.8667 pt` | **+0.0006 pt** (ruído de baseline) |
| **Rótulo (`1.`) xMax** | `78.4011 pt` | `78.4017 pt` | `78.4017 pt` | **+0.0006 pt** (ruído de baseline) |
| **Corpo (`Um`) xMin** | `83.9011 pt` | `78.4017 pt` (colapsado) | `83.9017 pt` | **+0.0006 pt** (ruído de baseline) |
| **Corpo (`Um`) xMax** | `99.8621 pt` | `94.3627 pt` | `99.8627 pt` | **+0.0006 pt** (ruído de baseline) |
| **Gap Rótulo $\to$ Corpo** | $\mathbf{5.5000\text{ pt}}$ ($0.5\text{em}$) | $0.0000\text{ pt}$ ($0\text{pt}$) | $\mathbf{5.5000\text{ pt}}$ ($0.5\text{em}$) | $\mathbf{\Delta\text{gap} = 0.0000\text{ pt}}$ (exato) |
| **Saída `pdftotext -layout`** | `1. Um` / `2. Dois` | `1.Um` / `2.Dois` | `1. Um` / `2. Dois` | **Idêntico** |

> **Nota sobre o Ruído de Baseline**: O deslocamento absoluto de $+0.0006\text{ pt}$ nos valores de margem ($70.8661\text{ pt}$ vs $70.8667\text{ pt}$) decorre da conversão de ponto flutuante $2.5\text{cm} \to \text{pt}$ no setup inicial da página. Como o deslocamento afeta igualmente o marcador e o corpo, o cálculo do **gap relativo** cancela o ruído e atinge paridade matemática estrita ($\Delta\text{gap} = 0.0000\text{ pt}$).

### 3.3 Preservação de Precedência de `body-indent` Explícito
Testado `#list(body-indent: 12pt, [Alpha], [Beta])`:
* Marcador `•`: `xMax = 74.7277 pt`
* Corpo `Alpha`: `xMin = 86.7287 pt`
* Gap resultante: $86.7287 - 74.7277 = \mathbf{12.0010\text{ pt}} \approx 12\text{ pt}$ (o valor do usuário continua tendo precedência estrita).

---

## 4. Alterações Implementadas

1. **Código-Fonte**:
   * [`01_core/src/compiler/layout/list_item.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/list_item.rs): Atualizado fallback de `body_indent` para `Length::em(0.5)` com citação `ref: lab/typst-original/crates/typst-library/src/model/list.rs:100-102`.
   * [`01_core/src/compiler/layout/enum_item.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/enum_item.rs): Atualizado fallback de `body_indent` para `Length::em(0.5)` com citação `ref: lab/typst-original/crates/typst-library/src/model/enum.rs:152-154`.
2. **Prompts L0 Atualizados e Selados**:
   * [`00_nucleo/prompts/compiler/layout/list_item.md`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout/list_item.md) (Hash: `ce14a92b`)
   * [`00_nucleo/prompts/compiler/layout/enum_item.md`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout/enum_item.md) (Hash: `27e8129c`)
3. **Testes Unitários Dedicados**:
   * `p1072_list_item_default_body_indent_05em` e `p1072_enum_item_default_body_indent_05em` adicionados a `01_core/src/compiler/layout/tests.rs`.

---

## 5. Validação Final

* `crystalline-lint .`: APROVADO (0 erros, 0 avisos de drift).
* `cargo test --workspace`: APROVADO (5.944 testes, 100% PASS).
