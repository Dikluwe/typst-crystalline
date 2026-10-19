# Relatório de Verificação — Passo 794: Paridade de Smartquote

**Data:** 2026-07-21  
**Status:** Concluído com Sucesso  
**Proveniência da Medição:**
- **Commit Base:** `f0db5cd20` (working tree não commitado)
- **Modificações na Working Tree (`git diff HEAD --stat`):**
  ```
  01_core/src/engine/eval/mod.rs          |  76 +++++----
  01_core/src/engine/eval/rules.rs        |  55 ++++++-
  01_core/src/engine/eval/tests.rs        | 145 ++++++++++++++++-
  01_core/src/engine/lang/quotes.rs       |   6 +-
  01_core/src/engine/layout/enum_item.rs  |  20 ++-
  01_core/src/engine/layout/mod.rs        |   3 +
  01_core/src/engine/layout/sequence.rs   |  13 +-
  01_core/src/engine/layout/tests.rs      |  22 ++-
  01_core/src/engine/stdlib/mod.rs        |   4 +-
  01_core/src/engine/stdlib/structural.rs | 263 +++++++++++++++++++++++++++++++-
  10 files changed, 552 insertions(+), 55 deletions(-)
  ```
- **Hora da Medição:** 2026-07-21T02:47:26Z (UTC)

---

## 1. Verificação 1 — Aspas Duplas Curvas por Padrão

### Sondas Efetuadas:
Confirmada a paridade com o compilador vanilla de que as aspas duplas `"..."` devem virar aspas tipográficas curvas `“...”` por padrão (mesmo que o idioma não esteja especificado ou seja desconhecido).

**Crystalline markup:**
```typst
"test"
```
- **Vanilla output:** `“test”`
- **Crystalline output:** `“test”`
- **Status:** Sucesso. O fallback `DEFAULT_QUOTES` foi alterado em `quotes.rs` para `("\u{201C}", "\u{201D}")`.

---

## 2. Verificação 2 — Suporte de `#set smartquote(...)`

### Sondas Efetuadas:
Conectadas as propriedades `enabled` e `quotes` do target `smartquote` à `StyleChain` no layout/eval, cessando o aviso `"target 'smartquote' ainda não suportado"`.

**Caso 1 (enabled: false):**
```typst
#set smartquote(enabled: false)
"test" e 'single'
```
- **Vanilla output:** `"test" e 'single'`
- **Crystalline output:** `"test" e 'single'`
- **Status:** Sucesso.

**Caso 2 (quotes: "«»"):**
```typst
#set smartquote(quotes: "«»")
"test" e 'single'
```
- **Vanilla output:** `«test» e ‘single’`
- **Crystalline output:** `«test» e ‘single’`
- **Status:** Sucesso (aspas duplas configuradas para chevrons, aspas simples continuam usando o default).

---

## 3. Verificação 3 — Validação de `quotes:`

### Sondas Efetuadas:
Tentativa de setar `quotes:` com string de tamanho incorreto (diferente de 2 caracteres) deve retornar um erro de compilação apropriado.

```typst
#set smartquote(quotes: "abc")
```
- **Vanilla compilation error:** `expected 2 characters, found 3 characters`
- **Crystalline compilation error:** `expected 2 characters, found 3 characters`
- **Status:** Sucesso.

---

## 4. Testes Automatizados Persistidos

Para garantir o funcionamento contínuo de `smartquote` em face de regressões, foram criados 4 novos testes unitários isolados no codebase (situados em [01_core/src/engine/eval/tests.rs](file:///home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core/src/engine/eval/tests.rs)):
- `p794_smartquote_double_curved_default`: Valida a transformação padrão de aspas duplas `"..."` em curvas tipográficas.
- `p794_smartquote_enabled_false`: Valida que aspas permanecem retas quando `enabled: false`.
- `p794_smartquote_quotes_custom`: Valida a aplicação de aspas duplas customizadas quando configurado via `quotes: "«»"`.
- `p794_smartquote_quotes_validation`: Garante que tentativas de passar strings com mais ou menos de 2 caracteres resultem no erro esperado.

---

## 5. Verificação 5 — Workspace `cargo test`

Todos os testes unitários e de integração do workspace foram executados com **sucesso (zero falhas)**. A contagem total na suíte core subiu de **4303** para **4307** após a persistência dos 4 novos testes automatizados inseridos:

```
1. Suite 'typst-core' (lib):
   test result: ok. 4307 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.58s

2. Suite 'typst-infra' (lib):
   test result: ok. 655 passed; 0 failed; 5 ignored; 0 measured; 0 filtered out; finished in 1.40s

3. Suite 'typst-shell' (lib):
   test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

4. Suite 'typst' (CLI bin):
   test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

5. Suite 'tests/cli.rs' (CLI integration):
   test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.76s

6. Suite 'tests/crystalline_lint.rs' (Linter rules):
   test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

## 6. Linter de Linhagem `crystalline-lint`

O linter `crystalline-lint .` foi executado e retornou **zero violações** de desvio de linhagem (drift) ou inconsistência nos arquivos modificados.
