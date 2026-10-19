# Passo 940 — por que `render_ms` de emoji é ~75× mais lento que o vanilla (CJK não sofre disso)

**Precede este passo**: `typst-passo-939-relatorio.md`, seção 2.3 — achado novo, não investigado:
`render_ms` do cristalino é ~300ms para `05-utf8`/`utf8-emoji` contra ~4ms do vanilla (~75×), mas
para `utf8-cjk` é só 0.3ms — praticamente igual ao vanilla. **O sinal é específico de emoji, não de
fallback em geral.**

**Pré-condição de árvore**: `git status`. Confirmar estado P938 (P939 revertido, conforme
registado) presente.

---

## Fase A — isolar o que é diferente sobre emoji, especificamente

1. Confirmar exatamente quais glifos de emoji estão a ser exportados nos casos de teste
   (`utf8-emoji.typ`, `05-utf8.typ`) e que formato a fonte emoji usa para representá-los —
   emojis coloridos normalmente usam uma de várias tabelas OpenType diferentes (`COLR`/`CPAL`
   — glifos vetoriais em camadas de cor; `CBDT`/`CBLC` — bitmap embutido; `SVG` — glifos como SVG).
   Confirmar qual a fonte de fallback do sistema usa para os emojis destes testes.
2. Confirmar como o cristalino exporta cada um desses formatos para PDF (`03_infra/src/export/`)
   — se lida com `COLR`/`CBDT`/`SVG` de forma específica, ou se cai num caminho genérico mais lento
   (por exemplo, rasterizar o glifo em vez de embutir a estrutura vetorial/bitmap directamente).
3. Confirmar como o vanilla exporta o mesmo tipo de glifo — ler o código-fonte real
   (`typst-pdf` ou equivalente no vanilla) para o caminho de exportação de glifos coloridos.
4. Medir, isoladamente, o tempo gasto por **glifo individual** de emoji exportado (não só o total)
   — confirmar se o custo é constante por glifo (multiplicado pelo número de emojis no documento)
   ou se há um custo fixo grande pago uma vez (por exemplo, processar a fonte inteira antes de
   exportar o primeiro glifo).
5. Confirmar se o mesmo padrão se repete para outros formatos de fonte colorida (se o sistema de
   testes tiver mais que uma fonte emoji disponível) — para saber se é específico de uma fonte ou
   do formato em geral.

## Fase B — Implementação (só depois da Fase A confirmar a causa exacta)

TDD directo ou protocolo de dois agentes, conforme a causa revelar (ajuste pontual vs. mudança de
caminho de exportação).

1. Teste com medição real (tempo por glifo, ou contagem de operações caras identificadas na Fase
   A) confirmando o problema antes da correcção.
2. Implementar.
3. Suíte completa verde, discriminada por crate.
4. Confirmar visualmente que os emojis continuam a aparecer corretamente no PDF depois da
   correcção — mudar o caminho de exportação de glifo é o tipo de mudança que pode acelerar às
   custas de desenhar o emoji errado ou em preto-e-branco.

## Fase C — Medição completa

7 cenários canônicos (`depois/antes`, zero regressão) + 5 casos UTF-8 (`depois/antes` e
`cristalino/vanilla-real`), `--min-runs 10`+, vanilla confirmado por string distintiva.
`render_ms` isolado (não só tempo total) para confirmar que a distância de 75× foi reduzida.

## Resultado esperado

- Causa exacta do `render_ms` lento para emoji confirmada (formato de fonte, caminho de
  exportação específico, ou outra coisa) — não presumida a partir de "emoji é mais complexo".
- Distância de 75× em `render_ms` reduzida, com número antes/depois.
- Confirmação visual de que os emojis continuam corretos.
- Nota explícita sobre se isto também explica parte da distância geral que sobrava depois de P938/
  939 (4.4×-6.1× no tempo total) — quantificar quanto do total essa correção fecha.
