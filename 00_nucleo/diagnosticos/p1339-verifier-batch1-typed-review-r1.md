# P1339 — revisão das definições tipadas do lote 1

Verificador `/root/p1311_review`, 2026-09-10. Executado sem atestação de
isolamento. Nenhum programa Typst, comparador futuro ou matriz foi executado;
nenhum mock JSON foi usado como evidência. Nenhum input julgado foi editado.
Esta é auditoria estática anterior ao manifesto do lote, não selo/RED/GREEN.

Manifesto r2 `842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b`;
contrato r3 `c0cd1826679ddaf77e937a677839c5cbe64fdae6ec71bfa34e635d584d9c3e17`;
budget e limites mantidos conforme suplemento
`4bdd984d7b9d482d68ebb787e98eeed8333a1d4447c637615d4ceb9c78ed6c19`.

## Entradas auditadas e pinadas

- Retention-lifecycle-typed:
  `fb0cd01e603fef04460e0f06605dde367fab60c92407d8f9ec9da9f7aa0cdf85`.
- Lifecycle-predicate.py:
  `8cae4f1719a9310d4e627bc85bfe5d2589bb411af1f1527d8a872e2a7a94ed14`.
- Same-context-style-fixture:
  `70d572c86aee6499c0f5ebf20849187d4113ebc6016888f1e0b4a0ea43d572aa`.
- Inventory-resolution:
  `25d0e5e8cddae2cb0b377cd0f65fe52765b3de4765d9b54734db97492187178a`.
- Public-opaque-harness:
  `5bb5aba6ff59a8d90927a7b2ed5e68d0f978202d499294e16fbcf5e8d158a3b3`.

Nomes acima têm prefixo `00_nucleo/diagnosticos/p1339-ab-batch1-`, exceto
inventory-resolution e public-opaque-harness, cujo prefixo é
`00_nucleo/diagnosticos/p1339-mutant-closed-state-`.

## Gaps anteriores: definições agora adequadas, runtime ainda devido em F

O cenário same-context fixa um único EvalContext/Engine, duas demandas reais
com cadeias 10pt/20pt, deixando Engine em 30pt antes da validação. IDs de
request/context/chain e bits IEEE devem vir dos hooks reais de captura e
replay; não de labels do driver. Os predicados exigem duas capturas, dois
replays, mesma identidade ponto a ponto, resultado real Ok(true), seleção e
zero publicação do sink de validação. Isso discrimina usar a cadeia final
do mesmo corpo. Aceito a definição; tradutor e operação real ainda não foram
compilados/executados.

SW-F08-shared-pagination-budget define testemunha estrutural específica:
armazenamento do budget, todas as inicializações, arestas de retry, SCCs e
def-use D5/I4/história I0–I5. Não presume que o caso oscilante mediu
invalidação de paginação. É definição admissível do componente estrutural
conjuntivo; cada aresta/prova deve ser resolvida no candidato em F. Ausência
ou ambiguidade não vira aceite. SW-F09 também exige os caminhos completos de
sink/Err. As definições estruturais não dispensam transições reais obrigatórias
dos cenários executados.

## Bloqueios do checker tipado — ausência aceita por vacuidade

Li as 154 linhas integralmente e todos os typed_predicates dos 12 casos.
O checker exige eventos de tentativa e projeções em alguns cenários, mas
vários loops não exigem que os eventos obrigatórios existam:

1. Increasing/oscillating não exigem nenhum ValidationCompleted. O loop de
   validação vazio satisfaz a checagem apesar de faltar toda a evidência de
   replay/validação desses cenários.
2. Retained-sibling exige que o primeiro output/Func chegue ao final, mas
   não exige ContributionRetained em A2–A5. A ausência de todas as transições
   de retenção não gera falha.
3. Replaced-child exige BodyStarted/generation/producer distintos, porém não
   exige GenerationInvalidated nem o correspondente descarte de descendentes.
4. SinkDisposition vazio satisfaz todas as restrições dentro do loop de
   dispositions. Não prova descarte/publicação real ou completude dos canais.

São conclusões da leitura dos quantificadores, não execução de observações
fabricadas. Corrigir no mesmo lote com cardinalidade/presença/bijeções
causais por cenário, incluindo relação entre gravações, replays, retained e
invalidated. Declarar e verificar tipos/chaves obrigatórios/envelope e
identidade dos perfis/ordens na integração. Mapeamento completo dos antigos
predicados para typed ou SW é necessário: `predicate_precedence` não pode
transformar obrigação não traduzida em mera rationale descartável.

## Veto opaco não inteiramente traduzido

Li integralmente o harness público opaco. Ele constrói as mesmas instâncias
reais para o par privado e query pública, preserva diagnóstico separado e
não implementa uma relação alternativa. Porém `opaque_predicates` ignora a
lista `forbidden` de nonconvergence: qualquer Err com vetor não vazio é
aceito, inclusive um warning vanilla indevido empacotado em Err. Os sinks
de validação/diagnóstico são registrados, mas o veto a warning fabricado
não é verificado neles.

É necessário traduzir explicitamente esse veto nos canais aplicáveis ou
mapear uma testemunha estrutural conjuntiva que cubra exatamente essas
origens/propagações. Não ampliar normalização, converter erro em Unknown ou
alterar a regra semântica. O discriminante Unproven privado continua exigido.

## Inventário direto: resolução nominal aceita, mapa semântico pendente

Conferidos os 136 registros diretos (38 Value, 98 Content) e 296 referências
de hash, sem falhas. Os payloads homônimos, incluindo HtmlElem, agora estão
presentes. As declarações externas distinguem wrapper Decimal/InnerDecimal
e wrapper Regex/motor externo/cache; não concedem equivalência por endereço
ou Eq genericamente. A importação real de InnerDecimal e o constructor
privado pattern/compiled do wrapper Regex foram inspecionados como contexto
de resolução, não como oráculo que suplante L0.

A resolução enumera dados e fronteiras; ainda não é o mapa obrigatório de
cada campo/variante para teste concreto ou testemunha estrutural normativa.
O mapa W01–W10/F01–F10 e a integração dos checkers continuam pendentes.
Nenhum número de inventário é usado como quantidade de testes aprovados.

As insuficiências acima pertencem à lista já aberta de tradução de
predicados/DTOs/opacidade do lote 1 e foram relatadas antes do manifesto.
Continuam zero novas execuções focais e zero matrizes completas nesta revisão.
