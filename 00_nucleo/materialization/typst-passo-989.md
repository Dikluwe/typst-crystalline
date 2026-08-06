# Passo 989 — pontos duplos de acento (segunda derivada, `𝑥̈`) sobrepostos em vez de deslocados

**Precede este passo**: achado da auditoria (§8.2, 2026-08-06) — no construto de "ponto sobre x"
repetido (segunda derivada), o cristalino coloca os dois pontos combinantes exatamente na mesma
posição (x e y idênticos) — sobrepostos, só um visível. Vanilla desloca os dois pontos 2.49pt um
do outro (mesmo x, top diferente) — dois pontos distintos, visualmente corretos.

**Pré-condição de árvore**: `git status`. Confirmar P988 presente.

---

## Fase A — confirmar a causa

1. Reproduzir isoladamente o construto de ponto duplo (segunda derivada — confirmar a sintaxe
   exacta usada no documento de teste, provavelmente `dot(dot(x))` ou equivalente de acento
   aninhado).
2. Ler o mecanismo de empilhamento de acentos aninhados no vanilla — quando um acento é aplicado
   sobre outro acento já aplicado, o segundo precisa de considerar a altura do primeiro como parte
   da "base" para o seu próprio posicionamento vertical (mecanismo já usado noutros contextos desta
   frente — `accent_base_height`, P922) e possivelmente um deslocamento horizontal para não
   colidir visualmente.
3. Confirmar a implementação atual do cristalino — candidato à causa: o segundo acento pode estar
   a ser posicionado usando as mesmas coordenadas do primeiro, sem considerar que a "base" agora
   inclui o acento anterior.

## Fase B — Implementação (TDD directo se for correção de composição vertical pontual)

1. Teste com `dot(dot(x))` (ou o construto real usado) — confirmar que os dois pontos ficam em
   posições distintas, com o deslocamento medido contra o vanilla.
2. Implementar.
3. Suíte completa verde, discriminada por crate.
4. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Medir de novo no documento de 30 secções — confirmar os dois pontos visíveis, distância
   próxima de 2.49pt.
2. Confirmação visual a alta resolução.
3. Benchmark completo, 7 cenários canónicos, `depois/antes`, zero regressão.

## Resultado esperado

- Pontos duplos de acento aninhado visualmente distintos, não sobrepostos.
- Benchmark sem regressão.
