# L0 — Passo 1127: Reabrir `stack(dir:ttb)` e Investigar Causa Única do Padrão Recorrente

**Gate**: `ADR-0127` se confirmar necessidade de correcção. Parte
investigação, parte reabertura de correcção já dada como fechada
incorrectamente.

**Base**: P1126 `§11` afirmou ter "calibrado" o espaçamento de
`stack(dir:ttb)`, com resíduo de `0.12pt`. Duas notas independentes desde
então (secção 43 do `extended-test`, secção 9 do documento completo)
medem exactamente esse mesmo `0.12pt`/`3.01pt` como **não resolvido**,
tratando-o como mais um gatilho do padrão recorrente já catalogado.
`§11` nunca passou por um L0 revisto individualmente — apareceu junto de
um relatório já rejeitado por sair do âmbito do passo em curso.

Neste ponto, o padrão recorrente já tem **5 gatilhos confirmados**:
`text(size:)` (secções 35, 38), `display()`/`script()`/`sscript()`
(secção 19, 35), `box()` (secção 40), `stack()` (secção 43, agora),
`cases()` (secção 9, agora). Isto é evidência suficiente para investigar
como **mecanismo único**, não continuar a corrigir gatilho a gatilho.

---

## 1. Reabrir `§11` — `stack(dir:ttb)` não foi corrigido de facto

**Não presumir que o "fix" do P1126 fez alguma coisa** — comparar
directamente o código antes e depois da alteração descrita (`helpers.rs`,
detecção de caracteres com descendente; `stack.rs`, avanço
`descent_i+spacing+ascent_i+1`). Se os números não mudaram
(`14.35→14.23` continua igual antes e depois), confirmar se a alteração
foi mesmo aplicada ao caminho de código que gera este resultado, ou se
foi aplicada a outro lugar sem efeito aqui.

## 2. Hipótese de mecanismo único — não presumir sem testar

Com 5 gatilhos confirmados, a pergunta central: existe um único ponto no
código onde todos passam (ex.: todos usam `layout_sub_frame` ou
`layout_external`, já extensamente tocados nesta investigação), e o
resíduo pequeno e consistente (`~0.1-0.6pt`, por vezes crescendo, por
vezes encolhendo consoante o caso — já visto bidireccional na secção 35)
vem de uma única fonte, ou os 5 gatilhos têm 5 causas independentes que
só parecem parecidas por produzirem sintoma semelhante (deslocamento
pequeno entre blocos)?

**Testar antes de escolher hipótese**: para cada um dos 5 gatilhos,
confirmar se o caminho de código que calcula `ascent`/`descent`/`gap`
resultante é o mesmo arquivo/função, ou se são caminhos distintos que
só coincidem em sintoma. Se for a mesma função (ex.: `layout_external`),
a correcção correcta é uma só, nesse ponto; se forem caminhos diferentes,
não faz sentido tentar unificar — continuar gatilho a gatilho, mas com
essa conclusão registada explicitamente (não presumida).

## 3. Secção 9 (documento completo) — `cases()` como gatilho, não confundir com o `|`

**Já reconciliado nesta conversa**: a secção 9 tem duas causas
independentes — o `|` (já corrigido, P1126 `§5`) afecta o espaçamento
**dentro** da segunda equação (`|x|=...`); a decomposição de `-6.02pt`
em 3 gaps (título→eq1, eq1→eq2, eq2→título seguinte) é sobre o espaço
**ao redor** do bloco `cases()` inteiro — mesma família dos outros 4
gatilhos, não o mesmo bug do `|`. Confirmar que a correcção de `§5`
(fence) não afecta nem resolve estes 3 gaps — são causas distintas,
tratadas em paralelo, não sequencialmente uma dependente da outra.

## 4. Não fechar §10 (integral verde) sem verificação própria

`§10` também nunca teve L0 revisto individualmente — mas os números
apresentados (`Δ=0.0000pt`, coordenadas idênticas a 2 casas decimais)
parecem correctos e não têm o mesmo tipo de contradição que `§11`.
Não reabrir a não ser que apareça evidência contraditória (mesmo padrão
usado para `§7` — não reabrir sem motivo, mas não presumir correcto só
porque ninguém contestou ainda).

## Critérios de verificação

1. §1: `stack(dir:ttb)` genuinamente convergindo para `0.0000pt`
   (±0.0005pt), código real citado, não "calibrado dentro de 0.12pt".
2. §2: veredicto claro — causa única confirmada (com o ponto exacto no
   código) ou causas múltiplas confirmadas como tal, não presumido.
3. §3: os 3 gaps da secção 9 convergindo, sem misturar com a correcção
   do `|`.
4. Re-rodar P1086-1126 — zero regressão.
5. `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- §1 fechado com número real, não presumido do relatório anterior.
- §2 respondido com veredicto claro sobre causa única vs múltipla.
- Nenhum item novo (`§10`/`§11`) tratado como fechado sem verificação
  própria, mesmo que já apareça em relatório anterior.
