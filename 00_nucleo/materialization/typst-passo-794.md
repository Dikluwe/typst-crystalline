---
# P794 — Smartquote: aspas curvas por padrão, `#set smartquote` ignorado, validação ausente

> **Passo:** 794
> **Data:** 2026-07-20
> **Foco:** P786 confirmou três divergências em `text::smartquote`: (1) aspas duplas retas (`"..."`) deveriam virar aspas curvas tipográficas por padrão (`"..."` → `“…”`), mas o cristalino mantém retas; aspas simples já funcionam corretamente; (2) `#set smartquote(enabled: false)` e `#set smartquote(quotes: "«»")` são ignorados — o cristalino emite um warning próprio (`"target 'smartquote' ainda não suportado"`) em vez de aplicar a configuração; (3) `#set smartquote(quotes: "abc")` (valor inválido, mais de 2 caracteres) deveria errar no vanilla, mas o cristalino aceita sem validação.
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — confirmar o mecanismo exato de smartquote do vanilla, incluindo por que só aspas simples funcionam hoje (pista de causa).
> **Prioridade:** Alta — aspas retas em vez de curvas afeta qualquer documento com texto entre aspas, um padrão extremamente comum.
> **Dependências:** P786 (achado, evidência em `temp/temp_p786/a_smartquote_min.typ`, `a_smartquote.typ`, `a_smartquote_err_str.typ`).

---

## Sonda — mecanismo exato do vanilla

```bash
grep -n "fn.*smartquote\|SmartQuoteElem\|SmartquoteAlternation" lab/typst-original/crates/typst-library/src/text/smartquote.rs 2>/dev/null
```

Confirmar:
1. Como o vanilla decide entre aspas simples/duplas por padrão (Unicode típico: `“” / ‘’`).
2. Como `#set smartquote(...)` é conectado ao layout (por que o cristalino já suporta o target para outros elementos mas não `smartquote`).
3. A validação de `quotes:` — exatamente 2 caracteres, mensagem de erro exata.

```bash
cat > /tmp/p794-test1.typ <<'EOF'
"duplas" e 'simples'.
EOF
lab/typst-original/target/release/typst compile /tmp/p794-test1.typ 2>&1

cat > /tmp/p794-test2.typ <<'EOF'
#set smartquote(quotes: "abc")
EOF
lab/typst-original/target/release/typst compile /tmp/p794-test2.typ 2>&1
```

### Estado atual do cristalino

```bash
grep -n "smartquote\|target.*ainda não suportado" 01_core/src/rules/eval/rules.rs 01_core/src/rules/layout/*.rs 2>/dev/null
```

Confirmar por que aspas simples já funcionam mas duplas não — pode indicar que o mecanismo existe parcialmente, só falta conectar duplas, em vez de estar totalmente ausente.

---

## Implementação

1. Conectar aspas duplas ao mesmo mecanismo que já produz aspas simples corretas.
2. Adicionar `smartquote` à lista de targets suportados por `#set`, conectando `enabled`/`quotes` ao layout.
3. Adicionar validação de `quotes:` (exatamente 2 caracteres), erro com a mensagem exata do vanilla.

---

## Validação

```bash
./target/release/typst compile /tmp/p794-test1.typ 2>&1
```

Confirmar aspas curvas para ambos.

```bash
cat > /tmp/p794-test3.typ <<'EOF'
#set smartquote(enabled: false)
"reta"
EOF
./target/release/typst compile /tmp/p794-test3.typ 2>&1
```

Confirmar que desabilitar funciona (aspas ficam retas).

```bash
./target/release/typst compile /tmp/p794-test2.typ 2>&1
```

Confirmar erro de validação.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [x] Mecanismo do vanilla confirmado, incluindo por que só aspas simples funcionavam antes.
- [x] Aspas duplas curvas por padrão.
- [x] `#set smartquote(enabled:/quotes:)` conectado ao layout, não mais "ainda não suportado".
- [x] Validação de `quotes:` implementada.
- [x] `cargo test --workspace` verde, contagem da suíte `typst-core` mostrada.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p794.md`, com comandos e saídas reais.

---

## Próximo passo

Math/symbol scope, sys.version/CLI, ênfase/fontes — restam da lista de P786 §5.
