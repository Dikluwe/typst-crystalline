---
# P579 — Corrigir o cálculo do line_height em parágrafos com fontes multi-size

> **Passo:** 579
> **Data:** 2026-07-05
> **Foco:** O P578 refutou a hipótese de double flush em RTL e localizou o bug real de sobreposição vertical: a quebra de linha em parágrafos com tamanhos de fonte não-padrão (ex: 40pt) usa o valor fixo `self.font_size_pt` (11.0) para avançar o cursor vertical em `flush_line()`, em vez de usar o tamanho real/ativo do texto na linha. Este passo planeia a sonda, a atualização de especificação L0 e a implementação da correção geral do `line_height`.
> **Tipo:** Correção de bug estrutural de layout.
> **Tamanho:** M.
> **ADR-0108 e ADR-0109 EM VIGOR.**
> **Dependências:** P578 (onde a causa raiz foi isolada e provada).

---

## 1. Contexto e Sonda

O cristalino inicia o `Layouter` com `self.font_size_pt` derivado do default da página (11.0). Quando o utilizador redefine o tamanho do texto (ex: `#set text(size: 40pt)`), a cadeia de estilos atualiza `self.style.size` para 40.0, mas o campo `self.font_size_pt` permanece inalterado.

Ao quebrar a linha:
1. `flush_line()` em `01_core/src/rules/layout/cursor.rs` calcula o avanço usando:
   `let (_, line_height) = self.metrics.vertical_metrics(self.font_size_pt);`
   Isto faz o cursor avançar ~16pt (altura correspondente a 11pt) em vez de ~56pt (altura correspondente a 40pt), resultando em sobreposição visual das linhas consecutivas do parágrafo.
2. O `line_leading_pt` também é resolvido usando `self.font_size_pt.val()`.

### Metodologia de Medição (Proveniência)
* **Caso de Teste:**
  ```typst
  #set text(size: 40pt)
  one two three four five six seven
  ```
* **Estado esperado pós-correção:** `seven` posicionado na segunda linha com um espaçamento vertical proporcional a 40pt (cerca de 56pt de diferença no eixo Y em relação à primeira linha), sem sobreposição com a linha superior.

---

## 2. Planeamento das Tarefas

### Tarefa 1: Atualização de L0
Atualizar `00_nucleo/prompts/rules/layout.md` para especificar que o cálculo de métricas verticais (`line_height`, `line_leading_pt` e `ascender`) em `flush_line` e transições de página/coluna deve ser sensível ao tamanho real da fonte dos elementos contidos na linha (ou ao estilo ativo no layouter), em vez de usar a constante estática `self.font_size_pt`.

### Tarefa 2: Implementação da Correção
1. **Em `01_core/src/rules/layout/cursor.rs::flush_line`:**
   - Obter o tamanho máximo de fonte (`style.size`) entre todos os elementos textuais (`FrameItem::Text` e `FrameItem::TextShaped`) presentes na `current_line` que está prestes a ser drenada.
   - Caso `current_line` esteja vazia (ex: flush preventivo), utilizar `self.style.size` (tamanho ativo).
   - Utilizar esta fonte dinâmica para chamar `vertical_metrics` e para resolver `line_leading_pt`.
2. **Em `01_core/src/rules/layout/cursor.rs::new_page` e `start_column`:**
   - Usar `self.style.size` no cálculo de `ascender` inicial no topo da página/coluna.
3. **Em `01_core/src/rules/layout/mod.rs::layout_sub_frame_with_width`:**
   - Usar `self.style.size` para o cálculo de métricas da célula e do `ascender` inicial do sub-frame.

### Tarefa 3: Testes de Regressão e Validação
- Executar `cargo test --workspace` para assegurar que a alteração não quebra a suíte existente de snapshots ou comportamento básico de layout.
- Criar um teste dedicado para verificar a quebra de parágrafo com tamanho de fonte grande, validando matematicamente a distância vertical entre as linhas no PDF resultante.

---

## 3. Critérios de Aceitação

- [ ] L0 atualizado com a especificação dinâmica de `line_height` e hash do prompt recalculado.
- [ ] Implementação de `flush_line`, `new_page` e `layout_sub_frame_with_width` corrigida para usar o tamanho ativo/real da fonte.
- [ ] Teste de integração adicionado para parágrafo com fonte grande (`40pt`) validando que as linhas não se sobrepõem verticalmente.
- [ ] Todos os testes da workspace passam com sucesso.
- [ ] Linter `crystalline-lint .` limpo.
