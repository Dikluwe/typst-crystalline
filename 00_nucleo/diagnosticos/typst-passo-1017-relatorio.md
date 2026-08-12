# Passo 1017 — Relatório final

**Data**: 2026-08-12  
**Commit de base**: `b7dc28b67` (refactor(eval): long_type_name com ponto único de verdade — Passo 1015)  
**Ficheiros alterados**:

- `01_core/src/compiler/eval/bindings/access.rs` — removeu-se `long_type_name`; importa `vanilla_type_name` de `error_formatting.rs`
- `01_core/src/compiler/eval/bindings/mod.rs` — removeu reexport de `long_type_name`
- `01_core/src/compiler/eval/bindings/field_access.rs` — importa `vanilla_type_name`
- `01_core/src/compiler/eval/bindings/value_methods.rs` — importa `vanilla_type_name`
- `01_core/src/compiler/eval/bindings/method_dispatch.rs` — importa `vanilla_type_name`
- `01_core/src/compiler/eval/mod.rs` — removeu reexport de `long_type_name`; importa e usa `vanilla_type_name`
- `01_core/src/compiler/eval/call_dispatch.rs` — usa `vanilla_type_name`
- `01_core/src/compiler/eval/operators/join.rs` — importa `vanilla_type_name`
- `01_core/src/compiler/stdlib/plugin.rs` — importa `vanilla_type_name`
- `01_core/src/compiler/stdlib/foundations.rs` — importa `vanilla_type_name`
- `01_core/src/compiler/stdlib/eval.rs` — importa `vanilla_type_name`
- `01_core/src/compiler/stdlib/figure_image.rs` — usa `vanilla_type_name`
- `01_core/src/compiler/stdlib/loading.rs` — removeu cópia privada de `vanilla_type_name`; importa a canónica
- `01_core/src/compiler/stdlib/pdf.rs` — removeu cópia privada de `vanilla_type_name`; importa a canónica
- `00_nucleo/prompts/compiler/eval/bindings/access.md` — L0 actualizado
- `00_nucleo/prompts/compiler/eval/operators/error_formatting.md` — L0 actualizado

---

## Resumo

Consolidámos as seis implementações históricas da função de nome longo de tipo numa única canónica: `vanilla_type_name` em `compiler/eval/operators/error_formatting.rs` (tabela completa de 36 arms).

### Estado antes

- `long_type_name` em `eval/bindings/access.rs` (pub(crate), 3 arms + fallback).
- `long_type_name` privada em `eval/operators/join.rs` (removida no P1015).
- `long_type_name` privada em `stdlib/foundations.rs` (removida no P1015).
- `vanilla_type_name` privada em `stdlib/loading.rs` (3 arms + fallback).
- `vanilla_type_name` privada em `stdlib/pdf.rs` (3 arms + fallback).
- `vanilla_type_name` pub(crate) em `eval/operators/error_formatting.rs` (36 arms).

### Estado depois

Apenas a última sobrevive. Todos os consumidores importam directamente de `crate::compiler::eval::operators::error_formatting::vanilla_type_name`.

### Porque `vanilla_type_name` e não `long_type_name`

Decisão do dono registada no Passo 1015: a forma com tabela explícita de 36 arms falha de compilação quando `Value` ganha uma variante nova, o que é preferível a herdar silenciosamente o nome curto via `type_name()`.

---

## Validação

```
cargo test --workspace
```

Resultado:

- typst-core: 4968 passed
- typst-infra: 787 passed
- typst-shell: 41 passed
- benches: 2 passed
- wiring: 37 passed
- crystalline_lint: 2 passed

Total: **5839 tests passed**, 0 failed.

```
crystalline-lint .
```

Zero erros. Três warnings V7 pré-existentes (`auditar-fatiamento.md`, `auditar-spec.md`, `package_version_resolution.md`), nada relacionado com esta mudança.

```
crystalline-lint --fix-hashes .
```

Nenhum drift restante.

---

## Notas para passos futuros

- A mensagem de erro é o observável (ADR-0107); a equivalência mecânica entre as formas foi provada no P1015, logo esta consolidação não altera comportamento.
- A tabela completa em `error_formatting.rs` deve ser actualizada quando novas variantes de `Value` forem adicionadas a L1.
