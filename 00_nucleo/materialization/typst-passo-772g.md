---
# P772g — Correcção: `layout_place` duplica origem de célula quando aninhado em `Content::Align`/outros wrappers

> **Passo:** 772g
> **Data:** 2026-07-16
> **Foco:** P772f identificou, com instrumentação directa e confirmação por coordenadas, que `layout_place` calcula coordenadas absolutas mesmo quando aninhado dentro de `layout_align` (que já soma a sua própria origem absoluta), duplicando o deslocamento. A correcção parcial de P763f (comentário "P763f" no código) só cobre o ramo de fallback (`regions.cell` ausente); o ramo principal (`Some(cx), Some(cy), Some(cell)`) nunca verifica `in_sub_frame`. Isto não é específico de `grid` — afecta qualquer combinação de `place()` aninhado dentro de `align`/`pad`/`box`/`stack`/`columns` que use o mesmo padrão de sub-frame.
> **Tipo:** Implementação directa (causa já confirmada por instrumentação e coordenadas em P772f).
> **Tamanho:** M/L — generalizar a correcção exige auditar todos os wrappers que chamam `layout_sub_frame` com `origin_x: 0.0`, não só o caso de `align` já confirmado.
> **ADR-0108 EM VIGOR** — validar por coordenadas em cada wrapper, não assumir que a correcção generaliza automaticamente.
> **Regra 5 do handoff** — checklist de sub-layouts obrigatório, dado mexer no mecanismo de posicionamento partilhado.
> **Dependências:** P772f (causa identificada, commit `2c7025a9a949ecafda8a2550c8c3692db3c9ad0b`), P763f (correcção parcial anterior, mesmo ficheiro).

---

## Sonda — confirmar o mecanismo exacto e levantar todos os wrappers afectados

```bash
grep -n "layout_sub_frame\|origin_x: 0.0\|in_sub_frame" 01_core/src/rules/layout/*.rs
```

Para cada wrapper encontrado (`align`, `pad`, `box`, `stack`, `columns`, outros), confirmar se chama `layout_sub_frame` com `origin_x: 0.0` e depois recompõe `target_x` a partir de `line_start_x`/equivalente — o mesmo padrão que causa a duplicação em `align`. Não assumir que todos têm o mesmo bug; testar cada um.

```bash
grep -n "match scope" 01_core/src/rules/layout/placement.rs
```

Confirmar a estrutura exacta do `match` em `layout_place` e onde o ramo `(Some(cx), Some(cy), Some(cell))` precisa da mesma verificação de `in_sub_frame` já usada no ramo de fallback.

---

## Implementação

Generalizar a correcção de P763f para o ramo principal: quando `in_sub_frame` for verdadeiro (independentemente de `regions.cell` estar `Some` ou não), usar `(0.0, 0.0)` como origem local em vez de coordenadas absolutas, deixando o wrapper externo (align/pad/box/stack/columns) responsável por recompor a posição final — não `layout_place` a antecipar essa composição.

---

## Validação — os 3 repros de P772f mais os wrappers adicionais confirmados na sonda

```bash
# place directo (sem align) dentro de block — já batia parcialmente
# align + place dentro de block — o caso confirmado com duplicação
# caso completo do grid original (2 colunas, block+align+place+circle)
```

Para cada um, `mutool trace` antes/depois, confirmando ΔY≈0 e ΔX≈0 contra o vanilla, com tolerância sub-pt.

```bash
cargo test --workspace
crystalline-lint .
```

### Checklist de sub-layouts (regra 5)

Repetir para `place()` aninhado em `pad`, `box`, `stack`, `columns` — não só `align`/`grid` — usando os wrappers confirmados na sonda como afectados.

---

## Critério de fecho do passo

- [x] Todos os wrappers que usam `layout_sub_frame` com `origin_x: 0.0` levantados e testados individualmente.
- [x] Ramo principal de `layout_place` — **decisão do humano (Opção A)**: em vez de generalizar `in_sub_frame` em `layout_place` (Opção B, que exigiria estado novo no `Layouter`), corrigidos os wrappers recompositores (`layout_align`, footnotes) para se comportarem como consumidores absolutos. `layout_place` fica inalterado — ver relatório §1.3 para a justificação completa.
- [x] Os 3 repros de P772f validados com coordenadas exactas (ΔX/ΔY ≈ 0).
- [x] Cada wrapper adicional confirmado como afectado (`align`, footnotes) ou não (`columns`, `box`, `pad`, `stack`) e testado individualmente.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` zero violações.
- [x] L0 de `placement`/`layout` actualizado antes do código (`00_nucleo/prompts/rules/layout.md`, hash `b1739fd9`).
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772g.md`.

---

## Próximo passo

Retomar a varredura da stdlib (`visualize::image::svg`, `foundations::scope`, `text::font::*`), ou os achados extra de P772f (header/footer inventados, código órfão), conforme as decisões já tomadas sobre esses dois itens.
