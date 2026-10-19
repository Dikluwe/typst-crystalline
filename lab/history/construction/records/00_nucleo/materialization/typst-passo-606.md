---
# P606 — Separar `outlined` de `bookmarked` como duas flags distintas

> **Passo:** 606
> **Data:** 2026-07-05
> **Foco:** P605 implementou os dois parâmetros na sintaxe, mas ambos afectam a mesma flag interna (`HeadingElem::outlined`). No vanilla, são duas coisas separadas: `outlined` controla o índice do próprio documento (`#outline()`); `bookmarked` controla a árvore `/Outlines` do PDF. Um utilizador não consegue hoje, no cristalino, ter um heading no índice impresso mas fora dos bookmarks do leitor de PDF, ou o inverso. Este passo separa as duas.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M. Toca `Content::Heading`, `HeadingElem`, `introspect.rs`, e a geração de `/Outlines` — mais do que um ficheiro.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P605 (onde a limitação foi declarada e aceite como está, para o caso comum), P602/603/604 (código de bookmarks e índice).

---

## Sonda

### Confirmar os quatro casos possíveis no vanilla

```bash
cat > /tmp/p606-quatro-casos.typ <<'EOF'
#outline()

= Caso A — outlined true, bookmarked true (por defeito)

#heading(outlined: false)[Caso B — só bookmarked]

#heading(bookmarked: false)[Caso C — só outlined]

#heading(outlined: false, bookmarked: false)[Caso D — nenhum dos dois]
EOF
lab/typst-original/target/release/typst compile /tmp/p606-quatro-casos.typ /tmp/p606-vanilla.pdf
pdftotext /tmp/p606-vanilla.pdf - | head -10
mutool show /tmp/p606-vanilla.pdf outline
```

Confirmar, para cada um dos quatro casos: aparece no índice impresso (`#outline()`)? Aparece nos bookmarks do PDF (`/Outlines`)?

Esperado, a confirmar:
- Caso A: nos dois.
- Caso B (`outlined: false`): nem no índice nem nos bookmarks (porque `bookmarked` segue `outlined` quando não definido explicitamente, per a documentação encontrada em P605).
- Caso C (`bookmarked: false`): no índice, não nos bookmarks.
- Caso D: em nenhum dos dois.

Esta relação (bookmarked segue outlined por defeito, mas pode ser definido à parte) precisa de ficar clara antes de implementar.

### Critério de fecho da sonda

- [x] Os quatro casos confirmados directamente contra o vanilla, não assumidos.
- [x] Confirmado, com comportamento observado, que `bookmarked: auto` segue `outlined` por defeito.

---

## Implementação

- `HeadingElem` ganha dois campos separados: `outlined: bool` e `bookmarked: Option<bool>` (ou `Smart<bool>`, seguindo o padrão já usado no vanilla).
- Quando `bookmarked` não é definido pelo utilizador, o valor efectivo segue `outlined`.
- `introspect.rs` usa `outlined` para decidir se entra no índice do documento (`#outline()`).
- A geração de `/Outlines` (código já tocado em P602/603) usa o valor efectivo de `bookmarked` (explícito, ou `outlined` como fallback), não `outlined` directamente.

### Critério de fecho da implementação

- [x] Os quatro casos testados em código, confirmando o comportamento esperado de cada um.
- [x] `heading()` sem nenhum dos dois parâmetros continua a funcionar como antes (regressão zero para o caso comum).
- [x] Testes de P605 (`bookmarked: false` sozinho) continuam a passar, agora com o comportamento correcto e não só coincidente.

---

## Validação

```bash
./target/release/typst /tmp/p606-quatro-casos.typ /tmp/p606-cristalino.pdf
pdftotext /tmp/p606-cristalino.pdf - | head -10
mutool show /tmp/p606-cristalino.pdf outline
```

Comparar os quatro casos, directamente, com o resultado já obtido do vanilla.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [x] Sonda completa, quatro casos confirmados no vanilla.
- [x] `outlined` e `bookmarked` implementados como campos separados, com a relação de fallback correcta.
- [x] Os quatro casos testados e a bater com o vanilla.
- [x] Sem regressão nos testes de P602 a P605.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p606.md`, com hash do commit.
- [x] Entrada da lista de disparidades para `bookmarked`/`outlined` actualizada de "corrigido parcialmente" para "corrigido", ou mantida separada se a limitação for maior do que P606 conseguir fechar.
