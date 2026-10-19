# L0 — compiler/stdlib/registry/contextual
Hash do Código: ca2c18a3

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/contextual.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Register metadata, state, counter, context, state/counter operations, query, selector, here, target and locate. State/counter/selector remain Type values; native ABIs remain unchanged. Do not decide targets or execute contextual reads.

Preserve the owned public names, Value kinds, native bindings, children and
aliases. Do not implement Args validation, casts, export or lookup behavior;
those operations are outside this registration consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.


## A1-S2 — leitura por objeto aprovada em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
UTC/status/diffstat/fontes em before.json.gz e final-freeze.json.gz de
lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17.
state-counterproof.json.gz registra RED bilateral: init 1/10 da mesma chave
após +1 exige 2/11; callback não lido não executa; atualização reutilizada
executa por ocorrência. Pin bd5ebec285505889e8ff1d4f68804ab147a699f3,
upstream introspection/state.rs:226–245, 419–455 e 477–520. O init do leitor
inicia a sequência; a ordem vem do documento, não do inteiro de Location.
O método atual trata Func como Set e o pós-passe eager perde essa semântica.
Estes são valores/erros de linguagem, não igualdade Rust ou algoritmo exigido.
A1-S1 foi retirada; gate A1-S2 confirmado pelo dono nesta sessão, revisão
lab/history/construction/records/00_nucleo/materialization/state-reader-runtime-2026-10-17/review.md,
SHA-256 6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4.

### Responsabilidade exclusiva — supersede a fase histórica de State

Registrar somente native_state_final/native_state_at via Func::native_with_engine,
com mesmo nome público e Value::Func. As demais nativas conservam ABI/bindings.
Não executar reads nesta composição. O ABI NativeFuncWithEngine já existe;
esta cláusula ratificada supersede a exigência anterior de ABI intacto destas duas.

### Aceitação e fronteiras

Gate A1-S2 ratificado para as cinco assinaturas e a fase de leitura. L0 primeiro,
RED funcional antes do código; GREEN contra o pin indicado para linguagem e
controles próprios das extensões Tekt. Não ampliar outros contratos públicos.
Build/workspace/doctests, Clippy/Rustdoc estritos, fmt, linhagem V1/V5/V15/V26 e
reversa; manter V16 sem aumento. MemoryMax12GiB/High8GiB/swap0/jobs1, tarefas
pesadas sequenciais. Medir consumo sem inferir ganho global só da representação.
