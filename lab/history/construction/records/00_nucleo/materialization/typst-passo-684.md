---
# P684 — Remover `pre`/`build` de `version()`, confirmado como confusão com SemVer

> **Passo:** 684
> **Data:** 2026-07-10
> **Foco:** Investigação directa (documentação oficial do Typst + histórico do próprio projecto) confirmou a origem do erro. O tipo `version` do Typst é "uma sequência de componentes inteiros, com nome só para os três primeiros" — não tem pré-lançamento nem metadados de build como texto. O passo original que introduziu `pre`/`build` no cristalino (parte de uma série "P404 Decimal → P405 Duration → P406 Version") baseou-se no SemVer 2.0.0 geral, não na especificação real do Typst, apesar do relatório desse passo dizer "forma vanilla". Este passo remove o que não devia ter sido adicionado.
> **Tipo:** Implementação directa. Causa já confirmada com duas fontes independentes.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P682 (onde a divergência foi confirmada por teste directo), a pesquisa que confirmou a origem do erro (documentação oficial + histórico do próprio projecto).

---

## Contexto

Confirmado por duas fontes independentes:

1. **Documentação oficial do Typst** (`https://typst.app/docs/reference/foundations/version/`): "A version with an arbitrary number of components. The first three components have names... All following components do not have names." Todos os componentes são inteiros; não há `pre`/`build` como texto.
2. **Histórico do próprio projecto**: o passo original que introduziu `pre`/`build` justificou a decisão citando explicitamente "semver 2.0.0", não a especificação do Typst.

A forma `version(major, minor, patch, pre: "...", build: "...")`, e a variante posicional com um quarto argumento de texto, não existem no vanilla e nunca existiram — não é uma funcionalidade que o Typst tivesse e removido, é uma funcionalidade que o cristalino inventou por confundir dois conceitos de versionamento diferentes.

---

## Implementação

Remover de `native_version` (`01_core/src/engine/stdlib/primitives_constructors.rs`):

- O quarto argumento posicional de texto (pré-lançamento).
- Os argumentos nomeados `pre` e `build`.
- Qualquer lógica de comparação (`==`, `!=`, `<`, etc.) que trate `build` como metadado ignorado na igualdade — se `build` deixa de existir como conceito, essa lógica especial também deixa de fazer sentido.

Manter:
- A forma posicional com qualquer número de inteiros (`version(1, 2, 3)`, `version(1, 2, 3, 4)`, etc., seguindo "arbitrary number of components" da documentação).
- A forma de array de inteiros, já implementada em P682.

### Critério de fecho da implementação

- [ ] `version(1, 2, 3, pre: "alpha")` produz erro (argumento desconhecido), igual ao vanilla.
- [ ] `version(1, 2, 3, "alpha.1")` (quarto posicional de texto) produz erro, igual ao vanilla.
- [ ] `version(1, 2, 3, 4)` (quarto posicional inteiro) continua a funcionar, seguindo "arbitrary number of components".
- [ ] Comparação `==`/`!=` entre versões volta a ser comparação directa de todos os componentes, sem tratamento especial de "build".

---

## Validação

```bash
cat > /tmp/p684-teste.typ <<'EOF'
#version(1, 2, 3, pre: "alpha")
EOF
lab/typst-original/target/release/typst compile /tmp/p684-teste.typ /tmp/p684-vanilla.pdf
echo "Vanilla exit: $?"
./target/release/typst /tmp/p684-teste.typ /tmp/p684-cristalino.pdf
echo "Cristalino exit: $?"
```

Confirmar que os dois agora rejeitam da mesma forma.

```bash
cat > /tmp/p684-arbitrario.typ <<'EOF'
#version(1, 2, 3, 4, 5)
EOF
lab/typst-original/target/release/typst compile /tmp/p684-arbitrario.typ /tmp/p684-arb-vanilla.pdf
./target/release/typst /tmp/p684-arbitrario.typ /tmp/p684-arb-cristalino.pdf
```

Confirmar que ambos aceitam qualquer número de componentes inteiros.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] `pre`/`build` removidos, com erro claro igual ao vanilla.
- [ ] Forma de qualquer número de componentes inteiros preservada, testada.
- [ ] Comparações de igualdade simplificadas, sem tratamento especial de "build".
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p684.md`, com hash do commit.
- [ ] Nota adicionada ao histórico do passo original (P404-P406) explicando o erro de origem (confusão com SemVer 2.0.0).
