# Problemas encontrados nos passos 385–427

Este documento lista problemas concretos identificados na análise dos passos.
Cada problema tem: onde ocorre, o que está errado, e o que devia ter sido feito.

---

## P393 — `#show regex(...)` — paridade declarada incorretamente

**Problema:** a tabela de paridade no relatório diz "texto com dígitos após `#show regex('\d+'): it => strong(it)` → strong ✓". Isso é enganador. No vanilla Typst, `#show regex('\d+'): it => strong(it)` aplicado a `"abc123def"` produz `"abc"` + `strong("123")` + `"def"` — só os dígitos ficam strong. Na implementação do P393, o nó de texto inteiro `"abc123def"` é transformado se o regex casar, produzindo `strong("abc123def")`.

**Onde:** relatório P393, tabela de paridade, primeira linha.

**O que devia ter sido feito:** a tabela devia ter uma linha separada para o caso `"abc123def"`, com o resultado real do cristalino (`strong("abc123def")`), e o item devia estar marcado como `parcial` ou scope-out no inventário, não como `implementado`.

---

## P394 — Ambiguidade sobre se `apply_func` mudou de assinatura ou não

**Problema:** o relatório diz que `apply_func` recebe `&mut Scopes<'_>` (o que implica que a assinatura mudou e se propagou a todos os callers), mas também diz que a intrusão foi "mecânica". Essas duas afirmações não se reconciliam no texto. Se `apply_func` mudou de assinatura, todos os callers (closures, state/counter callbacks, show-rules) foram afetados, e os que não precisam de scopes passam um scope vazio — o que é um ABI de mentira. Se não mudou, o relatório está mal descrito.

**Onde:** relatório P394, secção "Decisão de engenharia" e "Protocolo de Nucleação cumprido".

**O que devia ter sido feito:** o relatório devia deixar claro qual foi o caminho real: (a) `apply_func` mudou de assinatura e a propagação foi N callers com scope vazio onde não é necessário, ou (b) o dispatch especial ficou interno à função sem mudar a assinatura. Uma das duas. Sem ambiguidade.

---

## P395 — `Fill::Tiling` assumido sem verificar se `Fill` existe como enum

**Problema:** a spec do P395 propõe "adicionar `Fill::Tiling(Tiling)` ao enum `Fill`" e lista como risco "se `Fill` não existe isolado, criar agora". O risco foi identificado mas não foi medido pela sonda antes de escrever a spec. Se `Fill` não existisse como enum separado (por exemplo, se o código usava `Option<Color>` diretamente), o passo M tornava-se um refactor de layout adicional não previsto na estimativa de custo.

**Onde:** spec P395, secção 2.3, tabela de impacto cross-module; secção 7, primeiro risco.

**O que devia ter sido feito:** a sonda de substrato devia ter incluído um grep confirmando que `Fill` existe como enum com `file:line` antes de a spec depender dele. É o mesmo método que a sonda P389 usava para todos os outros substratos.

---

## P389 — Arquivo de output da sonda não está presente

**Problema:** o passo P389 descreve o que deve produzir (`typst-sonda-ausentes-ordem-passo-389.md`). Os passos P390 a P394 referenciam "§2D" desse arquivo como fonte de autoridade para cada materialização. O arquivo não está nos materiais fornecidos. Os passos de materialização citam uma sonda que, do ponto de vista dos arquivos disponíveis, não existe como documento verificável.

**Onde:** passos P390–P394, campo "Sonda fonte" em cada cabeçalho.

**O que devia ter sido feito:** o arquivo `typst-sonda-ausentes-ordem-passo-389.md` devia ter sido incluído ou referenciado de forma que o seu conteúdo fosse verificável. Sem ele, as âncoras `file:line` que os passos citam não podem ser confirmadas.

---

## P388 — Spec escrita antes da sonda responder

**Problema:** o passo P388 diz "abrir com sonda de viabilidade do runtime de introspecção antes de qualquer código". Mas a spec completa — com arquitetura, fases, critérios de aceitação e escopo — foi escrita antes de a sonda responder. A spec já fixou o escopo da Fase 1 (autor-data + bibliografia alfabética) sem ler o código do runtime. Isso inverte a ordem de ADR-0108: a sonda devia determinar o escopo, não confirmar uma decisão já tomada.

**Onde:** spec P388, secções 4 ("Faseamento") e 2 ("Sonda de viabilidade").

**O que devia ter sido feito:** redigir apenas a secção da sonda, executá-la, e só depois escrever o faseamento com base no resultado real. Se a sonda revelasse que o runtime não suporta coleta cross-document, a Fase 1 encolheria para autor-data puro. O P388 escreveu as duas coisas ao mesmo tempo.

---

## P403 e P405 — Constructor de `duration()` dividido em dois passos com L0 desatualizado

**Problema:** P403 criou o constructor `duration("1h30m")` aceitando apenas string posicional. P405 adicionou a forma vanilla com named args (`duration(seconds: 90, hours: 1)`). O L0 de P403 (`primitives-constructors.md`) descrevia apenas a forma string. Quando P405 expandiu o constructor, o L0 foi atualizado, mas isso significa que o L0 ficou desatualizado entre os dois passos — um utilizador do L0 depois de P403 e antes de P405 teria uma descrição incompleta do que o código faz.

**Onde:** relatório P403 (secção "Decisão de engenharia") e relatório P405 (secção "Nota sobre o Tekt").

**O que devia ter sido feito:** se a divisão em dois passos era intencional, o L0 de P403 devia declarar explicitamente que `duration()` estava incompleto (só forma string) e que a forma named args viria em passo seguinte. Ou a divisão devia ter sido planeada desde o início com dois L0 distintos, um por forma.

---

## P407 e P414 — Critério de fecho de DEBT-52 foi revisado sem declarar a revisão

**Problema:** P407 fechou DEBT-52 com a forma dict legada cristalina (`("Name": ("Bold"))`). O relatório de P414 diz que "DEBT-52 já estava formalmente encerrado no Passo 407" mas que "a forma named fields do vanilla ainda faltava". Isso implica que o critério de fecho de DEBT-52 foi ou muito amplo (fechou antes de a paridade estar completa) ou foi revisado depois — mas essa revisão não foi declarada.

**Onde:** relatório P407 ("Inventário / DEBT") e relatório P414 ("Inventário / DEBT").

**O que devia ter sido feito:** quando P407 fechou DEBT-52, devia ter declarado explicitamente que a paridade com a forma vanilla named fields estava fora do escopo e aberto uma nova DEBT ou marcado como `implementado+` (graded). Em vez disso, DEBT-52 foi fechado como completo, e P414 clarificou depois que não estava.

---

## P409 e P413 — Specs escritas para trabalho já feito

**Problema:** P409 foi escrito como spec de materialização de aritmética de `Duration`. P413 foi escrito como spec de materialização de aritmética de `Decimal`. Em ambos os casos, a sonda A.0 revelou que o código já existia (P405 e P404 respetivamente). As specs foram escritas sem verificar se o trabalho estava feito.

**Onde:** specs P409 e P413, secção de contexto ("O tipo existe, mas não participa em operações aritméticas").

**O que devia ter sido feito:** antes de escrever qualquer spec de materialização, executar a sonda A.0 para confirmar o estado atual. Os critérios de passagem da sonda são exactamente para isto. Escrever a spec antes da sonda inverte a ordem do protocolo.

---

## P416 — Mesmo padrão: spec escrita para trabalho já feito

**Problema:** P416 foi escrito como spec de materialização de footnote body no rodapé. A sonda revelou que P304/P305 já tinham implementado isso, com 11 testes. É o mesmo problema de P409 e P413.

**Onde:** spec P416 completa; relatório P416 ("Resumo executivo").

**O que devia ter sido feito:** antes de escrever a spec de P416, grep em `footnote.rs` e `cursor.rs` para confirmar estado atual. O relatório diz que foi isso que aconteceu, mas a spec existe como documento completo, o que sugere que foi escrita antes da sonda.

---

## P420 — Campo de cache em struct de dados de domínio

**Problema:** P420 adicionou `resolved_style: Option<Arc<IndependentStyle>>` a `BibliographyElem`. A justificativa foi que `layout_with_introspector` não recebe `World`, então não havia outro lugar para guardar o style resolvido. Isso significa que um struct de dados de domínio passou a guardar estado computado (cache). O relatório documenta a divergência em relação ao L0 original ("struct inalterado"), mas não avalia o custo futuro: `BibliographyElem` participa de `PartialEq` e `Hash`, e `resolved_style` tem de ser explicitamente excluído dessas comparações ou incluído de forma consistente.

**Onde:** relatório P420, secção 6 ("Notas epistêmicas"), nota sobre "Honestidade".

**O que devia ter sido feito:** o relatório devia declarar explicitamente como `resolved_style` se comporta em `PartialEq` e `Hash` — se é excluído (dois `BibliographyElem` com styles diferentes mas mesmo path são iguais) ou incluído (dois `BibliographyElem` com o mesmo path mas styles resolvidos diferentes são desiguais). A não declaração deixa o comportamento ambíguo.

---

## P421 — Reclassificação S→M não foi antecipada pela sonda

**Problema:** P421 foi planeado como S mas a sonda A.0 revelou que `native_repr` não existia, forçando reclassificação para M. A sonda previu exatamente esta situação (critério de passagem (1) era confirmar que `native_repr` existe), e o critério de reclassificação estava documentado na spec. O problema não é que houve reclassificação — é que uma sonda obrigatória de 5 minutos podia ter revelado isso antes de o passo ser marcado como S. Se a sonda foi feita antes da spec, a spec devia ter sido escrita já como M.

**Onde:** relatório P421, secção 1 ("Sonda do substrato") e nota de reclassificação.

**O que devia ter sido feito:** executar a sonda antes de redigir a spec. Se a sonda mostra que `native_repr` não existe, a spec é escrita como M desde o início, sem precisar de reclassificação.

---

## P423 — Sintaxe infixa `|`/`&` scope-out sem custo medido

**Problema:** a spec de P423 propõe opção β (methods `.or()`/`.and()`) em vez de operadores infixos `|`/`&` no parser, com a justificativa de que o parser é mais caro (M). O custo de M para o parser é uma estimativa, não uma medição. A sonda A.0 verificou se o parser já suportava `|`/`&` (não suportava), mas não mediu o custo de adicioná-los. A decisão de scope-out baseou-se numa estimativa de custo, não numa medição.

**Onde:** spec P423, secção A.1.3 ("Estratégia de implementação"), decisão de opção β.

**O que devia ter sido feito:** a sonda devia incluir um passo de avaliação do custo de adicionar tokens `|` e `&` ao lexer — quantos ficheiros do parser seriam afetados, se há conflito com outros usos de `|` e `&` no código existente. Com essa medição, a decisão de scope-out seria baseada em facto, não em estimativa.

---

## P424 — Bbox aproximada para `Group` não documentada como limitação

**Problema:** P424 calcula a bbox de `FrameItem::Link` para emitir a annotation URI no PDF. Para links que contêm `Group` internamente (por exemplo, um link com texto formatado em bold, que produz `FrameItem::Group` internamente), o cálculo usa `inner_width`/`inner_height` sem aplicar a transform do Group. O relatório menciona isso como "bbox aproximada" nos scope-outs. Mas essa limitação não está nos critérios de aceitação nem nos testes — os testes E2E testam links com texto simples, não links com conteúdo que produz `Group` internamente.

**Onde:** relatório P424, secção "Scope-out / bloqueadores"; `export/tests.rs`.

**O que devia ter sido feito:** adicionar um teste que documenta o comportamento atual para links com `Group` interno — não para forçar que seja correto, mas para que o comportamento seja registado e qualquer alteração futura que o mude seja visível.

---

## P427 — Spec propõe arquitetura que contradiz o L0 vigente

**Problema:** a spec de P427 propõe criar `shape_emit.rs` como módulo separado (opção β). O L0 vigente de `stream.md` (hash `9acca994`) diz explicitamente para não subdividir. O Kimi seguiu o L0 e não criou o módulo. O resultado foi correto, mas a spec foi escrita sem verificar o L0 vigente. Isso significa que a spec continha uma proposta arquitetural que estava em conflito com uma decisão já tomada e registada.

**Onde:** spec P427, secção 5 ("Decisão arquitetural"), opção β; relatório P427, secção "Decisão arquitetural".

**O que devia ter sido feito:** antes de propor arquitetura numa spec, verificar o L0 vigente dos módulos afetados. Se o L0 de `stream.md` já diz para não subdividir, a spec não devia propor a subdivisão como opção preferida.

---

## Padrão geral: specs escritas antes das sondas

Nos passos P388, P409, P413, P416, e P421, a spec foi escrita antes de a sonda de substrato ser executada. Em todos os casos, a sonda revelou que a situação real era diferente da assumida na spec (trabalho já feito, infraestrutura ausente, ou reclassificação necessária). O protocolo exige sonda antes de spec. Quando isso não acontece, o trabalho de escrever a spec pode ser desperdiçado, e a reclassificação tem de ser feita a posteriori, com texto de relatório explicando a divergência.
