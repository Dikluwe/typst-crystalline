# Passo 411 — Relatório de execução: Field Access `Version`

**Tipo**: Materialização (L1 eval).  
**Data**: 2026-06-22.  
**Status**: concluído.

---

## 1. Sonda do substrato (FASE A.0)

Executados os comandos de sonda definidos em `typst-passo-411.md`:

- `Value::Version` existe em `entities/value.rs` ✅
- `Version` tipo L1 tem 5 campos públicos (`major`, `minor`, `patch`, `pre`, `build`) ✅
- `eval_field_access` existe em `rules/eval/bindings.rs` (match em `Value::Dict` e `Value::Content`) ✅
- `Value::Version` ainda não estava em `eval_field_access` ✅
- `Value::Array` existe para retorno de `.pre`/`.build` ✅

**Resultado da sonda**: critérios todos passados; passo é S/M viável.

---

## 2. Prompt L0 (FASE A)

- Prompt criado em `00_nucleo/prompts/rules/eval/version-field-access.md`.
- Descreve field access `.major`, `.minor`, `.patch`, `.pre`, `.build` para `Value::Version`.
- Checkpoint considerado salvo (prompt versionado no repo).

---

## 3. Implementação (FASE B)

Arquivo alterado: `01_core/src/rules/eval/bindings.rs`.

Adicionado ramo `Value::Version(v)` em `eval_field_access`:

```rust
Value::Version(v) => {
    match field.as_str() {
        "major" => Ok(Value::Int(v.major as i64)),
        "minor" => Ok(Value::Int(v.minor as i64)),
        "patch" => Ok(Value::Int(v.patch as i64)),
        "pre" => Ok(Value::Array(v.pre.iter().map(|s| Value::Str(s.clone())).collect())),
        "build" => Ok(Value::Array(v.build.iter().map(|s| Value::Str(s.clone())).collect())),
        _ => Err(vec![SourceDiagnostic::error(
            access.span(),
            format!("campo desconhecido em version: '{}'", field),
        )]),
    }
}
```

- `u64` → `Value::Int(i64)` para `major`/`minor`/`patch`.
- `Vec<EcoString>` → `Value::Array` de `Value::Str` para `.pre` e `.build`.
- Campos vazios retornam `Value::Array([])`.
- Campo desconhecido retorna erro eval com mensagem clara.

Também foi adicionado `@prompt` para `version-field-access.md` no header de `bindings.rs`.

---

## 4. Testes

Adicionados 8 testes em `01_core/src/rules/eval/tests.rs` (bloco `P411 — Field Access Version`):

- `version_field_major` → `Value::Int(1)`
- `version_field_minor` → `Value::Int(2)`
- `version_field_patch` → `Value::Int(3)`
- `version_field_pre` → `Array([Str("alpha"), Str("1")])`
- `version_field_pre_empty` → `Array([])`
- `version_field_build` → `Array([Str("build"), Str("2")])`
- `version_field_build_empty` → `Array([])`
- `version_field_unknown` → erro eval

---

## 5. Validações

- `cargo test -p typst-core version_field` — 8 testes passaram; 0 falhas.
- `cargo test --workspace -- --skip p350c_flag_on_nao_convergente_classifica` — verde.
- `crystalline-lint` — 0 erros / 0 drift; resta apenas warning pré-existente de `show-regex.md` órfão.
- Hashes propagados manualmente (o linter `--fix-hashes` apresenta comportamento de troca de hashes com prompts múltiplos; ajustado manualmente para `eval.md @prompt-hash 7a92cc2d` e `version-field-access.md @prompt-hash 7dde40ef`).

---

## 6. Conclusão

Field access para `Value::Version` implementado e testado. Nenhum tipo novo, nenhuma mudança em `entities/version.rs` (campos já eram públicos), nenhum I/O. O passo cumpre todos os critérios de aceitação.
