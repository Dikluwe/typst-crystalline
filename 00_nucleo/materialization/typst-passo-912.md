# Passo 912 — Achado A de P911: `covering()` nunca prefere a fonte MATH para glifos comuns; três desvios de fórmula secundários

**Precede este passo**: `typst-passo-911-relatorio.md`, "Achado transversal A" e sua subsecção
"Achado secundário no mesmo mecanismo". Ler antes de começar — a causa já está isolada por
medição real (`eprintln!` + `mutool trace`, revertido), este passo é sobre corrigir, não
redescobrir.

**Antes de escrever qualquer código — decisão de produto a levar ao dono**: este é o quarto
consumidor (`math_kern` P890, generalização P891, `math_constants` P893, agora
`vertical_glyph_variants`/`vertical_glyph_assembly`) a precisar da mesma classe de correção em
"preferir fonte MATH sobre fonte de corpo". Perguntar explicitamente: corrigir só este consumidor
(mesmo padrão pontual dos três anteriores), ou investir agora numa correção única em `covering()`
que resolva a preferência de fonte MATH **de uma vez, na raiz**, para qualquer consumidor presente
ou futuro? Registar a decisão antes da Fase B — não decidir sozinho.

**Pré-condição de árvore**: `git status`. Confirmar estado de `P909` (atomização) commitado —
este passo toca `stretchy.rs`/`delimited.rs`/`matrix.rs`/`cases.rs`/`font_metrics.rs`, arquivos que
P909 não tocou, mas confirmar de qualquer forma.

---

## Achado A — causa raiz (não redescobrir, só corrigir)

`03_infra/src/font_metrics.rs:867-905`, `fn covering`: itera `primary` na ordem em que
`resolve_primary_with_math_fallback` a construiu — fonte de corpo primeiro, fonte MATH só
`push`ada depois (linha 852) — e devolve a **primeira** face cujo `glyph_index(c)` exista (linha
875). Glifos comuns (`(`, `)`, `{`, `[`, `]`) existem em qualquer fonte de corpo, logo `covering()`
acerta a fonte de corpo (sem tabela MATH) antes de tentar a MATH. `extract_variants`/
`extract_assembly` devolvem `default()` (listas vazias) assim que `face.tables().math` é `None`.

## Fase A — confirmar o desenho da correção (a decisão de produto acima já deve estar registada)

1. Se a decisão for "corrigir na raiz": desenhar a mudança em `covering()` — quando `style.math` é
   verdadeiro, dar prioridade a qualquer face **com tabela MATH** na lista `primary` sobre faces sem
   essa tabela, independentemente da ordem em que foram inseridas, em vez de "primeira que cobre o
   carácter". Confirmar que isto não regride os casos já corrigidos (`math_kern`, `math_constants`)
   — esses já dependem de um comportamento parecido, confirmar que não há conflito.
2. Se a decisão for "corrigir só este consumidor": desenhar uma verificação local em
   `extract_variants`/`extract_assembly` (ou no ponto que os chama) que tente explicitamente a
   fonte MATH antes de aceitar o resultado de `covering()` genérico.
3. Confirmar os três desvios de fórmula secundários (já medidos por P911, `file:line` dos dois
   lados no relatório) e desenhar a correção de cada:
   - `delimited.rs:26-31` — usar `2.0 * (ascent - axis).max(descent + axis)` quando `balanced`
     (sempre verdadeiro para `MathDelimited`/`lr()` de utilizador, confirmar isso antes de
     hardcode), em vez de soma simples.
   - `matrix.rs:94-99`/`cases.rs:28-33` — aplicar `target = 1.1 × relative_to` (`balanced=false`
     nestes dois, soma simples continua correta nesse aspecto — só falta a margem de 10%).
   - `stretchy.rs`/`glyph_variants.rs::select_with_advance` — subtrair `DELIM_SHORT_FALL = 0.1em`
     do alvo antes de comparar.

## Fase B — Implementação (protocolo de dois agentes de P898 — é geometria com efeito em cascata
sobre 3+ módulos, risco maior que a média desta frente)

1. Agente A escreve testes **com métricas reais, não `FixedMetrics`** — o próprio P911 já
   confirmou que este é o gap de cobertura que permitiu o achado sobreviver. Usar o mesmo padrão de
   dados de fonte real já usado em P890-893 (`fontTools`/`ttf_parser` para confirmar valores
   esperados, não hardcoded). Confirma vermelho.
2. Agente B implementa conforme o desenho escolhido na Fase A (raiz ou pontual).
3. Revisão do orquestrador — mesmo padrão de P898/901/906: testar pelo menos um caso que não seja
   `(`/`)` simples (por exemplo, um delimitador que já tinha variantes correctas antes, tipo `√` já
   corrigido por P906, para confirmar que a mudança em `covering()` não regride esse caso).
4. Suíte completa verde, discriminada por crate.
5. Recompilar `mat(...)`/`cases(...)`/`lr(...)` com conteúdo alto e confirmar visualmente/via
   `mutool trace` que o glifo do delimitador agora **cresce** com o conteúdo, igual ao vanilla —
   mesma tabela de casos que P911 já mediu (`(1/2)` vs `(1/2/3/4)` vs 8 linhas), confirmando que os
   glifos deixam de ficar "inalterados".
6. `cargo run -- .` — zero violations.

## Fase C — Regressão

Esta correção mexe em `covering()`, usado por vários consumidores (`math_kern`, `math_constants`,
agora isto) — benchmark completo, 7 cenários, `--warmup 5 -m 20`, atenção a `04-math` e a qualquer
cenário com delimitadores.

## Resultado esperado

- Decisão de produto registada (raiz vs pontual) antes da Fase B.
- Testes novos com métricas reais (não `FixedMetrics`), fechando o gap de cobertura que P911
  identificou.
- Confirmação visual/geométrica de que os delimitadores escalam com o conteúdo nos três módulos.
- Benchmark completo.
- Nota explícita se a correção também resolveu, de graça, algum resíduo do achado "`√` não escala"
  registado em P905/P906 (mesmo mecanismo, `stretchy.rs`) — confirmar, não presumir.
