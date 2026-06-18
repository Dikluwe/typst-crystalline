# F-recon caso 1 (composição) — reconciliar com α ou divergência declarada (P354)

> **Tipo**: recon read-only (zero código de produto, zero L0). Probes descartáveis compiladas e
> **revertidas** (árvore limpa; suíte **2733/0** antes e depois; `RUST_MIN_STACK=33554432`). Mede a
> colisão caso-1/α que o P352 Fase A §4 achou; entrega **(A) reconciliar** e **(B) divergência
> declarada** com custo e `file:line`; **recomendação marcada**; **a decisão é do dono** (ADR-0108:
> mede, não decide). Pré-condição: P352/P353 fechados, HEAD pós-P353.

---

## 1 — O modelo do vanilla, em cheio (leitura da quarentena, `file:line`)

**Iteração innermost-first, 1 func/passe** (`lab/typst-realize/src/lib.rs:449-486`,
`foundations/styles.rs:835`):
- `for (r, recipe) in styles.recipes().enumerate()` (`lib.rs:449`); `Entries::next` consome o link
  da cabeça **em reverso** (`inner.next_back()`, `styles.rs:835`) → a recipe **mais recente /
  innermost** é vista **primeiro**.
- A **primeira** recipe func não-guardada vira o passo; as demais func são saltadas nesse passe
  (`lib.rs:467-469`, `if step.is_some() { continue }`). → **≤1 transformação func por passe**.

**Guard por-instância (`RecipeIndex`)** — o mecanismo que dá composição **e** termina recursão:
- `let index = RecipeIndex(*depth - r); if elem.is_guarded(index) { continue; }` (`lib.rs:472-474`).
- O guard mora no **bitset de lifecycle do elemento empacotado**: `is_guarded`/`guarded` inserem em
  `meta().lifecycle` (`content/mod.rs:148-156`). É **por-instância**.
- Ao aplicar, o **input é guardado** e passado ao closure: `recipe.apply(.., output.into_owned().
  guarded(guard))` (`lib.rs:363-367`). O output é **re-realizado** (`visit_styled`, multi-passe;
  teto `route.check_show_depth`, `lib.rs:401-407`).

**Como o vanilla reconcilia composição e recursão** (a pergunta-chave do passo): o **mesmo** guard
por-instância serve **dois papéis**:
- **(i) composição** — recipe R1 aplica ao nó (guarda a instância para o índice de R1); o output,
  se **carrega** a instância guardada (o caso comum: o closure embrulha/retorna `it`), faz a
  re-realização **saltar R1** → R2 (índice distinto, não guardado) aplica. **N regras distintas,
  uma vez cada, innermost-first, acumulando.**
- **(ii) terminação de recursão** — se o closure produz uma **instância nova** do mesmo kind (ex.:
  `it => [= Z]`, heading fresco **não** guardado), a recipe **re-aplica** → o vanilla **erra** no
  teto (`maximum show rule depth exceeded`) quando não converge por instância.

→ **Sim: o vanilla reconcilia os dois via guard por-instância + recursão por instâncias novas.** Os
dois papéis são o **mesmo** maquinário (o bitset por-instância).

---

## 2 — A decisão P347d/P348: o que "GEROU", e bloqueia a composição?

Do relatório do P348 (`typst-passo-348-relatorio.md:23-30,126-140`) e do L0 `f_fronteira_e1.md`
§3a.7-bis (ATERRADO P348):
- Reproduzir o vanilla **exato** (terminar por **identidade de instância**) exigiria multi-passe +
  guard-por-instância + Revocation. Mas essa terminação é **GEROU** (mecânica, **não** promessa de
  língua — P347b/c, commit #3327 "Support text show rules that match their own output") e a
  **Revocation é INTERNA** (P347d — só o motor a constrói, sem porta de usuário). Por ADR-0107
  (paridade é com a língua, não a mecânica), o cristalino **recusou** portá-la e adotou o
  **ponto-fixo morfológico (α)** como **divergência consciente** (`#show heading: it => [= Z]`
  **converge para "Z"** onde o vanilla erra).
- **O que a recusa cobria (medido):** o relatório do P348 (`:27-28`) diz — *"a travessia dá a mesma
  **composição** (P341/p3b já mediu a cascata A→B fiel); a única lacuna era a **recursão da mesma
  regra** ao próprio output — agora o loop a resolve."* Ou seja, o P348 considerou a composição
  **cross-kind cascata** (R1 output é de **outro** kind que R2 casa) **já resolvida** (via
  interceção aninhada) e tratou **só** a **recursão**. A composição **same-kind multi-regra (B1)**
  — N regras sobre o **mesmo** elemento, acumulando — **não foi endereçada** (caso 1 adiado).

**Conclusão (medida):** a recusa do P347d/P348 foi do **papel (ii)** (terminação de recursão por
identidade de instância) — **não** do **papel (i)** (composição same-kind por recipe-uma-vez). O
papel (i) **nunca foi construído** (caso 1 adiado), **não foi rejeitado**. Mas no vanilla os dois
papéis são o **mesmo** bitset por-instância — logo trazer o papel (i) "à letra do vanilla" reabre o
maquinário por-instância que o α substituiu.

---

## 3 — O cristalino hoje (medido, `file:line` + probes revertidas)

**O loop α** (`rules/eval/rules.rs:97-248`): aplica a **primeira regra declarada que casa**
(`:124-187`, `for rule in &node_rules` → primeiro match, `break`), re-alimenta o output
(`work = out`), termina por **ponto-fixo morfológico** (`:210-213`, `out.morph_canon()==work.
morph_canon()`) ou teto-64 (`:214`). **Sem guard por-regra entre revisitas.**

**O que funciona (medido):**
- **Cascata cross-kind** (R1 output é outro kind que R2 casa) — via **interceção aninhada**: o
  closure de uma regra **nativa**, ao criar conteúdo, passa por `intercept_content` (`markup.rs:100`
  etc.) com `active_guards` contendo a regra atual mas **não** as outras → a outra regra aplica
  durante a criação. Teste vivo: `show_rule_encadeamento_duas_regras` (heading→strong, strong→emph).
- **Recursão same-rule** — via α (ponto-fixo morfológico). Teste: `p348_show_recursao_converge_
  para_ponto_fixo` (a→b→c).

**O que NÃO funciona — same-kind multi-regra (B1), o caso 1 (probes P354, revertidas):**
- **output muda de tipo** — `#show heading: it=>[AA ]+it.body` ⨁ `[BB ]+it.body`, `= orig` →
  **`"AA orig"`**: só a **1ª declarada** aplica; a 2ª é **faminta** (o output é Sequence, não casa
  heading). [+ P352 PROBE1a: a ordem diverge — cristalino aplica a **1ª declarada**, vanilla a
  **innermost / mais-recente**.]
- **ordem** — `#show callout: it=>[R1FIRST]` ⨁ `[R2FIRST]` → **`"R1FIRST"`**: o loop pega a
  **1ª declarada**.
- **output fica do mesmo kind (nativo)** — `it=>[= AA]` ⨁ `[= BB]`, `= orig` → **`"BB"`**: encadeia
  via interceção aninhada (R2 dispara dentro da criação do `[= AA]` de R1) + ponto-fixo morfológico;
  **converge para UM output** (o de R2), **não acumula** os dois efeitos.
- **ctor dinâmico não passa por intercept** (`closures.rs:71`, `FuncRepr::Element(ef) =>
  Ok((ef.ctor)(..))` — sem `intercept_content`) → a cascata aninhada **não** vale para conteúdo
  produzido por elemento dinâmico (assimetria nativo vs dinâmico).

**Conclusão (medida):** o cristalino **não tem** a composição acumulativa same-kind do vanilla
(N regras, uma vez cada, innermost-first, acumulando). Tem α (recursão) + interceção aninhada
(cascata cross-kind) — o **guard por-recipe-uma-vez** (papel (i)) **falta**.

---

## 4 — Opção (A): reconciliar — modelos, custo, `file:line`

### A1 — guard por-`RuleId`-uma-vez (sem instância) — **REJEITADO** (regride caso 2)
Guardar cada `RuleId` após a 1ª aplicação no nó. **Dá** a composição (R1 uma vez → R2 aplica). **Mas**
uma regra guardada **não pode** re-aplicar ao seu próprio output recursivo → **quebra o α**: o
`p348_show_recursao_converge_para_ponto_fixo` (a→b→c) pararia em **b**. **Regressão do caso 2
(fechado)** — proibido pelos limites duros. *Onde tocaria:* `rules.rs:97-248`. **Não viável.**

### A2 — guard por-`(RuleId, morph_canon)` — **VIÁVEL, mas diverge na ORDEM**
Uma regra só re-aplica a uma **morfologia nova** (par `(RuleId, morph_canon)` não visto). Assim:
- **recursão a→b→c preservada** (cada passo é morfologia distinta → α intacto; `p348` verde).
- **regra não re-aplica à mesma morfologia** (sem loop infinito por idempotência).
- **composição**: regras distintas aplicam (cada uma à sua vez).

**Custo medido:** a ordem **diverge do vanilla** — dá "R1 até ponto-fixo, depois R2", não o
**interleaved one-recipe-per-pass** innermost-first do vanilla. Para regras same-kind que **não
comutam**, o resultado difere do vanilla. *Não* reabre a identidade de instância / o "GEROU" (usa
`RuleId` + morfologia — ADR-0107). *Onde tocaria:* só `rules.rs:97-248` (adicionar um conjunto
`seen: Set<(RuleId, MorphCanon)>` e mudar a seleção de regra para preferir a não-aplicada-nesta-
morfologia). `morph_canon` já existe (P345). *Testes:* `p348_*` deve ficar verde (a trava); novos
testes de composição. **Risco:** a fronteira "regra que é **ao mesmo tempo** parceira de composição
**e** auto-recursiva" precisa de cuidado; a divergência de ordem é uma **nova divergência consciente
a declarar**. **Reabre P347d/P348?** **Não** (não traz identidade de instância). É um **3º modelo**,
nem vanilla-exato nem o α-puro atual.

### A3 — copiar o guard por-instância + multi-passe do vanilla — **paridade exata, mas reabre o α**
Trazer o `RecipeIndex` por-instância (`content/mod.rs:148-156`) + multi-passe externo. **Dá paridade
exata** (composição interleaved innermost-first **e** recursão por instâncias novas). **Mas reabre o
P347d/P348**: ressuscita a identidade de instância que o α **substituiu** por decisão consciente
(ADR-0107), e **substitui o caso 2 fechado**. Maior superfície (toca o `Content`/meta dos 65 nativos
ou um wrapper `Content::Guarded` — o que a Trava-Q1 do L0 deixou "dispensado" no P348). **Só se o
dono quiser reverter a decisão α.**

**Modelo achado?** **Sim — A2** dá composição **e** preserva o α **sem** reabrir o "GEROU", ao custo
de uma **divergência de ordem** (não-comutativas same-kind). Paridade **exata** só com **A3**
(reabre o α). **A1 não é viável** (regride o caso 2).

---

## 5 — Opção (B): divergência consciente declarada

O cristalino **aplica a 1ª regra declarada que casa** (+ cascata cross-kind via interceção aninhada
+ recursão via α) — declarado como **divergência consciente** vs vanilla (que aplica **todas** as
same-kind, uma vez cada, innermost-first, acumulando).

**O custo (nomeado, não escondido):**
- **B1 (2+ regras same-kind sobre o mesmo elemento) diverge**: só a 1ª declarada tem efeito quando o
  output muda de kind (a 2ª é faminta); quando o output fica do mesmo kind, encadeia-mas-sobrescreve
  (converge a um output, não acumula). A cascata cross-kind e a recursão **não** são afetadas.
- **Sub-melhoria barata, independente da escolha:** inverter a iteração de `node_rules` para
  **innermost-first** (mais-recente-declarada primeiro) — casa a polaridade do vanilla
  ("última-declarada vence") no subconjunto de **uma-regra-efetiva** (P352 PROBE1a/1b). **Não** toca
  o α. Pode entrar em qualquer rumo.

**Gatilho de reabertura concreto** (padrão SetPage/show, ADR registrada): quando a **cobertura da
linguagem** exigir composição acumulativa same-kind real (um pacote/autor a depender de 2 `#show
heading` que acumulam), os casos B1 viram **testes de paridade** contra o **vanilla medido**, e o
caso 1 vira **lote** nessa hora (com A2 ou A3 conforme a paridade exigida).

---

## 6 — Recomendação (marcada) — a DECISÃO é do dono

**Recomendação do agente: Opção (B) (divergência declarada) + a sub-melhoria de ordem innermost-
first.** Razões medidas:
1. **ADR-0107** — a paridade é com a **língua**; o α já é uma divergência consciente da **mecânica**
   de terminação. A composição acumulativa same-kind é mecânica de realização (ordem de passes,
   guard por-instância) — exatamente a classe que o projeto diverge de propósito.
2. **Demanda não medida** — nenhuma cobertura de linguagem atual exercita B1 acumulativo (0 testes;
   o caso é raro em documentos reais). Construir A2/A3 agora é **infra à frente da demanda**
   (ADR-0107) — o mesmo critério que fechou o de-bake do P353 em (b).
3. **Custo/risco** — mesmo "reconciliar" (A2) **não** dá paridade exata (diverge na ordem das
   não-comutativas) e adiciona risco na fronteira composição/recursão; A3 reabre o α (caso 2
   fechado). B é **zero-risco** e **nomeia** a divergência com gatilho.

**Se o dono quiser composição agora:** **A2** é o caminho de menor reabertura (não toca o "GEROU"),
aceitando a divergência de ordem declarada; **A3** só se a paridade exata de ordem for requisito (e
aí reabre-se a decisão α conscientemente).

**A escolha (A2 / A3 / B) é do dono.** O próximo lote (implementação ou divergência registrada em
ADR/L0) só nasce **depois** dela. Nenhum código/L0 tocado neste recon; suíte 2733/0; árvore limpa.

---

## 7 — DECISÃO DO DONO (P354): **Opção (B)**

Escolhido **(B) — divergência consciente declarada + ordem innermost-first**. O próximo lote (P355)
materializa: (1) **declarar** a divergência (cristalino aplica a 1ª regra que casa + cascata
cross-kind + α; **não** acumula same-kind como o vanilla) numa **ADR/L0** com o **gatilho de
reabertura** (cobertura de linguagem exigir B1 acumulativo → testes de paridade → caso 1 vira lote
com A2/A3); (2) a **sub-melhoria barata**: inverter a iteração de `node_rules` (`rules.rs:124`) para
**innermost-first** (mais-recente-declarada primeiro), casando a polaridade "última-declarada vence"
do vanilla no subconjunto de uma-regra-efetiva — **sem** tocar o α/caso 2. A2/A3 ficam descartados
(infra à frente da demanda, ADR-0107). Este recon (P354) **fecha** aqui; a implementação é o lote
seguinte.
