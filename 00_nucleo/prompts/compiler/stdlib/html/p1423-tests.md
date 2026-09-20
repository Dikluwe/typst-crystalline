# Prompt L0 — testes independentes dos constructors HTML P1423
Hash do Código: 900e0f1b

**Camada:** L1 (testes)  
**Ficheiro alvo:** `01_core/src/compiler/stdlib/html/p1423_tests.rs`  
**Owner produtivo observado:** `00_nucleo/prompts/compiler/stdlib/html.md`

## Obrigação

Este consumer é exclusivamente de teste e materializa o contrato A/B validado
do P1423 sem possuir código produtivo. Deve provar:

- conjunto exato dos 43 bindings P1423, disjunto dos 69 anteriores;
- módulo corrente com `elem`, os 112 constructors tipados e `frame` de P1424;
- partição disjunta de body em 10 void, 2 raw e 31 content;
- omissão/body fornecido e rejeições de tipo para as três classes;
- casts representativos de globais, específicos, Presence, limites, enums,
  listas e uniões, inclusive rejeição de atributo cruzado;
- construção L1 independente do target, mantendo o gate de feature no acesso
  público ao módulo.

O teste importa o módulo pai apenas sob `cfg(test)`. Não redefine contrato de
produto, não legitima implementação produtiva e não amplia o escopo para
`html.frame`, CSS, MathML ou comportamento de browser.

`frame` não pertence ao conjunto P1423, mas pertence ao módulo atual por
P1424. O teste deve provar simultaneamente essas duas afirmações e nunca voltar
a exigir ausência global de `html.frame`.

## Proveniência

As expectativas derivam do L0 proprietário e do comportamento público acima.
Alterações nas fixtures devem continuar a provar essas invariantes, sem catálogos
de mutantes, autoria, ordem de agentes ou hashes congelados da implementação.
