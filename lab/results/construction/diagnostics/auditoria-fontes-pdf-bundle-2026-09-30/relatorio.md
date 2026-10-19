# PDF isolado × PDF no bundle: seleção de fontes

## Proveniência

Medição em working tree não commitado, HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d` em 2026-09-30. `working-tree-stat.txt` lista os ficheiros alterados face a HEAD no instante do fecho; os seis pares desta frente têm hashes SHA-256 verificados na sessão. `results.json` contém hora UTC, SHA-256 do binário de teste e das fontes, comandos, hash das entradas, saídas, páginas, nomes das fontes e caixas de palavras. `probe.py` reproduz a matriz. Os resultados anteriores à correção vieram do binário release SHA-256 `2b366c2b7349c196c38bca83e00293ff0e11bb7979a5537b7ab5116ee6bee971`, sem alterações desta frente; os resultados arquivados são do build debug após a correção.

Hashes SHA-256 dos ficheiros no fecho (ordem: resources, pdf, bundle):

| Owner | Código | L0 |
|---|---|---|
| resources | `f53f3f30934d5a1ed9fe13248323104a381fd84b472c2c4a052a58aa379dcc05` | `02636842e35bb784614f1b3600488d5463156e2a6186859df4915a6aee62b5ee` |
| pdf | `b5dbed74c032522316ebb3946cd6a3f9e558f1564b78e49ff89f1ed3a68ed027` | `1d95d7a55c58703b17a172c9d286be86fc15e546909aae5f5b53ce1e96753614` |
| bundle | `757c42d781ff2d5e03595b04d5c808b5bb4d17f968fb150ebd248588b921334a` | `d474ba2231f14d784954a4a5c57e1e9886c9b61ec163a27d977bbbe68f70ea2b` |

## Medição antes da decisão

Em `pdf.rs:~430-540`, PDF isolado escolhia fallback com zero fontes, caminho single com uma fonte comum, multifont com várias ou VF com eixos não padrão. Em `bundle.rs:~475-535`, PDF interno usava multifont sempre. A matriz com as fixtures Noto Sans, Nimbus Sans e Ubuntu Sans Variable mostrou as mesmas páginas, fontes embutidas, texto e caixas de palavras em zero, uma, várias, VF padrão e VF weight 700. Um deslocamento inicial de `World` no controle de duas fontes veio de quebra de linha sob o wrapper `document`; foi refutado ao usar dois trechos `#text` explícitos no mesmo conteúdo. A escolha de função exportadora é mecânica e não prova desvio de linguagem (ADR-0107/0108); hashes PDF distintos não são critério de paridade.

Havia um desvio observável: com `TYPST_CRYSTALLINE_PYTHON` apontado para executável inexistente e pedido de weight 700 na Ubuntu Sans, ambos falhavam, mas o PDF isolado dizia `fonte variável 'ubuntu sans' requer instanciação, mas Python/fontTools não está disponível`, enquanto o bundle emitia a frase inglesa `variable font 'ubuntu sans' requires instantiation, but Python/fontTools is unavailable`. `resources.rs` repetia a frase portuguesa para SVG/PNG. Esta é mensagem de erro e, portanto, observável (ADR-0108). A hipótese de deriva por duplicação seria refutada se os três percursos delegassem o mesmo construtor; antes da correção, não delegavam.

## Correção

Os L0 de `resources`, `pdf` e `bundle` foram atualizados antes do código. `resources.rs` agora concentra um gate privado e preguiçoso: só sonda Python se um eixo pedido existir na fonte variável resolvida, preserva a primeira fonte aplicável e constrói o diagnóstico português vigente. PDF isolado, PDF interno e SVG/PNG usam esse gate. O despacho de zero/uma/várias fontes e as opções PDF permaneceram como estavam. Nenhuma API pública, default ou fase mudou; correção interna em fluxo contínuo da ADR-0127.

## Verificação

O teste novo falhou RED por helper ausente e passou GREEN; cobre eixo conhecido com instanciador indisponível e eixo desconhecido sem sondagem. `cargo test -q -p typst-infra --lib`: 1.140 passaram. `cargo build -q`: passou. CLI pós-build: os dois percursos agora falham com a mesma mensagem portuguesa. A matriz `results.json` passou nas cinco classes. `crystalline-lint --checks v5,v15 .`: passou, sem deriva nesses owners. `crystalline-lint .` global permanece falhando por 12 pins V26 de `paragraph-selection.toml` e um V2 em `01_core/src/compiler/math/layout/large_operator.rs`, ficheiros de outras frentes simultâneas. `git diff --check` no escopo passou.

## Classificação separada: `--pdf-standard` no bundle

`04_wiring/src/compile.rs:155-219` só resolve `standard_override` quando `OutputFormat::Pdf`; em bundle, passa `None` ao campo `pdf_standard_override`. `00_nucleo/prompts/shell/cli.md:98` regista explicitamente que esse override permanece ausente no bundle e requer decisão de escopo. Logo, não foi incluído nesta correção de fontes; seria uma decisão de contrato CLI/default, com gate ADR-0127 próprio. Os standards documentais de cada entrada PDF continuam a ser resolvidos em `bundle.rs`.
