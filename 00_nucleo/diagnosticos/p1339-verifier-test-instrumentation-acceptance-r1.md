# P1339 — configuração instrumental prospectiva

2026-09-10, `/root/p1311_review`; executado sem atestação de isolamento.
Li integralmente `p1339-test-instrumentation-configuration.json` e conferi
SHA-256 `7979f51201809ae69a272a1b61b859d8801f1e6f543b244e4d1dbb2ad2bac6b9`.

Aceito a definição do cfg `p1339_observation` como binding exclusivamente
de testes previsto em candidate_test_binding do contrato R3. Propagação
explícita para core como dependência de infra resolve a insuficiência de
supor propagação automática de cfg(test). Target isolado, rustflags,
comandos --no-run e futura proveniência dos executáveis estão especificados.

O aceite depende das restrições integrais do documento: nenhum modo/feature
ou API produtiva adicional, storage puro por instância, nada global, hooks
passivos nos caminhos reais, sem execução extra para telemetria, sem expected
no port e auditoria independente ordinary/instrumented antes de crédito F.
O build normal deve excluir hooks e usar target separado. Somente nomes e
layout dos bindings permanecem late-bound; DTO e predicados não mudam.

Pinar declaração e este aceite antes do selo. Isso não é ativação: nenhum
build, alteração produtiva, teste ou crédito foi realizado por esta revisão.
Ativação permanece condicionada ao selo independente e RED real. Não altera
L0, R3, budget ou obrigações; não concede atestação de isolamento.
