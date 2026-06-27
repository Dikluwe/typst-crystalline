---

# P478 — Sonda de estado geral + Footnotes Fase 2 (rodapé por acumulação)

> **Passo:** 478
> **Data:** 2026-06-27
> **Foco:** (A) Sonda de estado geral — varrer DEBT.md e inventário de cobertura para identificar itens pendentes fora das trilhas numeradas; (B) Footnotes Fase 2 — renderizar `Content::Footnote.body` no rodapé da página (P295.1), por acumulação sem 2-pass completo.
> **Tipo:** Sonda-first + Materialização condicional (M).
> **Tamanho:** XS (sonda) + M (footnote rodapé, ~40 min, condicional à sonda).
> **ADR-0117 Cláusula 4:** `Content::Footnote` em `entities/elements/footnote.rs` (P326); consumer Layouter Fase 1 em `rules/layout/mod.rs` — `footnote_counter: u32` + body descartado (P295). `Page` em `entities/layout_types.rs`. Verificar antes de propor estrutura nova.

---

## Contexto

**Sub-item A — Sonda de estado geral:**
P477 fechou ADR-0083 §"Operadores cor" e Trilhas 3, 7, 8. O panorama das trilhas numeradas é:

| Trilha | Estado |
|--------|--------|
| 1 — Numeração | COMPLETA |
| 2 — Referências cruzadas | COMPLETA |
| 3 — Selectors show rules | COMPLETA |
| 4 — Color/Gradient/Visualize | ~completa (ColorSpace runtime = scope-out) |
| 5 — Shaping rustybuzz | Épico XL declarado |
| 6 — Bibliografia Fase 2 | 4/5 (LoF/LoT page numbers = scope-out longo prazo) |
| 7 — Layout multi-região | COMPLETA |
| 8 — Refinos stdlib | COMPLETA |

O DEBT.md e o inventário de cobertura podem ter itens não cobertos por nenhuma trilha — por exemplo, `Bytes` stdlib (P398), `Version` stdlib (P401), `Decimal`/`Duration` user-facing, `calc` module, `TermItem`/`Terms`, `raw`/`Raw` refinos, `Divider`, `TermItem` layout. A sonda varre estes itens e determina o que é pendente real vs scope-out declarado.

**Sub-item B — Footnotes Fase 2:**
`Content::Footnote` (P295) emite apenas o marker `[N]` inline; body é silenciosamente descartado. A Fase 2 (P295.1) renderiza o body no rodapé.

O L0 documenta P295.1 como "requer 2-pass layout; magnitude L". Porém, uma abordagem por **acumulação sem 2-pass** pode ser S–M:

- Durante o layout, acumular bodies de footnote num buffer `pending_footnotes: Vec<(usize, Content)>`.
- No final de cada página (em `new_page()`), antes de fechar a página, render os footnotes acumulados na parte inferior.
- Sem refluxo de conteúdo (não recalcular o corpo da página): se o rodapé empurrar conteúdo para fora da página, não há re-quebra — o overflow fica truncado (scope-out declarado; paridade ADR-0054 graded).

Esta abordagem é magnitude S–M, não L.

---

## Sub-item A — Sonda de estado geral

### A.1 — Itens a verificar

| Item | Verificar em | Hipótese |
|------|-------------|----------|
| `calc` module (math functions) | `rules/stdlib/` | Provável stub ou ausente |
| `bytes` stdlib (len, at, slice) | `rules/stdlib/` | P398 modelou tipo; stdlib scope-out |
| `version` stdlib | `rules/stdlib/` | P401 modelou tipo; stdlib scope-out |
| `decimal`/`duration` stdlib | `rules/stdlib/` | Modelados mas sem funcs user-facing |
| `terms`/`term_item` layout | `rules/layout/` | Listado em roteiro como pendente |
| `raw`/`Raw` refinos (`lang`, `block`) | `rules/layout/` | Raw tem highlighting scope-out |
| `Divider` layout | `rules/layout/` | Linha horizontal (simples) |
| `TermItem` stdlib (`terms(...)`) | `rules/stdlib/` | Verificar estado |
| DEBT.md itens abertos | `DEBT.md` | Verificar lista actual |
| Inventário cobertura `ausente` restantes | cobertura vanilla | Verificar o que sobrou |

Todas as sondas com `grep`/`file:line` e conclusão: `implementado`, `stub`, ou `ausente`.

### A.2 — Resultado documentado no relatório

A sonda não produz código. Produz uma tabela actualizada de itens pendentes e define a ordem de prioridade para P479+.

---

## Sub-item B — Footnotes Fase 2 (acumulação sem 2-pass)

### B.1 — Estratura de acumulação

**Campo novo no Layouter** (`rules/layout/mod.rs`):

```rust
// P478 — buffer de footnotes pendentes para rodapé da página actual.
// Vec<(número, Content)> acumulado durante layout_content;
// esvaziado em new_page() após render do rodapé.
pub(super) pending_footnotes: Vec<(usize, Content)>,
```

Inicializado como `Vec::new()` em `Layouter::new()`.

### B.2 — Arm `Content::Footnote` actualizado

**Ficheiro:** `rules/layout/mod.rs` ou `rules/layout/footnote.rs`

Antes (P295 Fase 1):
```rust
Content::Footnote(e) => {
    self.footnote_counter += 1;
    let n = self.footnote_counter;
    self.layout_content(&Content::text(format!("[{}]", n)));
}
```

Depois (P478 Fase 2):
```rust
Content::Footnote(e) => {
    self.footnote_counter += 1;
    let n = self.footnote_counter;
    // Marker inline — preservado.
    self.layout_content(&Content::text(format!("[{}]", n)));
    // Acumular body para rodapé — P478.
    self.pending_footnotes.push((n, *e.body.clone()));
}
```

### B.3 — Render do rodapé em `new_page()`

**Ficheiro:** `rules/layout/mod.rs` — método `new_page()`

Antes de avançar para a nova página, renderizar o rodapé:

```rust
fn new_page(&mut self) {
    // P478 — Render footnotes acumulados antes de fechar a página.
    if !self.pending_footnotes.is_empty() {
        self.render_footnote_area();
    }
    // ... lógica existente de new_page ...
    self.pending_footnotes.clear();
}

fn render_footnote_area(&mut self) {
    // 1. Linha separadora (opcional; paridade vanilla default).
    let separator = Content::text("—".repeat(20));  // ou Shape::Line
    self.layout_content(&separator);

    // 2. Para cada footnote acumulado:
    for (n, body) in self.pending_footnotes.drain(..) {
        // Prefixo numérico como superscript + body.
        let prefix = Content::text(format!("[{}] ", n));
        self.layout_content(&prefix);
        self.layout_content(&body);
        // Newline entre footnotes.
        self.layout_content(&Content::text("\n"));
    }
}
```

**Problema chave:** o cursor.y no momento de `new_page()` pode já ter excedido `page_bottom`. O render do rodapé empurra ainda mais. Sem refluxo, o excesso é truncado.

**Solução de subset:** reservar espaço para o rodapé antes de começar o layout do corpo da página. Mas isso requer saber quantas footnotes haverá — que só se sabe após o layout. O trade-off aceite:

**Abordagem B simplificada:** renderizar o rodapé no início da **página seguinte** (não no final da actual). Isso é menos fiel ao vanilla (que põe no rodapé da página onde a footnote ocorre) mas evita o problema de refluxo e é magnitude S:

```rust
fn layout_content(part: &Content) {
    // ... no início de layout_content (após new_page):
    if !self.pending_footnotes.is_empty() && acabou_de_abrir_nova_pagina {
        self.render_footnote_area();
        self.pending_footnotes.clear();
    }
    // ...
}
```

**Decisão de subset antes de implementar (via sonda):** verificar se a página tem campo `footnote_area` ou se o Layouter tem acesso a `page_bottom - cursor_y` no momento de `new_page()`. Isso determina qual abordagem é viável sem refluxo.

### B.4 — Linha separadora

O vanilla emite uma linha horizontal fina (`\noindent\rule{2in}{0.4pt}`) antes das footnotes. Subset P478: linha de texto simples `"─" × N` ou `FrameItem::Shape(Line)`. A sonda deve verificar se `FrameItem::Line` existe.

### B.5 — Scope-out declarado

- **Refluxo:** sem recalcular posições do corpo da página. Se o rodapé não cabe, trunca.
- **Footnote que excede uma página** (P295.2) — scope-out.
- **Numeração por página** (vanilla recomeça em 1 por página) — cristalino continua sequência global.
- **`numbering:` configurável** — scope-out P295.

---

## Tests

### Sub-item A

Nenhum teste de código. Relatório de sonda com tabela de itens e estado.

### Sub-item B

- **L1:** `Layouter::pending_footnotes` começa vazio.
- **L2:** Documento com uma footnote — layout do corpo inclui marker `[1]`; `pending_footnotes` acumula o body.
- **L2:** `render_footnote_area()` emite separador + `[1] body`.
- **L2:** Após `new_page()`, `pending_footnotes` está vazio.
- **L2:** Documento com 2 footnotes na mesma página — ambos acumulados; ambos renderizados no rodapé em ordem.
- **L2:** Documento com footnotes em páginas diferentes — cada página tem o seu rodapé.
- **L3 (E2E):** Documento simples com `#footnote[Nota de rodapé]` — output tem marker `[1]` inline e texto `[1] Nota de rodapé` mais abaixo.

---

## Spec L0

### Sub-item A

- Nenhum ficheiro L0 novo. Tabela de itens pendentes no relatório.

### Sub-item B

- `entities/elements/footnote.md` — §"Fase 2 P478" adicionada: `pending_footnotes` no Layouter; `render_footnote_area`; scope-outs declarados.
- `rules/layout/mod.md` (ou equivalente) — `pending_footnotes: Vec<(usize, Content)>` documentado.

---

## Critério de fecho

- [ ] Sonda A executada: DEBT.md varrido; inventário cobertura varrido; tabela de itens pendentes produzida com estado e prioridade.
- [ ] Sonda B: `FrameItem::Line` existe (sim/não); `page_bottom - cursor_y` acessível em `new_page()` (sim/não); conclusão sobre abordagem (final-de-página vs início-de-próxima).
- [ ] `pending_footnotes: Vec<(usize, Content)>` adicionado ao Layouter.
- [ ] Arm `Content::Footnote` acumula body em `pending_footnotes`.
- [ ] `render_footnote_area()` implementado com separador + entries numeradas.
- [ ] `new_page()` chama `render_footnote_area()` quando `pending_footnotes` não vazio; limpa o buffer.
- [ ] Abordagem (final-de-página vs início-de-próxima) fixada e documentada como divergência de paridade se necessário.
- [ ] 7+ testes verdes (1 L1 + 4 L2 + 2 L3).
- [ ] Spec L0 actualizada (2 ficheiros).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **P295.1 FECHADO** — `Content::Footnote.body` renderizado no rodapé.

---

## Próximo passo

Com P478:

- Se sonda A revelar itens S–M pendentes (calc, terms, divider, etc.) → P479 materializa o mais prioritário.
- Se sonda A revelar tudo é scope-out ou pequeno → P479 pode ser uma sonda de paridade geral (varrer diferenças vs vanilla) ou iniciar Épico Trilha 5.

---

## Estado pós-P477 (para referência)

| Passo | Descrição | Estado |
|-------|-----------|--------|
| P476 | Operadores cor 4/6 + sonda Trilha 5 | ✅ FECHADO |
| P477 | saturate/desaturate + constantes cor | ✅ FECHADO |
| **P478** | Sonda geral + Footnotes Fase 2 | 🔄 EM PREPARAÇÃO |

**Trilhas 1-4, 7, 8: COMPLETAS ou ~completas.**
**Trilha 5: Épico XL (shaping rustybuzz).**
**Trilha 6: 4/5.**
**ADR-0083: 3/4 scope-outs fechados.**
**P295: Fase 1 completa; Fase 2 pendente (P478).**
**Inventário de débitos: LIMPO.**
