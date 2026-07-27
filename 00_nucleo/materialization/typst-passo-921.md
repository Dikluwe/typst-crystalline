# Passo 921 — `assembly` não atinge a altura-alvo total em matrizes/casos de 6+ linhas

**Precede este passo**: `typst-passo-916-relatorio.md`/`typst-passo-917-relatorio.md` — achado
registado, não investigado: matrizes/casos de 6+ linhas montam o delimitador por peças
(`assembly`), mas não atingem a altura total pedida; caminho estruturalmente diferente do vanilla
(vanilla usa um único glifo de variante grande nesses casos, não montagem por partes).

**Menor prioridade dos quatro** — só afeta conteúdo muito alto (6+ linhas), caso menos comum que
os outros três itens desta rodada.

**Pré-condição de árvore**: `git status`. Confirmar P918/919/920 (se já commitados) presentes.

---

## Fase A — confirmar por que o cristalino vai para `assembly` onde o vanilla usa variante única

1. Para o mesmo caso (`mat(...)` 6 linhas), confirmar no vanilla real (`mutool trace`,
   `typst 0.15.1`) se de facto usa uma variante única grande (não montagem) — o achado de P916/917
   já sugeria isto, confirmar com medição directa antes de prosseguir.
2. Se o vanilla usa variante única: confirmar até que altura a fonte (`NewCMMath`) tem variantes
   prontas de `)` (8 variantes, já confirmado por P911/912) — se a altura pedida por 6 linhas
   ultrapassa a 8ª variante, o vanilla teria de ir para assembly também; se não ultrapassa, o
   cristalino está a ir para assembly cedo demais, sem tentar todas as 8 variantes primeiro.
3. Ler o código de decisão do cristalino (`stretchy.rs`/`layout_stretchy_delimiter`) que escolhe
   entre "usar variante pronta" e "montar por partes" — confirmar a condição exacta e comparar
   com a do vanilla (`glyph.rs`, mesmo módulo já lido em P913 para o algoritmo de `assemble`).
4. Se confirmado que o cristalino pula variantes prontas demasiado cedo: isto pode não ser um bug
   de `assembly` em si (a implementação de P913 já está correcta per o próprio relatório de P913),
   mas um bug na **decisão** de quando usar cada caminho — achado diferente do que o nome do passo
   sugere. Registar qual dos dois é, não presumir.

## Fase B — Implementação (protocolo de dois agentes se a causa for a lógica de decisão; TDD
directo se for só ajuste de fórmula/limiar)

1. Teste com ground-truth medido do vanilla real, para o limiar exacto de troca entre variante e
   assembly.
2. Implementar.
3. Suíte verde, discriminada por crate.
4. `mutool trace` para os 7 casos já catalogados por P911/916/917 (`(1/2)` até `cases(...)` 8
   linhas) — confirmar que a altura final bate com o vanilla em todos, não só nos que já
   funcionavam.

## Fase C — Regressão

Benchmark completo, 7 cenários, `--warmup 5 -m 20`, attestation completa (`L11`).

## Resultado esperado

- Causa confirmada: decisão de quando usar variante vs. assembly, ou algo em `assembly` em si.
- Recibo comparando os 7 casos ao vanilla, antes/depois.
- Benchmark completo atestado.

---

## Nota — isto pode fechar a frente de geometria matemática (P885-921)

Se os três passos desta rodada (P919, P920, P921) não revelarem achados novos que exijam mais um
passo, esta é a última rodada da frente iniciada em P885. Vale, no fim deste passo, decidir
explicitamente se é hora de consolidar o handoff, em vez de assumir que há sempre mais um achado
por aí.
