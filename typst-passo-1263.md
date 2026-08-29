# P1263 — corrigir máximos interiores não saturados de cor e alpha

**Estado:** EXECUTADO — DEZ FRONTEIRAS FECHADAS, PROMOÇÃO NÃO APLICADA  
**Predecessores:** P1259 e P1260  
**Owner inicial:** `infra/export/gradients/adaptive`.

## Escopo congelado

- quatro `color_max` interiores e não saturados;
- seis `alpha_max` interiores e não saturados;
- métricas já dentro do envelope permanecem controles negativos e não podem
  justificar stops adicionais.

## Contrato RED

1. congelar `worst_t`, intervalo, endpoints, cor e alpha exatos;
2. reproduzir cor premultiplicada e alpha como observáveis independentes;
3. medir a decisão midpoint vigente no segmento que contém o máximo;
4. testar refinamento guiado pelo erro causal, não uma lista fixa de quartos;
5. publicar stops/decisões adicionais por fixture e limitar o custo;
6. exigir que os seis alpha máximos fechem sem piorar cor e vice-versa.

## Mutantes

- ignorar alpha na decisão;
- usar alpha apenas dentro do RGB premultiplicado;
- comparar alpha depois de u8;
- adicionar quartos globalmente (mutante já refutado pelo P1259);
- escolher pontos a partir do campo de aprovação candidato;
- usar máximo sem p95 ou p95 sem máximo;
- compartilhar decisão Linear/Radial sem execução individual;
- remover stops coincidentes ou alterar right-continuity.

Somente mutantes executáveis contam; selo exige `mutation_score=1.0`.

## Implementação condicionada

Atualizar `adaptive.md` antes de código se o contrato exigir critério explícito
de alpha ou refinamento fora do midpoint. A alteração deve ser helper puro L3,
sem I/O, sem mudança no seletor PDF P274 e sem novo contrato público. O custo
adicional só ocorre no segmento cujo observável viola o envelope.

## Fechamento

- dez fronteiras não saturadas dentro de máximos/p95 congelados;
- nenhum controle preservado regride;
- determinismo direto/inverso;
- cap, custo e número de decisões publicados;
- reexecução P1237 individual, sem promoção cruzada.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO` se aplicável.

## Resultado

Implementação e recibos: `00_nucleo/diagnosticos/typst-p1263-nonsaturated-interior.md`.
Veredito: **EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO**.
