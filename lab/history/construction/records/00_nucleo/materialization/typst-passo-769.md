---
# P769 — Correcção: `Content::Image` (ancoramento vertical) + medição condicional de `Content::Curve`

> **Passo:** 769
> **Data:** 2026-07-15
> **Foco:** P768 confirmou, por coordenadas (`mutool trace`), que `Content::Image` tem exactamente o mesmo padrão de bug que `Content::Shape` tinha antes de P767c: tratado como bloco, mas sem `above`/`below` nem ancoramento na próxima baseline quando sucede texto. Desvio medido: ~39pt em `A #image(...) B`. `Content::Curve` foi classificado como risco médio-alto mas não medido com texto — este passo mede primeiro, decide depois se corrige.
> **Tipo:** Implementação directa para `Image` (causa já confirmada por coordenadas). Sonda + decisão condicional para `Curve`.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **Regra 1 do handoff** — decisão de corrigir `Curve` só depois de medir, não antes.
> **Dependências:** P768 (achado de `Image` confirmado, achado de `Curve` classificado mas não medido), P767c (lógica de ancoramento a replicar).

---

## Parte A — `Content::Image`: replicar a correcção de P767c

### Localizar o código

```bash
grep -n "fn layout" 01_core/src/engine/layout/image.rs
```

Confirmar se `image.rs` usa o mesmo padrão de `shape.rs` antes de P767c (ancoragem no topo da linha em vez da base na próxima baseline, sem `above`/`below` quando sucede texto).

### Aplicar a mesma correcção

Replicar exactamente a lógica já validada em P767c (`shape.rs`):
- Guardar `baseline_before_flush` antes de `flush_line()`.
- Detectar `had_text_line`.
- Para imagem depois de texto: `image_base = baseline_before_flush + above`; cursor avança para `image_base + height + below`.
- Para imagem depois de bloco ou isolada: manter o comportamento actual (já correcto, conforme P768 não achou divergência nesses casos).

### Validação — reproduzir a medição de P768

```bash
cat > /tmp/p769-image.typ <<'EOF'
A #image("tiny.png") B
EOF
lab/typst-original/target/release/typst compile /tmp/p769-image.typ /tmp/p769-image-vanilla.pdf
./target/release/typst compile /tmp/p769-image.typ /tmp/p769-image-cristalino.pdf
mutool trace /tmp/p769-image-vanilla.pdf > /tmp/p769-trace-vanilla.txt
mutool trace /tmp/p769-image-cristalino.pdf > /tmp/p769-trace-cristalino.txt
```

Reconstruir a tabela de P768 (A, image inferior, image superior, B) e confirmar ΔY ≈ 0 nos quatro pontos.

```bash
mutool draw -o /tmp/p769-image-vanilla.png -r 300 /tmp/p769-image-vanilla.pdf
mutool draw -o /tmp/p769-image-cristalino.png -r 300 /tmp/p769-image-cristalino.pdf
compare -metric AE /tmp/p769-image-vanilla.png /tmp/p769-image-cristalino.png /tmp/p769-image-diff.png
```

### Regressão

Repetir os casos de `Image` isolado e dentro de sub-layouts que já batiam antes, confirmando que não regrediram.

---

## Parte B — `Content::Curve`: medir com texto antes de decidir

### Sonda

```bash
cat > /tmp/p769-curve.typ <<'EOF'
A #curve(curve.move((0pt,0pt)), curve.line((20pt,20pt))) B
EOF
lab/typst-original/target/release/typst compile /tmp/p769-curve.typ /tmp/p769-curve-vanilla.pdf
./target/release/typst compile /tmp/p769-curve.typ /tmp/p769-curve-cristalino.pdf
mutool trace /tmp/p769-curve-vanilla.pdf > /tmp/p769-curve-trace-vanilla.txt
mutool trace /tmp/p769-curve-cristalino.pdf > /tmp/p769-curve-trace-cristalino.txt
```

Construir a mesma tabela (A, curve inferior, curve superior, B), com coordenadas exactas.

### Decisão condicional

- **Se ΔY significativo (mesma ordem de grandeza de Image/Shape antes das correcções):** aplicar a mesma correcção de ancoramento a `curve.rs`, com a mesma validação de regressão.
- **Se ΔY ≈ 0:** registar que `Curve` não tem o problema, apesar de `flush_line` sem `above`/`below` explícito — pode ser que `curve` raramente apareça isolado no meio de texto em documentos reais (baixo risco prático), ou que outro mecanismo já compense. Não corrigir especulativamente; registar o porquê se for possível confirmar por leitura de código.

---

## Validação final

```bash
cargo test --workspace
crystalline-lint .
```

Confirmar que os testes de P745-762 (texto puro) e P767c (shapes) continuam sem regressão.

---

## Critério de fecho do passo

- [ ] `Content::Image`: ΔY ≈ 0 nos quatro pontos de `A #image(...) B`, confirmado por `mutool trace`.
- [ ] `Content::Image`: sem regressão nos casos isolados/sub-layouts já correctos.
- [ ] `Content::Curve`: medido com texto; decisão registada (corrigir ou não) com base na medição, não em suposição.
- [ ] Se `Curve` corrigido: mesma validação de coordenadas aplicada.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] L0 de `Image` (e de `Curve`, se corrigido) actualizado antes do código, conforme a regra de ouro do CLAUDE.md.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p769.md`, com as tabelas de coordenadas de ambos.

---

## Próximo passo

Restam do backlog de P768: `Figure`/`Table`/`Grid` (baixa prioridade, sem divergência visível) e a pequena diferença de ~0,27pt em equação inline (fora do âmbito de classificação, possivelmente métrica de fonte matemática — não investigado ainda). Nenhum dos dois é urgente; revisar em ciclo futuro se surgir evidência de impacto real.
