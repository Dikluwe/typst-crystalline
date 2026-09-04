# P1301r2 — recibo de implementação P6-r2

## Estado

Implementação candidata materializada no regime Tekt completo, papel
`P6-r2` / implementador independente.

Atestação: **executado sem atestação de isolamento técnico**.

A fase foi iniciada somente após o root comunicar a reprodução do RED P5-r2:
receipt SHA-256
`4a34b2006daebea10ee250ef6c249acc2ad06b5ed36b67ac965a48f87f7c6913`,
`4` testes, `2 passed`, `2 failed` semânticos e `exit 101`. O selo P4-r2 foi
tratado como autorização apenas de P5-r2; a autorização de P6-r2 veio do root
depois desse RED.

## Entradas congeladas e prehash

Hashes confirmados antes da escrita:

- Prompt L0 `00_nucleo/prompts/compiler/eval/bindings/field_access.md`:
  `38d6f5cdee302491ec059577174d4f6dc6d3c28e3d2da2748f61c05853cc0c16`.
- Manifesto `00_nucleo/diagnosticos/p1301r2-manifest.json`:
  `1526dc81e08a36ce0153fe7639dd2558c220a85d44c4b1de7098a59c099a5712`.
- Contrato `00_nucleo/diagnosticos/p1301r2-contract.json`:
  `69e852d38636afd0ce996f10a64cbf0e6dc0be12dca453a65ee3e0d515c88fad`.
- Selo `00_nucleo/diagnosticos/p1301r2-contract-seal.json`:
  `760846d4be24e6422b3284ae0853bc7647114cdfa25a92909207b1b10de9dcca`,
  estado `SEALED`, veredito `PASS`.
- Receipt RED `00_nucleo/diagnosticos/p1301r2-red-tests-receipt.md`:
  `4a34b2006daebea10ee250ef6c249acc2ad06b5ed36b67ac965a48f87f7c6913`.
- Consumer produtivo antes da edição
  `01_core/src/compiler/eval/bindings/field_access.rs`:
  `7873e47635ad2e99df9132bae9b13472581f5026a3f3a671f2f48de8a3b15497`,
  igual ao prehash declarado pelo manifesto.

Depois da edição, os cinco artefactos protegidos acima conservaram exatamente
os mesmos SHA-256. O consumer produtivo passou a:
`e929aa9004ad74497c1bc40fbc64498877dd155423f88707231bed98ae2fdf81`.

## Forma da mudança

Somente `01_core/src/compiler/eval/bindings/field_access.rs` foi editado:

- qualquer `Value::Module` passa `access.field().span()` ao lookup;
- a falha usa exatamente
  ``module `<nome-público>` does not contain `<field>` ``;
- o nome interno `std` é projetado semanticamente para `global`; os demais
  módulos usam `Module::name()`;
- o lookup bem-sucedido continua a clonar e devolver o mesmo binding;
- a exceção P1293 de `Value::Float` + `is-nan` foi preservada;
- targets não-`Module` preservam o span anterior;
- não foi criada blacklist de fields, correção de `repr(std)`, fallback
  reflexivo, API pública, entidade, wrapper, default ou mudança de fase.

O header, `@prompt-hash` e o prompt L0 permaneceram inalterados. `rustfmt` foi
executado somente no consumer produtivo e terminou com `exit 0`.

## Diff candidato exato

```diff
@@
-    let span = if field == "is-nan" && matches!(&target, Value::Float(_)) {
+    let span = if matches!(&target, Value::Module(_))
+        || field == "is-nan" && matches!(&target, Value::Float(_))
+    {
         access.field().span()
@@
         Value::Module(m) => m.scope().get(field).cloned().ok_or_else(|| {
+            let name = if m.name() == "std" { "global" } else { m.name() };
             vec![SourceDiagnostic::error(
                 span,
-                format!("module '{}' does not contain field \"{field}\"", m.name()),
+                format!("module `{name}` does not contain `{field}`"),
             )]
         }),
```

## Proveniência

Snapshot recolhido em `2026-09-03T22:23:30,392580636-03:00`, depois de
`rustfmt` e antes da criação deste recibo.

`HEAD`:

```text
1f082370e59939de7b57992e137a9f74bfb6758f
```

Working tree: **não commitada / dirty**. SHA-256 de `git status --short` nesse
instante:
`5e2f6227c576bfa14d736fd819ba54a36f244b41c5a58dc2186d329d69063877`.

`git diff HEAD --stat` nesse instante:

```text
 00_nucleo/prompts/compiler/eval.md                 |  39 ++-
 .../prompts/compiler/eval/bindings/field_access.md |  72 ++++
 00_nucleo/prompts/compiler/eval/tests.md           |  90 ++++-
 00_nucleo/prompts/compiler/stdlib/color.md         |  46 ++-
 01_core/src/compiler/eval/bindings/field_access.rs |   7 +-
 01_core/src/compiler/eval/mod.rs                   |  14 +-
 01_core/src/compiler/eval/tests.rs                 | 363 ++++++++++++++++++++-
 01_core/src/compiler/stdlib/color.rs               |   6 +-
 8 files changed, 615 insertions(+), 22 deletions(-)
```

O stat inclui mudanças preexistentes do utilizador e de fases segregadas
anteriores. A contribuição P6-r2 limita-se ao diff candidato reproduzido acima
e a este recibo.

## Capacidades e gates

Leituras do repositório limitaram-se ao allowlist concedido: prompt owner,
manifesto, contrato, selo, receipt RED e consumer produtivo. Não foram lidos
os testes candidatos, prompt test-only, oracle, mutantes, runner, receipts
proibidos, artefactos P1301 v1, `00_nucleo/materialization/` ou
`00_nucleo/context/`.

Escritas limitaram-se ao consumer produtivo e a este receipt. Por instrução da
fase, nenhum teste, build, lint, runner, GREEN, verificação de refinamento ou
resselo de lineage foi executado. Este recibo atesta somente a forma e a
proveniência da implementação candidata; não certifica paridade nem
equivalência funcional.
