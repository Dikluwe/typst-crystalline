# L0 — stdlib/shapes/line
Hash do Código: 1e3a12bb

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/shapes/line.rs`. No HEAD `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9` e árvore limpa, `01_core/src/compiler/stdlib/shapes.rs:351-506` contém o comportamento extraído. SHA-256, horário e estado estão em `lab/results/construction/diagnostics/atomizacao-shapes-2026-09-30-before.json.gz`. Esta é separação interna por comportamento.

## Contrato

Construir line com start/end/length/angle e extensão dx/dy. Preservar precedência, incompatibilidades, Path para origem não-zero e dimensões da caixa causal.

As regras detalhadas, evidências históricas, limites de paridade e diagnósticos continuam em `00_nucleo/prompts/core/compiler/stdlib/shapes.md`. A extração não altera assinatura pública, defaults, fase ou compatibilidade. Nenhum I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1 e os testes de shapes, compilar e passar `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho.


## Operações numéricas puras — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/stdlib/shapes/line.rs` nas linhas 134, 135 contém as chamadas sinalizadas como disallowed_methods pelo Clippy vigente. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. `lab/results/construction/diagnostics/geometria-numerica-ci-2026-10-13/before.json.gz` conserva fonte, owner, configuração, libm, compilador, UTC, status e diffstat. A sonda host-math compara os backends antes da escolha: 4968 casos, 69 diferenças de até 1 ULP em trigonometria/atan2, sem mudança de classe numérica ou sinal de zero; potências e hypot coincidem na amostra. É medição deste host, não prova universal nem nova paridade contra vanilla.

### Decisão e aceitação

Usar libm::cos/sin para projetar length e angle em dx/dy. Preservar start/end, precedência de argumentos, rejeições, origem do Path e dimensões causais. Não introduzir arredondamento, clamp ou novo default.

Mudança interna em fluxo contínuo ADR-0127, sem contrato público novo, default ou fase. Preservar testes de geometria/linguagem existentes e comparar os exemplos SVG medidos antes/depois, declarando diferenças. A comparação geométrica não certifica todos os casos ou plataformas. Validar build, todos os targets, core, Clippy e linhagem. Não suprimir o lint nem substituir aceitação por identidade de bytes do compilador.
