# P1301 — recibo de implementação segregada P6

**Estado:** `IMPLEMENTED_PENDING_INDEPENDENT_VERIFICATION`
**Regime:** protocolo completo de materialização segregada
**Atestação:** executado sem atestação de isolamento técnico

## Papel e capacidades

- Executor: subagente fresco P6 `/root/p1301_impl`, no workspace partilhado
  `/repos/Antigravity/typst-crystalline`.
- Papel: implementador; não foi autor do contrato, oracle, mutantes, testes A/B,
  selo ou veredito.
- Artefactos de repositório lidos: `AGENTS.md`, manifesto P1301, Prompt L0 de
  `field_access`, contrato público selado, selo do contrato e fonte produtiva
  pré-candidata `field_access.rs`.
- Escritas: somente
  `01_core/src/compiler/eval/bindings/field_access.rs` e este recibo.
- Entradas proibidas não lidas nem executadas: `compiler/eval/tests.rs`, recibo
  RED, oracle, mutantes e qualquer teste candidato.

O ambiente usa filesystem e processo partilhados, portanto este recibo prova a
allowlist lógica observada pelo executor, não isolamento técnico do workspace.

## Entradas congeladas validadas antes da edição

| Artefacto | SHA-256 observado |
|---|---|
| `00_nucleo/diagnosticos/p1301-manifest.json` | `074fabeda17f741f064fe1d561ad6a78ad81e45654861825586664a9646f05a8` |
| `00_nucleo/prompts/compiler/eval/bindings/field_access.md` | `38d6f5cdee302491ec059577174d4f6dc6d3c28e3d2da2748f61c05853cc0c16` |
| `00_nucleo/diagnosticos/p1301-contract.json` | `2d8d7a141d63c13473681abdefa24ed9370e03925824a9cae74ca26305ea85f5` |
| `00_nucleo/diagnosticos/p1301-contract-seal.json` | `658fe879eaf7dce5b0a6372f686e376e7cc652f621d298c817628b03e16f82fb` |
| fonte produtiva pré-candidata | `7873e47635ad2e99df9132bae9b13472581f5026a3f3a671f2f48de8a3b15497` |

As quatro entradas seladas foram novamente re-hashadas depois dos gates e
permaneceram byte-idênticas.

## Proveniência e tempo

- HEAD: `1f082370e59939de7b57992e137a9f74bfb6758f`.
- Estado: working tree não commitida, já suja antes de P6; as alterações alheias
  foram preservadas.
- Início da implementação: `2026-09-03T21:31:56.117683006-03:00`.
- Fim da recolha de evidência: `2026-09-03T21:34:00.182727249-03:00`.
- Duração de parede da janela registrada: aproximadamente `124.065 s`.
- Delta produtivo P6 contra a fonte pré-candidata: `1 file changed, 5
  insertions(+), 2 deletions(-)`.

## Implementação candidata

Em `eval_field_access`, qualquer alvo `Value::Module` seleciona
`access.field().span()` antes de delegar ao lookup. A exceção P1293
`Value::Float` + `is-nan` foi preservada literalmente; todos os demais targets
continuam a usar `access.span()`.

No braço fechado `Value::Module`, lookup bem-sucedido continua a devolver o
binding clonado sem mudança. Lookup ausente agora emite exatamente
``module `<public_name>` does not contain `<field>` `` sem hints. O nome interno
`std` projeta `global`; qualquer outro módulo preserva `m.name()`. Não existe
lista ou teste de fields, não houve mudança de `repr(std)`, API, entidade,
default, ordem de avaliação ou fase.

SHA-256 da implementação candidata, antes do resselo P7:
`e266c40b625621fcf318e5c8d95556d5da708903ba4e59400f6f487b61483fcd`.

Diff produtivo registrado:

```diff
@@ eval_field_access
-    let span = if field == "is-nan" && matches!(&target, Value::Float(_)) {
+    let span = if matches!(&target, Value::Module(_))
+        || (field == "is-nan" && matches!(&target, Value::Float(_)))
+    {
         access.field().span()
     } else {
         access.span()
@@ Value::Module
         Value::Module(m) => m.scope().get(field).cloned().ok_or_else(|| {
+            let public_name = if m.name() == "std" { "global" } else { m.name() };
             vec![SourceDiagnostic::error(
                 span,
-                format!("module '{}' does not contain field \"{field}\"", m.name()),
+                format!("module `{public_name}` does not contain `{field}`"),
             )]
         }),
```

O `@prompt-hash` e o `Hash do Código` não foram ressellados, por fronteira
explícita desta fase; P7 é o owner desse resselo.

## Comandos e resultados

1. `sha256sum` sobre manifesto, L0, contrato, selo e fonte pré-candidata —
   `PASS`, hashes acima.
2. `rustfmt --edition 2021 --check
   01_core/src/compiler/eval/bindings/field_access.rs` — `PASS`, sem diff.
3. `cargo check --manifest-path 01_core/Cargo.toml --lib` — `PASS` em
   `18.67 s`; `typst-core` gerou 72 warnings já presentes fora do delta P6.
4. `cargo build --manifest-path 01_core/Cargo.toml --lib` — `PASS` em
   `31.59 s`; os mesmos 72 warnings, sem erro.
5. `/usr/bin/time -p rustfmt --edition 2021 --check
   01_core/src/compiler/eval/bindings/field_access.rs` — `PASS`; `real 0.03 s`,
   `user 0.02 s`, `sys 0.01 s`.
6. `git diff --check --
   01_core/src/compiler/eval/bindings/field_access.rs` — `PASS`.

Nenhum comando de teste, corpus, runner, oracle, mutant ou binário funcional
foi executado. Consequentemente este recibo atesta somente implementação,
formatação e compilação da library; o veredito funcional e o certificado
pertencem ao verificador independente posterior.
