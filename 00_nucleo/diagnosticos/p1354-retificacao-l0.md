# P1354 — retificação do L0

## Veredito

A inflação processual foi retirada dos Prompts L0 sem alterar comportamento do
produto. O conteúdo preservado descreve obrigações atuais de linguagem,
ownership, interfaces, geometria, diagnóstico e contratos reais de teste.

O problema era documental: evidência de sessão e método de execução haviam
passado a ocupar o lugar do contrato. Não foi encontrada justificativa para
reverter a implementação da refatoração.

## Proveniência

- Base de medição isolada: `ed4da535a3a2d554c981cf20af81fec41d2a410a`.
- Integração final sobre `e7ad8290054cdd3f9309e3738e224d7f407ecb60`;
  os dois commits P1353 possuem o mesmo tree
  `d3de6cb56f4e3b79f465c2f486e4ff89ff361dff`.
- Data da auditoria final: `2026-09-12T12:41:12-03:00`.
- Estado: working tree não commitada na cópia isolada P1354.
- Antes de adicionar este relatório, `git diff --stat` registrava 120 ficheiros,
  1.016 inserções e 5.119 remoções; 60 eram Prompts L0.
- O inventário lexical pós-limpeza ficou em
  `/dev/shm/p1354-candidatos-pos.txt`; é temporário e não integra o projeto.

## Medição

O inventário inicial do passo encontrou 123 prompts candidatos pela lente
lexical ampla. A mesma lente restrita aos termos mais característicos de rito
— preseal, autoria independente, mutation score, veredito de passo, estado
`DRAFT_L0_AWAITING`, integração pendente e STOP ADR — encontra zero Prompts
após a retificação.

A busca ampla por palavras ambíguas caiu de 87 para 56 prompts. As ocorrências
restantes não são usadas como meta: incluem proveniência curta exigida por
ADR-0108, restrições negativas, manifests reais de pacote e certificados TLS.

No conjunto alterado, o saldo do namespace `prompts/` é 956 inserções e 5.059
remoções, redução líquida de 4.103 linhas. A redução é consequência da remoção
de história, não critério arquitetural.

## Classes aplicadas

### Obrigações preservadas

- carrier triestatal de `MathAttach` e sua projeção em eval, repr e layout;
- morfologia `TextItem`, correção itálica e exclusão de `ssty`;
- spans sintáticos de float, math e constructors HTML;
- coordenadas globais de wrappers e observers de teste;
- serialização HTML explícita e separação entre target, feature e modo;
- composição `ArmedWatch` e comportamento externo de watch;
- estabilização contextual seletiva e comparação fechada;
- vocabulário semântico de tabela e transporte de `a11y-extras`.

### Contratos de teste deshistoricizados

Os owners de `eval/tests`, `layout/tests`, `infra/integration_tests`, CLI,
P1289, P1292, P1293 e watch agora declaram entradas, observáveis, controles e
condições de sucesso. Foram removidos agentes, ordem de autoria, campanhas,
recibos bloqueantes, scores e resultados passados convertidos em obrigação.

### História e metaprocesso removidos

- gates já consumados tratados como estado pendente;
- cronologias P1293–P1297 e tentativas refutadas dentro de L0;
- hashes de recibos, manifests de sessão e selos intermediários;
- instrumentação P1341/P1342 já aposentada por P1353;
- obrigação universal de protocolo Tekt completo.

## Linhagem

O pré-gate produziu somente V5 nos prompts editados; V15 e V26 permaneceram
ausentes. `crystalline-lint --fix-hashes .` atualizou 60 consumers. A inspeção
de `git diff --unified=0 -- '*.rs'` confirmou que todas as mudanças Rust são
exclusivamente linhas `//! @prompt-hash`.

## Gates finais

- `git diff --check`: GREEN.
- `crystalline-lint .`: GREEN, exit 0; V5/V15/V26 = 0.
- `cargo check --workspace --locked`: GREEN, exit 0; apenas warnings já
  existentes.
- `cargo fmt --all -- --check`: GREEN, exit 0.

## Retorno ao prumo

O próximo trabalho deve partir de lacuna observável da linguagem e seguir o
fluxo proporcional:

```text
medição → L0 conciso → RED → implementação → GREEN
```

Skill, diagnóstico e passo apoiam esse fluxo, mas não são conteúdo normativo do
Prompt L0. Segregação e mutação voltam a ser ferramentas selecionadas pelo
risco, não rito que gera sucessores por si próprio.
