# Prompt L0 — constructor primitivo `duration`
Hash do Código: defbd6c4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/stdlib/primitive-calls.toml sha256:761d5adeca09f6a60ba2960f8492aa6c0d934798bbc826f341004cf993ba8736

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/stdlib/primitives_constructors/duration.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129.

## Medição anterior à decisão

O consumer vigente possui duas formas exclusivas: uma string canónica
posicional e componentes inteiros nomeados. Misturar as formas é erro. A forma
nomeada aceita `weeks`, `days`, `hours`, `minutes`, `seconds`, `milliseconds`,
`microseconds` e `nanoseconds`, inclusive valores negativos. P1200 não muda
essa superfície; apenas individualiza seu owner.

## Contrato

`native_duration(ctx, args, world, current_file) -> SourceResult<Value>`:

- na forma posicional, aceitar exatamente uma string com componentes `d`, `h`,
  `m`, `s` em ordem canónica, sinal inicial opcional e segundos fracionários;
- na forma nomeada, somar os oito componentes inteiros em nanos usando `i128`;
- ausência total de argumentos produz duração zero;
- rejeitar mistura de formas, tipos errados, strings vazias/malformadas,
  sufixos desconhecidos ou fora de ordem;
- retornar `Value::Duration` e permanecer L1 puro.

## Limites e gates

- Não adicionar cast, `.in(unit)`, `.display()` ou extractores de componente.
- A forma string é compatibilidade existente; removê-la ou mudar forma/erro é
  mudança pública sujeita à ADR-0127.
- A entrada por tipo chamável delega a esta mesma free function.

## Aceitação

Cobrir zero, componentes de string, fração, sinal, componentes nomeados,
negativos, tipos inválidos, mistura de formas e ordem/sufixo inválidos.
