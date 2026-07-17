# Sugestão futura — modelo de avanço de linha personalizável

**Contexto:** P762 corrigiu o cristalino para usar `cap-height + leading`, a convenção específica do Typst, alcançando paridade exacta com o vanilla. Esta sugestão é sobre ir **além** da paridade, como extensão opcional — não muda nem ameaça o que P762 já fixou como comportamento por defeito.

---

## A ideia

Diferentes sistemas de composição tipográfica escolhem fórmulas diferentes para o avanço vertical entre linhas, todas legítimas:

| Sistema | Fórmula aproximada |
|---|---|
| Typst (vanilla, e agora o cristalino por defeito) | `cap-height + leading` |
| Navegadores web (CSS) | `ascent + descent + line-gap` (métricas da própria fonte) |
| LaTeX | `\baselineskip`, historicamente um múltiplo do tamanho de ponto |
| Processadores de texto (Word, etc.) | espaçamento "simples" como múltiplo fixo (~1,15-1,2×) do tamanho |

O cristalino, tal como está depois de P762, replica só a primeira opção — correctamente, para efeitos de paridade. Mas um utilizador que venha de LaTeX, ou que precise de compatibilidade visual com um documento Word, ou que simplesmente prefira o comportamento baseado em métricas de fonte (mais comum fora do universo Typst), não tem forma de pedir isso.

## Proposta

Um argumento nomeado novo, `#set text(line-advance-model: "typst")` (ou nome semelhante a decidir), com um conjunto pequeno de valores predefinidos:

- `"typst"` (default, o comportamento actual pós-P762, paridade com o vanilla)
- `"font-metrics"` (o modelo antigo do cristalino, pré-P762 — reaproveitar o código já existente antes da mudança, não reescrever)
- Possivelmente `"fixed-multiple"` com um segundo argumento (`line-advance-multiple: 1.2`), para o caso de espaçamento simples de processador de texto

## Porque não fazer isto agora

Segue a mesma regra já estabelecida nesta conversa para divergências de linguagem (P662-664): qualquer coisa que a sintaxe aceite tem de ter nome distinto do vanilla, ser uma decisão consciente, e estar documentada como extensão não-portável — nunca disfarçada do nome/comportamento oficial. `line-advance-model` não existe no Typst real; um documento que o use deixaria de compilar num Typst real, o que é aceitável para uma extensão deste tipo (o mesmo princípio de `table.numbering`, P459), mas exige decisão e documentação explícitas, não implementação apressada.

Também não há, agora, nenhum consumidor real a pedir isto — foi uma pergunta levantada por curiosidade sobre a natureza do modelo de avanço de linha, não uma necessidade identificada num documento real ou num pacote da comunidade. Seguindo a disciplina já estabelecida (nunca implementar por antecipação sem medir a necessidade real), isto fica registado como ideia, não como próximo passo.

## Quando reconsiderar

Se, no futuro:
- Um pacote real da comunidade precisar de compatibilidade com métricas de fonte para algum efeito visual específico, ou
- Houver um pedido concreto de utilizador para replicar comportamento de LaTeX/Word dentro de um documento Typst, ou
- O código antigo do modelo `font-metrics` (pré-P762) estiver prestes a ser removido/já não for facilmente recuperável do histórico

— nessa altura, vale a pena reabrir esta ideia com uma sonda própria (confirmar a sintaxe exacta a usar, os valores predefinidos que fazem sentido, se algum sistema de referência real justifica cada opção), seguindo o mesmo processo já usado para todas as outras extensões desta conversa.

## Nota técnica para quando for retomado

O código do modelo `ascender+descender+lineGap` não deve ser apagado ao implementar P762 — só deixar de ser o caminho por defeito. Se P762 já o tiver removido por completo, esta ideia exige reconstruir esse caminho a partir do histórico de commits, não do zero.
