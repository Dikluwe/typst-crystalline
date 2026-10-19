# Passo 347c — histórico do vanilla (branch `main`): o tekt achou uma falha corrigível?

> **Veredito atualizado: NÃO — não é uma falha; é um design deliberado e completo.** O
> histórico real do vanilla (`main`, 4476 commits) mostra que a recursão auto-casante de
> `#show` foi **intencional** (#3327 "Support text show rules that match their own output",
> Laurenz, 2024), com um mecanismo **completo**: guard por-instância **+ Revocation**
> (escotilha de usuário, **já existe** no 0.14.2) **+ teto** como backstop genérico
> (família `MAX_*_DEPTH`, #4845). Isto **refina o GEROU do P347b** e **corrige o seu
> enquadramento**: a *feature* de auto-casamento foi **QUIS**; só a **borda de identidade
> de instância** (I4) é artefato de mecanismo — e até ela tem mitigação (revocation).
> Logo o tekt **não achou uma falha corrigível**; achou uma **diferença de prioridade de
> design**. A correção candidata (α, ponto-fixo morfológico) é **condicional** (paga
> comparação de conteúdo no caminho de realização; diverge de um design deliberado),
> **não** "o vanilla errou". Read-only: árvore da `tekt` intacta, lint 0/0, suíte
> 2723/3242 não re-rodada.

## Quarentena (confirmada)
Só leitura de histórico (`git log/show/grep/ls-tree -- main`); **nenhum** checkout/merge/
restore; branch permaneceu `Tekt`; `git status` de produto limpo ao fim. Nenhum código do
vanilla trazido para a `tekt`.

## Passo 0 — a `main` tem histórico real
**Sim.** `main` = **4476 commits** (histórico real do Typst; autores reais). Código do
vanilla em `crates/` (movido para `lab/` na `tekt`). **Versão `main` = 0.14.2** (=`lab/`);
**`upstream/main` também = 0.14.2** (`main..upstream/main` = **0 commits**) → não há
upstream mais novo acessível; **0.14.2 é o estado atual** (a Revocation já é o presente,
não um futuro).

## I7 — a intenção na origem: **GEROU refinado → a feature foi QUIS**
- **`92aba81a9` — "Support text show rules that match their own output (#3327)"** (Laurenz,
  2024-02-05). É o commit que introduz a mensagem `"maximum show rule depth exceeded"`, o
  hint, **e** a Revocation. O título declara a **intenção**: *suportar* regras que casam o
  próprio output — i.e., a **revisitação/auto-casamento é feature desejada**, não acidente.
- `MAX_SHOW_RULE_DEPTH` tocado por **`92f2c7b47` — "Refactor depth checks and apply them in
  math (#4845)"** (Laurenz, 2024-08-27): o teto é refatorado como **família genérica**
  (`MAX_LAYOUT_DEPTH`/`MAX_HTML_DEPTH`/`MAX_CALL_DEPTH`/`MAX_SHOW_RULE_DEPTH`,
  `engine.rs:335-344`), não um limite específico de show.
- **Conclusão I7:** o P347b inferiu **GEROU** da ausência de promessa na fonte do `lab/`. O
  histórico **refina**: a *semântica de auto-casamento* foi **QUIS** (#3327 a nomeia como
  feature); o **guard + revocation** são o **mecanismo desenhado** para torná-la limitada;
  o **teto** é backstop genérico. A **borda de identidade de instância** (I4: reconstruir
  idêntico → recursa) permanece **artefato de mecanismo** (GEROU **na borda**), mas o
  modelo global é **deliberado**. *(Marca de inferência: o título do commit é evidência de
  intenção da feature; que a borda-I4 específica seja não-intencional é inferência minha —
  nenhum commit a discute.)*

## I8 — o estado da correção upstream: **a "Revocation" já está implementada (não é falha aberta)**
A web (externa, P347b) apontou "Revocation" como solução planejada. Medido na fonte:
**`Style::Revocation(RecipeIndex)`** (`foundations/styles.rs:225`) **já existe no 0.14.2**,
introduzida em **#3327** (refinada em `#3408`, `#4840`). Uso (`typst-realize/lib.rs:1223-
1259`): em text rules, uma `Revocation(index)` no style chain **revoga** o recipe no
subárvore que ele produz (`revoked.insert(index.0)`; `if revoked.contains(index.0)
{ continue }`) — a regra aplica **uma vez** e não re-casa o próprio output.
- **Estado da falha: NÃO há falha aberta.** O modelo é **deliberado e completo**: text
  rules → **revocation** (apply-once); element rules → **guard por-instância** + **teto**
  backstop; e o usuário tem **controle explícito** (revocation). Não é um bug à espera de
  conserto; é a solução upstream, **presente**.

## I9 — o teto-64: descuido ou troca de performance? → **limite de segurança genérico (nem um nem outro)**
- O teto **não é** uma otimização específica de show: é um membro da **família uniforme
  `MAX_*_DEPTH`** (layout/html/call/show), refatorada junta em **#4845**. É um **limite de
  segurança genérico** (anti-runaway/stack), não uma escolha "contador-vs-comparar-conteúdo"
  no caminho de show.
- A **terminação real** de show **não é o teto** — é o **guard + revocation** (O(1):
  bitset `lifecycle` / `SmallBitSet` revoked). O teto só pega o que esses não pegam
  (recursão de conteúdo fresco genuíno).
- A candidata α (ponto-fixo morfológico + histórico de morfologias) substituiria
  guard+revocation por **comparação de conteúdo por passe** — O(árvore)/passe vs O(1) do
  guard, **no caminho de realização** (`realize`, comemo-tracked). *(Inferência marcada: a
  diferença assintótica O(1)-guard vs O(árvore)-compare é real; que o vanilla a tenha
  evitado **conscientemente por perf** não está documentado num commit — o que está
  documentado é que a terminação é guard+revocation, não comparação de conteúdo.)*
- **Conclusão I9:** α é **condicional** — troca o modelo de identidade (O(1), com escotilha
  revocation) por convergência morfológica (custo de comparar conteúdo + sem a escotilha),
  **prioridade diferente** (clareza/graça > mecanismo de identidade), **não** "o vanilla
  errou nem foi descuidado".

## VEREDITO ATUALIZADO + a tese do tekt, respondida
**O tekt NÃO identificou uma falha real e corrigível.** Identificou uma **diferença de
prioridade de design** num modelo **deliberado e completo** do vanilla:
- a recursão auto-casante é **feature intencional** (#3327, I7);
- a terminação tem **mecanismo desenhado + escotilha de usuário** (guard + **revocation**
  já presente, I8) + backstop genérico (teto, I9);
- a "borda" que o P347/P347b mediram (identidade de instância: reconstruir idêntico →
  recursa) é **artefato do mecanismo de guard**, mas **mitigável** pela revocation e não é
  um bug declarado.

**Em que o cristalino "melhoraria":** α (morfológico) daria uma terminação que **vê a
convergência** que o modelo de identidade não vê (mais graciosa: `o_inf` → "Z" em vez de
erro). Isso é **clareza/correção-de-forma**, coerente com o norte (ADR-0107), **mas a
custo**: comparação de conteúdo no caminho de realização **e** abrir mão da escotilha
revocation do vanilla. → **melhoria condicional (prioridade diferente), não pura.**

**Correção honesta ao P347b:** o P347b enquadrou α como "fidelidade à intenção que o
vanilla traiu". O histórico **desautoriza** esse enquadramento — a intenção do vanilla
**foi** o auto-casamento bounded por guard+revocation (deliberado). α é um **modelo
diferente**, não a "intenção verdadeira" recuperada. (O experimento se autocorrige: a
evidência da origem rebaixou "achamos uma falha" para "escolheríamos outra prioridade".)

## O que implica para α/β/γ (sem escolher)
- **α** — reenquadrado: divergência **consciente de prioridade** (não conserto de falha);
  defensável pela graça/clareza, registrável como tal; custo de perf + perda da revocation.
- **β** — reproduz um modelo **deliberado e completo**; para paridade real **precisaria
  também da revocation** (não só guard+teto) — maior do que o P347 dimensionou.
- **γ** — fatiar (revisitação convergente já; terminação/erro depois) segue viável e
  neutro quanto à tese.

A escolha continua do **dono**, agora sabendo que **não é "corrigir um bug do vanilla"** —
é escolher entre reproduzir um design deliberado (β), divergir por prioridade consciente
(α), ou fatiar (γ). **Parei; não escolhi.**

## Verificação (gates)
```
content-preserving: zero código/teste/.rs/.toml. Árvore da `tekt` intacta (git status de
  produto limpo). Suíte 2723/3242 não re-rodada. Sem checkout/merge/restore — só
  log/show/grep/ls-tree read-only.
lint: crystalline-lint . = 0/0.
evidência: hashes 92aba81a9 (#3327), 92f2c7b47 (#4845); styles.rs:225, lib.rs:1223-1259;
  versões main=upstream/main=0.14.2. Inferências marcadas (borda-I4 não-intencional; perf
  do teto). Zero "~".
quarentena: confirmada — só leitura de histórico; nenhum conteúdo da `main` na `tekt`.
fronteira: não escolheu α/β/γ; veredito atualizado e parada.
```

## Mapa de filtro (campo)
**Lugar lógico:** o tekt, ao reconstruir, testou se conseguia **identificar e corrigir** uma
falha do vanilla; a recursão de `#show` foi o primeiro caso a sério — e a honestidade
(I7-I9) **separou "corrigimos uma falha" de "escolhemos outra prioridade"**: o histórico
mostrou um design **deliberado** (não falha), então o ganho do tekt aqui, se houver, é
**clareza/graça de forma** (α), não correção de bug. **Rastro:** P347 mediu o comportamento
(por instância); P347b inferiu GEROU da fonte do `lab/` (sem histórico, I6); P347c abriu o
histórico real e **refinou para "feature QUIS + borda-mecanismo + design completo com
revocation"**, corrigindo o enquadramento de α. A escolha α/β/γ vem depois, sem a ilusão
de "consertar o vanilla".

## Item carregado
`content→elements` aponta para o **Marco G** (P346) — não volta como órfão.
```
