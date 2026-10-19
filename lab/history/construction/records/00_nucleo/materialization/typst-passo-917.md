# Passo 917 — por que `vertical_glyph_variants` continua vazia para casos simples, mesmo com `covering()` corrigido

**Precede este passo**: `typst-passo-916-relatorio.md`, Parte A ("Achado aberto de P916"). Ler
antes de começar — a medição já está feita, com números reais; este passo é diagnóstico da causa,
não remedição.

**Pré-condição de árvore**: `git status`. Confirmar estado de P916 (fixture nova, testes de
revisão, relatórios retroativos) commitado antes de começar.

---

## O que já se sabe (não redescobrir)

- `(1/2)`, `(1/2/3/4)`, `(1/2/3/4/5/6/7/8)`, `mat(1,2;3,4)`: glifo de `)` **fixo** (ID 2) nos
  quatro, cristalino. Vanilla cresce nos quatro (ID 4, 8, 12, 6).
- `mat(...)` 6 linhas, `cases(...)` 8 linhas: cristalino **cresce**, mas via montagem de peças
  (`assembly`, múltiplos glyph IDs — 34 peças no caso da matriz de 6 linhas), não via seleção de
  uma variante única maior — caminho estruturalmente diferente do vanilla nesses dois casos
  (vanilla usa `)` = ID 14 num único glifo, não montagem).
- `covering()` (P912) já prioriza fonte MATH — confirmado não ser a causa aqui (Parte D de P916 não
  achou problema em `attach.rs`, mas não auditou especificamente este ponto).

## Fase A — diagnóstico

1. Instrumentar (temporariamente, revertido antes de fechar, mesmo protocolo de P890/906)
   `layout_stretchy_delimiter`/`select_with_advance` para os 4 casos que ficam fixos — confirmar,
   com `eprintln!` ou equivalente, se `vertical_glyph_variants(')', ...)` está a devolver lista
   vazia (repetindo a medição de P911) ou uma lista não-vazia que depois não é usada
   correctamente (dois sintomas diferentes, causas diferentes).
2. Se ainda vazia: confirmar por que — `covering()` já deveria estar a devolver a fonte MATH para
   este carácter (P912). Ler o call site exacto que `layout_stretchy_delimiter` usa para obter a
   fonte antes de chamar `extract_variants` — confirmar se é o mesmo `covering()` corrigido, ou se
   há um caminho diferente (por exemplo, uma resolução de fonte separada para delimitadores vs.
   para outros consumidores de `covering()`, que P912 não tocou).
3. Se não-vazia: confirmar por que a variante não é seleccionada — comparar `min_height_du`/alvo
   (já auditado por P911/P912 quanto à fórmula) com os `advance`s reais das 8 variantes de `)` na
   fonte (`fontTools`, mesmo método de sempre) para estes casos específicos — pode ser que a
   fórmula do alvo, mesmo corrigida, ainda não ultrapasse o primeiro `advance` da lista de
   variantes para conteúdo de 2-4 andares (ou seja: as variantes existem, mas nenhuma delas é
   "grande o suficiente" que a fórmula peça, então a base é sempre escolhida por ser >= o alvo
   pequeno — isto seria um achado diferente do "lista vazia").
4. Confirmar por que os casos de 6-8 linhas pulam direto para `assembly` em vez de tentar as 8
   variantes primeiro — o vanilla tenta variantes prontas antes de montar por partes (montagem é
   o último recurso, quando nenhuma variante é alta o suficiente). Se o cristalino está a ir direto
   para assembly sem tentar variantes, isso é uma ordem de decisão diferente da vanilla, mesmo que
   o resultado final "cresça" — vale confirmar se é isso, porque explicaria por que os casos
   pequenos falham (variantes nunca tentadas com sucesso) e os grandes "funcionam por acidente"
   (assembly sempre disponível como fallback, independente de variantes).

## Fase B — Implementação (só depois da Fase A confirmar a causa exacta)

TDD normal ou protocolo de dois agentes, conforme a Fase A revelar (mapeamento simples de causa →
correcção pontual; ou lógica de decisão nova → geometria, dois agentes).

1. Teste que falhe primeiro, com o caso mínimo mais simples que falha (`(1/2)`).
2. Corrigir.
3. Repetir a tabela de `mutool trace` da Parte A de P916 para os 7 casos — confirmar que os 4 que
   falhavam agora crescem, e que os 2 que já cresciam (via assembly) continuam a funcionar, e
   idealmente passam a usar variante única quando apropriado (não assembly desnecessário).
4. Suíte completa verde, discriminada por crate — **números reais, não "ver cargo test"**.
5. `cargo run -- .` — zero violations.

## Fase C — Regressão

Benchmark completo, 7 cenários, `--warmup 5 -m 20`.

## Resultado esperado

- Causa exacta confirmada (lista vazia vs. fórmula de alvo vs. ordem de decisão variante-vs-assembly).
- Tabela de `mutool trace` final, todos os 7 casos de P911 crescendo correctamente.
- Suíte completa com números reais por crate.
- Benchmark completo.
