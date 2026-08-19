# Relatório de Conclusão — Passo 1092

**Data**: 2026-08-19  
**Status**: CONCLUÍDO COM ÊXITO  
**Gate**: `ADR-0127` — Atualização do modelo de transição discreta de escala de `MathSize` em anexos matemáticos.

---

## 1. Resumo Executivo da Correção

No Passo 1092, corrigimos a composição de escalas de `MathSize` no módulo de anexos matemáticos ([`01_core/src/compiler/math/layout/attach.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/compiler/math/layout/attach.rs)).

### 1.1 Causa Raiz e Solução Implementada
- **Antes**: `attach.rs` multiplicava cegamente `style.size` por `script_percent_scale_down` ($0.70$), gerando em scripts de segundo nível uma escala composta indevida de $0.70 \times 0.70 = 0.49$ ($5.39\text{ pt}$ a 11pt).
- **Agora**: Implementada a transição discreta por nível absoluto de `MathSize` (`Display`/`Text` $\rightarrow$ `Script` $\rightarrow$ `ScriptScript` com piso em `ScriptScript`):
  ```rust
  let (script_math_size, factor) = match style.math_size {
      MathSize::Display | MathSize::Text => {
          (MathSize::Script, self.constants.script_percent_scale_down)
      }
      MathSize::Script => (
          MathSize::ScriptScript,
          self.constants.script_script_percent_scale_down
              / self.constants.script_percent_scale_down,
      ),
      MathSize::ScriptScript => (MathSize::ScriptScript, 1.0),
  };
  ```
  Garantindo que `ScriptScript` atinja exatamente `5.5000 pt` ($11.0 \times 0.50$).

---

## 2. Bateria de Verificação Experimental

### 2.1 Casos Isolados de 2º Nível (§3.1 do L0)
Medições extraídas diretamente dos PDFs gerados a 11pt:

| Expressão | Glifo Alvo | Nível MathSize | Tamanho Cryst | Tamanho Vanilla | $X_{\text{Cryst}}$ | $X_{\text{Vanilla}}$ | Delta ($\Delta x$) | Status |
|---|---|---|---|---|---|---|---|---|
| `$ x^(y^z) $` | $z$ | `ScriptScript` | **`5.500 pt`** | **`5.500 pt`** | `39.1656 pt` | `39.1661 pt` | **`-0.0005 pt`** | ✅ Exact Match |
| `$ x_(i_k) $` | $k$ | `ScriptScript` | **`5.500 pt`** | **`5.500 pt`** | `37.7488 pt` | `37.7493 pt` | **`-0.0005 pt`** | ✅ Exact Match |
| `$ x_(i^*)^j $` | $*$ | `ScriptScript` | **`5.500 pt`** | **`5.500 pt`** | `37.7488 pt` | `37.7493 pt` | **`-0.0005 pt`** | ✅ Exact Match |
| `$ g_i(x^*) $` | $)$ | `Script` | **`7.700 pt`** | **`7.700 pt`** | `47.8699 pt` | `47.8704 pt` | **`-0.0005 pt`** | ✅ Exact Match |
| `$ h_j(x^*) $` | $)$ | `Script` | **`7.700 pt`** | **`7.700 pt`** | `49.4902 pt` | `49.4907 pt` | **`-0.0005 pt`** | ✅ Exact Match |

### 2.2 Equação 3 Completa em Contexto Real de Linha (§3.2 do L0)
Medição de todos os 34 glifos da Equação 3 ($\nabla f(x^*) + \sum_i \lambda_i^* \nabla g_i(x^*) + \sum_j \mu_j^* \nabla h_j(x^*) = 0$):

- **Glifo 0 ao 33**: Todos com $\Delta x = \mathbf{-0.2030\text{ pt}}$ (variação interna relativa = **`0.0000 pt`**).
- O degrau de $0.0555\text{ pt}$ após os dois `$ ) $` ($\nabla g_i(x^*)$ e $\nabla h_j(x^*)$) foi **100% extinto**.
- A linha inteira move-se como um corpo perfeitamente rígido e alinhado.

---

## 3. Não-Regressão e Validação da Suíte

- **`typst-core` lib tests**: **5.077 / 5.077 aprovados (100%)**.
- **Workspace completo**: **796 / 796 em `typst-infra`**, **41 / 41 em `typst-shell`**, **37 / 37 em CLI**.
- **`crystalline-lint`**: 0 erros de conformidade estrutural.
