# Passo 1488 — retificar extras e decidir aliases sem quebrar extensões

**Estado:** DIAGNÓSTICO EXECUTADO em 2026-09-24, sem remoções públicas.
Retificação, decisão por rota e proveniência em
`00_nucleo/diagnosticos/typst-p1488-extras.md`.

**Baseline:** commit limpo `90aad760ba472cffb70063baa684c461a240e447`
(2026-09-22), vanilla `586e1bd43`; hashes de binário em P1480 §1.

## 1. Medição anterior à decisão

O inventário default tem 48 extras brutas; seis defaults `format.*` não
são rotas extra reais. O inventário HTML tem 49, com um sétimo default
`format.html.pretty`. O diagnóstico
`00_nucleo/diagnosticos/typst-p1474-adjudicacao-42-extras.md` normaliza
ambos a 42 rotas: 10 extensões deliberadas, 18 aliases candidatos a
depreciação, 12 que exigem sonda, `asset` com gate ausente e `underover`.

**Retificação obrigatória de P1474:** a conclusão “falta
`math.underover`” não sobrevive à sonda bilateral atual: nem vanilla nem
cristalino contêm esse membro. O global `underover` existe apenas no
cristalino. Não adicionar `math.underover` para corrigir uma ausência
inexistente; tratar o global como extra, com decisão própria. `asset` é
tratado em P1484/P1485, não como alias a remover.

As 18 candidatas são 17 aliases globais de estilo/math com caminho
qualificado confirmado mais `calc.log10`. As 12 sem decisão são os nove
aliases `counter_*`/`state_*`, `calc.deg`, `calc.rad` e `replace`. As dez
extensões deliberadas têm L0/decisão histórica; não somá-las ao numerador
de paridade vanilla nem apagá-las automaticamente.

Classificação: nomes e resolução de bindings são superfície de linguagem;
contagem de chaves de um catálogo com defaults duplicados é mecânica de
inventário. Inferência: remover aliases redundantes reduzirá a divergência
sem perder função canônica; refutação é uso real sem substituto compatível
ou L0 que os contrate deliberadamente.

## 2. Sequência de decisão

1. Corrigir o diagnóstico P1474 por adendo explícito, com sonda bilateral
   reproduzível de `underover` e `math.underover`; não apagar a afirmação
   histórica. Atualizar a classificação 42 = 10 + 18 + 12 + 1 (`asset`) +
   1 (`underover` extra).
2. Para cada uma das 18 candidatas e `underover`, medir `type`, `repr`,
   chamada real, uso no workspace e caminho substituto. Para as 12
   indeterminadas, repetir as sondas e investigar contratos de L0 antes de
   decidir. Registar `file:line`, comportamento bilateral, inferência e o
   achado que a refutaria. Não agrupar decisões só para atingir uma meta
   numérica.
3. Preservar as dez extensões já deliberadas e rotulá-las fora do
   numerador. Onde uma remoção for recomendada, redigir L0 causal e plano
   de compatibilidade por owner. Onde uma extensão for preservada, explicitar
   o desvio aceito; não camuflá-la no inventário.
4. **Parar para confirmação humana** antes de qualquer remoção de binding
   público (ADR-0127). Depois do gate, RED de ausência do alias e GREEN do
   caminho canônico, incluindo documentos do próprio repositório e quatro
   perfis de feature. Sem confirmação, fechar somente o diagnóstico, não a
   remoção.

## 3. Gates finais

Regenerar inventários duas vezes com ordenação determinística e separar
`MISSING_MEMBER`, `UNVERIFIED_METADATA`, extra deliberada e extra removível.
Não afirmar “paridade total” enquanto probes sem resultado e gaps funcionais
P1480–P1486 permanecerem. Para qualquer código: L0 primeiro, hashes e
V5/V15/V26, testes RED→GREEN, `cargo build && crystalline-lint .`.
Todo número decisório precisa de commit/tree ou diff exato, hora quando
necessário e hashes dos binários.

## 4. Resultado P1488 — decisão nova sem quebra

A fotografia 48/49 → 42 de P1474 era histórica. Depois do gate de
`asset`, duas gerações independentes dos seis perfis deram 47/48 extras
brutas e **41** rotas normalizadas, exatamente as 42 antigas menos
`asset`. O adendo no diagnóstico P1474 retifica `math.underover`
(ausente bilateralmente), a falsa identidade geral de `cancel` e a
decisão P817-E de manter `calc.log10/deg/rad`, sem apagar o histórico.

As 41 rotas ficam separadas em 10 extensões antes deliberadas, 3
extensões numéricas P817-E, 9 contratos legacy counter/state, 1
contrato `replace`, 17 globais matemáticos históricos e o global
`underover`. Os L0 de `eval` e `calc` explicitam a preservação como
extensão/compatibilidade sem crédito de paridade; nenhuma remoção é
recomendada com a evidência atual. O gate ADR-0127 continua obrigatório
para uma futura proposta concreta de quebra. Passaram teste P817-E,
`cargo build`, `crystalline-lint .` e `git diff --check`.
