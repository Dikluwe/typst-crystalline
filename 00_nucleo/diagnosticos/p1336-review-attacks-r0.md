# P1336 — primeira rodada adversarial insuficiente

Veredicto: R0 não permite fechar o gate de mutantes. Somente C e M1 possuem evidência aproveitável; M2–M6 são Unknown instrumental por reutilização do artefato de M1. Não são cinco rejeições válidas nem cinco sobrevivências do produto.

Manifesto `54864bfd67d50197560011d5aae055a0303fcc9cadc4230221dfd4f7a6daea88`; estado principal e comandos estão em `p1336-attacks-results.json` e nos sete recibos `p1336-attacks-run-{C,M1..M6}.json`. Os snapshots/patches permanecem íntegros em `/tmp/p1336-attacks-1e4k8nye`.

O reviewer leu independentemente os stdout/stderr pinados por cada recibo. C passou os cinco testes usando o cache candidato já validado no produto. M1 registrou `Compiling typst-core`, levou cerca de 191 segundos e falhou especificamente nos controles Int pela regressão do nome. M2–M6 não registram `Compiling typst-core`; cada um concluiu em menos de um segundo e tem exatamente o mesmo SHA de stdout de M1: `76e0c934558b31754b1127b6e09b47faef219be4a3ad8eafa1157c339fe2752e`. Isso confirma o risco de cache descrito em `p1336-review-attacks-instrumentation.md`. A datação exata e a duração de cada execução constam nos recibos originais identificados; nenhuma contagem é atribuída a uma árvore desconhecida.

As flags iniciais `valid_execution` apenas atestavam que algum executável com cinco testes correu, não que o snapshot mutante correto foi compilado. Logo são insuficientes para a aceitação; todos os recibos originais devem ficar preservados com esta ressalva explícita. A nova rodada será focal M2–M6, com timestamp fresco do source instalado no workspace, recompilação observável e identidade do executável. Não há necessidade de repetir C/M1, o corpus CLI ou mudar qualquer expectativa congelada.

O incidente é do protocolo de execução de mutantes, sem achado de regressão no candidato. Não há score 6/6 válido nesta primeira rodada. Somente os resultados válidos e discriminatórios de R2 poderão completar o denominador congelado.
