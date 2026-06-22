# Prompt L0 — `rules/eval` — Field Access `Version`
Hash do Código: dd4e5d3b

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/eval/bindings.rs` (`eval_field_access`)
**Origem**: Passo 411 (`typst-passo-411.md`) — materializar field access em `Value::Version`.
**ADRs relevantes**: ADR-0107 (paridade linguagem), ADR-0033 (paridade vanilla), ADR-0108 (medir-antes-de-decidir).
**Prompt pai**: `00_nucleo/prompts/rules/eval.md`

---

## 1. Contexto

P401 modelou `Version` em L1; P403 ativou o constructor `version("1.2.3")`; P406 activou comparações. O tipo ainda **não tem field access**:

```typ
#version("1.2.3-alpha+build.1").major  // 1
#version("1.2.3-alpha+build.1").minor  // 2
#version("1.2.3-alpha+build.1").patch  // 3
#version("1.2.3-alpha+build.1").pre    // ("alpha",)
#version("1.2.3-alpha+build.1").build  // ("build", "1")
```

Este passo adiciona o ramo `Value::Version(v)` em `eval_field_access` para os 5 campos acima.

---

## 2. Interface pública

Field access via operador `.` em expressões Typst:

```typ
version("1.2.3-alpha.1+build.2").major  // Int
version("1.2.3-alpha.1+build.2").minor  // Int
version("1.2.3-alpha.1+build.2").patch  // Int
version("1.2.3-alpha.1+build.2").pre    // Array<Str>
version("1.2.3-alpha.1+build.2").build  // Array<Str>
```

---

## 3. Semântica

- `eval_field_access` (em `rules/eval/bindings.rs`) faz match no `target` e delega para o tipo.
- Adicionar ramo `Value::Version(v)` ao match de `target`.
- Campos suportados:
  - `"major"` → `Value::Int(v.major as i64)`
  - `"minor"` → `Value::Int(v.minor as i64)`
  - `"patch"` → `Value::Int(v.patch as i64)`
  - `"pre"` → `Value::Array(v.pre.iter().map(|s| Value::Str(s.clone())).collect())`
  - `"build"` → `Value::Array(v.build.iter().map(|s| Value::Str(s.clone())).collect())`
- Campo desconhecido → erro eval com mensagem clara.
- `.pre` e `.build` vazios retornam `Value::Array([])` (não erro).

---

## 4. Implementação

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

---

## 5. Erros

- Campo desconhecido → erro eval: `"campo desconhecido em version: '{field}'"`.
- Field access em tipo não suportado → erro existente (não alterado).

---

## 6. Scope-out

- Não adicionar `.raw` (string original) — não existe no vanilla.
- Não implementar field access mutável (set).
- Não adicionar `get_field` em `entities/version.rs`.
- Não tocar em `Duration` ou `Decimal`.

---

## 7. Critérios de Verificação

```
Dado version("1.2.3").major
Quando eval
Então Value::Int(1)

Dado version("1.2.3-alpha.1+build.2").pre
Quando eval
Então Value::Array([Str("alpha"), Str("1")])

Dado version("1.2.3").pre
Quando eval
Então Value::Array([])

Dado version("1.2.3").foo
Quando eval
Então erro eval
```

---

## 8. Testes

- `version_field_major` → `Value::Int(1)`.
- `version_field_minor` → `Value::Int(2)`.
- `version_field_patch` → `Value::Int(3)`.
- `version_field_pre` → array com strings.
- `version_field_pre_empty` → array vazio.
- `version_field_build` → array com strings.
- `version_field_build_empty` → array vazio.
- `version_field_unknown` → erro eval.
