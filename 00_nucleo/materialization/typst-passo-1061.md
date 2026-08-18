# L0 — Passo 1061: Margin Collapsing Parágrafo ↔ Bloco (correcção real)

**Gate**: `ADR-0127` — mudança de comportamento por defeito (afecta espaçamento
vertical de qualquer documento com parágrafo seguido/precedido de bloco). **Requer
confirmação do dono antes de codificar.**

**Base**: mecanismo real confirmado no P1060 (`typst-passo-1060-relatorio.md`), não
suposição. Substitui o L0 especulativo anterior (P1060, retirado).

---

## 1. Causa raiz confirmada

Em `01_core/src/compiler/layout/sequence.rs`, o consumer de `Sequence` reseta
`prev_block_below_pending`/`block_chain_active` para **qualquer** parte que não seja
`Content::Block`/`Content::Shape`:

```rust
if !matches!(part, Content::Block { .. } | Content::Shape(_)) {
    layouter.block_chain_active = false;
    layouter.prev_block_below_pending = 0.0;
}
```

`Content::Parbreak` cai neste braço — mesmo que o P1057 registasse o espaçamento do
parágrafo como pendente, o `Sequence` consumer apaga-o logo a seguir, antes de
qualquer bloco seguinte o poder ler.

Em `01_core/src/compiler/layout/mod.rs`, o braço `Content::Parbreak` não lê nem
escreve `prev_block_below_pending`/`block_chain_active` — aplica `extra_spacing`
directamente a `cursor_y`, sem participar no colapso `max()` de `P250`.

Os 4 casos medidos no P1059 (`typst-passo-1059-relatorio.md`) decorrem exactamente
disto:

| Caso | Transição | Causa |
|---|---|---|
| 1 | Parágrafo → Heading | `Parbreak` não regista pendente; `Heading` não lê pendente (não confirmado — ver §4) |
| 2 | Parágrafo → Block(spacing: 2em) | `Parbreak` não regista pendente para o `Block` colapsar |
| 3 | Block(spacing: 0.5em) → Parágrafo | `Sequence` consumer apaga o pendente do `Block` antes do `Parbreak` o ler |
| 4 | Parágrafo → Block(spacing: 0.5em) | mesma causa do Caso 2 |

---

## 2. Mecanismo autorizado

### 2.1 `Content::Parbreak` (`mod.rs`) — ler e escrever o pendente

Substituir o avanço puramente aditivo por participação no colapso `max()`, com uma
assimetria deliberada face ao `Content::Block`: quando não há pendente activo
(`block_chain_active == false`), o `Parbreak` **não suprime a zero** (ao contrário do
`Block`) — aplica o seu próprio `spacing_pt` na íntegra. A supressão a zero do
`Block` existe para o primeiro bloco de uma `Sequence` (nada para colapsar contra);
o `Parbreak` já é gated por `had_items` (só corre entre duas linhas de texto reais),
logo não tem o mesmo caso de "primeiro elemento" a proteger.

```rust
Content::Parbreak => {
    let had_items = !self.regions.current.current_line.is_empty();
    self.flush_line();
    if had_items {
        // ... cálculo de spacing_pt e leading_pt inalterado ...
        let extra_spacing = (spacing_pt - leading_pt).max(0.0);

        // NOVO — colapso contra pendente de um Block anterior (Caso 3/4).
        let gap = if self.block_chain_active {
            self.prev_block_below_pending.max(extra_spacing)
        } else {
            extra_spacing
        };
        let advance = (gap - self.prev_block_below_pending).max(0.0);
        self.regions.current.cursor_y += Pt(advance);

        // NOVO — regista o próprio spacing como pendente, para um
        // Block/Heading seguinte colapsar contra ele (Caso 1/2).
        self.prev_block_below_pending = extra_spacing;
        self.block_chain_active = true;
    }
}
```

**Pergunta em aberto (não decidir sozinho)**: o valor a registar como pendente deve
ser `extra_spacing` (já líquido de `leading`) ou `spacing_pt` bruto? O `Block` regista
`below_pt`/`above_pt` brutos (sem descontar nada). Registar `extra_spacing` mantém a
paridade com o que o `Parbreak` já aplica a si próprio; mas se o vanilla usa o
`spacing` bruto na hierarquia de `weakness` (P1059 §1.2, `weakness = 4` para
`par.spacing`), pode ser `spacing_pt` que devia entrar no `max()`. **Confirmar contra
o vanilla antes de codificar** — os 4 casos do P1059 servem de oráculo directo.

### 2.2 `Content::Sequence` (`sequence.rs`) — não resetar em `Parbreak`

```rust
if !matches!(part, Content::Block { .. } | Content::Shape(_) | Content::Parbreak) {
    layouter.block_chain_active = false;
    layouter.prev_block_below_pending = 0.0;
}
```

Acrescenta `Content::Parbreak` à lista de tipos que preservam a cadeia. Não altera o
comentário `P767a` sobre `Content::Shape` — só acrescenta um braço.

### 2.3 `Content::Heading` — scope explícito, não silencioso

**Não confirmado nesta investigação.** O relatório do P1059 (§1.2) diz que no
vanilla `HeadingElem` define `above` via `BlockElem::above`, mas não sabemos se
`compiler/layout/heading.rs` no cristalino já lê/escreve
`prev_block_below_pending`/`block_chain_active`, ou se aplica o seu espaçamento
`above` de forma directa (como o `Parbreak` fazia antes desta correcção).

**Duas opções**:
- (a) Este L0 cobre só Caso 2/3/4 (`Block`↔`Parágrafo`); Caso 1
  (`Parágrafo→Heading`) fica scope-out explícito para um passo seguinte, depois de
  ler `compiler/layout/heading.rs` real.
- (b) Pedir `compiler/layout/heading.rs` agora e estender este mesmo L0 para cobrir
  os 4 casos de uma vez.

Recomendo (a) — evita repetir o erro de escrever mecanismo sobre arquivo não lido.

---

## 3. Critérios de verificação

Re-executar os 4 casos medidos no P1059:

1. Parágrafo → Heading — **fora de âmbito se opção (a)**, ver §2.3.
2. Parágrafo → Block(spacing: 2em) — Δy alvo: 0.0000 pt.
3. Block(spacing: 0.5em) → Parágrafo — Δy alvo: 0.0000 pt.
4. Parágrafo → Block(spacing: 0.5em) — Δy alvo: 0.0000 pt.

Mais:

5. Re-rodar o decalque do corpus canónico completo (7 documentos) — zero regressão
   nos casos que já tinham paridade (P1057/P1059).
6. Teste de não-regressão do P864 (agrupamento por `Parbreak` entre `ListItem`/
   `EnumItem`/`TermItem`) — a mudança em `sequence.rs` toca o mesmo consumer;
   confirmar que `parbreak_since_last_item`/`last_seen_item_group` (campos
   distintos, não tocados por este L0) continuam a funcionar sem alteração.
7. `crystalline-lint .` — 0 erros.
8. `cargo test --workspace` — 100% pass.

---

## 4. Scope-out explícito

- `Content::Heading` (Caso 1) — ver §2.3, decisão pendente.
- `par.leading` intra-parágrafo — inalterado, fora de âmbito (igual ao L0 anterior).
- `Content::Shape` já está coberto pelo mecanismo existente (P767a) — não tocado.
