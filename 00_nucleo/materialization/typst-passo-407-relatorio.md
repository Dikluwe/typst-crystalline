# Relatório — Passo 407

**Título**: `text.font` dict: pattern-matching por regex (S-M)  
**Data**: 2026-06-22  
**Tipo**: Refinamento de feature existente (L1 stdlib + eval; zero tipo novo de usuário; zero I/O; reusa `Value::Regex` P402 + `FontList` P140B/P141/P146).

## Resumo executivo

Implementada a forma dict de `#set text(font: (...))`, fechando o **gap 8** de DEBT-52 (font dict), que permanecia como candidato futuro desde o Passo 142. O passo reusa `Value::Regex` (P402) para permitir regex keys no dict, sem adicionar novos tipos de usuário.

## Decisões de engenharia

### 1. `FontNamePattern` (L1)

- Novo enum em `entities/font_list.rs`:
  - `Literal(EcoString)` — lowercased na construção.
  - `Regex(Regex)` — wrapper L1 com `Arc<regex::Regex>`.
- `FontFamily.name` passou de `EcoString` para `FontNamePattern`.
- Adicionado campo `variants: Vec<EcoString>` (armazenado; uso variant-aware continua scope-out ADR-0054bis).
- `covers: Option<Covers>` mantido (enum inabitado; reserva estrutural).

### 2. Parsing de `#set text(font: ...)`

- String/array continuam funcionando (compatibilidade retroativa P292/P373).
- Dict é processado inspeccionando o AST (`Expr::Dict`) porque `Value::Dict` tem keys `EcoString` e não suporta regex keys.
- Keys aceitas: string literal, identificador (`Named`), `regex("...")` (`FuncCall`).
- Values aceitos: string ou array de strings.
- Persistência na chain custom como `Value::Array` de items:
  - `Value::Str(name)` para literais (forma antiga).
  - `Value::Dict` com `"name"` (Str|Regex) + `"variants"` (Array[Str]) para dict.
- Zero tipo novo em `Value`: reusa `Array`, `Dict`, `Str`, `Regex`.

### 3. Resolução de fonte

- `FontBook::select_pattern(&FontNamePattern, &FontVariant)` adicionado.
- Literais: match case-insensitive (`eq_ignore_ascii_case`).
- Regex: scan linear O(n) via `Regex::is_match`.
- Pipeline `resolve_font` atualizado para usar `select_pattern`.

## Ficheiros alterados

| Ficheiro | Alteração |
|----------|-----------|
| `01_core/src/entities/font_list.rs` | `FontNamePattern`, refino `FontFamily`, testes unitários |
| `01_core/src/entities/font_book.rs` | `select_pattern` para regex + literal |
| `01_core/src/rules/eval/rules.rs` | parsing AST de `text.font` dict |
| `01_core/src/rules/layout/text.rs` | decodificação da chain custom para `FontList` |
| `03_infra/src/pipeline.rs` | `resolve_font` usa `select_pattern`; testes regex |
| `01_core/src/rules/eval/tests.rs` | testes de eval dict/regex |
| `01_core/src/rules/layout/tests.rs` | ajuste `family.name.as_str()` |
| `00_nucleo/prompts/entities/font-list.md` | prompt L0 actualizado |
| `00_nucleo/prompts/rules/style/font-dict.md` | novo prompt L0 do parsing |

## Scope-out mantido

- Variant-aware selection (peso/estilo a partir de variant names) — ADR-0054bis condicional.
- Otimização O(1) para literais em dict com regex.
- Fallback chain para múltiplos regex matches — primeiro match wins.
- Dict spread (`..dict`) em `text.font`.

## Validação

```bash
cargo test --workspace -- --skip p350c_flag_on_nao_convergente_classifica
# → todos verdes (o teste p350c mantém stack overflow pré-existente)

crystalline-lint .
# → 0 drift; único warning é prompt órfão show-regex.md (pré-existente)
```

## Inventário / DEBT

- DEBT-52 já estava formalmente encerrado no Passo 142. O **gap 8** (font dict) transitou de "candidato futuro" para **implementado na forma dict legada**.
- A paridade named fields do vanilla (`font: (family: "Name", variant: "Regular", weight: "Bold", style: "Italic")`) ficou **fora do escopo deste fecho**; foi materializada posteriormente no Passo 414.
- Saldo de DEBTs abertos: **10** (inalterado).
