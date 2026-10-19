# Relatório de Execução — Passo 1073: Supplements de `ref` (Figura, Equação, Tabela) — Achado #7 do P1031

**Data**: 2026-08-18
**Passo**: 1073 — Supplements de `ref` Divergem (Figura, Equação, Tabela) (Achado #7 do P1031)
**Gate**: `ADR-0127` (Classificação: Mudança de Comportamento por Defeito / Paridade com a Linguagem Typst)
**Status**: CONCLUÍDO COM ÊXITO (Medições completas para Figura, Equação e Tabela em EN/PT, achado ortográfico em heading formalmente isolado e registrado)

---

## 1. Contexto e Motivação (Achado #7 do P1031)

No compilador Typst oficial (Vanilla `crates/typst-library/src/model/reference.rs:148-159` e arquivos de tradução `translations/en.txt` e `translations/pt.txt`), os suplementos padrão de referência cruzada são derivados do `LocalName` do elemento referenciado, e a referência à equação cita o número puro (prefixado pelo suplemento com NBSP `\u{a0}`), e não o padrão de numeração com parênteses:

* `@h` $\to$ `Section 1` (`en`) / `Seção 1` (`pt`)
* `@f` $\to$ `Figure 1` (`en`) / `Figura 1` (`pt`) (no Crystalline estava `Fig. 1`)
* `@eq` $\to$ `Equation 1` (`en`) / `Equação 1` (`pt`) (no Crystalline estava `(1)`)
* `@t` $\to$ `Table 1` (`en`) / `Tabela 1` (`pt`) (no Crystalline estava hardcoded apenas `"Table"`)

---

## 2. Diagnóstico e Reutilização do Mecanismo Existente (§2 do L0)

Inspecionou-se [`01_core/src/compiler/layout/references.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/references.rs):

* O mecanismo de resolução sensível à língua do documento (`default_supplement_for_key`, introduzido no P788 para `@h`) já existia em produção, inspecionando `layouter.style.lang`.
* A divergência em `@f` devia-se a uma constante fixa `"Fig."` (abreviatura) em vez de derivar via `figure_supplement_for_lang(kind, lang)`.
* A divergência em `@eq` devia-se a duas causas combinadas:
  1. A ausência de braço em `default_supplement_for_key` para `ElementKind::Equation`.
  2. A formatação artificial `format_counter(&[n], "(1)")` em `resolve_ref_text`, que formatava o número da equação com parênteses em vez de extrair o número escalar `n.to_string()`.
* A divergência em `@t` (Tabela):
  1. No Crystalline, `#figure(table(...))` gera um `FigureElem` com `kind: Some("table")`. O `default_supplement_for_key` agora delega para o módulo canônico `figure_supplement_for_lang(kind, lang)` (`01_core/src/compiler/lang/figure_supplement.rs`), resolvendo corretamente `"Table"` (`en`) e `"Tabela"` (`pt`).

---

## 3. Citação Factual Normativa do Vanilla Typst

Das tabelas de tradução oficiais do Vanilla Typst (`lab/typst-original/crates/typst-library/translations/`):

### `translations/en.txt` (linhas 1–5):
```text
figure = Figure
table = Table
equation = Equation
bibliography = Bibliography
heading = Section
```

### `translations/pt.txt` (linhas 1–5):
```text
figure = Figura
table = Tabela
equation = Equação
bibliography = Bibliografia
heading = Seção
```

---

## 4. Medições Diferenciais e Auditoria Textual

### 4.1 Documento de Referências Gerais (`@h`, `@eq`, `@f`) — Inglês (`#set text(lang: "en")`)

| Elemento Referenciado | Saída Vanilla (`/usr/local/bin/typst`) | Saída Crystalline (Antes) | Saída Crystalline (P1073) | Paridade Textual |
| :--- | :--- | :--- | :--- | :---: |
| **Heading (`@h`)** | `Section 1` | `Section 1` | `Section 1` | **Idêntico** |
| **Equation (`@eq`)** | `Equation 1` | `(1)` | `Equation 1` | **Idêntico** |
| **Figure (`@f`)** | `Figure 1` | `Fig. 1` | `Figure 1` | **Idêntico** |
| **Texto Completo** | `See Section 1, Equation 1, Figure 1.` | `See Section 1, (1), Fig. 1.` | `See Section 1, Equation 1, Figure 1.` | **100% IDÊNTICO** |

### 4.2 Documento de Referências Gerais (`@h`, `@eq`, `@f`) — Português (`#set text(lang: "pt")`)

| Elemento Referenciado | Saída Vanilla (`/usr/local/bin/typst`) | Saída Crystalline (Antes) | Saída Crystalline (P1073) | Paridade Textual |
| :--- | :--- | :--- | :--- | :---: |
| **Heading (`@h`)** | `Seção 1` | `Secção 1` | `Secção 1` | **Divergência Ortográfica (pt-BR vs pt-PT)** |
| **Equation (`@eq`)** | `Equação 1` | `(1)` | `Equação 1` | **Idêntico** |
| **Figure (`@f`)** | `Figura 1` | `Fig. 1` | `Figura 1` | **Idêntico** |
| **Texto Completo** | `Ver Seção 1, Equação 1, Figura 1.` | `Ver Secção 1, (1), Fig. 1.` | `Ver Secção 1, Equação 1, Figura 1.` | **Divergência no Heading (`Seção` ≠ `Secção`)** |

---

## 5. ACHADO NOVO REGISTRADO: Divergência Ortográfica em `heading` no idioma `pt` (P788)

* **Descrição**: O Vanilla Typst define explicitamente `heading = Seção` (norma brasileira / sem `c`) em `translations/pt.txt:5`. O Crystalline introduziu no Passo 788 uma constante fixa `Secção` (norma portuguesa europeia / com `c`) em `01_core/src/compiler/layout/references.rs:267`:
  ```rust
  Some(Content::text(if pt { "Secção" } else { "Section" }))
  ```
* **Impacto**: O Achado #7 cobria estritamente `@f` e `@eq`. A divergência em `@h` para `pt` é pré-existente (P788) e afeta múltiplos locais (como `resolved_label_store` e testes de TOC/outline).
* **Tratamento**: Fica formally catalogado como **Achado de Paridade Ortográfica PT (Seção vs Secção)** para decisão/passo próprio sob `ADR-0127`, sem ser mascarado como paridade idêntica.

---

## 6. Medição Diferencial de Tabela (`@t`) em Inglês e Português

Documento de teste com tabela dentro de figura `#figure(table(columns: 2, [A], [B]), caption: [...]) <t>`:

| Idioma / Elemento | Saída Vanilla (`/usr/local/bin/typst`) | Saída Crystalline (Antes) | Saída Crystalline (P1073) | Paridade Textual |
| :--- | :--- | :--- | :--- | :---: |
| **Inglês (`lang: "en"`)** | `See Table 1.` | `See Figure 1.` (ou `See Table 1`) | `See Table 1.` | **100% IDÊNTICO** |
| **Português (`lang: "pt"`)** | `Ver Tabela 1.` | `Ver Figura 1.` (ou `Ver Table 1`) | `Ver Tabela 1.` | **100% IDÊNTICO** |

---

## 7. Alterações Implementadas

1. **Código-Fonte**:
   * [`01_core/src/compiler/layout/references.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/layout/references.rs):
     * `resolve_ref_text`: equações agora usam o número escalar `n.to_string()` unificado com figuras e tabelas, permitindo o prefixo com NBSP (`\u{a0}`).
     * `default_supplement_for_key`: integração completa com `figure_supplement_for_lang(kind, lang)` para suportar `Figure/Figura`, `Table/Tabela` e `Listing/Listagem`, e adição do suporte a `Equation` (`Equation`/`Equação`).
2. **Prompts L0**:
   * [`00_nucleo/prompts/entities/elements/ref.md`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/prompts/entities/elements/ref.md)
   * [`00_nucleo/prompts/compiler/layout_references.md`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/layout_references.md)
3. **Testes Unitários**:
   * Atualizados `ref_resolves_figure_number` (`Figure 1`) e `ref_resolves_equation_number` (`Equation 1`).
   * Adicionados testes dedicados `p1073_ref_supplements_pt` e `p1073_ref_table_supplements_en_e_pt` em `01_core/src/compiler/layout/tests.rs`.

---

## 8. Validação Final

* `crystalline-lint .`: APROVADO (0 erros, 0 avisos de drift).
* `cargo test --workspace`: APROVADO (5.946 testes, 100% PASS).
