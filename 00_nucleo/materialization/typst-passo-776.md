---
# P776 — Correcção: orientação EXIF via matriz PDF (sem recodificar) + registo da origem

> **Passo:** 776
> **Data:** 2026-07-16
> **Foco:** P775 identificou a causa exacta do AE~87k nas orientações EXIF 2-8: o vanilla preserva os bytes originais do JPEG e aplica a orientação como transformação na matriz `cm` do PDF (`exif_transform`, `typst-pdf/src/image.rs`); o cristalino descodifica, roda os pixels e recodifica, mudando a cor de quase todo pixel por diferença de tabelas de quantização. P775 tinha fechado isto como "fora do âmbito", mas pelo critério do próprio projecto (ADR-0107, resultado observável diverge = dívida de linguagem) isto devia ter passado pela regra 1 antes de ser arquivado — este passo reabre, corrige, e regista a origem do desvio (P774, que introduziu o caminho de decodificação/recodificação).
> **Tipo:** Implementação directa (causa já identificada com precisão por P775) + registo de proveniência.
> **Tamanho:** M.
> **ADR-0107 EM VIGOR** — divergência de cor observável no documento final é dívida de linguagem, não detalhe de implementação aceitável, mesmo quando a geometria bate. **Regra 1 do handoff** — decisão de fechar como "fora do âmbito" não pode ser tomada dentro do próprio relatório de investigação sem passar por este registo.
> **Dependências:** P775 (causa identificada, commit `b7fef82476dfd0d8cba8d23ee3a273b787faf6bc`), P774 (origem do caminho de decodificação/recodificação, a confirmar na Parte B).

---

## Parte A — Correcção: aplicar orientação via matriz `cm`, sem recodificar

### Sonda — confirmar exactamente o mecanismo do vanilla

```bash
grep -n "fn exif_transform\|Orientation\|cm\b" lab/typst-original/crates/typst-pdf/src/image.rs 2>/dev/null
```

Confirmar:
1. A matriz de transformação exacta para cada uma das 8 orientações (combinação de escala ±1 e rotação, aplicada como `cm` antes do `Do` da imagem — o mesmo padrão de matriz já usado para outras transformações no exportador, conforme visto em P771 para o `clip_path`).
2. Se o vanilla ainda lê a tag `Orientation` para decidir a matriz, ou se delega isso a outro componente.
3. Se há algum caso em que o vanilla *precisa* de recodificar (ex: formato não suporta metadados residuais, ou combinação com outra transformação) — não assumir que nunca recodifica.

### Implementação

Reverter, em `03_infra/src/image_sizer.rs`/`03_infra/src/world.rs`, o caminho de `apply_exif_rotation` introduzido em P774 que descodifica e recodifica os pixels. Substituir por:

1. Continuar a ler a tag `Orientation` (reutilizar o parsing já existente, não descartar).
2. Preservar os bytes originais do JPEG/PNG sem modificação.
3. No exportador PDF (`03_infra/src/export/stream.rs`, mesmo ficheiro alterado em P771 para o `clip_path`), calcular a matriz de transformação correspondente à orientação e compô-la com a matriz de posicionamento/escala já existente, antes do `Do`.
4. Confirmar que a composição de matrizes (posição + escala + orientação) não quebra o trabalho já feito em P767c/P769/P771 (ancoramento vertical, `fit`, `clip_rect`) — a ordem de composição importa.

### Validação — reproduzir as medições de P775

```bash
for orient in 1 2 3 4 5 6 7 8; do
  echo "=== Orientação $orient ==="
  # gerar/compilar/rasterizar/comparar, mesmo pipeline de P775
done
```

Esperado: AE ≈ 0 (ou no patamar do sanity-check de recompressão isolada, 83, se alguma recodificação continuar a ser inevitável nalgum caso) para as 8 orientações — não mais ~87000.

Confirmar também os JPEGs embebidos no PDF final (`pdfimages -list`) — devem ter as mesmas dimensões e tamanho de bytes do JPEG original, não um JPEG recodificado maior.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Parte B — Registo da origem (regra 9 do handoff)

P774 introduziu `apply_exif_rotation` (decodificação + recodificação) como a implementação da rotação EXIF. Confirmar por `git log`/leitura do relatório de P774 (já disponível nesta conversa) que foi de facto a origem, sem reescrever o passo:

```bash
git log --oneline -S "apply_exif_rotation" -- "03_infra/src/image_sizer.rs" "03_infra/src/world.rs"
```

### Tabela de rastreabilidade

| Passo | Commit | O que decidiu (de facto) |
|---|---|---|
| P772 | achado inicial | Classificou rotação EXIF como infra-estrutura sem efeito de língua (classificação incompleta, corrigida em P774). |
| P774 | `680cbbc2466412b6086cddaddc107b678e84e671` | Confirmou divergência de orientação; implementou correcção via decodificação + rotação de pixels + recodificação JPEG qualidade 95 — sem verificar se o vanilla usa a mesma estratégia. Introduziu o efeito colateral de AE~87k por diferença de recodificação. |
| P775 | `b7fef82476dfd0d8cba8d23ee3a273b787faf6bc` | Investigou o AE~87k; identificou a causa (recodificação vs preservação de bytes originais); fechou como "fora do âmbito" sem passar pela regra 1. |
| P776 (este passo) | — | Reverte a estratégia de P774 para matriz `cm` (sem recodificar), replicando o vanilla; registo de proveniência completo. |

**Lição a registar** (sem culpar, só descrever o padrão): P774 confirmou a divergência de orientação correctamente, mas implementou a correcção sem confirmar *como* o vanilla resolve o problema (decodificação vs matriz) — só *que* precisa de resolver. É o mesmo tipo de lacuna que originou a cadeia P763-P767 (implementar sem consultar o mecanismo real do vanilla primeiro), agora num passo que, ironicamente, fazia parte da cadeia de correcções motivada por essa mesma lição.

---

## Critério de fecho do passo

- [ ] Matriz de transformação EXIF confirmada contra o código-fonte do vanilla (`typst-pdf/src/image.rs`).
- [ ] Implementação revertida de decodificação/recodificação para composição de matriz `cm`.
- [ ] Bytes originais do JPEG/PNG preservados (confirmado via `pdfimages -list`, não só pela ausência de erro).
- [ ] As 8 orientações revalidadas com AE ≈ 0 (ou no patamar do sanity-check de recompressão, se recodificação for inevitável nalgum caso confirmado).
- [ ] Composição com `fit`/`clip_rect`/ancoramento vertical (P767c/P769/P771) confirmada sem regressão.
- [ ] Tabela de rastreabilidade preenchida, sem reescrever P772/P774/P775.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] L0 de `image` actualizado com a estratégia de matriz (não recodificação), antes do código.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p776.md`.

---

## Próximo passo

Se tudo fechar: a linha de trabalho de imagem (P769-P776) fecha definitivamente. Retomar a varredura da stdlib em P772a (lote 3).
