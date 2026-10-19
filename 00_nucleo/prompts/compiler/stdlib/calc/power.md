# L0 — stdlib/calc/power
Hash do Código: 43ae3d72

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/calc/power.rs`. No HEAD `d9e851a850c66905bdb0c3ea4b155a6f36881106` e árvore limpa, `01_core/src/compiler/stdlib/calc.rs:256-359 e 1443-1483` contém o comportamento correspondente. SHA-256, horário e estado estão em `lab/results/construction/diagnostics/atomizacao-calc-2026-09-30-before.json.gz`. Esta decisão apenas separa algoritmos por comportamento.

## Contrato

Implementar pow, sqrt e root. Preservar 0^0, overflow, tipos Int/Float/Decimal, guards finitos, índice inteiro e raiz real negativa apenas para índice ímpar; root recebe radicando antes de índice.

Os detalhes de paridade, tipos, diagnósticos e bindings permanecem em `00_nucleo/prompts/compiler/stdlib/calc.md`. A extração não altera assinatura pública, função de módulo, fase, resultado ou política de erro. Nenhum I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1 e os testes de calc, compilar e passar `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho desta separação.

## Revisão V16 — medição anterior à decisão (2026-10-17)

No HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
`01_core/src/compiler/stdlib/calc/power.rs:32-95` contém três fallbacks abertos:
predicados de zero e despacho de pares. Estado completo, diffstat, hashes e UTC
estão em `lab/results/construction/diagnostics/tekt-v16-power-2026-10-17/before.json.gz`.
O corpus bilateral `before-measurement.json.gz` no mesmo diagnóstico mede 20 casos
contra o pin `9f2b6e8715237cb086899a42873660fe744622e8`, binários identificados
por SHA-256. Em `calc.pow(none,2147483648)`, o cristalino acusa expoente grande;
o vanilla rejeita primeiro a base. Um expoente decimal é rejeitado nos dois,
mas só o vanilla fornece o hint de conversão explícita. `calc.rs:172-201` da
fonte em quarentena confirma base DecNum, expoente Num e validação numérica
após os casts. Isso mede comportamento; não deduz intenção do upstream.

### Decisão interna e classificação

Tipos, resultados, ordem de validação e hint são semântica/diagnóstico da língua.
A forma dos enums e do match é mecânica local: divergir é permitido.
Validar base antes do expoente e ambos antes dos guards de potência. A base
aceita somente Int/Float/Decimal, o expoente somente Int/Float. Rejeitar decimal
no expoente com hint `if loss of precision is acceptable, explicitly cast the decimal to a float with `float(value)``.
O erro de base deve nomear também Decimal. Preservar os demais textos locais,
spans existentes, aridade, argumentos nomeados, resultados/overflow/guard_float,
sqrt e root; não declarar paridade de mensagens/spans ou infinitos.

Projetar Value explicitamente para um enum privado de base com três variantes;
a conversão enumera todos os casos Value não numéricos em um braço de erro.
Projetar o expoente para um enum privado de duas variantes, rejeitando Decimal
antes dos guards. Os predicados de zero e o despacho numérico são exaustivos
sobre esses enums; não usar catch-all, unreachable ou supressão V16. Esta é
uma inferência de manutenção: uma variante Value futura exige decisão na
fronteira; uma sonda de compilação que aceite a variante sem E0004 a refutaria.
O agrupamento explícito dos tipos não numéricos é V19 justificado por uma única
política de erro; não o atomizar em dezenas de braços idênticos. Nenhuma API,
trait, fase ou modo público muda (ADR-0127, fluxo contínuo).

### Aceitação complementar

Testes funcionais de precedência base inválida/expoente grande ou não normal,
rejeição de expoente decimal com hint, preservação de kinds numéricos, 0^0,
overflow e erro decimal×float. Obter RED funcional antes da implementação;
GREEN e suíte completa depois. Sonda enum futura em target isolado deve rejeitar
a conversão Value. Lint diferencial remove apenas três V16 deste owner e permite
V19 somente no agrupamento prescrito. Resselo restrito ao par owner/consumer,
build, fmt, Clippy estrito, Rustdoc, linhagem direta/reversa, diff check e todos
os testes sob cgroup 12 GiB, swap zero e Cargo jobs=1.
