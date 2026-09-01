# Passo 1286 — Fechar REDs semânticos confirmados

## Natureza

Documento tático de execução. Não é Prompt L0 e não legitima código.

## Objetivo

Corrigir as divergências públicas já reproduzidas em `smartquote`, `line`, `color.mix`
e funções PDF, medindo o vanilla antes de decidir cada contrato.

## Fatias

1. `smartquote(alternative: true)` e `smartquote(quotes: ...)`.
2. `line(start: ..., end: ...)` com origem diferente de zero.
3. `color.mix` com mais de duas cores e combinações de pesos aceitas pelo vanilla.
4. `pdf.attach`.
5. Semântica observável de `pdf.artifact`, separada de mera validação/passthrough.

## Sequência obrigatória por fatia

1. Localizar fonte vanilla ratificada e registrar `file:line`.
2. Medir casos válidos, limites, defaults e mensagens de erro relevantes.
3. Classificar semântica/sintaxe/morfologia versus mecânica conforme ADR-0107.
4. Auditar e atualizar o Prompt L0 proprietário antes do teste ou código.
5. Se houver novo default, contrato público, fase de pipeline ou incompatibilidade,
   parar após o L0 para confirmação ADR-0127.
6. Escrever RED bilateral, implementar a menor fatia e demonstrar GREEN.

## Critério de fechamento

- Cada fatia possui casos positivos, negativos e limítrofes comparados ao vanilla.
- Não permanece `scope-out` para comportamento que o inventário ratificado classificou
  como parte da linguagem-alvo, salvo divergência intencional formalmente aprovada.
- `cargo test --workspace`, matriz focal, build e `crystalline-lint .` passam.

## Entrega ao Passo 1287

Corpus de regressão com todos os REDs deste passo e inventário residual semântico.
