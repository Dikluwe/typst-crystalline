# Prompt L0 — constructor primitivo `version`
Hash do Código: 76701aae

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/stdlib/primitive-calls.toml sha256:761d5adeca09f6a60ba2960f8492aa6c0d934798bbc826f341004cf993ba8736

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/stdlib/primitives_constructors/version.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129.

## Medição anterior à decisão

O consumer vigente aceita string numérica pontuada, qualquer quantidade de
componentes inteiros posicionais não negativos, ou um array desses componentes.
Argumentos nomeados e componentes `pre`/`build` são rejeitados. P1200 preserva
integralmente esse comportamento e individualiza somente ownership e linhagem.

## Contrato

`native_version(ctx, args, world, current_file) -> SourceResult<Value>`:

- uma string isolada usa `Version::from_str`;
- um array isolado ou os próprios posicionais fornecem zero ou mais componentes
  inteiros não negativos;
- zero componentes produz `Version::default()`;
- rejeitar negativos, componentes não inteiros, strings inválidas e quaisquer
  argumentos nomeados;
- retornar `Value::Version(Arc<Version>)` e permanecer L1 puro.

## Limites e gates

- `pre` e `build` não pertencem à linguagem Typst e não são aceitos.
- Não adicionar bump, `.at(index)`, conversão para string ou novas operações.
- A entrada por tipo chamável delega a esta mesma free function; mudança da
  superfície pública permanece sob ADR-0127.

## Aceitação

Cobrir strings válidas e inválidas, cardinalidades arbitrárias, vazio, array,
negativos, não inteiros e argumentos nomeados rejeitados.
