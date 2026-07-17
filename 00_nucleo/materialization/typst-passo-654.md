---
# P654 — Confirmar mensagens exactas do vanilla para `sorted()` rejeitado

> **Passo:** 654
> **Data:** 2026-07-09
> **Foco:** P653 corrigiu `array.sorted(key: ...)`, mas as mensagens de erro para argumento posicional e nome desconhecido ("array.sorted() does not accept positional arguments") não foram confirmadas contra o texto exacto do vanilla — só o caso de sucesso foi comparado lado a lado.
> **Tipo:** Verificação directa.
> **Tamanho:** XS.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P653 (onde as mensagens foram escritas sem confirmação directa).

---

## Verificação

```bash
cat > /tmp/p654-positional.typ <<'EOF'
#(3, 1, 2).sorted(x => -x)
EOF
lab/typst-original/target/release/typst compile /tmp/p654-positional.typ /tmp/p654-pos-vanilla.pdf
```

```bash
cat > /tmp/p654-unknown.typ <<'EOF'
#(3, 1, 2).sorted(reverse: true)
EOF
lab/typst-original/target/release/typst compile /tmp/p654-unknown.typ /tmp/p654-unk-vanilla.pdf
```

Registar o texto exacto de cada erro.

---

## Decisão

Se as mensagens já implementadas por P653 baterem certo: confirmar e fechar, sem alteração.

Se divergirem: corrigir o texto para bater com o vanilla.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Mensagens do vanilla confirmadas para os dois casos.
- [ ] Mensagens do cristalino ajustadas, se necessário.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p654.md`, com hash do commit.
