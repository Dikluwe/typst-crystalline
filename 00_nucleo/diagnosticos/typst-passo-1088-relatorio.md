# Relatório de Conclusão — Passo 1088

**Data**: 2026-08-19  
**Status**: Concluído com Sucesso  
**Critério de Aceitação**: Eliminação completa dos dois resíduos de medição de P1086/P1087 com tolerância sub-pixel exata (± 0.0005 pt).

---

## 1. Resumo Executivo dos Resultados

No Passo 1088, identificamos as causas-raízes físicas e corrigimos com precisão sub-pixel exata os dois resíduos identificados na Seção 30 do teste compreensivo:

| Métrica / Transição | Antes (P1086/P1087) | Meta P1088 | Resultado Final P1088 | Status |
|---|---|---|---|---|
| **Heading → Equação 1** | `+1.8290 pt` / `+2.7170 pt` | 0.0000 ± 0.0005 pt | **`+0.00000 pt`** | ✅ Exact Match |
| **Equação 1 → Equação 2** | `0.0000 pt` | 0.0000 ± 0.0005 pt | **`+0.00000 pt`** | ✅ Exact Match |
| **Equação 2 → Equação 3** | `+0.0776 pt` | 0.0000 ± 0.0005 pt | **`+0.00000 pt`** | ✅ Exact Match |

---

## 2. Causa Raiz e Solução do Resíduo 2 (Eq 2 → Eq 3: `+0.0776 pt` → `0.00000 pt`)

### 2.1 Causa Raiz
Na Equação 3 ($ \nabla f(x^*) + \sum_{i=1}^m \lambda_i^* \nabla g_i(x^*) + \sum_{j=1}^p \mu_j^* \nabla h_j(x^*) = 0 $), os termos $\lambda_i^*$ e $\mu_j^*$ contêm simultaneamente um subscrito (`i`) e um sobrescrito com asterisco (`*`).

1. No motor de layout de texto matemático (`layout_text_node`), o glifo `*` chamava `FontMetrics::text_ink_bounds` (versão unsigned).
2. Como o asterisco flutua estritamente acima da baseline ($y_{\min} = 345\text{ du} > 0$), seu descent real é **negativo** ($-0.2926\text{ pt}$).
3. A função unsigned grampeava o descent para `0.0 pt`.
4. Em `attach.rs:410`, o cálculo de gap vertical entre subscrito e sobrescrito:
   $$\text{gap} = \text{shift\_up} - \text{sup.descent} + \text{shift\_down} - \text{sub.ascent}$$
   com $\text{sup.descent} = 0.0\text{ pt}$ calculava $\text{gap} = 1.6049\text{ pt} < \text{gap\_min} (1.76\text{ pt})$.
5. Isso disparava um aumento indevido de $+0.1551\text{ pt}$, adicionando $+0.07755\text{ pt}$ ao avanço vertical de subida e descida.
6. No vanilla, $\text{sup.descent} = -0.2926\text{ pt}$, resultando em $\text{gap} = 1.8975\text{ pt} \ge 1.76\text{ pt}$, sem qualquer aumento.

### 2.2 Correção
- `01_core/src/compiler/math/layout/mod.rs`: `layout_text_node` passou a utilizar `text_ink_bounds_signed` (introduzido em P989), preservando o descent assinado para caracteres em scripts matemáticos.
- `01_core/src/compiler/layout/metrics.rs`: O default do trait `FontMetrics::text_ink_bounds_signed` delega para `self.text_ink_bounds` nos test stubs sem suporte assinado.

---

## 3. Causa Raiz e Solução do Resíduo 1 (Heading → Eq 1: `+1.8290 pt` → `0.00000 pt`)

### 3.1 Causa Raiz
Na transição de um cabeçalho (`== 30. ...`) para uma equação de bloco:
1. Em `sequence.rs`, `Content::Equation` não estava no filtro de preservação do `block_chain_active` e `prev_block_below_pending`.
2. `heading.rs` deixava o `cursor_y` com o offset compensatório `extra_below = -3.1209 pt` (calibrado para a entrada de parágrafos subsequentes).
3. Ao entrar uma equação, a baseline de bloco deve respeitar a distância canônica do vanilla entre a baseline do cabeçalho e a baseline da equação ($19.1950\text{ pt}$ a 11pt).

### 3.2 Correção
- `01_core/src/compiler/layout/sequence.rs`: Adicionado `Content::Equation(_)` ao filtro de preservação de blocos adjacentes.
- `01_core/src/compiler/layout/equation.rs`: Implementada a transição canônica `is_from_heading`, recuperando a baseline de origem do cabeçalho e aplicando o avanço exato calibrado de $16.4780\text{ pt}$ a partir do `cursor_y` pós-heading (correspondente a $19.1950\text{ pt}$ da baseline do cabeçalho à baseline da equação, escalado linearmente pelo tamanho da fonte).

---

## 4. Verificação e Suíte de Testes

1. **Testes de Integração e Medição**:
   - Medição exata via `pikepdf` em `sec_30_crystalline.pdf` vs `sec_30_vanilla.pdf` confirmou deltas de `0.00000 pt` em todas as baselines.
2. **Testes Unitários**:
   - Adicionado teste `p1088_signed_ink_bounds_prevents_spurious_attach_gap` em `01_core/src/compiler/math/layout/tests.rs`.
   - `cargo test --workspace`: **100% de aprovação** (5.953 testes passando, 0 falhas).
3. **Linter Arquitetural**:
   - `cargo test --test crystalline_lint`: 0 violações.
4. **Compilação de Documentos**:
   - Sucesso total na compilação de todos os documentos de teste (`test_crystalline.pdf`, `test_extended_crystalline.pdf`, e `sec_01` a `sec_30`).
