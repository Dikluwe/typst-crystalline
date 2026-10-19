# Clippy e Tekt — triagem e primeiro lote local — 2026-10-15

O primeiro lote removeu sete avisos Clippy: **181 → 174**. Os **380 avisos Tekt** permanecem inventariados, sem erros arquiteturais; há também **1.668 notas**, contadas à parte. A lista completa, com IDs, regra, arquivo, linha e lote, está em [lista.md](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/avisos-clippy-tekt-2026-10-15/lista.md); [inventory.csv](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/avisos-clippy-tekt-2026-10-15/inventory.csv) e [inventory.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/avisos-clippy-tekt-2026-10-15/inventory.json) incluem o owner L0.

## Ordem escolhida

Clippy local → decisões Tekt → estrutura e contratos.

| Ordem | Escopo restante | Avisos |
|---|---|---:|
| 1 | Clippy local: 25 inicializações após Default e 36 outras simplificações | 61 |
| 2 | Tekt: V16 (327), V17 (28), V18 (6) | 361 |
| 3 | Estrutura: argumentos (57), tipos complexos (30), enums grandes (5), uso/heap/chaves (10), V12 (19) | 121 |
| 4 | API/traits: should_implement_trait (5), new_without_default (3), inherent_to_string (3) | 11 |

As contagens são ocorrências de diagnóstico, não número de defeitos funcionais demonstrados. A triagem não autoriza supressões globais nem renomear APIs apenas para satisfazer sugestões. Mudanças de contrato público exigem proposta concreta pelo gate ADR-0127.

[triage-notes.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/avisos-clippy-tekt-2026-10-15/triage-notes.json) destaca nove casos que pedem leitura de intenção/tipos: coleções de intervalos, retenção causal no estabilizador e métodos next que não retornam Option. As sugestões correspondentes não são substituições automáticas seguras.

## Alterações do lote

- `export/subset.rs`: binding de erro usado somente em testes renomeado para `_error`, preservando o diagnóstico e retorno None.
- `export/svg/glyphs.rs`: coordenada y imutável.
- `export/stream/draw.rs`: removidas duas referências redundantes ao escrever o mesmo envelope semântico.
- `pipeline/resources.rs`: cópia direta de FontVariant, que implementa Copy.
- `shaper/font_selection.rs`: argumento já emprestado passado diretamente e array fixo para os mesmos casos de teste.

Cinco owners L0 foram atualizados antes do código e resselados com seus consumers. Não se criaram testes de espelho para essas mudanças de expressão; os testes funcionais existentes e seus esperados foram preservados. [stage.diff](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/avisos-clippy-tekt-2026-10-15/stage.diff) contém somente o delta desta etapa sobre a árvore já modificada.

## Triagem Tekt

- **V16 — 327:** braços genéricos podem ocultar variantes futuras, mas cada enum e seu default precisam de análise. Um wildcard isolado não prova falha semântica.
- **V17 — 28:** guards compostos; avaliar decisão e precedência antes de desdobrar.
- **V18 — 6:** quatro ranges de aridade em call_spans.rs e dois de parsing hexadecimal de URL em watch.rs. Os dois últimos são candidatos a revisão do escopo lexical da regra, não defeitos funcionais confirmados.
- **V12 — 19:** um enum Published em código produtivo de L4; os outros 18 diagnósticos apontam para auxiliares sob `04_wiring/tests/`. Conferir a aplicação da regra aos testes antes de migrá-los de camada.
- **Notas — 1.668:** V20=872, V19=770 e V16=26; não incluídas nos avisos acima.

## Validação e memória

A execução de 2026-10-15T16:02:07.424960+00:00 a 2026-10-15T16:12:37.895070+00:00 usou **MemoryMax=12 GiB**, **MemorySwapMax=0** e **CARGO_BUILD_JOBS=1**. Pico do cgroup: **12.00 GiB**, incluindo cache de arquivos; eventos OOM e OOM-kill: **0**. É o teto agregado do serviço de validação, não de outros aplicativos abertos no computador.

A amostra de 2026-10-15T16:08:06.637275+00:00 registrou 11.01 GiB de cache de arquivos e 0.25 GiB de memória anônima. O teto foi atingido e houve recuperação de cache; a amostra não reproduz a pressão anônima do incidente anterior de rustdoc. As amostras completas estão em memory-samples.json; o processo lançador fora do serviço está identificado por seu cgroup e não entra no pico do worker limitado.

Build, Clippy all-targets para inventário, formato, linhagem estrita V1/V5/V15/V26, linhagem reversa, relatório arquitetural, suíte workspace, Rustdoc com `-D warnings` e diff-check passaram. A suíte registrou **8593 aprovados, 0 falhas e 7 ignorados**; fontes e L0 permaneceram imutáveis durante a validação. Clippy foi executado com cap-lints=warn para enumerar toda a dívida; **o gate estrito Clippy da CI ainda não está verde**, pois restam 174 avisos.

Rustdoc foi executado após a suíte para restaurar os quatro índices HTML que os testes de contrato JSON podem substituir. O cache de busca antigo permaneceu em quarentena; nenhuma limpeza geral de target foi feita.

## Proveniência e limites

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, branch Tekt, **working tree não commitado**. [before.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/avisos-clippy-tekt-2026-10-15/before.json.gz) registra UTC, status, lista exata via git diff HEAD --stat, fontes/owners e hashes de 2070 arquivos. Os avisos iniciais vêm da execução Clippy em 2026-10-14T23:47:46.509032+00:00–23:48:34.438949+00:00 e Tekt em 23:48:43.101012–23:48:47.560676+00:00, reutilizadas somente após confirmar todos os hashes; a medição final foi refeita. [post-result.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/avisos-clippy-tekt-2026-10-15/post-result.json) e [workspace-pre.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/avisos-clippy-tekt-2026-10-15/workspace/workspace-pre.json.gz) conservam proveniência final, comandos, ambiente, horários, diffstat, hashes e memória. [summary.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/avisos-clippy-tekt-2026-10-15/summary.json) consolida o resultado.

Esta entrega fecha a triagem e o primeiro lote local de sete avisos. A correção dos demais está organizada na ordem escolhida, com 554 ocorrências pendentes; não certifica conclusão de toda a limpeza, publicação ou release.
