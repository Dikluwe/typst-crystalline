# L0 — compiler/eval/repr/math
Hash do Código: b3a664ff

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/repr/math.rs`. No HEAD `6b147bd213e07e1dc896a8e798b84bf4d8dbbdcb` e árvore limpa, `01_core/src/compiler/eval/repr.rs:571-672 e 1374-1466` possui o comportamento separado. SHA-256, horário e estado estão em `00_nucleo/diagnosticos/atomizacao-repr-2026-09-30-before.json.gz`. O objetivo é pesquisa algorítmica por comportamento, sem ganho de desempenho presumido.

## Contrato

Formatar campos matemáticos auxiliares, Attach, Binom canônico e matriz com identidade, ordem e presença causal. Não inventar fields a partir de geometria derivada nem trocar um construtor matemático por outro.

O contrato detalhado e as evidências históricas de paridade continuam em `00_nucleo/prompts/compiler/eval/repr.md`; a extração não muda saída, assinatura pública ou fase. Não há I/O nem estado global mutável.

## Aceitação

Preservar testes de morfologia de linguagem, compilar e passar `crystalline-lint .` com owner e hash válidos.
