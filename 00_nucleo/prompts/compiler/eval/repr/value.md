# L0 — compiler/eval/repr/value
Hash do Código: 98cd4a84

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/repr/value.rs`. No HEAD `6b147bd213e07e1dc896a8e798b84bf4d8dbbdcb` e árvore limpa, `01_core/src/compiler/eval/repr.rs:28-201 e 1516-1573` possui o comportamento separado. SHA-256, horário e estado estão em `00_nucleo/diagnosticos/atomizacao-repr-2026-09-30-before.json.gz`. O objetivo é pesquisa algorítmica por comportamento, sem ganho de desempenho presumido.

## Contrato

Formatar cada variante de Value em morfologia pública determinística. Preservar arrays limitados a 40 itens apenas no repr, dicionário vazio (:), Args causais, floats não finitos, DateTime, closures e snapshots de LocatedContent sem reconstruir campos por estilo.

O contrato detalhado e as evidências históricas de paridade continuam em `00_nucleo/prompts/compiler/eval/repr.md`; a extração não muda saída, assinatura pública ou fase. Não há I/O nem estado global mutável.

## Aceitação

Preservar testes de morfologia de linguagem, compilar e passar `crystalline-lint .` com owner e hash válidos.
