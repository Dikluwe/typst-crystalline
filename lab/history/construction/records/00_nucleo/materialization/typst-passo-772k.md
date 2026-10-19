---
# P772k — Varredura da stdlib: `typst_library::visualize::image::svg`

> **Passo:** 772k (continuação da série P765a→P772j)
> **Data:** 2026-07-16
> **Foco:** Classificar `visualize::image::svg` (7 itens). Módulo de alto risco observável (renderização de imagem, mesma categoria de `image::raster` que rendeu 2 bugs reais + 3 desdobramentos em P773/P774/P776/P777/P778). Não assumir baixo risco pela contagem pequena — a lição de `image::raster` é exactamente essa.
> **Tipo:** Sonda + Implementação directa para achados confirmados.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0107** — efeito observável em renderização de imagem é sempre candidato a bug real, não infra-estrutura sem efeito.
> **Dependências:** P772j (lote anterior fechado), P773/P776 (metodologia de comparação de imagem já estabelecida — reutilizar, não reinventar).

---

## Sonda — classificar os 7 itens

```bash
awk -F'\t' '$1=="lacuna-inventario" && $5 ~ /image::svg/' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt | cut -f5 | sort
```

```bash
grep -n "<item>" lab/typst-original/crates/typst-library/src/visualize/image/svg.rs 2>/dev/null
grep -rn "<item>\|svg\b" 01_core/src/engine/layout/image.rs 03_infra/src/export/images.rs 2>/dev/null
```

### Casos de teste — paridade com o vanilla

```bash
cat > /tmp/p772k-svg-test.typ <<'EOF'
#image("test.svg")
EOF
```

Gerar um SVG de teste simples (formas + texto, se o cristalino já suportar texto em SVG) e comparar renderização:

```bash
lab/typst-original/target/release/typst compile /tmp/p772k-svg-test.typ /tmp/p772k-vanilla.pdf
./target/release/typst compile /tmp/p772k-svg-test.typ /tmp/p772k-cristalino.pdf
mutool draw -o /tmp/p772k-vanilla.png -r 300 /tmp/p772k-vanilla.pdf
mutool draw -o /tmp/p772k-cristalino.png -r 300 /tmp/p772k-cristalino.pdf
compare -metric AE /tmp/p772k-vanilla.png /tmp/p772k-cristalino.png /tmp/p772k-diff.png
```

Atenção específica a: dimensionamento (`viewBox` vs `width`/`height` explícitos — mesma família de bug que DPI de raster em P773), texto dentro de SVG (fontes do documento vs fontes do sistema), e cores/gradientes SVG.

---

## Implementação

Só para achados confirmados como bug real (efeito visual observável, medido por `mutool trace`/`compare`, não só ausência de erro). Corrigir com teste e comparação directa.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Os 7 itens de `image::svg` classificados item a item.
- [ ] Caso de teste real (SVG com formas, texto se aplicável) comparado por coordenadas/AE.
- [ ] Bugs reais corrigidos com teste e comparação directa.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772k.md`.

---

## Próximo passo

`foundations::scope` (P772l) ou `text::font::*` (P772m), conforme a ordem de execução escolhida.
