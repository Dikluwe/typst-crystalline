# Paridade produção — P695 — `repr` de dict vazio `(:)` vs array vazio `()`

**Commit deste passo:** `__P695_COMMIT__` (preenchido no 2º commit; ver §Proveniência).
**Passo:** `00_nucleo/materialization/typst-passo-695.md`. **Tamanho:** XS.
**ADRs:** ADR-0108 (medir antes de decidir), ADR-0107 (paridade com a linguagem).
**Origem:** P694 classificou `repr((:)) == "()"` como diferença "mecânica de
formatação"; P695 testa se `repr(())` (array vazio) **também** é `()` — o que
a tornaria uma **ambiguidade real** (dois tipos, mesmo texto de depuração).

---

## Proveniência das medições (regra de proveniência)

- **Estado medido (sonda "antes"):** working tree em `HEAD = 53d3747b5f4ee7dde847c9adea25cad61b838fc8` (P694).
- **Estado medido (após fix) e testes:** working tree **não commitado** sobre a
  mesma base, hora `2026-07-10T21:25:28-03:00` (data do ambiente).
- **Ficheiros alterados no momento da medição** (`git diff HEAD --stat`):

```
 00_nucleo/prompts/rules/stdlib/foundations.md |  6 ++++--
 01_core/src/rules/eval/repr.rs                | 16 ++++++++++++++++
 01_core/src/rules/stdlib/foundations.rs       |  2 +-   (hash)
 3 files changed, 21 insertions(+), 3 deletions(-)
```

`01_core/src/rules/stdlib/foundations.rs` é propagação de `@prompt-hash` pelo
`crystalline-lint --fix-hashes` (edição de `foundations.md`), sem mudança de
lógica. Novo ficheiro: este relatório.

---

## Verificação (sonda directa)

Fixture (`#repr(())` / `#repr((:))` / `#(() == (:))`), texto via `pdftotext`:

| Ferramenta | `repr(())` | `repr((:))` | `(()) == (:)` | Conclusão |
|------------|-----------|-------------|---------------|-----------|
| vanilla 0.15.0 | `()` | `(:)` | `false` | distingue |
| cristalino **antes** | `()` | `()` | `false` | **ambiguidade** — texto igual, tipos diferentes |
| cristalino **depois** | `()` | `(:)` | `false` | distingue (igual ao vanilla) |

A comparação `(()) == (:)` ser `false` em ambos confirma que os **tipos** já eram
distintos no cristalino; o defeito era só no **texto** do `repr`. Mesmo assim,
dois tipos diferentes a produzir o mesmo texto de depuração (`()`) é uma
ambiguidade real para quem lê `repr` para depurar — logo deixa de ser "mecânica
cosmética" e passa a ser correcção de clareza necessária (decisão do passo).

## Decisão

Corrigir `repr` de **dict vazio** para `"(:)"`, mantendo array vazio `"()"` —
paridade com o vanilla, desambiguando os dois tipos. Alteração cirúrgica em
`rules/eval/repr.rs::repr_value` (braço `Value::Dict`): `if dict.is_empty() {
return "(:)".into() }`. Dicts não-vazios (`(a: 1)`) e arrays (incluindo vazio
`()`) ficam inalterados.

**Língua vs mecânica (ADR-0107):** a forma textual do `repr` é mecânica, mas a
**distingibilidade** de dois tipos no texto de depuração é propriedade
observável da linguagem (quem depura confia nela). A correcção é aceite ao
nível da linguagem, não como alinhamento de bytes.

## Testes e lint

- Novo teste `repr_value_empty_array_e_dict_distinguem` (em `rules/eval/repr.rs`)
  trava: `repr(()) == "()"`, `repr((:)) == "(:)"`, e os dois textos diferem.
- `cargo test --workspace`: **4386 passed / 0 failed** (3712 + 610 + 33 + 2 + 27
  + 2). P694 era 4385; o delta +1 é o teste novo. **Sem regressão.**
- `crystalline-lint .`: **0 violations** (após `--fix-hashes`).

## Limitações / notas

- Não foi tocado o repr de array de um elemento (`(1)` vs vanilla `(1,)` com
  vírgula): fora do escopo de P695 e não ambíguo com dict (arrays não usam `:`).
- O defeito era exclusivamente o caso **vazio** do dict.
