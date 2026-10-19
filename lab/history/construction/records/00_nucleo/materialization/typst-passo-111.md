# Passo 111 — Formato rico de diagnósticos (Span → ficheiro:linha:coluna)

**Série**: 111 (passo médio; L1 + L3).
**Precondição**: Passo 110 encerrado (DEBT-45 fechado); CLAUDE.md
actualizado com convenções comemo (edição directa, sem passo
formal); 803 L1 + 184 L3 + 6 ignorados; zero violations.
**ADRs aplicáveis**: ADR-0033 (paridade funcional), ADR-0043
(canal Sink → L3 com L1 data-only).
**ADR nova**: ADR-00NN "Formato de diagnósticos — resolução de
Span em L1, formatação em L3" — `PROPOSTO` em 111.B, `EM VIGOR`
em 111.E.

---

## Objectivo

Substituir o formato mínimo `"warning: {:?} {}"` (Passo 106) por
formato rico gcc/clang-compatível:

```
<ficheiro>:<linha>:<coluna>: warning: <message>
  hint: <hint 1>
  hint: <hint 2>
```

Aplica-se uniformemente a warnings e errors. `<ficheiro>` é o path
do `FileId` do span; `<linha>:<coluna>` resolvidos via método novo
em `Source` (L1). L3 consome a resolução; lógica de parsing fica
em L1.

Este passo **não**:
- Adiciona cores ANSI, JSON, SARIF. Formato é texto simples.
- Resolve spans detached (`Span::detached()`) — esses aparecem
  sem linha/coluna.
- Toca na arquitectura do canal Sink (ADR-0043 intacta).
- Altera a API pública do `eval()`.

---

## Decisões já tomadas

1. **Resolução em L1**: método novo em `Source`:
   `span_to_line_col(span: Span) -> Option<(u32, u32)>`.
   Devolve `None` se o span é detached ou do FileId errado.
2. **Campos incluídos**: severity, ficheiro, linha, coluna, message,
   hints. Trace fica de fora (passo futuro se for preciso).
3. **Formato ficheiro**: path completo ou display-name do FileId.
   A forma exacta depende do `FileId::path()` disponível — confirmar
   em 111.A.
4. **Âmbito uniforme**: mesmo formatter para `Vec<SourceDiagnostic>`
   de warnings (Sink) e errors (`SourceResult::Err`).

---

## Escopo

**Dentro**:
- `01_core/src/entities/source.rs` — novo método
  `span_to_line_col` + testes.
- `03_infra/src/integration_tests.rs` (ou helper equivalente) —
  novo formatter, substitui `drain_warnings_to_stderr`.
- Integração no caller `do_eval_with_sink`: warnings e errors
  passam pelo formatter.
- Testes unitários de `span_to_line_col`.
- Testes de integração do formato.

**Fora**:
- Cores, JSON, SARIF.
- Resolução de spans cross-file (imports). Se span aponta para
  FileId diferente do Source carregado, o formatter aceita e
  imprime sem linha/coluna, documentando a limitação.
- Novo argumento em `eval()`. Formatter vive em L3; recebe
  `&Source` que o caller já tem.
- Múltiplos sources (caller com N ficheiros). Por enquanto
  formatter assume 1 Source principal; spans fora dele imprimem
  sem resolução.

---

## Sub-passos

### 111.A — Inventário

**Parte 1 — Estado do `Source` e `Span`**:

1. `view` em `01_core/src/entities/source.rs`. Registar:
   - Campos (presumível: `id: FileId`, `text: EcoString`, etc.).
   - Métodos existentes (`text()`, `id()`, talvez `lines()`).
   - Se já tem qualquer forma de resolução de Span → offset.
2. `view` em `01_core/src/entities/span.rs` (ou equivalente).
   Registar:
   - Campos internos. Provavelmente `FileId` + `u32` offset
     (byte-offset no source).
   - Métodos públicos (`detached()`, `file_id()`, `offset()`?).
   - Se `Span::detached()` tem representação particular (ex:
     FileId zero).

**Parte 2 — Estado do `FileId`**:

1. `view` em `01_core/src/entities/file_id.rs`.
2. Registar se tem `path()`, `vpath()`, ou equivalente que
   devolva string legível. Se só tem `NonZeroU16` interno sem
   path público, o formatter precisa de mapear FileId → path
   via outro caminho (provavelmente o caller conhece).

**Parte 3 — Consumo actual do formato mínimo**:

1. Grep por `drain_warnings_to_stderr`, `"warning:"`, `"{:?}"` em
   `03_infra/src/`. Listar call sites.
2. Para cada, confirmar que o caller já tem acesso a `Source`
   (sim, tem — passou ao `eval()`).

**Parte 4 — Convenções de formato gcc/clang**:

Referência literal para o formato-alvo (não precisa grep — é
convenção externa estabelecida):

```
src/main.rs:42:10: error: cannot find value `x` in this scope
src/main.rs:50:5: warning: unused variable: `y`
  hint: consider using `_y` instead
```

- Uma linha por diagnóstico principal.
- Hints indentados 2 espaços.
- Coluna 1-indexada (convenção editor).
- Linha 1-indexada.

**Escrever** em
`00_nucleo/diagnosticos/inventario-formato-diagnosticos-passo-111.md`:

```
Source:
  campos: [lista]
  métodos relevantes: text(), id(), ...
  já tem span-to-offset? sim/não

Span:
  campos: [...]
  detached: representado como ...
  offset acessível via: <método ou campo>

FileId:
  path/vpath disponível? [resposta]

Formato:
  ficheiro:linha:coluna: severity: message
  indentação de hints: 2 espaços
  
Casos especiais:
  - Span detached → sem linha/coluna
  - FileId diferente do Source → sem linha/coluna
```

**Gate 111.A**: se `Span` não expõe byte-offset publicamente (ex:
é um índice num mapa interno do Source), `span_to_line_col`
pode precisar de usar outra via (ex: método existente do Source
que já faz resolução). Nesse caso, `span_to_line_col` passa a ser
wrapper fino. Documentar e prosseguir.

### 111.B — ADR nova

Criar `00_nucleo/adr/typst-adr-00NN-formato-diagnosticos.md`
com `PROPOSTO`.

Conteúdo:

- **Contexto**: Passo 106 abriu canal Sink → L3 com formato
  mínimo `{:?}` em Span. Ilegível para utilizador. DEBT-51
  encerrou o canal, mas deixou o formato como lacuna.
- **Decisão**:
  - L1 ganha `Source::span_to_line_col(span: Span) -> Option<(u32, u32)>`.
  - L3 ganha helper `format_diagnostic(diag: &SourceDiagnostic, source: &Source, path: &str) -> String`.
  - Formato `ficheiro:linha:coluna: severity: message` (gcc/clang).
  - Hints indentados com `  hint: <texto>`.
  - `SourceResult::Err` usa mesmo formatter (uniformidade).
- **Alternativas rejeitadas**:
  - **Formatter em L1**: confunde concerns (L1 fornece dados;
    L3 formata). ADR-0043 já estabeleceu separação.
  - **Só byte-offset em L1, conversão em L3**: duplica lógica se
    houver múltiplos formatters; `span_to_line_col` centraliza.
  - **Spans detached com formato especial**: aceitar formato
    `<detached>:<message>` é mais honesto que inventar
    linha/coluna fictícios.
- **Limitações documentadas**:
  - Trace (pilha de chamadas) não incluído.
  - Cores, JSON, SARIF — passos futuros.
  - Resolução cross-file requer caller com mapa FileId → Source.
    Fora do escopo.
- **Relação com ADR-0033 (paridade)**: formato `ficheiro:linha:coluna`
  é convenção externa (gcc/clang/rustc), não específica do vanilla.
  Paridade funcional preservada (utilizador vê mensagem
  informativa).

Promover a `EM VIGOR` em 111.E.

### 111.C — Implementação

**111.C.1 — `Source::span_to_line_col`**:

Em `01_core/src/entities/source.rs`:

```rust
impl Source {
    /// Resolve `span` para `(linha, coluna)` 1-indexadas.
    ///
    /// Devolve `None` se:
    /// - o span é detached (sem posição);
    /// - o span refere-se a outro ficheiro (`span.file_id() != self.id()`);
    /// - o offset está fora dos limites do texto (não deve acontecer
    ///   em execução normal, mas defensivo).
    pub fn span_to_line_col(&self, span: Span) -> Option<(u32, u32)> {
        if span.is_detached() || span.file_id() != self.id() {
            return None;
        }
        let offset = span.byte_offset()?;  // ou método equivalente
        let text = self.text();
        if offset > text.len() {
            return None;
        }
        
        // Contar linhas e colunas até offset
        let mut line = 1u32;
        let mut col = 1u32;
        for (i, ch) in text.char_indices() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        Some((line, col))
    }
}
```

Ajustes conforme 111.A:
- Se `Span::byte_offset` não existe, adaptar à API real.
- Se há método `detached()` vs campo privado, ajustar.
- Coluna em chars (Unicode code points) é convenção editor;
  em bytes seria diferente. Ir de chars (escolha gcc/clang).

**111.C.2 — Formatter em L3**:

Em `03_infra/src/integration_tests.rs` (ou ficheiro novo
`03_infra/src/diagnostic_format.rs`):

```rust
pub fn format_diagnostic(
    diag: &SourceDiagnostic,
    source: &Source,
    source_path: &str,
) -> String {
    let severity = match diag.severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
    };
    
    let location = match source.span_to_line_col(diag.span) {
        Some((line, col)) => format!("{}:{}:{}", source_path, line, col),
        None => format!("{}:<detached>", source_path),
    };
    
    let mut out = format!("{}: {}: {}\n", location, severity, diag.message);
    
    for hint in &diag.hints {
        out.push_str(&format!("  hint: {}\n", hint));
    }
    
    out
}
```

Ajustar:
- Se `SourceDiagnostic::severity` não existe como enum separado,
  adaptar.
- Se `diag.hints` é `Vec<EcoString>` vs outro tipo, ajustar loop.
- `source_path: &str` é o caller a passar — tipicamente
  `"input.typ"` ou similar em testes; path real em CLI futura.

**111.C.3 — Substituir `drain_warnings_to_stderr`**:

O helper actual (Passo 106) imprime com `{:?}`. Substituir por
loop com `format_diagnostic`:

```rust
// Antes (Passo 106):
pub fn drain_warnings_to_stderr(warnings: &[SourceDiagnostic]) {
    for diag in warnings {
        eprintln!("warning: {:?} {}", diag.span, diag.message);
    }
}

// Depois:
pub fn drain_diagnostics_to_stderr(
    diagnostics: &[SourceDiagnostic],
    source: &Source,
    source_path: &str,
) {
    for diag in diagnostics {
        eprint!("{}", format_diagnostic(diag, source, source_path));
    }
}
```

Nome muda de `drain_warnings_...` para `drain_diagnostics_...`
porque agora cobre errors também.

**111.C.4 — Integração em `do_eval_with_sink`**:

O helper L3 passa a aplicar o formatter a errors também:

```rust
// Em do_eval_with_sink (ou equivalente):
let result = eval(...);
let warnings = sink.into_diagnostics();

// Imprimir warnings sempre
drain_diagnostics_to_stderr(&warnings, &source, "input.typ");

// Se erro, imprimir errors com mesmo formatter
if let Err(errors) = &result {
    drain_diagnostics_to_stderr(errors, &source, "input.typ");
}
```

Ordem: warnings primeiro (podem dar contexto ao erro), errors
depois. Consistente com convenção gcc.

### 111.D — Testes

**111.D.1 — Testes unitários de `span_to_line_col`**:

Em `01_core/src/entities/source.rs` `#[cfg(test)]`:

1. `span_no_inicio_e_linha_1_coluna_1` — offset 0 → (1, 1).
2. `span_depois_de_newline` — após `"abc\n"`, offset 4 → (2, 1).
3. `span_multibyte_unicode` — `"áéí"` — coluna avança por char,
   não por byte. Offset após `á` (2 bytes) → (1, 2).
4. `span_detached_devolve_none` — `Span::detached()` → `None`.
5. `span_de_outro_ficheiro_devolve_none` — span com FileId
   diferente → `None`.
6. `span_no_fim_do_texto` — offset == text.len() → posição do
   último char + 1.

**111.D.2 — Testes do formatter L3**:

Em `03_infra/src/integration_tests.rs`:

1. `format_warning_com_ficheiro_linha_coluna` — warning com span
   real → output contém `"input.typ:N:M: warning:"`.
2. `format_warning_com_hint` — warning com 1 hint → output
   contém `"  hint: ..."` em segunda linha.
3. `format_warning_com_multiplos_hints` — 2 hints → 2 linhas
   `"  hint:"` sucessivas, ordem preservada.
4. `format_span_detached_usa_fallback` — warning com span
   detached → `"input.typ:<detached>: warning:"`.
5. `format_error_uniforme` — `SourceDiagnostic::error(...)` →
   output usa `"error:"` em vez de `"warning:"`, mesmo formato
   restante.
6. `format_pipeline_debt49` — input `#set text(font: "X")` →
   warning no formato completo, incluindo hint sobre ADR-0040.

Testes existentes que dependiam do formato antigo (`{:?}` em
Span) podem falhar — actualizar para esperarem formato novo.

### 111.E — Encerramento

1. Grep: `"{:?}"` em contexto de Span retorna zero matches em
   `03_infra/src/`.
2. Grep: `drain_warnings_to_stderr` retorna zero matches
   (renomeado ou removido).
3. `cargo test --workspace` passa com contagem ≥ linha de base
   + testes novos (803 + ~6 L1 + ~6 L3).
4. `crystalline-lint` zero violations.
5. ADR-00NN promovida a `EM VIGOR`.
6. Relatório `typst-passo-111-relatorio.md`:
   - API exacta de `span_to_line_col`.
   - Path de FileId usado pelos testes (ex: `"input.typ"`).
   - Exemplo antes/depois de output para um warning típico.
   - Limitações aceites (spans detached, cross-file).
   - Testes novos e números finais.

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 111.A escrito.
2. ADR-00NN criada e promovida.
3. `Source::span_to_line_col` implementado em L1 com testes.
4. Formatter L3 produz output gcc/clang-compatível.
5. `drain_warnings_to_stderr` substituído por
   `drain_diagnostics_to_stderr` (warnings + errors).
6. Testes unitários (L1) e de integração (L3) passam.
7. `cargo test --workspace` passa.
8. `crystalline-lint` zero violations.
9. Relatório 111.E escrito.

---

## O que pode sair errado

- **`Span::byte_offset` não existe publicamente**. Se `Span` é
  opaco, `span_to_line_col` tem de usar API existente de `Source`
  ou adicionar método privado ao Span. Gate 111.A detecta.
- **Coluna em chars vs bytes**. Se há teste existente que
  assume coluna em bytes, falha. Convenção: coluna em chars
  (Unicode code points). Gcc e clang fazem assim.
- **Spans cross-file**. Se um import gera span com FileId
  diferente, `span_to_line_col` devolve `None`. Output fica
  `"input.typ:<detached>"` — inexacto (não é detached, é
  cross-file). Aceitar; passo futuro para resolver.
- **Source path hard-coded em testes**. Testes passam
  `"input.typ"` como path; real CLI passaria o path do
  ficheiro aberto. OK para testes — CLI real é passo separado.
- **`EcoString` vs `String` em `hints`**. Se o campo é
  `Vec<EcoString>`, `format!` funciona na mesma. Se é `EcoVec`
  ou outro, adaptar.
- **Múltiplos diagnósticos com mesmo span**. Formatter imprime
  cada um numa linha. Ordem: preservada da `Vec`. Se Sink
  deduplicou (Passo 104), não há duplicados para imprimir.
- **Performance**: `span_to_line_col` é O(text.len()) por
  chamada (char_indices até offset). Para N warnings, é
  O(N × text.len()). Aceitável para counts típicos (< 100).
  Se aparecer input com milhares de warnings, optimizar com
  cache de line-starts. Fora do escopo.

---

## Notas operacionais

- O `source_path: &str` no formatter é responsabilidade do
  caller passar. Em testes, literal `"input.typ"`. Em CLI
  real (passo futuro), path do argumento.
- A ADR-0043 estabeleceu "L1 só fornece dados; L3 formata".
  Este passo respeita estritamente. `span_to_line_col` em L1
  é **resolução**, não formatação — devolve `(u32, u32)`,
  texto zero.
- `Severity::Error` e `Severity::Warning` assumidas com esses
  nomes. Se o enum no cristalino usa outros (ex: `Kind::Err`,
  `Kind::Warn`), ajustar o formatter.
- Se algum teste actual dependia do formato `{:?}` (improvável,
  mas possível em testes de regressão), actualizar. Contagem
  final tem de aumentar apenas pelos testes novos.
- Passo intencionalmente não toca cores. Cores requerem
  decisão sobre isatty detection, disable flags (`--color=never`),
  env vars (`NO_COLOR`). Passo dedicado quando a CLI real
  aparecer.
