# Passo 408 — Materialização: `smallcaps` (S-M)

**Tipo**: Materialização (L1 — Content variant + stdlib + consumer stub; zero tipo novo; zero I/O).
**Data**: 2026-06-22.
**Padrão**: diagnóstico-primeiro (sonda 389 + Tabela A.3); medir-antes-de-decidir (ADR-0108).
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0107 (paridade linguagem), ADR-0054 (graded scope-out — consumer real requer shaping OpenType `smcp` / `c2sc`).
**Sonda fonte**: `typst-sonda-ausentes-ordem-passo-389.md` §2D — `smallcaps` ausente; Tabela A.3 linha 376.

> **Nota de numeração.** Um passo só. Não numerar à frente.

---

## 1. Contexto

No vanilla:
```typ
#smallcaps([Hello World])
```

`smallcaps` é um elemento de texto que transforma o body para **small capitals** — letras minúsculas são renderizadas como versões reduzidas de maiúsculas. No vanilla, isto é implementado via **OpenType feature `smcp`** (ou `c2sc` para caps-to-small-caps) no shaping pipeline (rustybuzz/harfbuzz).

No cristalino, **shaping está scope-out** (DEBT-53, XL futuro). Sem shaping, não é possível implementar small caps real (OpenType feature activation). O consumer será **stub** (recursa body sem transformação), com scope-out ADR-0054 graded documentando a dependência de shaping.

**Decisão de honestidade epistémica**: não implementar fallback software (uppercase + scale) porque:
1. Não é paridade vanilla — o vanilla usa OpenType `smcp` quando a fonte suporta.
2. É complexo de implementar corretamente (requer modificação recursiva de `FrameItem::Text` em nested frames).
3. Cria dívida técnica de "quase funciona" que é mais difícil de remover depois.

O stub é preferível: a feature existe no pipeline (parse, eval, Content variant), mas o consumer é transparente até shaping estar disponível.

---

## 2. Decisão de engenharia

### 2.1 — `Content::SmallCaps` variant

```rust
// entities/content.rs — novo variant
SmallCaps {
    body: Box<Content>,
}
```

**Padrão**: variant rico com `body` (padrão N=4 do P284 — underline/strike/overline). Sem atributos opcionais (smallcaps no vanilla não tem attrs próprios além de `body`).

**Derives**: `Clone`, `PartialEq` (via body), `Debug`.
**Plain text**: `body.plain_text()`.
**Map content**: map sobre `body`.

### 2.2 — `native_smallcaps` stdlib

```rust
native_smallcaps(body: Content) -> Content::SmallCaps
```

Sem named args. Paridade vanilla: `#smallcaps(body)`.

### 2.3 — Consumer layout (stub)

```rust
// layout/mod.rs (ou equivalente)
Content::SmallCaps { body } => {
    // ADR-0054 graded: small caps requer shaping OpenType feature smcp.
    // Consumer real scope-out até DEBT-53 (shaping XL) ou fallback software futuro.
    self.layout_content(&body)
}
```

**Decisão**: stub transparente — o body é renderizado normalmente. Não emite placeholder visual (ex.: `[smallcaps]`). O documento compila e é visualmente idêntico ao body sem smallcaps — o que é correto para um scope-out documentado.

### 2.4 — Export (stub)

Nenhuma mudança — o consumer stub não adiciona novos `FrameItem`s. O export vê o body normal.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.1 — Prompt L0 `smallcaps.md`

Novo em `00_nucleo/prompts/engine/stdlib/smallcaps.md` (ou `text.md` se existir e for apropriado; confirmar path):

- **Paridade**: `smallcaps(body)` ≡ vanilla `SmallcapsElem` morfologicamente.
- **Substrato**: Content variant `SmallCaps { body }` + stdlib `native_smallcaps` + consumer stub.
- **Sem tipo novo**: reusa `Content::Text`, `Box<Content>`.
- **Sem atributos**: body obrigatório, único argumento.
- **Consumer**: stub — recursa body. ADR-0054 graded: small caps real requer shaping OpenType `smcp` / `c2sc` (DEBT-53).
- **Erro**: body ausente → Err; argumento extra → Err.
- **Teste**: `smallcaps([Hello])` parse+eval → `Content::SmallCaps` → layout recursa body → output idêntico a `[Hello]` (stub).

### A.2 — CHECKPOINT

Parar. Apresentar `smallcaps.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

### B.1 — Content variant `SmallCaps`

Em `entities/content.rs`:

```rust
SmallCaps {
    body: Box<Content>,
}
```

Atualizar:
- `PartialEq` — match arm `SmallCaps { body: a } => matches!(other, SmallCaps { body: b } if a == b)`.
- `map_content` — map sobre `body`.
- `plain_text` — `body.plain_text()`.
- `repr` — `"smallcaps(..)"` ou struct detalhada.
- `layout` — delega a consumer (stub).

### B.2 — `native_smallcaps` stdlib

Em `rules/stdlib/text.rs` (ou módulo apropriado):

```rust
pub fn native_smallcaps(args: &Args) -> SourceResult<Value> {
    let body = args.expect::<Content>("body")?;
    Ok(Value::Content(Content::SmallCaps {
        body: Box::new(body),
    }))
}
```

### B.3 — Registar em `make_stdlib`

```rust
scope.define("smallcaps", Func::native(native_smallcaps));
```

### B.4 — Consumer layout (stub)

Em `engine/layout/mod.rs` (ou onde `layout_content` match em `Content`):

```rust
Content::SmallCaps { body } => {
    // ADR-0054 graded: small caps requer shaping OpenType feature smcp / c2sc.
    // Consumer real scope-out até DEBT-53 (shaping XL) ou fallback software futuro.
    // Stub: recursa body sem transformação.
    self.layout_content(&body)
}
```

### B.5 — Testes

1. **Unit content** (3-4 tests em `entities/content.rs`):
   - `smallcaps_variant_ctor` — `Content::SmallCaps { body: Box::new(Content::text("Hello")) }`.
   - `smallcaps_partial_eq` — equality por body.
   - `smallcaps_map_content` — map sobre body.
   - `smallcaps_plain_text` — plain_text delega a body.

2. **Unit stdlib** (3-4 tests em `stdlib/mod.rs` ou `text.rs`):
   - `native_smallcaps_basic` — `smallcaps([Hello])` retorna `Content::SmallCaps`.
   - `native_smallcaps_body` — body é o argumento passado.
   - `native_smallcaps_no_args` — `smallcaps()` → Err (body ausente).
   - `native_smallcaps_extra_arg` — `smallcaps([Hello], extra: 1)` → Err (arg desconhecido).

3. **Integration / E2E** (2-3 tests):
   - `smallcaps_pipeline` — parse + eval + `smallcaps([Hello])` → `Content::SmallCaps`.
   - `smallcaps_layout_stub` — layout de `smallcaps([Hello])` emite mesmo output que `[Hello]` (stub).
   - `smallcaps_repr` — `repr(smallcaps([Hello]))` retorna string esperada.

### B.6 — Linhagem

- `@prompt` aponta para `smallcaps.md`.
- `@prompt-hash` via `--fix-hashes`.
- Referência cruzada: P284 (underline/strike/overline — padrão variant rico com body), DEBT-53 (shaping scope-out), ADR-0054 (graded), ADR-0107 (paridade linguagem).

---

## 5. O que NÃO fazer (scope-out)

- **Não** implementar fallback software (uppercase + scale) — não é paridade vanilla; cria dívida técnica.
- **Não** implementar consumer real com OpenType feature `smcp` — requer shaping (DEBT-53, XL).
- **Não** adicionar atributos opcionais (ex.: `delta` para tracking) — não existem no vanilla `SmallcapsElem` básico.
- **Não** implementar `text(smallcaps: true)` — é set rule, não elemento; scope-out ADR-0054 graded.
- **Não** adicionar tipo novo — reusa `Content` variant.
- **Não** quebrar invariantes de camada (L1 puro, zero I/O).

---

## 6. Critérios de aceitação

1. `smallcaps([Hello])` compila e retorna `Content::SmallCaps`.
2. Layout de `smallcaps([Hello])` emite output idêntico a `[Hello]` (stub transparente).
3. Zero tipo novo; zero I/O; zero variant novo em `Value`.
4. Testes verdes (≥8 unit + 2-3 integration); lint zero; hashes propagados.
5. Inventário 148: `smallcaps` transita `ausente` → `implementado⁺` (consumer real ADR-0054 graded).
6. L0 salvo e hashado antes do código (protocolo de nucleação).
7. Ritmo S-M: tempo de ciclo comparável a P284 (variant rico com body, padrão N=4).

---

## 7. O que pode sair errado

- **`Content` enum já tem muitos variants — adicionar 1 aumenta o match exhaustivo.** Mitigação: é esperado; garantir que todos os match sites são atualizados (grep por `Content::` em `entities/content.rs`, `eval/ops.rs`, `layout/mod.rs`, etc.).
- **Consumer stub é confundido com "funciona" em testes E2E.** Mitigação: testes de layout comparam output de `smallcaps([Hello])` com `[Hello]` — se forem idênticos, o stub está correto. Documentar no teste que é stub esperado.
- **Tentação de implementar fallback software.** Mitigação: ADR-0054 graded; honestidade epistémica — não criar "quase funciona".
- **Tentação de já implementar `text.script` ou `text.dir` junto.** Mitigação: um passo de cada vez; `text.script` e `text.dir` dependem de shaping/bidi (XL).

---

## 8. Referências

- Tabela A.3 — `smallcaps` ausente (linha 376).
- P284 — `Content::Underline`/`Strike`/`Overline` (padrão variant rico com body, N=4).
- DEBT-53 — shaping / rustybuzz (scope-out XL).
- ADR-0054 — graded scope-out (consumer real small caps).
- ADR-0107 — paridade linguagem vs mecânica (elemento existe, consumer é mecânica).

---

## 9. Nota sobre o Tekt

Este passo é **variant rico com body** — padrão N=4 consolidado em P284 (underline/strike/overline). A diferença é que `smallcaps` não tem atributos opcionais (só `body`), e o consumer é stub (não real como em P284).

A decisão de stub vs fallback software é **honestidade epistémica aplicada**: não sabemos como implementar small caps sem shaping (OpenType features), e um fallback visualmente incorreto é pior que um stub transparente. O stub preserva a paridade linguagem (o elemento existe no pipeline) sem criar dívida técnica visual.

Registar o tempo de ciclo como **baseline de variant rico com consumer stub** — comparar com P284 (variant rico com consumer real) para calibrar quando usar stub vs quando investir em consumer real.
