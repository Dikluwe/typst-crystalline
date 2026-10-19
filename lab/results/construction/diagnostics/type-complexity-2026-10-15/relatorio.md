# Correções de type_complexity — 2026-10-15

Etapa concluída: 30 avisos de `clippy::type_complexity` tratados, sendo 27 por alteração de representação/nomenclatura e três por `#[expect]` pontual com justificativa no L0. O inventário Clippy passou de 174 para 144 avisos, sem avisos novos de outras regras. Isso não encerra a dívida de Clippy/Tekt nem torna todo o CI livre de avisos.

## Leitura humana e por IA

Os nomes tornam os dados locais identificáveis: `RowCell` distingue quatro índices/spans antes posicionais; `EvaluatedPass`, `PreparedBundle` e `PagedCompilation` nomeiam resultados e evitam confundir campos. `TiffReaders` nomeia leitores de endianness e elimina duas seleções duplicadas e um boolean sem uso. As 21 repetições do tipo de fontes usam o alias canônico `FontKey`, já existente.

Não houve medição de desempenho, precisão ou custo de leitura de IA. A justificativa é semântica explícita e consistência no código, sem afirmar ganho quantitativo para modelos. As três exceções preservam contratos já legíveis: `SYM_GROUPS` (nome/base/factory), `native_fn_addr` (assinatura nativa explícita) e `ACCESSIBLE` (tabela de standards/labels em L4). Não há supressão no crate/módulo ou mudança de limiar; o gate de expectativas não cumpridas também passou.

## Escopo e contratos

Os 30 casos estão na [lista completa](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/type-complexity-2026-10-15/lista.md), no [CSV](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/type-complexity-2026-10-15/cases.csv) e em [JSON](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/type-complexity-2026-10-15/cases.json), com fonte anterior, fonte atual, L0 proprietário, ação e justificativa. Foram atualizados 15 owners e seus 15 consumers, incluindo três consumidores indiretos do resultado privado do pipeline. O [diff da etapa](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/type-complexity-2026-10-15/stage.diff) compara contra o estado exato de entrada, preservando as alterações anteriores da working tree.

Os L0 foram editados antes da implementação e resselados de modo restrito; cada exceção foi prescrita nominalmente. Os registros novos são privados ao código que os usa. A comparação das 80 funções públicas nos arquivos afetados confirma assinaturas equivalentes após expandir o alias FontKey preexistente e inalterado ([prova](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/type-complexity-2026-10-15/api-equivalence-final.json)). Não foram criados tipos/campos públicos nem alterados defaults ou fase do pipeline. Os 57 casos de `too_many_arguments` permanecem no estágio de auditoria anterior, sem materialização neste lote.

## Validação

O gate RED de type_complexity falhou antes da edição. O GREEN passou com `-D clippy::type_complexity -D unfulfilled_lint_expectations` ([recibo](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/type-complexity-2026-10-15/green-clippy-final-result.json)). As tentativas intermediárias encontraram um retorno de documento vazio e um consumidor de teste ainda em tupla; ambos migrados para o registro privado conservando valores e assertions. O primeiro inventário completo encontrou dois imports redundantes, removidos antes da validação final; os logs dessa tentativa permanecem em `validation-initial/`.

| Verificação | Resultado | Comando |
|---|---|---|
| build | passou | `cargo build --offline --locked` |
| clippy | passou | `cargo clippy --offline --locked --workspace --all-targets --message-format=json -- --cap-lints warn` |
| fmt | passou | `cargo fmt --check --all` |
| lineage | passou | `/tmp/tekt-linter-ci-target/debug/crystalline-lint . --checks v1,v5,v15,v26 --fail-on warning` |
| reverse | passou | `python3 tools/ci/check_reverse_lineage.py` |
| architecture | passou | `/tmp/tekt-linter-ci-target/debug/crystalline-lint . --format sarif` |
| workspace | passou | `python3 /tmp/tekt-types-suite.py` |
| rustdoc | passou | `cargo doc --offline --locked --workspace --no-deps` |
| diff | passou | `git diff --check` |

Suíte completa: **8593 testes passaram, 0 falharam e 7 foram ignorados**; nenhum esperado foi relaxado e nenhum teste artificial de alias foi criado. O teste consumidor do resultado paginado foi adaptado, conservando programas, tamanhos e assertions. O inventário final tem **0 type_complexity e 144 avisos Clippy**, com delta exclusivamente de −30 nessa regra. A linhagem estrita V1/V5/V15/V26 e a reversa passaram. O Tekt completo registrou {"warning": 380, "note": 1668}; avisos por regra: {"V16": 327, "V17": 28, "V18": 6, "V12": 19}. Sem novos avisos arquiteturais e sem erros; os demais avisos continuam pendentes.

Rustdoc passou com `RUSTDOCFLAGS="-D warnings"`; os quatro índices HTML foram confirmados depois da suíte. As fontes e os L0 permaneceram imutáveis durante a validação. [Comandos, horários, estado e resultados](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/type-complexity-2026-10-15/validation-details.json), [suíte](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/type-complexity-2026-10-15/workspace/workspace-result.json), [estado final](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/type-complexity-2026-10-15/final-state.json).

## Memória e proveniência

A validação foi executada com **MemoryMax=12 GiB, MemorySwapMax=0 e CARGO_BUILD_JOBS=1**. Pico total do cgroup: **10.183 GiB (10934267904 bytes)**, incluindo cache de arquivos; eventos OOM=0, OOM-kill=0. [Recibo do serviço](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/type-complexity-2026-10-15/validation-result.json). O limite cobre os processos da etapa, não outros aplicativos do host.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Origem em `2026-10-15T17:34:04.463231+00:00`; validação final entre `2026-10-15T17:54:15.136022+00:00` e `2026-10-15T18:04:32.994199+00:00` (UTC). O [snapshot inicial](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/type-complexity-2026-10-15/before.json.gz) e os snapshots de validação incluem hashes e a lista exata de arquivos alterados (`git diff HEAD --stat`), além do estado Git. A suíte verifica o vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, e usa o linter CI pinado em `49c6bb0e584104a0c89270431d8402f5c6c61150`.

Os scripts de execução estão preservados em `workers/` (a receita original usa caminhos temporários `/tmp` e as mesmas ferramentas já instaladas). [Manifesto SHA-256](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/type-complexity-2026-10-15/manifest.sha256). Sem commit, push ou publicação de release.
