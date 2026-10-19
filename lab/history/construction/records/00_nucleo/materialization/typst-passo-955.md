# Passo 955 — buscar a decisão original do formato PDF actual (Td vs Tm, sem q/cm/Q por bloco, sem BDC/EMC)

**Precede este passo**: `typst-adr-0126-modo-verboso-primeiro.md` — confirmou que a formulação nova
("verboso primeiro, compacto depois") não existia antes desta conversa. **Mas isso não confirma
que o formato actual do exportador (`Td`, sem `q`/`cm`/`Q` por bloco de texto, sem `BDC`/`EMC`)
nunca foi decidido antes** — só que essa formulação específica de prioridade é nova. É bem provável
que exista uma decisão mais antiga, dos passos que construíram o exportador de PDF pela primeira
vez, que explique por que o cristalino escreve desta forma — mesmo que ninguém a tenha chamado de
"modo verboso" na altura.

**Pré-condição de árvore**: `git status`. Confirmar ADR-0126 presente.

---

## Fase A — varredura completa da história, não só dos passos recentes

1. Localizar os passos que construíram o exportador de PDF pela primeira vez — candidatos prováveis
   pelos números já vistos nesta conversa: os passos referenciados por `ADR-0055` (font consumer via
   pipeline CIDFont), e qualquer passo anterior/posterior que tenha decidido a estrutura do content
   stream (`BT`/`ET`, `Td` vs `Tm`, presença ou ausência de `q`/`cm`/`Q`). Procurar por número de
   passo no intervalo onde o exportador PDF nasceu — confirmar esse intervalo por grep em
   `00_nucleo/` antes de adivinhar.
2. Varrer **todos** os relatórios/prompts de materialização do projeto (não só P944-953) por termos
   como `Td`, `Tm`, `content stream`, `BT.*ET`, `q.*cm.*Q`, `text matrix`, `posição de texto` — para
   confirmar se algum passo antigo já discutiu explicitamente a escolha entre `Td` e `Tm`, ou se foi
   simplesmente a primeira forma que funcionou e nunca foi revisitada.
3. Confirmar se há uma ADR antiga (número baixo, dos primeiros ~55-140) que trate da arquitectura do
   exportador de PDF em geral — mesmo que não mencione "verboso"/"compacto" pelo nome, pode conter a
   decisão de fundo (por exemplo, "cada item de texto é um bloco `BT...ET` independente, posicionado
   por deslocamento" como decisão de design, com ou sem razão registada).
4. Se não encontrar nenhuma decisão explícita: confirmar isso também é uma resposta válida — significa
   que o formato actual não foi *escolhido* deliberadamente sobre `Tm`/`q`/`cm`/`Q`, foi simplesmente
   a implementação mínima viável da altura, nunca reconsiderada. Isso é diferente de "decisão
   consciente de usar o formato simples" e vale registar essa distinção com precisão.

## Fase B — reconciliar com a ADR-0126

1. Se uma decisão antiga for encontrada: confirmar se ela é compatível com `ADR-0126` (a nova
   prioridade não contradiz uma decisão antiga de arquitectura) ou se há tensão entre as duas — por
   exemplo, se a decisão antiga já tinha razões específicas para preferir `Td` sobre `Tm` (não só
   "foi o mais simples"), essas razões precisam de ser reconciliadas com o plano de "modo compacto
   depois" da `ADR-0126`.
2. Actualizar `ADR-0126` com a referência à decisão antiga encontrada (se houver), corrigindo a
   secção de proveniência para não dizer "não consta de nenhum passo" se, na verdade, constar — só
   não tinha sido encontrado porque a varredura de P954 foi limitada aos passos recentes.
3. Se nenhuma decisão antiga for encontrada: `ADR-0126` fica como está, mas ganha uma nota explícita
   confirmando que a varredura desta vez foi mais ampla (todo o histórico, não só P944-953) e ainda
   assim não encontrou nada — reforça a conclusão em vez de a deixar incompleta.

## Resultado esperado

- Confirmação definitiva: existe ou não existe uma decisão antiga sobre o formato do exportador de
  PDF, com número de passo/ADR citado se existir.
- `ADR-0126` corrigida ou reforçada, conforme o resultado.
- Se uma tensão real for encontrada entre uma decisão antiga e a nova prioridade: registada
  explicitamente, não escondida, com uma recomendação de como reconciliar (não necessariamente
  resolvida neste mesmo passo, se for uma decisão maior).
