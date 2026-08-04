# Passo 957 — peça inferior do assembly de delimitador usa glifo errado (canto reto em vez da forma correta)

**Precede este passo**: confirmação visual do dono — três delimitadores diferentes (chave `{}`,
parêntese `()` em matriz aumentada, parêntese `()` em matriz de reticências), todos de 3+ linhas
(passando pelo caminho de assembly). Em todos os três, **a peça de cima tem a forma correta**
(gancho de chave curvo, curva redonda de parêntese) e **a peça de baixo, nos três casos, sai com a
mesma forma errada** — canto reto, tipo colchete (`⌊`-like), não combinando com o tipo do
delimitador. Padrão sistemático, não coincidência: mesma forma errada em três tipos diferentes de
delimitador.

**Hipótese a confirmar, não fato**: a seleção da peça inferior do assembly está a resolver para o
glifo errado — possivelmente um fallback/glifo por defeito partilhado por engano entre tipos de
delimitador diferentes, em vez do glifo específico do gancho inferior de cada um.

**Contexto**: isto pode ter ficado escondido até P945 corrigir o dimensionamento da grade (antes
disso, estas matrizes podiam não estar a disparar o caminho de assembly correctamente) — não
presumir isto como fato, confirmar.

**Pré-condição de árvore**: `git status`. Confirmar P945-956 presentes.

---

## Fase A — confirmar a causa exacta, glifo a glifo

1. Reproduzir isoladamente os três casos (`cases()`/chave de 3 ramos; `mat(...)` com `augment`;
   matriz de reticências 4×4) e extrair, via `mutool trace`/`fontTools`, o glyph ID exacto usado
   para a peça inferior de cada delimitador nos três casos.
2. Comparar esses glyph IDs com os glyph IDs corretos esperados — confirmar na tabela
   `MathGlyphAssembly` da fonte (`NewCMMath`, `fontTools`) qual é o glifo correto do gancho
   inferior para `{`, `(`, e confirmar se os três casos do documento estão a receber o mesmo
   glyph ID (confirmando fallback partilhado) ou glyph IDs diferentes mas todos errados
   (apontando para outra causa, como índice errado dentro da lista de peças).
3. Ler o código que selecciona a peça inferior do assembly (`assembly.rs`, `layout_assembly`,
   provavelmente o mesmo código que P906/913/949 já tocaram) — confirmar exactamente onde a
   peça de índice "gancho inferior" é escolhida, e se há alguma condição que a substitui
   incorrectamente por outra.
4. Confirmar se o vanilla, para os mesmos três casos, usa glifos diferentes por tipo de
   delimitador (esperado) — mesmo método de comparação já usado em P946/949.

## Fase B — Implementação (TDD directo se for correcção pontual de índice/mapeamento; protocolo
de dois agentes se envolver mudança mais ampla na lógica de assembly)

1. Teste com identidade de glifo (não só posição) confirmando o glyph ID correto da peça
   inferior para pelo menos dois tipos de delimitador diferentes (chave e parêntese) — mesmo
   padrão de P946 (charstring/glyph ID como prova, não só posição).
2. Implementar a correcção.
3. Suíte completa verde, discriminada por crate.
4. Confirmação visual (`mutool draw`, alta resolução) dos três casos originais, e de pelo menos
   mais um tipo de delimitador não testado ainda (colchete `[]`, se houver caso de 3+ linhas no
   documento de 30 secções) para confirmar que a correcção generaliza.
5. `cargo run -- .` — zero violations.

## Fase C — Revalidação

1. Recompilar o `.typ` de 30 secções inteiro, revalidação visual completa (mesma disciplina de
   P944-947), confirmando que a correcção não introduz regressão noutro delimitador.
2. `compare.py` (P948/951/956) para confirmar geometria, mais confirmação manual de glifo
   (`fontTools`) para a identidade — a ferramenta mede posição, não confirma forma/identidade do
   glifo sozinha (lição de P949, já registada no README).
3. Benchmark completo, 7 cenários canónicos, `depois/antes`, zero regressão.

## Resultado esperado

- Causa exacta confirmada: fallback partilhado, índice errado, ou outra causa — com glyph IDs
  reais, não suposição.
- Peça inferior do assembly corrigida para todos os tipos de delimitador testados.
- Confirmação visual e de identidade de glifo, não só posição.
- Revalidação completa das 30 secções, benchmark sem regressão.
