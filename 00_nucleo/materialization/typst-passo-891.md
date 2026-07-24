# Passo 891 — espaçamento anómalo em `04-math`: sub-índice (`i=0`) e expoente (`i²`)

**Precede este passo**: `typst-passo-889-relatorio.md`, secção 1.2–1.4. Ler antes de começar.

**Duas correcções independentes** (mecanismos de código diferentes, `spacing.rs` vs `attach.rs` —
confirmado por P889 secção 3). Podem ser feitas neste passo em sequência ou em dois passos
separados, à discrição de quem executar — mas não misturar os diffs de forma que fique difícil
distinguir qual mudança corrigiu qual sintoma.

**Pré-condição de árvore**: confirmar `git status` antes de começar. Se `typst-passo-890` (fallback
de fonte) ainda estiver em curso ou não commitado, decidir e registar como lidar com isso — as duas
áreas de código não se sobrepõem (`spacing.rs`/`attach.rs` vs `shaper.rs`/`font_metrics.rs`), mas
convém não medir benchmark de um em cima de working tree suja do outro.

---

## Parte A — `i=0`: thick space em torno de `=` dentro de sub-índice (causa confirmada, alta confiança)

### Sintoma

`sum_(i=0)^n`: vanilla renderiza `i=0` colado; cristalino renderiza `i  = 0`, com espaço grosso dos
dois lados de `=`.

### Causa (já confirmada por P889, não é para redescobrir)

`01_core/src/engine/math/layout/spacing.rs`, função `compute_gaps`/`spacing_between`
(linhas ~91-121): a regra `(Relation, _) => THICK` / `(_, Relation) => THICK` insere espaço grosso
em torno de qualquer `MathClass::Relation` (incluindo `=`), **sem excepção para conteúdo em script
size** (dentro de um sub-índice/super-índice, onde o vanilla suprime ou reduz este espaçamento). O
próprio cabeçalho do ficheiro já documenta este scope-out (linhas 9-14, "fora de escopo, registado,
não silencioso — a condição 'unless in script size' de cada regra vanilla").

### Fase A — Confirmar o comportamento exacto do vanilla antes de replicar

1. Ler a regra equivalente no vanilla (`lab/typst-original/`, módulo de espaçamento matemático) e
   confirmar **exactamente** a condição "unless in script size" — é suprimir o espaço por completo
   dentro de script size, ou reduzir para um valor menor (não necessariamente zero)? Não presumir
   "zero" sem confirmar no código-fonte.
2. Confirmar como o layouter do cristalino sabe, no ponto onde `compute_gaps`/`spacing_between` é
   chamado, se o conteúdo actual está em script size (dentro de um `MathAttach`) ou não — se essa
   informação já está disponível ali (via algum parâmetro de estilo/contexto) ou se precisa de ser
   passada de novo desde `attach.rs`.

### Fase B — Implementação (TDD)

1. Teste que falhe primeiro: `spacing_between`/`compute_gaps` com um contexto marcado como script
   size deve devolver o valor correcto confirmado na Fase A (não `THICK`) para o par
   `(Alphabetic, Relation)`/`(Relation, Normal)` (ou os pares exactos envolvidos em `i=0`).
2. Implementar a condição.
3. Suíte verde, discriminada por crate.
4. Recompilar `04-math.typ` e confirmar visualmente que `i=0` aparece colado (ou com o espaçamento
   correcto confirmado na Fase A) dentro do sub-índice, **sem** alterar o espaçamento de `=` fora de
   script size noutros testes existentes (por exemplo, uma equação simples `a = b` a nível normal
   deve continuar com thick space em volta do `=`).

---

## Parte B — `i²`: gap antes do expoente (causa candidata, precisa de instrumentação antes de corrigir)

### Sintoma

`i^2` (base `i`, `MathAttach` com `sup: 2`): vanilla renderiza `i²` colado; cristalino renderiza
`i  ²`, com gap antes do expoente.

### Hipótese de P889, não confirmada

`attach.rs` (~linhas 219-227) posiciona o expoente via `base_kern.top_right`, que vem de
`self.metrics.math_kern(c)` (`03_infra/src/font_metrics.rs:349-374`) — kerning lido da tabela
OpenType MATH da face activa para o glifo base. Se este lookup falhar silenciosamente, `base_kern`
fica `MathGlyphKern::default()` (kern zero), sem a aproximação que normalmente encaixa o expoente
mais perto de uma base itálica inclinada. P889 confirmou que a base `𝑖` (U+1D456) chega convertida
correctamente e que a fonte primária tem esse glifo — mas **não instrumentou `math_kern()` para
confirmar se retorna default() neste caso específico**.

### Fase A — Confirmar antes de implementar (não pular esta parte)

1. Instrumentar (temporariamente) `math_kern()` para imprimir o valor devolvido para o glifo `𝑖`
   nesta chamada específica, comparando com uma inspecção directa da tabela MATH de
   `NewComputerModernMath-Regular.otf` para esse glifo via `fonttools`/`ttx` (mesma ferramenta já
   usada em P889 secção 2.4).
2. Confirmar uma das duas:
   - `math_kern()` devolve zero/default quando não deveria — localizar por que o lookup falha
     (glifo não encontrado na face efectivamente activa nesse ponto — que pode não ser a mesma face
     "primária" assumida, se P890 ainda não tiver sido aplicado; considerar rodar este passo depois
     de P890 para eliminar essa variável), ou
   - `math_kern()` devolve o valor certo, e o gap vem de outro lugar em `attach.rs` (posicionamento
     do expoente soma algo além do kern, ou aplica o kern com sinal/eixo errado).
3. **Reverter a instrumentação antes de avançar para a implementação.**

### Fase B — Implementação (TDD)

Só depois da Fase A confirmar a causa exacta — não implementar uma correcção especulativa em cima
de uma hipótese não confirmada.

1. Teste que falhe primeiro, ao nível apropriado à causa confirmada (lookup de `math_kern` ou
   cálculo de posição em `attach.rs`).
2. Implementar.
3. Suíte verde, discriminada por crate.
4. Recompilar `04-math.typ` e confirmar visualmente que `i²` aparece colado, sem regressão no
   posicionamento de outros expoentes/sub-índices já testados (procurar testes existentes de
   `MathAttach` — `tests.rs:495+`, mencionado no `DEBT.md`, se ainda for o ficheiro certo).

---

## Fase C — Regressão (ambas as partes)

`spacing.rs` e `attach.rs` são código de layout matemático puro (L1), não deviam afectar tempo de
compilação de forma mensurável — mas correr o benchmark completo dos 7 cenários de qualquer forma,
por disciplina (Regra 4 do handoff pós-P884), com atenção a `04-math` (se P890 já tiver sido
aplicado antes deste passo, confirmar que o tempo continua bom depois desta correcção também).

## Resultado esperado

- Header de linhagem actualizado nos ficheiros tocados (`spacing.rs`, e `attach.rs`/`font_metrics.rs`
  se a Parte B confirmar causa lá).
- Instrumentação temporária da Parte B removida antes do commit final.
- Testes novos para as duas partes, cobrindo os casos corrigidos sem quebrar espaçamento/kerning já
  testado noutros contextos.
- Relatório com: confirmação do comportamento exacto do vanilla (Parte A, Fase A), causa exacta
  confirmada por instrumentação (Parte B, Fase A), e resultado visual final das duas.
