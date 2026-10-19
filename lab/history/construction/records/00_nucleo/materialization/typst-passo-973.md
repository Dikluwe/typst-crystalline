# Passo 973 — varredura sistemática: catch-alls silenciosos em `match` sobre `FrameItem` em `math/layout/`

**Precede este passo**: lição operacional de P972 — o bug de parênteses desalinhados existia porque
`frac.rs` tinha um braço `_ => {}` que tratava `Text`/`TextShaped` mas ignorava `Glyph`/`Line`
silenciosamente. O bug só se manifestava com fonte real (`with_fonts_and_system`), nunca com
`FixedMetrics` (onde o parêntese sempre sai como `Text`) — por isso nenhum teste de unidade
existente o tinha pego. Dúvida real, ainda não respondida: há mais braços `_ =>`/catch-all do mesmo
tipo em `math/layout/`, escondidos pela mesma razão?

**Pré-condição de árvore**: `git status`. Confirmar P969-972 presentes.

---

## Fase A — inventariar todos os `match`/`if let` sobre `FrameItem` em `math/layout/`

1. `grep -rn "FrameItem::" 01_core/src/engine/math/layout/` — listar todos os ficheiros e pontos
   que fazem pattern matching sobre `FrameItem` (não só `frac.rs`).
2. Para cada ponto encontrado: confirmar se cobre todas as variantes de `FrameItem` explicitamente,
   ou se tem um braço catch-all (`_ => {}`, `_ => item`, ou equivalente que não aplica a mesma
   transformação que os braços nomeados aplicam).
3. Para cada catch-all encontrado: confirmar se isso é intencional (documentado, com razão) ou é o
   mesmo padrão de omissão de P972 — braço esquecido, não decisão.
4. Para cada catch-all suspeito: confirmar se o caminho é exercitável só com fonte real (mesmo
   teste de P972 — construir o caso mínimo com `with_fonts_and_system(&[])` e ver se algum item
   chega como `Glyph`/`Line`/outra variante que o catch-all está a ignorar).

## Fase A.1 — priorizar achados

1. Separar achados confirmados como bugs reais (catch-all esconde transformação necessária) dos
   que são catch-all legítimo (documentar por que é seguro ali).
2. Para os bugs reais: confirmar se algum já está catalogado por outro passo desta frente sob outro
   nome (por exemplo, se algum resíduo já registado em P944-972 tinha esta mesma causa sem ter sido
   identificada assim) — evitar duplicar investigação.

## Fase B — corrigir os bugs reais confirmados

TDD directo por caso, mesmo padrão de P972 — teste de integração com fonte real (não `FixedMetrics`,
per a lição já registada), RED confirmado antes da correção.

## Fase C — Revalidação e prevenção

1. Suíte completa verde, discriminada por crate, para cada correção.
2. Revalidação do `.typ` de 30 secções, `compare.py`, benchmark completo — mesma disciplina de
   sempre.
3. **Prevenção**: considerar se vale adicionar um teste/lint próprio que force qualquer `match`
   novo sobre `FrameItem` em `math/layout/` a cobrir todas as variantes explicitamente (via
   `#[non_exhaustive]` ou revisão manual do padrão) — para que este tipo de omissão não precise de
   ser encontrado por acidente de novo. Registar a decisão, não necessariamente implementar se o
   custo for desproporcional ao benefício.

## Resultado esperado

- Inventário completo de todos os `match`/`if let` sobre `FrameItem` em `math/layout/`, com
  veredicto (catch-all legítimo vs bug) para cada um.
- Bugs reais confirmados corrigidos, com teste de integração (fonte real) que os teria pego.
- Decisão registada sobre prevenção estrutural futura (lint/padrão), mesmo que não implementada
  agora.
- Benchmark completo, zero regressão.
