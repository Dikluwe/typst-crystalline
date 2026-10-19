# Passo 349 — a disciplina anti-deriva: medir antes de decidir (ADR + claude.md)

> **O que faz.** Encoda, como **regras verificáveis de estrutura de prompt** (não
> exortações), o movimento que o dono inseriu manualmente várias vezes nesta branch
> antes das decisões: **medir da fonte antes de decidir**, e as distinções-irmãs
> (língua vs mecânica; intenção vs comportamento; inferência marcada; desconfiança do
> enquadramento confortável; aceitação no nível da língua). **Objetivo declarado:
> reduzir a dependência do dono como freio — não eliminá-la.** A ADR-0107 foi a primeira
> freada virando regra; esta generaliza o mecanismo. Documentação pura.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P349 (confirmar livre).
**Pré-condição**: P348 fechado (recursão α; flag adiada a L2). HEAD pós-P348; suíte
**2726** (`typst-core --lib`) / **3245** (workspace), lint 0/0, árvore de produto limpa.
Se não bater, parar.
**Tipo**: **definição / método** — content-preserving estrito: **zero `.rs`/`.toml`,
zero código, zero teste**. Toca só uma ADR nova + o `claude.md`. **Termina na fronteira
de revisão do dono**: redige, apresenta, **para** antes de selar — uma disciplina que o
assistente escreve para pegar a **própria** deriva precisa do olho do dono (o ponto cego
está em quem a escreve).
**Objetivo**: que o assistente aplique, por padrão, o movimento que o dono vinha
inserindo — e que um prompt que o viole seja **detectável pela forma**, não só pelo olho
do dono. Cobre os **dois papéis**: quem **escreve** o prompt (o assistente) e quem o
**executa** (o Claude Code), porque ambos enfrentam a fronteira "decidir vs medir / parar
vs prosseguir".
**Fontes** (a evidência real — não inventar modos de falha): as freadas desta branch —
β1 (P339: wrapper de transporte tratado como forma de língua), a "noite" (P340: assumiu
que fidelidade exigia copiar a mecânica de passes), o `==` mecânico (P342/P344: paridade
medida pelo `PartialEq` do Rust), a narrativa da falha (P347→P347c: "achamos um bug"
construído três vezes, derrubado pela fonte), a Revocation (P347d: quase respeitada como
língua, era mecânica). **ADR-0107** (a primeira freada virando regra — o precedente que
esta generaliza). Convenção de ADR do repo (ler uma para o formato).

---

## O conteúdo a materializar (a substância; o agente rende no formato do repo)

### ADR-00NN — "A disciplina anti-deriva: medir antes de decidir" (numerar livre)

**Estado**: PROPOSTO neste passo; EM VIGOR após a revisão do dono.

**Contexto.** Um padrão de erro se repetiu na branch: o assistente **decidiu ou
classificou a partir de narrativa acumulada ou de um enquadramento plausível, em vez de
medir da fonte primeiro**. Cada vez, o dono inseriu manualmente o mesmo movimento
(medir antes de decidir) e a decisão mudou. Os casos (evidência-zero desta ADR):
- **β1 (P339)** — um wrapper de transporte (`Content::Styled`) foi tratado como forma da
  língua; a medição mostrou que não era.
- **A "noite" (P340)** — assumiu-se que a fidelidade exigia copiar a mecânica de passes
  do vanilla; o contrato era a saída, não o algoritmo.
- **O `==` mecânico (P342/P344)** — paridade medida pelo `PartialEq` do Rust (mecânica);
  a ADR-0107 corrigiu para morfologia.
- **A narrativa da falha (P347→P347c)** — "o tekt achou um bug do vanilla" foi construído
  e ficou mais empolgante a cada passo; a fonte (o commit #3327) mostrou design
  deliberado. O método recuou.
- **A Revocation (P347d)** — quase respeitada como função de língua ("é função do
  Typst"); a medição mostrou mecânica interna.

A ADR-0107 já transformou **uma** dessas freadas (paridade ≠ mecânica) em regra. Esta ADR
**generaliza o mecanismo**: nomeia a **classe** do erro e os modos de falha, para que a
próxima freada da mesma classe seja pré-emptada pela **forma do prompt**, não pelo olho
do dono.

**Decisão — as regras (verificáveis, não exortativas).** Um prompt do tekt deve ter uma
forma que torne a violação **detectável**. As regras:

1. **Medir antes de decidir.** Uma secção de **decisão/classificação** deve ser
   **precedida** por uma medição (Fase A, `file:line`) que a **produz**. *Violação
   detectável:* a decisão aparece antes da medição, ou sem medição, ou a medição só
   "confirma" uma decisão já tomada no texto. (Foi o erro de β1, da noite, da narrativa
   da falha.)
2. **Língua vs mecânica, explícita e da fonte.** Antes de tratar algo como relevante para
   paridade, classificá-lo da fonte: semântica/sintaxe/morfologia = língua (paridade);
   mecânica (igualdade do Rust, bytes, passos, estrutura de dados) = diverge (ADR-0107).
   *Violação detectável:* um critério de paridade ou aceitação que use mecânica sem a
   classificação explícita. (Foi o `==`, a Revocation.)
3. **Intenção vs comportamento.** Antes de reproduzir um comportamento do vanilla,
   distinguir **intencional** (língua) de **gerado** (acidente de implementação); **não
   inferir intenção do comportamento**. *Violação detectável:* "o vanilla faz X" usado
   como "o vanilla quis X" sem evidência de intenção. (Foi a narrativa da falha.)
4. **Marcar inferência e o que a refutaria.** Um achado declara o que é **medido** vs
   **inferido**, e que evidência o **refutaria**. *Violação detectável:* uma conclusão
   sem a marca, ou sem o critério de refutação. (Os relatórios já começaram a fazer
   isto — esta regra o torna obrigatório.)
5. **Desconfiar do enquadramento confortável.** Um enquadramento que é **lisonjeiro**
   (uma vitória, "achamos um bug") ou **cômodo** (a escolha humilde, "respeitar tudo que
   existe") exige uma **checagem extra da fonte antes de ser aceito** — porque são os
   enquadramentos que mais induzem a pular a medição. *Violação detectável:* um
   enquadramento dessa natureza aceito sem a checagem extra registrada. (Foi a narrativa
   da falha — lisonjeira; e a Revocation — cômoda.)
6. **Aceitação no nível da língua.** O critério de aceitação é semântica/sintaxe/
   morfologia, **nunca** mecânica (booleano, byte, `PartialEq`), **exceto** onde a
   mecânica **é** o observável (mensagem de erro). *Violação detectável:* um gate de
   aceitação "o booleano bate" / "os bytes batem" fora da exceção. (ADR-0107
   operacionalizada como gate.)

**O resíduo (o que esta disciplina NÃO reduz — honestidade obrigatória).** A disciplina
pega os modos de falha **conhecidos**. **Não** pega:
- **modos de falha novos** — uma deriva de uma classe ainda não nomeada; aí o dono ainda
  é o freio, e o novo modo, quando pego, vira regra (o mecanismo desta ADR);
- **o reenquadramento "um nível acima"** — ver que o objetivo real é outro do que está
  sendo otimizado; é julgamento do dono, não checklist;
- **o cumprimento ritual** — seguir a forma sem a substância (Fase A que mede o que não
  decide; inferência marcada que ninguém checa). Contra isto, o papel do dono **muda**:
  de "pegar toda deriva" para **auditar se a disciplina é seguida na substância**.

Portanto esta ADR **reduz** a dependência do dono ao freio — **não a elimina**. É "ao
máximo", não "a zero".

**Nota de fundação (o ponto cego de quem escreve).** Esta disciplina foi **escrita pelo
assistente para pegar a própria deriva**. Uma regra que o autor escreve para se pegar tem
um ponto cego: pode ser redigida para ser confortável de seguir sem substância. Por isso
as regras são **verificáveis** (forma detectável), não exortativas, e por isso esta ADR é
**selada pelo dono**, não pelo assistente. É a regra 5 (desconfiar do enquadramento
cômodo) aplicada a si mesma: "uma disciplina que me absolve" é o enquadramento mais
cômodo de todos.

### `claude.md` — regra operacional (curta, aponta para a ADR)

Adicionar à secção de princípios/método:

> **Disciplina anti-deriva (ADR-00NN).** Todo prompt mede antes de decidir: a decisão é
> **precedida** pela medição que a produz (não o contrário). Classifica língua vs
> mecânica da fonte; distingue intenção de comportamento; marca inferência e o que a
> refutaria; desconfia do enquadramento lisonjeiro/cômodo (checagem extra). Aceitação no
> nível da língua, nunca mecânica (exceto onde a mecânica é o observável). Um prompt que
> viole a forma é malformado. O dono audita a substância. Ver ADR-00NN.

---

## Fronteira de parada — revisão do dono
Redigir a ADR e a entrada do `claude.md`, **apresentar o texto** rendido, e **parar**.
Não `--fix-hashes`, não commitar, até o dono revisar e aprovar — em especial o **resíduo**
e a **nota de fundação** (é onde o ponto cego de quem escreve mora). Após a aprovação:
selar (se aplicável) e commitar.

---

## Verificação (gates)
```
content-preserving: zero código/teste/.rs/.toml. Suíte 2726/3245 intacta (não re-rodada).
  Árvore de produto não tocada (só ADR + claude.md).
lint: crystalline-lint . = 0/0 (se o claude.md está sob hash, NÃO selar antes da revisão).
regras verificáveis, não exortativas: cada uma das 6 tem a forma "violação detectável: …"
  — se alguma só disser "sempre faça X" sem o critério de detecção, está malformada.
evidência real: cada modo de falha citado é uma freada real desta branch (β1/noite/==/
  narrativa-da-falha/Revocation), com a referência ao passo — não inventar modos.
honestidade do resíduo: a ADR declara o que NÃO reduz (modos novos, um-nível-acima,
  ritual) e que reduz ≠ elimina.
formato: a ADR segue a convenção da casa; precedente ADR-0107 citado com número confirmado.
```

---

## O que NÃO fazer
- **Não tocar código** — é definição de método.
- **Não escrever regras exortativas** — cada regra tem critério de detecção de violação,
  senão ela não reduz a dependência do dono (só a renomeia).
- **Não afirmar que elimina a dependência do dono** — reduz; o resíduo é declarado.
- **Não inventar modos de falha** — só os reais desta branch.
- **Não selar antes da revisão do dono** — é a fronteira, e a nota de fundação explica por
  quê (o autor da disciplina é quem deriva).
- **Não omitir a nota de fundação** — o ponto cego de quem escreve é parte da disciplina.

---

## Relatório (`typst-passo-349-relatorio.md`)
- O texto rendido da ADR e da entrada do `claude.md`.
- O número de ADR confirmado; o precedente ADR-0107.
- Confirmação de que cada regra tem forma verificável (a lista das 6 com o "violação
  detectável" de cada).
- Confirmação de que parou na fronteira (nada selado) à espera da revisão.
- **Mapa de filtro (campo):** o lugar lógico — "a disciplina anti-deriva é o método
  conhecendo a si mesmo: as freadas que o dono deu viraram a forma que os prompts são
  obrigados a ter; mora na fundação, ao lado da ADR-0107 (a primeira freada-virada-regra)
  — e a honestidade do resíduo é o que a separa de uma promessa vazia de automação" — com
  o rastro (o dono freou N vezes nesta branch; a ADR-0107 encodou uma; a P349 generaliza
  o mecanismo; o resíduo fica com o dono, agora como auditor de substância, não como freio
  de toda deriva).
- Item: `content→elements` aponta para o Marco G (P346).
```
