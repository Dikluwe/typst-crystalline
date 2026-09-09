# Passo 1329 — calc.abs para valores dimensionais

## Problema medido

P1328 corrigiu o erro de conteúdo, mas abs ainda rejeita length, angle,
ratio e fraction. A medição nova está em
`00_nucleo/diagnosticos/p1329-baseline.json` (SHA-256
`d0e1787fac8b6264122ca6dcf5e29e4729552e8031e591ce6f4ee725e14cbd23`),
sobre HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093` e working tree
P1328 não commitado, com diff/stat/inventários e UTC integral.
Referência: upstream/main ratificado a51e02804, binário pinado no recibo.

## Entrega pretendida

`calc.abs(-2pt)`, `calc.abs(-2em)`, `calc.abs(-2deg)`, `calc.abs(-2%)` e
`calc.abs(-2fr)` devem devolver o módulo conservando seu tipo da linguagem.
Comprimentos com abs e em não zero devem continuar rejeitados, mas com a
mensagem e origem corretas. A referência também rejeita os mistos de
mesmo sinal: não usar uma regra intuitiva diferente.

O L0 legitimador é `00_nucleo/prompts/compiler/stdlib/calc.md`, seção
P1329. Este passo não é L0. Owner único `01_core/src/compiler/stdlib/calc.rs`.
Não mudar entidades, operadores, dispatcher, aridade, named, overflow
inteiro, outras funções calc ou os diagnósticos Content P1328.

## Execução e gates

1. Atualizar L0 antes do código, explicitando sucessão das quatro rejeições
   históricas, condição dos comprimentos, IEEE e limites. Congelar norma,
   manifesto, baseline e política dos traces externos antes do candidato.
2. Regime A/B da skill tekt-materializacao-segregada: root escreve L0 e
   candidato, autor fresco escreve testes sem runtime/patch e revisor
   somente-leitura emite veredito. Ambiente compartilhado, sem atestação
   técnica de isolamento ou selo de refinamento.
3. Autor A/B migra somente quatro controles P1328 para resultados positivos
   e adiciona cobertura independente de sinais/unidades/zeros/mistos,
   origens reais/sintéticas, With/spread/UTF-8, tipos, NaN/Inf nativos,
   warnings, math e quatro perfis. Evidências P1328 não são reescritas.
4. Integrar bytes congelados, provar RED sem falha de compilação, aplicar
   correção somente no owner e provar GREEN dos mesmos testes.
5. Build/workspace/fmt/diff/lint e V5/V15/V26; comparação CLI completa
   normal/repetida/invertida; reexecutar corpus P1328 com expectativas
   sucessoras que mudem somente os quatro casos dimensionais medidos.
6. Relatório em diagnosticos deve mostrar ganho concreto, dívidas abertas,
   revisão de expectativas históricas e proveniência. Nenhum Unknown
   obrigatório pode virar sucesso. Duas revisões sem ganho da mesma causa
   exigem revisar método/escopo antes de continuar.

Correção de paridade ADR-0127 em fluxo contínuo. Se for necessário ampliar
owner/API/fase ou mudar interpretação, parar antes dessa ampliação.
Temporários em `/tmp/p1329-target.bg3p5A`, cópia sem hardlinks; RAM do host
medida com 3.3G livres para cache anterior de 3.8G. Não remover targets
anteriores. Preservar alterações preexistentes. Sem stage, commit, push
ou criação de passo subsequente nesta execução.
