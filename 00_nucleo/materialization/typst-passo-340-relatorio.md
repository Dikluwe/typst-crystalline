# Passo 340 — relatório (modo autônomo): F-realização fatia 2

> **Resultado:** fatia 2 **PARCIAL**. Aterrado o **caso 4 / `f3s3`** (confinamento
> de escopo do `#show`) + decisões medidas. **Estacionado** o multi-passe/fixpoint
> (casos 1/2 e os "2 passes" do caso 5) — por content-preservation, regra 6. Tudo
> resolvido por **medição** (sem paradas de checkpoint), conforme o modo autônomo.

Caveat de stack: `RUST_MIN_STACK=33554432` em todas as corridas de teste.

---

## A. Decisões autônomas com medição

1. **Confinar o `#show` no `ContentBlock` (caso 4).** Medição: o `[]` partilhava
   `&mut *engine.show_rules` (`eval/mod.rs`), o `{}` (`CodeBlock`) clonava
   `local_show_rules` → assimetria = o vazamento. Decisão: clonar também no `[]`
   (Arc O(1)). **Evidência de segurança:** rodando a suíte com a mudança, falhou
   **exatamente 1 teste** (`f3s3` — único `#[#show]` do corpus); 3237 verdes. Logo
   o confinamento é content-preserving em todo o resto. Paridade vanilla: confina
   via `StyledElem` (`content/mod.rs:744-752`).

2. **`f3s3` vira (a exceção autorizada).** De "vaza 2×" a "confina 1×": só o callout
   dentro do `[]` é transformado; o de fora sobrevive (asserção `has_dynamic`).
   Renomeado `f3s3_caso4_escopo_show_confina_no_content_block`. Razão escrita no teste.

3. **Trava-Q1 (representação do guard) — decidida pela fonte.** `Content::Guarded`
   transparente (não meta no `Dynamic`). Medição: vanilla usa `meta().lifecycle`
   no elemento (`content/mod.rs:148-156`), mas o `Arc<dyn DynElement>` do E1 é
   imutável/partilhado (clone O(1)); o wrapper transparente mantém os 65 nativos
   sem meta, o `dyn` limpo, e é uniforme nativo+dyn (S6). (Materializa no lote do
   multi-passe — §C.)

4. **Terminação do eager — já existe (medido).** `active_guards` (RuleId) +
   teto-64 (`check_show_depth`, `world_types.rs:249`; paridade `lib.rs:401-402`).
   O guard é tão eficaz que o teto-64 é backstop — o cenário que o dispara é, na
   prática, do multi-passe (§C).

## B. DECISÕES PROVISÓRIAS — revisar

- **B1 — o multi-passe é lote próprio, não retrofit.** (Commit do L0:
  `03619dc93`.) A medição decidiu **não** retrofitar o fixpoint sobre o eager
  (quebraria 18 testes — §C/§D); a decisão de o tratar como **lote dedicado
  futuro** (onde a paridade se mede contra um conjunto de testes deliberadamente
  evoluído) é conservadora e reversível, mas é uma **escolha de planeamento** que
  o dono deve confirmar. Alternativa: evoluir os 18 testes deliberadamente dentro
  de um P340b com a paridade vanilla compilada como referência.

## C. Sub-partes estacionadas

- **Multi-passe / loop até fixpoint (casos 1 e 2).** **NÃO feito.** É o coração da
  fatia 2 no molde, mas colide com content-preservation (§D). O desenho **medido**
  do vanilla está registado no L0 §3a.7-bis para o lote que o materializar:
  `realize→visit→visit_show_rules` (`lib.rs:43/224/335`), fixpoint **externo** na
  introspection loop (`:380`), `recipe-index` innermost-first (`:472`), teto via
  `route.increase()`+`check_show_depth()` (`:401-402`), escopo via `visit_styled`
  (`:574`). O `Content::Guarded` (Trava-Q1) nasce aqui.
- **Caso 5 "2 passes" (nativo+dyn multi-passe).** A **uniformidade da chain** veio
  na fatia 1; o aspecto multi-passe depende do fixpoint → estacionado com ele.
- **Teste de terminação que dispara o teto-64.** É concern do multi-passe (no eager
  o guard intercepta antes); estacionado com o loop.

**O que falta para fazer (lote do multi-passe):** materializar o loop externo +
`Content::Guarded` + recipe-index; **e** evoluir conscientemente os 18 testes
`#show` eager para a semântica multi-passe (paridade vanilla compilada como
referência, gatilho de 2º nível). Não é retrofitável sob no-change estrito.

## D. Achados

- **Achado central — content-preservation bloqueia o retrofit do multi-passe.** O
  cristalino é **eager por criação** (`intercept_content` intercepta na criação;
  compõe por cascata). **18 testes** asseveram essa semântica (ex.:
  `show_rule_composicao_sem_loop` → `matches("Prefixo:").count()==1` / "não
  reaplicada"; `show_rule_encadeamento_duas_regras`; os `show_rule_*` de
  composição/encadeamento). Um loop externo até fixpoint mudaria a saída deles. P340
  declara content-preserving **inviolável** (exceto `f3s3`). → regra 6: aterrar o
  seguro, estacionar o multi-passe. **Nenhuma asserção existente foi alterada além
  do `f3s3`.**
- **Perf:** sem regressão. A única mudança de caminho quente é **um `Arc::clone`
  O(1)** por `ContentBlock` (refcount). Docs sem `#show` em `[]`: caminho idêntico.
  Como o multi-passe (que adicionaria passes) foi estacionado, não há salto
  eager→multi-passe nesta corrida.
- **Lente:** **sem mudança de contagem de ciclos** — idêntica ao P339:
  219 módulos | 676 arestas | ciclos [90,4] | content→elements 66 | elem→elem 0.
  O confinamento é um `Arc::clone` local, zero arestas novas.

## E. Trava-Q1

**Escolha:** `Content::Guarded` transparente (invólucro de qualquer `Content`),
**não** campo meta no `Dynamic`. **Razão** (§A.3, medida do vanilla). **Prova de
transparência:** é requisito de teste **quando o invólucro nascer** (no lote do
multi-passe) — `plain_text`/`is_empty`/`map_*`/closures não veem o guard; nativo e
dyn tratados igual. Não nasceu nesta corrida (o multi-passe está estacionado), logo
a prova fica com ele. Registrado no L0 §3a.7-bis.

## F. Terminação

- **Teto:** **64** (`Route::MAX_SHOW_RULE_DEPTH`, `world_types.rs:249`; paridade
  vanilla `typst-realize/src/lib.rs:401-402`).
- **Testes vivos:** `show_rule_nao_recursiva_sem_stack_overflow`,
  `f3s2_show_callout_anti_recursao_termina` (guard por RuleId). Nenhuma corrida da
  suíte travou nem estourou a stack (3238 verdes).
- **Teto-64 disparado:** não exercitado nesta corrida (concern do multi-passe; o
  guard intercepta antes no eager). Estacionado com o loop.

## G. Estado da fila

- **F-4** ✅ (P338) · **F-realização fatia 1** ✅ (P339) · **fatia 2** ⚠️ **PARCIAL**
  (P340): caso 4 aterrado; multi-passe (fatia 2b) estacionado.
- Restam: **fatia 2b** (multi-passe/fixpoint + `Content::Guarded` — lote próprio),
  **fatia 3 / P341** (caso 3 show-set, `Transformation::Style`), **F-5** (de-bake),
  **F-6** (folhas).
- **5 casos do spike-2 (Estágio P):** caso 4 ✅ aterrado · casos 1/2 ⏸ estacionados
  (fatia 2b) · caso 5 ⏸ (uniformidade na fatia 1; "2 passes" com a 2b) · **caso 3
  → fatia 3 (P341)**, ponteiro mantido.

## H. Item aberto carregado — `content→elemento → 0`

Continua **fora da fila** F-1…F-6 + F-realização e **sem dono**. O baseline da
lente mede `content→elements = 66` e espera `target=0`, que **nenhum lote** entrega.
As três saídas (para decisão, não bloqueio):
1. **Reconciliar o baseline** — aceitar que o modelo D (enum fechado) tem
   `content→elements ≠ 0` por desenho, e ajustar a expectativa da lente.
2. **Nomear um marco pós-F-6** que faça o corte (núcleo deixa de importar `*Elem`).
3. **Registrar como lacuna** explícita do plano (a fila não inclui o corte).

## I. Verificação

```
build: limpo a cada estágio.
suíte (RUST_MIN_STACK=33554432): C0 2719 → 2719 (typst-core) / 3238 (workspace).
  N = 0 testes novos; 1 teste virado (f3s3, decisão registrada); 0 outras
  asserções alteradas. Zero regressão.
lint: crystalline-lint . = 0 violations, 0 warnings.
terminação: suíte verde, nenhuma corrida travou/estourou stack.
lente (so-referência): 219 | 676 | [90,4] | content→elem 66 | elem→elem 0
  — idêntica ao P339 (confinamento = Arc::clone local, ~nulo).
perf: sem regressão (1 Arc::clone O(1)/ContentBlock; multi-passe estacionado).
```

`git log --oneline` (commits isoláveis, reversíveis um a um):

```
03619dc93  Passo 340 — L0 §3a.7 realização (materialização parcial + decisões)
5cccf06bd  Passo 340 — caso 4 / f3s3 confinado
f859d8c9f  Passo 340 — caronas
```

(Estágios G/F do molde não geraram commit de código próprio: o guard+terminação
do eager já existem; o fixpoint/composição foi estacionado — §C.)
