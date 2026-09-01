# Passo 1285 — Completar eval, serialização, query e seletores

## Natureza

Documento tático de execução. Não é Prompt L0 e não legitima código.

## Objetivo

Alinhar os observáveis públicos de `typst eval`, formatos de saída, `query` e matching
de seletores com o vanilla ratificado.

## Escopo confirmado pela auditoria

1. Adicionar/alinhar `typst eval --format yaml`.
2. Completar JSON para valores públicos atualmente rejeitados, começando por `Version`.
3. Medir e implementar os demais formatos e tipos efetivamente expostos pelo vanilla.
4. Remover a limitação de serialização de query somente a headings.
5. Completar matching geral de seletores de texto e regex sem regredir o caminho
   específico já usado por show rules.

## Sequência obrigatória

1. Medir help, códigos de saída, stdout/stderr e valores semânticos nos dois binários.
2. Auditar os L0 proprietários de CLI, serialização, query e seletores.
3. Como YAML altera uma opção pública e comportamento do produto, atualizar L0 e parar
   para confirmação conforme ADR-0127 antes de implementar esse eixo.
4. Após confirmação, escrever testes RED por tipo/formato e por classe de selector.
5. Implementar sem confundir representação serializada observável com igualdade interna
   de valores.

## Casos sentinela mínimos

- `typst eval 'sys.version' --format json`;
- valor composto serializado em YAML;
- query de elemento não-heading com campos públicos;
- selector textual literal;
- selector regex com match e não-match.

## Critério de fechamento

- Formatos anunciados pelo CLI coincidem com os suportados e seus erros são equivalentes
  no nível observável.
- Valores públicos selecionados serializam com a mesma semântica do vanilla.
- Query não possui ramificação especial exclusiva para headings.
- Texto/regex funcionam no matching genérico coberto pelo contrato.
- Build, suíte completa, matriz focal e `crystalline-lint .` passam.

## Entrega ao Passo 1286

Lista somente de divergências semânticas confirmadas, sem misturar limitações do CLI ou
do harness.
