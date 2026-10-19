---
# P785c — Offset de coluna UTF-16 em diagnósticos com caracteres multi-byte

> **Passo:** 785c
> **Data:** 2026-07-20
> **Foco:** P785 confirmou que `#let a = "🚀 🇧🇷" + undefined_var_xyz` reporta coluna 1:19 no cristalino contra 1:18 no vanilla — divergência de 1 unidade, consistente com contagem incorreta de code units UTF-16 em torno de caracteres fora do BMP (emoji, incluindo os regional indicators de bandeiras, que ocupam 2 code units UTF-16 cada). Este passo corrige o cálculo de offset de coluna para diagnósticos.
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — confirmar a fórmula exata de conversão do vanilla (UTF-8 byte offset → UTF-16 code unit count) antes de implementar.
> **Dependências:** P785 (achado, caso de teste com emoji).

---

## Sonda — mecanismo exato do vanilla

```bash
grep -n "utf16\|len_utf16\|fn.*column" lab/typst-original/crates/typst-syntax/src/lines.rs 2>/dev/null
```

Confirmar a fórmula de conversão exata — para cada `char`, quantas UTF-16 code units conta (1 para BMP, 2 para fora do BMP/surrogate pairs) — e onde essa contagem é usada no cálculo de coluna reportado em diagnósticos.

```bash
grep -n "fn.*column\|utf16\|byte_to_utf16" 01_core/src/engine/syntax/lines.rs 01_core/src/engine/**/*.rs 2>/dev/null
```

Confirmar como o cristalino calcula coluna hoje — provavelmente contagem de caracteres Unicode simples (1 por `char`) em vez de code units UTF-16 (1 ou 2 conforme o plano Unicode).

---

## Implementação

Corrigir o cálculo de coluna para contar UTF-16 code units, não caracteres Unicode simples — cada `char` fora do BMP (código > U+FFFF) conta como 2, não 1.

---

## Validação

```bash
cat > /tmp/p785c-test.typ <<'EOF'
#let a = "🚀 🇧🇷" + undefined_var_xyz
EOF
./target/release/typst compile /tmp/p785c-test.typ 2>&1
```

Confirmar coluna 1:18, idêntica ao vanilla.

```bash
# Casos adicionais — outros emojis/caracteres fora do BMP, para confirmar que a correção generaliza
cat > /tmp/p785c-test2.typ <<'EOF'
#let a = "😀😀😀" + undefined_var_xyz
EOF
lab/typst-original/target/release/typst compile /tmp/p785c-test2.typ 2>&1
./target/release/typst compile /tmp/p785c-test2.typ 2>&1
```

```bash
# Não regressão — texto ASCII simples continua reportando coluna correta
cat > /tmp/p785c-ascii.typ <<'EOF'
#let a = undefined_var_xyz
EOF
./target/release/typst compile /tmp/p785c-ascii.typ 2>&1
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [x] Fórmula exata do vanilla confirmada (contagem UTF-16 code units, surrogate pairs para fora do BMP).
- [x] Cálculo de coluna corrigido no cristalino.
- [x] Caso original de P785 (emoji + bandeira) bate com o vanilla.
- [x] Caso adicional (múltiplos emoji) testado e batendo.
- [x] Texto ASCII simples sem regressão.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p785c.md`.

---

## Próximo passo

Com P785a/b/c fechados, retomar a triagem em lote (próximos 15 módulos de `lacuna-inventario`) ou fazer o resumo final pendente da série P765a-P784.
