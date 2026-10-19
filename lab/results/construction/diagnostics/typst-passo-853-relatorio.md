# Relatório — typst-passo-853: `ref()` aceita `label`

**Data:** 2026-07-23  
**Executor:** Kimi Code (agente principal).  
**Proveniência das medições:** working tree não commitado (sobre HEAD `dfe3c2282`).  
**Estado:** **implementado e validado**.

---

## 1. Problema

`#ref(<label>)` divergia do vanilla:

- **Vanilla:** aceita uma label directamente como argumento.
- **Cristalino (antes):** `error: ref() espera nome como string, recebeu label`.

Causa: `native_ref` validava o primeiro argumento como `Value::Str` e rejeitava `Value::Label`.

---

## 2. Sonda

Código identificado:

- `01_core/src/engine/stdlib/ref.rs:22` — `native_ref` só aceitava `Value::Str`.
- `01_core/src/entities/label.rs:12` — `Label(pub String)` guarda o nome da label.
- `01_core/src/entities/content.rs:2078` — `Content::reference_with_supplement` já recebia `EcoString` como nome.

A resolução da referência (busca do alvo numerado, renderização do número) já é feita pelo layout a partir de `Content::Ref`; bastava aceitar o tipo `Label` no constructor.

---

## 3. Implementação

Alterações:

- `01_core/src/engine/stdlib/ref.rs:28` — `native_ref` passou a aceitar `Value::Label(label)`, convertendo `label.0` para `EcoString`.
- Mensagem de erro actualizada para "ref() espera label ou string, recebeu ...".
- Testes adicionados em `01_core/src/engine/stdlib/ref.rs`:
  - `native_ref_aceita_label`
  - `native_ref_arg_nao_label_nem_string_erro` (renomeado do teste anterior).

---

## 4. Validação

- `cargo test --workspace`: **4649 passed; 0 failed** (estado após P851–P853).
- `crystalline-lint .`: exit 0 (zero violations relevantes).

Comportamento observável:

- `#ref(<sec1>)` → aceite; produz `Content::Ref { name: "sec1", ... }`.
- `#ref("sec1")` → continua aceite.
- `#ref(42)` → rejeitado.

Nota: a validação pós-resolução (alvo não numerado, label inexistente) continua a ser responsabilidade do layout/introspector, tal como no vanilla.

---

## 5. Próximo passo

O dono deve rever e fazer commit. Não há mais ações pendentes para o P853.
