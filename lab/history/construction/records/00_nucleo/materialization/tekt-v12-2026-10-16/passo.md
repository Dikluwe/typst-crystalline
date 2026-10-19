# Passo V12 — separação de responsabilidades

Registro tático consolidado a partir de classification.json, items-provenance.json, proposta ratificada e l0-before-code.json. Não é Prompt L0. Os arquivos de evidência mantêm a ordem efetivamente executada e UTC; este registro não antecipa artificialmente sua própria criação.

1. Conferir o checkpoint FrameItem por todos os hashes e registrar RED dos 19 V12.
2. Ler os owners e separar enum produtivo de 18 declarações de teste.
3. Ratificar Published em L3 por ADR-0127; prescrever quatro L0 de produção e 22 L0 da separação de harness.
4. Transferir helpers por AST e Published por delta mínimo; resselo restrito, replay e Clippy all-targets.
5. Corrigir o único import não usado e conservar a primeira tentativa como evidência; congelar fontes/L0/configuração.
6. Build, formato, linhagem estrita/reversa, V12 zero, SARIF completo, suíte workspace e Rustdoc sob 12 GiB, swap zero e um job.
7. Complementar T1 por sonda real de watch HTML (publicação, atualização HTTP, retenção após erro e recuperação), sem escrever produto ou mudar esperados.
8. Emitir relatório, lista das correções, pendências e manifesto; sem commit, push ou release.

Estado em 2026-10-16T22:15:49.015490+00:00: validação final em andamento. A meta V12 é distinta dos 242 V16 ainda pendentes.

## Encerramento

Concluído em 2026-10-16T22:20:04.550757+00:00. V12 19→0, Clippy zero, 8622 testes aprovados/zero falhas/sete ignorados; sonda HTML adicional aprovada. Rustdoc e índices, build, formato e linhagens aprovados. Restam 242 V16, sem fechamento da meta geral. Relatório: `00_nucleo/diagnosticos/tekt-v12-2026-10-16/relatorio.md`. Manifesto integralmente conferido, SHA-256 `b0588cd9882813fb95af06cf1b33bbe09075e442cfef6b8b8690a20c5ff02cad`. Nenhum commit, push ou release.
