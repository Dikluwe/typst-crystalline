# Passo 350 — relatório: Fase A (canal de config) + TRAVA — a medição contradiz "L2"

> **Resultado: PAROU NA TRAVA (M-canal = o canal de config NÃO existe).** A medição,
> exigida pela ADR-0108 regra 1, **contradiz a hipótese "L2"** do P348/dono: **nenhuma
> config de usuário desce ao eval — em camada nenhuma**. O caminho de produção
> `main.rs (L4) → compile_to_pdf_bytes (L3) → eval (L1)` passa **só `world` + `source`**;
> não há `Options`/`Config`/flag em L2/L4, e o trait `World` não tem método de config.
> Logo a flag não é "um campo a mais num canal existente" — exige **abrir um caminho
> multi-camada**. Pela TRAVA do passo (e pela ADR-0108: a medição **produz** a decisão,
> não a assume), **paro** para o dono decidir. **Nenhum código de flag escrito.**
> Primeira aplicação prática da ADR-0108 — e ela já pegou uma suposição (a minha/do P348).

## C0 / C1
- **C0**: HEAD `8d70d9082` (pós-P349), pilha P347c/d+P348 commitada, **árvore limpa**,
  lint 0/0, suíte 2726/3245. Bate.
- **C1**: `§3a.7-bis` (`f_fronteira_e1.md:295-298`) registra a flag como **adiada** (P348).
  Este lote **mediria** o canal e a realizaria — mas a medição parou na trava (abaixo).

## Fase A — a medição que produz a decisão de camada (ADR-0108 regra 1)

### M-canal — **o canal de config L4→L1 NÃO existe** (contradiz "L2")
Rastreado da fonte, o caminho de produção inteiro:
- **L4**: `04_wiring/src/main.rs:86` — `compile_to_pdf_bytes(&world, &source)`. Só world +
  source; **nenhuma** option/flag.
- **L3**: `03_infra/src/pipeline.rs:38-57` — `eval_to_module_with_sink(world, source)` (e
  `compile_to_pdf_bytes`): **constrói `routines`/`traced`/`sink`/`route`/`registry`
  internamente** (todos default); não recebe config.
- **L1**: `eval()` (`rules/eval/mod.rs:186-197`) — params: `routines, world, traced, sink,
  route, source, registry`. **Nenhum** struct de options; `EvalContext::new()` é construído
  **dentro** do eval (`:211`), sem entrada externa.
- **`World` trait** (`contracts/world.rs:22-61`): `library/book/main/source/file/font/today`
  — **nenhum** método de config de usuário.
- Busca por `struct …Options/Config/Flags` em `02_shell`/`04_wiring`: **zero**.

**Veredito M-canal: canal NÃO existe.** Nenhuma config de usuário chega a L1 hoje, por
**nenhuma** camada. A hipótese "L2" (P348/dono — "L1 não lê env, então o campo mora em L2")
está **contradita**: não há nem L2 nem L3 nem L1 carregando config. Adicionar a flag exige
**abrir um caminho** (param em `eval` L1 + default em `pipeline` L3 + arg em `main` L4,
provavelmente um `CompileOptions`) — **superfície nova multi-camada**.

### M-classif — viável, mas o loop não guarda histórico hoje
O loop de revisitação (`rules/eval/rules.rs:110-205`) mantém só `work` (estado atual) e
`applied` (contador) — **não** guarda o histórico de morfologias. Para classificar
**cíclico** (uma morfologia do caminho **repete**, via `morph_canon`/P345) vs **divergente**
(cresce sem repetir) vs **converge-fundo** (estabilizaria, passou do teto), seria preciso
**manter um `Vec` de `morph_canon`** do caminho — **só quando a flag está ligada** (custo
O(profundidade) de clones, fora do caminho quente). **Reconstruível barato no ponto do
erro?** Não — teria de ser mantido durante o loop. Viável sob flag; não-trivial (o loop
ganharia um ramo condicional de histórico).

### M-msg — a mensagem base permanece intacta
O erro do teto é emitido **no próprio loop** (`rules.rs:196-201`, inline no P348, não via
`world_types.rs:273` — ambos byte-idênticos): `"maximum show rule depth exceeded"` +
`with_hint("maybe a show rule matches its own output")` +
`with_hint("maybe there are too deeply nested elements")`. Acrescentar um **3º hint** de
classificação sob a flag **não altera** a base nem os 2 hints. ✓ (base intacta confirmada).

## TRAVA — emitida; PAROU (M-canal = canal não existe)

Per a regra do passo ("M-canal = canal não existe → abrir o caminho é superfície nova
multi-camada → fronteira de decisão do dono; a flag é melhoria, não paridade") e a **ADR-0108
regra 1** (a medição produz a decisão; "L2" era hipótese): **paro**. O default já é
byte-idêntico ao vanilla **sem** a flag (M-msg) — então adiar a capacidade **não** degrada a
paridade.

**As opções (o dono decide):**

- **A — abrir o canal mínimo agora.** `eval()` (L1) ganha um param de config (um `bool
  full_error` ou um pequeno `EvalOptions`); `pipeline.rs` (L3) passa **default false**;
  `EvalContext` carrega-o; a classificação (M-classif) corre no erro+flag. **Testável** via
  o param (eval_for_test passa `true`). A **CLI** (parsing `--full-error` em `main.rs` →
  threading L4→L3) fica como **débito**. Custo: ~1 param em `eval` + 1 default em
  `pipeline` + campo em `EvalContext` + o ramo de histórico/classificação + testes.
  Bounded, mas **abre** o canal que hoje não existe (a decisão que a TRAVA reserva ao dono).
- **B — adiar a flag INTEIRA como débito** (não só a CLI). Como não há canal e a flag é
  **melhoria sobre paridade** (o default já é vanilla-idêntico), talvez abrir um caminho
  multi-camada por um detalhe de diagnóstico não valha **agora**. Registrar a flag toda
  (capacidade + CLI) como débito nomeado; revisitar quando houver um `CompileOptions` por
  outra razão (várias features futuras o pedirão junto).
- **C — capacidade em `EvalContext` + injeção só-de-teste.** Adicionar `full_error: bool` a
  `EvalContext` (default false) + a classificação; a **produção** nunca a liga (fica false,
  vanilla-idêntico); um **teste** a liga via um entry de eval que aceita o booleano. É o
  meio-termo: entrega a *capacidade testável* (o objetivo do passo) com o **mínimo** de
  superfície, e deixa **todo** o canal de produção (L4→L1) + CLI como débito. Mas mesmo isto
  **abre** um ponto de injeção no boundary do `eval` (EvalContext é construído lá dentro).

**Recomendação do agente (marcada como tal):** **A** se a flag é desejada já (é o caminho
que satisfaz "capacidade interna testável" com a CLI como débito, e o canal mínimo
eval-param+pipeline-default é bounded); **B** se o dono prefere não abrir canal por uma
melhoria de diagnóstico até haver um `CompileOptions` motivado por mais de uma feature. **C**
é a variante mais enxuta de A (sem tocar `pipeline` se o entry de teste for separado), mas a
diferença com A é pequena. A escolha é do dono — a ADR-0108 pediu a medição **justamente**
para esta decisão não ser assumida.

**Parei. Nenhum código de flag escrito. Probes não foram necessárias (medição é leitura de
fonte).**

## Verificação (gates até a trava)
```
content-preserving (Fase A): zero .rs/.toml. Árvore limpa. Suíte 2726/3245 não re-rodada.
lint: crystalline-lint . = 0/0.
medição: M-canal (file:line do caminho L4→L3→L1; canal não existe — contradiz "L2"),
  M-classif (histórico ausente no loop; viável sob flag, custo registrado), M-msg (base +
  2 hints intactos, rules.rs:196-201). Hipótese "L2" marcada como contradita pela medição.
  Zero "~".
fronteira: o passo NÃO escolheu a camada nem escreveu a flag — a medição parou na trava.
ADR-0108 aplicada: regra 1 (medir antes de decidir) pegou a suposição "L2"; a decisão de
  abrir-canal-ou-adiar é do dono.
```

## Mapa de filtro (campo)
**Lugar lógico:** a flag de erro completo é **melhoria sobre paridade** (não paridade): a
mensagem base permanece a do vanilla, e o cristalino acrescentaria, sob pedido, a
classificação que o vanilla não dá. **Primeira aplicação prática da ADR-0108** — e a
disciplina **já pegou uma suposição minha** (a camada "L2" do P348): a medição mostrou que
**não há canal de config em camada nenhuma**, então "onde mora o campo" não era "L2", era
"um canal a abrir" — decisão do dono, não default. **Rastro:** P348 desenhou e adiou a flag
(dizendo "L2" por raciocínio L1-não-lê-env); o dono pediu a capacidade interna (CLI=débito);
P350 **mediu** o canal, achou que ele **não existe**, e parou na trava — a ADR-0108
funcionando na sua estreia.

## Item carregado
`content→elements` aponta para o **Marco G** (P346) — não volta como órfão.
```
