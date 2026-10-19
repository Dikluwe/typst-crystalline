# Passo 349 — relatório: disciplina anti-deriva (medir antes de decidir) — ADR + claude.md

> **Estado: redigido, apresentado, PARADO na fronteira de revisão do dono.** Nada selado,
> nada commitado, nenhum `.rs`/`.toml` tocado, `--fix-hashes` não corrido. A ADR-0108 e a
> entrada do `claude.md` estão rendidas abaixo, para revisão — em especial o **resíduo** e
> a **nota de fundação** (onde o ponto cego de quem escreve mora). Documentação pura.

## Pré-condição (desvio registrado)
Lint **0/0**. **Mas a árvore NÃO está limpa**: o **P348** (código `rules.rs` + 3 testes +
2 L0 + bumps de hash) e os relatórios **P347c / P347d / P348** estão **não commitados**
(pilha de commits adiados). A pré-condição literal do P349 ("árvore de produto limpa") não
bate. Como o P349 é doc-only e **para** antes de editar (não toca nada até a aprovação), a
redação é inofensiva; mas **recomendo commitar a pilha pendente** (347c/d + P348) **antes**
de selar o P349, para os commits ficarem distintos e a pré-condição ser satisfeita. Suíte
2726/3245 (estado pós-P348, não re-rodada).

## Número e precedente
- ADR nova = **0108** (0107 é a "Paridade é com a linguagem", do P344 — o precedente que
  esta generaliza). Confirmado: `typst-adr-0107-paridade-linguagem-nao-mecanica.md` existe.

---

## Texto rendido 1/2 — ADR-0108 (PROPOSTO; formato da casa)

```markdown
# ⚖️ ADR-0108: A disciplina anti-deriva — medir antes de decidir

**Status**: `PROPOSTO`
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

## Nota de fundação (o ponto cego de quem escreve)

Esta disciplina foi **escrita pelo assistente para pegar a própria deriva**. Uma regra que
o autor escreve para se pegar tem um ponto cego: pode ser redigida para ser confortável de
seguir **sem substância**. Por isso as regras são **verificáveis** (forma detectável), não
exortativas; e por isso esta ADR é **selada pelo dono**, não pelo assistente. É a **regra 5**
(desconfiar do enquadramento cômodo) aplicada a si mesma: "uma disciplina que me absolve" é
o enquadramento mais cômodo de todos.

## Precedentes que esta ADR generaliza
- **ADR-0107** — "Paridade é com a linguagem, não com a mecânica": a **primeira** freada
  virada regra (a regra 2/6 desta ADR é a sua operacionalização). Esta ADR nomeia a classe
  e adiciona as outras 5 faces.
- Casos-zero: P339 (β1), P340 (noite), P342/P344 (`==`), P347→P347c (narrativa da falha),
  P347d (Revocation).

## Consequências

**Positivas**: um prompt que viole a disciplina é **malformado pela forma** (detectável sem
o olho do dono); o dono migra de "freio de toda deriva" para "auditor da substância".

**Negativas/limites**: não pega modos novos, reenquadramento de objetivo, nem cumprimento
ritual (o resíduo). Reduz ≠ elimina.

**Neutras**: nenhuma mudança de código; é método.

## Referências
- ADR-0107 (precedente generalizado).
- Passos P339, P340, P342, P344, P347→P347d (as freadas reais; evidência-zero).
```

---

## Texto rendido 2/2 — entrada no `claude.md` (proposta; **não** inserida ainda)

```markdown
## Disciplina anti-deriva — medir antes de decidir (ADR-0108)

Todo prompt **mede antes de decidir**: a secção de decisão/classificação é **precedida** pela
medição (`file:line`) que a produz — nunca o contrário. Classifica **língua vs mecânica** da
fonte (semântica/sintaxe/morfologia = paridade; igualdade do Rust/bytes/passos/estrutura =
diverge, ADR-0107); distingue **intenção de comportamento** (não inferir intenção do
comportamento); **marca inferência** e o que a refutaria; **desconfia do enquadramento
lisonjeiro/cômodo** (checagem extra da fonte antes de aceitar). **Aceitação no nível da
língua**, nunca mecânica — exceto onde a mecânica **é** o observável (mensagem de erro). Um
prompt que viole a forma é **malformado**; o dono **audita a substância** (reduz a
dependência do freio, não a elimina). Ver **ADR-0108**.
```

> Nota de inserção (para quando o dono aprovar): logo após a secção "Paridade — com a
> linguagem, não com a mecânica (ADR-0107)" do `claude.md`, e acrescentar `ADR-0108` à
> tabela de ADRs vigentes.

---

## Confirmação: cada regra tem forma verificável (não exortativa)
| # | Regra | "Violação detectável" |
|---|---|---|
| 1 | Medir antes de decidir | decisão antes/sem medição, ou medição que só confirma |
| 2 | Língua vs mecânica da fonte | critério de paridade usando mecânica sem classificação |
| 3 | Intenção vs comportamento | "faz X" usado como "quis X" sem evidência de intenção |
| 4 | Marcar inferência + refutação | conclusão sem a marca medido/inferido ou sem critério de refutação |
| 5 | Desconfiar do enquadramento cômodo | enquadramento lisonjeiro/cômodo aceito sem checagem extra |
| 6 | Aceitação no nível da língua | gate "booleano/bytes batem" fora da exceção (mensagem de erro) |

As 6 são detectáveis pela **forma**; nenhuma é só "sempre faça X".

## Confirmação de parada na fronteira
- ADR-0108 **não** criada em `00_nucleo/adr/` — texto só neste relatório, para revisão.
- `claude.md` **não** editado. `--fix-hashes` **não** corrido. **Nada commitado.**
- Nenhum `.rs`/`.toml` tocado.
- **À espera do aval do dono** sobre: (a) o texto da ADR e do `claude.md`; (b) em especial o
  **resíduo** (reduz ≠ elimina) e a **nota de fundação** (o ponto cego de quem escreve).
  Após aprovação: criar a ADR, inserir no `claude.md`, selar hash se aplicável, commitar —
  **depois** de commitar a pilha pendente (P347c/d + P348).

## Verificação (gates)
```
content-preserving: zero código/teste/.rs/.toml. Suíte 2726/3245 (não re-rodada).
lint: crystalline-lint . = 0/0.
regras verificáveis: as 6 têm "violação detectável" (tabela acima) — nenhuma exortativa.
evidência real: os 5 modos de falha são freadas reais (P339/P340/P342-P344/P347-c/P347d) —
  não inventados.
honestidade do resíduo: a ADR declara o que NÃO reduz (modos novos / um-nível-acima /
  ritual) e que reduz ≠ elimina.
formato: convenção da casa (⚖️ + Status/Data/Contexto/Decisão/Consequências/Referências);
  ADR-0107 citada com número confirmado. ADR nova = 0108.
fronteira: nada selado/commitado antes da revisão do dono.
```

## Mapa de filtro (campo)
**Lugar lógico:** a disciplina anti-deriva é o **método conhecendo a si mesmo** — as freadas
que o dono deu viraram a **forma** que os prompts são obrigados a ter; mora na fundação, ao
lado da ADR-0107 (a primeira freada-virada-regra). A **honestidade do resíduo** é o que a
separa de uma promessa vazia de automação. **Rastro:** o dono freou N vezes nesta branch; a
ADR-0107 encodou uma (paridade ≠ mecânica); a P349 generaliza o mecanismo; o resíduo fica
com o dono, agora como **auditor de substância**, não como freio de toda deriva.

## Item carregado
`content→elements` aponta para o **Marco G** (P346) — não volta como órfão.
```
