---
# P772f — Varredura da stdlib: `typst_library::layout::grid::resolve`

> **Passo:** 772f (continuação da série P765a→P772e; substitui/valida a versão auto-gerada por Claude Code)
> **Data:** 2026-07-16
> **Foco:** P772e classificou este módulo (24 itens) como maior risco observável entre os restantes — layout renderizado de `grid()`, componente já tocado por bugs reais nesta conversa (P763g mediu divergências estruturais em `grid` durante a linha de trabalho de `cetz`/shapes, nunca investigadas a fundo por estarem fora do âmbito daqueles passos). Este passo classifica os 24 itens item a item e investiga especificamente se há sobreposição com o que P763g já tinha registado como "divergência estrutural pré-existente do grid" sem causa identificada.
> **Tipo:** Sonda + Implementação directa para achados confirmados.
> **Tamanho:** M/L — módulo de maior risco da lista, pode revelar mais do que os módulos anteriores.
> **ADR-0108 EM VIGOR.** **ADR-0107** — atenção a efeito observável em layout renderizado, o tipo de divergência mais caro de detectar tardiamente (lição de toda a linha P763-P771).
> **Dependências:** P772e (priorização, achado de que grid é o módulo de maior risco), P763g (achado anterior não resolvido: `grid` com `place` deu AE=4093 vs 2020 sem `place`, atribuído a "divergência estrutural pré-existente do grid", nunca investigado a fundo).

---

## Sonda — classificar os 24 itens

```bash
awk -F'\t' '$1=="lacuna-inventario" && $5 ~ /layout::grid::resolve/' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt | cut -f5 | sort
```

Para cada item, ler o código-fonte do vanilla e o estado actual do cristalino:

```bash
grep -n "<item>" lab/typst-original/crates/typst-layout/src/grid/resolve.rs 2>/dev/null
grep -rn "<item>\|fn.*grid\|GridLayouter" 01_core/src/engine/layout/grid.rs 2>/dev/null
```

### Verificar a ligação com o achado não resolvido de P763g

P763g mediu, sem investigar a causa:

| Variante | AE |
|----------|-----|
| grid com place | 4093 |
| grid sem place | 2020 |

E concluiu, sem confirmar por coordenadas: "a inspecção visual mostra que o grid cristalino já não renderiza a segunda célula (ou posiciona-a fora da página) mesmo sem place. A divergência principal é estrutural do próprio grid, não da correcção de layout_place." Esta frase nunca foi verificada com `mutool trace` — é exactamente o tipo de afirmação que esta conversa já desmentiu várias vezes (P763d, P774, P777) quando não acompanhada de coordenadas.

```bash
cat > /tmp/p772f-grid-repro.typ <<'EOF'
#set page(width: 8cm, height: 6cm)
#grid(
  columns: 2, gutter: 5pt,
  block(width: 3cm, height: 2cm, align(top, place(top+left, dx: 5pt, dy: 5pt, circle(radius: 10pt)))),
  block(width: 3cm, height: 2cm, align(top, place(top+left, dx: 5pt, dy: 5pt, circle(radius: 10pt)))),
)
EOF
lab/typst-original/target/release/typst compile /tmp/p772f-grid-repro.typ /tmp/p772f-vanilla.pdf
./target/release/typst compile /tmp/p772f-grid-repro.typ /tmp/p772f-cristalino.pdf
mutool trace /tmp/p772f-vanilla.pdf > /tmp/p772f-trace-vanilla.txt
mutool trace /tmp/p772f-cristalino.pdf > /tmp/p772f-trace-cristalino.txt
diff /tmp/p772f-trace-vanilla.txt /tmp/p772f-trace-cristalino.txt
```

Confirmar directamente: a segunda célula do `grid` está mesmo ausente/fora da página no cristalino, ou essa era uma leitura apressada de P763g? Se confirmado ausente, este é provavelmente o achado mais sério desta série de varredura até agora — grid é um elemento estrutural comum, não um caso de canto.

---

## Implementação

Para cada bug real confirmado (incluindo o achado de P763g se procedente), corrigir com teste e comparação directa. Se o achado do grid for grande (célula ausente é candidato a causa estrutural, não pontual), não forçar correcção no mesmo passo se exigir investigação equivalente à cadeia P763-P767 — registar e propor passo dedicado, seguindo a mesma disciplina (sonda de localização → decisão → implementação → validação por coordenadas).

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Para o achado de grid especificamente: checklist de sub-layouts adaptado (grid dentro de columns, grid dentro de box, etc.), dado ser um mecanismo estrutural.

---

## Critério de fecho do passo

- [x] Os 24 itens de `layout::grid::resolve` classificados item a item.
- [x] Achado de P763g (segunda célula ausente/fora da página) confirmado ou desmentido por coordenadas, não por inspecção visual — **desmentido**: célula não está ausente, está sobreposta (colisão de coluna a 0pt).
- [x] Se confirmado como bug real e grande: registado com decisão explícita (regra 1) sobre corrigir agora ou abrir cadeia dedicada, não forçado neste passo — causa raiz B (`layout_place` duplica origem quando aninhado em `align`) registada, passo dedicado P772g proposto.
- [x] Se confirmado como bug real e pequeno: corrigido com teste e coordenadas antes/depois — causa raiz A (`measure_content_constrained` sem braço `Content::Align` + `Content::Block` a ignorar largura explícita) corrigida, 3 testes novos.
- [x] Outros itens do módulo classificados (mecânica/scope-out ou bug real).
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772f.md`.

Achados adicionais fora do checklist original, registados no relatório: (1) `grid()`
aceita `header:`/`footer:` nomeados sem equivalente vanilla e descarta o conteúdo em
silêncio; (2) código não commitado pré-existente em `grid.rs` (wrap de `effective_align`
via `Place`) sem L0 correspondente e divergente do vanilla — decisão do humano pendente
(reverter vs formalizar).

---

## Próximo passo

Conforme P772e: `visualize::image::svg` (7 itens), `foundations::scope` (7 itens), `text::font::*` (~22 itens) — nesta ordem ou pela ordem que a gravidade do achado de grid sugerir.
