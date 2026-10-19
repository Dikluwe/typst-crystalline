---
# P783 — Fallback de fontes matemáticas

> **Passo:** 783
> **Data:** 2026-07-17
> **Foco:** P772w registrou "fallback de fontes matemáticas" como débito, junto com `image::pdf` (já resolvido/scope-out consciente em P781) e `math.class` (já resolvido em P772y). Este passo primeiro reconstrói o achado exato de P772w (o relatório original não está disponível na íntegra nesta sessão) antes de investigar e implementar — não assumir a natureza do gap sem confirmar o que foi medido originalmente.
> **Tipo:** Sonda de reconstrução + Sonda de mecanismo + Implementação condicional.
> **Tamanho:** M/L — depende do que a reconstrução revelar.
> **ADR-0108 EM VIGOR** — reconstruir o achado original antes de investigar, não reinventar a partir do nome.
> **Dependências:** P772w (achado original, relatório em `00_nucleo/diagnosticos/paridade-producao-p772w.md`).

---

## Passo 0 — Reconstruir o achado original de P772w

```bash
grep -n -A20 "fallback de fontes\|font.*fallback\|fallback.*math" 00_nucleo/diagnosticos/paridade-producao-p772w.md
```

Ler a seção completa onde este débito foi registrado, com o texto exato do que foi medido — não assumir que é sobre a mesma coisa que o fallback de fontes de texto (glyf/gvar/CFF2 já corrigido em P772o/P772u). Confirmar se é:
1. Fallback de glifo matemático específico ausente na fonte principal, precisando de fonte de fallback matemática dedicada (o vanilla usa fontes especiais tipo "New Computer Modern Math" com tabela `MATH` do OpenType).
2. Algo relacionado à tabela `MATH` do OpenType em si (constantes de espaçamento, posicionamento de scripts, etc.) não sendo lida.
3. Outra coisa — confirmar antes de prosseguir.

---

## Sonda — mecanismo exato do vanilla (conforme o que o Passo 0 revelar)

```bash
grep -rn "MathConstants\|table.*MATH\|math_table" lab/typst-original/crates/typst-library/src/text/font/*.rs lab/typst-layout/src/math/*.rs 2>/dev/null | head -30
```

Se for sobre a tabela `MATH` do OpenType: confirmar quais constantes o vanilla lê e usa (posicionamento de sub/superscript, espessura de barra de fração, etc.) e se o cristalino as lê.

```bash
grep -rn "MathConstants\|table.*MATH" 01_core/src/rules/math/**/*.rs 03_infra/src/font_metrics.rs 2>/dev/null
```

### Caso de teste real

```bash
cat > /tmp/p783-math-test.typ <<'EOF'
$ frac(a, b) $
$ x^2_1 $
EOF
lab/typst-original/target/release/typst compile /tmp/p783-math-test.typ /tmp/p783-vanilla.pdf
./target/release/typst compile /tmp/p783-math-test.typ /tmp/p783-cristalino.pdf
mutool trace /tmp/p783-vanilla.pdf > /tmp/p783-trace-vanilla.txt
mutool trace /tmp/p783-cristalino.pdf > /tmp/p783-trace-cristalino.txt
diff /tmp/p783-trace-vanilla.txt /tmp/p783-trace-cristalino.txt
```

Confirmar se há divergência de posicionamento que aponte para constantes da tabela `MATH` não lidas (ex: espessura da barra de fração, posição vertical de sub/superscript).

---

## Decisão de âmbito

Registar com base no que a sonda encontrar:
- Se for uma constante isolada ou pequeno conjunto: implementar diretamente.
- Se for a tabela `MATH` inteira ausente (mecanismo grande, tipo o que faltava para espaçamento por classe em P772y): decidir se cabe neste passo ou precisa de fase própria — mesmo padrão de decisão já usado em P772y.

---

## Implementação (conforme decisão)

A definir pela sonda — não especificado de antemão neste prompt, dado que o Passo 0 pode revelar uma natureza de gap diferente da assumida.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Reconfirmar testes de math existentes (P299-301, P765b, P772w, P772y, P780, P782) sem regressão.

---

## Critério de fecho do passo

- [ ] Achado original de P772w reconstruído com o texto exato (não assumido do título).
- [ ] Mecanismo exato do vanilla confirmado.
- [ ] Decisão de âmbito registrada.
- [ ] Se implementado: validado por coordenadas/comparação visual contra o vanilla.
- [ ] Se não implementado: scope-out consciente registrado com custo medido, mesmo padrão de P781.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p783.md`.

---

## Próximo passo

Com este fechado, todos os débitos conhecidos de P772w e seus desdobramentos (P772x, P772y, P780, P781, P782) estão endereçados. Momento natural para um resumo final da série completa P765a-P783.
