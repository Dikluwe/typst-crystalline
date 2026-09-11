# P1339 — estabilização autorizada e fronteira pública concretizada

## O que avançou

A resposta do dono autorizando a estabilização foi registrada em
`p1339-stabilization-approval.json`, SHA-256
`15027b4ce1c5eb0833ec6f6178c589d85356d62c8c1f41e88cf2506ee6c88ef7`.
A instrução posterior “Continue” foi atendida com redação normativa e checks;
não é registrada como aprovação de interface pública ainda não submetida.

Foram redigidos os blocos P1339 de `prompts/compiler/eval.md` e
`prompts/infra/pipeline.md`; a referência em introspect/from_tags foi
atualizada para o escopo já aprovado. Escolhida a validação de **todas as
entradas contextuais observadas** do bloco selecionado, não um comparador de
closures nem um teste isolado do valor de counter. Resultado original pode
ser conservado quando seus inputs permanecem válidos.

Contadores legados não são reparados por associação: bloco sem demanda real
Element conserva contribuição/erro da passagem ordinária; erro anterior à
demanda não se torna provisório pelo texto da expressão. O snapshot permanece
read-only durante eval; o registro é separado e privado.

## Evidência adicional que alterou o desenho

`p1339-stabilization-boundaries-runs.json`, SHA-256
`4d82648d895ec5dba23a9775c2bf09dfae84a175603f3ce6d742c8c7f2a09241`,
conserva HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree,
diff/stat, fontes, binário, argv, UTC e canais integrais. Runner
`p1339-stabilization-boundaries-probe.py`: seis casos em duas ordens no
vanilla ratificado a51e02804, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

| Fronteira | Resultado nas duas ordens |
|---|---|
| update oscilante | saída [[0]], exit 0, warnings de não convergência |
| update crescente | saída [[4]], exit 0, warnings de não convergência |
| chave NaN, sem matches | saída [[0]], sem warning de convergência |
| callback recriada em context | saída [[12]], sem warning de convergência |
| produtor context aninhado | saída [[12]], sem warning de convergência |
| erro de bloco alheio | exit 1, panicked with: unrelated-error |

A warning de depreciação de query está preservada, mas não é warning de
convergência. A primeira execução só ficou no transcript de ferramenta;
a repetição para persistir o recibo não mudou o runner/casos e foi registrada
no próprio JSON. Nenhuma medição cristalina candidata foi realizada.

Fonte pinada `typst/src/lib.rs:133-180` e
`typst-library/src/introspection/convergence.rs:16,56-116,267-278` confirma
validação dos observáveis e a saída com warning no teto. Portanto a hipótese
exploratória de sempre abortar por erro após o teto foi corrigida no L0.
Isso não autoriza chamar estado oscilante de convergente nem usar warning para
disfarçar registro incompleto. Valores/warnings são linguagem; quantidade
interna de passes só importa aqui porque pode alterar esses observáveis.

## Contrato público agora proposto

Adicionar armazenamento privado ao EvalContext e exatamente estes métodos,
no owner existente `compiler/eval.md`:

```rust
pub fn has_filtered_counter_reads(&self) -> bool;

pub fn context_reads_valid_for(
    &self,
    candidate: &TagIntrospector,
    engine: &mut Engine<'_>,
) -> SourceResult<bool>;

pub fn context_nonconvergence_diagnostics(
    &self,
    history: &[TagIntrospector],
    engine: &mut Engine<'_>,
) -> SourceResult<Vec<SourceDiagnostic>>;
```

O primeiro informa dependência efetiva. Os outros validam/reproduzem as
leituras registradas e seus diagnósticos sob recursos explícitos, sem refazer
o corpo do bloco, alterar snapshot nem avaliar chaves nunca demandadas. A
pipeline continua com assinaturas atuais e coordena tentativas/sinks locais.
O texto completo no L0 distingue erro do corpo, observação alterada e falha
do validador; true nunca implica que um bloco com Err teve sucesso.

### Impacto de compatibilidade, não escondido

EvalContext atualmente possui somente campos públicos
(`01_core/src/compiler/eval/mod.rs:124-220`). Acrescentar armazenamento privado
impede que consumidores externos construam a struct por literal; precisam usar
`EvalContext::new()` e os campos/métodos públicos existentes. A busca dirigida
nas quatro camadas não encontrou initializers externos, mas não prova que não
existam consumidores fora do repositório. Campos/métodos atuais e assinaturas
de chamadas não mudam; **isso não é garantia de compatibilidade irrestrita**.

Essa interface nova e essa restrição à construção por literal exigem
confirmação ADR-0127 antes do código. A autorização de estabilização tratou do
escopo/ordem; não se retrodata a aprovação desta forma concreta.

## O que ainda não está concluído

O contrato de estabilização declara explicitamente sua integração incompleta
no próprio P1339: cada owner de leitura alcançado precisa registrar e reproduzir
os inputs pertinentes, inclusive consultas anteriores à primeira demanda
Element. Não há getter/comparador implementado, código produtivo modificado,
resselo, contrato de testes selado, mutantes executados, RED independente,
GREEN, build de candidato, commit ou fechamento do passo.

A revisão de desenho foi feita separadamente em `p1339-stabilization-design.md`.
Uma revisão focal adicional dos L0 recém-redigidos foi solicitada ao mesmo
revisor com nova allowlist; a execução falhou por limite de uso antes de
entregar parecer. Não há `p1339-stabilization-l0-review.md` nem aprovação
independente presumida. Nenhum reset de uso foi solicitado ou consumido.

Skill `tekt-materializacao-segregada`: protocolo completo ainda pré-contrato,
executado sem atestação de isolamento. A separação de desenho evidenciou a
insuficiência de observar somente Element e levou à validação dos inputs
completos; não equivale a selo. A pausa antes de código decorre do gate de
contrato público/compatibilidade, não da necessidade de autorizar novamente
a estabilização já aprovada.

Checks, hashes e preservação dos arquivos anteriores ficam no recibo
`p1339-stabilization-receipt.json`. V15/V26 e diff-check não são GREEN
funcional; V5 continua pendente enquanto os L0 estão em redação.
