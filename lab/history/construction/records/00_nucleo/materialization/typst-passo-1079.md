# L0 — Passo 1079: Auditoria — `corpus-docs` Alargado e Pergunta Estratégica (Atomização vs Medição)

**Gate**: nenhum — auditoria e levantamento de evidência, sem código alterado,
sem decisão executada. Serve para instruir a decisão do dono nos dois pontos em
aberto (item 6 e item 7 da lista de pendências desta conversa), não para
decidir por conta própria.

**Base**: pendências 6 e 7 do documento de continuação — `corpus-docs`
alargado (nunca dimensionado) e a pergunta estratégica sobre o valor relativo
de atomização/fatiamento vs disciplina de medição (nunca fechada
explicitamente).

---

## Parte A — Dimensionamento de `corpus-docs` alargado

### A.1 Estado actual

`00_nucleo/corpus-docs/math/` (criado no P998) é o único domínio coberto. Não
tenho, nesta conversa, o inventário real de quantos arquivos `.typ` existem lá
nem o formato exacto usado — **preciso pedir isto antes de estimar esforço de
expansão com qualquer precisão**:

```bash
find 00_nucleo/corpus-docs/math -name "*.typ" | wc -l
ls 00_nucleo/corpus-docs/math | head -20
```

Sem isto, qualquer número de "esforço estimado" abaixo é palpite, não medição —
mesma disciplina já aplicada ao resto desta conversa.

### A.2 Evidência indirecta: quantos achados reais já vieram dos domínios propostos, sem `corpus-docs` formal

Cruzando contra o que esta própria conversa já fez (não o histórico completo do
projecto, só o que couber aqui), os domínios propostos para expansão
(`par`/`list`/`heading`/`quote`/`block`) já produziram achados reais **sem**
`corpus-docs` dedicado — via medição pontual, dirigida por sintoma:

| Domínio proposto | Achados reais nesta conversa, sem corpus-docs dedicado |
|---|---|
| `par` | P1057 (`par.spacing` −6.05pt), P1059-1061 (margin collapsing) |
| `block` | P1059-1061 (margin collapsing bloco↔bloco/parágrafo), P1050 (inset de `table`/`grid`, tecnicamente `structural`, não `block` puro) |
| `heading` | P1063 (above/below ausente), P1073 (supplements `Fig.`/`(1)`), P1074 (italic forçado), P1078 (Secção/Seção) |
| `list` | P1072 (`body_indent` 0pt vs 0.5em) |
| `quote` | **Nenhum achado nesta conversa** — domínio não tocado |

`quote` é o único dos 5 domínios propostos sem nenhum achado registado nesta
conversa — não prova que não haja bugs lá, só que não foi investigado, por
nenhuma via (nem `corpus-docs`, nem ad-hoc).

### A.3 O que isto sugere, sem decidir por conta própria

A quantidade de achados reais em `par`/`block`/`heading`/`list` **sem**
`corpus-docs` formal é alta — sugere que o valor já está a ser capturado por
investigação dirigida (a mesma disciplina de "medir antes de decidir" que
gerou P1057-P1078). Isto é evidência a favor de **não** precisar de
`corpus-docs` alargado para continuar a encontrar bugs nestes 4 domínios — mas
não é evidência de que `corpus-docs` não ajudaria a encontrá-los **mais
cedo** ou de forma mais **sistemática** (cobertura completa vs achados por
acaso/sintoma).

`quote`, por não ter sido tocado por nenhuma via, é o caso onde `corpus-docs`
teria vantagem clara sobre esperar por um sintoma aparecer.

### A.4 Dimensionamento de esforço (estimativa, a confirmar com dados reais do A.1)

Sem saber o tamanho real de `corpus-docs/math/`, não dá para estimar esforço de
réplica com confiança. Pedir os dados do A.1 antes de qualquer número aqui ter
valor.

---

## Parte B — Evidência para a Pergunta Estratégica

### B.1 O que a pergunta pede

Do documento de continuação: "se a atomização/fatiamento é caminho útil ou
'queima de energia'... a disciplina de 'medir antes de decidir' tem prova real
de valor; o fatiamento de ficheiros grandes tem prova mais fraca (critério de
isolamento de teste vazio em 5/5 aplicações)".

### B.2 Evidência desta conversa (P1059-P1078)

**Nenhum passo desta conversa foi fatiamento de arquivo.** Todos os ~20 passos
executados nesta conversa (P1059 a P1078) foram, sem excepção, do tipo
"medir → confirmar causa real → corrigir → reverificar por medição". Zero
passos do tipo "dividir hub grande em nós menores" (o padrão dos Blocos 2-3 do
histórico anterior, P1000-1032).

Isto não prova que fatiamento não vale a pena — só que esta conversa,
especificamente, não precisou dele para produzir valor real (14+ bugs
corrigidos com prova de paridade, todos por medição).

### B.3 Limitação desta evidência

Esta conversa é uma amostra de conveniência, não um teste controlado — não
houve fatiamento para comparar porque nenhuma pendência desta conversa exigia
fatiamento, não porque fatiamento tenha sido tentado e falhado aqui. A
comparação justa exigiria ver as duas actividades a competir pelo mesmo tempo
no mesmo período, o que não é o caso desta conversa.

### B.4 O que perguntar ao dono, não decidir sozinho

- Desde o P1032 (último fatiamento registado no histórico), quantos passos
  subsequentes (fora desta conversa também) foram fatiamento vs medição? Só
  quem tem acesso ao histórico completo do projecto sabe isso.
- A pergunta original já tinha "consenso implícito de seguir" sem fechar — vale
  perguntar directamente se essa dúvida ainda é activa, ou se já foi resolvida
  informalmente (por exemplo, se ninguém abriu um passo de fatiamento desde
  então, isso já é uma resposta de facto, mesmo sem decisão formal).

---

## Conclusão — isto é material para decisão, não a decisão

Parte A: recomendo pedir o inventário real (A.1) antes de dimensionar
qualquer coisa — sem isso, não há decisão informada possível, só palpite.
`quote` destaca-se como o domínio com menos cobertura de qualquer tipo.

Parte B: a evidência desta conversa é consistente com a hipótese já registada
("medir antes de decidir tem prova mais forte"), mas é amostra pequena e
enviesada (não houve fatiamento para comparar). Não é prova suficiente para
fechar a pergunta estratégica sozinha.

## Critério de conclusão deste passo

- A.1: inventário real de `corpus-docs/math/` obtido (contagem de arquivos,
  formato), não presumido.
- A.2/A.3: cruzamento contra achados desta conversa, feito e citado por número
  de passo (P1057, P1059-1061, P1063, P1072-1074, P1078) — não afirmação
  genérica.
- B.4: perguntas formuladas para o dono, sem resposta assumida.
- Nenhuma decisão executada — nem abertura de `corpus-docs` alargado, nem
  fechamento da pergunta estratégica. Isso fica para depois desta auditoria
  ser lida.
