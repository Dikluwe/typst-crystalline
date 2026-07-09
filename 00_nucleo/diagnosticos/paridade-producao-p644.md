# Relatório de Paridade — P644

**Passo:** 644  
**Data:** 2026-07-09  
**Foco:** Entradas de bibliografia omitidas silenciosamente — casos 5 e 6 de P633.  
**Dependências:** P633 (casos confirmados), P638 (comportamento do vanilla testado directamente).

---

## 1. Sonda

### 1.1 `hay_entry_to_bib_entry` (`01_core/src/rules/eval/bibliography.rs:146`)

Função convertia `hayagriva::Entry` → `Option<BibEntry>`. A condição:

```rust
if key.is_empty() || (author.is_empty() && title.is_empty()) {
    return None;
}
```

- Chave vazia → `None` (silêncio).
- Sem título nem autor → `None` (silêncio).

Caller usava `filter_map`, descartando silenciosamente.

### 1.2 `bib_entry_to_hayagriva` (`01_core/src/rules/layout/bib_csl.rs:214`)

Função convertia `BibEntry` → `Option<Entry>` gerando YAML intermédio:

```rust
let library = hayagriva::io::from_yaml_str(&yaml).ok()?;
library.get(&entry.key).cloned()
```

- Falha de YAML → `None`.
- Chave não encontrada no library → `None`.

Caller usava `filter_map`, descartando silenciosamente.

### 1.3 Comportamento do vanilla (confirmado por P638)

| Situação | Vanilla |
|---|---|
| Chave vazia | Erro: `bibliography contains entry with empty key` |
| Sem título | Aceite — renderiza de forma degradada |
| YAML inválido | Erro propagado |

---

## 2. Implementação

### 2.1 `hay_entry_to_bib_entry`

- Assinatura alterada para `SourceResult<BibEntry>`.
- Removeu-se a condição `author.is_empty() && title.is_empty()` — entradas sem título/autor já não são omitidas.
- Chave vazia devolve erro:

```rust
if key.is_empty() {
    return Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "bibliography contains entry with empty key".to_string(),
    )]);
}
```

- Caller `parse_bibliography` alterado de `filter_map` para `map(...).collect::<Result<Vec<_>, _>>()?`.

### 2.2 `bib_entry_to_hayagriva`

- Assinatura alterada para `SourceResult<Entry>`.
- Chave vazia verificada antes de gerar YAML:

```rust
if entry.key.is_empty() {
    return Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "bibliography contains entry with empty key".to_string(),
    )]);
}
```

- Falha de `from_yaml_str` propaga erro:

```rust
let library = hayagriva::io::from_yaml_str(&yaml).map_err(|e| {
    vec![SourceDiagnostic::error(
        Span::detached(),
        format!("failed to parse bibliography entry '{}': {}", entry.key, e),
    )]
})?;
```

- Chave inexistente no library também propaga erro descritivo.

### 2.3 Propagação de erros de layout

`layout_with_introspector_and_metrics` retorna `PagedDocument`, não `SourceResult`. Para propagar erros de `bib_entry_to_hayagriva` (chamado durante o layout):

- Adicionado campo `layout_errors: Vec<String>` a `PagedDocument` (`01_core/src/entities/layout_types.rs`).
- Adicionado campo `layout_errors` ao `Layouter` (`01_core/src/rules/layout/mod.rs`).
- `build_cache` e `build_cache_with_style` passaram a devolver `SourceResult<BibRenderCache>`.
- Em `layout_with_introspector_and_metrics`, erros de `build_cache*` são colectados em `layout_errors` e passados ao `Layouter`.
- Em `Layouter::finish()`, `layout_errors` são copiados para o `PagedDocument`.
- Em `03_infra/src/pipeline.rs`, após o layout, se `doc.layout_errors` não estiver vazio, o pipeline retorna `Err(errors)`.

### 2.4 Callers de `build_cache`/`build_cache_with_style`

- Teste de style inválido alterado de `.is_none()` para `.is_err()`.
- Outros testes que usavam `.unwrap()` continuam a funcionar (panicam em `Err`).

---

## 3. Testes

### 3.1 `01_core/src/rules/eval/bibliography.rs`

- `p644_yaml_chave_vazia_quoted_produz_erro`: chave vazia em YAML quoted gera erro.
- `p644_yaml_sem_titulo_aceite`: entrada YAML sem título é aceite e não omitida.

### 3.2 `01_core/src/rules/layout/bib_csl.rs`

- `p644_bib_entry_to_hayagriva_chave_vazia_produz_erro`: chave vazia gera erro.
- `p644_bib_entry_to_hayagriva_yaml_invalido_produz_erro`: caractere de controlo no título faz `from_yaml_str` falhar e propaga erro.

---

## 4. Validação

```bash
cargo test --workspace
```

Resultado: **todos os testes passaram**.

```bash
crystalline-lint .
```

Resultado: **✓ No violations found**.

---

## 5. Decisão

- Chave vazia produz erro em ambos os caminhos (`hay_entry_to_bib_entry` e `bib_entry_to_hayagriva`).
- Entradas sem título são aceites (não omitidas).
- YAML inválido propaga erro.
- Erros de layout são transportados via `PagedDocument.layout_errors` e convertidos a erro de compilação pela pipeline.

---

## 6. Estado de fecho

- [x] Sonda mínima completa.
- [x] Chave vazia produz erro.
- [x] Entrada sem título aceite e não omitida.
- [x] YAML inválido produz erro.
- [x] Callers atualizados.
- [x] `cargo test --workspace` sem regressões.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p644.md`.

---

## 7. Hash do commit

`PENDING`
