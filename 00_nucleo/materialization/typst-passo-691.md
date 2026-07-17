---
# P691 — `str.find()` deve devolver substring/`none`, não um índice

> **Passo:** 691
> **Data:** 2026-07-10
> **Foco:** P690 encontrou, ao classificar todos os métodos de `str`, que `find()` do cristalino devolve um `int` (índice em bytes), enquanto o vanilla devolve a substring encontrada (`str | none`). É a mesma classe de violação corrigida em P690 para `len`/`at`/`slice` — um nome igual ao do vanilla, com comportamento diferente — só que aqui a diferença é o próprio tipo de retorno, não byte contra carácter.
> **Tipo:** Sonda mínima + Implementação.
> **Tamanho:** S-M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P690 (onde a diferença foi encontrada e correctamente separada como fora de alcance), a regra de P662-P664 (nome igual obriga a comportamento igual).

---

## Sonda mínima

### Confirmar o comportamento exacto do vanilla

```bash
cat > /tmp/p691-find.typ <<'EOF'
#("café mais texto").find("mais")
#("café mais texto").find("inexistente")
#("café mais texto").find(regex("m..s"))
EOF
lab/typst-original/target/release/typst compile /tmp/p691-find.typ /tmp/p691-vanilla.pdf
pdftotext /tmp/p691-vanilla.pdf -
```

Confirmar: `find` aceita só string, ou também regex (como `position`)? O que devolve exactamente quando encontra — a substring completa da correspondência?

### Confirmar quantos call-sites internos dependem do comportamento actual (índice)

```bash
grep -rn "\.find(" 01_core/src/ 03_infra/src/ --include="*.rs" | grep -v test
```

Mesma verificação de risco já feita por P690 para `len`/`at`/`slice` — confirmar se `find` é usado só pelo próprio módulo, ou se há dependências internas no comportamento actual (índice) que precisam de ser actualizadas junto.

### Critério de fecho da sonda mínima

- [ ] Comportamento exacto de `find` confirmado (tipos aceites, valor devolvido).
- [ ] Alcance de call-sites internos confirmado.

---

## Implementação

Alterar `find()` para devolver `Value::Str` (a substring encontrada) ou `Value::None`, em vez de `Value::Int`. Se `find` aceitar regex além de string (a confirmar pela sonda), seguir o mesmo padrão já usado em `position`/`match` (P689).

### Critério de fecho da implementação

- [ ] `find()` devolve a substring encontrada, ou `none`, testado contra o vanilla.
- [ ] Call-sites internos (se algum depender do comportamento antigo) actualizados.
- [ ] Testes de regressão para os casos já cobertos pela sonda.

---

## Validação

```bash
./target/release/typst /tmp/p691-find.typ /tmp/p691-depois.pdf
pdftotext /tmp/p691-depois.pdf -
```

Comparar com o resultado do vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda mínima completa.
- [ ] `find()` corrigido, testado contra o vanilla.
- [ ] Call-sites internos revistos, sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p691.md`, com hash do commit.
