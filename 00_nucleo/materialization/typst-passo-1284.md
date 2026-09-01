# Passo 1284 — Fechar módulos, tipos e membros estáticos

## Natureza

Documento tático de execução. Não é Prompt L0 e não legitima código.

## Objetivo

Fechar a superfície pública residual de namespaces como `array`, `bytes`, `arguments`,
`str`, `dictionary`, `color`, `direction`, `alignment`, `duration`, `length`,
`selector`, `state`, `location`, `outline` e `pdf`.

## Pré-condição

Passos 1282 e 1283 fechados e inventário residual regerado.

## Sequência obrigatória

1. Separar método de instância, função estática, constante, módulo e alias antes de
   decidir qualquer implementação.
2. Confirmar cada RED diretamente no vanilla ratificado e no cristalino.
3. Auditar e atualizar o L0 proprietário antes do código; respeitar ownership 1:1.
4. Aplicar ADR-0127: correção interna/tabela de paridade pode seguir continuamente;
   mudança de contrato público/default/pipeline/compatibilidade exige paragem humana.
5. Escrever testes que cubram resolução estática e chamada real, não apenas presença do
   nome no escopo.
6. Implementar por família e reexecutar o inventário depois de cada lote.

## Casos sentinela mínimos

- `array.len`;
- `bytes.len`;
- `arguments.len`;
- `alignment.left`;
- `direction.rtl`;
- `color.map`.

Os sentinelas não substituem o inventário completo.

## Critério de fechamento

- Todos os `MISSING_MEMBER`, `MISSING_BINDING` e `WRONG_KIND` confirmados dessas
  famílias estão fechados ou possuem divergência intencional formalmente decidida.
- Assinaturas relevantes deixam de estar em `UNVERIFIED_METADATA` mediante testes de
  argumentos posicionais, nomeados, defaults e erros observáveis.
- Build, suíte completa e `crystalline-lint .` passam.

## Entrega ao Passo 1285

Lista residual de lacunas de serialização, CLI, query e introspecção, separada da
superfície nominal.
