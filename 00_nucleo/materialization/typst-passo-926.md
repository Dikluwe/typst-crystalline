# Passo 926 — implementar Opção 1 (pré-computar coverage no arranque), com custo no caso comum medido primeiro

**Precede este passo**: `typst-passo-925-relatorio.md` — diagnóstico completo, protótipo medido
(`8.6s → 0.20s` em `05-utf8`), Opção 1 recomendada (alinhada ao vanilla). **Antes de aprovar a
Opção 1 como está, este passo exige medir o custo no caso comum** (documento sem fallback nenhum),
que P925 não mediu.

**Pré-condição de árvore**: `git status`. Confirmar P925 (diagnóstico, sem código commitado —
protótipo foi removido após medição, per o próprio relatório) e P922-924 presentes.

---

## Fase A — medir o custo no caso comum antes de decidir a forma final

1. Reaplicar o protótipo de P925 (as três alterações temporárias já descritas: coverage em
   `FontInfo`, filtro por coverage no shaper, filtro por coverage em `FallbackFontMetrics`) e medir
   os **7 cenários canônicos** (`01-hello`...`07-context`, nenhum usa CJK/emoji), razão
   `depois/antes` — não só `05-utf8`. Isto é a pergunta que P925 não respondeu.
2. Se o aumento de arranque (~1.6s medido nesta máquina) se propagar integralmente para os 7
   cenários comuns: isso é uma regressão de ~9× no caminho mais frequente, para resolver um
   caminho raro. Decidir com o dono se isso é aceitável, ou se vale desenhar uma variante híbrida
   (ponto 3).
3. **Opção 5, candidata, só desenhar se o ponto 2 mostrar regressão inaceitável**: pré-computar a
   coverage numa thread de fundo, iniciada assim que `SystemWorld` é construído, sem bloquear o
   caminho principal — só bloquear (esperar a thread terminar) se `candidates_for_char` for de
   facto chamado antes da pré-computação acabar (caso raro: documento pequeno com CJK logo no
   início). Documentos sem fallback nunca esperam pela thread. Confirmar viabilidade (thread-safety
   do `coverage_cache`/`FontInfo`, já usa `Mutex` per P925 A.1) antes de desenhar em detalhe.
4. Confirmar também se o número de slots (1112, "608 famílias únicas") é representativo, ou se
   varia muito por máquina — se possível, medir também num ambiente com menos fontes instaladas
   (contentor mínimo, por exemplo), para saber se o custo de arranque é proporcional e até que
   ponto.

## Fase A.1 — gate (mudança de contrato de `FontInfo`, per o próprio P925)

Depois da Fase A decidir a forma final (Opção 1 pura, ou híbrida com thread de fundo): editar os
L0s identificados por P925 (`fontdb.md`, `system-world.md`, possivelmente `shaper.md`/
`font_metrics.md`), sincronizar hashes, **parar para confirmação do dono** — mesmo protocolo já
usado nesta frente inteira (P893/896/906/909/915/918/919/922).

## Fase B — Implementação (protocolo de dois agentes se a forma final envolver threading; TDD
directo se for só mover a extracção de coverage para `font_info_from_bytes`, sem concorrência)

1. Teste com medição real (mesmo padrão de P890/925 — `/usr/bin/time -v`/contagem de aberturas de
   ficheiro e faces carregadas, não só resultado final).
2. Implementar a forma decidida na Fase A.
3. Suíte completa verde, discriminada por crate.
4. Recompilar os 4 casos de bloco Unicode de P923 (`utf8-latin`, `utf8-greek`, `utf8-cjk`,
   `utf8-emoji`) — confirmar melhoria com números.
5. `cargo run -- .` — zero violations.

## Fase C — Regressão (a parte mais importante deste passo)

Benchmark completo, **9 cenários** desta vez: os 7 canónicos (`depois/antes`, para confirmar que o
caso comum não regrediu) **mais** os 2 piores casos de P925 (CJK, emoji, `depois/antes` e
`cristalino/vanilla`, para confirmar o ganho). Attestation completa (`L11`) para os 9.

## Resultado esperado

- Custo no caso comum medido e decidido explicitamente (aceitável como está, ou resolvido com
  forma híbrida) — não descoberto depois de já commitado.
- Implementação da forma decidida, com gate cumprido se mudar contrato público.
- Números antes/depois para os 4 casos de P923 (ganho) e para os 7 cenários canónicos (sem
  regressão, ou regressão aceita explicitamente com razão).
- Benchmark completo, 9 cenários, atestado.
