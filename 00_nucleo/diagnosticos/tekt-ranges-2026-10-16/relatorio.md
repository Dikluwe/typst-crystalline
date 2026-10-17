# V18 — cardinalidade e hexadecimal: etapa concluída

Tekt **380 → 374 avisos**: os seis V18 foram corrigidos, sem supressões ou avisos novos por regra. **Clippy permanece zero**, inclusive workspace/all-targets com -D warnings e expectativas não cumpridas como erro. Dois consumers e seus dois L0 foram atualizados antes do código e resselados de modo restrito. [stage.diff](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-ranges-2026-10-16/stage.diff) · [code-guard.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-ranges-2026-10-16/code-guard.json).

## O que mudou

Em FloatIsNanCallSpans e nos ramos Attach/Mono/Script de P1293MathCallSpans, os quatro matches agora observam a forma de args.items.as_slice(): vazio, um item ou dois-ou-mais. Os corpos de seleção dos spans permanecem. Named/bound/spread e sua prioridade, fallback de posição ausente, erros/tipos vizinhos e as decisões Binom/HTML ficaram intactos. A forma da coleção expressa a cardinalidade diretamente, sem faixas numéricas em match de domínio.

No decoder privado do servidor de preview, somente a closure de dígito hexadecimal manual foi substituída por char::from(value).to_digit(16).map(... as u8). Permanecem lowercase, limites e combinação dos bytes, UTF-8, query, normalização, NUL/backslash/above-root, barra final e decodificação única. Sem helper/API nova, I/O adicional, mudança de fase/default ou política de rotas; HTTP/SSE/publicação não foram alterados.

## Medição e RED→GREEN

A sonda extrai a função decoder original completa e cria o candidato com uma única substituição. Todos os **256 bytes** e **65.536 pares** coincidem: **22 bytes** e **484 pares** reconhecidos como hexadecimais; 13 controles de rotas coincidem com saídas exigidas. É evidência de equivalência local, sem nova prova de paridade global, benchmark de HTTP, ganho de memória ou leitura por IA. [probe-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-ranges-2026-10-16/probe-details.json).

Antes da implementação passaram os quatro novos controles funcionais (dois de spans, dois de decoder). Eles protegem o comportamento existente; não foi criada uma falha artificial de produto. O gate **--checks v18 --fail-on warning** falhou com os seis avisos esperados. Após a correção, o mesmo gate passou sem V18. [baseline-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-ranges-2026-10-16/baseline-details.json) · [baseline-red.log](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-ranges-2026-10-16/baseline-red.log) · [validation-v18.log](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-ranges-2026-10-16/validation-v18.log).

Os testes cobrem cardinalidades 0/1/2/4 com named/bound/spread, falta de posição, diferenças de precedência Attach/Mono/Script/cramped, octetos percent-encoded upper/lower, ASCII e rejeição UTF-8, e rotas com raiz, query, traversal, truncamento, percent inválido, NUL/backslash e dupla codificação. As expectativas antigas não foram alteradas. Fluxo contínuo ADR-0127: manutenção interna, com L0 primeiro, sem contrato público ou compatibilidade nova.

## Validação e recursos

Suíte workspace: **8615 aprovados, 0 falhas, 7 ignorados**. Build, Clippy normal e estrito, formato, V18, linhagem estrita V1/V5/V15/V26 e reversa, Rustdoc -D warnings com quatro índices HTML e diff --check passaram. [validation-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-ranges-2026-10-16/validation-details.json) · [strict-details.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-ranges-2026-10-16/strict-details.json) · [workspace/workspace-result.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-ranges-2026-10-16/workspace/workspace-result.json).

Tekt restante: {'V16': 327, 'V17': 28, 'V12': 19}, sem erros. [lista-pendencias.md](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-ranges-2026-10-16/lista-pendencias.md). A meta geral continua aberta: 374 avisos ainda exigem correção/decisão e prescrição nos owners.

Serviços pesados sequenciais: **12 GiB**, swap zero e CARGO_BUILD_JOBS=1. Pico **12.000 GiB** do cgroup, incluindo cache; max=237, OOM=0, OOM-kill=0. [validation-result.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-ranges-2026-10-16/validation-result.json).

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Snapshot 2026-10-16T18:35:00.760074+00:00; validação 2026-10-16T18:41:53.194952+00:00–2026-10-16T18:53:18.798330+00:00 UTC, fontes/L0 e configurações congelados durante os comandos. [before.json.gz](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-ranges-2026-10-16/before.json.gz) · [final-state.json](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-ranges-2026-10-16/final-state.json) guardam código/L0/config/manifests/lock, hashes, status e git diff HEAD --stat. Linter CI pin 49c6bb0e584104a0c89270431d8402f5c6c61150 com hash de binário reconfirmado; vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea conferido pela suíte. Scripts em workers/; [manifest.sha256](/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/tekt-ranges-2026-10-16/manifest.sha256). Sem commit, push ou release.
