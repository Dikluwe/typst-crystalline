# Prompt — typst-passo-857: `measure()` — checar se a Opção 1 (injeção no Engine) pode ser feita via inversão de dependência, sem quebrar a pureza de L1

**Origem**: revisão da rejeição da Opção 1 em P849 — a rejeição foi por "custo de pureza de L1", mas o projeto já tem um padrão que evita esse custo em outro caminho (`FontMetrics` como trait em L1, implementado em L3, injetado por wiring) e não está claro se P849 considerou essa forma antes de descartar.
**Estado**: aguardando execução — **isto é verificação/design, não implementação ainda**. Só implementar se a checagem confirmar que o padrão se aplica sem exceção.

---

## A hipótese a testar

O padrão já usado no projeto para `Layouter`/métricas de fonte é: L1 declara um **trait** (`FontMetrics`), não conhece nenhuma implementação concreta; `FixedMetrics` (heurística, sem I/O, definida em L1) satisfaz o trait para uso em contexto puro/teste; a implementação real de L3 (`FallbackFontMetrics`, em `03_infra`) também satisfaz o mesmo trait; a decisão de qual implementação usar acontece na camada de wiring (L4 ou o ponto de composição do pipeline), não dentro de L1 nem de L3 isoladamente.

A hipótese é: o mesmo padrão pode se aplicar a `measure()`/`Engine` — estender `Engine` (ou o que for usado durante `eval`) com um campo do tipo `&dyn FontMetrics` (ou equivalente), satisfeito por `FixedMetrics` fora de contexto de produção e pela implementação real de L3 quando o `Engine` é construído durante a compilação de verdade. Se isso for viável, a Opção 1 nunca precisou tocar `03_infra` a partir de L1 — só precisava de um campo que ainda não existe.

---

## Passo 1 — Confirmar o padrão existente, de verdade

1. Ler o código de `Layouter`, o trait `FontMetrics` (onde está definido — deve ser L1), `FixedMetrics` (L1) e `FallbackFontMetrics` (L3) para confirmar exatamente como a injeção acontece hoje: onde o `Layouter` recebe a implementação concreta, e em que ponto do pipeline essa decisão é tomada.
2. Confirmar que L1, nesse caminho, de fato nunca importa nada de `03_infra` — nem tipo, nem função, nem constante. Se houver alguma exceção sutil (um `#[cfg(test)]` que faz isso, por exemplo), registrar, porque isso mudaria a força do argumento.

## Passo 2 — Verificar se `Engine`/`eval` pode receber o mesmo tratamento

1. Localizar a estrutura `Engine` (usada durante `eval`, onde `measure()` é despachado) e confirmar se ela já tem algum campo de acesso a capacidades externas (world, sink, etc.) — se sim, isso é precedente direto de que `Engine` já aceita "capacidades injetadas" sem quebrar pureza, e adicionar mais uma (métricas) é extensão natural, não uma mudança de categoria.
2. Verificar se existe algum motivo técnico específico (não estimado, real) pelo qual `measure()`/`eval_func_call` não poderia simplesmente chamar `engine.font_metrics()` (ou equivalente) do mesmo jeito que `Layouter` chama a métrica hoje. Se existir um motivo real (por exemplo: o `Engine` de `eval` é construído num ponto do pipeline anterior a onde as métricas de fonte já foram carregadas — confirmar isso com uma leitura real do pipeline, não suposição), documentar exatamente qual é.
3. Se o motivo real existir, ele é sobre **ordem de construção no pipeline** (resolvível reordenando quando `Engine` ganha o campo populado), não sobre **pureza de L1** (que seria um problema categórico, não de ordem). Essa distinção é o cerne desta checagem — confirmar qual das duas categorias é o obstáculo de verdade.

## Passo 3 — Comparar com a Opção 1 como P849 a descreveu

Reler a descrição original da Opção 1 em P849 e confirmar se ela já era essencialmente isto (um `&dyn FontMetrics` injetado) ou se P849 estava pensando em algo mais direto (ex.: `Engine` importando `FallbackFontMetrics` concreto, o que aí sim quebraria a fronteira). Se for a segunda, a reformulação deste passo é uma opção nova, não a mesma Opção 1 revisitada — registrar isso com clareza, porque muda o que está sendo comparado com a abordagem 4.2.

## Passo 4 — Veredito

1. Se a reformulação funcionar sem exceção (L1 continua sem importar nada de L3, `measure()` recebe métricas reais via trait injetado, o único obstáculo real é ordem de construção no pipeline e não pureza): esta é a recomendação nova, mais barata que a 4.2, e sem o custo de pureza que motivou rejeitá-la antes.
2. Se aparecer algum obstáculo real de pureza (não estimado — encontrado de fato lendo o código): documentar exatamente onde, e nesse caso a rejeição original de P849 se sustenta e a 4.2 (ou a prototipagem já desenhada) volta a ser o caminho.

## Relatório

`00_nucleo/diagnosticos/typst-passo-857-relatorio.md` com: a confirmação do padrão existente (Passo 1), a checagem sobre `Engine` (Passo 2, com o motivo real se houver obstáculo), a comparação com a Opção 1 original de P849 (Passo 3), e o veredito (Passo 4) — atualizando DEBT-69 com a conclusão. Não implementar nada além desta checagem neste passo, mesmo que o veredito seja favorável — a implementação, se recomendada, é o próximo passo.
