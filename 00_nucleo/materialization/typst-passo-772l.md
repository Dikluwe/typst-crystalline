---
# P772l — Varredura da stdlib: `typst_library::foundations::scope`

> **Passo:** 772l (continuação da série P765a→P772k)
> **Data:** 2026-07-16
> **Foco:** Classificar `foundations::scope` (7 itens). Módulo de semântica de bindings/escopos — P772e marcou como risco "alto" por afectar potencialmente resolução de nomes, mas isto é mais próximo de mecânica de avaliação do que de renderização visual; verificar se algum item tem efeito observável real (ex: comportamento de shadowing, escopos de `#import`, visibilidade de bindings dentro de blocos) antes de assumir que "alto risco" se traduz em bug real.
> **Tipo:** Sonda + Implementação directa para achados confirmados.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0107** — comportamento de binding/escopo é semântica de língua se afecta o que um programa Typst pode observar (que variável resolve para quê), não é "mecânica interna" mesmo sendo sobre estrutura de dados de avaliação.
> **Dependências:** P772k (lote anterior).

---

## Sonda — classificar os 7 itens

```bash
awk -F'\t' '$1=="lacuna-inventario" && $5 ~ /foundations::scope/' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt | cut -f5 | sort
```

```bash
grep -n "<item>" lab/typst-original/crates/typst-library/src/foundations/scope.rs 2>/dev/null
grep -rn "<item>\|struct Scope\|Scopes" 01_core/src/engine/eval/*.rs 2>/dev/null
```

### Casos de teste — comportamento observável de escopo

```bash
cat > /tmp/p772l-scope-test.typ <<'EOF'
#let x = 1
{
  let x = 2
  x
}
x
EOF
```

Testar shadowing dentro de blocos, escopos de função, e visibilidade de bindings importados:

```bash
for spec in \
  "#let x = 1; { let x = 2; x }" \
  "#let f() = { let y = 1; y }; f()" \
  ; do
  echo "=== $spec ==="
  echo "$spec" > /tmp/p772l-case.typ
  lab/typst-original/target/release/typst compile /tmp/p772l-case.typ 2>&1 | tail -3
  ./target/release/typst compile /tmp/p772l-case.typ 2>&1 | tail -3
done
```

---

## Implementação

Só para achados confirmados como bug real (comportamento de resolução de nomes diverge, com caso de teste mínimo comparando saída). Corrigir com teste comparando o valor resolvido, não só ausência de erro.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Os 7 itens de `foundations::scope` classificados item a item.
- [ ] Casos de shadowing/escopo testados com saída comparada directamente.
- [ ] Bugs reais corrigidos com teste comparando valor resolvido.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772l.md`.

---

## Próximo passo

`text::font::*` (P772m).
