# P1327 — import sem efeito e ordem dos diagnósticos de eval

## Resultado

`import std`, aliases e bare imports de módulos ordinários agora emitem
`this import has no effect`, com o identificador fonte destacado, sem hints
ou trace. O módulo continua ligado sob o mesmo nome lexical; valores e
lookup não mudam. Field access, literal de arquivo, `as`, items, wildcard
e falhas anteriores ao binding não recebem esse aviso.

Quando o import funciona e uma expressão posterior falha, o aviso continua
presente. No comando `eval`, os erros de avaliação são apresentados antes
dos warnings, como no vanilla ratificado. Sucesso continua drenando warnings
antes da serialização. Compile, query, watch e o formatter não foram alterados.

## O achado que ampliou o passo

A primeira implementação (C1) passou nos seis testes novos e na suíte
completa, mas falhou em **24 de 336** comparações CLI: os blocos completos
estavam corretos, porém apareciam na ordem warning→erro. A causa era
`run_eval`, não o avaliador de imports. Assim, a hipótese inicial de que
somente o owner produtivo modules bastaria foi refutada.

O resultado vermelho permanece em `p1327-ab-cli-candidate.json` e
`p1327-ab-verdict.json`. A revisão `p1327-review-cli-failure.md` motivou
L0-first e manifesto R2 para o owner wiring, sem afrouxar o comparador.
O C2 move apenas a drenagem de errors para antes dos warnings em `run_eval`.
O binário C1 foi preservado em `/tmp/p1327-c1.ZvH24a/typst`.

## Verificação final

- RED real: seis testes executados, dois controles passaram e quatro
  falharam exclusivamente por avisos ausentes. GREEN dos mesmos seis testes.
- CLI original: **336/336** observações correspondem aos oráculos intactos.
  Suplemento independente com warnings antigos/múltiplos e serialização:
  **96/96**. Comparação integral de exit/stdout/stderr, sem normalização,
  nas ordens normal, repetida e invertida, nos quatro perfis de features.
- Workspace C2: **6.686 passaram, zero falhas, três ignorados**, incluindo
  as seis observações históricas P1305/P1306 com expectativas sucessoras
  independentes. Não se removeram testes nem se relaxou o helper de silêncio.
- Build release/locked, fmt e diff-check passaram. Lint geral:
  **zero erros, 240 warnings e 1.139 infos**; não é um repositório sem warnings.
  Gate estrito V5/V15/V26 sem violations. Linhagem recíproca dos três pares
  e integridade dos núcleos conferidas separadamente.

As 432 observações não são 432 alegações de igualdade com vanilla: os
controles de dívida exigem explicitamente preservação do baseline.
Perfis de features em eval não equivalem a testar o target HTML.

## O que continua faltando

Continuam fora: aviso de rename redundante, ampliação dos tipos importáveis,
erros/trace de import ainda divergentes e identidade nativa de math pendente
desde P1325. Em serialização raw, o erro `int` versus `integer` e a retenção
cristalina de warning anterior continuam dívidas medidas, preservadas pelo
contrato R2 do caminho Ok. Este passo não fecha paridade geral de imports,
CLI, diagnósticos ou linguagem.

## Proveniência e reprodução

HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
Os recibos guardam status, diff integral, `git diff HEAD --stat`, inventário
SHA-256 e horários antes/depois de cada execução. Base inicial:
`p1327-baseline.json`; sucessão causal: `p1327-r2-baseline.json` e
`p1327-r2-manifest.json`. Os únicos pares alterados neste passo são
modules, eval/tests (test-only) e wiring; alterações anteriores nos demais
owners e evidências históricas foram preservadas.

Vanilla: upstream/main ratificado **a51e02804**, `/usr/local/bin/typst`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
C2: `/tmp/p1327-target.k9Mq0s/release/typst`, SHA-256
`75e8b97b3788c0feaf457cb4c06b1c2c6735cff5ef3b9804a75ec57f8b148e31`.

Workspace final: `p1327-r2-workspace-tests.json`, SHA-256
`3867b5c75fd0d28eb2065974d094e1640a70860e56d7d4b56b61f531a9e63e6f`,
UTC `2026-09-09T11:12:42.770057+00:00`–`11:13:18.597704+00:00`.
As matrizes estão em `p1327-ab-cli-candidate-c2.json` e
`p1327-ab-r2-candidate.json`, acompanhadas pelos recibos de execução
`p1327-r2-cli-candidate.json` e `p1327-r2-cli-supplement.json`.
Comandos completos e fixtures estão nesses artefatos; gates usam
`CARGO_TARGET_DIR=/tmp/p1327-target.k9Mq0s`, `cargo build --release --locked`
e `cargo test --release --locked --workspace`.

Tekt A/B executado sem atestação técnica de isolamento e sem selo de
refinamento: autor independente congelou testes/oráculos antes dos candidatos;
revisor separado não editou produto ou testes. O relatório de revisão final
é `p1327-review-final.md`; o recibo agregado é `p1327-closure.json`.
O erro inicial de opção do lint e o ajuste puramente de formatação dos testes
permanecem registrados como instrumentação, não como RED semântico.
Nenhum stage, commit ou push realizado; nenhum passo seguinte iniciado.
