---
# P656 — `args.named.keys()` tem ordem determinística?

> **Passo:** 656
> **Data:** 2026-07-09
> **Foco:** P655 encontrou dois casos `never_loop` (`structural.rs:1856`, `structural.rs:1895`) que iteram `args.named.keys()` e devolvem erro no primeiro encontrado, classificados como "semântica intencional" sem confirmar se a ordem de iteração é estável. Se `args.named` for um `HashMap` comum, a ordem não é garantida nem estável entre execuções — o mesmo documento, com mais do que um argumento nomeado inválido, podia reportar um nome diferente em execuções diferentes. Isto não é falta de erro; é um erro com o detalhe imprevisível, que nunca foi confirmado.
> **Tipo:** Verificação directa. Correcção só se confirmado o problema.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P655 (onde os dois casos foram encontrados e classificados sem esta verificação).

---

## Verificação

### Confirmar o tipo de `args.named`

```bash
grep -n "struct Args\|named:.*Map\|named:.*Vec" 01_core/src/entities/*.rs 01_core/src/engine/eval/*.rs 2>/dev/null | head -10
```

Confirmar se `named` é `HashMap`, `IndexMap` (ordem de inserção preservada), `BTreeMap` (ordem alfabética, mas determinística), ou `Vec<(EcoString, Value)>` (ordem de inserção, sem custo de hash).

### Testar directamente, várias vezes, se a ordem muda

```bash
cat > /tmp/p656-multi-invalido.typ <<'EOF'
#(3, 1, 2).sorted(reverse: true, stable: false, alpha: 1)
EOF
for i in 1 2 3 4 5; do
  ./target/release/typst /tmp/p656-multi-invalido.typ /tmp/p656-out.pdf 2>&1 | grep "unexpected argument"
done
```

Se a mensagem nomear um argumento diferente em execuções diferentes, o problema está confirmado directamente, sem depender só da leitura do tipo.

### Confirmar o comportamento do vanilla no mesmo caso

```bash
for i in 1 2 3; do
  lab/typst-original/target/release/typst compile /tmp/p656-multi-invalido.typ /tmp/p656-vanilla.pdf 2>&1 | grep "unexpected argument"
done
```

Confirmar se o vanilla é determinístico (provavelmente reporta sempre o primeiro na ordem de escrita do documento) — se for, essa é a semântica a replicar.

### Critério de fecho da verificação

- [ ] Tipo de `args.named` confirmado.
- [ ] Testado directamente, múltiplas execuções, se a mensagem varia.
- [ ] Comportamento do vanilla confirmado como referência.

---

## Decisão

Se confirmado que a ordem não é determinística: corrigir para reportar sempre o primeiro argumento inválido pela ordem em que foi escrito no documento (a ordem que o utilizador vê, não uma ordem interna de estrutura de dados) — provavelmente a mesma coisa que o vanilla já faz.

Se `args.named` já for uma estrutura com ordem preservada (Vec, IndexMap): confirmar que é mesmo esse o caso, e a "suspeita" fica descartada com prova, não com suposição.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Repetir o teste de múltiplas execuções depois de qualquer correcção, confirmando que a mensagem passa a ser sempre a mesma.

---

## Critério de fecho do passo

- [ ] Tipo de `args.named` confirmado.
- [ ] Determinismo testado directamente, não assumido.
- [ ] Se não determinístico: corrigido para reportar sempre o primeiro na ordem de escrita.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p656.md`, com hash do commit.
- [ ] Nota em P655 actualizada — a classificação "semântica intencional" fica confirmada com prova, ou corrigida.
