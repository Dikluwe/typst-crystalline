# Passo 398 — relatório: `Value::Bytes` + `read` binário + byte-strings CBOR

**Tipo:** modelagem de tipo primitivo L1 + ativação de consumer L3 (`read` binário); expande enum `Value` per ADR-0017.  
**Data:** 2026-06-22. **HEAD:** pós-`111dccdc8`.  
**Caveat de stack:** suíte completa corre com `RUST_MIN_STACK=33554432` (overflow pré-existente em `p350c_flag_on_nao_convergente_classifica`, alheio a este passo).

## O que se fez

Modelou-se o tipo `Bytes` e activou-se o fallback binário em `read(...)`, fechando **DEBT-62** e promovendo `read` e `cbor` de `implementado+` (graded) para `implementado`.

- L0:
  - `00_nucleo/prompts/entities/bytes.md` — tipo `Bytes` e variant `Value::Bytes`.
  - Actualização de `00_nucleo/prompts/engine/stdlib/loading.md` — `read` binário e byte-strings CBOR.
- `01_core/src/entities/bytes.rs`:
  - Tipo L1 puro `Bytes(pub Vec<u8>)` com `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, `Default`.
  - Helpers `new`, `len`, `is_empty`, `as_slice`; conversões `From<Vec<u8>>` e `From<Bytes>` para `Vec<u8>`.
- `01_core/src/entities/value.rs`:
  - Novo variant `Value::Bytes(Bytes)`.
  - `type_name()` retorna `"bytes"`.
  - `cast_bytes()` extrai `&Bytes`.
  - `From<Bytes> for Value`.
  - `PartialEq` e `Hash` reaproveitam os derives existentes (a struct `Bytes` já implementa as traits necessárias).
- `01_core/src/entities/mod.rs`:
  - `pub mod bytes;` registado com nota P398.
- `01_core/src/engine/stdlib/loading.rs`:
  - `native_read`: heurística vanilla — `String::from_utf8` → `Value::Str`; falha → `Value::Bytes(Bytes::new(data.to_vec()))`.
  - `decode_cbor`: byte-strings `ciborium::value::Value::Bytes` mapeadas para `Value::Bytes(Bytes::new(b))`.
  - Testes unitários: `cbor_byte_string_returns_bytes`, `read_texto_utf8`, `read_binario_nao_utf8`, `read_vazio`.
- `03_infra/src/integration_tests.rs`:
  - E2E `read_texto_utf8_pipeline`.
  - E2E `read_binario_pipeline`.
- `00_nucleo/DEBT.md`:
  - DEBT-62 marcado como **FECHADO (Passo 398)** com descrição da resolução.
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:
  - Tabela A.8: `read(...)` e `cbor(...)` reclassificados de `implementado+` para `implementado`.
  - Tabela B.1: `Bytes` reclassificado de `ausente` para `implementado`; contagem de variants 18 → **19**, total 31 → **32**.
  - Tabela B resumo: implementado 74 → **75**, total arquitectural 106 → **107**.
  - Contagem user-facing total mantida em **141** (os itens já estavam contabilizados).
  - Notas de rodapé ⁸¹ (Tabela B.1) e ⁸² (Tabela A.8) para P398.

`cargo test --workspace` verde; `crystalline-lint .` — `✓ No violations found`; hashes propagados via `crystalline-lint --fix-hashes`.

## Protocolo de Nucleação cumprido

1. L0 (`bytes.md` + actualização `loading.md`) escritos e hashes propagados.
2. TDD: testes unitários + E2E escritos antes/paralelamente à implementação.
3. Novo `Value` variant justificado por ADR-0017 (portão aberto em P395) e necessário para resolver DEBT-62.
4. Nenhum I/O novo — `native_read` reusa `World::read_bytes` de P387; decode CBOR continua L1 puro.

## Decisão de engenharia

`Bytes` é um **tipo L1 puro** (`Vec<u8>`) — sem `Arc` porque o `Vec` já é heap-allocated; `Arc` só adicionaria overhead de refcount sem benefício imediato. A struct wrapper permite futura migração para `EcoVec<u8>` (XS) sem alterar o variant `Value::Bytes(Bytes)`.

A paridade com o vanilla em `read(path)` é ao nível da **linguagem** (heurística UTF-8 → `str`, fallback → `bytes`), não da mecânica interna de encoding detection sofisticado — essa continua scope-out per ADR-0054 graded. O mesmo se aplica ao decode CBOR: byte-strings deixam de ser graded e passam a devolver `Value::Bytes` real.

## Paridade

| Caso | Resultado esperado | Estado |
|------|--------------------|--------|
| `read("texto.txt")` com UTF-8 | `Value::Str` | ✓ unit + E2E |
| `read("logo.png")` com PNG magic | `Value::Bytes` | ✓ unit + E2E |
| `read("empty")` (0 bytes) | `Value::Str("")` | ✓ unit |
| `cbor` com byte-string | `Value::Bytes` | ✓ unit |
| `type(data) == "bytes"` | `true` para binário | ✓ unit |
| `Value::Bytes` igualdade por conteúdo | `==` funciona | ✓ unit |

## Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 1 | `Value::Bytes(Bytes)` compila e integra-se no enum | ✓ |
| 2 | `read("texto.txt")` com UTF-8 retorna `Value::Str` | ✓ |
| 3 | `read("logo.png")` com binário retorna `Value::Bytes` | ✓ |
| 4 | `cbor` com byte-strings retorna `Value::Bytes` | ✓ |
| 5 | Zero I/O novo (reusa `read_bytes` de P387) | ✓ |
| 6 | Testes verdes; lint zero; hashes propagados | ✓ 11 unit + 2 E2E novos |
| 7 | Inventário 148 actualizado; DEBT-62 fechado | ✓ |
| 8 | L0 salvos e hashados | ✓ `bytes.md` + `loading.md` |
| 9 | `read` e `cbor` reclassificados `implementado+ → implementado` | ✓ |

## Artefactos

- Código:
  - `01_core/src/entities/bytes.rs`
  - `01_core/src/entities/value.rs`
  - `01_core/src/entities/mod.rs`
  - `01_core/src/engine/stdlib/loading.rs`
  - `03_infra/src/integration_tests.rs`
- L0:
  - `00_nucleo/prompts/entities/bytes.md`
  - `00_nucleo/prompts/engine/stdlib/loading.md`
- Inventário 148: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`.
- DEBT: `00_nucleo/DEBT.md`.
- Plano: `00_nucleo/materialization/typst-passo-398.md`.
- Este relatório.

## Nota sobre o Tekt

P398 fecha o **débito técnico DEBT-62**, última trava do cluster `loading` (P387). Após este passo:
- `read(...)` está completo (texto + binário).
- `cbor` decode está completo para byte-strings.
- O portão ADR-0017 permanece aberto para os tipos S restantes (`Decimal`, `Duration`, `Version`).

O ciclo deste passo serve de baseline para **ativação de consumer** (tipo modelado em L1 + consumer L3 ligado) versus passos de modelagem pura sem consumer.
