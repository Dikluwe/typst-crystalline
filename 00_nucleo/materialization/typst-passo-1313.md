# Passo 1313 — CSV com fonte Bytes e cast DataSource

**Estado: contrato aprovado pelo dono em 2026-09-08; implementação em curso.**

A resposta específica «Autorizo» encerrou o gate abaixo. Aprovação, estado e
hashes anteriores à execução: `00_nucleo/diagnosticos/p1313-implementation-baseline.json`.
O registro da paragem permanece como histórico; não constitui bloqueio atual.

## Medição e escolha

P1312 separou read de CSV: a antiga coorte loader-path-cast não tinha contrato
homogêneo. O sucessor selecionado é o restante CSV, não uma extensão a read.
Medição fresca `00_nucleo/diagnosticos/p1313-measurement.json`, SHA-256
`f07efa356c554a4b80a5702c516c4e7f7bee51f4b70fc8cc419ea6cf8754cb43`,
com HEAD, working tree, horários, fontes, argv e saídas bilaterais.
Baseline `00_nucleo/diagnosticos/p1313-baseline.json`, SHA-256
`0ad67b8f92c25160f044c352241572916716e2489ad105a6314608d51f59de92`.

Vanilla ratificado a51e02804 aceita CSV Bytes, vazio, delimiter/row-type e With;
cristalino rejeita Bytes como caminho inválido. Fonte vanilla csv.rs:27-46 e
loading/mod.rs:46-110 declara DataSource. O decoder puro já existe no owner
cristalino, loading.rs:944-1006. Mensagens de parsing, named e Symbol ainda
divergem; não os rotular como paridade nem corrigir incidentalmente.

## Contrato para aprovação

Owner: `01_core/src/compiler/stdlib/loading.rs`.
L0: `00_nucleo/prompts/compiler/stdlib/loading.md`, seção P1313 proposta.
CSV recebe Path/Str/Bytes; Bytes alimenta o decoder existente em RAM, sem I/O.
Cast inválido usa tipo longo e origem do primeiro valor posicional. Path/Str,
opções e ordem de validação mantêm política vigente. Não mudar read, cinco
decoders, encoders, entidades, signatures Rust, dispatch ou csv.encode.
Symbol mantém rejeição com formatter novo explícito. Falhas de parsing/opções
que agora se tornam atingíveis por Bytes usam o comportamento legado; não
se promete paridade desses diagnósticos neste recorte.

## Paragem obrigatória nesta sessão

O pedido para escrever/implementar o próximo passo precede este contrato.
A ampliação CSV Path/Str → Bytes encontra dúvida real entre a permissão de
paridade contínua e o gate de compatibilidade/precedente de casts P1141.
A ADR-0127 determina parar na dúvida; parecer independente em
`00_nucleo/diagnosticos/p1313-review-gate.md`.
Portanto: apresentar a proposta ao dono e obter confirmação específica antes
de escrever testes L1 ou patch. Não ressellar header nem criar GREEN fictício.

## Execução após confirmação

1. Registrar aprovação e conferir novamente baseline/alterações existentes.
2. Atualizar status/tabela vigente L0; validar V15/V26 e resselo autorizado.
3. Regime A/B proporcional: testador em contexto novo congela observáveis
   sem candidato; revisor distinto valida políticas antes do patch. Filesystem
   compartilhado: executado sem atestação de isolamento técnico; sem selo completo.
4. Medir alvos e controles nos quatro perfis. Testes de valores de linguagem,
   cast, origem e ausência de I/O; preservar parsing/named explicitamente.
5. Testes locais RED real; patch mínimo no owner; GREEN e build em target
   RAM dedicado, sem sobrescrever executáveis anteriores.
6. A/B normal/repeat/reverse e replays P1310–P1312/P1308. Declarar os deltas
   intencionais CSV dos oráculos antigos, sem reescrevê-los ou esconder dívidas.
7. Testes workspace, fmt/check, crystalline-lint, diff/check e veredito
   independente; relatório substantivo em diagnósticos. Preservar qualquer
   incidente de teste, como o observado no P1312; não afirmar causa sem prova.

Unknown obrigatório, crash, timeout ou fixture inválida bloqueia. Até duas
revisões focais por causa; sem ganho, reexaminar contrato antes de repetir corpus.
Escritas autorizáveis: este passo, owner/L0, diagnósticos p1313-* e temporários
dedicados. Sem commit, stage, push, limpeza histórica ou passo 1314.
