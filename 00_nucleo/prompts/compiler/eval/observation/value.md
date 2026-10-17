# L0 — compiler/eval/observation/value
Hash do Código: e1d677b9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:c2062be931c790cf1de7f77349801100f3837e58e5ae2f933bf2b392a9fb6a0a
- 00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml sha256:32890de1247262b54fa39d7ae1d640a068774a28cf1fa1c8890d0508d999af51
- 00_nucleo/prompts/_nuclei/selectors/native-bare-identity.toml sha256:a37779d03a4ba0f16ebcc5919477bfdcf783021511a28cf8c9203c67cd64a9e5

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/observation/value.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/observation/value.rs:11` — `impl ObservationEq for crate::entities::value::Value {`.
- `01_core/src/compiler/eval/observation/value.rs:249` — `impl ObservationEq for crate::entities::path::VirtualRoot {`.
- `01_core/src/compiler/eval/observation/value.rs:265` — `impl ObservationEq for crate::entities::package_spec::PackageSpec {`.
- `01_core/src/compiler/eval/observation/value.rs:274` — `impl ObservationEq for crate::entities::package_spec::PackageVersion {`.

## Contract

Own the exhaustive Value match and runtime, selection, argument, diagnostic,
function and module carriers outside the other observation families. Compare
variant, presence, order and payload. Scope retains binding order and values.
Native functions compare call identity, name and namespace; closures are Same
only by Arc identity. Plugins and services lacking proof remain Unproven.
Diagnostics distinguish success, errors, spans and causal metadata. Never
execute a compared value. Signed zero and NaN retain their bit distinctions.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

## D08A — consumer adicional de observação

Medição antes da decisão: table-cell-selector-2026-10-06/check.log registra
E0004 em observation/value.rs:470 após a variante pública ratificada.
O match usa Self::Element, por isso o inventário por Selector::Element não
o incluiu no manifesto. O L0 vigente já exige relação exaustiva por variante
e payload; esse consumer não pode ficar silenciosamente sem ramo.

Decisão interna em fluxo contínuo ADR-0127: NativeElement compara somente
outra NativeElement pela relação ObservationEq existente de Func (call, nome
e namespace), sem executar função. Outros variants são Different. Esta relação
conservadora de observação não substitui a igualdade da linguagem por endereço
real no owner equality; nomes não legitimam reconhecimento de elemento.
Nenhum contrato público/fase nova além da variante já ratificada. Testar e
validar junto com o conjunto D08A; ressellar exclusivamente este owner adicional.


## Formatos como funções de elemento — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não
commitado. `00_nucleo/diagnosticos/native-format-catalogue-2026-10-17/`
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

Comparar Format somente com Format pelo kind e pelos campos NativeFunc
call/name/namespace, sem executar. Kind é comparado por Eq privado; callback
por fn_addr_eq, nome e namespace pela relação já existente. Nomear Format
no eixo exaustivo de pares diferentes. Native/Format cruzados são Different,
mesmo se Eq público for true. Isso observa a capacidade causal de elemento
e não substitui a igualdade da linguagem por mecânica. Manter todos os
outros braços de Value/Func/Module e suas prioridades.

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


### Cobertura direta das alternativas V19 — 2026-10-17

Medição anterior à decisão: candidate-1/state.json.gz e
differential-review.json em
00_nucleo/diagnosticos/native-format-catalogue-2026-10-17 registram oito
notas V19 novas e quatro atualizadas. Estas últimas apenas incluem Format
no padrão e elevam a contagem; não são violações removidas. O primeiro
comparador exigia mensagem idêntica e por isso recusou a atualização.

Este owner recebe testes diretos complementares; não mudar o comportamento
produtivo, suprimir notas ou usar somente um caminho como prova do grupo.
Acrescentar matriz dos cinco kinds com nome/callback/namespace fixos para
isolar o papel da testemunha. Variar separadamente callback, nome e namespace
para demonstrar que cada eixo causal importa. Nenhuma função é executada;
Eq público pode continuar true enquanto observação é Different.
