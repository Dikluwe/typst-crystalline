# Relatório — Passo 998: corpus de testes `.typ` derivado da documentação oficial (`reference/math/`)

**Tipo**: infra-estrutura de teste — sem fix neste passo (auditoria, não
implementação).
**Estado**: HEAD `957af2575` (pós-P997) + ficheiros novos do corpus.
**Método**: swarm de 19+1 agentes (1 ficheiro por página da documentação),
fetch real de `typst.app/docs/reference/math/` e subpáginas (não memória) —
a página `styles/` tinha sido reorganizada (funções movidas para
`variants/` e `sizes/`), apanhada exactamente porque o fetch foi real.

## Corpus criado — 19 ficheiros em `00_nucleo/corpus-docs/math/`

`00-index, accent, attach, binom, cancel, cases, class, equation, frac, lr,
mat, op, primes, roots, sizes, stretch, styles, underover, variants, vec`
— cada caso com citação verbatim da frase da documentação que o motiva
(literal dos blocos "Copy" sempre que existem; derivados marcados).

**Matriz de compilação** (cristalino `target/release/typst` = HEAD; vanilla
= baseline ratificado main `a51e02804`):

| ficheiro | cristalino | vanilla |
|---|---|---|
| attach, binom, class, op, roots, sizes, styles, variants | OK | OK |
| cases, lr, vec | OK (warnings set) | OK |
| 00-index, accent, cancel, equation, frac, mat, stretch, underover | **FALHA** | OK |
| primes | OK (mais permissivo) | **FALHA** |

## Catálogo de achados (Fase D — para passos de correcção futuros)

### A. Exemplos LITERAIS da documentação que falham no cristalino (prioridade máxima)

1. **`mat.typ:12` — `dots.v`/`dots.down` rejeitados** (`error: unknown
   symbol modifier 'v'`, `eval/bindings.rs:1885`): a tabela de símbolos não
   tem as variantes pontuadas de `dots`. Mesma causa em **`underover.typ:17`
   — `dots.c`**.
2. **`stretch.typ:11` — `harpoons.ltrb` desconhecido** (`unknown variable:
   harpoons`): símbolo ausente da tabela.
3. **`equation.typ:49` — `#math.equation(...)` não é chamável** (`não é
   possível chamar none`): `make_math_module` (`stdlib/mod.rs`) não regista
   `equation` como campo chamável do módulo `math`. Afecta o exemplo
   literal de `alt:`.
4. **`frac.typ:53` — `math.equation.where(...)` falha** (`type none has no
   method 'where'`): mesma raiz do anterior + `.where` sobre elemento.
5. **`accent.typ:24` — `box(width: 0.4em + 0.3em * i)`** rejeitado
   (`espera length, recebeu content`): aritmética `length + length × int`
   dentro de `#for` produz content. A investigar em eval de expressões.
6. **`cancel.typ` — named args rejeitados** (`length`/`inverted`/`cross`/
   `angle`/`stroke`): **scope-out documentado** (P296, ADR-0054 graded,
   `stdlib/structural.rs:2228`) — dívida conhecida, não lacuna nova; 5 dos
   6 casos literais da página dependem dela.

### B. Cristalino MAIS permissivo que o vanilla (erro de linguagem em falta)

7. **`primes(2)` aceite** — vanilla: `error: expected integer, found
   content` (o elemento `primes` só é usável via apóstrofes). Cristalino
   compila.
8. **`limits(A, inline: false)` bare aceite** — vanilla: `error: unknown
   variable `false``. O cristalino aceita por causa do caso especial de
   P992 (bare `true`/`false` como MathIdent). **Medido neste passo**
   (`/tmp/lim-false.typ`, vanilla main: rejeita). Nota: contradiz a
   premissa registada em P992 ("sintaxe vanilla real, sem `#`") — a
   verificar se é drift 0.15.1→main ou erro de leitura original;
   passo próprio deve medir e alinhar (provisoriamente: o cristalino está
   mais permissivo que o vanilla actual).

### C. Set-rules de elementos math ignoradas silenciosamente

9. **`#set math.cases/frac/lr/vec/mat(...)` sem efeito** (warning `set:
   target '' ainda não suportado`, `eval/rules.rs:251` — targets suportados
   são só `heading, page, figure, text, par, table, smartquote`). Os
   parâmetros `delim`/`align`/`gap`/`reverse`/`size` de exemplos LITERAIS
   não têm efeito no PDF — divergência silenciosa em 5 ficheiros do corpus
   (cases, frac, lr, vec, mat).

### D. Módulo `math` incompleto

10. **`math.mat`/`math.root` (e outros construtores) inexistem como campos
    do módulo** (`module 'math' does not contain field "mat"`): o
    constructor bare em math funciona, o acesso namespaced não. Vanilla
    expõe ambos.

### E. Sinal geométrico (compare.py, ficheiros que compilam nos dois)

| ficheiro | glifos > 0.5pt | padrão |
|---|---|---|
| styles | 0/16 | — |
| attach | 9/38 | indet. |
| roots | 14/29 | sistemático |
| binom | 15/32 | sistemático |
| class | 13/23 | sistemático |
| op | 29/82 | sistemático |
| vec | 32/56 | sistemático (reflecte C.9) |
| cases | 35/83 | indet. (C.9) |
| lr | 36/98 | sistemático |
| sizes | 37/54 | indet. |
| variants | 40/81 | sistemático |

O sinal é whole-page (mistura deriva acumulada com diferenças reais de
casos) — serve de triagem, não de atribuição. As divergências accionáveis
são as de A–D.

## Notas

- A página `styles/` foi reorganizada na documentação viva: `serif/sans/
  cal/scr/frak/mono/bb` → `variants/`, `display/inline/script/sscript` →
  `sizes/`. O corpus segue a estrutura NOVA (`styles.typ` só tem
  `upright`/`italic`/`bold`; `variants.typ` foi criado fora da lista
  original do passo, que ainda a reflectia).
- `attach.typ` derivado: `limits(A, inline: false)` foi ajustado para
  `#false` no corpus (para compilar nos dois) — o achado B.8 preserva a
  divergência.
- Regra respeitada: nenhum exemplo literal foi corrigido para passar no
  cristalino; as falhas ficam como achados.

**Commit final**: ver rodapé.
**Commit final**: o commit que introduz este ficheiro e o corpus
(`git log -1 -- 00_nucleo/corpus-docs/`).
