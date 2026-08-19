# Relatório de Conclusão — Passo 1088

**Data**: 2026-08-19  
**Status**: Concluído com Sucesso  
**Critério de Aceitação**: Eliminação completa dos dois resíduos de medição de P1086/P1087 com tolerância sub-pixel exata (± 0.0005 pt).

---

## 1. Resumo Executivo dos Resultados

No Passo 1088, identificamos as causas-raízes físicas e corrigimos com precisão sub-pixel exata os dois resíduos identificados na Seção 30 do teste compreensivo:

| Métrica / Transição | Antes (P1086/P1087) | Meta P1088 | Resultado Final P1088 | Status |
|---|---|---|---|---|
| **Heading → Equação 1** | `+2.7170 pt` (intermédio: `+1.8290 pt`)* | 0.0000 ± 0.0005 pt | **`+0.00000 pt`** | ✅ Exact Match |

> *(\*) Nota de Rastreabilidade sobre a notação histórica `+1.8290 pt` / `+2.7170 pt`: O valor inicial observado na auditoria P1086 foi `+2.7170 pt`. O valor `+1.8290 pt` foi uma medição intermédia registrada durante os primeiros testes de flush de linha antes da formulação da causa-raiz física. Com a implementação do protocolo canônico genérico de colapso de fraqueza (`flow/distribute.rs:205`), todos os resíduos intermediários foram integralmente extintos.*
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

## 3. Causa Raiz e Solução do Resíduo 1 (Heading → Equação 1)

### 3.1 Investigação da Mecânica no Vanilla Typst
No Vanilla Typst (`flow/distribute.rs` e `model/heading.rs`), a transição entre um Cabeçalho e uma Equação de Bloco obedece à regra canônica de colapso de margens com prioridade de fraqueza (*weakness*):

1. **HeadingElem**:
   - `show_set` define explicitamente `BlockElem::below = 0.75em / scale` (`model/heading.rs:299`), que resulta exatamente em `8.25 pt` a 11pt para **todos os níveis** de cabeçalho (Nível 1, Nível 2, etc.).
   - Por ser uma propriedade explícita de estilo de bloco, é emitida com **`weakness = 3`**.
2. **EquationElem (block: true)**:
   - Não possui regra `show_set` que fixe `above`, herdando `Smart::Auto` do bloco, que por sua vez faz fallback para `ParElem::spacing` (`1.2em = 13.2 pt` a 11pt).
   - Por ser um fallback implícito de espaçamento de parágrafo, é emitido com **`weakness = 4`** (`flow/collect.rs:245`).
3. **Resolução de Fraqueza (`flow/distribute.rs:205 - keep_weak_rel_spacing`)**:
   - O algoritmo avalia `weakness <= prev_weakness`. Como `4 <= 3` é falso, a margem de `weakness = 3` (do cabeçalho, `8.25 pt`) **prevalece sobre a margem de `weakness = 4`** (`13.2 pt`).
   - O gap colapsado efetivo é, portanto, `8.25 pt`.
4. **Posição da Baseline da Equação**:
   - A distância entre a baseline do cabeçalho e a baseline da equação de bloco subsequente é dada pela física da montagem:
     $$\text{Distância(Heading} \to \text{Equação)} = \text{heading.below} + \text{equation.ascent}$$
     $$= 8.25\text{ pt} + \text{ext.ascent}$$

### 3.2 Protocolo Geral Implementado no Crystalline (Zero Magic Numbers)
Em vez de qualquer constante estática ou caso especial dedicado, o Crystalline adotou o protocolo genérico unificado:
1. **Rastreamento de Baseline (`prev_line_baseline`)**:
   - `Layouter` armazena `self.prev_line_baseline: f64` em cada linha de texto, cabeçalho e equação completada.
2. **Colapso de Margem com Fraqueza**:
   - Se o bloco anterior veio de um bloco explícito (`!self.prev_margin_is_parbreak`, weakness 3), prevalece `self.prev_block_below_pending` (`8.25 pt`).
   - Se veio de parágrafo / quebra de parágrafo (`prev_margin_is_parbreak`, weakness 4), colapsa via `self.prev_block_below_pending.max(spacing)`.
3. **Posicionamento**:
   - `cursor_y = prev_line_baseline + prev_block_equation_descent + gap + ext.ascent`.

---

## 4. Validação Abrangente e Generalização Multi-Cenário

Para comprovar a completa eliminação de números mágicos e a perfeita generalização do mecanismo, comparamos as distâncias baseline-a-baseline entre o Crystalline e o compilador Vanilla oficial (`typst 0.13.1` em `lab/typst-original`) em 7 cenários distintos cobrindo:
- **Heading Nível 1 vs Heading Nível 2**
- **Equações Simples, Frações de Alto Ascent, Operadores com Limites Inferiores e Equações com Texto em Bloco**

| Cenário de Teste | Vanilla Dist (pt) | Crystalline Dist (pt) | Delta ($\Delta$) | Status |
|---|---|---|---|---|
| **H1 + Simples** (`$ x + y = z $`) | `14.6630 pt` | `14.6630 pt` | **`+0.00000 pt`** | ✅ Exact Match |
| **H1 + Fração** (`$ (a+b)/(c+d) = 1 $`) | `23.3310 pt` | `23.3310 pt` | **`+0.00000 pt`** | ✅ Exact Match |
| **H1 + Operador Min** (`$ min_(x in RR) f(x) $`) | `16.4780 pt` | `16.4780 pt` | **`+0.00001 pt`** | ✅ Exact Match |
| **H2 + Simples** (`$ x + y = z $`) | `14.6630 pt` | `14.6630 pt` | **`+0.00000 pt`** | ✅ Exact Match |
| **H2 + Fração** (`$ (a+b)/(c+d) = 1 $`) | `23.3310 pt` | `23.3310 pt` | **`-0.00000 pt`** | ✅ Exact Match |
| **H2 + Operador Min** (`$ min_(x in RR) f(x) $`) | `16.4780 pt` | `16.4780 pt` | **`+0.00000 pt`** | ✅ Exact Match |
| **H2 + Otimização Seção 30** (Eq 1) | `19.1950 pt` | `19.1950 pt` | **`+0.00000 pt`** | ✅ Exact Match |

---

## 5. Verificação da Suíte de Testes

- `cargo test --workspace`: **5.952 testes unitários e de integração passando com 0 falhas**.
- `cargo test --test crystalline_lint`: **PASS** (Zero violações arquiteturais e de regras de tipos).
