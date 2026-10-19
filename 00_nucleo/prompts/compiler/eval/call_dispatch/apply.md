# L0 — compiler/eval/call_dispatch/apply
Hash do Código: c18e82a6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml sha256:32890de1247262b54fa39d7ae1d640a068774a28cf1fa1c8890d0508d999af51

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/call_dispatch/apply.rs`. No HEAD `d5c563dfab39f789e8e2f72c9a9364f5a761e3ca` e árvore limpa, `01_core/src/compiler/eval/call_dispatch.rs:503-603` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `lab/results/construction/diagnostics/atomizacao-call-dispatch-2026-09-30-before.json.gz`.

## Contrato

Aplicar FuncRepr fechado a closures, elementos, plugins e funções nativas, incluindo seleção de span nativo. Preservar assinaturas públicas, diagnósticos, prioridades de intercepção, rastros e semântica de argumentos especificados em `00_nucleo/prompts/compiler/eval/call_dispatch.md`. A extração permanece em L1 com despacho estático, sem I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1, os testes de call_dispatch e `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho.


## Formatos como funções de elemento — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não
commitado. `lab/results/construction/diagnostics/native-format-catalogue-2026-10-17/`
conserva before.json.gz com UTC, status, diff HEAD --stat e hashes exatos;
baseline-reuse.json prova igualdade de fontes/configuração/binário com a
matriz final anterior de 3956 casos em oito perfis. before-batch.json.gz
mede 144 casos detalhados; before-compat.json.gz mede 456 controles de
nomes, aliases e namespace. Cada sonda conserva argv, binários e saídas.
O pin é `9f2b6e8715237cb086899a42873660fe744622e8`, executável SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

As quinze rotas restantes são rejeitadas por where/kind no Tekt e aceitas
no pin. format.rs:17–27/96–146 compartilha native_format entre os cinco
formatos, logo só ponteiro não distingue schemas. pinned-schema.json.gz
conserva nove declarações upstream com blobs/hash/linhas e atributos elem.
FuncRepr é pub(crate) em entities/func.md:21 e entities/func.rs:24; a nota
anterior de que o L0 o mostrava público foi refutada em retificacao-func-repr.md.
public-contract-before.json fixa assinaturas, reexports e structs públicos
antes da mudança; a classificação é representação interna e paridade, não
novo contrato público ou inferência de intenção por render.

### Responsabilidade exclusiva

Aplicar Format pelo mesmo braço estático de Native, usando exatamente o
NativeFunc.call, ctx, args, World e current_file existentes. Não introduzir
chamada recursiva/wrapper, nova fase ou nova instalação/restauração de contexto.
A identificação de span por native_fn_addr continua com o mesmo ponteiro.

### Aceitação e fronteiras

Consumir as invariantes compartilhadas do Núcleo Tekt pinado
`00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml`. As novas
alternativas Native/Format são agrupáveis porque usam o mesmo payload/ABI
para projeção e execução; cada uma deve ser exercitada nos testes de nome,
namespace, ponteiro, Eq/Hash, clone/With e despacho. V19 novo nessas uniões
deve ser inventariado e ligado aos testes; não adicionar suppressions.
A nova variante deve ser explicitamente nomeada nos matches fechados.

Fluxo contínuo ADR-0127: nenhuma definição, assinatura, campo ou trait
externo novo, compatibilidade, default ou fase muda. Antes do código, RED
funcional dos quinze casos por factories atuais e controle de observação
real contra Native não marcada. GREEN exige corpus completo, 144 casos de
campos/chamadas e 456 controles de compatibilidade preservados fora de
where/kind. Comparar public-contract-before/after e sizeof antes/depois.

L0 antes do código; ownership/V26 antes de resselo restrito aos onze pares.
Build, suíte workspace, Clippy/Rustdoc estritos, fmt, V1/V5/V15/V26, reverse
e diferencial global sem novo erro/aviso. Comandos pesados sequenciais sob
MemoryMax12GiB/High10GiB/swap0/jobs1. Rastrear notas V19 legítimas com cobertura.

Esta etapa fecha admissão where/kind, não todas as operações de seleção,
show/query, síntese, morfologia dos carriers ou counter. As frentes FG1, RM1,
SY1, RG1 e V16 anteriores preservam seus escopos/gates; não aplicar candidatos
antigos por cópia nem publicar.
