---
# P757 — `em` resolve para `0pt` em `#set page(...)`, e revalidar P756 contra o binário vanilla correcto

> **Passo:** 757
> **Data:** 2026-07-14
> **Foco:** Duas coisas encontradas em P756 que precisam de resolução antes de aceitar esse passo como fechado. (1) `#set page(width: 7em)` resolveu para `0pt`, descartado como "fora do âmbito", mas com alcance potencialmente universal — qualquer documento com dimensões de página em `em`. (2) A comparação visual da secção 3 de P756 usou `/usr/local/bin/typst`, não `lab/typst-original/target/release/typst` (o binário de referência confirmado como Typst 0.15.0 usado em toda esta conversa) — as conclusões sobre paridade greedy vs Knuth-Plass podem não estar a comparar com a versão certa.
> **Tipo:** Sonda + Implementação. Prioridade alta para o bug de `em`; verificação directa para a questão do binário.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P756 (onde os dois problemas surgiram, sem serem resolvidos nem devidamente registados).

---

## Parte A — `em` em dimensões de página

### Sonda

```bash
cat > /tmp/p757-em-page.typ <<'EOF'
#set page(width: 7em, height: 5em)
X
EOF
lab/typst-original/target/release/typst compile /tmp/p757-em-page.typ /tmp/p757-vanilla.pdf
mutool show /tmp/p757-vanilla.pdf trailer 2>&1 | grep -i mediabox
./target/release/typst /tmp/p757-em-page.typ /tmp/p757-cristalino.pdf 2>&1
mutool show /tmp/p757-cristalino.pdf trailer 2>&1 | grep -i mediabox
```

Confirmar o comportamento do vanilla — `em` em `page(width:)` resolve contra o tamanho de fonte activo no momento (o `em` "solto", sem parágrafo, pode não ter uma referência clara — confirmar exactamente o que o vanilla faz, incluindo se é sequer uma forma válida de uso, ou se o vanilla também rejeita/erra).

### Localizar a causa no cristalino

```bash
grep -n "fn.*page.*width\|resolve.*em\|Length::Em\|Em(" 01_core/src/engine/eval/rules.rs 01_core/src/engine/layout/set_page.rs 2>/dev/null | head -20
```

Confirmar onde a resolução de `em` para dimensões de página falha silenciosamente e produz zero, em vez de erro ou do valor correcto.

### Implementação

Corrigir a resolução de `em` em `page(width:)`/`page(height:)`, seguindo o comportamento confirmado pela sonda (valor correcto, ou erro claro se o vanilla também rejeitar este uso).

### Critério de fecho da Parte A

- [ ] Comportamento do vanilla confirmado.
- [ ] Causa localizada e corrigida no cristalino.
- [ ] Testado, sem produzir `0pt` silenciosamente.

---

## Parte B — Revalidar P756 contra o binário vanilla correcto

### Verificação

```bash
lab/typst-original/target/release/typst --version
/usr/local/bin/typst --version 2>/dev/null || echo "confirmar se este binário existe e qual versão é"
```

Confirmar se são a mesma versão. Se forem diferentes, repetir os testes visuais da secção 3 de P756 (CJK sem aspas, CJK com aspas, Thai) usando `lab/typst-original/target/release/typst`, não `/usr/local/bin/typst`.

```bash
cat > /tmp/p757-cjk.typ <<'EOF'
#set page(width: 100pt, margin: 5pt)
#set text(font: "Noto Serif CJK SC", size: 12pt)
测试文本，"测试引号的位置"。这是一段很长的中文文字用来测试换行的效果如何。
EOF
lab/typst-original/target/release/typst compile /tmp/p757-cjk.typ /tmp/p757-cjk-vanilla-correto.pdf
mutool draw -o /tmp/p757-cjk-vanilla-correto.png -r 150 /tmp/p757-cjk-vanilla-correto.pdf
./target/release/typst /tmp/p757-cjk.typ /tmp/p757-cjk-cristalino.pdf
mutool draw -o /tmp/p757-cjk-cristalino.png -r 150 /tmp/p757-cjk-cristalino.pdf
```

### Critério de fecho da Parte B

- [ ] Confirmado se `/usr/local/bin/typst` e o binário de referência desta conversa são a mesma versão.
- [ ] Se forem diferentes: testes visuais de P756 repetidos contra o binário correcto, conclusões revistas se necessário.
- [ ] Se forem a mesma versão: confirmado e registado, sem mudança nas conclusões de P756.

---

## Validação final

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Parte A: bug de `em` em dimensões de página corrigido e testado.
- [ ] Parte B: testes de P756 revalidados contra o binário correcto, conclusões confirmadas ou revistas.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p757.md`, com hash do commit.
