# Passo 946 — `binom(n,k)` com delimitador duplicado por linha; chave de `cases()` com glifo errado no gancho inferior

**Precede este passo**: `typst-passo-945-relatorio.md`, seção 9 (`review.typ`, imagens
`review-crys.png`/`review-vanilla.png`) — usadas pelo próprio P945 como prova de sucesso, mas
comparação direta pelo dono mostra **dois defeitos novos, distintos do que P945 corrigiu**:

1. **`binom(n, k)`**: cristalino mostra `(n)` e `(k)` como **dois delimitadores separados**, um
   por linha. Vanilla mostra **um único delimitador** envolvendo as duas linhas como uma coluna.
2. **Chave de `cases()`**: o gancho inferior da chave (`{`) no cristalino parece renderizar como
   um glifo diferente (formato de colchete, não de chave) — a peça de assembly do fundo está
   provavelmente mapeada para o codepoint errado.

**Pré-condição de árvore**: `git status`. Confirmar P945 commitado e presente.

---

## Fase A — `binom`: confirmar por que cada linha ganha o próprio delimitador

1. Localizar a implementação de `binom` (`eval/math.rs` ou onde as funções nativas de P899
   foram implementadas) — confirmar exatamente como ela constrói o conteúdo: se monta uma grade
   de 2 linhas (`n` sobre `k`) e envolve com **um** `Content::MathDelimited`, ou se (bug
   suspeito) envolve `n` e `k` **cada um individualmente** antes de compor.
2. Comparar com a implementação do vanilla (`lab/typst-original/`, `binom`/`vec`-like helpers) —
   confirmar a estrutura esperada.
3. Confirmar se este é um bug isolado de `binom`, ou se `vec(...)` de um único elemento por linha
   (ou outras construções que montam coluna + delimitador) têm o mesmo problema — grep por outros
   consumidores do mesmo padrão.

## Fase B — chave de `cases()`: confirmar o mapeamento do gancho inferior

1. Reproduzir isoladamente `cases(0, x^2, 1)` (3 ramos, o mesmo caso que expôs o problema) e
   confirmar via `mutool trace`/`fontTools` qual glyph ID está sendo desenhado para a peça do
   fundo do assembly da chave — comparar com o glyph ID que a tabela `MathVariants`/assembly da
   fonte (`NewCMMath`) associa à peça inferior de `{` (`uni23A9` ou equivalente, confirmar o
   nome real via `fontTools`).
2. Confirmar se o bug é: mapeamento errado no código do cristalino (pegando o glyph ID de outra
   peça, ou de outro delimitador tipo `[`/`⌊` por engano), ou se é a extração de `GlyphAssembly`
   (`assembly.rs`) devolvendo a peça errada para este símbolo específico.
3. Confirmar se o mesmo problema afeta outros delimitadores com gancho (não retos) — `⌊⌋`/`⌈⌉`
   não têm esse formato, mas `{`/`}` sim; confirmar se há mais algum símbolo na mesma família.

## Fase C — Implementação (protocolo de dois agentes de P898 para o item 1 se envolver mudança de
estrutura de `Content`; TDD directo para o item 2 se for mapeamento pontual de glyph)

1. Testes com contagem/identidade de glifo (mesmo padrão de P945 — `mutool trace`, não só
   "compila") para os dois casos.
2. Implementar as duas correções (podem ser passos separados internamente, mas fecham juntos
   dado que vieram do mesmo `review.typ`).
3. Suíte completa verde, discriminada por crate.
4. `cargo run -- .` — zero violations.

## Fase D — Revalidação (mesma disciplina de P944/945)

1. Recompilar `review.typ` (o mesmo ficheiro usado pelo dono) e comparar lado a lado com
   `review-vanilla.png` de novo — confirmar pixel a pixel onde possível, não só "parece melhor".
2. Recompilar o `.typ` de 30 seções inteiro, revalidação visual completa, comparar com o render
   pós-P945 (nenhuma regressão nova) e com o vanilla real.
3. Benchmark completo, 7 cenários canônicos, `depois/antes`, zero regressão.

## Resultado esperado

- `binom` (e qualquer outro consumidor do mesmo padrão) envolvido por um único delimitador, não
  um por linha.
- Chave de `cases()` com o glyph correto no gancho inferior, confirmado por `fontTools`/`mutool
  trace`, não só inspeção visual.
- `review.typ` e o documento de 30 seções revalidados, com comparação explícita (não só
  qualitativa) contra o vanilla.
- Benchmark completo, zero regressão.

---

## Nota de processo

Isto é a segunda vez que uma revisão "visualmente próxima do vanilla" (P945, seção 9) deixou
passar defeitos reais que só apareceram quando o dono olhou a mesma imagem com atenção. Vale
considerar, no handoff, formalizar que "revisão do orquestrador" para geometria matemática exige
comparação explícita elemento a elemento (contagem de glifo, posição), não impressão visual geral
— mesmo padrão que `L11` já pede para benchmarks, aplicado também a comparação visual.
