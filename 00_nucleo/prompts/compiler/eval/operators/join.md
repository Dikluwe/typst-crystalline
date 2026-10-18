# L0 — compiler/eval/operators/join
Hash do Código: f19b94af

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/operators/join.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/operators/join.rs:27` — `fn join`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/func.rs:372-374`.
- `foundations/ops.rs:24-45`.
- `lab/typst-original/crates/typst-library/src/foundations/args.rs:468-482`.
- `typst-eval/code.rs:57`.
- `typst-eval/flow.rs:86,132`.

## Contract

Own pure sequential value combination. None is identity on either side,
preserving the other value and all its metadata. Strings and symbols concatenate
complete grapheme sequences; content/string/symbol combinations construct the
existing Content sequence/text form. Arrays concatenate in order. Dictionary
merge keeps the first key position and right-hand value on collisions.
Unsupported combinations preserve the existing long-type-name diagnostic.

Args+Args removes every left named occurrence whose name appears anywhere on the
right, preserves all left positional and remaining named occurrences in order,
then appends every right occurrence including duplicates. Construct through
Args::from_occurrences with a detached aggregate span; retain each survivor's
argument/value origin and coherent views. Do not merge collapsed named views.
This differs deliberately from With concatenation owned by
00_nucleo/prompts/compiler/eval/call_dispatch.md, which preserves all prearguments.

Code-block/loop accumulation belongs to its evaluating consumer; this helper
does not evaluate callbacks, mutate scopes or add context. Canonical long type
names delegate to the error_formatting owner.


## Concatenação de conteúdo — paridade e limite de pilha, 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
fontes, UTC/status/diffstat em before.json.gz de
00_nucleo/diagnosticos/state-stack-2026-10-17. morphology-before.json.gz e
morphology-measured.json.gz medem pin bd5ebec285505889e8ff1d4f68804ab147a699f3:
for/while com quatro corpos [X] e + repetido produzem Sequence encaixada no Tekt,
filhos planos no vanilla. Aliases de Sequence e ambos operandos Sequence também
divergem. Empty + conteúdo conserva Sequence de um filho no vanilla; Empty +
Empty é vazio. Text de string vazia é filho real e não deve ser filtrado.
Upstream foundations/ops.rs:24–45 delega a Content::add; content/mod.rs:645–665
apensa filhos de Sequence de primeiro nível, preserva a identidade/metadata
da Sequence escolhida e não recursa nos filhos internos.

GDB debug-trace/debug-root.log encontra SIGSEGV em map_content_pre, chamado
por table_cells::needs_preparation: a cadeia produzida por 64 updates cria
recursão artificial; dois frames consecutivos desse mapper distam 104664 bytes.
O ensaio grande de A1 (64 updates de arrays com 2048 valores) aborta nos dois
binários Tekt debug, mas o vanilla release aceita. Isso não prova defeito em
State nem resultado do release Tekt. O aninhamento observável em repr é língua,
a escolha Arc/Vec e o tamanho do frame são mecânica. Não inferir intenção.

### Decisão e limites

Corrigir a operação produtora, sem aumentar stack configurada ou achatar a
árvore pública indiscriminadamente. Content::sequence conserva sua normalização
atual e não é alterado. Concatenação remove só a camada Sequence de cada
operando imediato, reconhecendo Source como origem transparente. Demais wrappers
(Styled/Located/Realized/ShowRecipe e elementos com body) continuam filhos opacos.
Filhos Sequence já internos não são achados/achatados recursivamente.

Preservar ordem e aliases imutáveis. Source do resultado é o do operando esquerdo
quando ele é Sequence/Empty; senão o direito quando este é Sequence/Empty; sem
Sequence, formar pai novo e conservar origens dos dois filhos. Reusar source_span
e with_source_span existentes, sem redefinir a prioridade geral de origem.
Empty é a representação Tekt da Sequence vazia para esta operação: não inserir
Empty como filho, mas preservar Sequence com um filho quando a soma envolve
uma sequência. Não confundir Text("") com Empty nem usar is_empty/plain_text para
filtrar filhos. None de Value é identidade anterior à operação, sem reconstrução.
Coerções Str/Symbol existentes ficam intactas; a morfologia de Symbol/styled e
reflexão children/func/fields de Sequence vazia identificadas nas sondas continuam
dívidas próprias, não provas de paridade deste recorte.

Correção interna/de paridade em fluxo contínuo ADR-0127: sem API/trait/campo público,
modo/flag/default de produto ou fase novos. Prescrever antes de testes/código,
RED funcional de forma/ordem/origem, GREEN e revalidar. Pesados sequenciais sob
12GiB/High8GiB/swap0/jobs1. Confirmar o ensaio original completo, não substituí-lo
pelo caso menor; controles maiores de for/while/add, State e callbacks, fonte e
produto visual. Não prometer custo linear de append sobre Arc<[Content]>; medir
tempo/RSS e conservar a limitação se a representação atual exigir cópia do prefixo.

Responsabilidade exclusiva: a combinação sequencial de Values e o helper privado
pub(super) concat_content(Content, Content)->Content. Os cinco braços Content
com Content/Str/Symbol delegam a esse helper após as coerções já owned. O helper
conserva filhos por clone compartilhado, não muta os Arcs dos aliases nem muda
a igualdade Rust. Seu booleano de classificação é nominal Empty/Sequence após
source_body; casos restantes continuam valores opacos completos. Grupo nominal
Empty|Sequence é uma só política de operando sequencial, sem supressão V19.
Testes locais cobrem operandos de ambos os lados, Empty/Text vazio, None, aliases,
spans, wrappers, filho Sequence interno e acumulação repetida.
