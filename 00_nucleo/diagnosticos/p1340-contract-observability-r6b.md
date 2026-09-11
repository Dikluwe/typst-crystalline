# P1340 — sucessor contratual R6b

**Estado:** `NÃO_SEALED`  
**Regime:** `executado sem atestação de isolamento`  
**Revisão:** 2/2; nenhuma revisão automática restante.

## Medição e causa única

Os findings independentes R6-INHERITANCE-001, R6-UNKNOWN-001 e
R6-CAUSAL-COVERAGE-001 medem três faces da composição executável R6: o
entrypoint pulava checks de cenário do predecessor; `opaque` podia ocultar
ledger malformado; e listas esperadas no próprio DTO aceitavam omissões
coerentes. Todos são corrigidos nesta única revisão pré-publicação R6b.

## Composição executável

`p1340-contract-predicate-r6b.py` executa, nesta ordem:

1. `p1340-contract-predicate-r4.py::check`, que já contém os checks integrais
   de cenário, sinks, história e requests e internamente herda R4-base/R3;
2. `p1340-contract-observability-r6b.py::classify`, sem reimplementar aqueles
   checks;
3. o fragmento semântico R6 congelado, precedido por preflight estrutural e
   seguido pela cobertura contra expectations externas congeladas.

O CLI exige `--fixtures`, `--observations` e `--r6-expectations`. O terceiro
arquivo deve conter exatamente uma entrada por caso. Cada `expectations`
declara `status: frozen`, `authority_sha256` e os universos externos de
`callback_dispatches`, `dict_values` e `counter_events`. O manifesto/selo
futuro deve pinar esse arquivo; o DTO observado não pode alterá-lo.

## Unknown estrito

Antes de ler qualquer `status`, R6b valida presença/tipo de todos os campos
obrigatórios do caso, expectations, witnesses e ledgers de features, Func,
dispatch, origins, occurrences, lineage e Dict. `malformed + opaque` é
`Violated`. Só um ledger estruturalmente completo pode alcançar o `Unknown`
explícito do R6; qualquer `Unknown` obrigatório continua bloqueante.

## Cobertura externa

Depois da coerência interna R6, R6b compara os callbacks, Dicts e eventos de
counter reais com os universos congelados por caso. A remoção conjunta de
evento+ledger+lista autodeclarada continua `Violated`, porque não pode remover
a expectativa externa. Hash/linha Rust não entram no DTO semântico; o pin da
autoridade externa fica no arquivo de expectations e no futuro manifesto.

## Delta focal e limites

- herança: 4 mutantes antes `Preserved` → 4 `Violated` pela lógica real do
  predicado R4;
- política/cobertura: 5 `malformed+opaque` antes `Unknown` → 5 `Violated`;
  3 omissões coerentes antes `Preserved` → 3 `Violated`;
- sintaxe: quatro scripts R6b analisados; zero produto/candidato executado.

R6b permanece `NÃO_SEALED` porque as expectations externas das 144 células
reais ainda não foram autoradas/congeladas por autoridade independente. Não
há budget para revisão 3: qualquer nova classe de falha exige parar. Terminal
R5 permanece pinado e imutável; NT01–NT06, binding real, matriz, gates finais,
selo e veredito pertencem aos papéis seguintes.
