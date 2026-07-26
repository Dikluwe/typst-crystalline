# Passo 913 — Achado B de P911: `assembly.rs` nunca repete peças extensoras (`is_extender`)

**Precede este passo**: `typst-passo-911-relatorio.md`, "Achado transversal B". Ler antes de
começar. **Recomendado depois de P912** (mesma cadeia `layout_stretchy_delimiter → variantes →
assembly`), mas o próprio relatório de P911 nota que o teste pode ser construído com
`StubMetrics`/dados configuráveis sem depender de P912 estar fechado primeiro — confirmar isso na
Fase A antes de decidir a ordem.

**Pré-condição de árvore**: `git status`. Confirmar se `P912` já fechou; se não, confirmar que este
passo não depende de facto dele antes de prosseguir em paralelo.

---

## Achado (não redescobrir — já confirmado por leitura de código, prova é o parâmetro morto)

`layout_assembly` (`assembly.rs:17-77`) itera `assembly.parts` exactamente uma vez cada, sem nunca
repetir uma peça marcada `is_extender`. O parâmetro `_target_advance` está explicitamente marcado
como não usado na própria assinatura. Resultado: a altura montada é sempre a mesma soma fixa,
independente do alvo pedido.

Vanilla (`typst-layout/src/math/fragment/glyph.rs:567-660`, `fn assemble`): `loop` com contador
`repeat`, testando repetidamente quantas cópias das peças extensoras são precisas até
`full >= target` (ou `repeat >= MAX_REPEATS`), com um `ratio` de espalhamento entre sobreposição
máxima e mínima.

## Fase A — confirmar antes de implementar

1. Ler `fn assemble` do vanilla linha a linha (`glyph.rs:567-660`) e confirmar o algoritmo exacto —
   não presumir a partir só da descrição de P911. Em particular: como `MAX_REPEATS` é definido, e o
   que acontece se mesmo o máximo de repetições não atingir o alvo (fallback, ou aceita o que der).
2. Confirmar o `ratio` de espalhamento entre sobreposição máxima e mínima — este é provavelmente o
   termo mais subtil da fórmula (decide como distribuir cada repetição entre as peças, não só
   quantas repetições).
3. Confirmar se um teste pode ser construído sem depender de `covering()`/P912 estarem corrigidos —
   usando um assembly sintético (`GlyphAssembly` construído directamente no teste, com peças
   marcadas `is_extender` explicitamente), não dependendo do pipeline real de resolução de fonte.
   Se confirmado, este passo não precisa esperar por P912.

## Fase B — Implementação (protocolo de dois agentes de P898 — é geometria/algoritmo novo)

1. Agente A escreve teste(s) com assembly sintético (Fase A ponto 3), confirmando que:
   - Um alvo maior que a soma das peças sem repetição produz repetição real de peças `is_extender`.
   - O resultado final atinge (ou fica dentro de tolerância de) o alvo pedido, não a soma fixa
     original.
   - Um alvo menor ou igual à soma sem repetição não produz repetição desnecessária (comportamento
     preservado no caso já coberto, o que P906/909 já testaram).
   Confirma vermelho.
2. Agente B implementa o `loop`/`repeat` conforme o desenho da Fase A, usando `_target_advance`
   (deixa de ser morto).
3. Revisão do orquestrador — mesmo padrão dos passos anteriores: testar um caso de alvo muito maior
   que o esperado (para confirmar que `MAX_REPEATS`/limite é respeitado, sem loop infinito nem
   crescimento sem limite).
4. Suíte completa verde, discriminada por crate.
5. Se P912 já tiver fechado: recompilar um caso real de conteúdo muito alto (matriz grande, radical
   muito alto) e confirmar visualmente que o delimitador monta por partes correctamente, escalando.
   Se P912 ainda não tiver fechado: confirmar só via teste sintético, registar que a confirmação
   visual end-to-end fica pendente de P912.
6. `cargo run -- .` — zero violations.

## Fase C — Regressão

Benchmark completo, 7 cenários, `--warmup 5 -m 20`. Atenção a qualquer cenário com delimitadores
grandes — mas nota que este mecanismo só é exercitado quando o conteúdo excede o alcance das
variantes prontas, caso raro nos 7 cenários actuais; não esperar diferença mensurável.

## Resultado esperado

- Algoritmo de repetição implementado, `_target_advance` deixa de ser parâmetro morto.
- Testes novos com assembly sintético, cobrindo alvo maior/igual/muito-maior que a soma sem
  repetição.
- Confirmação visual end-to-end se P912 já tiver fechado; caso contrário, registado como pendente.
- Benchmark completo.
