---

# P505 — Materialização de Indentação em Listas e Enums (P500 Pendente)

> **Passo:** 505
> **Data:** 2026-06-29
> **Foco:** Fechar empiricamente os gaps de indentação em listas e enums identificados no audit P500. Não declarar conclusão — medir antes e depois de cada sub-tarefa.
> **Tipo:** Implementação M-size com validação via bateria P500.
> **Tamanho:** M (~45 min de implementação + 15 min de validação).
> **ADR-0075 ACEITE** — mecanismo de comparação via `typst query --format json` vs `Introspector::query_*`.
> **ADR-0054 ACEITE** — graded parity: MATCH / DIFF / ERRO_DESCRITIVO / PANIC / AUSENTE.
> **ADR-0109 ACEITE** — atomização de código.
> **Dependências:** P504 (novas funcionalidades 0.15.0 fechadas), P502 (audit P500 fechado), P470 (list/enum base).

---

## 1. Contexto

O P502 (audit P500) identificou que `list` e `enum` não suportam os parâmetros de indentação do vanilla:

- `indent` — indentação do marker em relação ao pai.
- `body-indent` — indentação do corpo do item em relação ao marker.
- `tight` — se `true`, sem espaçamento entre itens; se `false`, espaçamento de parágrafo.

O P504 fechou `list.marker-align` (0.15.0), mas `indent`/`body-indent`/`tight` são funcionalidades que existem desde antes do 0.15.0 e continuam pendentes.

---

## 2. Metodologia

Implementar os parâmetros de indentação, criar ficheiros `.typ` de teste, correr contra vanilla 0.15.0 e cristalino, e classificar o resultado.

```bash
# Baseline P504 (antes de tocar código):
cargo test --test structural_parity p504_audit_novas_funcionalidades_0150

# Após implementação:
cargo test --test structural_parity p505_indentacao_listas_enums
```

Classificar cada resultado: `MATCH` | `DIFF` | `ERRO_DESCRITIVO` | `PANIC` | `AUSENTE`.

---

## 3. Estado Baseline (P504)

```typst
// test-list-advanced.typ (P500)
#list(
  indent: 1.5em,
  body-indent: 0.5em,
  tight: false,
  [Item A],
  [Item B],
)

// test-enum-advanced.typ (P500)
#enum(
  indent: 1.5em,
  body-indent: 0.5em,
  tight: false,
  [Primeiro],
  [Segundo],
)
```

- **Vanilla 0.15.0:** `ok` — listas/enum com indentação renderizam corretamente.
- **Cristalino (pré-505):** `ERRO_DESCRITIVO` — `argumento nomeado inesperado: 'indent'`.
- **Classificação P504:** **AUSENTE**

---

## 4. Diagnóstico de Causa Raiz

O cristalino implementou `ListElem` e `EnumElem` em P470, mas sem os campos de indentação. O gap é:

1. **H1:** `ListElem`/`EnumElem` não têm campos `indent`, `body-indent`, `tight`.
2. **H2:** O layout de listas/enum não aplica indentação — coloca marker e body na mesma coluna.

**Verificação rápida:**
```bash
rg -n "struct ListElem" src/entities/elements/ --type rs
rg -n "struct EnumElem" src/entities/elements/ --type rs
rg -n "indent\|body_indent\|tight" src/rules/layout/lists.rs --type rs
```

---

## 5. Implementação

### 5.1 — Adicionar Campos a `ListElem` e `EnumElem`

**Arquivo alvo:** `src/entities/elements/list.rs` e `src/entities/elements/enum.rs`

```rust
// list.rs
pub struct ListElem {
    pub marker: Option<Value>, // Str ou Array<Content>
    pub indent: Option<Length>,
    pub body_indent: Option<Length>,
    pub tight: Option<bool>,
    pub items: Vec<Content>,
}

// enum.rs
pub struct EnumElem {
    pub start: Option<i64>,
    pub numbering: Option<Str>,
    pub indent: Option<Length>,
    pub body_indent: Option<Length>,
    pub tight: Option<bool>,
    pub items: Vec<Content>,
}
```

### 5.2 — Atualizar Construtores Nativos

**Arquivo alvo:** `src/rules/stdlib/structural.rs`

```rust
fn native_list(args: Args) -> SourceResult<Value> {
    let marker = args.named.get("marker").cloned();
    let indent = args.named.get("indent").and_then(|v| v.cast::<Length>().ok());
    let body_indent = args.named.get("body-indent").and_then(|v| v.cast::<Length>().ok());
    let tight = args.named.get("tight").and_then(|v| v.cast::<bool>().ok());
    let items = args.expect_positional::<Content>()?;

    Ok(Value::Content(Content::List(ListElem {
        marker,
        indent,
        body_indent,
        tight,
        items,
    })))
}

fn native_enum(args: Args) -> SourceResult<Value> {
    let start = args.named.get("start").and_then(|v| v.cast::<i64>().ok());
    let numbering = args.named.get("numbering").and_then(|v| v.cast::<Str>().ok());
    let indent = args.named.get("indent").and_then(|v| v.cast::<Length>().ok());
    let body_indent = args.named.get("body-indent").and_then(|v| v.cast::<Length>().ok());
    let tight = args.named.get("tight").and_then(|v| v.cast::<bool>().ok());
    let items = args.expect_positional::<Content>()?;

    Ok(Value::Content(Content::Enum(EnumElem {
        start,
        numbering,
        indent,
        body_indent,
        tight,
        items,
    })))
}
```

### 5.3 — Atualizar Layout de Listas

**Arquivo alvo:** `src/rules/layout/lists.rs` (ou onde o layout de listas é feito)

O layout de listas no cristalino precisa de:
1. **Indentação do marker:** `indent` define o deslocamento horizontal do marker em relação à margem esquerda.
2. **Indentação do body:** `body-indent` define o deslocamento horizontal do corpo do item em relação ao marker.
3. **Modo tight:** `tight: true` → sem espaçamento entre itens; `tight: false` → espaçamento de parágrafo.

```rust
fn layout_list(list: &ListElem, ctx: &mut LayoutContext) -> Vec<Frame> {
    let indent = list.indent.unwrap_or(Length::zero());
    let body_indent = list.body_indent.unwrap_or(Length::zero());
    let tight = list.tight.unwrap_or(true);

    let mut frames = Vec::new();
    for (i, item) in list.items.iter().enumerate() {
        // Layout do marker
        let marker_frame = layout_marker(list, i, ctx);
        let marker_width = marker_frame.width();

        // Layout do body com indentação
        let body_x = indent + marker_width + body_indent;
        let body_frame = layout_content(item, ctx, body_x);

        // Combinar marker + body
        let mut item_frame = Frame::new();
        item_frame.place(marker_frame, Point::new(indent, 0));
        item_frame.place(body_frame, Point::new(body_x, 0));

        frames.push(item_frame);

        // Espaçamento entre itens
        if !tight && i < list.items.len() - 1 {
            frames.push(Frame::spacer(ctx.paragraph_spacing()));
        }
    }
    frames
}
```

**Nota:** Se o layout de listas do cristalino é mais simples (sem `Frame` explícito), adaptar ao modelo existente. O princípio é: marker deslocado por `indent`, body deslocado por `indent + marker_width + body_indent`.

### 5.4 — Atualizar Layout de Enums

**Arquivo alvo:** `src/rules/layout/enums.rs` (ou onde o layout de enums é feito)

Analogo ao layout de listas, mas com numbering:

```rust
fn layout_enum(enum_elem: &EnumElem, ctx: &mut LayoutContext) -> Vec<Frame> {
    let indent = enum_elem.indent.unwrap_or(Length::zero());
    let body_indent = enum_elem.body_indent.unwrap_or(Length::zero());
    let tight = enum_elem.tight.unwrap_or(true);
    let start = enum_elem.start.unwrap_or(1);
    let numbering = enum_elem.numbering.as_deref().unwrap_or("1.");

    let mut frames = Vec::new();
    for (i, item) in enum_elem.items.iter().enumerate() {
        let number = format_numbering(start + i as i64, numbering);
        let marker = Content::Text(number);
        let marker_frame = layout_content(&marker, ctx);
        let marker_width = marker_frame.width();

        let body_x = indent + marker_width + body_indent;
        let body_frame = layout_content(item, ctx, body_x);

        let mut item_frame = Frame::new();
        item_frame.place(marker_frame, Point::new(indent, 0));
        item_frame.place(body_frame, Point::new(body_x, 0));

        frames.push(item_frame);

        if !tight && i < enum_elem.items.len() - 1 {
            frames.push(Frame::spacer(ctx.paragraph_spacing()));
        }
    }
    frames
}
```

---

## 6. Validação

Rodar `test-list-advanced.typ` e `test-enum-advanced.typ` contra vanilla e cristalino. Esperado:

- `list(indent: 1.5em, body-indent: 0.5em, tight: false, ...)` → **MATCH**
- `enum(indent: 1.5em, body-indent: 0.5em, tight: false, ...)` → **MATCH**
- `list(tight: true, ...)` → **MATCH** (sem espaçamento entre itens)
- `list(tight: false, ...)` → **MATCH** (com espaçamento de parágrafo)
- `list(indent: auto, ...)` → **MATCH** (default do vanilla, se aplicável)

---

## 7. Formato do Relatório de Resultados

```
| 505a list indent | Vanilla: ok | Cristalino: ok | MATCH | indent + body-indent + tight |
| 505b enum indent | Vanilla: ok | Cristalino: ok | MATCH | indent + body-indent + tight |
```

Colunas: `sub-tarefa | vanilla | cristalino | classificação | notas`

---

## 8. Testes de Não-Regressão

Após as 2 sub-tarefas, rodar a bateria completa P490 + P500 + P504 (33+ ficheiros):

| Ficheiro | Vanilla | Cristalino pré-505 | Esperado pós-505 | Δ |
|---|---|---|---|---|
| test-list-advanced.typ | ok | AUSENTE | ok | **AUSENTE → MATCH** |
| test-enum-advanced.typ | ok | AUSENTE | ok | **AUSENTE → MATCH** |
| (outros 31+) | — | MATCH | MATCH | preservados |

**AUSENTEs restantes esperados:** 2 - 2 = **0** (list/enum indent fechados).  
**PANICs esperados:** **0** (preservado).

---

## 9. Critério de Fecho

- [ ] 505a implementado: `list(indent:, body-indent:, tight:)` funciona.
- [ ] 505b implementado: `enum(indent:, body-indent:, tight:)` funciona.
- [ ] 2 testes unitários novos passam (`p505_list_indent`, `p505_enum_indent`).
- [ ] Bateria P500: 2 AUSENTEs viraram MATCH.
- [ ] AUSENTEs restantes: **0** (todos os AUSENTEs do P500 fechados).
- [ ] PANICs: 0 (preservado).
- [ ] Documentação atualizada (`rules/stdlib/structural.md` para list/enum; `rules/layout/lists.md` e `rules/layout/enums.md` se existirem).
- [ ] ADR-0107 checklist atualizado (2 itens marcados implementado).
- [ ] Sentinela `p505_indentacao_listas_enums` adicionada em `lab/parity/tests/structural_parity.rs`.
- [ ] `00_nucleo/diagnosticos/paridade-funcional-p505.md` produzido com tabela de resultados.

---

## 10. Próximo Passo (P506)

Com P505 fechado, todos os AUSENTEs do P500 estão resolvidos. O único gap restante do projeto é:

| Gap | Tamanho | Recomendação |
|---|---|---|
| `state.update/get` + `context` + `counter` | L | P506 = runtime state (trilha separada) ou scope-out |

**Recomendação:** P506 = **DEBT-42 Benchmark** — comparar tempo cristalino vs vanilla 0.15.0, avaliar impacto da passagem dupla do P498 e das novas funcionalidades do P504/P505. O gap `state/counter/context` é L-size e pode ser scope-out como divergência arquitetural documentada (o cristalino não tem runtime state mutável).

Alternativa: P506 = **Documentação de paridade final** — produzir relatório consolidado P490-P505 com matriz de cobertura completa.

---

## A. Apêndice — Referência Rápida

```typst
// 505a: list com indentação
#list(
  indent: 1.5em,
  body-indent: 0.5em,
  tight: false,
  [Item A com texto suficiente para testar a indentação do corpo em relação ao marker.],
  [Item B],
)

// 505b: enum com indentação
#enum(
  indent: 1.5em,
  body-indent: 0.5em,
  tight: false,
  [Primeiro item com texto longo para testar indentação.],
  [Segundo item],
)

// tight: true (sem espaçamento)
#list(tight: true, [A], [B], [C])

// tight: false (com espaçamento de parágrafo)
#list(tight: false, [A], [B], [C])
```
