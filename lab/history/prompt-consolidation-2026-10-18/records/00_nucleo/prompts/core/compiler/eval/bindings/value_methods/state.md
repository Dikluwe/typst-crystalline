# L0 — compiler/eval/bindings/value_methods/state
Hash do Código: 64d3a70c

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/bindings/value_methods/state.rs`. Extração interna medida no HEAD `ce27af795` e árvore limpa em 2026-09-30 23:12:44 UTC. A fonte é o consumer pai desta unidade; a extração não é medição bilateral de paridade nem prova de desempenho.

A lógica extraída estava em `01_core/src/compiler/eval/bindings/value_methods.rs:42-176` and `:655-682` no HEAD indicado. A classificação é extração interna de comportamento, não nova decisão de paridade.

## Contrato

Own State method evaluation, at-location/label resolution and context observations. Preserve validation order, arguments, spans and contextual behavior. Não alterar contrato público, fase do pipeline, comportamento por defeito ou compatibilidade.

## Aceitação

Os testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho da divisão estrutural.


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

Projetar update funcional pelo adapter state_method_update sem executá-lo.
Get/at/final passam Engine e contexto mutável para os helpers aprovados.
Preservar validação antes de contexto, resolução/observação de labels, spans e
captura. Rejeitar argumentos excedentes/nomeados onde a linguagem exige aridade
exata; não permitir que o caminho AST aceite o que o método capturado rejeita.

### Aceitação e fronteiras

Gate A1-S2 ratificado para as cinco assinaturas e a fase de leitura. L0 primeiro,
RED funcional antes do código; GREEN contra o pin indicado para linguagem e
controles próprios das extensões Tekt. Não ampliar outros contratos públicos.
Build/workspace/doctests, Clippy/Rustdoc estritos, fmt, linhagem V1/V5/V15/V26 e
reversa; manter V16 sem aumento. MemoryMax12GiB/High8GiB/swap0/jobs1, tarefas
pesadas sequenciais. Medir consumo sem inferir ganho global só da representação.
