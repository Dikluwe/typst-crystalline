---
# P793 — `#numbering`, formatação de `enum` e warning de fallback hebrew-zero

> **Passo:** 793
> **Data:** 2026-07-20
> **Foco:** P786 confirmou três divergências em `model::numbering_`: (1) `#numbering(...)` como função standalone (formata um número segundo um padrão, ex: `numbering("1.a", 3, 1)`) é `unknown variable` no cristalino; (2) `enum` com marcador `+` (auto-incremento) não numera corretamente — vanilla produz "1. primeiro 2. segundo 3. terceiro", cristalino produz "-primeiro -segundo -terceiro" (traço literal, sem número); (3) warning de fallback para numerais hebraicos além do intervalo suportado, ausente no cristalino.
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — confirmar se os itens 1 e 2 compartilham o motor de formatação de numeração.
> **Prioridade:** Alta — item 2 afeta qualquer lista numerada automática (`+`), um dos padrões mais comuns de documentos Typst.
> **Dependências:** P786 (achado, evidência em `temp/temp_p786/a_numbering.typ`, `a_numbering_enum.typ`).

---

## Sonda — mecanismo exato do vanilla

```bash
grep -n "fn numbering\b\|fn numbering_pattern" lab/typst-original/crates/typst-library/src/model/numbering.rs 2>/dev/null
```

Confirmar:
1. A assinatura de `numbering()` como função nativa e o motor de formatação de padrões (`"1.a"`, `"I"`, `"א"` hebraico, etc.).
2. Como `enum` com `+` resolve o número do item — usa o mesmo motor de `numbering()` internamente?
3. A mensagem exata do warning de fallback hebrew-zero.

```bash
cat > /tmp/p793-numbering.typ <<'EOF'
#numbering("1.a", 3, 1)
EOF
lab/typst-original/target/release/typst compile /tmp/p793-numbering.typ 2>&1

cat > /tmp/p793-enum.typ <<'EOF'
+ primeiro
+ segundo
+ terceiro
EOF
lab/typst-original/target/release/typst compile /tmp/p793-enum.typ 2>&1
```

### Estado atual do cristalino

```bash
grep -n "fn.*enum\|fn.*numbering" 01_core/src/rules/stdlib/*.rs 01_core/src/rules/layout/*.rs 2>/dev/null | head -20
```

Confirmar se o motor de formatação de padrão de numeração existe (mesmo que não exposto como função `numbering()`), e por que `enum` com `+` não está usando esse motor (ou se o motor em si está ausente/quebrado).

---

## Implementação

1. Implementar `numbering()` como função nativa, usando o motor de padrão (implementar do zero se não existir, ou expor o que já existe internamente).
2. Corrigir `enum` com `+` para usar esse motor corretamente.
3. Adicionar o warning de fallback hebrew-zero.

---

## Validação

```bash
./target/release/typst compile /tmp/p793-numbering.typ 2>&1
./target/release/typst compile /tmp/p793-enum.typ 2>&1
```

Confirmar saída idêntica ao vanilla nos dois casos (via `pdftotext`, não só ausência de erro).

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [x] Motor de formatação de padrão de numeração confirmado (existe/não existe, compartilhado entre `numbering()` e `enum`).
- [x] `numbering()` funciona como função standalone.
- [x] `enum` com `+` numera corretamente.
- [x] Warning de fallback hebrew-zero implementado.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p793.md`, com comandos e saídas reais (seguindo o padrão restaurado em P792a).

---

## Próximo passo

Smartquote, math/symbol scope, sys.version/CLI, ênfase/fontes — restam da lista de P786 §5.
