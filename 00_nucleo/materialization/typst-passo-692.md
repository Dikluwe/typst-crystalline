---
# P692 — `str.matches()` (plural) e `str.normalize()`, ausentes por completo

> **Passo:** 692
> **Data:** 2026-07-10
> **Foco:** A documentação oficial do Typst confirma dois métodos de `str` que nunca apareceram em nenhum passo desta conversa: `matches()` (devolve array com todas as ocorrências de um padrão, ao contrário de `match()`, singular, já implementado em P689) e `normalize()` (normalização Unicode). Isto não é comportamento errado — é ausência total, o mesmo tipo de gap que a Parte 3 de P664 pediu para procurar sistematicamente e nunca chegou a cobrir para `str`.
> **Tipo:** Sonda mínima + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P689 (onde `match()` singular e a infra-estrutura de `RegexMatch` já foram construídas, reaproveitável para `matches()`).

---

## Sonda mínima

### Confirmar o comportamento exacto de `matches()`

```bash
cat > /tmp/p692-matches.typ <<'EOF'
#("um dois três quatro").matches(regex("\\w+"))
#("abc").matches(regex("z"))
#("a1b2c3").matches(regex("[a-z]"))
EOF
lab/typst-original/target/release/typst compile /tmp/p692-matches.typ /tmp/p692-vanilla.pdf
pdftotext /tmp/p692-vanilla.pdf -
```

Confirmar: `matches()` aceita só regex, ou também string? Cada elemento do array devolvido tem a mesma estrutura de `match()` (`start`/`end`/`text`/`captures`)?

### Confirmar o comportamento exacto de `normalize()`

```bash
cat > /tmp/p692-normalize.typ <<'EOF'
#("café").normalize()
#("café").normalize(form: "nfd")
#("café").normalize(form: "nfc")
EOF
lab/typst-original/target/release/typst compile /tmp/p692-normalize.typ /tmp/p692-normalize-vanilla.pdf
```

Confirmar as formas de normalização aceites (`nfc`, `nfd`, `nfkc`, `nfkd`), qual é o default, e se o resultado visual muda ou só a representação interna (que só seria visível através de outros métodos como `len()`, já que caracteres compostos podem ter contagens de bytes diferentes consoante a forma).

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p692-matches.typ /tmp/p692-matches-cristalino.pdf
echo "Exit code: $?"
./target/release/typst /tmp/p692-normalize.typ /tmp/p692-normalize-cristalino.pdf
echo "Exit code: $?"
```

### Critério de fecho da sonda mínima

- [ ] Comportamento de `matches()` confirmado, incluindo estrutura de cada elemento do array.
- [ ] Comportamento de `normalize()` confirmado, incluindo formas aceites e default.

---

## Implementação

### `matches()`

Reaproveitar `Regex::captures_first` (P689) generalizando para `Regex::captures_all` (ou equivalente), devolvendo um array de `RegexMatch` em vez de um só.

### `normalize()`

Implementar as quatro formas de normalização Unicode padrão (NFC, NFD, NFKC, NFKD) — confirmar se existe já uma dependência no projecto capaz disto (crate `unicode-normalization` ou equivalente), ou se precisa de ser adicionada.

### Critério de fecho da implementação

- [ ] `matches()` implementado, devolvendo array de dicionários com a mesma estrutura de `match()`.
- [ ] `normalize()` implementado, com as formas confirmadas pela sonda.
- [ ] Métodos existentes de `str` sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p692-matches.typ /tmp/p692-matches-depois.pdf
pdftotext /tmp/p692-matches-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

```bash
./target/release/typst /tmp/p692-normalize.typ /tmp/p692-normalize-depois.pdf
pdftotext /tmp/p692-normalize-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda mínima completa, os dois métodos confirmados contra o vanilla.
- [ ] `matches()` implementado e testado.
- [ ] `normalize()` implementado e testado.
- [ ] Métodos existentes sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p692.md`, com hash do commit.
- [ ] Lista de métodos de `str` actualizada — confirmar que, depois deste passo, não fica nenhum outro método da documentação oficial por implementar.
