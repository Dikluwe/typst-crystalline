# Passo 961 — legenda de `underbrace`/`overbrace` não reduzida de tamanho; base de acento não itálica

**Precede este passo**: dois achados da auditoria externa (2026-08-04, seção 5.4), ambos na seção
10 (Decoradores e Acentos), relacionados (aplicação de estilo/tamanho reduzido a sub-conteúdo de
construções matemáticas), mas tecnicamente distintos.

---

## Parte A — legenda de `⏟`/`⏞` (`underbrace`/`overbrace`) não reduzida

**Achado**: no vanilla, a palavra "soma" (legenda) é desenhada a 7.7pt (~70% de 11pt) — tamanho
reduzido, esperado para legenda. No cristalino, a mesma palavra é desenhada a 11pt, igual ao resto
da equação — sem redução. Problema de tamanho, não de posição.

### Fase A
1. Ler a fórmula real do vanilla para o tamanho da legenda de `underbrace`/`overbrace` — confirmar
   se usa o mesmo nível de `MathSize` já implementado em P945/952 (`Script`, por exemplo) ou uma
   fórmula própria.
2. Confirmar a implementação actual do cristalino (`underover.rs`, P906) — confirmar se o parâmetro
   de legenda (`label`) é layoutado com o `style`/`MathSize` ambiente sem redução, ou se há uma
   redução planejada mas não aplicada correctamente.

### Fase B (TDD directo, mapeamento de estilo — mesma família de P945)
1. Teste com o tamanho esperado da legenda derivado da fórmula real do vanilla.
2. Implementar.
3. Suíte verde, discriminada por crate. Confirmação visual/medição de tamanho de fonte real
   (`mutool trace`, tamanho do glifo).

## Parte B — base de acento usa `x` latino em vez de `𝑥` itálico matemático (U+1D465)

**Achado**: `x̂`, `x̃`, `ẋ` — cristalino desenha a base como `x` comum; vanilla desenha como `𝑥`
(itálico matemático). **Já catalogado**: P906 achado #5 registou exactamente isto — "`apply_math_
default` não recursa em `Content::MathAccent`" — a base/`under`/`over` de acentos nunca recebem o
tratamento itálico automático que identificadores de 1 letra normalmente recebem em modo
matemático.

### Fase A
1. Confirmar se o achado de P906 ainda descreve a causa correctamente (o código pode ter mudado
   desde então, em P944/945/952/957) — reler `apply_math_default` e `Content::MathAccent` no
   estado actual antes de presumir que a descrição de P906 continua válida sem alteração.

### Fase B (TDD directo — adicionar o braço em falta, já identificado por P906)
1. Teste confirmando que a base de `hat(x)`/`tilde(x)`/`dot(x)` recebe itálico automático quando é
   um identificador de 1 letra, mesma regra já aplicada fora de acentos.
2. Implementar o braço em falta em `apply_math_default` para `Content::MathAccent` (e confirmar se
   `Content::MathUnderover` tem o mesmo problema, já que P906 mencionou os dois).
3. Suíte verde. Confirmação visual/glifo.

## Fase C — Revalidação conjunta (as duas partes)

1. Recompilar o `.typ` de 30 secções, `compare.py`, confirmar melhoria na seção 10.
2. Confirmar visualmente os casos `hat`/`tilde`/`dot`/`underbrace`/`overbrace` contra o vanilla.
3. Benchmark completo, 7 cenários canónicos, `depois/antes`, zero regressão.

## Resultado esperado

- Legenda de `underbrace`/`overbrace` no tamanho correto (reduzido), confirmado por medição real.
- Base de acentos recebendo itálico automático, fechando o achado #5 de P906 que ficou pendente.
- Revalidação e benchmark sem regressão.
