# Passo 902 — `partial`: cristalino usa `∂` (U+2202), vanilla usa `𝜕` (U+1D715)

**Precede este passo**: mensagem desta conversa que catalogou o achado (comparação de texto
extraído da secção 4/11/18, `∂/∂x` vs `𝜕/𝜕x`). Este é o achado mais simples da fila actual — mesma
classe do `dot`/`⋅` já corrigido em P894, mapeamento de tabela.

**Pré-condição de árvore**: `git status`.

---

## Fase A — confirmar antes de corrigir

1. Confirmar no vanilla (`lab/typst-original/`, tabela de símbolos) se `partial` de facto mapeia
   sempre para `𝜕` (U+1D715, variante itálica matemática), ou se depende de contexto (por exemplo,
   dentro vs fora de itálico automático — alguns símbolos têm variante upright e itálica e a escolha
   depende do estilo activo, não é sempre a mesma). Não presumir que é sempre `𝜕` sem confirmar.
2. Localizar a entrada de `partial` em `01_core/src/rules/math/symbols.rs` (mesmo ficheiro onde
   `dot` foi corrigido em P894) e confirmar o codepoint actual (`∂`, U+2202).
3. Confirmar se `𝜕` (U+1D715) está coberto pela fonte usada (`NewCMMath`) antes de trocar — se não
   estiver, trocar produziria um símbolo em falta em vez de um símbolo errado, o que seria pior.
   Verificar com `fontTools`, mesmo método já usado em P890/891/892/893.

## Fase B — Implementação (TDD, sem necessidade de dois agentes — mapeamento de tabela simples,
mesmo padrão do `dot` em P894)

1. Teste que falhe primeiro: `partial` resolve para o codepoint correcto confirmado na Fase A.
2. Corrigir a entrada.
3. Suíte completa verde, discriminada por crate.
4. Recompilar as secções 4/11/18 e confirmar visualmente.
5. `cargo run -- .` — zero violations.

## Fase C — Regressão

Este é um mapeamento de tabela sem impacto de layout — benchmark completo de qualquer forma, por
disciplina, mas não se espera diferença.

## Resultado esperado

- Header de linhagem actualizado.
- Teste novo.
- Relatório com: confirmação do codepoint correcto (contextual ou fixo), confirmação de cobertura
  na fonte, confirmação visual, benchmark completo.
