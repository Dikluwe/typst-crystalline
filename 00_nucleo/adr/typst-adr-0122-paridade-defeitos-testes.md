# ADR-0122 — Paridade de definições por defeito nos testes de comparação

**Estado:** `EM VIGOR`
**Data:** 2026-07-05
**Aplica-se a:** todos os testes que comparam directamente o cristalino com o vanilla (posições, larguras, contagem de páginas, ou qualquer número medido dos dois lados).
**Histórico de numeração:** sem ficheiro/número próprio até P910; reconciliada nesse passo por
varredura real de `00_nucleo/adr/` (slot `0122` livre, atribuído nesta remessa junto com `0121`).

---

## O problema

P588 mediu uma diferença de 2,8 pontos entre o cristalino e o vanilla, que pareceu ser um bug de layout. Depois de investigação, a causa era simples: o cristalino usa Liberation Serif como fonte por defeito (decisão de P558); o vanilla usa Libertinus Serif. Os dígitos das duas fontes têm larguras diferentes. Não havia bug nenhum — havia uma comparação entre duas coisas que nunca podiam ser iguais, porque partiam de bases diferentes.

Isto já aconteceu antes desta forma, nesta conversa, com fontes diferentes a mascarar ou a imitar problemas reais. Cada vez que acontece, gasta-se tempo a investigar uma diferença que não é sobre o algoritmo — é sobre a fonte escolhida.

## Distinção necessária

Há dois tipos de teste, e não podem ser tratados da mesma forma:

### 1. Testes de paridade de algoritmo

A pergunta é: "o mecanismo de layout do cristalino comporta-se da mesma forma que o do vanilla, para o mesmo conteúdo?" Para responder a isto, as duas fontes têm de ser a mesma nos dois lados — se não forem, qualquer diferença de posição ou largura pode vir da fonte, não do algoritmo, e a pergunta fica sem resposta clara.

**Regra:** estes testes têm de forçar explicitamente a mesma fonte (e outras definições sensíveis — margem, tamanho de página, língua) nos dois compiladores, através de `#set text(font: ...)` explícito no documento de teste, não confiando em nenhum valor por defeito de nenhum dos dois lados.

### 2. Testes de resultado de produção

A pergunta é: "o que um utilizador real recebe do cristalino, hoje, com as suas escolhas por defeito reais?" Para isto, forçar a mesma fonte nos dois lados esconderia a experiência real — incluindo a diferença de fonte por defeito, que é uma decisão já tomada e registada (P558), não um erro a esconder.

**Regra:** estes testes usam os valores por defeito reais de cada lado, sem forçar nada. Uma diferença encontrada aqui pode ser a fonte por defeito (aceitável, já decidido) ou pode ser outra coisa — precisa de investigação para separar as duas, exactamente como P588 fez.

## Como aplicar

Antes de escrever um teste de comparação, decidir explicitamente qual dos dois tipos é: algoritmo, ou produção. Escrever essa decisão no próprio teste ou no relatório, não deixar implícito.

Para testes de algoritmo: usar sempre `#set text(font: "<nome explícito>")` no documento de teste, escolhendo uma fonte disponível nos dois compiladores (por exemplo, uma fonte do sistema operativo comum aos dois ambientes de teste, confirmada por sonda antes de decidir qual).

Para testes de produção: não mexer nos valores por defeito, mas registar explicitamente, no relatório, quando uma diferença encontrada é atribuível a um valor por defeito diferente (fonte, margem, etc.), com a mesma medição directa que P588 fez (comparação de métricas de fonte reais, não suposição).

## Implementação recomendada

Criar um documento `00_nucleo/testing/fontes-padrao-teste.md` (ou equivalente) que registe:

- A fonte a usar em testes de algoritmo (uma só, escolhida por disponibilidade nos dois ambientes).
- A lista de outras definições sensíveis a considerar da mesma forma (margem por defeito, tamanho de página por defeito, língua por defeito) — se algum destes também divergir entre cristalino e vanilla, aplicar a mesma regra.
- Uma função ou helper de teste (`fn documento_teste_com_fonte_neutra(conteudo: &str) -> String`, ou equivalente) que insira automaticamente o `#set text(font: ...)` neutro em qualquer documento de teste de algoritmo, para não depender de cada pessoa lembrar-se de o escrever à mão.

## Ligação às regras anteriores

- **Decisão nova obrigatória** — nenhuma escolha de fonte por defeito fica aceite para sempre sem revisão (citada por nome; não materializada como ADR própria — ver nota de reconciliação em ADR-0119).
- **Disciplina de verificação** (ADR-0119) — uma diferença medida precisa de causa confirmada, não suposição.
- **Proveniência de medição** (ADR-0121) — o estado exacto do código usado numa medição fica registado.
- **Esta regra** — antes de comparar números entre cristalino e vanilla, decidir se a fonte (e outras definições por defeito) têm de ser neutralizadas, ou se fazem parte do que está a ser testado.
