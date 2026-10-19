# L0 — compiler/eval/call_dispatch/call_spans
Hash do Código: 3cfa1922

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/call_dispatch/call_spans.rs`. No HEAD `d5c563dfab39f789e8e2f72c9a9364f5a761e3ca` e árvore limpa, `01_core/src/compiler/eval/call_dispatch.rs:69-389` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `lab/results/construction/diagnostics/atomizacao-call-dispatch-2026-09-30-before.json.gz`.

## Contrato

Capturar âncoras sintáticas para diagnósticos de chamadas float, math e html, preservando identidades nativas e ordem de escolha dos spans. Preservar assinaturas públicas, diagnósticos, prioridades de intercepção, rastros e semântica de argumentos especificados em `00_nucleo/prompts/core/compiler/eval/call_dispatch.md`. A extração permanece em L1 com despacho estático, sem I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1, os testes de call_dispatch e `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho.


## Decisões por cardinalidade e hexadecimal — V18, 2026-10-16

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado. before.json.gz em lab/results/construction/diagnostics/tekt-ranges-2026-10-16 conserva UTC, fontes/L0, hashes, Cargo manifests/lock/toolchain/config, status e git diff HEAD --stat. A validação anterior, conferida por hashes antes desta etapa, registrou seis V18: quatro em call_spans.rs:52/161/206/226 e dois em watch.rs:287/288. Os L0 atuais e os ramos foram lidos antes da decisão. É estrutura de decisões Rust; as mensagens/spans/rotas são observáveis a preservar, sem nova medição de paridade global.

A sonda isolada extrai exatamente decode_url_path da fonte e substitui somente sua closure digit no candidato. probe-details.json registra argv/rustc 1.92/SHA-256/UTC/retornos; probe-result.json registra 12 GiB, swap zero, um job e eventos. Compara todos os 256 bytes e 65.536 pares: os 22 bytes hexadecimais ASCII e 484 pares aceitos são iguais. Compara também as duas funções completas nas strings de pares e em 13 controles com saídas exigidas de UTF-8, query, segmentos ponto, truncamento, inválidos, NUL e backslash. Os números são desta sonda, não benchmark de HTTP/RSS. Nenhum dado deriva de main móvel do vanilla.

### Responsabilidade exclusiva

Nos quatro matches sinalizados, observar args.items.as_slice() diretamente: [] conserva o caso zero, [_] conserva o caso um e [_, _, ..] conserva o caso dois-ou-mais. Em FloatIsNanCallSpans::anchor manter guards !has_spread nos casos não vazios e o fallback args.span; os branches named/bound e sua ordem ficam intactos. Em P1293MathCallSpans::anchor alterar somente os matches de cardinalidade Attach, Mono e Script: [] retorna call, [_, _, ..] retorna a segunda âncora/fallback e [_] prossegue, sem mover as validações vizinhas. Não antecipar named de Attach nem atrasar named de Mono/Script. Não acessar posição que não exista, alterar captura/snapshot de Args ou tocar Binom/HTML. Preservar todos os corpos de resultados e assinaturas; o match passa a declarar a forma da coleção, não faixas numéricas de domínio. A lógica de diagnóstico continua exclusivamente neste owner.

### Aceitação e limites

Fluxo contínuo ADR-0127: correção interna de representação de decisão, sem assinatura/campo/trait público, comportamento por defeito, dependência, fase ou compatibilidade externos novos. L0 primeiro, testes funcionais antes da implementação e gate V18 RED real. Os novos testes devem passar no baseline: demonstram preservação, não fabricar uma falha de produto onde só havia aviso estrutural. O RED é o linter --checks v18 --fail-on warning; GREEN exige zero V18 e exatamente seis avisos Tekt a menos, sem avisos novos por regra.

T1: âncoras float para zero/um/dois/mais argumentos, bound/spread, named e falta de posição capturada; named prevalece, unbound vazio ancora call mesmo com spread, bound vazio/spread usa args.span. T2: Attach/Mono/Script preservam prioridade relativa de aridade, tipo, nome desconhecido e cramped, com e sem spread; Binom e HTML não mudam. T3: decoder reconhece todos os octetos percent-encoded ASCII válidos e mantém rejeições (NUL, backslash e UTF-8 inválido), case de hex, query, segmentos ponto, raiz e barra final. T4: controles existentes de diagnóstico real P1293 e rotas HTTP/CLI mantêm spans, mensagens, 200/301/404 e saída. T5: build, Clippy workspace/all-targets -D warnings com unfulfilled_lint_expectations como erro, fmt, suíte workspace, Rustdoc -D warnings com quatro índices, diff --check e linhagens estrita/reversa; 12 GiB, swap zero e um job. Fontes/L0 congelados durante comandos; resselo restrito aos dois pares.

Não renomear módulos para escapar da regra, adicionar expectativas/supressões, rebaixar o linter ou alterar esperados para mascarar divergência. Não alegar melhoria de performance ou leitura por IA não medida. Nenhum release.
