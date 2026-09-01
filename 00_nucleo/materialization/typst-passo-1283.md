# Passo 1283 — Fechar tabelas de símbolos e variantes

## Natureza

Documento tático de execução. Não é Prompt L0 e não legitima código.

## Objetivo

Materializar a paridade confirmada das famílias declarativas `math`, `sym` e `emoji`,
que concentram a maior parte dos membros nominais ausentes.

## Pré-condição

Passo 1282 fechado com inventário bilateral atual e amostras RED reproduzíveis.

## Sequência obrigatória

1. Identificar o Prompt L0 proprietário de cada consumer produtivo e validar ownership
   1:1 e pins de Núcleos Tekt (ADR-0129).
2. Atualizar L0 antes do código com a fonte vanilla `file:line`, classificação
   língua-versus-mecânica e critérios de aceite semântico/sintático/morfológico.
3. Para entradas de tabela e correções internas de paridade, seguir fluxo contínuo
   L0 → hash → RED → GREEN. Se surgir contrato público Rust, default, fase de pipeline
   ou quebra de compatibilidade, parar após o L0 conforme ADR-0127.
4. Materializar em lotes pequenos por família, preservando aliases, modifiers,
   variantes e kinds observáveis.
5. Testar existência, resolução, kind, representação e comportamento em expressões.

## Proibições

- Não gerar nomes apenas por semelhança ou por contagem histórica.
- Não usar igualdade mecânica de Rust ou bytes internos como critério de linguagem.
- Não consolidar consumers distintos sob um único Prompt L0.
- Não remover extensões cristalinas sem decisão explícita de compatibilidade.

## Validação por lote

- Sondas bilaterais focais.
- Testes do módulo.
- Inventário do Passo 1282 novamente executado.
- `cargo test --workspace`.
- `cargo build --workspace --bin typst`.
- `crystalline-lint .` com zero violações.

## Critério de fechamento

- Nenhuma lacuna confirmada das famílias selecionadas permanece sem decisão: cada item
  está materializado ou classificado com evidência como divergência intencional.
- Redução do inventário é registrada por família, sem alegar percentagem global.

## Entrega ao Passo 1284

Inventário residual excluindo `math`, `sym` e `emoji`, com foco em módulos, tipos e
membros estáticos.
