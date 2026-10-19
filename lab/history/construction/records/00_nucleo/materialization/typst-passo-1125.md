# L0 — Passo 1125: Inspecção Visual (Secções 7/9) e Padrão `log_a` com Subscrito

**Gate**: nenhum para a inspecção em si; correcções que resultarem seguem
`ADR-0127`.

**Base**: verificação cruzada nos arquivos comprehensive/extended-test.
Secção 27 confirmada como única causa da deriva de largura em todo o
documento (consistente com P1124 §2, já em correcção). Numeração de
equações confirmada correcta (falso alarme, correctamente descartado pela
própria nota). Cinco pontos de salto ainda não catalogados: `6→7`,
`7→8`, `9→10`, `14→15`, `22→23`. Padrão `log_a`/`log_x` com subscrito
confirmado generalizar além do `log_2` já catalogado na secção 29.

---

## 1. Inspecção visual das secções 7 e 9 — prioridade, per a própria nota

**Não usar comparação automática por texto nestas duas** — já confirmado
que a codificação de glifo diverge (`cid` no vanilla vs Unicode directo
no cristalino para `ℵ`/`ℶ`/coeficiente binomial na secção 7; delimitador
de `cases` na secção 9), invalidando o pareamento por `difflib`.

Renderizar (`pdftoppm`, mesma resolução já usada nesta investigação,
150-300dpi) e comparar visualmente as duas secções, nos dois arquivos.
Não converter para métrica numérica sem antes confirmar visualmente que
há de facto divergência — pode ser só diferença de codificação de fonte
sem diferença de posição real (glifo diferente, mesmo lugar).

## 2. Padrão `log_a`/`log_x` — mesma família do `log_2`, investigar juntos

**Base**: secção 29 já catalogou `log_2` com subscrito `1.75pt` baixo
demais (baixa prioridade, isolado). Este achado confirma o mesmo padrão
em `log_a` (secção 6), mesma magnitude — não é caso único, é qualquer
`log` com subscrito de base.

**Não presumir se é a mesma família do "espaço entre blocos"** (P1122/
P1123) ou categoria própria — per o próprio pedido da nota. Testar:

1. `log_a x` isolado, sem mais nada na linha — o subscrito da base de
   `log` diverge sozinho, ou só diverge quando há mais conteúdo depois
   na mesma linha/bloco (como os casos de "espaço entre blocos" que
   crescem com o número de elementos)?
2. Se isolado já diverge por `1.75pt` fixo (não crescente), é mais
   provável ser causa própria — específica ao layout de `log` com
   subscrito, não ao mecanismo de colapso entre blocos.

**Ler o código real**: onde `log` (com subscrito de base, distinto de
`ln`/`exp` que não têm) é layoutado — provavelmente em `math/layout/
mod.rs` ou `attach.rs`, caminho de operador nomeado com subscrito anexo,
não sintaxe genérica `x_y`.

## 3. Saltos ainda não catalogados — investigar cada um antes de presumir causa

Não presumir que os 5 saltos (`6→7`, `7→8`, `9→10`, `14→15`, `22→23`)
partilham causa — a nota já atribui `6→7` ao achado do `log_a` (§2), o que
faz sentido dado que a secção 6 é "Funções Especiais". Os outros 4 ainda
precisam de identificação:

- `7→8`: pode estar ligado à mesma questão de codificação de glifo da
  secção 7 (§1) — confirmar depois da inspecção visual, não antes.
- `9→10`: maior salto (`-6.10pt`), secção 9 é `cases`/função por partes
  — também pode estar ligado à codificação divergente (§1); ou pode ser
  causa geométrica própria de `cases()`, mecanismo já tocado nesta
  investigação para outros delimitadores (P906, esticamento de glifo).
- `14→15`: secção 14, Números Complexos — sem hipótese ainda, investigar
  do zero.
- `22→23`: secção 22, Delimitadores Escaláveis — candidato a mesma
  família de bugs de delimitador já visto (P906, `lr()`, secção 37
  `mat()` já corrigida em P1121) — confirmar, não presumir.

## Critérios de verificação

1. §1: secções 7 e 9 inspeccionadas visualmente, divergência real
   confirmada ou descartada (mesmo tratamento do falso alarme de
   numeração, já correctamente resolvido pela própria nota).
2. §2: `log_a`/`log_x` isolado testado, causa (própria ou família de
   bloco) determinada com código real.
3. §3: os 4 saltos restantes (`7→8`, `9→10`, `14→15`, `22→23`)
   identificados individualmente, não presumidos como uma coisa só.

## Critério de conclusão

- §1 executado antes de qualquer conclusão numérica sobre secções 7/9.
- §2 resolvido com teste isolado, não presumido família de bloco.
- Secções 1-5, 10-13, 16, 17, 21, 24, 25 — confirmado sem pendência de
  espaçamento vertical, per a própria nota; não reinvestigar sem motivo
  novo.
