# Relatório de Execução — Passo 1057

**Data**: 2026-08-15
**Passo**: 1057 — Paridade, Resolução Dinâmica de `par.spacing` e Suporte a `#set par(spacing: ...)`
**Gate**: `ADR-0127` (Classificação: Resolução Dinâmica de Estilo / Paridade Arquitetural)
**Status**: CONCLUÍDO COM ÊXITO (Suporte dinâmico a `set par(spacing)` implementado, 100% dos testes do workspace aprovados, crystalline-lint com 0 erros e zero drift)

---

## 1. Contexto e Objetivos do Passo 1057

No Passo 1056, identificou-se e corrigiu-se a divergência de $-6.050\text{ pt}$ no gap entre parágrafos simples (`Before\n\nAfter`), decorrente da transição de `leading` ($0.65\text{em}$) para `par.spacing` ($1.20\text{em}$).

O Passo 1057 fechou o ciclo do mecanismo arquitetural completo:
1. **Suporte Dinâmico ao `#set par(spacing: ...)`**: No `eval/rules.rs`, chamadas `#set par(spacing: ...)` eram tratadas pelo fallback de propriedade não suportada. O canal `custom("par.spacing")` agora é preenchido com `Value::Length`.
2. **Resolução Dinâmica no Layouter**: No `layout/mod.rs`, o tratador de `Content::Parbreak` agora consome dinamicamente a cadeia de estilos (`StyleChain`), extraindo `par.spacing` (default $1.20\text{em}$) e `par.leading` (default $0.65\text{em}$) para computar o avanço vertical exato:
   $$\text{extra\_spacing} = \max(0, \text{spacing\_pt} - \text{leading\_pt})$$
3. **Absorção de Resíduos**: Garantir que customizações de espaçamento entre parágrafos funcionem ponta a ponta sem gerar warnings de propriedades desconhecidas e reflitam instantaneamente na geometria das páginas.

---

## 2. Implementações Realizadas

### 2.1 Avaliador de Regras (`01_core/src/compiler/eval/rules.rs`)
* Adicionado o ramo `"spacing"` no tratamento de `#set par(...)`:
  ```rust
  "spacing" => {
      if let Value::Length(l) = val {
          *engine.styles = engine
              .styles
              .push_custom("par.spacing", Value::Length(l));
      }
  }
  ```

### 2.2 Motor de Layout (`01_core/src/compiler/layout/mod.rs`)
* O manipulador de `Content::Parbreak` passou a resolver dinamicamente os valores de `par.spacing` e `par.leading` com fallbacks canônicos do vanilla:
  ```rust
  Content::Parbreak => {
      let had_items = !self.regions.current.current_line.is_empty();
      self.flush_line();
      if had_items {
          let font_size = self.style.size.val();
          let spacing_pt = match self.chain.custom("par.spacing") {
              Some(Value::Length(l)) => l.resolve_pt(font_size),
              // ref: lab/typst-original/crates/typst-library/src/model/par.rs:224
              _ => font_size * 1.2,
          };
          let leading_pt = match self.chain.custom("par.leading") {
              Some(Value::Length(l)) => l.resolve_pt(font_size),
              // ref: lab/typst-original/crates/typst-library/src/model/par.rs:210
              _ => self.style.leading.map(|l| l.resolve_pt(font_size)).unwrap_or(font_size * 0.65),
          };
          let extra_spacing = (spacing_pt - leading_pt).max(0.0);
          self.regions.current.cursor_y += Pt(extra_spacing);
      }
  }
  ```

---

## 3. Validação e Testes

### 3.1 Teste Unitário Dedicado (`01_core/src/compiler/layout/tests.rs`)
* Adicionado o teste `p1057_par_spacing_custom_e_default`:
  * **Caso Default**: Dois parágrafos simples `Before\n\nAfter` utilizam $1.20\text{em}$.
  * **Caso Custom**: `#set par(spacing: 2em)` aplicado sobre `Before\n\nAfter`.
  * **Validação**: $\Delta y_{\text{custom}} - \Delta y_{\text{default}} = (2.0 - 1.2) \times 11\text{ pt} = 0.8 \times 11\text{ pt} = 8.80\text{ pt}$ (Validado com tolerância $< 10^{-3}$).

### 3.2 Suíte de Testes do Workspace
* `cargo test --workspace` aprovado com **100% de sucesso**:
  * `typst-core`: 5.050+ testes aprovados
  * `typst-infra`: 796 testes aprovados
  * `typst-shell`: 41 testes aprovados
  * `cli / integration`: 37 testes aprovados

### 3.3 Verificação de Arquitetura e Linter
* `crystalline-lint .`: **0 erros**, **0 deriva de prompt L0 (V5)**.

---

## 4. Commits Relacionados

1. `f880e5af7` — `chore: sincronizar hashes L0, correções clippy e introduzir Passo 1057`
2. `ebeb297a5` — `feat(layout): Passo 1057 - paridade e resolução dinâmica de par.spacing e eliminação de resíduo`
