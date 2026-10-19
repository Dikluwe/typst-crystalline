# Passo 347c — o histórico do vanilla (branch main): o tekt achou uma falha corrigível?

> **Por que este passo.** O P347b concluiu **GEROU** (a terminação por identidade de
> instância é mecânica, não semântica) com uma lacuna: o `lab/` não tinha o histórico
> git do vanilla (I6). O dono informou que **a árvore do vanilla está na branch `main`**
> do mesmo repositório (o código foi movido para `lab/` na branch `tekt`). Isso abre o
> histórico — e muda o objetivo: não é só fechar a citação, é **testar uma tese do
> tekt**: o método identificou uma **falha real e corrigível** do vanilla? Se sim, é um
> ganho do experimento. Este passo mede isso, **read-only**, e **não escolhe** α/β/γ.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P347c (confirmar livre).
**Pré-condição**: P347b fechado (veredito GEROU, lacuna I6 aberta). HEAD pós-P346 na
`tekt`; suíte **2723** / **3242**, lint 0/0, árvore limpa. Se não bater, parar.
**Tipo**: **exploração / medição de intenção e estado** — content-preserving estrito:
**zero código, zero teste, zero `.rs`/`.toml`, zero mudança na árvore de trabalho**.
**Read-only no histórico git**. Termina na **TRAVA** com o veredito atualizado. **NÃO
escolher** α/β/γ; **NÃO** escrever a recursão.
**Objetivo**: três medições — (I7) a intenção na origem; (I8) o estado da correção
upstream; (I9) se o teto-64 é descuido ou troca de performance — para responder: **o
tekt achou uma falha corrigível, e a correção seria melhoria pura ou condicional?**

---

## REGRA DE QUARENTENA (dura — a fronteira do projeto)

A árvore do vanilla na `main` é **oráculo de leitura**, como o `lab/`. Permitido:
inspecionar o **histórico** (`git log`, `git show`, `git log -S`, `git blame`) de
caminhos da `main`. **PROIBIDO**:
- `git checkout main` ou qualquer comando que **mude a árvore de trabalho** da `tekt`;
- `git merge`/`cherry-pick`/`restore` que traga conteúdo da `main` para a `tekt`;
- copiar qualquer linha de código do vanilla para fora do `lab/`/da `main`;
- deixar a árvore da `tekt` modificada ao fim (confirmar `git status` limpo).
Usar `git show <commit>:<path>` e `git log <ref> -- <path>` que **leem** sem trocar de
branch. Se algum comando exigir checkout, **não fazê-lo** — registrar como limite.

---

## Fase A — as três frentes (read-only; evidência com hash de commit / `file:line`)

### Passo 0 — a `main` tem histórico real ou é import achatado?
`git log --oneline main -- typst-realize typst-library 2>/dev/null | head -50` (ou os
caminhos equivalentes na `main`). Decidir:
- **histórico real** (muitos commits, mensagens de autores do Typst) → I7/I8/I9 são
  acessíveis na origem; seguir.
- **import achatado** (1 commit "import vanilla 0.14.2") → a `main` **não** adiciona
  sobre o `lab/`; registrar isso, e I7/I8/I9 ficam limitados ao que a fonte+web já deram
  (o veredito GEROU não muda). Reportar e parar cedo.

### I7 — a intenção na origem (confirma ou reabre GEROU)
Se há histórico: achar os commits que introduziram
- `MAX_SHOW_RULE_DEPTH` / `check_show_depth` (`git log -S "MAX_SHOW_RULE_DEPTH" main`);
- o guard (`guarded`/`is_guarded`/`lifecycle`) (`git log -S "guarded" main`);
- a mensagem `"maximum show rule depth exceeded"` e os hints.
Ler a **mensagem de cada commit** (`git show <hash>`): o autor descreve **conserto
anti-loop / limite protetor** (→ confirma GEROU) ou **design semântico** da terminação
(→ reabre para QUIS)? Citar o hash e o texto.

### I8 — o estado da correção upstream (a falha está aberta?)
A web (P347b-externo) apontou a feature **"Revocation"** como solução planejada. Medir o
estado na `main`:
- procurar "revoke"/"revocation" no histórico e no código da `main`
  (`git log -S "revok" main`; `git grep -i "revok" main -- '*.rs'`);
- há implementação? design-doc/comentário? só menção? **nada**?
Decidir o estado da falha:
- **aberta** (sem conserto na `main`) → corrigi-la é ganho real do tekt;
- **em design** (há plano, sem código) → o tekt pode **antecipar**; comparar a direção;
- **já corrigida** (a `main` é mais nova que o 0.14.2 do `lab/` e já tem Revocation) →
  o tekt não corrige falha aberta; estaria divergindo de uma solução upstream — medir
  qual é a solução upstream e comparar. (Nota: o `lab/` é 0.14.2; a `main` pode ser
  outra versão — registrar a versão da `main`.)

### I9 — o teto-64 é descuido ou troca de performance? (melhoria pura vs condicional)
A correção candidata do cristalino (ponto-fixo morfológico + detecção de ciclo) paga
`==` morfológico por passe + histórico de morfologias. O vanilla escolheu o teto-64
(contador O(1)). Medir **por que**:
- no histórico/comentários/PR do teto e da realização, há menção a **performance / hot
  path / custo de comparar conteúdo**? (`git log -S` no commit do teto; ler a discussão
  se acessível);
- a realização do vanilla é caminho quente declarado (comentários de perf, memoização
  `comemo` ligada a ela)?
Decidir:
- **descuido / limitação reconhecida sem razão de perf** → a correção do cristalino é
  **melhoria** (correção que o vanilla não tinha, sem custo que o vanilla evitava);
- **troca de performance consciente** (o teto evita o custo de comparar conteúdo no hot
  path) → a correção do cristalino é **condicional**: melhora a *correção* ao *custo* de
  performance — coerente com o norte do tekt (clareza/correção > velocidade máxima), mas
  **não** "o vanilla errou". Registrar honestamente como prioridade diferente, não
  superioridade.

---

## TRAVA — o veredito atualizado (parar; não escolher α/β/γ)

Emitir, com evidência (hash/`file:line`):
- **I7**: GEROU **confirmado na origem** (a intenção do autor era anti-loop) ou
  **reaberto** (havia design semântico). 
- **I8**: o estado da falha — **aberta** / **em design (Revocation)** / **já corrigida**
  — e, se há solução upstream, qual é e como se compara à do cristalino.
- **I9**: a correção do cristalino é **melhoria pura** ou **condicional (correção vs
  performance)**.
- **A tese do tekt, respondida**: o método identificou uma falha real e corrigível? Em
  que sentido o cristalino melhoraria o vanilla (correção? clareza? ambas?), e a que
  custo? Com a honestidade do I9 — melhoria pura só se não ignora uma razão do vanilla.

Isto **informa** α/β/γ (não escolhe): se I8 = aberta e I9 = melhoria → **α** ganha força
como "antecipa o conserto / corrige a falha"; se I8 = já corrigida → comparar com a
solução upstream antes de α; se I9 = condicional → α é prioridade-diferente registrada,
não vitória. **Parar** para a decisão do dono.

---

## Verificação (gates)
```
content-preserving: zero código/teste/.rs/.toml. Árvore da `tekt` INTACTA (git status
  limpo ao fim — confirmar). Suíte 2723/3242 não re-rodada (nada de código). Nenhum
  checkout que mude a árvore; nenhum conteúdo da `main` trazido para a `tekt`.
lint: crystalline-lint . = 0/0.
evidência: cada achado com hash de commit (git show) ou file:line; a versão da `main`
  registrada. Inferências marcadas. Zero "~".
quarentena: confirmar que só houve leitura de histórico (log/show/grep/blame), sem
  checkout/merge/restore. Se algum comando exigiu checkout, NÃO foi feito (registrar).
fronteira: o passo NÃO escolheu α/β/γ — entregou o veredito atualizado e parou.
```

---

## O que NÃO fazer
- **Não trocar de branch nem mudar a árvore** — read-only no histórico (a quarentena).
- **Não trazer código do vanilla** para a `tekt` — leitura de intenção, não de código.
- **Não escolher α/β/γ** nem escrever a recursão.
- **Não afirmar "o tekt melhorou o vanilla" sem o I9** — melhoria pura só se não ignora
  uma razão de performance do vanilla; senão é prioridade diferente, dito como tal.
- **Não inferir intenção sem marcar** — "o commit conserta loop" é evidência; "o autor
  nunca quis a distinção" é inferência.
- **Não estimar com "~"**.

---

## Relatório (`typst-passo-347c-relatorio.md`)
- Passo 0: a `main` tem histórico real ou import achatado; a versão da `main`.
- I7: os commits do guard/teto/mensagem (hash + texto do autor) — GEROU confirmado ou
  reaberto.
- I8: o estado da Revocation/da falha (aberta / em design / corrigida) + a solução
  upstream se houver.
- I9: o teto-64 — descuido ou troca de performance (evidência); melhoria pura vs
  condicional.
- **VEREDITO ATUALIZADO** + **a tese do tekt respondida**: achou falha corrigível? que
  melhoria, a que custo? — com a honestidade do I9.
- O que isso implica para α/β/γ (sem escolher).
- **Mapa de filtro (campo):** o lugar lógico — "o tekt, ao reconstruir, testou se
  conseguia **identificar e corrigir** uma falha do vanilla; a recursão de `#show` é o
  primeiro caso onde isso foi a sério — e a honestidade (I9) separa 'corrigimos uma
  falha' de 'escolhemos outra prioridade'" — com o rastro (P347 mediu o comportamento;
  P347b inferiu GEROU da fonte; web confirmou 'limitação a resolver' + Revocation; P347c
  busca a intenção na origem e o estado do conserto).
- Item: `content→elements` aponta para o Marco G (P346).
```
