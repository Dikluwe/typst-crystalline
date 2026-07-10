# Relatório de Paridade — P665

**Passo:** 665  
**Data:** 2026-07-10  
**Foco:** Reverter `text.bold`/`text.italic` como argumentos nomeados de `#set text(...)`, alinhando com o vanilla; adicionar `text.style` como alternativa canónica.  
**Dependências:** P664 (onde a divergência foi confirmada), P662 (padrão de reversão).  
**Hash do commit com as alterações:** `30ec87843`

---

## 1. Sonda

P664 confirmou que o vanilla 0.15.0 rejeita:

```text
error: unexpected argument: bold
error: unexpected argument: italic
```

para `#set text(bold: true)` e `#set text(italic: true)`. O vanilla usa `#set text(weight: "bold")` e `#set text(style: "italic")`.

Mapeamento dos sítios internos:

- `01_core/src/rules/eval/rules.rs`: arms `bold`/`italic` no match de `#set text(...)`.
- `01_core/src/rules/eval/markup.rs`: `eval_strong`/`eval_emph` usam `StyleDelta { bold: Some(true) }` / `StyleDelta { italic: Some(true) }`.
- `01_core/src/rules/layout/text.rs`: decodifica `"text.bold"` / `"text.italic"` da chain.
- `TextStyle`/`StyleChain` mantêm campos `bold`/`italic` internos, usados pelo shaper/export.

Conclusão da sonda: é seguro remover `bold`/`italic` do parser de `#set text(...)` sem afectar `*...*` / `_..._`, desde que se mantenham os campos internos.

---

## 2. Implementação

### 2.1 Parser de `#set text(...)`

`01_core/src/rules/eval/rules.rs`:

- Removidos os arms `bold` e `italic` que propagavam `Style::bold(b)` / `Style::italic(b)`.
- Adicionado arm `"bold" | "italic"` que devolve erro hard: `unexpected argument: {key}`.
- Adicionado arm `style` que aceita `"normal"`, `"italic"` ou `"oblique"` e propaga para a chain como `"text.style"`.

### 2.2 Layout de texto

`01_core/src/rules/layout/text.rs`:

- Lê `"text.style"` da chain.
- `"italic"` / `"oblique"` contribuem para `effective.italic = true`, coexistente com o campo tipado `italic` vindo de markup.

### 2.3 Markup `*...*` / `_..._`

Manteve-se inalterado: continua a usar `StyleDelta { bold: Some(true) }` / `StyleDelta { italic: Some(true) }`.

### 2.4 Testes

Atualizados testes que usavam `#set text(bold: ...)` / `#set text(italic: ...)` para `weight` / `style`:

- `01_core/src/rules/eval/tests.rs`: 12 testes ajustados; helpers `styles_has_text_bold` e `texto_bold_contendo` passaram a ler `"text.weight"` em vez do campo tipado `bold`.
- `01_core/src/rules/layout/tests.rs`: 4 testes ajustados para verificar `weight`/`style`.
- `03_infra/src/integration_tests.rs`: substituições de `bold: true` por `weight: 700`.

### 2.5 Documentação

- `00_nucleo/prompts/rules/eval.md`: adicionada secção §P665.
- `00_nucleo/diagnosticos/paridade-producao-p525.md`: nota póstuma explicando a reversão.

### 2.6 Infra do linter

- `crystalline.toml`: adicionado `temp_p664 = "temp_p664"` à secção `[excluded]`.
- `.gitignore`: adicionado `/temp_p664/`.

---

## 3. Validação

### 3.1 Testes automáticos

```bash
cargo test --workspace --lib
```

Resultado: `3650 passed; 0 failed` (typst-core), `606 passed; 0 failed` (typst-infra), `28 passed` (typst-shell).

### 3.2 Build

```bash
cargo build --workspace
```

Resultado: sucesso (warnings preexistentes).

### 3.3 Linter

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

### 3.4 Verificação manual

```bash
./target/debug/typst /tmp/p665-rejeitado.typ /tmp/p665-rejeitado.pdf
```

com `/tmp/p665-rejeitado.typ`:

```typst
#set text(bold: true)
Texto.
```

Resultado:

```text
error: unexpected argument: bold
exit: 1
```

```bash
./target/debug/typst /tmp/p665-style.typ /tmp/p665-style.pdf
```

com:

```typst
#set text(style: "italic")
Italic via style.
```

Resultado: `exit: 0`.

```bash
./target/debug/typst /tmp/p665-italic.typ /tmp/p665-italic.pdf
```

com:

```typst
Texto normal, *negrito*, _itálico_, *_ambos_*.
```

Resultado: `exit: 0`; o output de debug mostra variações de eixo `wght` e `ital` correctas.

---

## 4. Decisão

- `text.bold`/`text.italic` como argumentos nomeados de `#set text(...)` foram removidos; agora produzem erro, igual ao vanilla.
- `text.style` foi adicionado para permitir itálico/oblique via sintaxe canónica.
- `text.weight` (já existente) continua a ser a forma canónica para negrito.
- Markup `*...*` / `_..._` preservado sem regressão.
- Nota histórica adicionada a P525.

---

## 5. Estado de fecho

- [x] Sonda completa, uso interno mapeado.
- [x] `bold`/`italic` como argumentos nomeados removidos; erro igual ao vanilla.
- [x] Markup `*...*`/`_..._` sem regressão visual.
- [x] `weight`/`style` continuam a funcionar.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p665.md`.
- [x] Nota adicionada ao histórico de P525.
