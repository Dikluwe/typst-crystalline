# Passo 988 — dois achados de decorador na seção 10: gap de `⎵_⎵` não corrigido (mesmo bug de P985) + acento sem compensar itálico

**Precede este passo**: dois achados da auditoria (§8.3, §8.4, 2026-08-06).

---

## Parte A — `⎵_⎵` (traço reto com cantos, sem legenda) tem o mesmo bug que P985 corrigiu em `⏟`

**Achado**: gap conteúdo→chave já corrigido para `⏟` (chave curva com legenda, P985: 0.91pt =
vanilla). Mas `⎵_⎵` (traço reto com cantos, sem legenda) continua com o bug antigo — 9.78pt no
cristalino contra 0.91pt no vanilla. Mesmo sintoma, mesma família (`underover`/spreader), mas P985
não cobriu este segundo construto.

### Fase A
1. Confirmar se `⎵_⎵` passa pelo mesmo código de `resolve_underoverspreader`/`layout_accent`
   corrigido em P985, ou se é um caminho de código irmão, separado, que replica a mesma lógica
   antiga sem receber a correção.
2. Se for caminho separado: confirmar por que existem dois caminhos para construções visualmente
   semelhantes (chave com legenda vs traço reto sem legenda) — pode ser intencional (símbolos
   diferentes, tratamento genuinamente distinto no vanilla) ou pode ser duplicação de código que
   devia convergir num só.

### Fase B
TDD directo se for aplicar a mesma correção de P985 a este caminho; ou unificar os dois caminhos
se a Fase A confirmar que deviam ser o mesmo código.

## Parte B — acento (chapéu/til) não desloca para compensar inclinação do itálico

**Achado**: vanilla desloca o acento 0.47pt para a direita do centro da caixa delimitadora do "x"
— compensação pela inclinação do itálico. Cristalino centra exatamente no centro da caixa, sem
compensação. Pequeno, mas sistemático.

### Fase A
1. Ler o mecanismo do vanilla para este deslocamento — provavelmente relacionado à
   `italics_correction` do glifo base (mesmo termo já implementado em P971/975 para outros
   contextos, aplicado aqui à centragem horizontal do acento).
2. Confirmar a implementação actual de centragem de acento (`accent.rs`).

### Fase B
TDD directo — adicionar o termo de compensação à fórmula de centragem horizontal.

## Fase C — Revalidação conjunta

1. Medir os dois achados de novo no documento de 30 secções.
2. Confirmação visual.
3. Benchmark completo, 7 cenários, `depois/antes`, zero regressão.

## Resultado esperado

- `⎵_⎵` com o mesmo gap correto de `⏟` (0.91pt, não 9.78pt).
- Acento deslocado 0.47pt compensando itálico, como o vanilla.
- Benchmark sem regressão.
