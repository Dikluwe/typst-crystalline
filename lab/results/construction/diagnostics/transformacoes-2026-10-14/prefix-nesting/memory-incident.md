# Incidente de RAM — 2026-10-14

O kernel registra três global OOM às20:00:16,20:04:05 e20:07:52 (America/Sao_Paulo).
Os processos encerrados foram rustdoc, com anon-rss46.236.912,44.668.680 e45.921.636KiB,
respectivamente: cerca de44,1/42,6/43,8GiB. As retomadas da documentação não concluíram;
não são provas de rustdoc aprovado. A interrupção dos jobs foi confirmada por ausência
de PID/resultado final. A identificação da causa vem do kernel, não da ausência de logs.

Na medição às20:09, o computador tinha62GiB de RAM,17GiB usados e44GiB disponíveis.
Não havia swap. Pressão de memória nos minutos anteriores foi registrada. O consumo
atual baixo não refuta os picos anteriores. Fontes/L0 permanecem iguais ao snapshot de
validação. Fonte completa, horário UTC, HEAD/status/stat e métricas: memory-incident.json.

No diagnóstico inicial, a repetição da documentação sem limite foi suspensa e a checagem
ficou pendente até execução concluída sob limite próprio. O reparo e a execução aprovados
estão registrados abaixo; a configuração de memória do host permaneceu intacta.

Proteção operacional verificada: systemd-run --user com MemoryMax=12G e
MemorySwapMax=0 executou o comando de controle com exit 0. O worker desta
etapa agora entra nesse limite antes de iniciar validações; Cargo usa um job.
As configurações do host e dos outros programas permanecem intactas.

## Investigação e reparo sob limite

A sonda workspace-b localizou o crescimento na documentação privada do
binário typst em 04_wiring, limitada a 12 GiB. A mesma fonte/argumentos com
saída vazia passou; copiar apenas o antigo search.index reproduziu o OOM
local. O índice foi preservado em tar.gz e os artefatos/fingerprints de doc
foram colocados em quarentena, sem editar fonte produtiva ou API.

Regeneração integral 23:45:40Z–23:45:59Z: exit 0, source_guard true, quatro
páginas raiz HTML presentes e pico observado 813.740.032 bytes (~0,76 GiB).
O índice observado depois dos contratos JSON caiu de 43 MiB para 3,2 MiB.
Isso identifica o conjunto de artefatos de índice anterior como causa
reproduzida; o arquivo individual/algoritmo interno do Rust 1.92 não foi
isolado. Fontes, comandos, resultados e arquivos anteriores: rustdoc/.

O PATH do serviço foi preservado depois de três testes visuais reportarem
rsvg-convert ausente. Na rodada corrigida, a amostra 23:59:28Z registrou
149.131.264 bytes de memória anônima e 11.943.477.248 bytes de cache de arquivos;
o cgroup tinha memory.events oom 0/oom_kill 0, teto de 12 GiB e pico de 11,72 GiB.
Essa amostra não é uma afirmação sobre o pico de memória anônima durante
toda a execução; as séries de limites/picos estão em memory-budget-samples.json.
