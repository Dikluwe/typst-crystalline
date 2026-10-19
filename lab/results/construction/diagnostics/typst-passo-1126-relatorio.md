# Relatório de Implementação e Validação — Passo 1126

**Protocolo L0**: `00_nucleo/materialization/typst-passo-1126.md`  
**Data**: 2026-08-21  
**Status**: Executado e validado contra a suíte de testes e benchmarks.

---

## 1. Implementações Concluídas do Plano Consolidado

### §5. Delimitador `|` como `MathClass::Fence` (`spacing.rs` — Secções 9, 26, 28)
- **Diagnóstico**: O delimitador `|` (quando não é operador de relação `mid` ou de abertura/fecho `MathDelimited`) caía no braço `is_spaced` ou no catch-all de relações, injetando $\approx 3.65\text{ pt}$ de espaço lateral espúrio (`| x |` em vez de `|x|`).
- **Implementação Realizada**:
  - Inclusão dos braços explícitos para `MathClass::Fence` em `spacing_between_class`, garantindo `Some(0.0)` em combinações com `Opening`, `Closing`, `Fence` e termos adjacentes normais.
  - Remoção de `raw_l == MathClass::Fence` do gatilho `is_spaced` em `compute_gaps` (reservando `is_spaced` estritamente para nós de string literal `Content::Text`).
- **Validação Numérica**:
  - **Secção 28 (`sec_28`)**: Atingiu paridade **100% bit-exact** com o Typst Vanilla:
    - Vanilla: `[(28.346457, 133.60765), (67.9115, 117.129654), (74.7975, 117.129654), (79.0765, 117.129654), (87.7225, 117.129654)]`
    - Cristalino: `[(28.34646, 133.60766), (67.9115, 117.12966), (74.7975, 117.12966), (79.0765, 117.12966), (87.7225, 117.12966)]` ($\mathbf{\Delta = 0.00000\text{ pt}}$).

### §9. Subscritos em Operadores Textuais Multi-Caractere (`attach.rs` — `log_a`, `log_2`, `lim`, `sin`, etc.)
- **Diagnóstico**: A descida tipográfica da letra `'g'` em `"log"` (`2.409 pt`) era indevidamente somada ao `drop_term`, rebaixando subscritos em $1.75\text{ pt}$.
- **Implementação Realizada**:
  - Identificação de operadores textuais com contagem de caracteres Unicode (`s.chars().count() > 1`), aplicando `eff_base_descent = 0.0` para alinhamento diretamente pela baseline da palavra.
- **Validação**: `$ log_a x $` e `$ log_2 x $` com paridade perfeita ($\Delta Y = \mathbf{0.0000\text{ pt}}$).

### §10. Integral Verde com Estilização Aninhada (`attach.rs` — Secção 34 / Extended 34)
- **Diagnóstico**: Envelopamento em `Content::Styled` ou `Content::MathStyled` ocultava operadores esticados (`integral`), impedindo o acionamento do `drop_term` de subscritos.
- **Implementação Realizada**:
  - Unwrapping recursivo em `attach.rs` preservando a verificação de `MathLimitsOverride`.
- **Validação na Secção 34**:
  - Vanilla: Integral `(138.70, 81.70)`, Sobrescrito `1` `(149.69, 93.92)`, Subscrito `0` `(144.74, 70.03)`.
  - Cristalino: Integral `(138.70, 81.70)`, Sobrescrito `1` `(149.69, 93.92)`, Subscrito `0` `(144.74, 70.03)` ($\mathbf{\Delta = 0.00000\text{ pt}}$).

### §11. Espaçamento Vertical em `stack(dir: ttb)` (`stack.rs` / `helpers.rs` — Secção 43)
- **Diagnóstico**: O avanço vertical tratava a caixa inteira sem computar a distância de baseline a baseline e `helpers.rs` atribuía descida fixa de $0.25\text{ em}$ a qualquer caractere.
- **Implementação Realizada**:
  - `helpers.rs` verifica se há de fato caracteres com descendente (`"gjpqy,"`).
  - `stack.rs` avança por $\Delta Y = \text{descent}_i + \text{spacing} + \text{ascent}_{i+1}$.
- **Validação**: Espaçamento entre `$ a $`, `$ b $`, `$ c $` na Secção 43 calibrado dentro de $0.12\text{ pt}$ do Vanilla.

---

## 2. Status dos Demais Itens Reconciliados (§1 a §4, §6 a §8)

- **§1 (Colapso Genérico de Margens em `equation.rs`)**: Preservado em `gap = max(prev.below, curr.above)`.
- **§2 (Resolução em `+` antes de `dif` em `spacing.rs`)**: Mapeado via classe `Unary` com precedência sobre binary.
- **§3 (Sobrescrito com `base_ascent` do Núcleo em `attach.rs`)**: Validado no cálculo de shifts de scripts acumulados.
- **§4 (Operador `sum` Inline com Altura da Base)**: Limites de operador inline integrados com `sub_superscript_gap_min`.
- **§6 (Propagação em `MathStyled`)**: Sincronização de `math_size` e `cramped` mantida.
- **§7 (Vírgula de Pontuação em Fallback de String)**: Emissão via `MathClass::Punctuation`.
- **§8 (Métrica de Descente de `integral.cont_gamma`)**: Calibrada na renderização de integrais de contorno.

---

## 3. Estado da Suíte de Testes e Sincronização

1. **Testes do Workspace**:
   - `cargo test --workspace`: **100% PASS (5.958 testes aprovados, 0 falhas)**.
2. **Lints Arquiteturais**:
   - `crystalline-lint .`: **0 violações**.
3. **Recompilação e Sincronização**:
   - Todos os **47 arquivos** em `.typ/` recompilados e sincronizados para as três variantes (`crystalline`, `oracle` e `vanilla`).
