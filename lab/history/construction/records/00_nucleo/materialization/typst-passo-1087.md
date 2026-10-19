# L0 — Passo 1087: Corrigir Espaçamento Entre Blocos de Equação (P1086)

**Gate**: `ADR-0127` — mudança de comportamento por defeito (todo documento
com 2+ equações de bloco consecutivas, ou equação seguida de heading, é
afectado). **Requer confirmação do dono antes de codificar.**

**Base**: P1086 (investigação, causa dupla confirmada com código real).

---

## 1. Duas correcções de natureza diferente, não misturar

### 1.1 Literal `1.2` duplicado → `BLOCK_SPACING`

`01_core/src/compiler/layout/equation.rs:119,335`:
```rust
let spacing = Pt(self.style.size.val() * 1.2);
```
Substituir por `vanilla_defaults::BLOCK_SPACING` (já existe desde P1058, não
precisa de nova proveniência). Trivial, baixo risco.

### 1.2 `TextStyle::default()` → `glyph_ink_bounds` com estilo real

`01_core/src/compiler/math/layout/mod.rs:537-544`:
```rust
FrameItem::Glyph { pos, x_advance, size, .. } => {
    extent.width = extent.width.max(pos.x.val() + x_advance.val());
    let up = self.metrics.cap_height(*size, &TextStyle::default());
    extent.ascent = extent.ascent.max(up.val() - pos.y.val());
    extent.descent = extent.descent.max(pos.y.val());
}
```
`style` e `glyph_id` já estão disponíveis no padrão (`FrameItem::Glyph { pos,
x_advance, size, style, glyph_id, .. }` — confirmar nomes exactos dos campos
contra a definição real de `FrameItem::Glyph` antes de escrever, não presumir).
Substituir por `self.metrics.glyph_ink_bounds(*glyph_id, *size, style)`
(mecanismo já existente, P952b — não inventar nada novo).

**Contexto histórico a registar no relatório**: isto não é bug nunca antes
identificado — o próprio comentário no código ("Aproximação documentada
P813") mostra que era scope-out consciente, à espera de `glyph_ink_bounds`
(chegado só em P952b, depois de P813). É débito técnico já rastreado, nunca
fechado quando a ferramenta certa passou a existir — não atribuir isto a
descuido silencioso.

## 2. `layout_equation` não participa do protocolo de colapso

P1086 também identificou que `layout_equation` não usa
`block_chain_active`/`prev_block_below_pending` (mecanismo do P1061,
estendido a heading no P1063). Confirmar se isto é uma terceira correcção
separada ou se, ao corrigir 1.1/1.2, a Causa Raiz A (cabeçalho→eq1, +1.96pt)
já fica resolvida sem precisar de mexer no protocolo de colapso. **Não
presumir** — o P1086 descreveu a Causa A como "não participa do protocolo" E
"soma incondicionalmente spacing+ascent" como duas facetas do mesmo problema;
confirmar se corrigir só o valor (1.1) já resolve, ou se a integração com o
protocolo (mesmo padrão de `heading.rs` no P1063) é mesmo necessária à parte.

## 3. Medição

Reproduzir a seção 30 (ou equivalente, já que os PDFs originais foram
perdidos por reinício de sandbox — usar um documento novo com a mesma
estrutura: heading + 3 equações de bloco com sub/sobrescrito de alturas
diferentes) e confirmar os 3 deltas do P1086 chegam a ~0.00pt.

## 4. Critérios de verificação

1. `cabeçalho→eq1`: Δ de +1.96pt → ~0.00pt.
2. `eq1→eq2`: Δ de −2.09pt → ~0.00pt.
3. `eq2→eq3`: Δ de −2.02pt → ~0.00pt.
4. Re-rodar os casos já fechados de P1059-1063 (margin collapsing
   parágrafo/bloco/heading) — zero regressão, já que `math/layout/mod.rs` é
   consumido por outros caminhos além de equações de bloco isoladas.
5. `crystalline-lint .` — 0 erros.
6. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- Nomes reais dos campos de `FrameItem::Glyph` confirmados antes de editar.
- §2 respondido — protocolo de colapso, terceira correcção ou resolvido de
  graça pelas outras duas.
- Contexto histórico P813→P952b citado no relatório final.
- 6 critérios de verificação confirmados.
