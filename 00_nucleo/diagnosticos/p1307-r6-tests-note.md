# P1307-R6 — testes Rust independentes antes do candidato

Estado: patch documental preparado; não aplicado, não compilado nem executado
por este autor. Nenhum veredito de produto. Regime: **executado sem atestação
de isolamento técnico**. Autor `/root/p1307_contract`, com contexto anterior
de auditor de contratos, sem leitura de implementação candidata. As únicas
escritas desta tarefa são este relatório e `p1307-r6-tests.patch`.

## Entradas e autorização

Baseline aprovado `p1307-r6-baseline.json`, SHA-256
`8cc0eae00457d2e7d54b420024eae49344032b34b295b4eda536faeb1ec3c4a3`,
capturado em `2026-09-07T18:50:58.687912+00:00`. Registra autorização humana
`autorizo`, papéis/allowlists, HEAD
`b303f1f15b610e09872b567027e0d806387fde8c` e working tree não commitado:
32 arquivos, 2164 inserções, 191 remoções, com diff/stat integral no próprio
baseline. Não se confunde esse estado com HEAD limpo. O arquivo Rust alvo
antes do patch tem SHA-256
`c10516880bff8677739416d314d742075a478842db8d81eb917a54f83be39c2f`;
contém os testes P1306, preservados integralmente.

Skill de materialização segregada e ambos os documentos de papéis/gates
lidos integralmente, assim como `01_core/CLAUDE.md` e ADR-0127/0130.
Os L0s abaixo foram lidos antes da escrita; os contratos de introspect
completos já lidos na auditoria anterior foram complementados com o novo
amendment R5. Os rótulos históricos de gate pendente dentro dos L0s não
substituem a autorização humana registrada no baseline R6.

```text
00_nucleo/prompts/compiler/eval/tests.md 5a6e958b243ea5b5f16b68ecae024d55b70ccbc752a75b0dcfeee5993d994c72
00_nucleo/prompts/compiler/stdlib/loading.md c800187d07bf3acb84eb620ba4305001588001d088dabff1941ca8fe4fb0a3db
00_nucleo/prompts/entities/args.md 75de6ac49d69331fc604984c92874a706442f823cc4706bd480b434c55c83d94
00_nucleo/prompts/compiler/eval/call_dispatch.md 10196db423a34d8255f27e7f48f715843d9bf601670057db26b9065741931b06
00_nucleo/prompts/entities/value.md 732b3fb512bd11b6585a214de445468e0f0f5fe5907857c3a0f17b4523d23de8
00_nucleo/prompts/compiler/eval/operators/equality.md 9b6ca72b1e8ca562bde7d1730892e727a9680ba6e99b8fc9d1f9922a4c3b5d95
00_nucleo/prompts/compiler/eval/bindings/field_access.md 077fd441aaceffe0470c50aa1e2e6caded3d446b48d1dbc39da7f42d320494db
00_nucleo/prompts/compiler/stdlib/foundations/query.md 7bb0514642d735b3caffb5d501c6ff71b7baf9c5d2fc2f13fb9e4961091db617
00_nucleo/prompts/compiler/introspect/heading.md 1bb5b84fe3658e78c2b18758b58e7ca3ce2d712a0711af145bfdbd2d49811ff0
```

Origem das expectativas, independente de candidato:

- R2 measurement `847faabad41df603a82f7fc5c5d0435180cdec66c33ae9b3a1fd55d7ee320fa2`:
  encoders, With, mensagens, ranges, f/g e CBOR primitivo/Bytes.
- R4 contract-refinement
  `ed6fa784cc79703497e2e45c18010c110b001d82f2b4638899a0fb8dba0859fa`:
  `args.mixed-duplicates`, map/filter, join, sink-consumed-named/default-absent.
- R5 oracle `ccfd62a66676561d58562123dfe91e039ef8a50d12ff707fe0d03c6a66204520`
  e measurement `82b2de8863ae5cd4b706eb9d5a6b285e1ed31dce3c126c8e5383a5af59ab46c8`:
  Heading fields, JSON, label, pt, clone e igualdade. A nova expectativa de
  igualdade vem de `equality.different-numbering`, não da afirmação antiga
  refutada de morph_canon suficiente.
- L0 loading R3/R5 determina a aplicação composicional dos mesmos formatos a
  valores especiais, raw Content, None e não finitos. Helpers não chamam o
  candidato para gerar expected. JSON compacto de fields deriva da sequência
  congelada e do contrato compacto; o oráculo externo R6 mede a projeção
  diretamente antes do selo final.

Referência ratificada: upstream `a51e02804`, `/usr/local/bin/typst` SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Não houve nova execução de vanilla ou baseline nesta tarefa de autoria.

## Patch e cobertura

`p1307-r6-tests.patch`, SHA-256
`d4232b6144b7def77498e2fd1353b5dbad3786276fb8ccd5f8c56175a08fa19b`.
Alvo único: `01_core/src/compiler/eval/tests.rs`, adição do módulo
`tests::p1307_independent`, com **22 funções de teste** e helpers puros.
Todos os casos percorrem os quatro perfis do helper P1300 existente.

| Família | Testes/observação |
|---|---|
| Namespace/defaults | Identidade encode, With anônimo, JSON/TOML pretty/compact, YAML ordem/newline. |
| Emissão integral | JSON Bytes/Symbol/Text, None/inf/NaN/-0.0, escapes, raw Heading sem defaults realizados. |
| Parent versus membro | Prebinding inválido no pai não contamina encode; alias e cadeia With; override válido tardio. |
| Diagnósticos | Missing chamada inteira, With missing, value named+hint, named desconhecido, pretty inválido anterior/posterior, TOML None array/tipo. |
| Origem | Factories f/g com Args iguais e sinks iguais: ranges diferentes, trace encode na chamada final e source resolvível. |
| Args | Duplicatas e ordem constructor/map/filter; len da linguagem; join diferente de With; sink não reintroduz named consumido ou default ausente. |
| Preservação | Decoders JSON/TOML/YAML e bytes CBOR exatos. As demais guardas antigas não são removidas. |
| Snapshot | Walk puro real, fields completos/ordenados por JSON, métodos estático/instância, numbering diferente afeta ==/in/igualdade aninhada, label/location não afetam ==, raw≠realizado, clone por array/closure e suplemento pt. |

O helper `query_value` usa um documento Typst real, eval vigente, o primeiro
`introspect_with_introspector` puro e eval de expressão com aquele snapshot.
Não constrói `IntrospectedContent`, não insere diretamente em elements, não
pressupõe campo novo e não chama encoder Rust inexistente. A instalação de
features ocorre no eval observador; a produção do documento usa eval padrão,
pois as fixtures de Heading não têm dependências de feature. Isso não é uma
prova E2E de que L3 encadeou todos os perfis corretamente: essa obrigação fica
com a suíte externa, inclusive primeiro context real antes da heading.

Não se usa `repr(Dict)` para observar fields: essa representação geral tem
dívida anterior. O JSON exato exige ordem, presença e todos os campos sem
abrir esse formatter. A representação de Args é, ao contrário, delta explícito
R3/R4 e possui expectativa própria congelada.

Os negativos comparam mensagem inteira, severidade, cardinalidade, hints,
laterais, range half-open e traces previstos. Como testes L1, não observam
exit/stdout/stderr CLI; não alegam cobrir esses observáveis. A suíte externa
independente cobre apresentação, cross-source, all-Value, formas opacas,
map/filter detached, sete loaders completos e ausência csv/xml/read.encode.
Esta seleção focal não substitui o contrato abrangente nem transforma esses
casos em scope-out.

## Checks executados e gate restante

Checks somente leitura realizados sobre o patch final:

```text
git apply --check 00_nucleo/diagnosticos/p1307-r6-tests.patch
exit 0

extração das linhas adicionadas para stdin de rustfmt --edition 2024 --emit stdout
exit 0 (saída descartada; nenhum Rust escrito)
```

O segundo check prova parsing Rust, **não** resolução de nomes/tipos ou build.
O patch usa só APIs presentes no baseline, mas compilabilidade efetiva precisa
ser confirmada pelo integrador em cópia de baseline após o selo e antes do
produto. Primeiro RED esperado dos encoders: field `encode` inexistente;
falha de compilação, fixture ou harness não conta como RED. Se isso ocorrer,
preservar tentativa e devolver a este autor para reparo focal sem acesso ao
candidato; não mudar expectativas para caber na implementação.

Permanecem pendentes aplicação/compilação, RED semanticamente válido,
integração/execução de candidato e veredito do verificador. Nenhuma morte de
mutante Rust ou certificação é reivindicada. Os 22 testes são contribuição
independente ao selo, não prova de paridade global nem autorização adicional
para implementação fora dos owners aprovados.
