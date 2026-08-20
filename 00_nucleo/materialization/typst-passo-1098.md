# L0 — Passo 1098: Margem de Página `auto` Errada com Texto Fluido + Matemática Inline

**Gate**: `ADR-0127` — mudança de comportamento por defeito (qualquer
documento com `width/height: auto` misturando parágrafo e `$...$` inline é
afectado).

**Base**: nota externa (2026-08-20). Vanilla confirmado genuíno (recompilado
do binário oficial, bate ao ponto decimal). Secção 31 (parágrafo + math
inline): margem direita `61.24pt` (esperado `28.35pt`, mais do dobro);
margem inferior `5.50pt` (esperado `26.21pt` no vanilla, quase zero no
cristalino). Secção 32 (só blocos de fórmula, sem parágrafo): margem bate
quase certo (`29.62pt`/`28.35pt`). **Escopo já isolado pela nota**: o bug é
específico à combinação parágrafo-fluido + math inline, não a `width/height:
auto` em geral, nem a matemática em geral.

---

## 1. Não repetir a instrução da nota — ela já isolou a variável certa

A nota já fez o trabalho de descartar hipóteses (testou secção 32 sem
parágrafo, bug não aparece) — não retestar isso do zero. Partir directamente
de: por que a presença de um parágrafo de texto corrido, com `$...$` inline
misturado, faz o cálculo de largura/altura `auto` da página errar
especificamente na direcção "direita a mais, baixo a menos"?

## 2. Hipótese a testar — medição de largura de parágrafo com inline math

O padrão (direita sobra, baixo falta) sugere que o cálculo de `auto` está a
usar uma medição de largura **maior que a real** para a linha com math
inline (sobra à direita) e ao mesmo tempo uma estimativa de altura
**menor que a real** (falta embaixo) — duas medições diferentes,
possivelmente dois mecanismos diferentes, não necessariamente a mesma causa.

Não presumir uma causa única para os dois sintomas. Testar separadamente:

1. Um documento só com um parágrafo (sem math inline nenhum) sob `width:
   auto` — a largura já diverge sem matemática, ou só diverge quando há
   `$...$` misturado?
2. Um documento com parágrafo + math inline, mas sob `width` fixo (não
   auto) — a posição horizontal de cada glifo bate com o vanilla? (a nota já
   sugere que sim — "conteúdo bate 100%... indicando conteúdo centralizado
   numa página mais larga, não é o conteúdo que se moveu" — mas confirmar
   isto isola definitivamente o problema no cálculo de `auto`, não no layout
   do próprio parágrafo).

## 3. Ler o código real do cálculo de `width: auto`/`height: auto`

Esta investigação (P1086-1097) já mexeu extensivamente em
`compute_page_width`/`compute_page_height` (P1096) — ler esse código
primeiro, não presumir que o bug desta nota é o mesmo já corrigido ou é
território novo. Confirmar se o mecanismo de medição de largura considera
correctamente uma `Sequence` com `Content::Text` e `Content::Equation`
(inline, não bloco) misturados — o P1096 focou em equações de **bloco**
(`$ $`), pode não cobrir o caminho de medição de parágrafo com inline
(`$...$` sem quebra).

## 4. Achado secundário — offset de ~1.27pt por linha, investigar se relacionado

Medir se o offset de altura de linha (texto corrido + math inline) tem
relação numérica com a causa principal (§1-3) ou é independente. Não
presumir a relação — se a causa principal for sobre `width`/`height` de
página, e este offset for sobre avanço vertical de linha dentro do fluxo
normal, podem ser duas coisas sem conexão, apesar de aparecerem no mesmo
documento.

## Critério de conclusão

- §2: dois testes de isolamento executados, sintoma de largura e de altura
  tratados como potencialmente independentes até prova em contrário.
- §3: código real de `compute_page_width`/`height` (pós-P1096) revisto
  especificamente para o caminho de parágrafo+inline, não só bloco.
- §4: relação entre o offset de linha e a causa principal confirmada ou
  descartada com dados.
- Causa(s) identificada(s) com código citado, não descrição em prosa.
