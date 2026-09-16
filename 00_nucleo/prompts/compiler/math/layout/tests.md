# Prompt L0 — `compiler/math/layout/tests` — suíte matemática
Hash do Código: 57c9f8a4


**Camada:** L1, testes
**Ficheiro proprietário:** `01_core/src/compiler/math/layout/tests.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/layout-observables.toml sha256:42a441e59c53fdc5bf619005c0e018eb63251533fb6dc6c16c13d000e592e40c

## Contrato

A suíte verifica despacho, estilos, baseline, extents, spacing e regressões dos
submódulos matemáticos usando métricas injetadas e oráculos medidos. Testes não
legitimam o algoritmo produtivo nem elevam coordenadas de fixture a contrato.
Casos novos registram a fonte/proveniência da fórmula ou do observável.

## P1426 — reparo mecânico das fixtures (APROVADO)

Os 14 callsites existentes de `Content::math_root` recebem o terceiro
argumento exigido pelo novo constructor. Fixtures de raiz quadrada usam
`index_explicit:false`; fixtures com índice presente usam `true`. Isto não
altera coordenadas, oráculos nem algoritmo de layout. O reparo foi confirmado
explicitamente pelo dono em 2026-09-16.
