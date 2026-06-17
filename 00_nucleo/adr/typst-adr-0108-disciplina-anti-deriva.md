# ⚖️ ADR-0108: A disciplina anti-deriva — medir antes de decidir

**Status**: `EM VIGOR`
**Data**: 2026-06-17

---

## Contexto

Um padrão de erro repetiu-se nesta branch: o assistente **decidiu ou classificou a partir
de narrativa acumulada ou de um enquadramento plausível, em vez de medir da fonte primeiro**.
Cada vez, o dono inseriu manualmente o mesmo movimento — **medir antes de decidir** — e a
decisão mudou. Os casos (evidência-zero desta ADR, todos reais desta branch):

- **β1 (P339)** — um wrapper de transporte (`Content::Styled`) tratado como forma da
  língua; a medição mostrou que era mecânica de transporte.
- **A "noite" (P340)** — assumiu-se que a fidelidade exigia copiar a mecânica de passes do
  vanilla; o contrato era a **saída**, não o algoritmo.
- **O `==` mecânico (P342/P344)** — paridade de conteúdo medida pelo `PartialEq` do Rust
  (mecânica); a ADR-0107 corrigiu para **morfologia**.
- **A narrativa da falha (P347→P347c)** — "o tekt achou um bug do vanilla" foi construído e
  ficou mais empolgante a cada passo; a fonte (o commit #3327, "Support text show rules that
  match their own output") mostrou **design deliberado**. O método recuou.
- **A Revocation (P347d)** — quase respeitada como função de língua ("é função do Typst");
  a medição mostrou **mecânica interna** (construída só pelo motor, sem porta de usuário).

A **ADR-0107** já transformou **uma** dessas freadas (paridade ≠ mecânica) em regra. Esta
ADR **generaliza o mecanismo**: nomeia a **classe** do erro e os seus modos de falha, para
que a próxima freada da mesma classe seja pré-emptada pela **forma do prompt**, não pelo
olho do dono. **Objetivo declarado: reduzir a dependência do dono como freio — não
eliminá-la.**

---

## Decisão — as regras (verificáveis, não exortativas)

Um prompt do tekt deve ter uma forma que torne a violação **detectável**. Cada regra traz o
seu critério de detecção.

1. **Medir antes de decidir.** Uma secção de **decisão/classificação** é **precedida** por
   uma medição (Fase A, `file:line`) que a **produz**.
   *Violação detectável:* a decisão aparece antes da medição, ou sem medição, ou a medição
   só "confirma" uma decisão já escrita. (β1, noite, narrativa da falha.)
2. **Língua vs mecânica, explícita e da fonte.** Antes de tratar algo como relevante para
   paridade, classificá-lo da fonte: semântica/sintaxe/morfologia = língua (paridade);
   mecânica (igualdade do Rust, bytes, passos, estrutura de dados) = diverge (ADR-0107).
   *Violação detectável:* um critério de paridade/aceitação que use mecânica sem a
   classificação explícita. (O `==`, a Revocation.)
3. **Intenção vs comportamento.** Antes de reproduzir um comportamento do vanilla,
   distinguir **intencional** (língua) de **gerado** (acidente de implementação); **não
   inferir intenção do comportamento**.
   *Violação detectável:* "o vanilla faz X" usado como "o vanilla quis X" sem evidência de
   intenção. (A narrativa da falha.)
4. **Marcar inferência e o que a refutaria.** Um achado declara o que é **medido** vs
   **inferido**, e que evidência o **refutaria**.
   *Violação detectável:* uma conclusão sem a marca, ou sem o critério de refutação.
5. **Desconfiar do enquadramento confortável.** Um enquadramento **lisonjeiro** (uma
   vitória, "achamos um bug") ou **cômodo** (a escolha humilde, "respeitar tudo que existe")
   exige uma **checagem extra da fonte antes de ser aceito**.
   *Violação detectável:* um enquadramento dessa natureza aceito sem a checagem extra
   registrada. (A narrativa da falha — lisonjeira; a Revocation — cômoda.)
6. **Aceitação no nível da língua.** O critério de aceitação é semântica/sintaxe/morfologia,
   **nunca** mecânica (booleano, byte, `PartialEq`), **exceto** onde a mecânica **é** o
   observável (a mensagem de erro).
   *Violação detectável:* um gate de aceitação "o booleano bate" / "os bytes batem" fora da
   exceção. (ADR-0107 operacionalizada como gate.)

---

## O resíduo (o que esta disciplina NÃO reduz — honestidade obrigatória)

A disciplina pega os modos de falha **conhecidos**. **Não** pega:
- **modos de falha novos** — uma deriva de uma classe ainda não nomeada; aí o dono ainda é
  o freio, e o novo modo, quando pego, vira regra (o próprio mecanismo desta ADR);
- **o reenquadramento "um nível acima"** — ver que o objetivo real é outro do que está
  sendo otimizado; é julgamento do dono, não checklist;
- **o cumprimento ritual** — seguir a forma sem a substância (Fase A que mede o que não
  decide; inferência marcada que ninguém checa). Contra isto, o papel do dono **muda**: de
  "pegar toda deriva" para **auditar se a disciplina é seguida na substância**.

Portanto esta ADR **reduz** a dependência do dono ao freio — **não a elimina**. É "ao
máximo", não "a zero".

---

## Nota de fundação (o ponto cego de quem escreve)

Esta disciplina foi **escrita pelo assistente para pegar a própria deriva**. Uma regra que
o autor escreve para se pegar tem um ponto cego: pode ser redigida para ser confortável de
seguir **sem substância**. Por isso as regras são **verificáveis** (forma detectável), não
exortativas; e por isso esta ADR é **selada pelo dono**, não pelo assistente. É a **regra 5**
(desconfiar do enquadramento cômodo) aplicada a si mesma: "uma disciplina que me absolve" é
o enquadramento mais cômodo de todos.

---

## Consequências

**Positivas**: um prompt que viole a disciplina é **malformado pela forma** (detectável sem
o olho do dono); o dono migra de "freio de toda deriva" para "auditor da substância".

**Negativas/limites**: não pega modos novos, reenquadramento de objetivo, nem cumprimento
ritual (o resíduo). Reduz ≠ elimina.

**Neutras**: nenhuma mudança de código; é método.

---

## Precedentes que esta ADR generaliza

- **ADR-0107** — "Paridade é com a linguagem, não com a mecânica": a **primeira** freada
  virada regra (a regra 2/6 desta ADR é a sua operacionalização). Esta ADR nomeia a classe
  e adiciona as outras 5 faces.
- Casos-zero: P339 (β1), P340 (noite), P342/P344 (`==`), P347→P347c (narrativa da falha),
  P347d (Revocation).

---

## Referências

- ADR-0107 (precedente generalizado).
- Passos P339, P340, P342, P344, P347→P347d (as freadas reais; evidência-zero).
