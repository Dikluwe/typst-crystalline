# Passo 984 — largura do acento (`̂`/`̃`) ~30% maior que o vanilla, mesma letra base

**Precede este passo**: achado da auditoria (2026-08-06, §7.1) — inspeção manual da seção 10,
fora do alcance do emparelhamento por conteúdo (§6). Largura do glifo de acento: `̂` 7.08pt
(cristalino) vs 5.50pt (vanilla); `̃` 7.17pt vs 5.50pt. Largura do "x" base: 6.30pt vs 6.29pt
(idêntica) — a divergência é só no acento, não na base.

**Pré-condição de árvore**: `git status`. Confirmar P983 presente.

---

## Fase A — confirmar a causa

1. Confirmar qual glyph ID o cristalino está a escolher para `̂`/`̃` sobre uma base de largura
   normal (`x`) — comparar com o glyph ID que o vanilla escolhe no mesmo caso, via `fontTools`/
   `mutool trace`.
2. Ler o mecanismo de seleção de variante de acento (`accent.rs`, já tocado em P906/922/961) —
   confirmar se há mais de uma variante de largura disponível na fonte para `̂`/`̃` (acentos
   normalmente têm variantes largas para bases largas, mesma família de mecanismo de assembly já
   visto para delimitadores) e se o cristalino está a escolher a variante errada para este
   tamanho de base.
3. Confirmar a fórmula real do vanilla para seleção de largura de acento — provavelmente baseada
   na largura da base (`accent_base_height`/mecanismo já parcialmente lido em P922), não um
   tamanho fixo.

## Fase B — Implementação (TDD directo se for correção de seleção de variante)

1. Teste com a largura esperada do acento para pelo menos duas larguras de base diferentes (base
   estreita como "x", base larga como "abc" sob acento largo se suportado) — confirmar que a
   largura do acento escala com a base, não é fixa.
2. Implementar.
3. Suíte completa verde, discriminada por crate.
4. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Medir `̂`/`̃` de novo no documento de 30 secções — confirmar largura próxima de 5.50pt para
   base "x".
2. Confirmação visual a alta resolução.
3. Benchmark completo, 7 cenários canónicos, `depois/antes`, zero regressão.

## Resultado esperado

- Largura de acento escalando corretamente com a base, não fixa/maior que o necessário.
- Benchmark sem regressão.
