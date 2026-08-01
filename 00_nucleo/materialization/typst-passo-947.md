# Passo 947 — verificar duplicação real de operadores de desenho no content stream (não pixel, não charstring isolada)

**Precede este passo**: `typst-passo-946-relatorio.md` concluiu "sem defeito de render", provado
por charstring idêntica e pixel-diff `None` a 300dpi. **Mas o texto extraído do PDF actual
(`review-now.pdf`, via `pdftotext`) continua a mostrar o mesmo padrão de duplicação de P945**
(`(||`, `{|{|{`, `|||||`) — e `pdftotext` lê operadores `Tj`/`TJ` do content stream directamente,
não pixels renderizados. Resolução/DPI não pode causar isto. Se o texto sai duplicado, o content
stream tem operadores de desenho duplicados, ponto — independente de como o pixel final aparenta.

**Hipótese de reconciliação com P946 (a confirmar, não presumir)**: os desenhos duplicados podem
estar exactamente na mesma posição (x,y), um sobre o outro — nesse caso, pixel-diff dá `None` e a
charstring de cada instância bate (é o mesmo glifo), mas ainda assim há um operador `Tj` a mais
por peça, real, no arquivo. Duplicação inofensiva ao olho, mas real, e provavelmente o mesmo
mecanismo de origem do achado de P945 (não corrigido, só mascarado da comparação visual/
charstring que P946 usou).

**Primeiro passo, antes de qualquer investigação**: confirmar se `review-now.pdf` foi de facto
gerado a partir do binário commitado de P946, ou se é um artefacto reenviado de sessão anterior —
`git status`/hash do binário/data de geração do PDF. Não presumir.

**Pré-condição de árvore**: `git status`. Confirmar P946 commitado.

---

## Fase A — confirmar a duplicação directamente no content stream, não por proxy

1. Se confirmado que `review-now.pdf` é do binário actual: extrair o content stream bruto da
   página (`qpdf --qdf`/`mutool show`/equivalente) e contar literalmente quantos operadores `Tj`/
   `TJ`/`Do` desenham a peça de delimitador para o caso mais simples do padrão (`(1 2 3/4 5 6/
   7 8 9)`, a matriz 3×3) — não inferir de `pdftotext`, ler o stream bruto.
2. Para cada operador de desenho da peça duplicada: extrair a matriz de transformação/posição
   (`cm`/coordenadas do `Tj`) activa nesse ponto — confirmar se as duas (ou mais) instâncias têm
   exactamente a mesma posição (x,y) ou se estão em posições ligeiramente diferentes (o que
   mudaria a explicação — nesse caso P946 deveria ter visto no pixel-diff, a menos que a
   diferença seja sub-pixel).
3. Se confirmado que são duas instâncias na mesma posição: localizar no código (`stretchy.rs`/
   `assembly.rs`/o ponto que emite os operadores de desenho em `export/`) onde a mesma peça é
   emitida duas vezes — isto é precisamente a suspeita original de P945 ("dois blocos de código
   independentes desenhando a mesma coisa em vez de um `else if`"), nunca confirmada nem
   refutada correctamente até agora.
4. Repetir para a chave de `cases()` (`{|{|{`) e para a matriz de reticências (`|||||`, o caso
   com mais peças, útil para confirmar se a duplicação escala com o número de peças do assembly).

## Fase B — corrigir a causa real (só depois da Fase A confirmar com o stream bruto)

TDD directo ou protocolo de dois agentes, conforme a causa exigir. Se for de facto "dois pontos
de código emitindo a mesma peça" (a hipótese original de P945, Fase A, ponto 5 — nunca
definitivamente resolvida): remover a emissão redundante, com teste que conte operadores `Tj` no
content stream (não só glifos visíveis), para que este tipo de duplicação invisível-mas-real não
volte a escapar de uma revisão futura.

## Fase C — revalidação, desta vez incluindo contagem de operador no content stream

1. `review.typ` e o `.typ` de 30 seções recompilados — desta vez a atestação inclui **contagem
   de operadores de desenho por peça de delimitador**, não só pixel-diff e charstring. Confirmar
   que a contagem bate com o vanilla (mesmo número de `Tj` por peça de assembly).
2. Tamanho do PDF antes/depois — se a duplicação for real, removê-la deve reduzir o tamanho do
   arquivo de forma mensurável, mesmo que pequena.
3. Benchmark completo, 7 cenários canônicos, `depois/antes`, zero regressão.

## Resultado esperado

- Confirmação definitiva, a nível de content stream bruto, se há ou não duplicação real de
  operadores de desenho — não mais inferência por pixel ou por charstring isolada.
- Se confirmada: causa exacta corrigida, com teste que conte operadores, não só resultado visual.
- Se refutada (o stream bruto mostra só uma instância por peça, e a duplicação no `pdftotext` tem
  outra explicação não considerada ainda): essa outra explicação identificada e registada — não
  fechar sem entender por que `pdftotext` duplica se o stream não tem operador a mais.

---

## Nota — por que isto importa além do arquivo específico

Se a causa for confirmada como "dois pontos de código emitem a mesma peça", isso é exactamente o
tipo de defeito que **nenhuma das validações usadas até agora (P945, P946) foi desenhada para
pegar** — pixel-diff e charstring-diff são cegos a duplicação perfeitamente sobreposta. Vale
considerar, para o handoff, registar isto como uma lacuna de método (mesma família de `L11`):
atestação visual/pixel não é suficiente para provar ausência de trabalho redundante no content
stream — precisa de contagem de operador quando o sintoma original veio de contagem de operador
(como aconteceu aqui, com o achado original de P945 vindo de texto extraído, não de pixel).
