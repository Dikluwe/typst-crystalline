# Passo 350b — relatório: a forma de C, medida — **C-COM-ORIGEM** (com uma nuance L3)

> **Forma decidida (medida, não raciocinada): C-com-origem.** V1 = **sim** (o `EvalContext`
> é o ponto de **leitura** certo: portador único, criado uma vez, disponível no ponto do
> erro). V2 = **existe** uma origem natural: **`RunIntent`** (`02_shell/src/cli.rs:103`), o
> struct de config que L4 consome, já com `colored: bool` (precedente de flag diagnóstica) —
> `full_error` mora ali. **Nuance medida:** a API L3 (`compile_to_pdf_bytes`/
> `eval_to_module_with_sink`) **não** tem struct de opções (é `world + source`), então a
> origem existe em L2 mas o **canal L4→L3→L1 ainda precisa ser aberto** — é o que C-com-origem
> faz, deixando **só o parsing da CLI** (`Args`→`RunIntent`) como débito. **Não implementei.**
> Segunda aplicação prática da ADR-0108 — agora a uma recomendação **minha** (eu sugeri C por
> raciocínio; a medição refinou-o para C-com-origem). Read-only, árvore limpa, lint 0/0.

## Pré-condição
P350 parado na trava (M-canal: sem canal L4→L1). HEAD `8d70d9082`, árvore limpa (o relatório
P350 é untracked), lint 0/0, suíte 2726/3245. Read-only no `tekt`.

## V1 — o `EvalContext` é o ponto de LEITURA certo? **SIM**
1. **Disponível no ponto do erro?** O erro do teto é emitido em `apply_all`, dentro de
   `apply_show_rules` (`rules/eval/rules.rs:196-201`), cuja assinatura recebe
   `ctx: &mut EvalContext` (`rules.rs:72`); a closure `apply_all` captura `ctx`. → **sim**,
   o `EvalContext` está disponível exatamente onde a classificação leria a flag.
2. **Portador único (não fragmentado)?** `EvalContext::new()` é chamado **uma vez**
   (`rules/eval/mod.rs:211`) e propagado **por `&mut`** em todas as assinaturas de eval
   (`ctx: &mut EvalContext`, `:279/:411/:602`). As reconstruções de `Engine` por escopo
   (`local_engine`, `:444/:506`) **reusam o mesmo `ctx`** (`eval_expr(expr, scopes, ctx,
   &mut local_engine)`, `:454`) — o `ctx` **nunca** é clonado/recriado por escopo. → **portador
   único**: um campo nele chega consistente ao ponto do erro.

**V1 = SIM.** C é sólido no ponto de **leitura**: um campo `full_error` no `EvalContext` é
lido no ponto do erro, consistente.

## V2 — existe um struct de opções que seja a ORIGEM? **EXISTE (`RunIntent`), com nuance L3**
"Não há canal **que desce** a L1" (M-canal/P350) **não** é "não existe struct de opções".
Busca em L2–L4:
- **`RunIntent`** (`02_shell/src/cli.rs:103-109`): *"L2 traduz argumentos + env vars + isatty
  para este struct. L4 consome directamente sem conhecer clap ou env vars."* Campos: `input,
  output, root, font_paths, **colored: bool**`. → é a **origem natural** de um flag
  diagnóstico (o `colored` é o precedente exato — um bool de modo, não de conteúdo). O
  `full_error` mora aqui, ao lado do `colored`.
- **`Args`** (`cli.rs:61`, clap `Parser`): a fonte crua dos argumentos; `RunIntent` é a sua
  tradução limpa.
- **API L3** (`03_infra/src/pipeline.rs`): `compile_to_pdf_bytes(&world, &source)` /
  `eval_to_module_with_sink(world, source)` — **não** têm struct de opções (só world+source).

**V2 = EXISTE em L2 (`RunIntent`), mas o canal L4→L3→L1 não está aberto.** A origem tem
lugar certo (`RunIntent`); falta o **fio** dela até o `EvalContext`, cruzando a API L3 que
hoje não carrega config.

## V3 — o histórico de morfologias está disponível/barato sob flag?
O loop (`rules.rs:110-205`) mantém só `work` (estado atual) e `applied` (contador) — **não**
guarda histórico. Para classificar (cíclico = uma `morph_canon` do caminho **repete**;
divergente = cresce sem repetir; converge-fundo = estabilizaria mas passou do teto), é
preciso **manter um `Vec<Content>` (ou de `morph_canon`) do caminho** — **só quando a flag
está ligada** (custo O(profundidade) de clones, fora do caminho quente; reconstruir no ponto
do erro **não** é possível sem o histórico). **Viável sob flag.** *(Inferência marcada: a
distinção converge-fundo vs divergente é heurística — ambos "não repetem"; converge-fundo só
se distingue se o crescimento desacelera. A classificação fina é detalhe do P350c.)*

## TRAVA — a forma de C decidida (parar; não implementar)

- **V1**: sim — `EvalContext` é o ponto de leitura (portador único, disponível no erro).
- **V2**: existe (`RunIntent`, `cli.rs:103`) — origem natural; nuance: a API L3 não tem
  struct de opções, o canal L4→L3→L1 precisa abrir.
- **V3**: histórico viável sob flag (mantido no loop só quando ligada).

**Forma decidida: C-COM-ORIGEM** (regra do passo: V1 sim + V2 existe). Concretamente:
- **Origem (escrita):** `full_error: bool` em `RunIntent` (`cli.rs:103`), ao lado de
  `colored`. `cli::parse()` o define (default `false` até a CLI ligar o `--full-error`).
- **Canal (a abrir agora):** `RunIntent.full_error` → `main.rs` (L4) → `compile_to_pdf_bytes`/
  `eval_to_module_with_sink` (L3, **novo param** ou um `CompileOptions`) → `eval` (L1, novo
  param) → `EvalContext.full_error`.
- **Leitura:** `EvalContext.full_error` no ponto do erro (`apply_all`).
- **Classificação:** histórico sob flag → 3º hint (cíclico/divergente/converge-fundo); base +
  2 hints intactos.
- **Teste:** preenche via o param de `eval` (ou `RunIntent`); flag-on vê o hint, flag-off
  byte-idêntico ao vanilla.
- **Débito (só isto):** o **parsing da CLI** — o `Arg` `--full-error` em `Args` (`cli.rs:61`)
  e o seu mapeamento para `RunIntent.full_error`. O resto do fio fica construído.

**Nuance honesta (ADR-0108 — não esconder):** C-com-origem **abre** o canal L4→L3→L1 agora,
incluindo um **param novo na API pública L3** (`compile_to_pdf_bytes`), por um flag
**off-by-default sem consumidor de produção ainda** (a CLI é débito). Isso é construir um
fio à frente da demanda da CLI. A alternativa **C-enxuto** (campo só no `EvalContext` +
injeção de teste; `RunIntent` + canal + CLI **todos** débito) **não** toca a API L3 e é
mínima agora, ao custo de mais refactor futuro quando a CLI chegar. A regra do passo aponta
C-com-origem (V2 existe); a ADR-0108 (não abrir canal especulativo) tempera: a parte
**especulativa** aqui é o param na API L3 (sem consumidor). **Recomendação marcada:**
**C-com-origem até a borda L2/L4 (`RunIntent` + threading L4)**, mas **confirmar com o dono**
se a API **L3** ganha o param agora (C-com-origem pleno) ou se o `EvalContext` é alimentado
por um caminho que **não** muda a assinatura pública de L3 (ex.: o param de `eval` recebe um
default em `eval_to_module_with_sink` sem expô-lo na assinatura de `compile_to_pdf_bytes`) —
uma variante intermédia. **Parei para o dono confirmar a forma exata** antes do P350c.

### Forma CONFIRMADA pelo dono: **C-com-origem intermédio** (sem mexer na assinatura pública L3)
O dono escolheu a variante intermédia. A forma exata para o P350c implementar:
- **Origem:** `full_error: bool` em `RunIntent` (`cli.rs:103`), ao lado de `colored`
  (default `false`).
- **Canal:** `eval()` (L1) ganha o param `full_error`; alimentado por um **default no
  caminho INTERNO de L3** (`eval_to_module_with_sink`/`eval_to_module`) **sem** expor o param
  na assinatura **pública** de `compile_to_pdf_bytes` (a API L3 pública **não muda**).
- **Leitura:** `EvalContext.full_error` no ponto do erro (`apply_all`).
- **Teste:** preenche via o param de `eval` (eval_for_test passa `true`).
- **Débito nomeado:** (1) o parsing da CLI (`--full-error` em `Args` → `RunIntent`); (2) o
  fio `RunIntent`→o ponto interno de L3 que passa o booleano a `eval` (hoje o default `false`
  entra no caminho interno; ligar `RunIntent` a ele é débito junto com a CLI).

## O que fica como débito
- **Sempre:** o parsing da CLI (`--full-error` em `Args` → `RunIntent.full_error`).
- **Se C-enxuto (alternativa):** também o campo em `RunIntent`, o canal L4→L3→L1, e a CLI.

## Verificação (gates)
```
content-preserving: zero código/teste/.rs/.toml. Árvore intacta (git status limpo de
  produto). Suíte 2726/3245 não re-rodada. Read-only (grep/leitura).
lint: crystalline-lint . = 0/0.
evidência: V1 (rules.rs:72/196-201 erro + ctx; mod.rs:211 criação única + :454 reuso),
  V2 (cli.rs:103 RunIntent + colored; pipeline.rs sem options), V3 (loop sem histórico,
  rules.rs:110-205; viável sob flag). Inferência da classificação fina marcada. Zero "~".
distinção ler≠escrever: leitura = EvalContext (V1, agora); origem/escrita = RunIntent
  (V2, L2) + canal a abrir; CLI parsing = débito. Separados explicitamente.
fronteira: não implementou; decidiu a forma (C-com-origem, com a nuance L3 para o dono
  confirmar) e parou.
```

## Mapa de filtro (campo)
**Lugar lógico:** a escolha de C foi **medida, não raciocinada** — separou-se o ponto de
**leitura** (verificável agora: o `EvalContext`, portador único) da **origem** (`RunIntent`
existe em L2; o canal a ela ainda não desce a L1). **Segunda aplicação prática da ADR-0108**,
agora a uma recomendação **minha** (eu sugeri C por raciocínio "o EvalContext é o lugar
certo"; a medição **confirmou** a leitura e **refinou** a origem para `RunIntent`, expondo a
nuance da API L3 que o raciocínio tinha pulado). **Rastro:** P350 mediu que não há canal
L4→L1; P350b mede o ponto de leitura (sim) e a origem (`RunIntent`, com a API L3 sem options);
P350c implementa a forma confirmada.

## Item carregado
`content→elements` aponta para o **Marco G** (P346) — não volta como órfão.
```
