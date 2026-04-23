# Passo 111 — Relatório (formato rico de diagnósticos)

**Data**: 2026-04-23
**Precondição**: Passo 110 encerrado; 803 L1 + 184 L3 + 6 ignorados;
zero violations.
**ADR criada**: ADR-0045 "Formato de diagnósticos — resolução em
L1, formatação em L3" — **PROMOVIDA A EM VIGOR** em 111.E.

---

## Sumário

Formato de diagnósticos mudou de
`"warning: {:?} {}"` (opaco; Span interno) para
`"path:linha:coluna: severity: mensagem"` + hints indentados
(convenção gcc/clang, conhecida por editores).

L1 ganhou `Source::span_to_line_col(span) -> Option<(u32, u32)>` —
resolução de spans numbered (via `LinkedNode::find`) e raw-range
(via `Span::range`). L3 ganhou `format_diagnostic(diag, source,
source_path)` e `drain_diagnostics_to_stderr` (renomeado de
`drain_warnings_to_stderr` — agora cobre errors também).

Zero regressão funcional. **803 → 811 L1** (+8 testes
`span_to_line_col`). **184 → 189 L3** (+5 testes
`format_diagnostic_*`). Zero violations.

---

## 111.A — Inventário

Inventário em
`00_nucleo/diagnosticos/inventario-formato-diagnosticos-passo-111.md`.

Descoberta chave: `LinkedNode::find(span) -> Option<LinkedNode>`
em `syntax_node.rs:687` + `LinkedNode::offset()` → resolve spans
numbered para byte offset. Spans raw-range já têm `range()`
público. **Gate 111.A.4 não disparou** — plano directo.

---

## 111.B — ADR-0045

Criada em `00_nucleo/adr/typst-adr-0045-formato-diagnosticos.md`.
**EM VIGOR em 111.E**.

Pontos-chave:

- **L1 resolve, L3 formata** — consistente com ADR-0043.
- **Convenção gcc/clang**: `path:linha:coluna: severity: mensagem`.
- **Coluna em Unicode code points** (chars), não bytes.
- **Uniforme warnings + errors** via `severity` do `SourceDiagnostic`.
- **Detached/cross-file** → fallback `path:<detached>:`.
- **Trace, cores, JSON, SARIF** — fora do escopo (limitações
  documentadas).

---

## 111.C — Implementação

### API L1

```rust
impl Source {
    pub fn span_to_line_col(&self, span: Span) -> Option<(u32, u32)> {
        if span.is_detached() { return None; }
        if span.id() != Some(self.0.id) { return None; }
        let offset = if let Some(range) = span.range() {
            range.start  // raw-range span
        } else {
            LinkedNode::new(&self.0.root).find(span)?.offset()  // numbered
        };
        if offset > self.0.text.len() { return None; }
        // Contar linhas/colunas em chars até offset.
        let mut line = 1u32;
        let mut col = 1u32;
        for (i, ch) in self.0.text.char_indices() {
            if i >= offset { break; }
            if ch == '\n' { line += 1; col = 1; } else { col += 1; }
        }
        Some((line, col))
    }
}
```

### API L3

```rust
fn format_diagnostic(
    diag: &SourceDiagnostic,
    source: &Source,
    source_path: &str,
) -> String {
    let severity = match diag.severity {
        Severity::Error   => "error",
        Severity::Warning => "warning",
    };
    let location = match source.span_to_line_col(diag.span) {
        Some((l, c)) => format!("{source_path}:{l}:{c}"),
        None         => format!("{source_path}:<detached>"),
    };
    let mut out = format!("{location}: {severity}: {}\n", diag.message);
    for hint in &diag.hints {
        out.push_str(&format!("  hint: {hint}\n"));
    }
    out
}

fn drain_diagnostics_to_stderr(
    diagnostics: &[SourceDiagnostic],
    source: &Source,
    source_path: &str,
) {
    for diag in diagnostics {
        eprint!("{}", format_diagnostic(diag, source, source_path));
    }
}
```

### Substituição

`drain_warnings_to_stderr` (Passo 106) **removido**. Substituído
por `drain_diagnostics_to_stderr` (renomeado + assinatura
estendida). Zero matches de `drain_warnings_to_stderr` ou
`"warning: {:?}"` em `03_infra/src/`.

---

## 111.D — Testes

### L1 (`01_core/src/entities/source.rs`): 8 novos

1. `span_to_line_col_inicio_do_texto` — offset 0 → (1, 1).
2. `span_to_line_col_depois_de_newline` — "abc\nde" offset 4 → (2, 1).
3. `span_to_line_col_multibyte_unicode_coluna_em_chars` —
   "áéí" offset 2 (fim do á) → (1, 2). Valida coluna em chars.
4. `span_to_line_col_detached_devolve_none` — `Span::detached()` → `None`.
5. `span_to_line_col_ficheiro_diferente_devolve_none` — FileId
   mismatch → `None`.
6. `span_to_line_col_fim_do_texto` — "abc" offset 3 → (1, 4).
7. `span_to_line_col_span_numbered_via_linked_find` — span numbered
   real da CST resolve-se via `LinkedNode::find`.
8. `span_to_line_col_offset_fora_de_limites_retorna_none` — raw-range
   com start além do texto → `None`.

### L3 (`03_infra/src/integration_tests.rs`): 5 novos + 1 actualizado

Novos:
1. `format_diagnostic_warning_com_ficheiro_linha_coluna` — fluxo
   DEBT-49 completo: `"input.typ:1:N: warning: ..."` + hint
   indentado com ADR-0040.
2. `format_diagnostic_com_multiplos_hints` — 2 hints → 2 linhas
   `"  hint: ..."` em ordem.
3. `format_diagnostic_error_uniforme` — severity `Error` produz
   `"error:"`, resto do formato idêntico.
4. `format_diagnostic_span_detached_usa_fallback` — pilot do
   ficheiro vazio (span detached) → `"input.typ:<detached>: warning:"`.
5. `format_diagnostic_pipeline_debt49` — estrutura multi-linha:
   principal + 1 hint, ambos no formato esperado.

Actualizado:
- `sink_canal_formato_minimo` — reescrito para usar
  `format_diagnostic` com span detached. Output:
  `"input.typ:<detached>: warning: exemplo de warning\n"`.

### Exemplo antes/depois

Input: `#set text(font: "Arial")`

**Antes (Passo 106)**:
```
warning: Span(281474976710670) text: propriedade 'font' ainda não suportada
```

**Depois (Passo 111)**:
```
input.typ:1:11: warning: text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
```

---

## 111.E — Encerramento

### Verificação

```
$ cargo test --workspace | grep "test result"
test result: ok. 811 passed; 0 failed; 0 ignored ...   (L1 +8)
test result: ok. 189 passed; 0 failed; 6 ignored ...   (L3 +5)

$ grep -rn "drain_warnings_to_stderr" 03_infra/src/
(zero matches — removido)

$ grep -rn 'warning: {:?}' 03_infra/src/
(zero matches — formato antigo eliminado)

$ crystalline-lint .
✓ No violations found
```

### ADR promovida

**ADR-0045** `EM VIGOR`.

---

## Números finais

| Métrica | Antes | Depois |
|---------|------:|-------:|
| L1 tests | 803 | **811** (+8) |
| L3 tests | 184 | **189** (+5) |
| Ignorados | 6 | 6 |
| Violations | 0 | 0 |
| ADRs activas | 44 | **45** (+0045) |
| DEBTs abertos | 11 | 11 (inalterado) |

---

## Lições

1. **Spec antecipou `LinkedNode::find`**: o inventário 111.A
   encontrou o mecanismo exacto. `Span` é opaco mas os dois
   caminhos (raw-range via `.range()`, numbered via
   `LinkedNode::find().offset()`) já existiam. Mudança mínima
   em L1 — um método novo na Source.

2. **Coluna em chars paga-se barato**: `text.char_indices()` já
   itera code points. A diferença entre "coluna em bytes" e
   "coluna em chars" é literalmente `if i >= offset` vs
   `col_bytes += ch.len_utf8()`. Alinhar com editores é gratuito.

3. **Uniformidade warning/error paga-se pelo enum `Severity`**:
   o `SourceDiagnostic` tem `severity` como campo; o formatter
   só mapeia a string. Um único formatter cobre ambos — não
   houve tentação de bifurcar. O nome `drain_diagnostics_*`
   reflecte a uniformidade.

4. **Fallback `<detached>` é mais honesto**: inventar
   `(1, 1)` para spans detached seria silencioso. `<detached>` é
   visivelmente diferente de `1:1` — o utilizador pode
   distinguir "erro numa posição conhecida" de "erro sem
   posição" só olhando para o output.

5. **Cross-file continua por resolver mas claro**: o formatter
   aplica `<detached>` também para spans de outro ficheiro.
   Inexacto, mas:
   - Não oculta (é visível como `<detached>`).
   - Não inventa (não põe linha/coluna fictícios).
   - Não duplica infra (não tenta resolver sem mapa).
   Passo dedicado quando o caller tiver mapa FileId → Source.

6. **O5 teste mais útil foi o numbered span real**: validou que
   `LinkedNode::find` retorna nó e `offset()` dá posição correcta
   para spans produzidos pelo parser. Sem este teste, poderia
   passar só para raw-range e falhar em produção.

---

## Estado pós-Passo 111

### Pipeline de diagnósticos end-to-end

```
Input Typst
     ↓ parse → Source (SyntaxNode com spans numbered)
     ↓ eval → SourceDiagnostic { span, message, hints, severity }
     ↓ Sink acumula (warnings) / SourceResult::Err (errors)
     ↓ caller L3 drena:
     ↓     for diag in diagnostics:
     ↓         source.span_to_line_col(diag.span)  [L1 resolve]
     ↓         format_diagnostic(diag, source, path) [L3 formata]
     ↓         eprint! → stderr
     ↓
Output gcc/clang-compatible:
     input.typ:L:C: severity: message
       hint: ...
```

### Trabalho futuro identificado (não bloqueado)

1. **Cores ANSI** — passo dedicado quando a CLI real (`04_wiring`)
   aparecer. Requer decisão sobre `isatty`, `--color=never`,
   `NO_COLOR`.
2. **JSON/SARIF formatters** — outros formatters convivem com
   `format_diagnostic` (formato único de texto). Passo dedicado.
3. **Cross-file resolution** — caller com mapa `FileId → Source`.
   Passo dedicado.
4. **Trace integration** — incluir `diag.trace` no output.
   Hoje raramente populado no cristalino.
5. **Performance cache line-starts** — só se surgir input com
   milhares de diagnósticos.
6. **CLI real em `04_wiring`** — passa a usar
   `drain_diagnostics_to_stderr` directamente.
