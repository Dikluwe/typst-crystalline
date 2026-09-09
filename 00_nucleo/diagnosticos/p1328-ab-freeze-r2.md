# P1328 — sucessor mecânico do snippet, freeze r2

Em 2026-09-09T11:50:23Z, antes de C conforme informação do coordenador,
foi criado `p1328-ab-tests-r2.rs` a partir exclusivamente de R1, sem ler
o arquivo calc.rs integrado nem runtime. Regime A/B e capacidades R1
mantidos; não houve mudança de intenção, asserts ou observáveis.

R1 preservado, SHA-256
`fa971dded3237a1742905db2a45e3d94aef51a853fed17ca8046a0a8ba1cd353`.
R2 congelado, SHA-256
`b689de73f3bbe572ed4cc9a0fea0fc29a792f91a8125cc8e1a657a19e676315e`.
Config `rustfmt.toml`, SHA-256
`02ed44fa483d79cb29e2030e990fa0a840ca4db57c2ce9935e10d1e85142718e`.

Validação `rustfmt --check --edition 2021 --config-path rustfmt.toml
00_nucleo/diagnosticos/p1328-ab-tests-r2.rs` sem diferenças. O diff integral
R1→R2 contém somente a ordenação/formatação dos mesmos quatro imports:

```diff
-        EvalContext, EvalTarget, eval_expression_with_features,
-        eval_with_full_error_target_and_features,
+        eval_expression_with_features, eval_with_full_error_target_and_features,
+        EvalContext, EvalTarget,
```

CLI runner, medidas BASE/VANILLA e expectativas literais seguem R1,
inalterados. Nenhum teste foi compilado por esta autoridade. O coordenador
informou RED R1 válido; a revisão mecânica não acrescenta evidência de
execução nem muda a classificação das falhas. R2 substitui apenas o
snippet a integrar no owner e conserva o filtro `p1328`.
