# Passo 972 — parênteses `(`/`)` desalinhados da baseline em fração com conteúdo complexo (`n(n+1)/2`)

**Precede este passo**: achado 9.3 da auditoria externa (2026-08-05) — no vanilla, `n(n+1)` no
numerador de uma fração tem todos os caracteres (`n`, `(`, `n`, `+`, `1`, `)`) na mesma linha de
base. No cristalino, os dígitos ficam num `top`, mas os parênteses ficam 3.5pt mais abaixo —
desalinhados do conteúdo que envolvem. Mesmo tamanho de fonte nos dois lados (11pt) — problema
puramente de alinhamento vertical.

**Pré-condição de árvore**: `git status`. Confirmar P971 (se já executado) presente.

---

## Fase A — confirmar a causa

1. Reproduzir isoladamente `$ n(n+1) $` (fora de fração, para isolar se o problema é do delimitador
   em si ou específico ao contexto de numerador de fração) e medir a baseline dos parênteses vs os
   dígitos.
2. Confirmar se `delimited.rs`/`stretchy.rs` posiciona o delimitador pela baseline do conteúdo
   interno corretamente — este é território já tocado por P901 (barra de radical, convenção
   baseline) e outros passos de geometria; confirmar se este é o mesmo tipo de erro de convenção
   (`y=0=topo` vs `y=0=baseline`) reaparecendo num terceiro lugar, ou uma causa nova.
3. Confirmar a fórmula real do vanilla para ancoragem de delimitador de altura normal (não
   esticado — o `(`/`)` aqui não precisam de assembly, são glifo único, do mesmo tamanho do
   conteúdo).

## Fase B — Implementação (TDD directo se for correção de convenção pontual)

1. Teste com a baseline esperada dos parênteses, igual à do conteúdo interno.
2. Implementar.
3. Suíte verde. Confirmar que delimitadores esticados (assembly, já corrigidos em P957) não
   regridem.
4. `cargo run -- .` — zero violations.

## Fase C — Revalidação

`compare.py` nas seções com fração+delimitador aninhado. Benchmark completo, 7 cenários,
`depois/antes`.

## Resultado esperado

- Parênteses de altura normal alinhados à baseline do conteúdo que envolvem, em qualquer contexto
  (dentro ou fora de fração).
- Delimitadores esticados (assembly) não afetados.
- Benchmark sem regressão.
