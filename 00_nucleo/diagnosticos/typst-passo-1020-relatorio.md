# Passo 1020 — Relatório final

**Data**: 2026-08-13  
**Commit de base**: `6fc51d954` (P1018-P1020: CounterKey enum, heading counter gate e footnote superscript)  
**Ficheiros alterados**:

- `01_core/src/compiler/layout/footnote.rs` — layout do marcador inline passa a usar `format_pattern` e `Content::superscript`.
- `00_nucleo/prompts/compiler/layout/footnote.md` — L0 novo.

---

## Resumo

O marcador inline de `footnote` era desenhado como texto simples entre colchetes literais (`[1]`). O vanilla desenha o número formatado em superscript (ex.: `¹`, `*`). Aplicámos as duas mudanças em simultâneo.

### Estado antes

```rust
// Marcador literal entre colchetes
let marker = Content::text(format!("[{}]", n));
```

### Estado depois

```rust
// 01_core/src/compiler/layout/footnote.rs
let marker_text = match e.numbering {
    Some(ref pattern) => format_pattern(pattern, n),
    None => format_pattern("1", n),
};
let marker = Content::superscript(Content::text(marker_text));
```

- O número vem do introspector (`layouter.introspector.flat_counter_at(...)`), já alterado no P1016.
- Aplica-se o `numbering` do `FootnoteElem` via `format_pattern` (reutilizando o helper de P793/P844).
- Sem `#set footnote(numbering:)`, usa-se o default vanilla `"1"`.
- O marcador é envolvido em `Content::superscript(...)` para elevação visual; os colchetes literais foram removidos.

### Decisão do gate

Aprovada pelo dono com a condição de confirmar que `Content::superscript` já era primitivo reaproveitável. Confirmámos: existe em `01_core/src/entities/content.rs` como construtor público e é usado noutros pontos do layout (ex.: superior/esquerdo em expressões math). Não foi necessário construir nada de novo.

---

## Validação

```
cargo test --workspace
```

Resultado:

- typst-core: 4971 passed
- typst-infra: 787 passed
- typst-shell: 41 passed
- benches: 2 passed
- wiring: 37 passed
- crystalline_lint: 2 passed

Total: **5840 tests passed**, 0 failed.

```
crystalline-lint .
```

Zero erros. Três warnings V7 pré-existentes (`auditar-fatiamento.md`, `auditar-spec.md`, `package_version_resolution.md`), nada relacionado com esta mudança.

```
crystalline-lint --fix-hashes .
```

Nenhum drift restante.

---

## Notas para passos futuros

- A numeração em si (corpo da nota no rodapé) não mudou; só a forma do marcador inline mudou.
- Futuro: suporte a `footnote.entry` ou links do marcador para a nota no rodapé.
