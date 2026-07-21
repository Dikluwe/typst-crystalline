# Relatório de Verificação — Passo 795: Math/symbol scope

**Data:** 2026-07-21  
**Status:** Concluído com Sucesso  
**Proveniência da Medição:**
- **Commit Base:** `070c1cec7` (working tree não commitado)
- **Modificações na Working Tree (`git diff HEAD --stat`):**
  ```
  01_core/src/engine/eval/math.rs         | 34 +++++++++++++++++++++------------
  01_core/src/engine/eval/tests.rs        | 29 ++++++++++++++++++++++++++++
  01_core/src/engine/stdlib/mod.rs        |  6 +++---
  01_core/src/engine/stdlib/structural.rs |  4 ++++
  01_core/src/engine/stdlib/sym.rs        | 10 +++++++++-
  5 files changed, 67 insertions(+), 16 deletions(-)
  ```
- **Hora da Medição:** 2026-07-21T02:57:52Z (UTC)

---

## 1. Verificação 1 — Modificador `neq` para `subset`

### Sondas Efetuadas:
Confirmada a paridade com o compilador vanilla de que `#repr(sym.subset.neq)` deve retornar `symbol("⊊")` em vez do erro `unknown symbol modifier 'neq'`.

- **Vanilla output:** `symbol("⊊")`
- **Crystalline output:** `symbol("⊊")`
- **Status:** Sucesso. O símbolo `subset` foi movido de `SYM_SIMPLE` para `SYM_GROUPS` contendo a função de variantes `subset_variants` (`eq` -> `⊆`, `neq` -> `⊊`).

---

## 2. Verificação 2 — Identificadores de Símbolo Bare em Modo Math (`arrow.r`)

### Sondas Efetuadas:
Símbolos e grupos do módulo `sym` sem o prefixo `#` ou `sym.` (como `arrow`) agora resolvem diretamente a partir do escopo estático de símbolos no modo matemático.

**Crystalline markup:**
```typst
$ arrow.r $
```
- **Vanilla output:** `→` (MathText)
- **Crystalline output:** `→` (MathText)
- **Status:** Sucesso. Adicionado fallback para `sym_lookup` tanto em `Expr::MathIdent` (dentro de `eval_math_expr`) quanto em `eval_math_callee` (quando acessado como target de um `FieldAccess`).

---

## 3. Verificação 3 — Operadores `dif` e `Dif`

### Sondas Efetuadas:
Os operadores `dif` e `Dif` (usados para diferenciais como em `$ integral x dif x $`) agora são expostos no módulo `math` e avaliados como `Content::MathText` upright (reto).

**Crystalline markup:**
```typst
$ integral x dif x $
```
- **Vanilla output:** `∫ x d x`
- **Crystalline output:** `∫ x d x`
- **Status:** Sucesso. Os operadores foram inseridos no dicionário de `make_math_module()` em `structural.rs` e a função `lookup_math_op` foi generalizada para retornar qualquer `Content` mapeado (e não apenas `MathOp`), permitindo resolver `MathText` diretamente.

---

## 4. Testes Automatizados Persistidos

Para garantir o funcionamento contínuo e evitar regressões futuras, foram criados 3 novos testes unitários nominais em [`01_core/src/engine/eval/tests.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/engine/eval/tests.rs):
- `p795_sym_subset_neq`: Valida que a variante `neq` do símbolo `subset` retorna o caractere correto (`⊊`) sob `repr()`.
- `p795_math_arrow_r_bare`: Valida que o identificador bare `arrow.r` no modo matemático é resolvido com sucesso para a seta para a direita (`→`).
- `p795_math_dif_bare`: Valida a resolução dos operadores `dif` e `Dif` no modo matemático para os caracteres upright `d` e `D` respectivamente.

O teste unitário `p299_math_module_total_42_operadores` em [`01_core/src/engine/stdlib/mod.rs`](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/engine/stdlib/mod.rs) também foi atualizado para contemplar a nova contagem de operadores (46) com a adição de `dif` e `Dif`.

---

## 5. Verificação de Sucesso do Workspace

Todos os testes unitários e de integração foram executados com sucesso (zero falhas). A contagem da suíte core subiu para **4310 passed**:

```
1. Suite 'typst-core' (lib):
   test result: ok. 4310 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.51s

2. Suite 'typst-infra' (lib):
   test result: ok. 655 passed; 0 failed; 5 ignored; 0 measured; 0 filtered out; finished in 1.40s

3. Suite 'typst-shell' (lib):
   test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

4. Suite 'typst' (CLI bin):
   test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

5. Suite 'tests/cli.rs' (CLI integration):
   test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.74s

6. Suite 'tests/crystalline_lint.rs' (Linter rules):
   test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`crystalline-lint .` reportou zero violações de linhagem.
