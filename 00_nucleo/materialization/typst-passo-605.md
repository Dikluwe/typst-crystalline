---
# P605 — Expor `bookmarked`/`outlined` como argumento de `heading()`

> **Passo:** 605
> **Data:** 2026-07-05
> **Foco:** P604 encontrou que o campo interno `HeadingElem::outlined` existe, mas a função `heading()` do avaliador não aceita nenhum argumento nomeado para o controlar. Isto significa que o utilizador não tem forma nenhuma de excluir um heading específico da árvore de bookmarks, uma capacidade que o vanilla tem (`bookmarked`, por defeito `true`). Este passo confirma o nome exacto do parâmetro no vanilla e implementa o equivalente.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P604 (onde o campo interno sem exposição foi encontrado), P602/603 (código de bookmarks, mesma área).

---

## Sonda

### Confirmar o nome e comportamento exacto do parâmetro no vanilla

```bash
cat > /tmp/p605-bookmarked.typ <<'EOF'
= Secção normal

#heading(bookmarked: false)[Secção sem bookmark]

= Outra secção normal
EOF
lab/typst-original/target/release/typst compile /tmp/p605-bookmarked.typ /tmp/p605-vanilla.pdf
mutool show /tmp/p605-vanilla.pdf outline
```

Confirmar se o nome do parâmetro é mesmo `bookmarked`, e se `outlined` é um parâmetro diferente com outro efeito (por exemplo, `outlined` pode controlar se aparece no `#outline()` do próprio documento, um índice diferente de `/Outlines` do PDF).

```bash
grep -n "bookmarked\|outlined" lab/typst-original/crates/typst-library/src/model/heading.rs 2>/dev/null | head -20
```

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p605-bookmarked.typ /tmp/p605-cristalino.pdf
```

Confirmar o erro exacto (provavelmente "unexpected argument" ou equivalente).

### Critério de fecho da sonda

- [ ] Nome exacto do parâmetro confirmado (`bookmarked`, e separadamente `outlined` se forem coisas diferentes).
- [ ] Comportamento de cada um confirmado — `/Outlines` do PDF vs índice interno do documento (`#outline()`), se forem mesmo separados.
- [ ] Localizado, com `file:line`, onde `heading()` é definida no avaliador do cristalino.

---

## Implementação

Adicionar o(s) parâmetro(s) nomeados à função `heading()` no avaliador (`01_core/src/rules/eval/rules.rs` ou onde as funções nativas são definidas), ligando ao campo interno `HeadingElem::outlined` já existente. Se `bookmarked` e `outlined` forem parâmetros distintos no vanilla, com efeitos diferentes, implementar os dois separadamente, não assumir que são a mesma coisa.

### Critério de fecho da implementação

- [ ] Parâmetro(s) reconhecidos por `heading()`.
- [ ] Heading com `bookmarked: false` (ou nome confirmado) não aparece em `/Outlines` do PDF.
- [ ] Se `outlined` for separado: testado o seu efeito próprio, distinto de `bookmarked`.
- [ ] Headings sem o parâmetro continuam a aparecer nos bookmarks como antes (valor por defeito `true`).

---

## Validação

```bash
./target/release/typst /tmp/p605-bookmarked.typ /tmp/p605-depois.pdf
mutool show /tmp/p605-depois.pdf outline
```

Comparar directamente com o resultado do vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Parâmetro(s) implementados e testados contra o vanilla.
- [ ] Sem regressão nos testes de bookmarks já existentes (P602/603/604).
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p605.md`, com hash do commit.
- [ ] Listas de disparidades actualizadas.
