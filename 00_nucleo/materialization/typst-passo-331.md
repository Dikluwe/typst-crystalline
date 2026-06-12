# Tarefa P331 — Diagnóstico do F (execução longa, autônoma): inventário + rede de caracterização + dossiê de opções

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P331 (confirmar livre).
**Pré-condição**: P330 fechado — roteiro de lotes encerrado (65 migradas),
baseline 10× registrado (`medicao-pre-f-passo-330.md`), lint 0, suíte verde
(typst-core 2697). Se não, parar.
**Tipo**: diagnóstico-primeiro do F / DEBT 99.E — **execução longa sem
supervisão (~6 h)**. **ZERO decisão de desenho. ZERO código de produto.**
O único código permitido é **teste novo** (Fase 2). A decisão do F é do dono,
no checkpoint da manhã, com o dossiê deste passo na mão.
**Fontes**: ADR-0105 (F como destino), DEBT 99.E (StyleChain), triagem
DEBT-58 (P329), `medicao-pre-f-passo-330.md` (baseline 10×),
`medicao-pre-f-passo-318.md` (superseded, contexto), relatório P330,
`lab/typst-original/` (quarentena — **fonte de leitura autorizada**; nunca
importar dela).
**Commits**: um por fase concluída — "Passo 331 — F fase 1 (inventário)",
"Passo 331 — F fase 2 (caracterização)", "Passo 331 — F fase 3 (dossiê)".
Isoláveis; fase não concluída não commita parcial sem nota no progresso.

---

## Regras da execução autônoma (valem as ~6 h inteiras)

1. **Bloqueio não existe**: dúvida ou ambiguidade **não para o passo** —
   vira pergunta numerada no dossiê (Fase 3, §perguntas) e o trabalho segue
   na próxima frente. Nada de esperar resposta humana.
2. **Progresso em disco**: manter
   `00_nucleo/diagnosticos/f-progresso-passo-331.md` atualizado ao **fim de
   cada subfase** (o quê está feito, o quê falta, onde parou) — o passo deve
   ser retomável se a sessão morrer.
3. **Ordem por valor**: Fase 1 > Fase 2 > Fase 3. Se o tempo acabar, o que
   estiver pronto já serve; a fase seguinte declara-se incompleta no
   progresso.
4. **Fronteiras duras**: nenhum arquivo de produto muda (Fases 1 e 3 são só
   leitura/registro; Fase 2 só cria/edita arquivos de teste). Nenhuma
   asserção existente muda. `crystalline-lint .` = 0 ao fim de cada fase.
   Nada de otimização (medir ≠ mexer). Pastas restritas seguem restritas.
5. **Fatos com referência**: toda afirmação sobre o código (deste repo ou do
   vanilla) carrega `arquivo:linha` ou comando grep — nunca memória.
6. **Paralelização** (se houver agentes): a Fase 1 divide em 3 frentes
   independentes (1a ‖ 1b ‖ 1c); Fases 2 e 3 são seriais após a Fase 1.
   Cada frente escreve em arquivo próprio; a consolidação é da Fase 3.

---

## Fase 1 — Inventário da superfície de estilo (só leitura; a maior)

Entregável: `00_nucleo/diagnosticos/f-inventario-passo-331.md` (ou um por
frente + consolidação).

### 1a — O lado cristalino: tudo que toca estilo hoje

- **Todo site** que toca `Styles`/StyleChain/PropMap (ou como se chamem aqui
  — descobrir os nomes reais por grep e registrá-los): construção, leitura,
  herança, merge. Tabela: site × arquivo:linha × papel (produz/consome/
  propaga).
- **As 4 `Set*` de ponta a ponta**: para cada uma
  (`SetHeadingNumbering`(64) · `SetEquationNumbering`(18) · `SetPage`(10) ·
  `SetFigureNumbering`(7)), o fluxo completo construção → transporte →
  consumo (quem lê o marcador e onde a propriedade vira efeito). Diagrama
  textual por variante.
- **`Styled`(78)**: o que o wrapper faz hoje — onde é criado, onde é aberto,
  o que `Styles` carrega nele, interação com os 6 matches do hub.
- **As 3 folhas provisórias** (`Text`(81) · `MathText`(42) · `MathIdent`(103)):
  quais campos de estilo o cristalino já lhes dá vs o que fica implícito;
  onde o layouter decide fonte/tamanho/peso hoje.
- **Larguras refeitas hoje** (grep registrado) — o preditor do custo de
  qualquer opção da Fase 3.

### 1b — O lado vanilla (quarentena, só leitura)

- Como a StyleChain real funciona: estrutura de dados, set rules, show
  rules, resolução em cascata, recipes — **descrição factual com
  arquivo:linha da quarentena**, no nível de detalhe suficiente para a Fase
  3 desenhar opções (não copiar código; descrever o desenho).
- O que o vanilla faz com os equivalentes de `Styled`/`Set*` (elementos?
  entradas da chain?) e com as propriedades de `Text`.
- **Fidelidade de comportamento** (critério do dono, P329): listar os
  comportamentos observáveis de estilo que o cristalino já cobre (set de
  numeração, página etc.) — é o contrato que qualquer opção F preserva.

### 1c — O custo do estado atual (números para a decisão)

- Os 12 arms restantes nos 6 matches: o que cada um custa em
  linhas/acoplamento (medir, não opinar).
- O baseline 10× (P330) citado como o "antes" oficial.
- Qualquer medição barata adicional que uma opção da Fase 3 vá precisar
  (ex.: nº de propriedades de estilo distintas consumidas no layouter) —
  medir agora, com comando.

## Fase 2 — Rede de caracterização do comportamento de estilo (só testes)

Entregável: testes novos que **fixam a saída atual** — a rede que detecta
regressão semântica quando o F refatorar. Zero mudança de produto.

- Cobrir, no mínimo: efeito de cada `Set*` na saída (numeração de heading/
  figure/equation; página); herança/escopo via `Styled` (estilo aplica ao
  corpo, não vaza); interação set + elemento migrado (ex.: heading numerado
  dentro de columns); plain_text/layout de `Text` com e sem `Styled` em
  volta. Preferir asserções sobre **saída observável** (texto/layout/export
  de teste), não sobre representação interna — o F muda a representação; a
  rede protege o comportamento.
- Convenções do repo (módulos de teste existentes como precedente). Suíte
  verde ao fim (contagem cresce só pelos novos; registrar +N). Lint 0.
- Se um comportamento atual parecer **bug** (divergência do vanilla na
  saída): **não consertar** — caracterizar o comportamento atual em teste
  com comentário `// caracteriza estado atual; divergência registrada no
  dossiê §bugs` e listar no dossiê. Conserto é decisão do dono.

## Fase 3 — Dossiê do F: opções desenhadas, decisão não tomada

Entregável: `00_nucleo/diagnosticos/f-dossie-opcoes-passo-331.md`.

- **2 a 4 opções de desenho** para a StyleChain/PropMap do cristalino (ex.:
  PropMap tipado próprio; StyleChain à vanilla; híbrido mínimo que só
  generaliza as 4 `Set*`; — as opções reais saem da Fase 1, não desta
  lista). Para **cada** opção:
  - o desenho em 1 página (estruturas, fluxo set→consumo, o que acontece
    com `Set*`/`Styled`/folhas provisórias);
  - **custo previsto pelo preditor** (largura por grep da Fase 1c — sites
    tocados, módulos novos, estimativa de lotes/passos);
  - riscos e o que quebra primeiro; o que a rede da Fase 2 cobre e o que
    não cobre;
  - compatibilidade com o critério do dono (comportamento idêntico;
    estrutura livre).
- **Sem recomendação final obrigatória**: pode haver uma seção "leitura do
  executor" claramente marcada como tal, mas as opções são apresentadas
  completas e a escolha fica em aberto.
- **§Perguntas ao dono**: numeradas, acionáveis (formato dos checkpoints dos
  lotes), incluindo as dúvidas acumuladas nas Fases 1–2.
- **§Bugs/divergências** (da Fase 2, se houver).

---

## Relatório (`typst-passo-331-relatorio.md` + resumo no chat)

- Estado por fase (completa/incompleta + onde está o progresso).
- Fase 1: os entregáveis e os 3–5 fatos mais relevantes para a decisão.
- Fase 2: +N testes, o que a rede cobre, suíte verde, lint 0; §bugs se
  houver.
- Fase 3: as opções (resumo de 2 linhas cada) e as perguntas ao dono.
- `git log --oneline` (commits por fase); `git status` limpo.
- Caveat conhecida do stack (`RUST_MIN_STACK=33554432`) — não é regressão.

## Fora de escopo (confirmado)

**A decisão do F** (do dono, no checkpoint da manhã, com o dossiê);
qualquer mudança de produto em `Styled`/`Set*`/`Text`/`MathText`/
`MathIdent` ou em qualquer outro lugar; consertos de bugs encontrados
(registrar, não consertar); otimizações (o baseline existe para o
antes/depois do F, não para mexer agora).
