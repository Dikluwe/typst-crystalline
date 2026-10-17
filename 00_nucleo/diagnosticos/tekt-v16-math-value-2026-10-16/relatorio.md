# V16 — projeções matemáticas de Value

Etapa concluída; objetivo geral de correções permanece aberto.

## Resultado

**V16: 244 → 240.** Quatro fallbacks internos foram substituídos pelo complemento nominal de Value em três consumers e seus três L0. Assinaturas, corpos das consequências, defaults, fase, mensagens, spans e carriers foram preservados. V19: **888 → 892 notas**, com quatro grupos equivalentes mantidos e prescritos individualmente nos L0; V20 permanece **871 notas**. Não houve isenção nem mudança de configuração para reduzir o gate.

[Patch desta etapa](stage.patch) · [Lista dos quatro casos e justificativas V19](lista-correcoes.md) · [classificação](classification.json) · [240 pendências e casos latentes](lista-pendencias.md).

M1 parse_delim_val e M2 parse_delim_char, em eval/math/delimiters.rs, preservam aliases, Unicode, None e os defaults de arrays. M3 vec_delim_char preserva a mesma distinção de ausência/primeiro caractere e a conversão de arrays do vetor. M4, em math_matrix_augment, preserva a presença de stroke já validado: Length marca thickness, Color marca paint e Stroke mantém seu payload/bits. Entradas inválidas continuam rejeitadas pelo extractor antes dessa decisão.

A comparação [SARIF anterior](baseline-architecture.log) / [novo](validation-architecture.log) removeu somente os quatro V16 e adicionou somente os quatro V19 previstos. [architecture-delta.json](architecture-delta.json) compara todos os demais resultados, níveis, mensagens, multiplicidade e localizações, mapeando exatamente os deslocamentos de linha nos três arquivos editados.

## Testes e proteção de evolução

Os sete testes foram escritos após os L0 e antes da implementação: passam antes e depois, preservando o comportamento existente. Os controles P1431 também passaram antes (29 testes no filtro). O RED estrutural era V16 nos quatro pontos e ausência de proteção local contra variante nova.

Em cópia descartável do workspace real, adicionar TektMathValueProbe a Value produz **0 → 4 E0004 nos pontos selecionados** e **24 → 28 E0004 no total**, sem outros erros. Já havia barreiras em outros owners; esta etapa acrescenta a barreira em cada uma das quatro decisões. Os fontes produtivos permaneceram intactos durante a sonda. [Prova anterior](probe-baseline-result.json) · [nova](probe-green-result.json); entradas completas e logs estão nos mesmos diretórios.

Suíte completa: **8640 passaram, zero falhas e 7 ignorados preexistentes**, mais sete testes que o último baseline completo. Oracle: pin 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, verificado pelo executor. [Resultado do workspace](workspace/workspace-result.json).

Build workspace, fmt, Clippy all-targets com warnings/expectativas não cumpridas como erro, Rustdoc workspace com -D warnings, V1/V5/V15/V26 estritos, linhagem reversa e diff-check passaram. [validation-result.json](validation-result.json) registra comandos/UTC e prova que os hashes dos fontes não mudaram durante a validação. O gate arquitetural global continua exit 1 pelos 240 V16 pendentes.

## Recursos e proveniência

Limite por cgroup **12 GiB (12884901888 bytes)**, swap **0**, Cargo **1 job**, comandos pesados sequenciais. Pico medido: **12884901888 bytes (12.000 GiB)**. Zero OOM e OOM kill; maior contador de pressão memory.events/max: 1033. Não se infere ganho de desempenho/memória desta manutenção. [Recursos finais](validation-resources.json).

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, branch Tekt, **working tree não commitado**. [before.json.gz](before.json.gz) conserva lista exata git diff HEAD --stat, status, fontes e hashes antes de L0/código; [planning-before-pin.json.gz](planning-before-pin.json.gz) conserva o planejamento anterior à aplicação do pin autorizado no lote precedente. O snapshot efetivo desta etapa já usa o pin publicado `12797073972e375d9ab078cf6750c8b30b71e2b9`, binário `/tmp/tekt-linter-none-target/debug/crystalline-lint`, SHA-256 `e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2`. Datas do diretório seguem America/Sao_Paulo (16/10); os registros UTC são 17/10.

Somente os três consumers e seus três owners mudaram no inventário produtivo. [code-guard.json](code-guard.json) e [materialization.json](materialization.json) conservam o diff prescrito, preservação dos testes e corpos, escopo e hashes bidirecionais. Não houve commit, push, merge, release ou nova alteração de pin nesta etapa. Relatórios e passos permanecem fora de prompts/.

## Continuidade

Permanecem 240 emissões V16, os casos latentes T027 e T158/T160, e a auditoria dos demais catchalls internos. A correção atual é estrutural de evolução Rust; não declara novo ganho de paridade da linguagem nem cobertura de execução das 39 variantes. Próximo lote: auditar os owners de eval/mod.rs e layout/realized.rs e os classificadores usados na prova de completude antes de prescrever a correção dos latentes.
