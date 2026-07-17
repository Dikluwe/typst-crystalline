---
# P767c — Correcção: ancoramento vertical de `Content::Shape` no fluxo de parágrafo

> **Passo:** 767c
> **Data:** 2026-07-15
> **Foco:** P767b confirmou, por coordenadas e leitura de código, que P767a não resolveu o caso texto+forma+texto. Causa exacta: (1) `block_chain_active` é posto a `false` por texto não-bloco antes da forma, suprimindo o `above` spacing quando a forma é a primeira depois de texto; (2) mesmo quando aplicado, o modelo actual ancora o **topo** da forma em `baseline + cap_height` (topo da linha), mas o vanilla ancora a **base** da forma em `baseline + above` (próxima baseline da grelha de linhas). O desvio medido (~6,05 pt) bate exactamente com `1.2em − cap_height`. Este passo corrige as duas causas.
> **Tipo:** Implementação directa. Causa já identificada com precisão de código e coordenadas — não é mais sonda de localização.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — medir antes/depois com a mesma disciplina de coordenadas usada em P767b, não só AE.
> **Dependências:** P767b (causa exacta identificada, commit base `beb4d4e4f`).

---

## Implementação

### 1. Não suprimir `above` quando a forma é a primeira depois de texto

Em `01_core/src/rules/layout/sequence.rs:55`, confirmar por que texto não-bloco reseta `block_chain_active`/`prev_block_below_pending` a zero, e se essa mesma regra deveria aplicar-se a `Content::Shape` da forma como está a aplicar-se hoje. A pergunta central: no vanilla, o `above` de um `BlockElem` aplica-se sempre que o bloco sucede conteúdo de parágrafo, ou só quando sucede outro bloco?

```bash
grep -n "above\|below\|Behaviour::" lab/typst-original/crates/typst-layout/src/flow/*.rs 2>/dev/null | head -30
```

Confirmar contra o vanilla antes de decidir se a correcção é "nunca zerar para `Content::Shape`" ou algo mais específico.

### 2. Corrigir o ponto de ancoragem: base da forma, não topo

Em `01_core/src/rules/layout/shape.rs:85` (ou onde a posição Y final é calculada), mudar de:

```rust
// actual: topo da forma em baseline + cap_height
let shape_top_y = cursor_y - cap_height;
```

para o modelo confirmado por P767b (base da forma na próxima baseline da grelha de linhas, `baseline + above`, estendendo-se para cima pela altura da forma):

```rust
// corrigido: base da forma em baseline + above; topo = base - altura_forma
let shape_base_y = cursor_y + above_pt; // ou equivalente, confirmar sinal contra o sistema de coordenadas local
let shape_top_y = shape_base_y - resolved_h;
```

Ajustar os sinais exactos conforme o sistema de coordenadas real do `Layouter` (Y cresce para baixo ou para cima — confirmar antes de aplicar, não assumir pelo pseudo-código acima).

---

## Validação

### Reproduzir exactamente o caso de P767b

```bash
cat > /tmp/p767c-misto.typ <<'EOF'
A #rect(width: 1cm, height: 0.8cm, fill: red) B
EOF
lab/typst-original/target/release/typst compile /tmp/p767c-misto.typ /tmp/p767c-vanilla.pdf
./target/release/typst compile /tmp/p767c-misto.typ /tmp/p767c-cristalino.pdf
mutool trace /tmp/p767c-vanilla.pdf > /tmp/p767c-trace-vanilla.txt
mutool trace /tmp/p767c-cristalino.pdf > /tmp/p767c-trace-cristalino.txt
```

Reconstruir a mesma tabela de P767b (Y de "A", `rect` inferior/superior, "B") e confirmar ΔY ≈ 0 para todos os quatro pontos, não só para "A".

```bash
mutool draw -o /tmp/p767c-vanilla.png -r 300 /tmp/p767c-vanilla.pdf
mutool draw -o /tmp/p767c-cristalino.png -r 300 /tmp/p767c-cristalino.pdf
compare -metric AE /tmp/p767c-vanilla.png /tmp/p767c-cristalino.png /tmp/p767c-diff.png
```

### Repetir as medições de P767a (todas as primitivas × isolada/mista) e o checklist de sub-layouts

Confirmar que a correcção generaliza — não só para `rect`, para todas as primitivas testadas em P767a/P763h (`square`, `ellipse`, `circle`, `line`, `polygon`), e que `grid`/`box`/`columns`/`place` não regridem em relação ao que já estava registado em P767a (mesmo que `grid`/`box` já tivessem divergência conhecida fora do âmbito).

### Não regredir texto puro

```bash
cargo test --workspace 2>&1 | grep -i "fail\|p745\|p748\|p750\|p751\|p752\|p761\|p762"
```

---

## Critério de fecho do passo

- [ ] Confirmado contra o vanilla se `above` deve aplicar-se sempre que bloco sucede parágrafo (não só bloco-após-bloco).
- [ ] Ponto de ancoragem corrigido: base da forma na próxima baseline, não topo da forma no topo da linha.
- [ ] Caso `A #rect(...) B`: os quatro pontos (A, rect inferior, rect superior, B) com ΔY ≈ 0, confirmado por `mutool trace`.
- [ ] Todas as primitivas (não só `rect`) testadas isoladas e misturadas com texto.
- [ ] Sub-layouts (grid, box, columns, place) sem regressão em relação a P767a.
- [ ] Testes de P745-762 (texto puro) continuam sem falha.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p767c.md`, com a tabela de coordenadas completa (não só AE).

---

## Próximo passo

Se tudo fechar com ΔY≈0 confirmado por coordenadas: a linha de trabalho iniciada em P763 fecha definitivamente — resumo final ligando P763 a P767c.
Se ainda houver desvio: não reabrir com nova hipótese solta — voltar ao mesmo padrão de P767b (coordenadas + leitura de código) antes de propor mais uma correcção.
