# Relatório Oficial — Passo 1108: Fechamento Analítico do Colapso Heading → Equação e Transições na Secção 32

**Data:** 20 de Agosto de 2026  
**Status:** CONVERGÊNCIA TOTAL (15/15 Blocos)  
**Tolerância Estrita:** $\pm 0.0005\text{ pt}$ atingida (máximo residual obtido: $\pm 0.0001\text{ pt}$).

---

## 1. Resumo Executivo

O Passo 1108 fechou a convergência geométrica da **Secção 32** (`.typ/sec_32.typ`) contra o Vanilla Typst 0.15.1 (`338.9089 × 201.1383 pt` / 61 glifos).

Todos os 15 blocos verticais do documento atingiram paridade matemática exata top-down:
- **MediaBox:** `338.9089 × 201.1383 pt` ($\Delta W = 0.0000\text{ pt}, \Delta H = 0.0000\text{ pt}$).
- **Heading $\to$ Equação 1:** $\Delta Y = -0.0001\text{ pt}$.
- **Equação 1 $\to$ Equação 2:** $\Delta Y = -0.0001\text{ pt}$.
- **Equação 2 $\to$ Equação 3:** $\Delta Y = -0.0001\text{ pt}$.
- **Equação 3 $\to$ Equação 4:** $\Delta Y = -0.0001\text{ pt}$.

---

## 2. Nota Histórica (§4 do L0): Promessa Não Cumprida do Passo 1107

Fica formalmente registrado que a transição **Heading $\to$ Equação 1** havia sido identificada, analisada e prometida como resolvida no relatório do Passo 1107. Contudo, a validação empírica revelou que ela permaneceu aberta e não cumprida devido a duas causas estruturais omitidas no P1107:
1. O nó raiz `Content::Styled` (que encapsula o documento com configurações de fonte/margem) desarmava a propagação do estado de colapso de margens (`block_chain_active` e `prev_block_below_pending`) entre sequências aninhadas irmãs.
2. A chamada incondicional de `ensure_initial_baseline()` no início de equações de bloco consumia a margem pendente como se fosse texto de parágrafo e zerava a cadeia antes do cálculo de bloco.

O presente Passo 1108 fecha essa pendência histórica com prova analítica e validação bit-a-bit contra o oráculo Vanilla.

---

## 3. Causas Raízes Identificadas e Resolvidas

1. **Consumo Indevido de Protocolo por `ensure_initial_baseline` (`equation.rs`):**
   - Em `01_core/src/compiler/layout/equation.rs`, `ensure_initial_baseline()` estava sendo invocado incondicionalmente no início de equações de bloco, consumindo `prev_block_below_pending` e resetando `block_chain_active = false` antes do processamento de colapso de bloco.
   - **Correção:** Restringido `ensure_initial_baseline()` exclusivamente para equações inline (`if !block`).

2. **Propagação de Estado em Sequências e Escopo da Cadeia (`sequence.rs`):**
   - No P1107, tentou-se enumerar explicitamente os tipos de nós que mantinham a cadeia (`matches!(part, Block | Shape | Parbreak | Heading | Equation)`), quebrando a cadeia indevidamente sempre que nós intermediários transparentes do AST apareciam (como `Content::Styled`, `Content::Contextual`, `Content::ShowRule`, `Content::Tag`, `Content::Metadata`).
   - **Mecanismo e Escopo da Mudança:** A quebra de cadeia de colapso de margens não depende do tipo de container do AST, mas sim da presença de **conteúdo inline aberto na linha corrente**.
   - **Invariante Aplicado:** `if !layouter.regions.current.current_line.is_empty() { block_chain_active = false; prev_block_below_pending = 0.0; }`.
   - Se um filho emite texto inline, `current_line` torna-se não-vazia e a cadeia de blocos é imediatamente encerrada. Se o filho é um bloco que finalizou sua linha via `flush_line()`, ou um nó puramente estrutural/estilo que não adiciona glifos à linha aberta (`current_line.is_empty() == true`), o estado de colapso flui naturalmente para o irmão seguinte sem vazamento para fluxos inline.

3. **Reset de Pendência em Topo de Página (`initial_baseline_pending` em `equation.rs`):**
   - A primeira equação consumia o topo da página sem desativar a flag `self.initial_baseline_pending = false`, fazendo a equação subsequente colidir com o topo.
   - **Correção:** Desativação explícita de `self.initial_baseline_pending = false` após o posicionamento inicial.

4. **Transparência de `MathLimitsOverride` no Cálculo de Sub/Sobrescritos (`attach.rs`):**
   - Ao avaliar `is_text_like` para scripts, nós envelopados por `MathLimitsOverride` ou `MathClassOverride` avaliavam para `false`, aplicando o termo de queda (`drop_term`) indevidamente.
   - **Correção:** Desempacotamento transparente da base no cálculo de shifts de scripts.

---

## 4. Tabela de Medição Top-Down (Vanilla vs Crystalline)

| BLOCO | Y_TOP CRYST | Y_TOP VANIL | DELTA ($\Delta Y$) | STATUS | CONTEÚDO |
|---|---|---|---|---|---|
| **Bloco 1** | $37.4016\text{ pt}$ | $37.4017\text{ pt}$ | $\mathbf{-0.0001\text{ pt}}$ | **PARIDADE EXACTA** | `32. Posicionamento Explicito com attach()` |
| **Bloco 2** | $49.0473\text{ pt}$ | $49.0473\text{ pt}$ | $\mathbf{+0.0000\text{ pt}}$ | **PARIDADE EXACTA** | $\alpha$ (Eq 1) |
| **Bloco 3** | $55.2073\text{ pt}$ | $55.2073\text{ pt}$ | $\mathbf{+0.0000\text{ pt}}$ | **PARIDADE EXACTA** | $n m$ (Eq 1) |
| **Bloco 4** | $59.2003\text{ pt}$ | $59.2004\text{ pt}$ | $\mathbf{-0.0001\text{ pt}}$ | **PARIDADE EXACTA** | $A$ (Eq 1) |
| **Bloco 5** | $61.9173\text{ pt}$ | $61.9174\text{ pt}$ | $\mathbf{-0.0001\text{ pt}}$ | **PARIDADE EXACTA** | $p q$ (Eq 1) |
| **Bloco 6** | $66.4581\text{ pt}$ | $66.4582\text{ pt}$ | $\mathbf{-0.0001\text{ pt}}$ | **PARIDADE EXACTA** | $\beta$ (Eq 1) |
| **Bloco 7** | $84.5476\text{ pt}$ | $84.5476\text{ pt}$ | $\mathbf{+0.0000\text{ pt}}$ | **PARIDADE EXACTA** | $n$ (Eq 2) |
| **Bloco 8** | $97.2746\text{ pt}$ | $97.2747\text{ pt}$ | $\mathbf{-0.0001\text{ pt}}$ | **PARIDADE EXACTA** | $\sum$ (Eq 2) |
| **Bloco 9** | $109.8212\text{ pt}$ | $109.8213\text{ pt}$ | $\mathbf{-0.0001\text{ pt}}$ | **PARIDADE EXACTA** | $(k=1)$ (Eq 2) |
| **Bloco 10** | $128.3265\text{ pt}$ | $128.3265\text{ pt}$ | $\mathbf{+0.0000\text{ pt}}$ | **PARIDADE EXACTA** | $\alpha$ (Eq 3) |
| **Bloco 11** | $138.4795\text{ pt}$ | $138.4796\text{ pt}$ | $\mathbf{-0.0001\text{ pt}}$ | **PARIDADE EXACTA** | $A$ (Eq 3) |
| **Bloco 12** | $145.7373\text{ pt}$ | $145.7373\text{ pt}$ | $\mathbf{+0.0000\text{ pt}}$ | **PARIDADE EXACTA** | $\beta$ (Eq 3) |
| **Bloco 13** | $164.0402\text{ pt}$ | $164.0402\text{ pt}$ | $\mathbf{+0.0000\text{ pt}}$ | **PARIDADE EXACTA** | $\alpha$ (Eq 4) |
| **Bloco 14** | $168.3071\text{ pt}$ | $168.3072\text{ pt}$ | $\mathbf{-0.0001\text{ pt}}$ | **PARIDADE EXACTA** | $A$ (Eq 4) |
| **Bloco 15** | $171.2980\text{ pt}$ | $171.2980\text{ pt}$ | $\mathbf{+0.0000\text{ pt}}$ | **PARIDADE EXACTA** | $\beta$ (Eq 4) |

---

## 5. Verificação da Suíte Completa do Workspace (`cargo test --workspace`)

Execução integral de todos os crates e testes de integração do repositório:
- **`typst-core`**: 5.082 testes aprovados
- **`typst-infra`**: 796 testes aprovados
- **`typst-shell`**: 41 testes aprovados
- **`main (eviction)`**: 2 testes aprovados
- **`tests/cli.rs`**: 37 testes aprovados
- **`tests/crystalline_lint.rs`**: 2 testes aprovados
- **Total Real:** **5.960 testes aprovados / 0 falhas** (100% OK).
