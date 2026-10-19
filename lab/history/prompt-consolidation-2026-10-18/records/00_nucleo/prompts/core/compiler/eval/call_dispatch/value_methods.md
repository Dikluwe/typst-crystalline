# L0 — compiler/eval/call_dispatch/value_methods
Hash do Código: c1ef8f77

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/call_dispatch/value_methods.rs`. No HEAD `d5c563dfab39f789e8e2f72c9a9364f5a761e3ca` e árvore limpa, `01_core/src/compiler/eval/call_dispatch.rs:722-1214` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `lab/results/construction/diagnostics/atomizacao-call-dispatch-2026-09-30-before.json.gz`.

## Contrato

Despachar métodos estáticos de valores e validar receiver, argumentos e formas especiais. Preservar assinaturas públicas, diagnósticos, prioridades de intercepção, rastros e semântica de argumentos especificados em `00_nucleo/prompts/core/compiler/eval/call_dispatch.md`. A extração permanece em L1 com despacho estático, sem I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1, os testes de call_dispatch e `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho.


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

Migrar somente o despacho de State: update usa o adapter interno Func/Set,
get/final/at encaminham o Engine já disponível. A rota capturada como valor
mantém a mesma semântica da chamada direta, sem reconhecimento por nome.
Preservar todos os outros receivers, validação e diagnósticos existentes.

### Aceitação e fronteiras

Gate A1-S2 ratificado para as cinco assinaturas e a fase de leitura. L0 primeiro,
RED funcional antes do código; GREEN contra o pin indicado para linguagem e
controles próprios das extensões Tekt. Não ampliar outros contratos públicos.
Build/workspace/doctests, Clippy/Rustdoc estritos, fmt, linhagem V1/V5/V15/V26 e
reversa; manter V16 sem aumento. MemoryMax12GiB/High8GiB/swap0/jobs1, tarefas
pesadas sequenciais. Medir consumo sem inferir ganho global só da representação.
