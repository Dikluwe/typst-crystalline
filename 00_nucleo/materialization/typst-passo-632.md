---
# P632 — Avaliação de `columns.rs` para a sequência de unificação

> **Passo:** 632
> **Data:** 2026-07-09
> **Foco:** P628 identificou `columns.rs` como o caso mais duvidoso da sequência de unificação — tem responsabilidades genuinamente diferentes (particionamento em modo segmentado/fluxo contínuo, ordenação regional de direcção RTL já corrigida em P626/P627, coordenação de notas de rodapé por coluna). Este passo decide, com sonda, se `columns.rs` merece um helper de nível superior próprio, ou se fica fora da unificação.
> **Tipo:** Sonda directa. Implementação só se a sonda confirmar que faz sentido.
> **Tamanho:** M para a sonda.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — esta é a parte da sequência que P628 já assinalou como mais arriscada; não implementar sem confirmar primeiro.

---

## Nota rápida, antes da sonda principal

Ficou por resolver de P631: `SubLayoutRegion` tem campos (`origin_x`, `width`) ignorados pela variante inline, sem aviso. Corrigir isto primeiro, é pequeno e independente do resto deste passo:

```bash
grep -n "struct SubLayoutRegion\|layout_sub_frame_inline" 01_core/src/rules/layout/sub_frame.rs
```

Decidir: separar uma struct mais pequena para a variante inline (sem `origin_x`/`width`), ou adicionar um `debug_assert!` que avisa se esses campos vierem diferentes do valor por defeito quando a variante inline é chamada.

---

## Sonda principal — `columns.rs`

### Mapear o que `columns.rs` faz que os outros quatro não fazem

```bash
grep -n "fn layout\|Segmented\|Flow\|footnote" 01_core/src/rules/layout/columns.rs | head -30
```

Confirmar, com números de linha, cada uma das responsabilidades já listadas por P628:

1. Divisão do body por `Content::Colbreak`.
2. Modo segmentado vs. modo fluxo contínuo.
3. Inversão da ordem de preenchimento em RTL (P626).
4. Divisão por página em RTL/LTR misto (P627).
5. Coordenação de notas de rodapé por coluna.
6. Tradução horizontal dos items de cada coluna para offsets absolutos.

### Confirmar se algum destes pontos já usa (ou podia usar) `layout_sub_frame`/`layout_sub_frame_inline`

Para cada uma das seis responsabilidades, perguntar: esta parte é sobre "decidir a estrutura de colunas" (não deve ser unificada), ou é sobre "colocar texto dentro de uma coluna já decidida" (pode reutilizar o helper)?

### Critério de fecho da sonda

- [ ] As seis responsabilidades confirmadas com `file:line`, não repetidas de memória de P628.
- [ ] Para cada uma: classificada como "estrutura de colunas" (fora do helper) ou "colocação de texto dentro da coluna" (candidata a reutilizar o helper).
- [ ] Decisão final: `columns.rs` ganha um helper de nível superior próprio, reutiliza parcialmente `layout_sub_frame` nalgumas partes, ou fica completamente fora da unificação.

---

## Implementação, condicional ao resultado da sonda

Só avançar se a sonda encontrar uma parte genuína de "colocação de texto dentro de uma coluna já decidida" que hoje duplica lógica do helper partilhado. Não forçar `columns.rs` inteiro a caber no mesmo molde só por consistência — a razão de P628 para não unificar à partida continua válida até prova em contrário.

---

## Validação, se houver implementação

```bash
cargo test -p typst-core columns
cargo test -p typst-core p552
cargo test -p typst-core p553
cargo test -p typst-core p595
cargo test -p typst-core p626
cargo test -p typst-core p627
cargo test --workspace
crystalline-lint .
```

Zero mudanças de snapshot inesperadas, mesma disciplina de P629/P631.

---

## Critério de fecho do passo

- [ ] Nota rápida sobre `SubLayoutRegion` resolvida.
- [ ] Sonda de `columns.rs` completa, com `file:line` para cada responsabilidade.
- [ ] Decisão final registada — com implementação, ou sem, com razão escrita das duas formas.
- [ ] Se houver implementação: testada sem regressão nos passos anteriores de colunas (P552, P553, P595, P626, P627).
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p632.md`, com hash do commit.

---

## Estado da sequência de unificação de layout

| Passo | Caminho | Estado |
|---|---|---|
| P593 | Largura de texto (todos) | Fechado |
| P625 | Alinhamento RTL (grid, box, place, footnote) | Fechado |
| P626/P627 | Ordem/direcção de colunas | Fechado |
| P629 | Helper base (`sub_frame.rs`, `SubLayoutRegion`) | Fechado |
| P631 | `boxed.rs` migrado | Fechado |
| **P632** | `columns.rs` avaliado | Em curso |

Depois deste passo, a sequência de unificação de layout, iniciada com a pergunta "porque é que estes casos de cascata não são apanhados", fica completa ou explicitamente delimitada.
