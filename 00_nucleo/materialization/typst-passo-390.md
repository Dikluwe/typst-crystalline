# Passo 390 — Materialização: `square(...)` (XS)

**Tipo**: Materialização (L1 — stdlib helper; zero tipo novo; zero I/O).
**Data**: 2026-06-22.
**Padrão**: diagnóstico-primeiro (sonda 389 cumprida); medir-antes-de-decidir (ADR-0108).
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0107 (paridade linguagem), ADR-0017 (não aplica — sem variant novo).
**Sonda fonte**: `typst-sonda-ausentes-ordem-passo-389.md` §2D — `square`, XS, derivável de `Rect`.

> **Nota de numeração.** Um passo só. Não numerar à frente.

---

## 1. Contexto

A sonda 389 confirmou `square` como dívida genuína acidental (balde D), XS, zero dependências. O vanilla expõe `square(width, height: auto)` — um retângulo com lados iguais. A forma cristalina já tem `Rect` (`native_rect`, `shapes.rs:48`); `square` é helper sintático sobre ele.

Este passo é o **piso da fila limpa**: menor superfície, zero tipo, zero graded, zero scope-out. Serve para validar que a fila flui sem travas antes de subir para S/M.

---

## 2. Decisão de engenharia

`square` é **morfologia sobre `Rect`**, não tipo novo. No vanilla:

```
square(1cm)       == rect(width: 1cm, height: 1cm)
square(1cm, 2cm)  == rect(width: 1cm, height: 2cm)  // fallback, raro
```

A paridade (ADR-0107) é morfológica: a forma do conteúdo é a mesma. No cristalino:

- `native_square` constrói `Rect` com `width == height` (ou delega a `native_rect`).
- Não cria `ShapeKind::Square` — over-engineering; `Rect` com lados iguais é a forma.
- Não toca layout/render — a saída é idêntica por construção.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.1 — Prompt L0 `square.md`

Novo em `00_nucleo/prompts/rules/stdlib/square.md` (ou área apropriada, confirmar path):

- **Paridade**: `square(w)` ≡ `rect(width: w, height: w)` morfologicamente.
- **Substrato**: helper stdlib que constrói `Rect` via `native_rect` existente.
- **Sem tipo novo**: reutiliza `Value::Content` → `Rect`.
- **Parâmetros**: `width` (obrigatório, `Relative`), `height` (opcional, default `width`).
- **Erro**: se `height` fornecido e diferente de `width`, comportamento vanilla (aceita; vira `Rect` genérico).
- **Teste**: `square(1cm)` e `rect(1cm, 1cm)` produzem mesmo layout (paridade morfológica, não byte-diff).

### A.2 — CHECKPOINT

Parar. Apresentar `square.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

1. **Implementar `native_square`** em `shapes.rs` (ou módulo stdlib apropriado), delegando a `Rect`:
   ```rust
   // pseudo: native_square(width: Rel, height: Option<Rel>) -> Content
   // constrói Rect com width e height.unwrap_or(width)
   ```
2. **Registar em `make_stdlib`** (ou equivalente).
3. **Testes**:
   - `square(1cm)` → layout idêntico a `rect(1cm, 1cm)` (morfologia, não bytes).
   - `square(1cm, 2cm)` → comporta-se como `rect(1cm, 2cm)` (fallback).
   - Erro: tipo errado de argumento (reusa validação de `Rect`).
4. **Linhagem**: `@prompt` aponta para `square.md`; `@prompt-hash` via `--fix-hashes`.
5. **Validação**:
   - `cargo test --workspace` — verde.
   - `crystalline-lint .` — zero violations novas.
   - `git diff --stat` dos `.rs`: apenas `native_square` + registro + testes.

---

## 5. O que NÃO fazer (scope-out)

- **Não** criar `ShapeKind::Square` — `Rect` com width==height é suficiente.
- **Não** tocar em layout/render — a forma é a mesma.
- **Não** adicionar tipo `Value` ou `Content` — reutiliza existente.
- **Não** abrir reservas.
- **Não** tocar em outro ausente — um passo de cada vez.

---

## 6. Critérios de aceitação

1. `square(1cm)` compila e produz saída morfologicamente idêntica a `rect(1cm, 1cm)`.
2. Zero tipo novo; zero variant novo; zero I/O.
3. Testes verdes; lint zero; hashes propagados.
4. Inventário 148: `square` transita `ausente` → `implementado`.
5. L0 salvo e hashado antes do código (protocolo de nucleação).

---

## 7. O que pode sair errado

- **`Rect` não expõe construtor puro.** Mitigação: se `native_rect` é o único entrypoint, `native_square` chama-o com `height = width`. Se `Rect` exige campo privado, o helper vive no mesmo módulo.
- **Paridade medida por bytes.** Mitigação: ADR-0107 — a forma é a mesma, não o diff de saída renderizada.
- **Tentação de já fazer `lorem`/`panic` junto.** Mitigação: um passo de cada vez; ritmo não é desculpa para violar escopo.

---

## 8. Referências

- `typst-sonda-ausentes-ordem-passo-389.md` §2D — confirmação de `square` como XS, derivável de `Rect`.
- `shapes.rs:48` (`native_rect`) — substrato existente.
- ADR-0107 — paridade morfológica.
- ADR-0033 — paridade vanilla.

---

## 9. Nota sobre o Tekt

Este passo é o **piso de momentum**: XS, zero deps, zero graded, zero scope-out. Se a fila limpa não flui aqui, o problema é infraestrutura (tooling, linter, prompt), não a fila. Registar o tempo de ciclo (L0 → hash → código → teste → lint) como baseline de saúde do pipeline.
