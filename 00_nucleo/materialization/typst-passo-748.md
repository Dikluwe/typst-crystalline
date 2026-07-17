---
# P748 — Margem por defeito da página parece errada (~81,9pt vs os ~70,87pt esperados)

> **Passo:** 748
> **Data:** 2026-07-10
> **Foco:** P747 mediu que o vanilla posiciona o primeiro elemento a ~70,87pt do topo da página (≈25mm), consistente com a fórmula de margem por defeito já estabelecida nesta conversa em P598/599 (`margem = min(largura, altura) × 2,5/21`, ≈25mm para A4). O cristalino posiciona o mesmo elemento a ~81,9pt — cerca de 11pt a mais. Isto não foi investigado como bug em P747, apesar de a fórmula correcta já estar documentada neste próprio projecto. Se confirmado, afecta a margem por defeito de qualquer documento sem margem explícita, não só este teste. Prioridade máxima.
> **Tipo:** Sonda + Implementação. Prioridade máxima — possível bug fundamental de layout, alcance potencialmente universal.
> **Tamanho:** S-M, se confirmado que é só um valor de cálculo errado.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — mecanismo central de layout de página.
> **Dependências:** P747 (onde a medição foi feita, sem investigar a causa), P598/599 (onde a fórmula correcta da margem foi estabelecida e confirmada nesta mesma conversa).

---

## Contexto

Fórmula já estabelecida em P598/599, confirmada contra o vanilla na altura: `margin = min(w, h) * 2.5 / 21`. Para A4 (595.28pt × 841.89pt de largura × altura, com `min` a apanhar a largura em orientação retrato): `595.28 * 2.5 / 21 ≈ 70.867pt` — bate exactamente com o valor medido do vanilla em P747 (70.8661pt, mesmo o `cm` do content stream).

O cristalino mediu ~81,9pt nesse mesmo ponto — a confirmar se é a margem em si que está errada, ou se há outra coisa a somar-se à margem correcta.

---

## Sonda

### Confirmar directamente a margem calculada pelo cristalino hoje

```bash
grep -n "min(w,h)\|2.5.*21\|fn.*margin\|default_margin" 01_core/src/rules/layout/*.rs 01_core/src/entities/*.rs 2>/dev/null | head -20
```

Localizar exactamente onde a fórmula de P598/599 está implementada, e confirmar se ainda produz o valor certo isoladamente (é possível que a fórmula esteja certa, mas algo mais seja somado à margem no caminho até à posição final do conteúdo).

### Confirmar com um documento mínimo, sem `line`/`circle`, só texto

```bash
cat > /tmp/p748-margem.typ <<'EOF'
X
EOF
lab/typst-original/target/release/typst compile /tmp/p748-margem.typ /tmp/p748-vanilla.pdf
mutool show /tmp/p748-vanilla.pdf 4 2>&1 | head -10
./target/release/typst /tmp/p748-margem.typ /tmp/p748-cristalino.pdf
mutool show /tmp/p748-cristalino.pdf 4 2>&1 | head -10
```

Confirmar a posição do texto "X" no topo da página, nos dois lados — isto isola se o problema é específico de formas de desenho (`line`/`circle`) ou é a margem da página em geral, afectando texto normal também.

### Confirmar se `line`/`circle` têm algum recuo/margem interna própria, além da margem de página

```bash
grep -n "fn native_line\|fn native_circle\|inset\|padding" 01_core/src/rules/stdlib/shapes.rs | head -20
```

Confirmar se os 11pt não vêm de uma margem interna às próprias formas de desenho, distinta da margem de página.

### Critério de fecho da sonda

- [ ] Fórmula de margem confirmada isoladamente, comparando com o valor esperado.
- [ ] Confirmado com documento de texto simples, isolando se o problema é geral (margem de página) ou específico de formas de desenho.
- [ ] Causa exacta localizada, com `file:line`.

---

## Implementação

Corrigir a causa confirmada pela sonda.

### Critério de fecho da implementação

- [ ] Margem por defeito corrigida, testada com documento mínimo de texto e com o documento `line`+`circle` de P746/P747.
- [ ] `cetz` e outros documentos já testados nesta conversa sem regressão visual inesperada (a margem sendo usada por todos, uma correcção aqui pode mudar posições em muitos documentos — confirmar que a mudança é para melhor, não introduz nova divergência).

---

## Validação

```bash
./target/release/typst /tmp/p748-margem.typ /tmp/p748-depois.pdf
mutool show /tmp/p748-depois.pdf 4 2>&1 | head -10
```

Comparar com o vanilla.

```bash
cargo test --workspace
crystalline-lint .
```

### Repetir a reprodução final de `cetz`

```bash
cat > /tmp/p748-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p748-cetz.typ /tmp/p748-cetz.pdf
mutool draw -o /tmp/p748-cetz.png -r 150 /tmp/p748-cetz.pdf
lab/typst-original/target/release/typst compile /tmp/p748-cetz.typ /tmp/p748-cetz-vanilla.pdf
mutool draw -o /tmp/p748-cetz-vanilla.png -r 150 /tmp/p748-cetz-vanilla.pdf
python3 /tmp/pngdiff.py /tmp/p748-cetz-vanilla.png /tmp/p748-cetz.png
```

Confirmar se o diff residual (~0,14%) diminui depois desta correcção — se a margem estava mesmo errada, isto devia melhorar a paridade de pixels, não só ser neutro.

---

## Critério de fecho do passo

- [ ] Sonda completa, causa exacta confirmada (margem em si, ou algo somado a ela).
- [ ] Corrigido, testado com documento de texto simples e com o documento de formas.
- [ ] `cetz` re-testado, diff de pixels comparado com o valor anterior (~0,14%) — confirmar se melhora.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p748.md`, com hash do commit.
