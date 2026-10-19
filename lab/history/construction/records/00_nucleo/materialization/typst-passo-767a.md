---
# P767a — Implementação: `Content::Shape` quebra parágrafo como bloco

> **Passo:** 767a
> **Data:** 2026-07-15
> **Foco:** L0 fechado em P767 (`00_nucleo/prompts/engine/layout/shape_block_behaviour.md`, hash pendente conforme explicação registada em P767 — aceite). Decisões já tomadas no L0: reutilizar o mecanismo de bloco existente (`block::layout`, opção A preferida entre as três listadas), espaçamento `above`/`below` de `1.2em` por defeito (medido do vanilla), sem replicar o aviso `block may not occur inside of a paragraph`, `place()` não afectado (já corrigido em P763f), sem testes de regressão conhecidos que dependam do comportamento actual. Este passo implementa exactamente essas decisões, sem as reabrir.
> **Tipo:** Implementação directa.
> **Tamanho:** L — mexe no realizador/fluxo de parágrafo, caminho partilhado por todo o documento.
> **ADR-0108 EM VIGOR.** **Regra 5 do handoff** — checklist de sub-layouts obrigatório, dado que o mecanismo afecta o fluxo principal.
> **Dependências:** P767 (L0), P763h (medições de baseline a reproduzir depois da correcção), P763f (mecanismo de `place` em sub-frames, a não regredir).

---

## Sonda — confirmar o mecanismo de bloco existente antes de tocar

```bash
grep -n "fn layout\b" 01_core/src/engine/layout/block.rs
grep -n "block_chain_active\|prev_block_below_pending\|above\|below" 01_core/src/engine/layout/block.rs 01_core/src/engine/layout/mod.rs 2>/dev/null | head -40
```

Confirmar a assinatura exacta e o ponto onde outros elementos de bloco (heading, list, etc.) já decidem "isto quebra o parágrafo corrente" — replicar esse padrão para `Content::Shape`, não inventar um novo caminho.

---

## Implementação

1. Em `01_core/src/engine/layout/shape.rs` (ou onde `Content::Shape` é despachado), envolver o layout da forma no mesmo mecanismo de `block::layout` já usado por outros elementos — fechando qualquer parágrafo/linha corrente antes de posicionar a forma, e reiniciando o fluxo depois.
2. Aplicar `above`/`below` de `1.2em` por defeito, usando o mecanismo de colapso de espaçamento já existente (P250, conforme o L0).
3. Não emitir aviso equivalente a `block may not occur inside of a paragraph` — decisão já registada no L0.
4. Confirmar que o caminho de `place()` não passa por esta alteração (deve continuar a usar posicionamento absoluto, inalterado desde P763f).

---

## Validação

### Reproduzir as medições de P763h (antes/depois)

```bash
# Isoladas — não devem regredir do baseline já bom (AE 5-285 em P763h)
for prim in rect square ellipse circle line polygon; do
  echo "=== $prim isolado ==="
  # documento de teste específico por primitiva, mesmo do P763h
done

# Misturadas com texto — devem sair de AE 1864-7070 para o baseline
for prim in rect square ellipse circle line polygon; do
  echo "=== A #$prim(...) B ==="
  # documento "A #<prim>(...) B", mesmo padrão de P763h
done
```

Usar sempre `mutool draw -r 300` antes de `compare -metric AE` — nunca comparação directa de PDF (regra herdada de P763e/P763f).

### Checklist de sub-layouts (regra 5)

Repetir para `grid`, `box`, `columns`, `place` com uma forma misturada com texto dentro de cada um.

### Regressão dos testes de layout vertical (P745-762)

```bash
cargo test --workspace 2>&1 | grep -i "fail\|p745\|p748\|p750\|p751\|p752\|p761\|p762"
```

Se algum teste de P745-762 falhar: confirmar se depende do comportamento antigo (fluxo contínuo) de forma incidental — se sim, actualizar conscientemente o teste, registando a mudança; se a falha for inesperada e não relacionada, tratar como regressão real a investigar antes de fechar.

### Caso original `cetz` (fecha a linha de trabalho toda)

```bash
cat > /tmp/p767a-cetz-completo.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
```

Medir com o mesmo pipeline rasterizado — este documento não deveria mudar (é `place`-based, já corrigido em P763f, e não passa pelo fluxo de parágrafo normal), mas confirmar que a nova alteração não o regride.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Mecanismo de bloco existente confirmado por leitura de código antes de reutilizar.
- [ ] `Content::Shape` quebra parágrafo, replicando o comportamento do vanilla.
- [ ] Espaçamento `above`/`below` de `1.2em` aplicado com o mecanismo de colapso existente.
- [ ] Primitivas isoladas sem regressão (baseline mantido).
- [ ] Primitivas misturadas com texto no baseline (não mais AE 1864-7070).
- [ ] Checklist de sub-layouts (grid, box, columns, place) confirmado.
- [ ] Testes de P745-762 sem regressão real (falhas, se houver, avaliadas e justificadas).
- [ ] Caso `cetz` original sem regressão.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p767a.md`, com tabela de AE antes/depois para todas as primitivas testadas.

---

## Próximo passo

Se tudo fechar: a linha de trabalho completa iniciada em P763 (download de pacotes → validação `cetz` → bug de coordenadas → causa raiz → correcção arquitectural) fecha definitivamente. Vale um resumo final ligando P763 a P767a, dado ter sido uma cadeia de oito passos derivados de uma pergunta inicial.
