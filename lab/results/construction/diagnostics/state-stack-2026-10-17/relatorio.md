# Concatenação e pilha das travessias

Corrigida a concatenação que criava sequências encaixadas em blocos/for/while/+ e isolados os frames nativos das reconstruções pré-ordem e bottom-up. O ensaio original 64×2048 e Strong64 agora concluem com a pilha padrão. **O objetivo amplo permanece aberto:** paridade e os V16 anteriores ainda têm pendências. Nenhum commit, push, merge ou publicação de release nesta etapa.

## Proveniência

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, working tree não commitado. Congelamento `2026-10-18T00:59:07.101450+00:00`, entrega `2026-10-18T01:17:28.236003+00:00`. A lista exata de fontes, scripts, novos arquivos, status e `git diff HEAD --stat` está em [final-state.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/state-stack-2026-10-17/final-state.json.gz). [before-sources.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/state-stack-2026-10-17/before-sources.json.gz) recupera oito arquivos anteriores com SHA-256 idêntico ao checkpoint; [change.patch.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/state-stack-2026-10-17/change.patch.gz) separa este lote das correções A1. Fonte e L0 permaneceram congelados na validação.

Vanilla pin **bd5ebec285505889e8ff1d4f68804ab147a699f3**, árvore `cac836c07e4dec2a89219370428292393a3216b5`, executável `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. Tekt anterior `321de82f1d5485f077b4aa58558acc2b63d845d5847c53463e9ab28fd74b5d2b`; candidato debug `fcf804d80089371a87bdee05675ca64ba202667d5f1cf58777ffae0278147d99`; release `2ab1495d555ff4d61e67eb14bef67597f7f668a0e3bce3a8129623011443441c`. Strings de versão não são usadas como prova.

## Diagnóstico e correções

1. `join` e `+` construíam Sequence([prefixo, novo]) repetidamente. `repr` de quatro corpos [X] divergia do vanilla: três sequências encaixadas contra quatro filhos planos. A concatenação agora expande somente as sequências dos operandos imediatos. Mantém ordem, aliases, spans, filhos internos já sequenciais e wrappers opacos. Empty não vira filho; Text vazio continua filho real. Sequence de um filho conserva sua identidade na soma. `Content::sequence` público não mudou.
2. GDB encontrou SIGSEGV em `map_content_pre` durante a busca de tabelas, com 104664 bytes entre níveis consecutivos. A concatenação corrigida eliminou profundidade artificial, mas Strong64 legítimo ainda abortava. Reconstruções recursivas foram isoladas em funções genéricas estáticas FnOnce, com inline(never); a enumeração permanece exaustiva, sem vtable, thread, unsafe ou configuração nova de pilha.
3. Strong256 revelou outro aborto em `map_content_inner`, durante wrap_page_columns. O mesmo isolamento preserva a prova de mudança: None reutiliza o Arc da raiz; Some, mesmo igual, reconstrói o caminho. Source, transform_root/Located, vistas preparadas, ordem bottom-up e erros permanecem conforme os contratos existentes.

Os L0s foram atualizados antes de testes/código; houve seis REDs funcionais na concatenação e dois REDs por stack overflow nos testes isolados. Os mesmos testes passaram depois. Hashes ressellados após V15/V26 válidos. Só houve helpers internos; não mudaram campos, traits ou assinaturas Rust públicos, fases, flags ou limites de profundidade.

## Validação

Workspace: **8950 passaram, 0 falharam, 7 ignorados**. Doctests: **5 passaram**. Os dois testes isolados de pilha passaram com RUST_MIN_STACK=2097152; o CLI usa a pilha padrão registrada nas sondas, sem aumento. A configuração de 2 MiB existe somente no comando de teste.

| Caso CLI | Antes: exit | Debug atual | Vanilla | RSS debug atual (MiB) |
|---|---:|---:|---:|---:|
| original-64-2048 | 134 | 0 | 0 | 157.04 |
| larger-256-128 | 134 | 0 | 0 | 77.12 |
| while-64 | 134 | 0 | 0 | 52.29 |
| shape-for-256 | 1 | 0 | 0 | 50.93 |
| strong-16 | 0 | 0 | 0 | 51.07 |
| strong-256 | 134 | 0 | 1 | 54.22 |
| strong-64 | 134 | 0 | 0 | 50.85 |

Exit 134 nas execuções antigas é SIGABRT/stack overflow; não se confunde com diagnóstico de linguagem. O caso shape-for-256 usa contagens de marcadores em repr para não impor uma apresentação de linha única que o próprio vanilla rejeita. As primeiras fixtures inválidas foram preservadas como tentativas, sem crédito de paridade.

**Strong256:** o Tekt produz X; o vanilla rejeita com maximum show rule depth exceeded. Nenhum limite foi alterado para obter esse resultado. A diferença operacional está registrada e não foi contada como paridade de rejeição. Não se afirma profundidade ilimitada. A auditoria encontrou `Route` e seus guardas no código de world_types, mas seu L0 ainda incompleto; essa política precisa de auditoria própria, não de um limite novo no mapper.

Os 113 controles A1 foram repetidos: nenhuma nova falha fora das exceções nominais anteriores. Mensagens e texto permanecem auditados separadamente do exit. Pendências anteriores não foram silenciadas.

| Gate | Exit | Comando |
|---|---:|---|
| build | 0 | `cargo build --offline --locked --workspace` |
| clippy | 0 | `cargo clippy --offline --locked --workspace --all-targets -- -D warnings -D unfulfilled_lint_expectations` |
| probe | 0 | `python3 /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/state-stack-2026-10-17/probe.py complete` |
| state-extended | 0 | `python3 /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/state-stack-2026-10-17/state-extended.py` |
| extra-probe | 0 | `python3 /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/state-stack-2026-10-17/extra-probe.py` |
| bounded-stack | 0 | `env RUST_MIN_STACK=2097152 cargo test --offline --locked -p typst-core --lib bounded_stack_ -- --test-threads=1` |
| stress | 0 | `python3 /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/state-stack-2026-10-17/stress.py final` |
| morphology | 0 | `python3 /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/state-stack-2026-10-17/morphology.py after` |
| target-controls | 0 | `python3 /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/state-stack-2026-10-17/target-controls.py` |
| runtime-memory | 0 | `python3 /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/state-stack-2026-10-17/runtime-memory.py` |
| probe-audit | 0 | `python3 /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/state-stack-2026-10-17/audit-probes.py` |
| workspace | 0 | `cargo test --offline --locked --workspace --lib --bins --tests --no-fail-fast -- --test-threads=1` |
| doctest | 0 | `cargo test --offline --locked --workspace --doc --no-fail-fast -- --test-threads=1` |
| fmt | 0 | `cargo fmt --all --check` |
| rustdoc | 0 | `cargo doc --offline --locked --workspace --no-deps` |
| lineage | 0 | `crystalline-lint . --checks v1,v5,v15,v26 --fail-on warning` |
| reverse | 0 | `python3 tools/ci/check_reverse_lineage.py` |
| architecture | 1 | `crystalline-lint . --format sarif` |
| lint-diff | 0 | `python3 /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/state-stack-2026-10-17/lint-diff.py` |
| diff | 0 | `git diff --check` |

Tekt: `{'V16:error': 106, 'V19:note': 1106, 'V20:note': 874}` → `{'V16:error': 106, 'V19:note': 1106, 'V20:note': 874}`. Nenhum erro/warning novo no diferencial por regra/consumer/mensagem, ignorando deslocamentos de linhas. O gate global continua exit 1 pelos erros V16 anteriores; não foi relaxado. Notas V19/V20 são informativas e têm política nos L0s; não houve supressão genérica.

## Build otimizado e RAM

Build release local e três controles completos passaram, sem publicar artefatos. Receita/ambiente/horários/SHA em [release-result.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/state-stack-2026-10-17/release-result.json.gz).

| Caso release | Exit | RSS (MiB) | Texto |
|---|---:|---:|---|
| original-64-2048 | 0 | 137.08 | A |
| strong-64 | 0 | 28.50 | X |
| strong-256 | 0 | 30.34 | X |

Todas as execuções pesadas usaram **MemoryMax 12 GiB, MemoryHigh 8 GiB, swap zero, um job Cargo e execução sequencial**. Pico agregado da etapa: **3.917 GiB** (4205498368 bytes). Eventos acumulados: `{'low': 0, 'high': 0, 'max': 0, 'oom': 0, 'oom_kill': 0, 'oom_group_kill': 0}`. Os abortos RED de pilha são distintos de OOM. [resources.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/state-stack-2026-10-17/resources.json.gz) conserva cada execução, inclusive falhas/tentativas.

RSS de processo e pico de cgroup de build são medidas diferentes. As amostras CLI são únicas, com perfis distintos quando comparadas ao vanilla release; não demonstram ganho universal. O caso grande de State ainda usa muito mais memória que o oráculo. Value::Array e clones de payloads merecem medição/proposta próprias; esta etapa não mudou seus contratos. Concatenação sobre Arc<[Content]> ainda copia o prefixo e não foi declarada linear.

## Pendências preservadas e próximas frentes

- Formalizar/auditar o L0 de Route/guardas em [world-types.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/world-types.md); a fonte contém contratos que a spec histórica de stubs não descreve completamente. Nenhum desses tipos/limites foi alterado.
- Reflexão de Sequence/Empty: children, fields e func ainda divergem nas sondas; são correções separadas de linguagem, não diferenças justificadas pela representação Rust.
- Morfologia de Styled observada nas sondas, e as pendências A1 de Box.body, Equation aninhada, Ref/math/StateDisplay legado no HTML e mensagens continuam abertas.
- Medir concatenação em volumes maiores e possíveis acumuladores transitórios para evitar cópia repetida do prefixo. Não trocar o carrier público ou introduzir cache global sem proposta/L0 e gate aplicável.
- Continuar os V16 restantes e os lotes do inventário de paridade. O relatório não encerra o objetivo Faça todas as correções.

Fontes principais: [join.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/operators/join.rs), [arithmetic.rs](/repos/Antigravity/typst-crystalline/01_core/src/compiler/eval/operators/arithmetic.rs), [traversal.rs](/repos/Antigravity/typst-crystalline/01_core/src/entities/content/traversal.rs). [evidence.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/state-stack-2026-10-17/evidence.json.gz), [final-state.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/state-stack-2026-10-17/final-state.json.gz), [lint-diff.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/state-stack-2026-10-17/lint-diff.json.gz) e [manifest.sha256](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/state-stack-2026-10-17/manifest.sha256) preservam os registros de fechamento.
