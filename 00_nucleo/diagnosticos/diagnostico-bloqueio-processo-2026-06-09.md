# Diagnóstico — por que o projeto typst-cristalino parou

**Data**: 2026-06-09
**Último passo materializado conhecido**: P311b (math styles, conversa de 2026-05-20)
**Tempo parado**: ~3 semanas
**Conclusão central**: o bloqueio não é técnico. É de processo entre sessões.

---

## §1 — Falhas identificadas (com evidência)

### F1 — Project knowledge desatualizado (~100 passos de atraso)

- Documento mais recente no project knowledge: ~P206 (DEBT.md menciona
  P206E de 2026-05-08; ADRs param perto da ADR-0075).
- Estado real do projeto: P311b (2026-05-20).
- Efeito: toda nova conversa lê um estado falso. O Claude trabalha
  sobre baseline velho até você anexar arquivos manualmente.

### F2 — Instrução de projeto ambígua

- A instrução "Documente os passos em arquivos .md" não diz QUAIS
  passos nem QUANDO.
- Efeito observado em pelo menos 4 sessões (P181, P241, P267, P283):
  a sessão começa com o Claude perguntando "o que você quer que eu
  documente?" em vez de trabalhar. 1-3 turnos perdidos por sessão.

### F3 — Sessões terminam sem decisão fixada

- A última sessão (2026-05-20) terminou com "Decisão sobre P312 fica
  contigo" + 4 categorias de frentes (Cat A/B/C/D).
- A decisão não foi tomada nem registrada. O projeto parou aí.
- Padrão recorrente: o fim de sessão produz menus, não compromissos.

### F4 — Drift documental interno

- Confirmado na análise do chat P306: o arquivo de cobertura
  (`typst-cobertura-vanilla-vs-cristalino.md`) só foi sincronizado
  até P283. Os passos P284-P305 modificaram código sem propagar.
- Provável extensão atual do drift: até P311b.
- Cobertura declarada 70,9% vs real estimada ~73,1% (números da
  análise de 2026-05-19).

### F5 — Continuidade depende de upload manual

- O fluxo gera documento de transição no fim de cada sessão.
- Se o documento não é anexado na sessão seguinte, ela começa cega
  (F1 agrava: o project knowledge não serve de fallback).

---

## §2 — Passos corretivos (ordem de execução)

### Passo C1 — Sincronizar o project knowledge (XS; ~15 min)

Substituir no project knowledge os arquivos desatualizados pelas
versões atuais do repositório:

- [ ] `00_nucleo/DEBT.md` (versão pós-P311b)
- [ ] `typst-cobertura-vanilla-vs-cristalino.md` (versão atual,
      mesmo com drift — será corrigido no C3)
- [ ] `blueprint-projecto.md` (versão mais recente)
- [ ] Documento de transição mais recente
      (pós-P311b, da conversa "Passo 306 -")
- [ ] README de ADRs atualizado

Critério de verificação: uma busca por "P311" no project knowledge
retorna resultado.

### Passo C2 — Corrigir a instrução do projeto (XS; ~5 min)

Substituir "Documente os passos em arquivos .md" por uma instrução
operacional sem ambiguidade. Sugestão literal:

> Ao iniciar uma sessão: (1) buscar no project knowledge o documento
> de transição mais recente; (2) confirmar o próximo passo nele
> registrado; (3) produzir a spec desse passo em .md sem perguntar
> o que documentar. Toda spec e todo relatório de passo são entregues
> como arquivo .md em outputs.

Critério de verificação: a próxima sessão nova não começa com
pergunta de clarificação.

### Passo C3 — Sincronização documental P284-P311b (S; ~1-2h)

- [ ] Propagar à tabela de cobertura as linhas afetadas por
      P284-P311b (mínimo conhecido: math accent/cancel/underover/op
      `parcial → implementado`; footnote `ausente → implementado⁺`;
      A.7 Text features ~57% → ~65%).
- [ ] Atualizar contagem de testes (2 899+ na última medição).
- [ ] Atualizar contagem de ADRs (86+ vigentes).

### Passo C4 — Fixar P312 (decisão sua; 1 turno)

As 4 frentes catalogadas no fim da sessão P306-P311b:

| Opção | Frente | Magnitude estimada |
|-------|--------|--------------------|
| Cat A | Text features (~65% → ~80%) ou Foundations stdlib | M por sub-passo |
| Cat B | Construct (array/dictionary/group/tuple); Tables refino | M |
| Cat C | DEBT-1 StyleChain real (M+); DEBT-libm (M); DEBT-37 | M-M+ |
| Cat D | Greek+dígitos math style; display/inline | S-M |

Regra nova para evitar repetição de F3: **toda sessão termina com
"Próximo passo = PNNN (frente X)" escrito no documento de
transição.** Se você não escolher, vale a recomendação do relatório
do passo anterior como default.

### Passo C5 — Retomar materialização (P312)

Só depois de C1-C4. A primeira sessão de retomada deve:

1. Anexar o documento de transição (ou confiar no project knowledge
   já sincronizado por C1).
2. Abrir com mensagem direta: "P312 = [frente escolhida]. Produz a
   spec." — sem espaço para clarificação.

---

## §3 — Regras permanentes para impedir reincidência

1. **Fim de sessão**: documento de transição sempre nomeia o próximo
   passo concreto (não um menu).
2. **A cada ~10 passos**: subir os documentos de estado atualizados
   ao project knowledge (DEBT.md, cobertura, blueprint).
3. **Drift**: relatório de passo que altera cobertura propaga a
   tabela no mesmo passo (já era a convenção; passou a ser ignorada
   a partir do P284).
4. **Início de sessão**: a primeira mensagem sua nomeia o passo.
   "Vamos continuar" sem anexo e sem número de passo é o gatilho
   conhecido do loop de clarificação.
