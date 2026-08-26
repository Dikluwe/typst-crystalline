# Prompt L0 — hub `stdlib/primitives_constructors`
Hash do Código: 6715df5b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/stdlib/primitive-calls.toml sha256:761d5adeca09f6a60ba2960f8492aa6c0d934798bbc826f341004cf993ba8736

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/stdlib/primitives_constructors.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129.

## Medição anterior à decisão

O Passo 1140.1-A já atomizou os três constructors em `decimal.rs`,
`duration.rs` e `version.rs`. O consumer deste prompt contém somente as três
declarações privadas de módulo, os três reexports das funções nativas e suporte
compartilhado restrito a `#[cfg(test)]`. Logo, pela cardinalidade 1:1 da
ADR-0129, este L0 não possui as semânticas particulares dos constructors.

## Contrato do hub

- Declarar os módulos privados `decimal`, `duration` e `version`.
- Reexportar `native_decimal`, `native_duration` e `native_version` sem wrapper,
  coerção ou alteração de erro.
- Manter o suporte de testes (`Args`, `EvalContext`, `FileId` e `NullWorld`)
  fora do build produtivo.
- Permanecer L1 puro e sem despacho dinâmico.

## Limites

- Cada função nativa é legitimada pelo L0 homónimo em
  `primitives-constructors/`.
- O hub não decide formas de chamada, coerções, mensagens de erro ou semântica
  dos valores produzidos.
- Gates públicos registrados nos owners individuais não são implementados por
  este saneamento documental.

## Aceitação

- Os três símbolos são reexportados diretamente de seus módulos donos.
- Os testes dos três módulos conseguem reutilizar o suporte local.
- O ficheiro produtivo, desconsideradas apenas as linhas de linhagem, permanece
  idêntico ao estado anterior ao saneamento P1200.
