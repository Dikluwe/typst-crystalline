# Passo 922 — decisão arquitetural: representar extensões com sinal (`accent_base_height`)

**Precede este passo**: `typst-passo-920-relatorio.md`, Parte A destacada. Ler antes de começar —
a pergunta já está isolada com precisão: `text_ink_bounds` do cristalino garante `ascent`/
`descent >= 0` por contrato (`engine/layout/metrics.rs:59-61`); o vanilla precisa de
`accent.descent()` poder ser **negativo** (tinta do acento inteiramente acima da própria
baseline) para portar `gap = -accent.descent() - base.ascent().min(accent_base_height)`
fielmente.

**Este é um passo de decisão arquitetural, não de correção pontual** — mesmo cuidado de
P893/896/906/909/915/918/919.

**Pré-condição de árvore**: `git status`. Confirmar P921 (último commit da rodada anterior)
presente.

---

## Fase A — mapear o problema antes de desenhar a solução

1. Confirmar, por leitura do vanilla (`typst-layout/src/math/`, módulos de fonte/glyph), como
   `descent()`/`ascent()` são representados lá — são sempre `f64` com sinal livre, ou há um tipo
   específico que já modela isto? Confirmar se este é um padrão isolado a acentos ou se aparece
   noutros lugares do vanilla (se aparecer em mais sítios, a decisão de arquitetura vale mais a
   pena; se for só aqui, um caso local pode bastar).
2. Confirmar todos os consumidores actuais de `FontMetrics::text_ink_bounds`/`ascent`/`descent`
   no cristalino — mudar o contrato para permitir sinal pode afectar mais do que só `accent.rs`.
   Listar exaustivamente, não presumir que é só o achado de P920.
3. Desenhar pelo menos duas opções, com custo/risco de cada:
   - **(a) Estender o contrato geral** (`text_ink_bounds` passa a poder devolver valores
     negativos) — mais correcto, mais risco de quebrar os outros consumidores já listados no
     ponto 2.
   - **(b) Método novo, específico** (algo como `text_ink_bounds_signed` ou um campo adicional só
     para este caso) — mais contido, não risca os consumidores existentes, mas introduz uma
     segunda forma de obter a mesma informação, potencial confusão futura.
   - Registar qual se escolhe e porquê — não implementar sem essa decisão registada e confirmada.

## Fase A.1 — gate (obrigatório, mudança de contrato de trait)

Editar os L0s afectados (`engine/layout/metrics.md`, `font_metrics.md`, `accent.md`, e qualquer
outro identificado no ponto 2 da Fase A), sincronizar hashes, **parar para confirmação do dono**
antes da Fase B.

## Fase B — Implementação (protocolo de dois agentes de P898/919 — geometria com risco de efeito
em cascata, dado o número de consumidores potencialmente afectados)

1. Agente A escreve testes com ground-truth medido do vanilla real (mesmo método de P920 — binário
   `lab/typst-original/target/release/typst`, `mutool trace`), cobrindo pelo menos: base pequena
   (gap maior, per o comentário do vanilla já citado em P920) e base grande (gap menor/cap).
2. Agente B implementa `gap = -accent.descent() - base.ascent().min(accent_base_height)`
   fielmente, usando a extensão de contrato desenhada na Fase A.
3. Revisão do orquestrador — confirmar que os consumidores existentes de `text_ink_bounds`
   (listados na Fase A ponto 2) não regrediram, um a um, não só "suíte passa".
4. Suíte completa verde, discriminada por crate.
5. Confirmação geométrica (`mutool trace`, `L11`): `hat`/`tilde`/`overbrace` com bases de tamanhos
   diferentes, comparado ao vanilla real.
6. `cargo run -- .` — zero violations.

## Fase C — Regressão

Benchmark completo, 7 cenários, `--warmup 5 -m 20`, attestation completa (`L11`).

## Resultado esperado

- Decisão de arquitectura registada e aprovada antes da Fase B.
- Extensão de contrato implementada sem regressão nos consumidores existentes.
- Recibo geométrico comparando ao vanilla, casos de base pequena e grande.
- Benchmark completo atestado.
