# ⚖️ ADR-0045: Formato de diagnósticos: resolução em L1, formatação em L3

**Status**: `EM VIGOR`
**Revoga**: nenhuma.
**Validado**: Passo 111.E.
**Data**: 2026-04-23
**Autor**: Passo 111
**Complementa**: ADR-0033 (paridade funcional), ADR-0043 (canal
Sink → L3 com L1 data-only).

---

## Contexto

O Passo 106 abriu o canal Sink → L3 (ADR-0043, DEBT-51) com formato
mínimo:

```rust
eprintln!("warning: {:?} {}", diag.span, diag.message);
```

Output literal:

```
warning: Span(8796093022226) ficheiro vazio: sem conteúdo
```

**Ilegível**. `Span(N)` é o número interno do AST node, não tem
sentido para o utilizador. Sem ficheiro, sem linha, sem coluna,
sem hints.

ADR-0043 aceitou este formato explicitamente como "formato opaco
aceito" — limitação documentada como trabalho futuro. DEBT-49
(encerrado Passo 107) emite warnings estruturados com hints; o
formato mínimo descarta os hints.

Passo 111 paga essa lacuna.

---

## Decisão

### L1 ganha `Source::span_to_line_col`

```rust
impl Source {
    /// Resolve `span` para `(linha, coluna)` 1-indexadas.
    ///
    /// Devolve `None` se:
    /// - o span é detached;
    /// - o span refere-se a outro ficheiro;
    /// - o span é numbered e o nó não é encontrado;
    /// - o offset está fora dos limites (defensivo).
    ///
    /// Coluna em **Unicode code points** (chars), não bytes —
    /// convenção editor (gcc, clang, rustc).
    pub fn span_to_line_col(&self, span: Span) -> Option<(u32, u32)>;
}
```

Suporta ambos os tipos de Span:
- **Raw-range**: `span.range()` dá `Some(start..end)`; usa `start`.
- **Numbered**: `LinkedNode::new(root).find(span)` → `offset()`.

### L3 ganha `format_diagnostic`

```rust
pub fn format_diagnostic(
    diag: &SourceDiagnostic,
    source: &Source,
    source_path: &str,
) -> String;
```

Formato gcc/clang-compatível:

```
input.typ:42:10: warning: set text: propriedade 'font' ainda não suportada
  hint: ver ADR-0040 para propriedades cobertas por set text
```

Detached / cross-file:

```
input.typ:<detached>: warning: ficheiro vazio: sem conteúdo
```

Uniforme para warnings (Sink) e errors (`SourceResult::Err`) —
mesmo formatter, distinção só em `severity`.

### L3 — `drain_diagnostics_to_stderr` substitui `drain_warnings_to_stderr`

Renomeado porque cobre errors também. Aceita `&Source` e
`&str source_path` para formatar cada diagnóstico.

---

## Alternativas rejeitadas

### R-1 — Formatter em L1

**Rejeitada**. ADR-0043 estabeleceu explicitamente "L1 fornece dados;
L3 formata". Mover o formatter para L1 introduziria concerns de UI
(severity labels, indentação de hints) em L1. Separação preservada:
`span_to_line_col` é **resolução** (devolve `(u32, u32)`), não
formatação.

### R-2 — Só byte-offset em L1; linha/coluna em L3

**Considerada e rejeitada**. Se outros formatters aparecerem
(JSON, SARIF, LSP), cada um duplicaria a conversão offset → linha/col.
Centralizar em L1 evita duplicação. Custo: 1 método novo em
`Source`.

### R-3 — Coluna em bytes

**Rejeitada**. Editores contam colunas em Unicode code points
(`á` = 1 coluna, não 2). Gcc, clang, rustc fazem assim. Coluna em
bytes divergiria da convenção e confundiria utilizadores em
código com acentos.

### R-4 — Spans detached com linha/coluna fictícios (1, 1)

**Rejeitada**. Inventar posição é falsa informação. `<detached>`
explícito é honesto.

### R-5 — ADR-0033 paridade literal com formato vanilla

O vanilla tem seu próprio formatter de diagnósticos (via crate
`codespan-reporting` em `typst-cli`). Replicar formato vanilla
exigiria migrar dependência completa. **ADR-0033 é paridade
funcional**, não paridade visual. Convenção gcc/clang é
**estabelecida externamente** e satisfaz a paridade funcional
(utilizador vê onde e o quê).

---

## Limitações aceites

1. **Trace não incluído**: `SourceDiagnostic::trace` (stack de
   `Tracepoint`) ignorado neste passo. Hoje raramente populado
   no cristalino. Futuro se/quando a pilha de chamadas ganhar
   valor prático.
2. **Sem cores ANSI**: adicionar cores requer detecção `isatty`,
   flags `--color=never`, env `NO_COLOR`. Passo dedicado quando
   a CLI real (04_wiring) aparecer.
3. **Sem JSON/SARIF**: adiado. `format_diagnostic` é um dos N
   formatters possíveis; outros formatters convivem quando forem
   materializados.
4. **Cross-file spans**: `span.id() != source.id()` → tratado
   como detached. Resolução cross-file exige mapa `FileId → Source`
   do caller. Fora do escopo. O formatter imprime `<detached>`
   — inexacto para cross-file mas honesto (não há source para
   resolver).
5. **Múltiplos sources**: o formatter assume 1 Source principal.
   Caller com múltiplos imports teria de escolher qual passar.
   Limitação natural do modelo single-source actual.
6. **Performance O(N × text.len())**: `span_to_line_col` é linear
   no offset; N warnings → O(N × text). Para inputs típicos (< 100
   diagnósticos, < 100KB texto), desprezível. Inputs gigantes
   optimizariam com cache de line-starts — passo dedicado se
   surgir a necessidade.

---

## Consequências

### Positivas

1. Warnings e errors têm **output legível** — utilizador vê
   ficheiro, linha, coluna, mensagem e hints.
2. Formato é **convenção externa** (gcc/clang/rustc) — conhecido
   por editores que parseiam output do compilador.
3. L1 mantém-se puro — `span_to_line_col` é função de dados
   (string → (u32, u32)).
4. Helper `format_diagnostic` é reutilizável — CLI futura, LSP,
   testes, tudo chama o mesmo.

### Negativas

1. Testes que dependiam do formato antigo (`sink_canal_formato_minimo`)
   precisam ser actualizados.
2. Custo O(text.len()) por diagnóstico (aceitável; ver limitação 6).
3. Mais 1 parâmetro no formatter (`source_path`) — caller tem de o
   fornecer. Teste usa `"input.typ"`; CLI real passaria path real.

### Neutras

1. ADR-0043 intacta — canal Sink não muda. Só a apresentação do
   que passa pelo canal.
2. `SourceDiagnostic` não muda — `span`, `message`, `hints`,
   `trace` permanecem como estão.

---

## Aplicação

Implementado no Passo 111.C — ver
`00_nucleo/materialization/typst-passo-111-relatorio.md`.

ADR promovida a **EM VIGOR** em 111.E.
