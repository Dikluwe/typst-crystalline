# P1269 — reexecutar a implementação vigente contra o contrato corrigido

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


**Estado:** PLANEADO
**Predecessor:** preseal P1268
**Regime:** diagnóstico segregado; nenhuma correção produtiva

## Objetivo

Medir novamente os quatro pares sem os falsos negativos mecânicos do P1266 e
produzir a lista mínima de divergências reais que ainda bloqueiam a conjunção.

## Execução

1. Verificar os hashes do preseal antes da primeira execução candidata.
2. Executar as mesmas 96 fixtures, 28 probes inválidos e controlos congelados.
3. Publicar por fixture G01–G13, custo por intervalo e total apenas como recibo,
   sem transformar contagem literal em paridade.
4. Exigir preservação dos stops originais, descontinuidades, alpha, geometria,
   anti-alias, grafo, raster e domínio.
5. Aplicar os quatro budgets numéricos vanilla-first na malha congelada.
6. Executar direto, inverso, repetido e toda a suíte de mutantes P1268.

## Resultado permitido

- Se cada par fechar 24/24, emitir `Generalization-Preserved` e seguir ao P1273.
- Se restarem apenas falhas numéricas, congelar exatamente as fixtures e seguir
  ao P1270.
- Qualquer falha de contrato, grafo, domínio ou determinismo volta ao owner
  correspondente; não é absorvida pelo passo numérico.

## Trava

Não editar budgets, corpus, L0 ou produto após observar o candidato. Este passo
mede a dívida; não escolhe a solução.
