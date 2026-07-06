# Relatório Diagnóstico — Passo 584
## Criação do teste de escape/shorthand/linebreak em markup

- **Commit de Referência:** `760822612` (Working Tree não commitado)
- **Data/Hora da Medição:** 2026-07-06 15:27:47 UTC
- **Linter Status:** ✓ Clean (0 violations)
- **Status dos Testes:** Sucesso completo (`cargo test --workspace`: 3570 + 597 + 24 + 2 + 21 + 2 passados; 0 falhas)

---

## 1. Contexto e Objetivos

O Passo 581 corrigiu `Expr::Escape`, `Expr::Shorthand` e `Expr::Linebreak` no avaliador de markup (`01_core/src/rules/eval/mod.rs`) para que deixassem de cair no braço genérico `_ => Ok(Value::None)`. O relatório de P581 afirmou que a correção fora validada por um teste unitário chamado `p581_cobertura_de_escape_e_shorthand_em_layout`. O Passo 583 confirmou que esse teste não existia no código — a correção estava protegida apenas por extração de texto manual, sem regressão automática.

Este passo cria o teste que devia já existir e adiciona um documento ao corpus de paridade para cobrir esta lacuna.

---

## 2. Implementação

### 2.1. Teste unitário

Ficheiro: `01_core/src/rules/eval/tests.rs`  
Nome do teste: `p584_escape_shorthand_linebreak_em_markup_preservados`  
Localização: secção de markup, após os testes de smart quotes (Passo 445).

O teste usa o helper existente `eval_plain_text` e verifica três construções:

1. **Escape:** `\# \$ \& \* \\` produz os caracteres `# $ & * \`.
2. **Shorthand:** `--` produz en-dash (U+2013), `---` produz em-dash (U+2014), `...` produz ellipsis (U+2026).
3. **Linebreak:** `\ ` (barra seguida de whitespace) introduz `Content::Linebreak`, que emite `\n` em `plain_text()`.

Nota sobre a sintaxe de linebreak: o lexer de markup (`01_core/src/rules/lexer/markup.rs:77-78`) só emite `SyntaxKind::Linebreak` quando a barra invertida é seguida de whitespace ou fim de input. `\\` em markup é lexado como um `Escape` que representa um único `\`, pelo que o teste de linebreak usa `\ ` conforme a semântica real do compilador cristalino.

### 2.2. Documento de corpus

Ficheiro: `lab/parity/corpus/markup/escape-shorthand-linebreak.typ`

Conteúdo:

```typst
Escape: \# \$ \& \* \\
Shorthand: a -- b --- c ... d
Linebreak: primeira \ segunda
```

Este documento foi compilado com o binário de release:

```bash
./target/release/typst lab/parity/corpus/markup/escape-shorthand-linebreak.typ /tmp/p584-corpus.pdf
pdftotext /tmp/p584-corpus.pdf -
```

Resultado da extração:

```text
Escape: # $ & * \ Shorthand: a – b — c … d Linebreak: primeira
segunda
```

Nenhum carácter desapareceu; os shorthands foram resolvidos para os caracteres Unicode corretos; o linebreak forçou a quebra de linha.

---

## 3. Validação

### 3.1. Teste unitário

```bash
cargo test -p typst-core p584_escape_shorthand_linebreak_em_markup_preservados
```

Resultado: `ok`.

### 3.2. Suite completa

```bash
cargo test --workspace
```

Resultado: 3570 + 597 + 24 + 2 + 21 + 2 testes passaram; 0 falhas.

### 3.3. Linter

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

---

## 4. Conclusão de Fecho do Passo 584

- [x] Teste unitário `p584_escape_shorthand_linebreak_em_markup_preservados` criado em `01_core/src/rules/eval/tests.rs`, confirmado por leitura directa do ficheiro e a passar.
- [x] Documento `lab/parity/corpus/markup/escape-shorthand-linebreak.typ` adicionado ao corpus de paridade.
- [x] Compilação do documento de corpus confirmada com `pdftotext`.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
