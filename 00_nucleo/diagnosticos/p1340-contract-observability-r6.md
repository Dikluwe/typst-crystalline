# P1340 — contrato observável focal R6

**Estado:** `NÃO_SEALED`  
**Regime:** `executado sem atestação de isolamento`  
**Autor:** `/root/p1340_contract_r6`  
**Escopo:** somente as pendências R4 de lifecycle/profile; o fragmento terminal
R5 permanece protegido e é um conjuncto externo inalterado.

## Medição antes da decisão

O pause R1 mede três faltas: a asserção de features confundia perfil pedido
com recursos reais; o evento não conservava identidade/origem dinâmica da
`Func`; `CounterEvent` não tinha vínculo à ocorrência e origem lexical. O
inventário R4 mede pontos que ainda possuem esses fatos, mas não concede nem
prova o binding. O contrato R4 também demonstra que Dict não tagueado colide
com carriers tipados. Os L0 atuais exigem recursos causais reais, retenção da
mesma identidade de função quando preservada, registro completo e `Unknown`
bloqueante.

Classificação: features, identidade de função, origem da invocação, ocorrência
e Span diagnóstico são observáveis causais do fragmento de linguagem. Hash e
linha da fonte Rust são metadata de recibo, não campos semânticos do DTO.
Inferência: um binding cfg-only nos pontos inventariados pode fornecer os
ledgers abaixo sem mudar API/default/fase. Refutam-na falta de qualquer fato em
todos os callsites permitidos ou necessidade de contrato público; nesses casos
o resultado é `Unknown`/paragem, nunca `Preserved`.

## Contrato executável

Entrypoint:
`00_nucleo/diagnosticos/p1340-contract-predicate-r6.py`.

O predicado é conjuntivo com `p1340-contract-base-r4.py` e acrescenta:

1. `feature_contexts`: cada contexto observado declara origem real
   `eval_context_default`, `entrypoint_supplied` ou `inherited`. Corpo e request
   referenciam o mesmo contexto; nenhuma regra usa perfil pedido ou bit
   `selected` para inventar features.
2. `func_identities` e `function_dispatches`: identidade/carrier/produtor da
   `Func` ficam separados do callsite causal. Cada `BodyStarted` liga-se
   bijetivamente a um dispatch `context_body_entry`; esse dispatch nunca gera
   `FunctionInvoked`. Somente dispatches `callback` geram esse evento, incluindo
   wrapper e target reais de `With`. Fase de comparação não pode executar Func.
3. `counter_origins`, `counter_occurrences` e `content_lineage`: cada evento da
   árvore atual liga-se bijetivamente a snapshot/índice/Location/produtor,
   conteúdo caminhado e origem lexical integral. Span de root/read/consumer ou
   igualdade de ação não substitui a origem.
4. `typed_dict_values`: todo Dict usa exatamente
   `{ "Dict": [[chave_string, valor_tipado], ...] }`, recursivamente e em ordem.
   Func/Length/Dict internos não podem colidir com a variante Dict exterior.

`causal_witnesses` exige `kind`, `observation: actual` e `binding_point`; não
exige hash/linha no DTO. A auditoria independente deve piná-los fora do DTO e
provar completude contra os callsites reais. Declaração passiva não é prova.

## Política rigorosa de `Unknown`

Somente `status: opaque` explícito num ledger bem formado produz `Unknown`.
Campo ausente, variante desconhecida, inconsistência, lista incompleta ou
projeção não bijetiva produz `Violated`. Qualquer `Unknown` numa célula
obrigatória bloqueia selo e aceitação. Os casos opacos sintéticos servem apenas
para provar essa polaridade; não contam como sucesso de produto.

## Herança e terminal R5

Todas as obrigações públicas/lifecycle R4/R3 continuam pelo predicado base.
O veredito terminal R5 de SHA-256
`4a8533a98129d2c527e18ef9075535ba7ece8ccf498291a488d16f8634196683`
permanece imutável; R6 não o reexecuta, reinterpreta ou amplia. Um verificador
futuro combina ambos os fragmentos e executa NT01–NT06, sem inferir fechamento
terminal a partir destes focais.

## Limites antes de qualquer selo

- os focais usam somente fixtures sintéticas e não auditam binding produtivo;
- completude de callsites/Dict continua a exigir auditoria independente;
- o predecessor de 144 células não foi executado nesta autoria;
- ataques NT01–NT06, mutações de branches reais, repetição/reordenação/race,
  build, testes, V15/V26 e lint pertencem aos papéis seguintes;
- qualquer necessidade de API/default/fase/compatibilidade reabre ADR-0127;
- este artefato não é selo, veredito de produto ou alegação de equivalência.
