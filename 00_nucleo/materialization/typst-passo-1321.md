# Passo 1321 — validar argumentos de CSV antes da leitura e do parsing

Estado: implementado e verificado em R2; PASS no recorte congelado.

Corrigir o primeiro cluster P1320: ausência/named source, erro vencedor de
cast/opções e rejeição causal de excedentes/desconhecidos. Owner inicial:
`00_nucleo/prompts/compiler/stdlib/loading.md` → loading.rs. Sem API Rust nova.

1. Preservar P1319/P1320, registrar baseline e confirmar fonte vanilla ratificada.
2. L0 primeiro; testes RED, expectativa de testes históricos alterada explicitamente.
3. A/B separado congela oráculos; revisor julga escopo e suficiência antes do patch.
4. Implementar no owner, sem tocar parser, resolução, Symbol ou outros loaders.
5. GREEN, workspace/build, lint, hashes bidirecionais explícitos, A/B e revisão final.
6. Relatório substantivo em diagnósticos com resultados, limites e pendências reais.

Regime A/B sem atestação técnica de isolamento. Root escreve L0/testes locais/
candidato; testador não lê produto; revisor não altera artefatos julgados.
Unknown obrigatório bloqueia. Um baseline, ajustes focais e um candidato final
normal/repeat/reverse; duas revisões sem ganho na mesma causa exigem reabertura.
Correção interna de paridade em fluxo contínuo ADR-0127, não mudança de pipeline.
Sem commit/push ou limpeza de temporários neste pedido.

## Reabertura R2

O A/B R1 refutou owner único: missing recebeu span da lista, não da chamada.
Preservar freeze/oráculos e recibo FAIL R1. Revisão confirmou fluxo contínuo
ADR-0127 para adicionar só native_csv ao transporte por identidade existente
no segundo owner compiler/eval/call_dispatch. Atualizar os dois L0 primeiro;
testar transporte em RED, refreeze normativo sem mudar expected, revisar,
implementar o mapeamento e medir focal missing/fronteiras antes do A/B final.
Loading continua dono da validação; nenhuma mudança de API/fase ou parser.

## Entrega

Validador CSV e transporte agregado R2 implementados. RED→GREEN nos dois
owners; A/B focal e integral aprovados, workspace/build/fmt/diff-check/lint e
linhagem bidirecional aprovados. Os resultados, proveniência, falha R1 e
lacunas remanescentes estão em `00_nucleo/diagnosticos/p1321-final-report.md`.
Oráculos originais preservados; sem alegação de paridade geral. Nenhum commit.
Parecer independente: `00_nucleo/diagnosticos/p1321-review-final-r2.md`.
