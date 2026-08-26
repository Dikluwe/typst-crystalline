# Prompt L0 — constructor primitivo `decimal`
Hash do Código: adaf2600

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/stdlib/primitive-calls.toml sha256:761d5adeca09f6a60ba2960f8492aa6c0d934798bbc826f341004cf993ba8736

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/stdlib/primitives_constructors/decimal.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129.

## Medição anterior à decisão

O consumer vigente recebe `Args`, rejeita argumentos nomeados e aceita somente
um posicional `Value::Str`. A string é convertida por `Decimal::from_str`; as
demais cardinalidades, tipos e strings inválidas produzem diagnóstico. O corpo
já está atomizado, portanto P1200 individualiza somente ownership e linhagem.

## Contrato

`native_decimal(ctx, args, world, current_file) -> SourceResult<Value>`:

- aceita exatamente uma string posicional e nenhum argumento nomeado;
- converte uma string válida em `Value::Decimal`;
- rejeita string inválida, tipo incorreto e cardinalidade diferente de um;
- não lê `world`, `current_file` nem produz I/O.

## Limites e gates

- Não aceitar argumentos nomeados nem criar novas formas de coerção.
- Não criar variants de `Value` nem operações aritméticas.
- A entrada por tipo chamável delega a esta mesma free function, sem wrapper
  semântico; qualquer mudança desse contrato público permanece sob ADR-0127.

## Aceitação

Cobrir string válida, string inválida, tipo incorreto, excesso de argumentos e
argumento nomeado rejeitado, preservando a semântica pública existente.
