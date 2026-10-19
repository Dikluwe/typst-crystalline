# Passo 985 — distribuição de gap invertida em `underbrace`/`overbrace` (chave longe do conteúdo, legenda colada na chave)

**Precede este passo**: achado da auditoria (2026-08-06, §7.2) — `⏟` + "soma": gap total
parecido nos dois (~15-16pt), mas distribuição invertida. Vanilla: chave colada no conteúdo
(0.91pt), legenda afastada da chave (13.91pt). Cristalino: chave afastada do conteúdo (9.78pt),
legenda quase colada na chave (6.08pt — "quase tocando a curva", defeito visível).

**Pré-condição de árvore**: `git status`. Confirmar P984 presente.

---

## Fase A — confirmar a fórmula real do vanilla

1. Ler `resolve_underoverspreader`/mecanismo de gap já parcialmente lido em P906/920/961 — desta
   vez focado especificamente nos dois gaps separados (conteúdo↔chave, chave↔legenda), não só o
   tamanho da legenda (já corrigido em P961).
2. Confirmar os dois termos/constantes que decidem cada gap separadamente — provavelmente a chave
   tem um gap mínimo pequeno e fixo ao conteúdo (comportamento tipo "acento", per P906), e a
   legenda tem um gap maior, próprio, à chave.
3. Confirmar a implementação atual do cristalino — onde os dois gaps são calculados e por que
   saem invertidos (candidato: os dois valores podem estar a ser lidos/aplicados trocados um pelo
   outro).

## Fase B — Implementação (TDD directo se for troca pontual de valores; protocolo de dois agentes
se a fórmula for mais complexa que uma troca simples)

1. Teste com os dois gaps esperados (conteúdo↔chave pequeno, chave↔legenda maior), derivados da
   fórmula real e valores da fonte.
2. Implementar.
3. Suíte completa verde, discriminada por crate.
4. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Medir os dois gaps de novo no documento de 30 secções — confirmar chave colada ao conteúdo,
   legenda com espaço adequado, sem tocar a curva.
2. Confirmação visual a alta resolução — confirmar que a legenda não toca mais a chave.
3. Benchmark completo, 7 cenários canónicos, `depois/antes`, zero regressão.

## Resultado esperado

- Chave colada ao conteúdo (gap pequeno), legenda com espaço real à chave (gap maior) — mesma
  distribuição do vanilla, não só o mesmo total.
- Defeito visual (legenda quase tocando a curva) eliminado.
- Benchmark sem regressão.
