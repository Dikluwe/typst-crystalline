# Tarefa: Diagnóstico — modelo de elemento do cristalino (enum atual vs alternativas)

**Repositório de trabalho**: typst-crystalline (raiz).
**Tipo**: diagnóstico, estilo P311a (ADR-0065: diagnóstico-primeiro). **Zero
código de produto. Zero decisão executada.** O documento termina em
recomendação; a decisão é humana e trava o arranque de qualquer implementação.
**Entregável único**:
`00_nucleo/diagnosticos/diagnostico-modelo-elemento-passo-312a.md`
(confirmar o número de passo contra o estado atual do repositório; se P312 já
tiver outro uso, usar o próximo sufixo livre — registrar a escolha).
**Pastas restritas**: `00_nucleo/materialization/` e `00_nucleo/context/` NÃO
estão autorizadas. Fontes: o código em `01_core/`–`04_wiring/`, o histórico
git, `00_nucleo/adr/`, `00_nucleo/diagnosticos/`, `00_nucleo/DEBT.md`,
`CLAUDE.md`, a tabela de cobertura, o mapa de migração.

---

## Motivação (transcrever no §0 do diagnóstico)

O dono do projeto formulou um requisito arquitetural que não está escrito em
nenhuma ADR: **atomicidade para agentes** — o custo real de um passo de
materialização é proporcional ao que a sessão de IA precisa **ler e editar**
(tokens de contexto), e o desenho atual (`Content` enum fechado, ADR-0026)
concentra cada elemento novo num hub (`content.rs` + um match por backend),
fazendo esse custo crescer com a cobertura. O vanilla era atômico (um módulo
por elemento) mas pagava com macro-mágica (`#[elem]`), que é opaca para
agentes por outro motivo: o código gerado não está no texto. A pergunta do
diagnóstico: **existe desenho com a atomicidade do vanilla, sem a opacidade
das macros, mantendo a verificação mecânica do enum — e qual o custo de
chegar lá a partir de 59+ variantes existentes?**

Dois débitos implícitos da ADR-0026, identificados em análise externa
(2026-06-10), entram como contexto: (a) a StyleChain (DEBT aberto, magnitude
L) pressupõe um **sistema de propriedades reificadas** que o enum não fornece;
(b) o churn do `content.rs` é custo recorrente por elemento (hash estável por
27 passos, quebrado pelo P311).

## §1 — Inventário: o pipeline de um elemento hoje (caso: Heading)

Mapear, com paths e contagens de linha reais, **tudo** que `Heading` toca no
cristalino de ponta a ponta: a variante em `content.rs`; onde os campos
vivem; as regras (show/realize equivalentes); cada match de backend (layout,
pdf, html, svg, render — os que existirem); testes. Resultado: a lista de
arquivos e o total de linhas que um agente precisa ter em contexto para
entender um elemento hoje. Repetir a contagem, mais curta, para um elemento
simples (ex.: `Divider`/`Hr`) — o piso do custo.

Medições mecânicas de apoio (registrar os comandos):
- nº de variantes atuais do `Content` (a auditoria F2 contou 59; confirmar);
- nº de sites de `match` sobre `Content` no workspace e em que crates/camadas;
- tamanho do `content.rs` (linhas) e dos 3 maiores matches.

## §2 — Medição: atomicidade nos passos reais (o baseline)

Pelo histórico git, reconstruir o **conjunto de toque** (arquivos + linhas
adicionadas/modificadas) dos passos registrados que adicionaram elemento ou
variante: **P298 (MathOp)** e **P311b (MathStyled)** — e mais um terceiro se
identificável com confiança pelo log. Se os commits não forem identificáveis
por mensagem, declarar o método alternativo usado (ex.: diff entre tags, ou
leitura dos relatórios de passo no repositório) — **não estimar de memória**.

Saída: tabela "passo × arquivos tocados × linhas × dos quais no hub
(content.rs + matches)". Este é o número que os candidatos prometem reduzir.

## §3 — Os candidatos, com números do repositório

Preencher a matriz com medições e estimativas derivadas dos §1–§2 (não de
opinião). Os três candidatos (mais a referência vanilla):

- **B — enum atual** (ADR-0026): baseline; custo por elemento = §2.
- **D — enum fino com delegação por módulo**: cada variante vira
  `Nome(Arc<nome::Nome>)`; campos+regras+layout no módulo do elemento;
  matches de backend viram dispatchers de 1 linha. Estimar: custo de migrar
  as 59 variantes (mecânico, por variante); custo por elemento novo depois
  (1 módulo + 1 linha no enum + 1 linha por dispatcher); o que acontece com
  a exaustividade (resposta esperada: intacta — verificar se há algum match
  que deixaria de ser exaustivo).
- **F — propriedades reificadas**: nó genérico
  `{kind: ElementKind, props: PropMap}` + descritor por elemento em módulo
  próprio + tabela const única. Estimar: o redesenho (quais tipos centrais
  mudam; quantos sites de match viram lookup); para onde migra a verificação
  mecânica que o compilador deixa de dar (teste que varre a tabela ×
  backends, ou regra nova do crystalline-lint — descrever a opção concreta);
  e o que o F entrega de graça para a StyleChain (§4).
- **(E — geração por macro_rules)**: registrar e descartar com a razão
  medida — código gerado é o ruído que a lente quantificou (resíduo
  `__ComemoCall`, medição 0077) e indireção custa tokens de leitura;
  contradiz o requisito de origem.

Matriz final: candidato × {tokens/contexto por elemento novo (proxy: arquivos
e linhas a ler+editar), verificação mecânica (quem garante exaustividade),
custo de migração a partir de B, efeito no DEBT StyleChain, riscos}.

## §4 — Interação com o DEBT da StyleChain

Constatação a verificar e desenvolver: **o caminho A do DEBT (materializar
StyleChain) e o candidato F são a mesma obra** — ambos exigem o sistema de
propriedades (elemento+campo → chave resolvível). Consequências para a
sequência: fazer D agora e A depois constrói propriedades duas vezes? D é
compatível como etapa intermediária de F (módulos de elemento do D viram
descritores do F)? O diagnóstico deve responder com base na forma real do
código, não em princípio.

## §5 — Recomendação (primária + secundária) e trava

Estilo P311a: recomendação primária com justificativa pelos números,
secundária como alternativa, e a trava explícita: **decisão humana antes de
qualquer implementação**. A recomendação deve incluir a sequência proposta
(ex.: "D incremental agora, F quando o DEBT StyleChain for atacado, com ADR
declarando F como destino" — ou o que os números sustentarem) e o tamanho
estimado em passos S/M/L.

## §6 — O princípio a registrar

Seção curta propondo o texto de uma ADR ou entrada de LESSONS: **"atomicidade
para agentes"** como força arquitetural de primeira classe do Tekt — custo de
um passo ∝ contexto necessário; hubs concentradores são anti-padrão para
manutenção por IA; verificação mecânica não pode depender de o agente
lembrar. Só o texto proposto; a adoção é decisão humana.

---

## Relatório no chat (além do arquivo)

- O baseline do §2 (a tabela passo × toque).
- A matriz do §3 resumida.
- A recomendação do §5 em 3 linhas.
- `git status` (apenas o arquivo novo de diagnóstico; nada mais tocado).

## Restrições finais

- Zero código de produto; zero edição de ADR/DEBT existente (o §6 propõe
  texto, não o grava em ADR).
- Toda contagem com o comando que a produziu registrado.
- Onde o histórico não permitir medir, declarar "não medível com as fontes
  autorizadas" — não estimar de memória.
