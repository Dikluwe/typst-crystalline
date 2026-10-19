# Passo 960 — quebra visível na curva do delimitador de assembly: residual pós-P957, ou build anterior?

**Precede este passo**: achado da auditoria externa (2026-08-04, seção 5.3) — comparando a mesma
matriz 3×3 (seção 5) a 400dpi nos dois PDFs: o vanilla produz curva contínua e lisa; o cristalino
mostra "quebra visível na curva, próxima ao ponto onde a peça de canto encontra o extensor — a
peça do meio não está alinhada em ângulo com as peças de canto".

**Antes de investigar como bug novo, confirmar se já foi corrigido por P957** — P957 (fechado em
2026-08-03) corrigiu a posição vertical (baseline) das peças de assembly; a auditoria externa que
achou este item está datada de 2026-08-04, mas não é possível confirmar, sem verificar, se o build
usado para gerar o `test_crystalline.pdf` da seção 5.3 já incluía a correção de P957 ou não.

**Pré-condição de árvore**: `git status`. Confirmar P957-959 (os que já tiverem sido executados)
presentes.

---

## Fase A — confirmar se o sintoma ainda existe pós-P957

1. Recompilar a mesma matriz 3×3 (seção 5) com o binário actual (pós-P957, e pós-P958/959 se já
   fechados) e renderizar a 400dpi, mesmo método da auditoria externa.
2. Comparar visualmente com o vanilla, no mesmo zoom/região que a auditoria usou — confirmar se a
   quebra na curva ainda é visível.
3. **Se não for mais visível**: o achado já estava corrigido por P957 (a build da auditoria era
   anterior) — registar isso, fechar o passo sem código.
4. **Se ainda for visível**: é um achado real, distinto de P957 — P957 corrigiu a posição vertical
   (translação) de cada peça; a quebra de ângulo sugere um problema de **alinhamento horizontal**
   ou de **rotação/tangente** entre peças consecutivas, não coberto pela correção de P957.

## Fase B — se confirmado como achado real, diagnosticar a causa

1. Confirmar, via `mutool trace`, a posição x de cada peça do assembly no ponto de junção —
   comparar com o vanilla. Se o x divergir entre peças consecutivas (não perfeitamente alinhado no
   mesmo eixo vertical), isso explicaria uma quebra visível na curva mesmo com y correto.
2. Confirmar se as peças de assembly precisam de alguma centragem horizontal própria (não só a
   centragem geral do delimitador já corrigida em P906/952) — ler o código de posicionamento x das
   peças em `assembly.rs`.
3. Confirmar contra a fórmula do vanilla (`glyph.rs`, mesmo módulo já lido em P906/913/957) se há
   algum termo de alinhamento horizontal por peça que o cristalino não esteja a aplicar.

## Fase C — Implementação (só se a Fase B confirmar causa real)

TDD directo ou protocolo de dois agentes, conforme o tamanho da correção. Teste com posição x real
de cada peça, comparando com o vanilla — não só visual.

## Resultado esperado

- Confirmação definitiva: já corrigido por P957 (registar e fechar), ou achado real com causa
  própria (corrigido, com prova numérica de alinhamento x, não só "parece melhor").
