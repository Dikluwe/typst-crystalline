# L0 — compiler/stdlib/state/resolve
Hash do Código: 9cfe05a8

Camada L1. Consumer produtivo único: 01_core/src/compiler/stdlib/state/resolve.rs.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:8de2c32fbeeee7a30088b4ff3808e15a211f23a182673ef5cdebe54bd3eadad6
- 00_nucleo/prompts/_nuclei/state/language-semantics.toml sha256:27acb21a5e0b2e0cb3b65de61bba5266158f3e9828392fe92a1a3b160e9d61a4


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

Resolver privadamente uma sequência registrada, com init pertencente ao leitor.
Aplicar Set por valor e Func com argumento único corrente, captura léxica e
Scopes local. Não herdar in_context/current_location do leitor no callback;
restaurar ambos mesmo em erro. Não executar Func na construção/walk/layout.
Resultados/erros são SourceResult<Value>; não usar fallback sólido, None ou
Empty para esconder erro do update. A rota moderna observa o resultado completo.

Guardar somente o valor selecionado e o acumulador necessário, não toda a
sequência de Values se não houver necessidade semântica. Ações compartilhadas
não copiam init nem payload Set profundamente por snapshot. Sem cache global,
I/O, dispatcher dinâmico, mudança de propriedade dos campos ou nova API pública.
Não prometer número global de chamadas: cada leitura legítima pode recalcular;
replay não cria corpos/callbacks de display ou leituras novas por instrumentação.

R1–R12 e controles de ordem/prefixo/erros devem refutar a implementação antes de
aceitá-la. RAM12GiB/High8GiB/swap0/jobs1; validar linguagem, não bytes/algoritmo.

### Contraprova de erro futuro — decisão interna de paridade

target-controls.json.gz registra state-future-error: get anterior a um update
panic termina em erro FUTURE no vanilla pinado, sucesso no Tekt. Upstream
StateAtIntrospection (state.rs:419–437) calcula sequence inteira antes de
selecionar o offset. O valor é do prefixo, mas erro de callback posterior na
mesma sequência também é observável. Não inferir que outras keys executem.
Resolver a sequência completa da key demandada, conservando apenas o valor do
prefixo/final selecionado e propagando qualquer erro; sem leitura não executar.
Isto corrige a formulação abreviada "até o limite" da revisão A1-S2 quanto aos
erros: assinaturas e fase aprovadas permanecem. Não copiar o vetor de todos os
estados intermediários do vanilla. Cow<Value> pode emprestar Set/init e só mover
ou clonar o Value quando necessário para argumento de Func/resultado escolhido.

Preparar a leitura legada em um plano privado contendo somente ações, init
compartilhado e limite; extrair isso antes de emprestar EvalContext mutavelmente.
Não clonar todo StateRegistry para contornar o borrow. Sem ações registradas,
o plano conserva o lookup materializado legado usado pelas fixtures/API Rust.


### Verificação local do plano de leitura — V2

Medição anterior: complete-architecture.log preservado em
pre-repair-validation.json.gz acusa V2 neste consumer: testes funcionais de State
existem nos callers, mas falta verificação no owner do plano privado. O teste
local deve cobrir captura por key, limite causal com IDs não monotônicos,
manutenção de ações futuras da key para propagação de erro e compartilhamento
do payload Set sem clone profundo. Sem ações, validar o fallback materializado
legado no prefixo/final e key ausente. São invariantes do plano, não um mock do
resolvedor nem mudança de contrato. A falha RED é o gate V2; adicionar o teste
local e exigir V2 zero, conservando os controles funcionais RED→GREEN já medidos.
