# L0 — wiring/watch
Hash do Código: 89e4b093

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/watch-font-generation.toml sha256:391cd1604976262b1b6e019b802ed72e78782260d2788bb04999297559605509
- 00_nucleo/prompts/_nuclei/watch-source-generation.toml sha256:7ec137b6996e0ea442e3cdda52ce723fd32cb67338ab363b7ff2d733458ef8fa
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/watch.rs`

```text
```

## Current evidence

- `04_wiring/src/watch.rs:14` — `run_watch`.
- `04_wiring/src/watch.rs:113` — `write_watch_block`.
- `04_wiring/src/watch.rs:118` — `emit_watch_block`.
- `04_wiring/src/watch.rs:124` — `watch_line_at`.
- `04_wiring/src/watch.rs:128` — `watch_block_at`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/watch.rs`.

## Owned contract

- Own run_watch and local status/write/time/cycle helpers. Reuse the compile owner’s private observed helper with the session below, L3 watch/server operations and the L4 eviction wrapper (`00_nucleo/prompts/wiring/eviction.md`); no local polling, fingerprinting or HTTP algorithm.
- Reject stdout output. Serve semantic Html/Bundle unless no_serve, preserving port/no_reload/URL/PID staging and the existing event loop. Never infer HTML from an asset extension.
- Capture Instant before compilation; report status before diagnostics. Map elapsed/warnings/exit into typed WatchStatus with local time or UTC fallback. Compiling uses one LF, other blocks two; write_all then flush with existing propagated errors/stderr expect.
- Preserve dependency fallback then arm, successful publish/publish_directory or failed abandon/abandon_directory. Publication failure exits 2; HTTP state changes only on successful publish and the last disk/HTTP artifact survives errors.
- Keep evict(10) and wait(100ms) call placement. Published is privately imported by the parent from typst_infra::watch. Local helper tests characterize status/order, not a new watcher or timing policy.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Medição anterior à decisão de retenção ratificada

00_nucleo/diagnosticos/watch-retencao-2026-10-07.md registra HEAD/árvore/binários/UTC,
edições reais, recuperação e contagem dinâmica de aquisição por ciclo.
00_nucleo/diagnosticos/watch-validacao-2026-10-07/measurement.json mede validação por
descoberta versus aquisição completa no mesmo estado; controls.json testa adição,
remoção, substituição, mtime restaurado, links e configuração. São custos isolados,
não ganho de CLI ou prova de intenção upstream. FontBook/slots/cobertura devem
preservar seleção, conteúdo, diagnóstico e render na linguagem; duração/Arc são
mecânica. Inferimos benefício possível, refutado por custo, stale resources,
regressão ou retenção ilimitada. Fixtures/proveniência/limites constam do relatório.

### Decisão ratificada sob ADR-0127 §2.1/§2.2

Dono ratificou com Confirmo o pacote 00_nucleo/materialization/watch-catalogo-2026-10-07/manifest.json. Receipt: 00_nucleo/diagnosticos/watch-catalogo-execucao-2026-10-07/ratification.json. Cada owner continua 1:1.

## Lifetime da sessão

Criar Arc<FontCatalogSession> uma vez por chamada run_watch, fora do loop, sem
font discovery. Passar clones ao helper observado privado do owner compile em
cada ciclo, mantendo o CompileIntent e staging existentes. Fonte/reforço de erro
não requer reset de World: cada chamada continua construindo World novo.
Conservar exatamente status-before-diagnostics, Instant/timers, deps/fallback,
arm antes da publicação, evict(10), polling 100ms e estado HTTP do último sucesso.
Não adicionar fontes como evento observado, não mudar política de servidor, nem
alterar modo público/flags. Sessão termina com o watcher; sem retenção global.
Aceitação com processo real: edição main/import/read/PNG; erro, criação após
negativo e recuperação; fontes adicionadas/removidas/substituídas reconhecidas na
próxima recompilação. Medir startup separadamente de ciclos, CPU/latência/RSS e
contadores separados; comparar com o binário anterior congelado e vanilla pinado.
Se não melhorar de forma repetível ou houver regressão, não declarar otimização
concluída; conservar evidencia negativa e reavaliar caminho produtivo.

## Reuso sintático de watch — lote A

### Medição anterior à decisão

00_nucleo/diagnosticos/incremental-prioridade6-2026-10-08/before.json.gz,
measurement.json.gz, control-measurement.json.gz e count.json registram árvore,
HEAD, UTC, comandos e releases. Source::new reconstrói CST; World novo constrói
Source main/import a cada edição. O import mínimo já absorve recursos no eval_ms;
não atribuir toda a métrica a parsing. O contrato causal de Source/id/caminho vem
dos owners existentes; o reuso é mecânica e precisa preservar observáveis de língua.
Hipótese de benefício refutada por custo, diagnóstico, efeito, stale source,
geometria ou identificação diferente. Não inferir intenção upstream.

### Contrato ratificado

Criar Arc<SourceSession> uma vez por run_watch fora do loop, independente da
FontCatalogSession já ratificada. Passar clones ao helper privado de compile em
cada ciclo; World continua novo. A chamada watcher delimita lifetime/retensão.
Preservar arm-before-publication, staging/HTTP/último sucesso, error recovery,
watch deps/fallback, evict(10) e polling100ms. Não criar nova flag ou modo público.

Aceitação: RED→GREEN, contagem de produção separada de tempos, três rodadas CLI
pareadas contra release P5, startup/RSS e controles de invalidação/identidade/
efeitos/Unicode/diagnóstico/assets/fontes/geométricas. Resselo apenas deste owner,
build/suítes pertinentes e lint zero erros; não fechar incrementalidade completa.


## Expressões locais verificadas — Clippy 2026-10-16

### Medição anterior à decisão

04_wiring/src/watch.rs:94 (clippy::collapsible_else_if). O snapshot `00_nucleo/diagnosticos/clippy-expressions-2026-10-16/before.json.gz` conserva fontes/L0, HEAD, working tree não commitado, status, diffstat e UTC; initial-diagnostics.json conserva sugestões e spans. Os usos e tipos foram inspecionados antes da decisão. São representações Rust internas; nenhuma nova paridade upstream ou ganho de performance é alegado.

### Responsabilidade exclusiva e aceitação

Achatar somente else { if Bundle ... } para else if Bundle no descarte após falha. Preservar abandon_directory versus abandon, staging, publicação apenas no sucesso, retorno e estado anterior disponível.

Fluxo contínuo ADR-0127: sem API pública, default de produto, fase, dependência ou supressão novos. L0 precede código e resselo restrito. Preservar inputs e resultados dos controles existentes, sem relaxar tolerâncias. Gate dos lints selecionados RED→GREEN; executar suíte workspace, build, Clippy all-targets, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe Rust.


## Transporte de publicação em L3 — V12, 2026-10-16

### Medição anterior à decisão

O snapshot before.json.gz em 00_nucleo/diagnosticos/tekt-v12-2026-10-16 conserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada, UTC, fontes, L0, hashes, status e git diff HEAD --stat. O inventário anterior, conferido por hashes, emite 19 V12: um enum produtivo em 04_wiring/src/main.rs:22 e 18 declarações auxiliares de testes. O enum privado Published tem exatamente Html(String) e Bundle(Vec<BundleFile>); compile.rs:90/99/363/539/541 o produz e watch.rs:103/104 despacha os dois braços. O proprietário atual é wiring.md, que autoriza o carrier privado em L4: esta proposta atualiza explicitamente essa decisão, em vez de contrariá-la sem registro. É propriedade/representação Rust, sem nova alegação de paridade ou intenção upstream.

### Verificação comum

T1: preservar os controles reais de watch HTML e bundle HTTP/reload, sucesso/erro, no-serve/no-reload e último snapshot válido; testes p1485_watch_bundle_http_reload_e_preserva_snapshot_apos_erro e p1485_watch_flags_no_reload_no_serve_e_porta_ocupada em 04_wiring/tests/cli.rs, mais controles HTTP de 03_infra/src/watch.rs. T2: preservar compile de HTML/bundle, árvore completa, assets byte-exatos, stdout, warnings, exit e ausência de publicação em erro nos testes CLI existentes, sem editar esperados. T3: V12 RED real antes e zero produtivo depois; conferir por diff que só houve transferência do enum e ajuste do import, sem inferir modalidade por extensão. Verificar build, Clippy workspace/all-targets -D warnings e -D unfulfilled_lint_expectations, suíte workspace, formato, Rustdoc -D warnings e quatro índices, linhagem V1/V5/V15/V26 e reversa, SARIF e diff --check. Não escrever testes que apenas espelhem a sintaxe da mudança de owner. Execuções pesadas sequenciais sob MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1. Não publicar release.

### Responsabilidade exclusiva

O match estático Html/Bundle continua no mesmo local: Html chama publish_html e Bundle chama publish com os mesmos empréstimos. Não inferir formato do nome de arquivo e não mudar publicação, wait, cache, server, snapshots ou status.
