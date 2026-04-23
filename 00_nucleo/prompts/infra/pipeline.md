# Pipeline — L3 orquestração
Hash do Código: 88dcf9b6

## Módulo
`03_infra/src/pipeline.rs`

## Propósito

Orquestra o pipeline completo de compilação em L3. Esconde o
boilerplate `comemo` (Routines, Traced, Sink, Route) e expõe
APIs alto-nível à L4 (04_wiring) e aos testes.

Materializado no Passo 113 (ADR-0046) a partir de helpers
test-only em `integration_tests.rs`.

## Contrato

### `eval_to_module_with_sink`

```rust
pub fn eval_to_module_with_sink(
    world: &dyn World,
    source: &Source,
) -> (SourceResult<Module>, Vec<SourceDiagnostic>);
```

- Chama `eval()` com o boilerplate `comemo` completo.
- Warnings drenados do `Sink` e devolvidos separadamente.
- Não formata — retorna `Vec<SourceDiagnostic>` cru.
- Caller (CLI ou testes) decide formatação via
  `diagnostic_format::drain_diagnostics_to_stderr`.

### `compile_to_pdf_bytes`

```rust
pub fn compile_to_pdf_bytes(
    world: &dyn World,
    source: &Source,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>);
```

- Pipeline `eval → introspect → layout → export_pdf`.
- Usa `export_pdf` (sem fonte custom, Helvetica Type1). Para
  output com fonte real, caller compõe manualmente via
  `eval_to_module_with_sink` + `export_pdf_with_font`.
- Warnings sempre devolvidos (mesmo em erro) — caller decide se
  os imprime.
- Módulo sem `content` (AST puramente executivo) produz
  `Ok(Vec::new())` — não é erro.

## Integração com ADR-0045

As funções deste módulo **não formatam** diagnósticos. O caller
usa `diagnostic_format::format_diagnostic` /
`drain_diagnostics_to_stderr` para converter
`Vec<SourceDiagnostic>` em texto gcc/clang-compatível.

Separação alinhada com ADR-0043 (L1 data-only) e ADR-0045
(formatação em L3 — num módulo próprio).
