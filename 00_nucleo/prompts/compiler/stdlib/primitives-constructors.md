# Prompt L0 — hub `stdlib/primitives_constructors`
Hash do Código: e5419abe

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

## P1339 — ligação interna da operação version.at

### Medição anterior à decisão

No HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`,
`primitives_constructors.rs:11,15` mantém version privado e só reexporta seu
constructor; `eval/bindings/value_methods.rs:623-662` implementa a validação
ligada de at antes de delegar a Version::at. As sondas P1339 full-final e
full-boundaries conservam a ausência da forma estática e as diferenças de
diagnóstico da ligada. Os novos helpers pertencem ao owner version, não ao hub.

### Decisão

Reexportar explicitamente, em `pub(crate)`, `version_type_field` e
`dispatch_version_method` do módulo privado version. O primeiro encaminha a
descoberta; o segundo recebe receiver/Args já avaliados e compartilha a
semântica da forma estática no owner. Este nó não valida argumentos, produz
Value, calcula índices nem define mensagens. Reexports públicos dos
constructors, demais módulos e suporte de testes ficam preservados.

Esta cláusula sucede exclusivamente a exigência de imutabilidade do hub P1200
para essas ligações. Não expõe o módulo version nem cria contrato Rust externo.
O núcleo de primitive-calls continua regendo constructors; não transformar
uma operação at em constructor para satisfazer sua claim typed-result.

## P1339 — pré-requisito autorizado Array a partir de Bytes

Medição anterior: `primitives_constructors.rs:9-16` declara owners privados
e reexports; a ausência de Array foi medida com HEAD/árvore/UTC em
`diagnosticos/p1339-array-prerequisite-measure-r2.json`. Após autorização em
`diagnosticos/p1339-array-authorization.md`, declarar módulo privado `array`
e reexportar somente `array::native_array_bytes` em `pub(crate)`.
Sem wrapper, cast, validação ou API externa. A semântica limitada e seu
estado incompleto pertencem a `primitives-constructors/array.md`.
Esta cláusula sucede a enumeração fechada e imutabilidade do hub apenas
para essa ligação adicional; demais owners e núcleo permanecem intactos.
