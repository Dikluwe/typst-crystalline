# P1340 R2 — contrato do binding terminal

Entrada normativa: L0 `infra/pipeline/context_stabilization.md`, SHA-256
`3094bd8cda8d1971e765cf91cbb110d376ff713e4348ae8f4bf8214e33b45b9b`.
Esta especificação é anterior ao candidato. Não implementa nem julga a solução.

O harness `p1340-contract-terminal-harness-r2.rs` é Rust comum, depende só de
`typst-core` e expõe `run_suite(ActualRun)`. O adaptador owner-local, sob
`cfg(test)`, fornece `ActualRun` e um teste que chama `run_suite`. Compilar a
fonte ou usar um runner fictício não satisfaz qualquer obrigação semântica.
O verificador exige ligação ao mesmo código da coordenação/retorno produtivos.

## Construção real dos inputs

Cada caso cria uma Source numerizada nova a partir de `Input.source_text`, com
FileId próprio, e conserva essa mesma Source até o retorno. O target é paged,
features vazias, World e métricas estáveis, conforme a mesma construção já
congelada para lifecycle. Não reconhecer o nome do caso na lógica produtiva.

Nos casos opacos, o harness constrói um `Value::Func(Func::element(...))` real.
Seu constructor entra em panic se invocado: a classificação/replay não tem
autorização de chamá-lo. Depois de eval ordinário e antes da descoberta, o
binding pode substituir somente o valor Int(0) do Metadata rotulado `<opaque>`
pelo `opaque_input` fornecido, na árvore real que entra na mesma sessão privada
produtiva. Refazer a introspecção real da entrada modificada, como requer o
pipeline; não substituir retornos de query, registros, comparador ou bool.
A travessia/substituição é construção do input de teste, nunca um algoritmo
alternativo de estabilização. O binding deve provar que query retorna o carrier
opaco real pela API existente. A fonte pública opaca não pretende ser um novo
literal Typst nem habilitar extensão no CLI.

`QueryDifferentThenStable` e `ClosedIncreasing` usam o conteúdo avaliado sem
transformação. O primeiro contém query de `<made>` em bloco selecionado e um
produtor legado de Metadata(1): a leitura 0 muda para 1, conserva o contador
filtrado zero e estabiliza. A metadata final ordenada é `[1,1]`. Não exigir
texto de não convergência da query: ausência de detalhe não prova Unproven.
O segundo conserva a fixture de crescimento: D5 lê contador `[4]`, metadata
`[4]`; os diagnósticos públicos exatos permanecem sob oráculo P1339.

`OriginalErrorPrecedence` executa o panic real do corpo. O hook guarda o vetor
original antes da decisão terminal; o harness compara severidade, span,
mensagem, hints e trace, na mesma ordem. Não fornecer como original um clone do
vetor final já decidido: a captura deve preceder o branch terminal.

## Sinks como inputs, sem reconstrução de resultados

Os cinco nomes de warnings recebidos são sentinelas de entrada. Cada um é
convertido em `SourceDiagnostic::warning` com span root da Source real:

- GLOBAL: emitir uma vez no sink global anterior à sessão.
- RETAINED: emitir no sink de cada execução do primeiro bloco contextual
  selecionado da fonte, na entrada do corpo; execuções substituídas levam sua
  própria instância e o sink final/retido deve publicar uma vez.
- DISCOVERY: emitir no sink da descoberta do primeiro bloco que alcança
  seleção, antes de sua disposição.
- REPLACED: emitir no sink de uma execução efetivamente invalidada, se houver,
  na aresta real de invalidação antes do descarte; não criar execução fictícia.
- VALIDATION: emitir no sink real de cada validação, antes de sua disposição.

Os hooks apenas adicionam diagnósticos de entrada aos sinks reais para tornar
o efeito observável; não podem marcar a disposição desejada, publicar por fora,
substituir a coleção devolvida ou fabricar warnings após o retorno. O binding
reporta os warnings públicos reais. Os casos de incapacidade exigem exatamente
GLOBAL, RETAINED, nessa ordem; nenhuma sentinela de descarte nem aviso vanilla.
Teto com erro original obedece à mesma disciplina. Metadados/eventos e contagem
de exportação são projeções passivas das transições produtivas.

## Ramo de topologia impossível

`ProvenImpossibleTopologyTerminal` é deliberadamente teste unitário da operação
terminal real, não teste de descoberta de colisão. A Source é válida; o binding
constrói o estado privado de impossibilidade já comprovada que essa operação
recebe, com nenhum erro original e só warning global. O estado precisa ser
representado pela mesma variante/condição que o chamador produtivo transmite;
não é permitido escrever um segundo formatador ou devolver um erro literal
diretamente pelo adaptador. Se não houver operação separável, executar o mesmo
branch pelo estado privado real da sessão, sem substituir a decisão.

O verificador exige testemunha estrutural do ramo produtor dessa condição e
da precedência dos erros originais, além do teste do ramo terminal. Este caso
não prova que um input válido seja ambíguo. Dois produtores válidos com ids
locais coincidentes são controle negativo obrigatório A03, conservado no
contrato adversarial: ambos filhos devem sobreviver. Não forçar seu erro para
obter coverage. Se a representação não permitir esse binding mecânico, declarar
lacuna antes de executar; nunca contar o caso como sucesso pela ausência do ramo.

## Saídas e auditoria

`Observation.result` projeta somente `Ok(document)` para `Ok(())` e conserva
o vetor Err integral. Source, warnings, erros originais, tentativas, contador
lido, metadata e export_calls devem vir da execução, não do esperado. O bool
topology_impossibility é projeção do estado comprovado; selected_body_ok é
projeção dos resultados reais dos corpos selecionados, nunca inferido do
diagnóstico terminal. Hooks não influenciam retenção, validação ou tentativas.

Rodar a extensão estreita em default, depois repetir na mesma ordem e em ordem
inversa mediante uma ligação de ordem de invocação; os 40 públicos/12 lifecycle
mantêm seus quatro perfis e ordens originais. Nenhum agregado substitui a
evidência de cada caso. Os mutantes negativos focais devem alterar a produção
real: mensagem/span errados, descarte de erro original, exportação indevida,
aceitar mixed opaque por detalhe presente, erro prematuro no primeiro false,
publicação de sink de validação/substituído e tratar colisão válida como fatal.
Cada um deve ser rejeitado pelo caso/testemunha correspondente; não alterar
oráculo. Esta definição não autoriza o autor do contrato a produzir mutantes.
