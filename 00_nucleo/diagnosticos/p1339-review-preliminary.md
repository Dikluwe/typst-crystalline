# P1339 — revisão preliminar das obrigações e da sonda focal

Veredito: **bloqueado antes de L0/contrato/materialização por obrigação incompatível com o baseline**. Não é `PASS_SCOPED`, selo, RED completo nem auditoria final. As dez rotas permanecem no escopo; esta revisão não autoriza exclusões nem substituição silenciosa das obrigações.

Regime: `executado sem atestação de isolamento`. Executor `/root/p1339_preflight_review`, contexto inicial novo, filesystem compartilhado. Escritas limitadas aos novos `p1339-review-*`; nenhum material julgado foi editado. Não li o diagnóstico do operador para formar este veredito. Usei a skill `tekt-materializacao-segregada` e suas duas referências para auditar cadeia causal e capacidades.

## Proveniência e método

Estado observado: HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, branch `Tekt`, `git diff HEAD --stat` vazio em 2026-09-09T21:55:25Z e novamente ao finalizar a revisão. O A0 registra estado inicial com apenas o passo P1339 não rastreado; os scripts e recibos desta fase foram acrescentados como diagnósticos não rastreados. Todos os números abaixo derivam dos recibos pinados em `p1339-review-receipt.json`, incluindo binários, runner, passo, ADRs, skill e fontes efetivamente consultadas.

O verificador reproduzível `node 00_nucleo/diagnosticos/p1339-review-check.cjs` reconstrói os pareamentos, comandos, contagens, repetição e cadeia de hashes sem executar os binários nem editar entradas. A primeira versão encontrou `spawnSync git EPERM` antes de produzir recibo; a versão final somente lê e imprime, e foi executada com exit 0. O recibo foi persistido via `apply_patch`.

Os recibos mediram 48 casos, quatro perfis e duas ordens: 384 execuções por binário e 384 comparações. Vanilla: 2026-09-09T21:54:32.046593+00:00 a 21:54:34.070380+00:00; cristalino: 21:54:34.119384+00:00 a 21:55:10.036131+00:00. Recomputação: 16 `Preserved`, 368 `Violated`, zero `Unknown`, nenhuma diferença normal/inversa. As dez descobertas estão ausentes no cristalino e presentes no vanilla em todos os perfis medidos. Os `Preserved` são os dois controles de multiplicação pelas unidades, não as rotas novas.

A cadeia A0 → autoridades → manifesto → vanilla → cristalino → comparação é coerente nos hashes e horários registrados. O A0 registra V5/V15/V26 com exit 0. Os binários atuais continuam com os hashes dos recibos. A alegação de identidade do produto com P1338 foi auditada como alegação do A0/recorder, sem reabrir e recertificar P1338. A identidade upstream `a51e02804` é a autoridade pinada do repositório; esta revisão hashou a fonte local, sem reconstruir o upstream.

## Achado bloqueante — direção da conversão de ângulos

Medição anterior à leitura da fonte:

- `p1339-vanilla-runs.json:968`: `angle.deg(1)` produz `expected angle, found integer`.
- `p1339-vanilla-runs.json:1398`: `angle.deg(90deg)` produz `90.0`.
- `p1339-vanilla-runs.json:3204`: `angle.rad(1rad)` produz `1.0`.
- `p1339-vanilla-runs.json:2344` e `:3978`: as duas equivalências literais do passo falham já no vanilla por receiver numérico inválido.

Fonte consultada depois da medição: `lab/typst-original/crates/typst-library/src/layout/angle.rs:140` abre o scope público; `:142`/`:148` documentam conversão do ângulo para radianos/graus; `:143`/`:149` expõem os nomes `rad`/`deg`; `:144`/`:150` recebem `self: Angle` e retornam `f64`. Já `:43` a `:50` são construtores internos Rust fora desse scope. Esta distinção tem evidência declarativa, além do comportamento executado.

Classificação: receiver, resultado, unidade, sinal e erro são linguagem; `#[scope]`, nomes internos `to_rad`/`to_deg`, `Scalar` e construtores Rust são mecânica. Inferência: a cláusula de equivalência do passo confundiu o construtor interno com a função pública. Refutação possível: uma declaração pública do baseline pinado e uma sonda válida demonstrando construtor numérico com esses nomes. As evidências atuais demonstram o contrário.

`00_nucleo/materialization/typst-passo-1339.md:103` a `:104` exige equivalência `angle.deg(x)` ↔ `x * 1deg` e `angle.rad(x)` ↔ `x * 1rad`. Isso não pode coexistir com paridade ao vanilla. A linha `:102` pode continuar como teste de rejeição de inteiros/floats; o conflito substantivo está na equivalência afirmativa.

É necessária decisão humana explícita para retificar essa obrigação antes de converter o passo em L0 e contrato: conservar as dez rotas e especificar conversão `Angle → float`, incluindo rejeição numérica medida. Não cabe fabricar uma extensão construtora nem retirar ângulos do lote. A skill lida, `/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md:64`, exige: “Pare e solicite decisao humana quando a intencao for insuficiente, interpretacoes forem incompatíveis”. A aplicação aqui é incompatibilidade normativa demonstrada, não uma exigência genérica de aprovação para toda correção de paridade. O gate ADR-0127 sobre assinaturas/default/fase continua dependente do desenho futuro; nenhum desenho foi autorizado por esta revisão.

## Pendência de protocolo — mutação estrutural

O passo `:277` exige rejeitar mutante que apenas duplica a fórmula estática/ligada. Duas fórmulas idênticas podem produzir todos os mesmos observáveis de linguagem. Portanto, a exigência de causa semântica única precisa de testemunha arquitetural explícita; um contrato somente de saída não distingue duplicação equivalente. Isso é uma inferência lógica, refutável se o mutante concreto demonstrar divergência de linguagem. Antes do selo, classificar essa obrigação como arquitetura e prever auditoria da delegação sem confundi-la com mutation score semântico. Não retirar esse mutante silenciosamente nem alegar que testes de saída provaram compartilhamento.

## Limites do ensaio e continuidade permitida

O manifesto `p1339-probe-manifest.json:10` declara o recorte focal. Os métodos de float, função e versão foram apenas descobertos, não exercitados funcionalmente. A fonte declarativa consultada é compatível em princípio com as obrigações dessas famílias: `float.rs:97` para sinais/NaN, `:121`/`:158` para conversão de bytes e defaults; `func.rs:395`/`:412` para pré-aplicação e seleção; `version.rs:109` para índices e lista explícita. Isso não substitui as sondas bilaterais restantes nem confirma seus diagnósticos.

O runner `p1339-probe.py:79` a `:92` classifica qualquer processo concluído como `Observed` e só timeout como `Unknown`. A comparação exata é adequada aos `repr` e erros públicos destes casos inspecionados, mas não distingue automaticamente falha de infraestrutura idêntica nos dois binários. Os controles presentes e os canais auditados não mostram essa falha nas execuções atuais. Para ampliar o runner ou usá-lo como gate, validar status/saída e construir controle opaco explícito; zero `Unknown` neste recorte não certifica a política de opacidade.

O manifesto de autoridades `:13` declara `capabilities_enforced: false`. A instrução de congelar antes de ler em `:57` não foi cumprida para as leituras iniciais da skill, passo e manifesto: estes foram hashados logo após a leitura, permanecendo com os mesmos hashes no recibo final. Isso está declarado, não reparado retroativamente. Não há atestação de isolamento nem verificador tecnicamente incapaz de editar o produto, como exige o fechamento do passo `:420`; houve restrição operacional respeitada. A redação final deverá resolver essa diferença de regime sem inventar conformidade.

Não houve alteração de L0, contrato, oráculos ou produto. Nenhuma fase completa deve ser apresentada como concluída. A revisão termina com o bloqueio concreto e os artefatos de evidência; a retomada depende da decisão sobre a obrigação de ângulos e preserva integralmente o restante do escopo e seus gates.
