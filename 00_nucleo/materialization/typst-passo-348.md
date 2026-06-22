# Passo 348 — recursão de `#show`: o modelo α (ponto-fixo morfológico + backstop + flag)

> **O que fecha.** O resíduo do `m1` (`b`≠`c`): o cristalino passa a **revisitar** o
> output de uma element rule até **ponto-fixo morfológico** (usa o `==` do P345 — para
> quando o output não muda a morfologia), com **teto** como backstop puro de runaway
> (número = mecânica) e uma **flag de erro completo** que classifica (cíclico /
> divergente / converge-fundo) num **canal de hint separado** (mensagem base
> byte-idêntica ao vanilla). É o **modelo α** decidido na cadeia P347→P347d: **não**
> reproduz o guard-por-instância do vanilla (mecânica que o vanilla considera limitação)
> nem a **Revocation** (interna, P347d) — substitui os dois pela convergência
> morfológica. A única divergência consciente do vanilla: `o_inf`
> (`#show heading: it => [= Z]`) **converge para "Z"** onde o vanilla erra. Primeiro
> lote de código desde o P345. **NÃO** content-preserving.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P348 (confirmar livre).
**Pré-condição**: P347d fechado (veredito INTERNA; α decidido sem Revocation). HEAD pós-
P346; P345 commitado (`3ebb397fb`); os relatórios P347/b/c/d são doc-only (untracked ou
commit de doc — confirmar que nenhum `.rs` mudou desde o P345). Suíte **2723**
(`typst-core --lib`) / **3242** (workspace), lint 0/0, árvore de produto limpa. Se não
bater, parar.
**Tipo**: recursão de `#show`, modelo α — **NÃO content-preserving**: muda a semântica
de terminação/revisitação de propósito. Regra do P340 + **ADR-0107**: a paridade e a
aceitação são **morfológicas** (a morfologia da árvore final = a do vanilla nos casos
convergentes; recursão sem convergência → erro), **exceto** a **mensagem base de erro**,
que é **comportamento observável** (ADR-0033) e portanto **byte-idêntica** ao vanilla. O
vanilla é oráculo da **semântica**.
**Limites duros**:
- **não** tocar o `==` morfológico (P345) nem o `PartialEq` do Rust (dois sistemas);
- **não** reproduzir a **Revocation** (P347d: INTERNA — mecânica que o cristalino
  substitui, não carrega);
- **não** mexer em **text rules** (`map_text`, passe único — confirmar na Fase A que não
  recursam; a Revocation existia só para elas no vanilla, e o cristalino já não as
  revisita);
- **não** des-assar nada (de-bake = F-5, **disjunto** por M4 do P347 — este lote não o
  toca);
- **não** consertar nada além da recursão de element rules.
**Objetivo**: element rules cujo output **re-casa** passam a ser **revisitadas** até
ponto-fixo morfológico; recursão que não converge é cortada pelo teto (backstop) e vira
erro com a mensagem base do vanilla; a flag de erro completo classifica no canal de hint.
O `m1` vai de "b" → **"c"**.
**Fontes**: relatórios P347 (M1 a semântica de revisitação; M2 as formas; M3 o que
exige multi-passe **para paridade exata** — α **não** é exata; M4 de-bake disjunto;
`apply_show_rules` `rules.rs:90-172`, `intercept_content` `rules.rs:193`, `visit_show_
rules`/`visit_styled` do vanilla em `lab/`), P347b/c/d (GEROU→QUIS-com-borda; α como
prioridade consciente; Revocation INTERNA), P345 (`Content::morph_canon`, o `==`
morfológico — a ferramenta da terminação), P341b (o `m1`; os testes anti-recursão
`f3s2_show_callout_anti_recursao_termina`, `show_rule_nao_recursiva_sem_stack_overflow`),
**ADR-0107** (paridade morfológica), `lab/typst-original/` (oráculo da semântica e da
**mensagem base** exata; `file:line`).
**Commits** (isoláveis): "Passo 348 — caronas" · "Passo 348 — Fase A (eager vs
multi-passe + oráculo)" · "Passo 348 — L0 da terminação (se superfície)" · "Passo 348 —
revisitação até ponto-fixo" · "Passo 348 — teto backstop + mensagem base" · "Passo 348 —
flag de erro completo (classificação)" · "Passo 348 — evolução de testes + divergência
o_inf registrada".

---

## Caronas (commit próprio)
- **C0 — base exata**: confirmar 2723 / 3242 no HEAD; nenhum `.rs` mudou desde o P345
  (os 347* são doc). Fixar antes de delta.
- **C1 — âncora `m1` e o registro da divergência**: registrar o `m1` atual (`= a` → "b")
  como o alvo deste lote (→ "c"); e abrir, em nota de paridade (ADR-0107), a divergência
  `o_inf` que este lote introduz **conscientemente** (vanilla erra; α converge para "Z").

---

## Fase A — as medições que decidem o lote (`file:line`; sem código de produção)

### M-eager — a fronteira real: α cabe no eager ou exige multi-passe?
O P347/M3 mediu que reproduzir o vanilla **exato** exige multi-passe (guard por-instância
+ realização recursiva). **α não é exato** — termina por morfologia, não por identidade.
A pergunta reaberta, só para α: a **revisitação de element rules até ponto-fixo** cabe num
**loop local** no caminho eager (`apply_show_rules`/`intercept_content`: aplica a regra,
re-alimenta o output no mesmo caminho, compara morfologia, repete), **ou** o output
precisa re-entrar a avaliação completa (o que exigiria a camada de realização recursiva)?
Medir da fonte o que a re-alimentação do output toca. **Decisão:**
- **eager-viável** (loop local basta; o output re-processado por `apply_show_rules` dá a
  mesma composição) → este lote executa, é o lote menor.
- **exige multi-passe** → **fronteira de decisão**: parar na trava; é o lote grande que o
  P340 estacionou, e o dono decide executar agora ou fatiar. (Suspeita a priori:
  eager-viável, porque α não precisa do guard propagado; mas **medir**, não supor.)

### M-trigger — quando a revisitação dispara, e o custo
Medir: em que casos o output de uma element rule **re-casa** (dispara revisitação) vs
**não re-casa** (a regra aplica uma vez, sem custo extra — o caso comum). Confirmar que o
caminho comum (output não re-casa) **não** paga `morph_canon` extra; o custo do ponto-fixo
só é pago quando há revisitação real. (É o que mantém o caminho quente barato — a base do
desenho híbrido.)

### M-oráculo — os casos, medidos no vanilla e no cristalino atual
Compilar (binário do P341) e tabelar o alvo de cada caso:

| caso | `.typ` | vanilla | crist atual | alvo α |
|---|---|---|---|---|
| convergente (`m1`) | `it.body==[a]→[= b]; ==[b]→[= c]; else it` · `= a` | **c** | b | **c** |
| `o_inf` (rewrite p/ constante) | `#show heading: it => [= Z]` · `= a` | **erro** | Z (trunca) | **Z** (converge — divergência consciente) |
| ciclo | `==[a]→[= b]; else [= a]` · `= a` | **erro** | b | **erro** (teto backstop; flag classifica "cíclico") |
| divergente (cresce) | regra que embrulha o output a cada passe | **erro** | — | **erro** (teto; flag "divergente") |
| profundo legítimo | recursão que converge fundo, abaixo do teto | (compila) | — | **compila** (ponto-fixo para antes do teto) |

### M-msg — a mensagem base e o canal de hint (do vanilla)
Confirmar o texto byte-exato da mensagem base (`"maximum show rule depth exceeded"`,
`engine.rs:350`) e que o `SourceDiagnostic` cristalino tem canal de hint separado (P347
confirmou: `with_hint`). A flag escreve **só** no hint; a mensagem base **não** muda.

### M-text — text rules não recursam (confirmar o limite)
Confirmar que `map_text` (`rules.rs:175-181`) é passe único e o output de uma text rule
**não** re-casa (o cristalino já não as revisita) — então a Revocation (que no vanilla
servia só para elas) **não tem papel** no cristalino, e este lote **não** toca text rules.

**Saída da Fase A**: M-eager (a fronteira), M-trigger (o custo), M-oráculo (os alvos),
M-msg (a mensagem base), M-text (o limite). Sem código antes disto.

---

## TRAVA — checkpoint
**Parar** se:
- **M-eager = multi-passe** → fronteira de decisão do dono (lote grande; o P340).
- M-oráculo revelar um caso novo que diverge do vanilla além do `o_inf` previsto (uma
  divergência não antecipada → o dono abençoa ou não).
- L0 de terminação for superfície nova → escrever, `--fix-hashes`, parar para selar.
- uma asserção que o vanilla **confirma** teria de mudar → achado, parar.
**Seguir sem parar** se M-eager = eager-viável, M-oráculo bate com os alvos da tabela
(só o `o_inf` diverge, e já está abençoado), e a mensagem base é clara.

---

## Fase B — execução (após a trava; só se eager-viável)

### Estágio L0 — a terminação (se M criou superfície)
Atualizar o L0 da realização (`f_fronteira_e1.md §3a.7` / `rules/eval.md`): o modelo de
terminação morfológico (ponto-fixo + teto backstop + flag), a divergência `o_inf`
registrada, e que a Revocation **não** é reproduzida (INTERNA). `--fix-hashes`; parar
para selar se superfície nova.

### Estágio R — revisitação até ponto-fixo (teste morfológico primeiro)
1. Teste: o `m1` produz a **mesma morfologia** que o vanilla (→ "c"); confirmar que
   **falha** antes (dá "b").
2. Código: no caminho eager de element rules, o output de uma regra que **re-casa** é
   re-alimentado no mesmo caminho; o loop para quando `morph_canon(output) ==
   morph_canon(input)` do passe (ponto-fixo — usa o `==` do P345). O `o_inf` converge
   para "Z" por este mecanismo (registrado como divergência). Confirmar M-trigger: o caso
   comum (não re-casa) não entra no loop.

### Estágio T — teto backstop + mensagem base = vanilla
1. Teste: recursão que **não converge** (ciclo / divergente) → **erro com a mensagem base
   byte-idêntica ao vanilla**; recursão profunda legítima → não erra. Confirmar que falham
   antes.
2. Código: o teto como **backstop** (número = mecânica; espelha o 64 do vanilla,
   declarado divergível). Não é o mecanismo de terminação — é o corte de runaway para o
   que o ponto-fixo não pega (ciclo, crescimento). Mensagem base = a do vanilla, **sem**
   acréscimo.

### Estágio Flag — a flag de erro completo (canal de hint separado)
1. Teste: com a flag ligada, o erro de recursão ganha um **hint separado** com a
   classificação correta — **cíclico** (a morfologia repete no caminho), **divergente**
   (cresce sem repetir), **converge-fundo** (passou do teto mas estabilizaria). Sem a
   flag, a mensagem base é byte-idêntica ao vanilla. Confirmar que falham antes.
2. Código: a flag; quando o teto dispara **e** a flag está ligada, a análise reconstrói o
   histórico de morfologias do caminho e classifica; emite no canal de hint. Sem a flag, e
   no caminho quente, **nada** disso corre.

### Estágio Ev — evolução de testes + divergência registrada
- O `m1` → "c": evoluir a asserção com a saída do vanilla colada.
- Os testes anti-recursão (`f3s2_show_callout_anti_recursao_termina`,
  `show_rule_nao_recursiva_sem_stack_overflow`): revalidar contra a nova terminação
  (devem continuar terminando — agora por ponto-fixo/teto, não por truncar no nível 1).
- **A divergência `o_inf`**: um teste **novo** que documenta `#show heading: it => [= Z]`
  → "Z" no cristalino, com a nota "vanilla 0.14.2: erro `maximum show rule depth
  exceeded`; divergência consciente (ADR-0107) — o vanilla erra por limitação reconhecida
  de mecanismo; o cristalino converge por morfologia". Não é falha — é o registro da
  escolha.
- Testes de classificação da flag (cíclico / divergente / converge-fundo).
- Os que o vanilla **confirma**: intactos.

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

---

## Verificação (gates) — morfológica, exceto a mensagem base

```
build: limpo a cada estágio (release buildável — perf).
suíte (RUST_MIN_STACK=33554432): C0 (2723) ± N. Asserções alteradas só as que a
  revisitação muda (cada uma justificada morfologicamente + vanilla). Reportar evoluídas
  e novas (incl. o teste da divergência o_inf e os da flag).
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (morfológica — ADR-0107):
  - convergente: a morfologia da árvore final == a do vanilla (m1 → "c"). NÃO bytes.
  - o_inf: converge para "Z" (divergência consciente registrada; vanilla erra).
  - ciclo e divergente: erro (como o vanilla — ambos erram); a flag classifica certo.
  - profundo legítimo: compila (ponto-fixo para antes do teto).
  - mensagem base de erro: BYTE-IDÊNTICA ao vanilla (aqui o byte importa — a mensagem é
    comportamento observável, ADR-0033). A flag adiciona só no hint separado.

caminho comum barato: o caso onde o output não re-casa NÃO paga morph_canon extra
  (M-trigger). Confirmar.

dois sistemas / == intacto: o morph_canon e o PartialEq do Rust (P345) não mudam de
  comportamento. Confirmar.

lente (--comparar antes/depois): a revisitação toca o anel show/realize — delta de
  aresta possível; content→elements 66; elem→elem 0. Se a contagem de ciclos mudar, é
  ACHADO a explicar, não esconder. Registrar.

perf (par back-to-back): o caminho quente (output não re-casa) ~nulo; a revisitação
  (quando dispara) adiciona passes + morph_canon — medir o custo num doc com recursão
  real; a análise da flag só no erro+flag. Regressão é achado, não se esconde.
```

---

## O que NÃO fazer
- **Não tocar o `==` morfológico (P345)** nem o `PartialEq` do Rust.
- **Não reproduzir a Revocation** (INTERNA, P347d) — α substitui o seu papel por
  convergência morfológica; não há revogação exposta ao autor (o vanilla também não expõe).
- **Não tocar text rules** — passe único, não recursam (M-text confirma).
- **Não des-assar** (F-5 disjunto, M4) — este lote não toca o `TextStyle`.
- **Não pôr a classificação na mensagem base** — vai no hint separado; a base é idêntica
  ao vanilla.
- **Não rodar a classificação no caminho quente** — só no erro + flag.
- **Não tratar o número do teto como paridade** — é backstop mecânico; a paridade é o
  comportamento (não-convergente → erro).
- **Não improvisar a camada multi-passe** se M-eager a exigir — é decisão do dono na trava.
- **Não medir aceitação pelo booleano/bytes** (exceto a mensagem base).
- **Não confiar em relatório** sobre o que recursa/toca — **medir da fonte**.
- **Não estimar com "~"**.

---

## Relatório (`typst-passo-348-relatorio.md`)
- Fase A: M-eager (eager-viável ou multi-passe + dimensão se multi-passe), M-trigger (o
  custo / o caminho comum barato), M-oráculo (a tabela com os alvos medidos), M-msg, M-text.
- Fase B: o diff por estágio; a revisitação até ponto-fixo; o teto backstop; a flag.
- Evolução de testes: cada um (vanilla colado); o `m1` → "c"; **a divergência `o_inf`
  registrada como teste documentado**; os da flag; os anti-recursão revalidados.
- Aceitação morfológica: a tabela de casos com o resultado (convergente=vanilla;
  o_inf=Z registrado; ciclo/divergente=erro; profundo=compila; mensagem base byte-idêntica).
- Verificação: suíte, lint, caminho-comum-barato, ==-intacto, lente, perf.
- **Mapa de filtro (dois campos):**
  1. **Lugar lógico desta fatia:** a recursão de `#show` do cristalino termina por
     **morfologia** (o `==` do P345), não por identidade de instância — é a ADR-0107
     levada à terminação: o cristalino escolhe o modelo que se explica (convergiu) em vez
     de reproduzir o mecanismo de identidade do vanilla (que o próprio vanilla trata como
     limitação). A divergência `o_inf` é a marca consciente dessa escolha.
  2. **Meta-achado da cadeia P347→P348 (entrada própria):** o método, ao investigar se
     tinha achado uma falha do vanilla, **recuou três vezes com evidência** — GEROU virou
     QUIS-com-borda (P347c, o commit #3327); "achamos uma falha" virou "diferença de
     prioridade" (P347c); "respeitar a Revocation" virou "Revocation é mecânica, substituir"
     (P347d). A capacidade de **concluir "não havia falha, eu estava enganado" e
     autocorrigir** é a evidência mais forte da branch de que o tekt mede antes de decidir.
     Registrar como entrada de fundação do mapa de filtro (candidata a abrir o documento
     de visão, junto com a entrada do `TextStyle`).
- Item: `content→elements` aponta para o Marco G (P346).
```
