# L0 — stdlib/calc/abs
Hash do Código: a8926822

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/calc/abs.rs`. No HEAD `d9e851a850c66905bdb0c3ea4b155a6f36881106` e árvore limpa, `01_core/src/compiler/stdlib/calc.rs:133-255` contém o comportamento correspondente. SHA-256, horário e estado estão em `00_nucleo/diagnosticos/atomizacao-calc-2026-09-30-before.json.gz`. Esta decisão apenas separa algoritmos por comportamento.

## Contrato

Implementar calc.abs com prioridade causal de argumentos e spans. Int usa checked_abs, Float/Decimal conservam tipo e valores dimensionais respeitam suas condições de magnitude. Preservar erros e hints de tipos/posicionais.

Os detalhes de paridade, tipos, diagnósticos e bindings permanecem em `00_nucleo/prompts/compiler/stdlib/calc.md`. A extração não altera assinatura pública, função de módulo, fase, resultado ou política de erro. Nenhum I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1 e os testes de calc, compilar e passar `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho desta separação.


## V16 — vocabulário explícito do diagnóstico (2026-10-17)

### Medição anterior à decisão

`00_nucleo/diagnosticos/tekt-v16-abs-2026-10-17/before.json.gz`,
`before-measurement.json.gz` e `supplement-before-measurement.json.gz`
registram HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não
commitado, UTC/status/diffstat, fontes/L0/binários e argv. O oráculo é o pin
9f2b6e8715237cb086899a42873660fe744622e8, SHA do binário7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
Foram medidos36 pares iniciais e3 suplementares.37 são testemunhas de abs
válidas e coincidem nos resultados/exit/mensagens/hints medidos; dois usam
literal -9223372036854775808, rejeitado pela validação de negação no eval
vanilla (typst-eval/src/ops.rs:116–133) e observado como Float no Tekt;
ficam como dívida de construção/validação do literal, anterior a abs. int.min produz
overflow antes de extras, inclusive via with. Não atribuir esse débito a abs.
Fonte pinada foundations/calc.rs:143–164 conserva os tipos admitidos e usa
ToAbs para a fronteira de erro. abs.rs:65–69 distingue os aliases string e
boolean, delegando os demais nomes no wildcard. O SARIF congelado da etapa
Par/Box aponta V16 nessa delegação, sem afirmar que ela devolva default neutro.
São diagnósticos da linguagem; buffers/particionamento de match são mecânica.
A intenção histórica não é inferida; a revisão estrutural é refutada por
variante futura silenciosa, nome alterado, tipo/resultados ou spans divergentes.

### Decisão contínua ADR-0127 e política V19

Enumerar explicitamente o domínio Value no match do nome: Str→string,
Bool→boolean, e todas as demais37 variantes atuais usam type_name canônico.
O grupo V19 é intencional por essa política comum de vocabulário, não por
supor que executar o braço cubra suas alternativas. Mesmo tipos já tratados
pela admissão externa pertencem à política completa do formatter; isso não
amplia a admissão numérica. Uma variante futura deve exigir classificação na
compilação deste consumer. Não adicionar allow/expect de V16 nem relaxar o
linter; a decisão cobrada passa a ser visível aqui e neste L0.

Nenhum contrato público, fase/default, coerção ou fórmula muda. Preservar
prioridade causal, kind de retorno, hints, spans e fronteira de extras. O RED
é estrutural: o match atual aceita variante futura sem revisão, e o desejado
é E0004; não inventar falha funcional para um refactor de política sem mudança
de runtime. Controle atual deve compilar. Testes funcionais de aliases,
nomes genéricos, erro antes de extras e valores dimensionais/numéricos são
controles de preservação, incluindo zero assinado e mixed length.

Aceitação: sonda isolada rustc com corpos produtivos extraídos e payloads
mínimos (sem usar target Cargo), controles funcionais, repetir os39 pares
separando os dois defeitos de literal, build/suíte/Clippy/Rustdoc/quatro índices,
fmt/diff, linhagem bidirecional e diferencial SARIF completo. Gate global
continua aberto pelos demais V16. Recursos: MemoryMax12GiB, High10GiB,
swap zero, Cargo jobs1, comandos pesados sequenciais. TS1/RG1 não entram
neste reparo e seus manifestos/candidatos/fontes protegidos são preservados.
