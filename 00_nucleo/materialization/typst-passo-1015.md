# Passo 1015 — Dedup: três cópias independentes de `long_type_name`

**Tipo**: Correcção de dívida pequena, não fatiamento. Achado lateral do Passo 1013.
**Motivo**: `long_type_name` existe em três sítios sem relação declarada entre si:
- `01_core/src/compiler/eval/bindings/access.rs:34` — `pub(crate)`
- `01_core/src/compiler/eval/operators/join.rs:86` — privada
- `01_core/src/compiler/stdlib/foundations.rs:1692` — privada

O L0 de `operators/join.md` documenta a sua cópia como se fosse a única implementação —
está desactualizado desde que a duplicação existe (data a confirmar na Fase A, não
presumir que começou com o P1013).

**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1014.

---

## Fase A — Confirmar que são mesmo idênticas antes de eliminar

```bash
sed -n '/fn long_type_name/,/^}/p' 01_core/src/compiler/eval/bindings/access.rs
sed -n '/fn long_type_name/,/^}/p' 01_core/src/compiler/eval/operators/join.rs
sed -n '/fn long_type_name/,/^}/p' 01_core/src/compiler/stdlib/foundations.rs
```

Comparar corpo a corpo. **Não presumir que são idênticas só porque têm o mesmo nome** —
podem ter divergido silenciosamente desde que foram duplicadas (é exactamente o tipo de
deriva que a duplicação sem ponto único de verdade permite). Se divergirem, este passo
pára e reporta a divergência em vez de eliminar às cegas — decidir qual versão é a
correcta fica para o dono, não presumido aqui.

## Fase B — Se idênticas: um ponto único de verdade

`bindings/access.rs` já é `pub(crate)` — candidato natural a única definição.

1. Remover as cópias de `operators/join.rs` e `stdlib/foundations.rs`.
2. Importar `bindings::access::long_type_name` (ajustar caminho ao real, confirmar
   visibilidade) nos dois sítios que a usavam localmente.
3. Corrigir `operators/join.md` para não afirmar posse da função — referenciar
   `bindings/access.md` como o dono.

## Fase C — Validar

```
crystalline-lint .
cargo test --workspace
```
Zero regressão — é remoção de duplicação, não mudança de comportamento.

---

## Resultado esperado

Uma implementação de `long_type_name`, dois pontos de chamada, L0 dos dois consumidores
corrigido para não afirmar posse. Se as três divergiam, relatório com a divergência
encontrada, sem eliminar nada, para decisão do dono.
