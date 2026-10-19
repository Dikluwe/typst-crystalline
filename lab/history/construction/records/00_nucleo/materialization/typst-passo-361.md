# Passo 361 — Recon/sub-spec do Marco G: desacoplar os 65 nativos (content→elements→0)

> **O que faz.** **Read-only.** O P360 (balanço da fila) decidiu o Marco G como próximo,
> **começando pelo desenho** (recon/sub-spec), não pelo código. Este passo **mede** a superfície de
> dispatch da migração dos 65 nativos pela fronteira E1 e **desenha** o Marco G, para o dono decidir
> o **modelo** com custo + dependência + tensão de ADR **medidos** — não supostos (a lição
> P351–P359). **A pergunta-linchpin já foi respondida no P360** (nem F-6 nem Marco G precisam do
> chain-threading no introspect); aqui a **pergunta central** é o **MODELO**: α (`Content::Dynamic`
> vtable, como o plano nomeia) **ou** β (reificação PropMap, como a ADR-0105 F-destino) — sabendo que
> a **ADR-0026 rejeitou o vtable de propósito**. **Não decide o rumo nem escreve código.** Produz a
> tabela de dispatch + a tensão de modelo + o trava (ADR-0105 cl.3) + o slicing, recomenda, e **o
> dono escolhe**. **Zero código de produto, zero L0.** Saída:
> `00_nucleo/diagnosticos/f-recon-marco-g-passo-361.md`.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P361 (confirmar livre).
**Pré-condição**: P360 fechado (balanço da fila; decisão do dono: Marco G como próximo, via
recon/sub-spec; F-5/F-6 adiados; DEBT-59 disponível; DEBT-60(a) selado). HEAD pós-P360
(`3c213179a`), árvore limpa, suíte **2737**, lint **0/0**, lente **66/0**. Caveat de stack:
`RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: **recon/sub-spec de desenho** — read-only, zero código de produto, zero L0. Probes
descartáveis (compiladas/lidas e **revertidas**; suíte não re-rodada). Termina numa **decisão
foundacional proposta ao dono** (o MODELO do Marco G é um ADR, não um lote). Saída: o recon + a
tabela + a tensão de modelo + o grafo de slicing.

---

## Leituras da Fase A (a fonte vence; `file:line`)

1. **ADR-0026** (Content enum vs vtable) e **ADR-0105** (modelo D agora / F destino; cláusula 3 —
   a trava de exaustividade). **A tensão entre elas e o plano é o coração deste recon** — reler à
   letra; se o plano contradisser as ADRs, **medir e registrar**, não escolher de memória.
2. **ADR-0107 e ADR-0108** — não erguer infra grande sem demanda/modelo medidos; afirmar só o medido.
3. **O plano** — `f-plano-lotes-passo-333.md:119-141`: a definição do Marco G, a métrica
   (`content→elements→0`), e o **status** ("FORA da fila F", "pós-F-6", "spec própria", "não é
   trabalho desta branch"). Confirmar contra o repo, não herdar.
4. **A lente** (`tekt-cargo-dsm`) — registrar versão/commit; medir `edges(content → elements::*)` e
   `edges(elemento → elemento)` (o baseline do marco).
5. **A fronteira E1** (`entities/elements/dynamic.rs`, F-1) e o registro
   (`entities/element_registry.rs`) — o que existe (trait `DynElement`, blanket, `as_any`,
   `dyn_kind`, registro nome→construtor) e o que falta (tabela kind→handler).

---

## A pergunta central (medir, não decidir): qual o MODELO?

`content→elements→0` é alcançável por **dois modelos opostos**:
- **α — `Content::Dynamic(Arc<dyn DynElement>)`** (o plano `:132`): migrar os 65 nativos para o
  `dyn`. **É um vtable** — o que a **ADR-0026 rejeitou** ("enum linear, sem vtable").
- **β — reificação PropMap** (ADR-0105 `:101-106`): "o enum fechado **permanece**; a lógica muda de
  morada → descritores + PropMap". **Sem vtable.**

Medir, com `file:line`: a superfície de dispatch (quantos arms em content.rs/layout/introspect/
export), **onde o acoplamento vai parar** em cada modelo (α: tabela kind→handler com downcast; β:
nó genérico + tabela const), e se a métrica `content→elements→0` **elimina** ou **reloca** o
acoplamento. **A escolha α/β é do dono — é um ADR.**

---

## Por item — o que medir

### Superfície de dispatch (`file:line` + contagens)
- **content.rs** hub (`plain_text`, `is_empty`, `map_content`, `map_text`, `get_field`, `eq`): #
  arms nativos; quais colapsam num arm `Content::Dynamic(e) => e.dyn_*()` (trait-delegáveis) e quais
  precisam de lógica por-variante.
- **layout/mod.rs** `layout_content`: # arms nativos; quantos lêem campos concretos (→ downcast/
  handler-table) vs delegam. **É o crux** — o arm `Content::Dynamic` atual é trivial?
- **introspect.rs** (`walk`, `extract_payload`, `populate_intr`): # arms; o que a trait
  (`to_payload`/`dyn_to_payload`) já cobre; o arm `Content::Dynamic` no walk (placeholder?).
- **export/PDF**: há dispatch por variante nativa, ou opera sobre `FrameItem` (desacoplado)?
- **Total** workspace: ordem de grandeza dos arms tocados.

### A Trava (ADR-0105 cláusula 3) — obrigatória antes de código G
Confirmar o mecanismo a repor: teste que varre tabela const × backends (cada kind → handler de
layout/introspect/show) **ou** regra do `crystalline-lint`. Erro-de-compilação **não** vira
erro-de-runtime silencioso.

### Slicing + status
- Cabe num lote ou é multi-lote? (O plano diz "spec própria".) Propor o fatiamento.
- Confirmar o status do plano (fora da branch / pós-F-6 / F-6 ainda não feito).

---

## A saída (o que decide)

Uma **tabela de dispatch** (consumidor × # arms × delegável-pela-trait × crux), a **tensão de modelo**
(α vs β, com as ADRs em `file:line`), o **custo do crux** (a tabela kind→handler de layout + o trava),
o **slicing** proposto, e o **status do plano**. Mais a **recomendação marcada** (o próximo passo é o
ADR de modelo, não código) e a **decisão proposta ao dono**: escolher o modelo (α/β) ou adiar (o plano
põe o Marco G fora da branch).

---

## O que NÃO fazer

- **Não escrever código de produto.** Probes descartáveis para medir são permitidas (revertidas);
  produção, não.
- **Não editar L0** nem escrever o ADR de modelo aqui — o ADR nasce da escolha do dono.
- **Não decidir o modelo** (α/β) nem a ordem — o recon mede e recomenda; o dono escolhe.
- **Não erguer a tabela kind→handler nem migrar nativo nenhum** nesta probe.
- **Não tocar a F-realização (fechada), o α / caso 2, o `morph_canon`/`==` nem a flag P350c.**
- **Não importar a quarentena.** `lab/` é leitura de semântica/doc, nunca import.

---

## Relatório (`typst-passo-361-relatorio.md` + resumo no chat)

A superfície de dispatch medida (tabela + `file:line`); a tensão de modelo α/β com as ADRs; o crux
do layout (o acoplamento que reloca); a Trava cl.3; o slicing e o status do plano; a versão da lente;
a recomendação marcada e a decisão proposta ao dono (modelo α/β ou adiar). Suíte não re-rodada
(read-only); árvore limpa; lint inalterado; caveat de stack.

## Fora de escopo (confirmado)

A **execução** do Marco G (multi-lote, após o ADR de modelo); o **ADR de modelo** em si (nasce da
escolha do dono); F-5/F-6 (adiados P360); DEBT-59 (disponível in-branch); qualquer toque no α /
`morph_canon` / `==` / flag ou na F-realização fechada.
