---
# P637 — `title` aceita array no vanilla, ou só string?

> **Passo:** 637
> **Data:** 2026-07-09
> **Foco:** P636 deu a `title`, `author`, e `keywords` a mesma mensagem de erro ("expected string or array of strings"). Faz sentido para `author` e `keywords`, mas um documento normalmente tem um título só — confirmar se o vanilla aceita mesmo um array para `title`, ou se a mensagem partilhada está a dizer algo impreciso para esse caso específico.
> **Tipo:** Verificação directa. Corrigir a mensagem só se necessário.
> **Tamanho:** XS.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P636 (onde a mensagem partilhada foi introduzida).

---

## Verificação

### Confirmar no código fonte do vanilla

```bash
grep -n "title:" lab/typst-original/crates/typst-library/src/model/document.rs 2>/dev/null
```

Confirmar o tipo declarado para `title` — `EcoString`, ou algo que aceite array.

### Testar directamente

```bash
cat > /tmp/p637-title-array.typ <<'EOF'
#set document(title: ("Título Um", "Título Dois"))
Texto.
EOF
lab/typst-original/target/release/typst compile /tmp/p637-title-array.typ /tmp/p637-vanilla.pdf
echo "Exit code vanilla: $?"

./target/release/typst /tmp/p637-title-array.typ /tmp/p637-cristalino.pdf
echo "Exit code cristalino: $?"
```

### Critério de fecho

- [ ] Tipo de `title` confirmado no código fonte do vanilla, ou por teste directo.
- [ ] Se `title` só aceitar string: separar a mensagem de erro de `title` da de `author`/`keywords`.
- [ ] Se `title` aceitar array também: confirmar que a mensagem partilhada está correcta, sem mudar nada.

---

## Implementação, se necessário

Se `title` só aceitar string, ajustar `value_to_eco_string` (ou o ponto onde a mensagem é gerada) para produzir uma mensagem diferente quando chamado para `title` especificamente: `"expected string, found {tipo}"`, sem a opção de array.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Tipo de `title` confirmado.
- [ ] Mensagem corrigida, se necessário.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p637.md`, com hash do commit.
