---
# P604 — Bookmarks a partir de labels manuais

> **Passo:** 604
> **Data:** 2026-07-05
> **Foco:** O cristalino só gera entradas na árvore `/Outlines` a partir de headings. O vanilla permite também marcar qualquer ponto do documento com um `<label>` e usar `#outline(target: ...)` ou mecanismo equivalente para incluir esse ponto na árvore de bookmarks, sem ser um heading. Registado em P535 como "não suporta labels manuais", sem razão além da ausência. Este passo confirma o mecanismo exacto do vanilla e implementa o equivalente.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P535 (implementação original de bookmarks, só a partir de headings), P602/603 (correcção de `/Count`, mesma área de código).

---

## Sonda

### Confirmar o mecanismo exacto do vanilla

```bash
cat > /tmp/p604-labels.typ <<'EOF'
= Secção com heading

Texto normal. #label("ponto-manual") Aqui está um ponto marcado manualmente, sem ser um heading.

Mais texto.
EOF
lab/typst-original/target/release/typst compile /tmp/p604-labels.typ /tmp/p604-vanilla.pdf
mutool show /tmp/p604-vanilla.pdf outline
```

Confirmar se um `#label(...)` sozinho já aparece na árvore de bookmarks do vanilla, ou se precisa de mecanismo adicional (por exemplo, `#bookmark(...)`, ou uma função própria) para entrar explicitamente na árvore.

```bash
grep -rn "bookmark\|outline.*target\|fn.*outline" lab/typst-original/crates/typst-library/src/ 2>/dev/null | grep -i "bookmark\|manual" | head -20
```

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p604-labels.typ /tmp/p604-cristalino.pdf
mutool show /tmp/p604-cristalino.pdf outline
```

### Critério de fecho da sonda

- [ ] Mecanismo exacto do vanilla confirmado (nome da função/sintaxe, e se `#label` sozinho basta ou precisa de algo mais).
- [ ] Estado actual do cristalino confirmado (provavelmente ignora labels avulsos para efeitos de bookmark).
- [ ] Localizado, com `file:line`, onde o cristalino decide o que entra na árvore de `/Outlines` hoje (deve ser algo que só olha para `ElementKind::Heading`, a confirmar).

---

## Implementação

Depende da sonda. Se o mecanismo do vanilla for uma função própria (não só `#label`), implementar o reconhecimento dessa função no avaliador, e estender a construção da árvore de `/Outlines` (`emit_outlines`, já tocada em P602/603) para incluir também estes pontos manuais, na posição correcta relativa aos headings (por ordem de aparição no documento, não só por nível de heading).

### Critério de fecho da implementação

- [ ] Sintaxe do vanilla reconhecida no cristalino.
- [ ] Ponto manual aparece na árvore de bookmarks, na posição certa (por ordem de aparição, misturado com headings se for o caso).
- [ ] Testado com documento que mistura headings e pontos manuais, confirmando a ordem e a hierarquia.
- [ ] Documentos só com headings (o caso já testado em P602/603) sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p604-labels.typ /tmp/p604-depois.pdf
mutool show /tmp/p604-depois.pdf outline
lab/typst-original/target/release/typst compile /tmp/p604-labels.typ /tmp/p604-vanilla-comparar.pdf
mutool show /tmp/p604-vanilla-comparar.pdf outline
```

Comparar directamente as duas árvores.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Mecanismo do vanilla confirmado e reproduzido.
- [ ] Árvore de bookmarks com pontos manuais e headings misturados, testada contra o vanilla.
- [ ] Sem regressão nos testes de P602/603.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p604.md`, com hash do commit.
- [ ] Listas de disparidades actualizadas.
