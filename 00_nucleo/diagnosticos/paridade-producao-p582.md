# Relatório Diagnóstico — Passo 582
## Consolidação de Paridade Geométrica com StyleChain (Grid e Tabelas)

- **Commit de Referência:** `76f73904c` (Working Tree não commitado)
- **Data/Hora da Medição:** 2026-07-06 14:12:00 UTC
- **Linter Status:** ✓ Clean (0 violations)
- **Status dos Testes:** Sucesso completo (cargo test passou sem erros)

---

## 1. Contexto e Objetivos

Este relatório registra o encerramento do **Passo 582**, focado na consolidação da migração arquitetural do gerenciamento de fontes. O antigo campo escalar estático `font_size_pt` foi integralmente removido da struct `Layouter`. Todo o motor de layout agora consome as dimensões tipográficas de forma puramente dinâmica a partir de `layouter.style.size` da `StyleChain`.

O objetivo principal desta etapa foi resolver as regressões geométricas causadas pela medição estática de caixas de grid, onde as colunas calculadas com largura `Auto` geravam sobreposição indevida do texto caso fossem estilizadas com tamanhos não padrão.

---

## 2. Resultados das Medições e Verificações

### 2.1. Correção Geométrica de Tabelas e Grids
Utilizou-se o caso de teste `p581-estruturas.typ` para analisar o posicionamento das colunas da tabela antes e depois da alteração:

- **Documento de Entrada:**
  ```typst
  #table(
    columns: 2,
    [#set text(size: 30pt); célula um linha um linha dois],
    [normal],
  )
  ```

- **Métricas de Posicionamento Extraídas (Antes da Correção):**
  - Célula 1 (Fonte: 30pt): `Position: 78.370 759.987 Td`
  - Célula 2 (Fonte: 8pt): `Position: 70.870 759.987 Td`
  - **Divergência:** A célula da segunda coluna começava em `70.870`, gerando uma sobreposição física completa com a primeira célula. O layouter media o texto de 30pt usando a métrica estática padrão de 8pt.

- **Métricas de Posicionamento Extraídas (Após a Correção):**
  - Célula 1 (Fonte: 30pt): `Position: 78.370 759.987 Td`
  - Célula 2 (Fonte: 8pt): `Position: 434.137 759.987 Td`
  - **Classificação:** **Sucesso**. A segunda coluna é empurrada exatamente para `434.137` pontos, respeitando a medição antecipada da primeira célula com o tamanho real de 30pt. O overlap geométrico foi totalmente extinto.

---

## 3. Implementação e Design no Motor de Layout

Para atingir a paridade geométrica em medições complexas sem introduzir estado global ou violar as restrições da camada L1, realizamos as seguintes alterações em `01_core/src/rules/layout/mod.rs`:

1. **Alteração de Assinatura:**
   - Métodos `measure_content_constrained` e `measure_stack` foram alterados de `&self` para `&mut self`.
   
2. **Propagação Dinâmica de Estilo no Motor de Medição:**
   - Adicionados os braços de pattern matching para as variantes `Content::Styled`, `Content::Strong` e `Content::Emph` dentro de `measure_content_constrained`.
   - Durante a medição antecipada, os novos estilos são acumulados na cadeia ativa (`self.chain`), recalculando `self.style` temporariamente, e restaurados imediatamente após a computação das dimensões:
     ```rust
     Content::Styled(body, styles) => {
         let prev_chain = self.chain.clone();
         let prev_style = self.style.clone();
         self.chain = self.chain.push_styles(styles);
         self.style = TextStyle::from(&self.chain);
         let res = self.measure_content_constrained(body, max_width);
         self.chain = prev_chain;
         self.style = prev_style;
         res
     }
     ```

3. **Adaptação dos Testes:**
   - Modificadas as declarações de `layouter` para `mut layouter` em `tests.rs` (linhas 2518, 2591, 2708) para suportar o borrow mutável exigido pela nova assinatura.

---

## 4. Conclusão de Fecho do Passo 582

- [x] O campo legado `font_size_pt` foi completamente eliminado do compilador.
- [x] Algoritmo `measure_content_constrained` calcula tamanhos corretos para elementos estilizados dinamicamente.
- [x] Testes integrados e testes unitários (646 no total) compilam e passam sem erros.
- [x] Linter `crystalline-lint .` retorna 0 violações (✓ Clean).
- [x] A paridade de layout geométrico de tabelas e grids foi restabelecida com sucesso absoluto.
