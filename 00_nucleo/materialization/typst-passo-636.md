---
# P636 — Regras `#set` que ignoram tipo inválido sem erro

> **Passo:** 636
> **Data:** 2026-07-09
> **Foco:** P633 confirmou nove casos (itens 8–16) com o mesmo padrão: uma regra `#set` recebe um valor do tipo errado (por exemplo, `#set page(numbering: 123)`, um inteiro em vez de texto) e, em vez de produzir erro, ignora silenciosamente o valor, mantendo o comportamento anterior. Este passo corrige os nove de uma vez, por serem a mesma causa estrutural repetida em `rules.rs`.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P633 (onde os nove casos foram confirmados, cada um com teste já escrito).

---

## Contexto

Os nove casos confirmados por P633:

| # | Regra | Propriedade | Local |
|---|---|---|---|
| 8 | `#set math.equation` | `numbering` (variável indefinida) | `rules.rs:678` |
| 9 | `#set math.equation` | `numbering` (tipo inválido) | `rules.rs:695` |
| 10 | `#set figure` | `numbering` (tipo inválido) | `rules.rs:819` |
| 11 | `#set table` | `numbering` (tipo inválido) | `rules.rs:848` |
| 12 | `#set page` | `numbering` (tipo inválido) | `rules.rs:772` |
| 13 | `#set page` | `columns` (tipo inválido) | `rules.rs:785` |
| 14 | `#set text` | `weight` (tipo inválido) | `rules.rs:967` |
| 15 | `#set document` | `title` (tipo inválido) | `rules.rs:706` |
| 16 | `#set page` | `width` (tipo inválido) | `rules.rs:746` |

Todos partilham a mesma forma: a função auxiliar que converte o `Value` para o tipo esperado (`value_to_eco_string`, `extract_pt`, ou o equivalente para cada propriedade) devolve `None`/`.ok()` descarta o erro, e o caller trata isso como "não fazer nada", em vez de "isto é um erro do utilizador".

---

## Sonda

### Confirmar se há um padrão comum extraível

```bash
grep -n "fn value_to_eco_string\|fn extract_pt\|\.ok()\|unwrap_or" 01_core/src/rules/eval/rules.rs | head -30
```

Confirmar se as nove correcções podem partilhar uma única forma de reportar o erro (uma função auxiliar tipo `require_type(value, expected_type_name, property_name) -> Result<T, SourceDiagnostic>`), em vez de nove correcções ad-hoc.

### Confirmar a mensagem de erro do vanilla para cada caso

```bash
grep -n "expected.*got\|expected int\|expected string" lab/typst-original/crates/typst-library/src/**/*.rs 2>/dev/null | head -20
```

Confirmar o formato exacto da mensagem de erro do vanilla ("expected string, found integer", ou equivalente), para reproduzir o mesmo formato, não inventar um novo.

### Critério de fecho da sonda

- [ ] Confirmado se existe um padrão comum extraível para os nove casos.
- [ ] Formato da mensagem de erro do vanilla confirmado.

---

## Implementação

Se a sonda confirmar um padrão comum: criar uma função auxiliar única, usá-la nos nove sítios. Se os nove forem genuinamente diferentes o suficiente para não caber na mesma função: corrigir cada um individualmente, mas com a mesma disciplina de mensagem de erro clara.

### Critério de fecho da implementação

- [ ] Os nove casos produzem erro claro quando o tipo é inválido.
- [ ] Mensagem de erro no mesmo formato do vanilla (ou próxima, se o cristalino tiver convenção própria já estabelecida).
- [ ] Testes de P633 (`p633_set_equation_numbering_int_silent`, etc.) invertidos de "confirma falha silenciosa" para "confirma erro correcto".
- [ ] Uso correcto de cada regra `#set` (tipo certo) sem regressão.

---

## Validação

```bash
cargo test -p typst-core p633_set_ -- --nocapture
```

Confirmar que os testes já escritos por P633 agora esperam `Err`, não `Ok`.

```bash
cat > /tmp/p636-uso-correcto.typ <<'EOF'
#set page(numbering: "1")
#set text(weight: 700)
#set document(title: "Título correcto")
Texto.
EOF
./target/release/typst /tmp/p636-uso-correcto.typ /tmp/p636.pdf
```

Confirmar que uso correcto continua a funcionar sem erro.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa.
- [ ] Nove casos corrigidos, com erro claro.
- [ ] Testes de P633 invertidos e a passar.
- [ ] Uso correcto sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p636.md`, com hash do commit.

---

## Próximo passo

`counter.display` com argumentos inválidos (itens 18–23 de P633) — considerar, ao corrigir, unificar a lógica duplicada entre a chamada directa e o despacho de método sobre `Value::Counter`, já apontada por P633 como o mesmo bug em dois sítios.
