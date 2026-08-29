# P1266 — ampliação do envelope SVG Oklab/LinearRgb

**Estado:** EXECUTADO — quatro pares `Unknown-generalization`  
**Regime:** protocolo Tekt completo, executado sem atestação de isolamento

## Resultado antes da decisão

Foram materializadas 96 fixtures válidas, 24 por par. O oráculo vanilla foi
executado primeiro e congelou source, grafo, máscaras, quatro budgets em malha
8192 e raster local. Só depois foi executado o adaptador diagnóstico, que usa o
`adaptive.rs` vigente por um probe em `lab/`. O binário produtivo foi medido
separadamente: 96/96 documentos conservaram o marcador
`gradient-color-space`, sem servidor Linear/Radial candidato e sem promoção.

O resultado agregado foi:

- grafo do adaptador: 96/96;
- superfície pública: 22/96;
- budgets numéricos: 72/96;
- raster local: 96/96;
- área zero: 92/96;
- cap global de 64 stops: 60/96;
- conjunção completa: 22/96.

Pair-localmente, Linear/Oklab e Radial/Oklab fecharam apenas 5/24; os dois
pares LinearRgb fecharam 6/24. Por isso os quatro vereditos são
`Unknown-generalization`; o total agregado não promove nenhum par.

## Bloqueios observados

A superfície pública divergiu principalmente na morfologia dos offsets
efetivos: a referência conserva valores de razão com precisão superior em
casos desiguais/concentrados, enquanto o carrier cristalino público passa por
`f32`. A maior diferença de componente amostrado foi
`0.00011473894119262695`, em S23 Radial/LinearRgb. Esta medição não atribui por
si só uma correção; apenas refuta a generalização do fragmento P1237.

O adaptador excedeu o cap global em 36 fixtures. O máximo foi 774 stops em S06
Linear/Radial × LinearRgb, muito acima de 64. O cap interno atual é por
intervalo, portanto não constitui um cap global quando há muitos stops; P1266
não autoriza mudar essa política.

As quatro S20 usam uma caixa de largura zero com stroke de 8pt. O stroke ainda
tem área pintada positiva, logo a obrigação congelada de área zero reprova as
quatro instâncias. S21, fill com altura zero, passou. Isto expõe uma tensão no
desenho do contrato/corpus, não uma licença para converter a falha em sucesso.

Na materialização, os seeds coincidentes S07–S09 duplicam o carrier necessário
para preservar simultaneamente a descontinuidade e endpoints 0/100%; a coluna
`stops` da matriz é tratada como cardinalidade lógica de cores. A geometria
`large-radius-center-outside-box` usa o máximo público válido de 100% com
centro fora da caixa; 150% seria rejeitado pelo domínio cristalino e não
poderia integrar a população válida.

## Domínio, determinismo e ataques

Os 28 probes aplicáveis de I01–I08 foram rejeitados pelos dois binários e
classificados `Rejected-by-domain`, nunca `Unknown`. A execução inversa e a
repetição produziram 192/192 recibos semanticamente idênticos ao percurso
direto.

Os 24 mutantes foram executados e rejeitados. O `mutation_score` é 1.0, com
zero `Unknown` no numerador. Isso prova o poder discriminatório do contrato
para este envelope, não equivalência SVG geral nem aptidão para promoção.

## Gates e limite de atestação

Passaram 6.332 testes do workspace, build, formatos root/probe, sintaxe Python,
`git diff --check`, V1/V5/V15/V26 e o lint completo com zero violações. Os
hashes de L0 e código de SVG/adaptive permanecem iguais aos certificados
P1264/P1265. Os controlos P1234/P1236/P1264 ficam preservados por identidade
dos inputs protegidos e pela revalidação integral do workspace.

Contrato, oráculos, adaptador, ataques e veredito foram exercidos pela mesma
autoridade `/root`. A ordem vanilla-first é reproduzível, mas não substitui
isolamento de capacidades. Veredito proporcional:
**EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO**.

Nenhuma promoção produtiva é autorizada; `paint_is_svg_native` e o fallback
continuam inalterados. Um passo futuro deve primeiro corrigir ou restringir o
envelope refutado e só poderá voltar ao dono após nova evidência completa.
