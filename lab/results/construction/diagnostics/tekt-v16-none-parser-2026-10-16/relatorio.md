# V16 — None/prelude e validação do linter

Estado: etapa concluída. Branch publicada com autorização do usuário; pin aplicado e revalidado no working tree do Typst. Objetivo geral de correções continua aberto.

## Resultado

O commit publicado `12797073972e375d9ab078cf6750c8b30b71e2b9` (árvore `7ee2488c8073efd85505b0a985a3361a3546e996`, branch `codex/v16-none-prelude`) corrige a classificação de `None`/`r#None` como binding genérico. No mesmo snapshot Typst, V16 **253 → 244**, com nove remoções e zero alterações nos demais diagnósticos: V19 permanece **888 notas** e V20 **871 notas**. O gate global continua falhando pelos 244 V16 restantes. O pin nos arquivos locais do CI agora é `12797073972e375d9ab078cf6750c8b30b71e2b9`; a validação após aplicação confirmou os mesmos 244 V16 e demais diagnósticos idênticos. Não houve commit/push do Typst, portanto não se afirma execução de um workflow remoto deste projeto.

A [lista dos nove casos](lista-correcoes.md), os SARIF [anterior](consumer-old.log) e [novo](consumer-new.log), e [consumer-result.json](consumer-result.json) conservam mensagens, níveis e localizações.

## Correções

- Parser: extrair o binding uma vez; reconhecer None do prelude sem isentar nomes maiúsculos arbitrários. `_`, bindings comuns/raw e desativação local do prelude continuam V16. Atributos internos/externos, herança no mesmo arquivo, cfg_attr conservador, fronteiras de módulos, comentários/literais e guards são testados. Helpers privados, sem contrato público, dependência, I/O ou nova supressão.
- V19: um grupo novo legítimo (`line_comment | block_comment`) preservado e classificado individualmente no manifesto 0037; ambas as alternativas são exercitadas. O ratchet exato permanece ativo, zero V16/V17 no próprio linter. Auto-lint integral: 74 V19 e 17 V20, somente notas.
- Fixtures: quatro `.gitkeep` conservam diretórios vazios necessários em export/checkout novo. A falha anterior foi reproduzida também com o binário base; harness e expectativas preservados.
- Rustdoc: 20 erros de apresentação corrigidos em 11 consumers e respectivos L0. Somente doc-comments e metadados mudaram nesses consumers; código executável foi comparado e preservado. Nomes de tabelas, exemplos de citações e tipos usam código literal, sem links/HTML falsos.

O [patch completo](candidate.patch) inclui 30 arquivos: 12 consumers, seus 12 owners L0, dois documentos do inventário e quatro marcadores de diretório. [package-result.json](package-result.json) lista todos os caminhos/hashes e comprova o escopo. O código e os prompts produtivos do Typst permaneceram intactos. No consumidor, somente `.github/workflows/ci.yml` e `tools/ci/README.md` receberam as quatro substituições de pin aprovadas. O checkout canônico do linter permaneceu intacto; o commit foi preparado em cópia isolada e publicado na branch aprovada.

## Validação e recursos

- RED: seis falhas reproduzidas e três controles negativos passaram antes da implementação; [red-tests.log](red-tests.log).
- GREEN: nove testes específicos; suíte completa do linter **1146 passaram, zero falhas, 7 ignorados preexistentes**; [green-suite.log](green-suite.log).
- Build, Clippy all-targets com `-D warnings`, Rustdoc com `-D warnings`, fmt e V1/V5/V15/V26 estritos passaram; [green-result.json](green-result.json). Rustdoc tinha 20 erros antes, preservados em [attempt-doc-green-doc.log](attempt-doc-green-doc.log), e agora passa.
- Consumidor: 16 testes Python do CI e dois testes Rust de integração passaram com o candidato no PATH; linhagem direta estrita/reversa e diff-check passaram. Comparação SARIF integral removeu somente nove V16 e manteve exatamente todos os outros resultados.
- Limite verificado por cgroup: **12 GiB**, swap **0**, Cargo **1 job**; pico registrado da etapa **5433348096 bytes (5.060 GiB)**. Sem OOM, OOM kill ou eventos de atingir o teto. [Registros de recursos](green-resources.json) e todos os ensaios em [review-state.json.gz](review-state.json.gz).

## Proveniência e entrega

Typst: HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree não commitado**. [before.json.gz](before.json.gz) conserva a lista exata `git diff HEAD --stat`, status, hashes completos de fontes/prompts/configuração e UTC; [review-state.json.gz](review-state.json.gz) conserva o estado após validação. As fontes permaneceram congeladas durante a medição. A data do diretório é 16/10 no fuso America/Sao_Paulo; horários UTC nos registros podem ser 17/10.

Linter base: `e53de6fd2bff9f3a87a3e8a076f5e470ce4bb7a0`. Binário base SHA-256 `9bc3a8521bc7db1dc0afffa04fc5d24012cf7ffbd5376bb8e9729242af2c40aa`. Candidato SHA-256 `e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2`. Fontes compiladas são byte a byte iguais ao [arquivo exportado](candidate-source.tar.gz) e ao commit local; [bundle](candidate.bundle) conserva o commit com o base como pré-requisito. Os hashes bidirecionais dos owners editados foram verificados.

O usuário autorizou a revisão de manifesto SHA-256 `fd4f7669549b54e80f80ca2cfc781f32c0e3179f5db28f1e07a2d0b607db8068`. A [revisão aprovada congelada](approved-review.tar.gz) e seu [manifesto original](review-manifest-sha256.json) preservam exatamente os artefatos analisados. [publication-result.json](publication-result.json) comprova push da branch, conferência por ls-remote e fetch independente do commit e árvore no GitHub, antes de aplicar as quatro referências do [patch aprovado](proposed-pin.patch). [post-pin-result.json](post-pin-result.json) comprova revalidação após aplicação. Não houve merge, release ou commit/push do Typst. A branch publicada está em [Dikluwe/tekt-linter](https://github.com/Dikluwe/tekt-linter/tree/codex/v16-none-prelude). Os binários de validação são identificados por caminho e SHA-256 nos registros; não foi substituído o binário genérico instalado no sistema.

## Limites e próximos lotes

O parser continua local ao SourceFile: não expande macros nem resolve aliases/tipos ou ancestrais de módulos em arquivos externos. O snapshot Typst foi conferido sem no_implicit_prelude. Não é prova geral de resolução de nomes Rust. Some(_) e outros catchalls internos exigem análise própria.

O objetivo geral segue aberto: 244 V16 ainda a analisar/corrigir, além da auditoria dos casos latentes. O próximo lote já foi medido: quatro decisões de Value em delimitadores matemáticos/vetores e presença de stroke de matriz, três owners, sem mudança pública. As listas não autorizam supressões globais nem a remoção de V19 informativo.

A [lista completa de pendências](lista-pendencias.md) conserva todas as 244 emissões por arquivo/linha, além de T027 e T158/T160 latentes. O fechamento desta etapa não declara encerrado o objetivo geral.
