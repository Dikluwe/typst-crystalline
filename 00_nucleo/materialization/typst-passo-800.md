# Prompt — typst-passo-800 (achado P798 #7): `syntax::kind` — ordem/conteúdo do output diverge em `#if` com math inline

**Origem**: P798 (lote 3 de triagem em lote, corrigido), tabela "Achados de P798, aguardando passo dedicado"
**Handoff**: `00_nucleo/handoff-novo-chat-p798.md`
**Módulo afectado**: `syntax::kind`
**Estado**: aguardando execução, ainda não corrigido

---

## Achado (texto exacto do handoff)

> `#if true [Hello $x^2$]` — ordem/conteúdo do output completamente diferente do vanilla

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" ou "mecanicamente correto" sem execução mostrada. Cada afirmação do relatório deste passo tem de vir acompanhada do comando exacto e da saída literal, comparando vanilla (`lab/typst-original/target/release/typst`) e cristalino (`./target/release/typst`). A contagem de testes da suíte `typst-core` tem de aparecer no relatório e bater com o número de testes novos declarados.

---

## Passo 1 — Sonda (obrigatória antes de qualquer alteração de código)

1. Compilar `#if true [Hello $x^2$]` com os dois binários e registar a saída literal completa de cada um (texto renderizado, e se possível a árvore de conteúdo/introspecção intermédia).
2. Descrever exactamente em que consiste a diferença: ordem dos elementos, conteúdo do sub/superscript, ou ambos.
3. Isolar a variável: testar separadamente `#if true [Hello]` (sem math) e `$x^2$` sozinho (sem `#if`), para determinar se a divergência nasce da combinação `#if` + conteúdo inline com math, ou se já existe em `$x^2$` isolado (achado #13 de P798 (typst-passo-799), `math::attach`, é candidato a sobreposição — ver handoff §"Recomendação para o próximo chat" item 2).
4. Localizar no código-fonte do vanilla (`lab/typst-original/`) o caminho de avaliação de `#if` com corpo de conteúdo (`SyntaxKind::Conditional` ou equivalente) e confirmar como o corpo é avaliado e materializado.
5. Localizar no código do cristalino o caminho equivalente e identificar o ponto de divergência.
6. Registar os pontos exactos (vanilla e cristalino) no relatório antes de tocar em código.

## Passo 2 — Implementação

Depende do resultado do Passo 1.3: se a causa for a mesma do achado #13 (`math::attach`), este passo deve ser fundido com esse — não corrigir duas vezes o mesmo mecanismo. Se for uma causa distinta ligada especificamente a `#if`, corrigir o caminho de avaliação identificado no Passo 1, replicando a ordem/estrutura do vanilla.

## Passo 3 — Validação

1. Recompilar o cristalino.
2. Repetir o comando do Passo 1 e mostrar a saída literal, agora igual à do vanilla.
3. Adicionar caso de teste cobrindo `#if` com corpo contendo math inline com sub/superscript.
4. Correr a suíte `typst-core` completa e mostrar o comando e a contagem de testes antes/depois.

## Passo 4 — Relatório

Produzir `00_nucleo/materialization/typst-passo-800-relatorio.md` com:
- Comando + saída literal do Passo 1 (antes da correcção), incluindo os testes isolados (`#if` sem math, math sem `#if`).
- Confirmação explícita se este achado se fundiu ou não com o achado #13.
- Trecho do código vanilla e do código cristalino identificados no Passo 1.
- Diff da correcção.
- Comando + saída literal do Passo 3 (depois da correcção).
- Contagem de testes antes/depois.
