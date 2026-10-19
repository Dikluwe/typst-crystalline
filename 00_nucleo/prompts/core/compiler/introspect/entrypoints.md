# L0 — introspect/entrypoints
Hash do Código: abbab208

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:8de2c32fbeeee7a30088b4ff3808e15a211f23a182673ef5cdebe54bd3eadad6

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/introspect/entrypoints.rs`.
No estado `df72a6c4a701917ab56a7c6169764a92ea775693` com árvore limpa, `01_core/src/compiler/introspect.rs:750-921` contém o comportamento que esta unidade recebe. A fonte e o estado estão registrados em `lab/results/construction/diagnostics/atomizacao-introspect-2026-09-30-before.json.gz`. A decisão é separá-lo por comportamento, sem alterar a API ou alegar ganho de desempenho.

## Contrato

Executar a passagem pura e as variantes de bundle/runtime com um Locator causal compartilhado, registrar sementes antes dos bodies, preparar outline e pais uma vez e coordenar os pós-processadores sem reavaliar o documento.

As invariantes de linguagem, a ordem causal e os limites de paridade continuam definidos em `00_nucleo/prompts/core/compiler/introspect.md`; este owner não cria uma segunda interpretação. O código usa chamadas estáticas na camada L1, sem vtable, I/O ou estado global mutável.

## Aceitação

Preservar os testes de introspecção e a suíte L1, compilar e passar `crystalline-lint .` com hashes coerentes. Mudanças semânticas futuras exigem medição de linguagem e atualização do L0 antes do código.


## Realização com avaliação única

Entry points reservam Locations explícitas de toda a entrada antes de novos elementos ordinários/contextuais, conservando disjunção e ordem de tags.

### Pertencimento de ocorrências em bundles

Os dois entrypoints de bundle registram privadamente no CounterRegistry as Locations dos Tag::Start produzidos por cada body. Checkpoints continuam servindo ao layout; não são limites exclusivos da identidade semântica. Bodies vazios registram um escopo vazio.

## A1 — callbacks de suplemento de Heading, 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c e working tree/UTC/hash
registrados em lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz.
before-pdf-probe.json.gz: heading-supplement-panic termina em sucesso no Tekt
e em A1_HEADING no vanilla bd5ebec285505889e8ff1d4f68804ab147a699f3.
heading-supplement-valid aceita ambos, mas só o vanilla usa Custom na referência.
Fonte upstream model/heading.rs:249-289 sintetiza supplement por ocorrência e
model/reference.rs:393-415 aplica a função e display. Set.rs:286-320 ignora
supplement e limpa numbering; snapshot_fields heading.rs:55-68 fixa tradução.
Isto é callback/valor observável, não exigência de mecânica upstream.

### Decisão e responsabilidade exclusiva

Em ambos os entrypoints runtime, chamar a finalização de Heading no mesmo
bloco pós-walk já dedicado a callbacks de equação/estado, antes de publicar intr.
Propagar erros. O entrypoint puro sem Engine permanece puro. Não reavaliar o
documento nem mudar a fase eval/layout ou a política de convergência.

Correção de paridade em fluxo ADR-0127: sem nova API Rust, campo público ou
transferência de lógica existente entre eval/layout. O pós-walk com Engine já
é a fronteira de aplicação de callbacks. L0 primeiro, RED e controles bilaterais,
sem callbacks em encoder/walk/layout e sem alegar paridade de todos os campos.


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

Retirar somente a chamada automática apply_state_funcs dos dois entrypoints
runtime. Manter registro puro das ações no walk. Coordenar versão interna
fallible dos displays de State para propagar erros de update exigidos pela
leitura; conservar callback/coerção defensiva legada de display. Não mudar as
assinaturas públicas destes entrypoints nem os demais pós-processadores.

### Aceitação e fronteiras

Gate A1-S2 ratificado para as cinco assinaturas e a fase de leitura. L0 primeiro,
RED funcional antes do código; GREEN contra o pin indicado para linguagem e
controles próprios das extensões Tekt. Não ampliar outros contratos públicos.
Build/workspace/doctests, Clippy/Rustdoc estritos, fmt, linhagem V1/V5/V15/V26 e
reversa; manter V16 sem aumento. MemoryMax12GiB/High8GiB/swap0/jobs1, tarefas
pesadas sequenciais. Medir consumo sem inferir ganho global só da representação.


## A1-H2 — suplemento antes das consultas, aprovado em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
final-freeze.json.gz e target-controls.json.gz no diagnóstico
lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17 registram
UTC/diffstat/fontes/argv. Pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
heading-raw-constructor-func passa no baseline/vanilla e falha no candidato:
Styled de transporte altera func. Query também vê wrapper causal temporário;
HTML ignora callback panic. Fonte walk.rs:930–945; pipeline/paged.rs:419–427 e
pipeline/html.rs:101–106 preparam a consulta pelo walk puro. São valores e erros
de linguagem, não um requisito de copiar algoritmo/estrutura do vanilla.
Gate A1-H2 confirmado pelo dono nesta sessão: revisão
lab/history/construction/records/00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md,
SHA-256 66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c.

### Responsabilidade exclusiva

Adicionar somente a fachada pública aprovada:
realize_heading_supplements(intr: &mut TagIntrospector, engine: &mut Engine<'_>,
ctx: &mut EvalContext) -> SourceResult<()>.
Delegar ao filho proprietário. Finaliza apenas Heading pendente, sem walk,
State/Counter/equações, layout ou exportação. Demais entrypoints preservam
assinaturas; atualizar chamada interna do filho para índice, sem exigir tags.

### Aceitação e limites

H1–H8 da revisão ratificada, RED bilateral antes do código. Nenhum contrato
público além da fachada Heading está autorizado por H2; A1-S2 cobre separadamente
cinco helpers State. Sem novos campos públicos. Styled ordinário e demais
callbacks conservam suas fronteiras. Workspace/doctests/build/Clippy/Rustdoc,
fmt, V1/V5/V15/V26/reversa e V16 sem aumento; 12GiB/High8GiB/swap0/jobs1,
comandos pesados sequenciais. Ganho de RAM exige medição, não intenção.
