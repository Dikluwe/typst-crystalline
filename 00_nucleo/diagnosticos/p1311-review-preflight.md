# P1311 — revisão independente de escopo antes do candidato

Revisor `/root/p1311_review`, sem contexto conversacional herdado (`fork_turns:
none`), regime A/B executado sem atestação técnica de isolamento. Entradas:
pedido delimitado do coordenador, skill `tekt-materializacao-segregada` e suas
duas referências, CLAUDEs, ADR-0127/0129/0130, L0 proprietário integral,
seleção P1309 e verificação/relatório P1310, fonte pré-candidata e vanilla.
Escritas concedidas: somente diagnósticos `p1311-review*` e
`p1311-verification.json`. Sem autoridade para mudar L0, código ou oracles.
Filesystem compartilhado; esses limites são disciplina operacional, não
atestação de isolamento. Nenhuma leitura de materialization/context ocorreu.

Proveniência observada em `2026-09-08T00:50:12Z`: HEAD
`eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`, working tree não commitado.
`git diff HEAD --stat` mostra somente os resíduos preservados de P1310:

```text
 00_nucleo/prompts/compiler/stdlib/loading.md |  63 +++++++++++-
 01_core/src/compiler/stdlib/loading.rs       | 143 ++++++++++++++++++++++++++-
 2 files changed, 200 insertions(+), 6 deletions(-)
```

Baseline canônico `00_nucleo/diagnosticos/p1311-baseline.json`, SHA-256
`2338de6be4567c9ea03be335c5935832241d8010272b10eb29c1ca858c9af753`.
L0 ainda sem amendment P1311, SHA-256
`f24721d430c58791e3e5075bc0a8a30ab61909cdf43992fe84eaa53dab193937`.
Consumer ainda pré-candidato, SHA-256
`72301adc138e0e323425145d077bf8bb16f1e38bba5159baaa92a29353a929f2`.

A seleção `plain-function-missing-field` P1309 aponta para este owner e para
`csv.encode`, `read.encode`, `xml.encode`; não autoriza introduzir encoders.
`field_access.rs:253-266` contém a bifurcação namespace Some/None;
`:113-124` escolhe a âncora. No vanilla ratificado, `func.rs:280-307` produz
o diagnóstico nominal para nativas, e `code.rs:347-366` usa o span do campo.
O carrier vigente `FuncRepr` distingue Native, NativeWithEngine, With,
Closure, Element custom e Plugin sem nova API. `Func::name` não basta para
classificar: também retorna nome de closures, elementos e plugins.

O amendment deve delimitar Native/NativeWithEngine com namespace ausente,
incluindo With, e o nome público curto sem depender do alias lexical.
Namespace Some e categorias excluídas precisam de política de preservação
baseline, mesmo se o vanilla divergir. A regra anterior P1306 que preserva
todos os targets não Module deve ser substituída expressamente somente no
recorte autorizado. Chamadas `f.nope()` exigem medição própria, pois podem
passar por outro caminho de dispatch.

ADR-0127 classifica a correção diagnóstica interna como fluxo contínuo, sujeito
a L0-first, RED→GREEN e revalidação. Não se vê necessidade de contrato público,
fase ou default novo no escopo descrito. Esta conclusão não é GO para código:
o gate prépatch depende do amendment e corpus independente congelados, com
políticas compatíveis auditadas antes de qualquer candidato. Isso previne a
reclassificação tardia de named-prefix registrada em P1310.

Estado: escopo preliminar compatível; gate prépatch ainda pendente.
