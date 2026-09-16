# Prompt L0 — testes independentes dos constructors HTML P1423
Hash do Código: 900e0f1b

**Camada:** L1 (testes)  
**Ficheiro alvo:** `01_core/src/compiler/stdlib/html/p1423_tests.rs`  
**Owner produtivo observado:** `00_nucleo/prompts/compiler/stdlib/html.md`

## Obrigação

Este consumer é exclusivamente de teste e materializa o contrato A/B selado
do P1423 sem possuir código produtivo. Deve provar:

- superfície exata de 43 bindings, sem `html.frame`, preservando os 69
  anteriores;
- partição disjunta de body em 10 void, 2 raw e 31 content;
- omissão/body fornecido e rejeições de tipo para as três classes;
- casts representativos de globais, específicos, Presence, limites, enums,
  listas e uniões, inclusive rejeição de atributo cruzado;
- construção L1 independente do target, mantendo o gate de feature no acesso
  público ao módulo.

O teste importa o módulo pai apenas sob `cfg(test)`. Não redefine contrato de
produto, não legitima implementação produtiva e não amplia o escopo para
`html.frame`, CSS, MathML ou comportamento de browser.

## Proveniência

As expectativas derivam do L0 proprietário confirmado em 2026-09-16 e dos
artefatos selados `p1423-contract.json`, `p1423-oracles.json` e
`p1423-mutants.json`. Alterar fixtures ou asserções exige novo resselo; ajustes
somente de cabeçalho/linhagem não mudam a substância do teste.
