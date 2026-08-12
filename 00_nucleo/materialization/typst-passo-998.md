# Passo 998 — Corpus de testes `.typ` derivado da documentação oficial (arranque: `reference/math/`)

**Tipo**: Construção de infra-estrutura de teste, não correcção de bug. Sem gate ADR-0127
(não toca código de produção, só adiciona ficheiros `.typ` e o mecanismo de comparação
sobre eles).
**Motivo**: dois erros recentes (P42/P51 nunca cruzados; investigação paralela que assumiu
comportamento por analogia com `binom()`) tinham as duas causas na mesma raiz — decisão
tomada sem consultar `typst.app/docs/` primeiro. Este corpus fecha essa lacuna de forma
permanente e sistemática, cobrindo tudo o que está documentado, não só o que alguém pensou
em testar.
**Fonte única de verdade**: `https://typst.app/docs/reference/math/` e as suas 18
subpáginas. Nenhum caso deste corpus deve vir de memória — todos citam a frase exacta da
documentação que motivou o `.typ`.

---

## Estrutura proposta

Um directório `00_nucleo/corpus-docs/math/`, um ficheiro `.typ` por página da
documentação — a estrutura espelha `typst.app/docs/reference/math/`, para que uma
actualização da documentação tenha correspondência directa a um ficheiro:

```
00_nucleo/corpus-docs/math/
  00-index.typ              # Variables, Symbols, Line Breaks, Function calls,
                             # Alignment, Math fonts, Accessibility (secções da
                             # própria página /math/, sem sub-página própria)
  accent.typ
  attach.typ
  binom.typ
  cancel.typ
  cases.typ
  class.typ
  equation.typ
  frac.typ
  lr.typ
  mat.typ
  primes.typ
  roots.typ
  sizes.typ
  stretch.typ
  styles.typ
  op.typ
  underover.typ
  variants.typ
  vec.typ
```

Cada ficheiro segue o formato: um bloco de comentário por caso, citando a frase da
documentação, seguido do `.typ` mínimo que a testa.

```typst
// Fonte: https://typst.app/docs/reference/math/#line-breaks
// Citação: "Formulas can also contain line breaks. Each line can contain one or
// multiple alignment points (&) which are then aligned."
$ sum_(k=0)^n k
    &= 1 + ... + n \
    &= (n(n+1)) / 2 $
```

Cada caso deve, sempre que possível, ser o exemplo literal da documentação (copiado
exactamente do bloco "Copy" da página) — não uma reformulação. Onde a prosa da
documentação implica um caso que não tem exemplo de código ao lado, escrever o `.typ`
próprio e marcar `// Caso derivado da prosa, sem exemplo na doc:` antes da citação.

## Casos já identificados nesta pesquisa (não esperar pelo passo de execução para os
perder de vista — registar aqui)

### `00-index.typ`

1. **Line Breaks** — o exemplo literal da documentação:
   ```
   $ sum_(k=0)^n k &= 1 + ... + n \ &= (n(n+1)) / 2 $
   ```
   Cobre: `\` como quebra de linha + `&` como ponto de alinhamento, no mesmo caso. Este é
   o tipo exacto de caso que, se existisse no corpus antes de Agosto, teria apanhado o bug
   de P991-996 no dia em que o corpus fosse comparado contra o vanilla pela primeira vez.

2. **Alignment** — exemplo com `&&` (dois pontos de alinhamento seguidos, alterna
   alinhamento duas vezes) e `&` simples — confirmar que o cristalino distingue os dois
   casos (o exemplo da documentação tem os dois no mesmo bloco).

3. **Variables** — símbolo de letra única vs. múltiplas letras (interpretadas como
   variável/função) vs. texto entre aspas — três comportamentos distintos, confirmar os
   três.

4. **Function calls** — escapar vírgula/ponto-e-vírgula com `\` dentro de uma chamada
   math — **terceiro uso distinto de `\`** (além de quebra de linha e de impedir
   emparelhamento de delimitador). Confirmar que os três não se confundem no lexer.

### `lr.typ`

5. **Escapar delimitador com `\` para impedir emparelhamento automático** — citação
   directa: "To prevent a delimiter from being matched by Typst, and thus auto-scaled,
   escape it with a backslash." Exemplo literal: `$ \{ (x / y) \} $`. **Confirmar
   explicitamente que isto não se confunde com o `\` de quebra de linha** (achado da
   pesquisa desta conversa — os dois mecanismos partilham o caractere inicial, nunca
   catalogados lado a lado antes).
6. `lr(size:)` com percentagem — exemplo `lr(]sum_(x=1)^n], size: #50%)`.
7. `#set math.lr(size: 1em)` para desactivar auto-scaling completamente.

### `equation.typ`

8. Equação inline vs. bloco (espaço a seguir a `$`/antes de `$` de fecho decide).
9. `alt:` para acessibilidade — confirmar que o cristalino aceita/preserva o parâmetro
   mesmo que não afecte o PDF visualmente (é metadado).

---

## Plano de execução

1. **Fase A**: para cada uma das 19 páginas, buscar o conteúdo real (`web_fetch`, não
   memória) e extrair todos os blocos "Copy" (exemplos de código) + qualquer comportamento
   descrito em prosa sem exemplo. Construir a lista completa de casos antes de escrever
   qualquer `.typ` — evitar começar a escrever e descobrir a meio que faltou reler uma
   página.
2. **Fase B**: escrever os 19 ficheiros `.typ`, cada caso com citação.
3. **Fase C**: compilar todo o corpus com o cristalino actual e com o vanilla de
   referência (baseline ratificado). Usar `compare.py` (já existe, P948) — não construir
   ferramenta nova.
4. **Fase D**: para cada divergência encontrada, **não corrigir dentro deste passo** —
   catalogar como achado, um por um, com o mesmo rigor de causa-raiz que já é praxis nesta
   frente (ADR-0084, `file:line`). Este passo produz uma lista de achados para passos de
   correcção futuros, não fixes directos — é auditoria, não implementação.

## Resultado esperado

- 19 ficheiros `.typ` em `00_nucleo/corpus-docs/math/`, cobrindo sistematicamente a secção
  de matemática da documentação oficial.
- Um relatório com a lista de divergências encontradas (se houver), cada uma com
  `file:line` da causa quando identificável rapidamente, ou marcada para investigação
  própria quando não.
- Este corpus fica disponível para ser corrido contra qualquer commit futuro — não é um
  teste único, é infra-estrutura permanente.

## Fora de âmbito deste passo (registar, não fazer aqui)

- As outras secções da documentação (`layout/`, `text/`, `model/`, etc.) — começar por
  `math/` porque é onde os erros recentes aconteceram; expandir depois, um passo por
  secção, mesma estrutura.
- A ideia mencionada pelo dono de rever prompts existentes contra a documentação — fica
  como frente separada, possivelmente mais dirigida (só prompts que decidiram
  comportamento de linguagem sem citar documentação), depois deste corpus existir.
