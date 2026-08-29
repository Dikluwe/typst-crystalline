# P1267 — readjudicação do contrato de generalização SVG

**Estado:** EXECUTADO — contrato v2 candidato, não selado

**Regime:** fase de autoria do protocolo Tekt completo, sem atestação de isolamento

**Escrita produtiva:** nenhuma

## Medição antes da decisão

O certificado final P1266 permanece byte a byte imutável e o seu SHA-256 é
`dce8aabec0d6bbf8b714d75f8cfc22e38f83539d04f1c8d034110c8dfa1067ec`.
Ele registra 74 falhas da conjunção, todas com G04, das quais 36 também
falharam o cap global inventado e quatro também falharam a obrigação de área
zero. Esses números pertencem ao estado não commitado já identificado pelo
snapshot P1266; não são uma nova execução nem autorizam reclassificação.

A fonte vigente contradiz as três cláusulas mecânicas. `svg.md:144-146` exige
offsets **originais**, ordem, descontinuidades, alpha e orçamento, não igualdade
de todas as posições introduzidas pelo aproximador. `adaptive.md:53-76` põe o
máximo de 64 subdivisões dentro de cada intervalo e declara mecânica a contagem
literal de stops. `p1266-generalization-matrix.tsv` distingue S20, stroke de
largura positiva sobre caixa de largura zero, de S21, fill sobre caixa de altura
zero.

No algoritmo ratificado (`gradient.rs:905-965`) e no owner cristalino vigente
(`adaptive.rs:115-164`), cada intervalo crescente começa com uma subdivisão e
dobra somente até 64. A árvore tem profundidade máxima 6, no máximo 63 stops
introduzidos, 64 segmentos e 127 avaliações de midpoint. Com `n` stops
originais e `p` intervalos crescentes, `p <= n-1`, a emissão é limitada por
`n + 63p`. Intervalo coincidente não introduz stop adaptativo. Logo o cap global
64 era falso, mas a terminação e a ausência de expansão ilimitada continuam
publicáveis por intervalo.

## Decisão no nível da linguagem

G04 é dividido em duas obrigações. G04A protege a morfologia dos stops
originais: offset efetivo resolvido pela linguagem, ordem, coincidência,
continuidade à direita, cor e alpha. G04B protege o efeito dos stops adaptativos:
ordem monotónica dentro do intervalo original, suporte da curva e envelopes
cromático/alpha. Carrier `f32/f64`, ortografia e contagem literal são mecânica,
salvo quando alteram um observável nomeado.

G07 é dividido pelo papel pintado. S20 passa a exigir preservação da região
efetivamente pintada pelo stroke, da geometria, do papel e do grafo fechado;
área positiva não é falha. S21 conserva a obrigação de nenhum paint positivo
para fill sem área, também com geometria e grafo corretos.

G10 passa a observar os limites derivados por intervalo e a fórmula agregada,
não `emitted_stops <= 64` global. Aqui a mecânica de custo é ela própria o
observável operacional de terminação; ela não é usada como substituto da
paridade morfológica.

Todas as alterações são `CONTRACT-CORRECTION`. Nenhuma delas conclui que o
produto está preservado: uma violação de stop original, suporte/envelope,
geometria pintada ou limite por intervalo continua `PRODUCT-DIVERGENCE`; falta
de identidade, oracle, budget, suporte ou atribuição de custo continua
`UNKNOWN` e nunca conta como sucesso.

## Gate e entrega

O contrato v2 está `CANDIDATE-UNSEALED`. Não foram escritos oráculos, ataques,
selo, código, L0 ou testes produtivos. O P1268 deve congelar estas entradas,
autorar vanilla-first os oráculos e custos por intervalo, executar mutantes e
somente então decidir se o contrato possui poder discriminatório. Os recibos e
vereditos P1266 não foram reescritos.

A mesma autoridade `/root` leu as fontes, redigiu e verificou os artefatos desta
fase; portanto o resultado é **EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO**. A
separação preservada é causal e de escrita: esta sessão ocupou apenas autoria
do contrato, enquanto autoria de oráculos, adversário e verificador permanecem
não iniciados.
