# Relatório — typst-passo-851: `stack(dir:)` aceita `direction` em vez de `string`

**Data:** 2026-07-23  
**Executor:** Kimi Code (agente principal).  
**Proveniência das medições:** working tree não commitado (sobre HEAD `dfe3c2282`).  
**Estado:** **implementado e validado**.

---

## 1. Problema

`#stack(dir: ltr, ...)` divergia do vanilla:

- **Vanilla:** aceita `ltr`/`rtl`/`ttb`/`btt` como valores do tipo `direction`.
- **Cristalino (antes):** `error: stack(dir:) deve ser string, recebeu direction`.
- Inverso: `#stack(dir: "ltr", ...)` — cristalino aceitava, vanilla rejeitava.

Causa: `native_stack` validava `dir:` como `Value::Str`.

---

## 2. Sonda

Código identificado:

- `01_core/src/engine/stdlib/layout.rs:1209` — `extract_dir` coagia só `Value::Str`.
- `01_core/src/engine/stdlib/layout.rs:1237` — `native_stack` usava `extract_dir`.
- O tipo `direction` já existia: `Value::Dir(Dir)` (`01_core/src/entities/value.rs:149`) e as constantes `ltr`/`rtl`/`ttb`/`btt` estavam no scope global (`01_core/src/engine/eval/mod.rs:1961-1964`).

---

## 3. Implementação

Alterações:

- `01_core/src/engine/stdlib/layout.rs:1209` — `extract_dir` passou a aceitar `Value::Dir(d)` e a rejeitar strings (paridade vanilla).
- Actualização da docstring de `native_stack` para refletir `dir: direction`.
- Testes actualizados em `01_core/src/engine/stdlib/mod.rs`:
  - `native_stack_aceita_dir_ltr` e `native_stack_aceita_todas_4_direcoes` passaram a usar `Value::Dir(...)`.
  - Adicionado `native_stack_rejeita_dir_string`.
  - `native_stack_combina_dir_spacing_children` passou a usar `Value::Dir(Dir::LTR)`.

---

## 4. Validação

- `cargo test --workspace`: **4649 passed; 0 failed** (estado após P851–P853).
- `crystalline-lint .`: exit 0 (zero violations relevantes).

Comportamento observável:

- `#stack(dir: ltr, [a], [b])` → aceite.
- `#stack(dir: "ltr", [a], [b])` → rejeitado.

---

## 5. Próximo passo

O dono deve rever e fazer commit. Não há mais ações pendentes para o P851.
