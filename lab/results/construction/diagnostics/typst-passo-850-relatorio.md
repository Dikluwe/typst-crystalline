# Relatório — typst-passo-850: `Duration` com representação com sinal — implementação (Opção A)

**Data:** 2026-07-23  
**Executor:** Kimi Code (agente principal).  
**Proveniência das medições:** working tree não commitado; HEAD base `dfe3c2282` (commit do dono com relatórios P848–P850 e ajustes em duration/DEBT).  
**Estado:** **implementado e validado**.

---

## 1. Problema

`#repr(-duration(seconds: 3))` divergia do vanilla:

- **Vanilla:** `duration(seconds: -3)`.
- **Cristalino (antes):** `error: cannot apply Neg to duration`.

Causa: a entidade `Duration` usava `u64` de nanossegundos.

---

## 2. Decisão

O dono escolheu a **Opção A**: migrar `Duration` para representação com sinal (`i128` de nanossegundos).

---

## 3. Alterações realizadas

| Ficheiro | Mudança |
|----------|---------|
| `01_core/src/entities/duration.rs` | `nanos: u64` → `nanos: i128`; constructores/acessores assinados; `impl Neg`; `to_string()` para negativos; testes novos. |
| `01_core/src/entities/value.rs` | `cast_duration` aceita `Int`/`Float` negativos; testes actualizados. |
| `01_core/src/engine/eval/operators.rs` | Aritmética `Duration` com sinal; braço `Neg`; testes. |
| `01_core/src/engine/stdlib/primitives_constructors.rs` | Constructor aceita componentes negativos; `parse_duration` aceita prefixo `-`; testes. |
| `01_core/src/engine/eval/repr.rs` | `repr_duration` com componentes negativos (`seconds: -3`); testes. |
| `01_core/src/engine/eval/bindings.rs` | Sem alteração de código (já devolve `Float` assinado); teste novo. |
| `01_core/src/engine/eval/tests.rs` | Testes end-to-end para `repr(-duration(seconds: 3))`, `duration(seconds: -3)` e subtracção negativa; testes de overflow/underflow e sinal ajustados à representação `i128`. |
| `00_nucleo/prompts/entities/duration.md` | Actualizado com a representação com sinal; hash `cefa3eaa`. |

---

## 4. Detalhes de testes ajustados

Devido à mudança para `i128`:

- `duration_add_overflow` passou a usar `i128::MAX` nanossegundos ( `u64::MAX` já não overflow).
- `duration_sub_underflow` passou a testar `i128::MIN` (subtrair duração maior de menor agora produz valor negativo, não erro).
- `duration_mul_int_neg` e `duration_div_int_neg` passaram a esperar resultados negativos em vez de erro.
- `duration_named_overflow` do constructor renomeado para `duration_named_i64_max_ok`: com `i128` interno, `i64::MAX` segundos é válido.
- Corrigida expectativa de `repr` para duração de 25 horas negativas: `duration(days: -1, hours: -1)` (estava `-2`).

---

## 5. Validação

- `cargo test --workspace`: **4645 passed; 0 failed; 2 ignored** (typst-core lib), restantes crates verdes. Log em `temp/p850/cargo-test.log`.
- `crystalline-lint .`: **zero violations** (mantém-se o aviso pré-existente V7 de prompt órfão não relacionado). Log em `temp/p850/crystalline-lint.log`.

---

## 6. Próximo passo

O dono deve rever as alterações e fazer commit. Não há mais ações pendentes para o P850.
