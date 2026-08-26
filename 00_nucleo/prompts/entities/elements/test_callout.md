# Prompt L0 — fixtures dinâmicas `callout` e `badge`
Hash do Código: b7b63e64

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/entities/element-boundary.toml sha256:cafcd80a58c3e84a9b44a49eb92a93cd2422ddbcf530d295c3645ae54bf1e41b

**Camada:** L1, somente `#[cfg(test)]`
**Ficheiro alvo:** `01_core/src/entities/elements/test_callout.rs`
**ADRs:** ADR-0026, ADR-0105, ADR-0106, ADR-0108, ADR-0129.

## Medição anterior à decisão

O consumer demonstra dois elementos de utilizador: `CalloutElem`, com body,
title e tone; `BadgeElem`, com label e note setável. Ambos implementam o mesmo
trait `Element` dos nativos e entram por `Content::Dynamic` via blanket.

## Contrato

- Callout usa kind estável `callout`, expõe seus três campos, recursa no body e
  produz texto `title: body`.
- Badge usa kind `badge`; label é obrigatório e note explícito vence o valor da
  chain, que só completa note ausente.
- Hash manual de Callout segue Debug porque Content não implementa Hash.
- As fixtures não são elementos nativos nem entram no build produtivo.

## Aceitação

Provar texto, campos, kinds distintos, map do body e precedência de settables.
