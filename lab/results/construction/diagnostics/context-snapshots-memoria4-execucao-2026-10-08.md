# Memória — prioridade 4: compartilhar snapshots imutáveis de entrada

2026-10-08. Autorização do dono: “faça o 4”. Mudança privada conforme ADR-0127, L0 primeiro e fluxo contínuo. Nenhum contrato público, campo público, default, fase ou compatibilidade de linguagem alterado.

## Resultado

Execution.input agora conserva Arc<TagIntrospector>. Um snapshot concreto por descoberta/entrada de tentativa é compartilhado entre os produtores e descendentes que leram essa entrada. ctx.introspector continua próprio, com clone independente para avaliações que o modificam. Não usar igualdade/hash/id/páginas para deduplicar; não compartilhar ctx mutável.

A próxima geração recebe outro snapshot; registros substituídos e retired conservam suas entradas originais. Vec histórico, cinco tentativas, contribuições, sinks, warnings, erros, replay, Locations, seleções e fachadas L1 continuam. A validação HTML faz clone próprio antes de substituir vistas Page. O mecanismo elimina somente a segunda cópia profunda por Execution; não elimina todos os snapshots ou buffers.

## Medição anterior e proveniência

O perfil memória-fases3 encontrou ~196 MiB no context300 contra ~46 MiB no plain300 e crescimento ~62,5 → ~138,7 MiB na preparação. Estado/UTC/fingerprints em `00_nucleo/diagnosticos/memoria-fases3-2026-10-08/`. A fonte inicial desta rodada, context_stabilization.rs:196,244, clonava input para ctx e para Execution. TagIntrospector deriva Clone e possui mapas/vetores próprios. Isso justificou medir uma alteração privada; RSS não foi usado como prova de leak ou de quais objetos estavam vivos.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado com extenso passivo herdado preservado. `context-snapshots-memoria4-2026-10-08/before.json.gz`, `red-state.json.gz` e `candidate.json.gz` registram textos completos, HEAD, UTC, status/stat. RED registra o stat naquele momento; status inicial enumera a árvore herdada e somente este par foi alterado. Mudanças produtivas/L0 exclusivamente:

- `03_infra/src/pipeline/context_stabilization.rs`
- `00_nucleo/prompts/infra/pipeline/context_stabilization.md`

Diagnósticos são adicionais. Baseline pós-memória 2, congelado `/tmp/typst-before-context-snapshots-memory4`: SHA-256 `4686d15bd91c1b9cd9ca4605fdec16b8c7babd59f4e125276ebcaba87969379c`, receita anterior em embedded-fonts-memoria2-execucao-2026-10-08.md. Novo `target/release/typst`: SHA-256 `564b1fe69f92880736485e882e2e1409a7a9df59712632783e6eae5f74470784`. Source/prompt/seals completos em seals.json; code hash f52e580d, prompt 3f86d039. Nenhum commit feito.

## RED→GREEN e validação

Antes da implementação, o teste produziu A/B corretos mas falhou na testemunha de compartilhamento: os dois registros de uma entrada usavam endereços distintos (`red.log`, `red-state.json.gz`). Após implementação passou: registros da mesma entrada compartilham snapshot; limpar introspector do ctx de um produtor não muda os registros; substituir execução conserva snapshot antigo no retired e no outro produtor, distinto da nova geração com os mesmos valores (`green.log`). Testemunha física verifica custo, não paridade da linguagem.

Infra completa: 1253 passed, 2 ignored, RUST_MIN_STACK=67108864, `cargo test --offline -p typst-infra --lib`. Inclui casos preexistentes de contexts selecionados, replay, warnings/erros, budgets, contadores Page, HTML/bundle e features. Não alegar nova comparação CLI HTML bilateral; a cobertura HTML vem da suite. Builds dev e release offline passaram. Ownership/núcleos validados antes do trabalho e resselo só do par; crystalline-lint final: 0 errors, 535 warnings, 1640 infos, passivo inalterado, não zero violations globais.

Onze PDFs antes/depois preservaram texto com layout, palavras, dimensões/páginas, bounding boxes (delta 0), estilos de fontes e raster a 36 dpi; stderr idêntico em todos os pares. Dados em qualification.json.gz/bench.json.gz. Não é certificação global de paridade vanilla.

## Bateria de memória/tempo

UTC 2026-10-08 23:06:10.840726–23:06:56.983891 (20:06 em São Paulo). 264 amostras: 11 fixtures × 3 blocos × 4 pares × 2 motores, ordem alternada e uma execução de aquecimento por motor/caso/bloco. CPU 4, Rayon 1, epoch 0, locale/fonts comuns. CLI fresco com filesystem aquecido. `%M` de time mede pico RSS do processo; wall inclui wrapper comum. Monitor exclui testes/builds/linter/GDB concorrentes; SHA de fontes/config/código/L0/fixtures/binaries verificados antes/depois. Arquivos measure.py/protocol.md/bench.json.gz permitem reproduzir.

| Caso | Pico RSS antes → depois (MiB) | Δ RSS pareado | Tempo antes → depois (ms) | Δ tempo pareado |
|---|---:|---:|---:|---:|
| context | 43.13 → 43.25 | +0.27% | 114.08 → 114.10 | -0.21% |
| context10 | 42.91 → 43.07 | +0.31% | 114.62 → 115.13 | +0.31% |
| context100 | 60.09 → 55.17 | -8.25% | 141.49 → 133.28 | -6.50% |
| context300 | 196.35 → 132.45 | -32.57% | 316.92 → 222.38 | -29.70% |
| image | 18.94 → 18.84 | +0.05% | 9.80 → 9.47 | -1.03% |
| pipeline-single-font | 41.93 → 41.72 | -0.21% | 112.33 → 112.56 | +0.39% |
| plain300 | 46.28 → 46.36 | +0.05% | 134.72 → 135.82 | +0.58% |
| shapes | 19.97 → 20.04 | +0.02% | 13.06 → 13.07 | +0.07% |
| table-alias | 44.18 → 44.28 | +0.20% | 120.53 → 120.91 | +0.39% |
| table-local | 44.60 → 44.72 | +0.17% | 131.27 → 131.31 | -0.20% |
| text-dense | 43.73 → 44.07 | +0.86% | 133.49 → 133.93 | +0.25% |

Razões pareadas têm mediana própria: não confundir com razão das medianas da tabela. Valores por bloco em summary.json.

Context100: RSS −8,31/−8,12/−8,00%; tempo −5,61/−6,55/−6,31%. Context300: RSS −32,54/−32,58/−32,55%; tempo −28,50/−29,93/−29,73%. Ganhos consistentes nos três blocos. Redução de pico mediano context300: ~63,9 MiB.

Nos controles sem população contextual grande não certificar economia geral. Context10 ~+0,31% RSS e +0,31% tempo, com sinais mistos entre blocos. Texto simples ~+0,39% de tempo nos três blocos (~0,23 ms de diferença entre medianas); os demais pequenos controles variam, até +0,58% agregado pareado em plain300. Registrar esse custo pequeno sem afirmar ausência total de regressão, nem extrapolar ganho grande a todos os documentos.

Decisão: conservar o compartilhamento privado da entrada com preservação causal. O caso problemático reduziu tanto memória quanto tempo; não usar isso para apagar retired/históricos ou alterar campos públicos. ctx.introspector e outras vistas ainda podem duplicar dados, mas exigem investigação e L0 próprios antes de outra mudança. Sem soma com percentuais anteriores, sem nova comparação vanilla/watch/disco frio.

completion.json registra identidade final inalterada e hashes dos artefatos.
