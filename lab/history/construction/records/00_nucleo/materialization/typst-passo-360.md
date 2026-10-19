# Passo 360 — Balanço da fila F restante: medir para escolher o próximo lote

> **O que faz.** **Read-only.** A F-realização fechou (casos 1–4); o DEBT-60 ficou resolvido na
> parte com conserto limpo (b) e registrado na parte sem demanda (a). Sobram **F-5** (de-bake),
> **F-6** (3 folhas, DEBT-58), **Marco G** (desacoplamento dos nativos) e **DEBT-59** (flag na
> CLI). Este passo **mede**, para cada um, os dados que faltam para escolher o próximo com **custo
> + demanda + dependência MEDIDOS** — não supostos (a lição P351–P359: dimensionar de memória
> erra; medir primeiro acerta). **A pergunta-linchpin que reordena tudo**: alguma coisa na fila
> (**F-6**, **Marco G**) **precisa** do **chain-threading no introspect** que o F-5 ergueria?
> Se sim, o F-5 deixa de ser limpeza-sem-demanda (P353) e vira **fundação medida** de outra coisa;
> se não, fica por último. **Não decide o rumo** — produz a tabela e o grafo de dependência,
> recomenda, e **o dono escolhe**. **Zero código de produto, zero L0.** Saída:
> `00_nucleo/diagnosticos/f-recon-fila-f-passo-360.md`.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P360 (confirmar livre).
**Pré-condição**: P359 fechado (DEBT-60 (b) feito, (a) registrado; F-realização fechada; suíte
**2737**; lint **0/0**; lente **66**). HEAD pós-P359, árvore limpa. Caveat de stack:
`RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: **medição/balanço de fila** — read-only, zero código de produto, zero L0. Probes
descartáveis (compiladas e **revertidas**; árvore limpa; suíte não re-rodada). Termina numa
**decisão proposta ao dono** (qual dos candidatos é o próximo). Saída: o recon + a tabela + o
grafo.

---

## Leituras transversais da Fase A (a fonte vence; `file:line`)

1. **ADR-0107 e ADR-0108** — reler e aplicar a cada item. **Demanda e custo só valem medidos**;
   não pré-classificar nada como "raro", "necessário" ou "limpeza" sem a medição.
2. **A referência da linguagem** (o gate da lição P355): `lab/typst-original/docs/reference/`,
   `docs/tutorial/`, `docs/guides/`. Para cada item observável (F-6 toca estilo de Text/Math; o
   de-bake toca numbering/text-style), medir o que a **linguagem promove** vs o que é **interno**
   (o Marco G é estrutural, não-observável — paridade dele é com a lente, não com o doc).
3. **A lente** (`tekt-cargo-dsm`) — para as medições **estruturais** (Marco G: `edges(content →
   elements::*)`; F-5/F-6: acoplamento). **Registrar a versão/commit da lente** usada (precedente
   P333 Parte 5). Se a lente não buildar, registrar e prosseguir com grep.
4. **O plano e os relatórios** — `f-plano-lotes-passo-333.md` (estado atual), `typst-passo-353-
   relatorio.md` (a medição do F-5), `typst-passo-338-relatorio.md` (a condição C1, o DEBT-58).
   Confirmar o estado registrado de cada item **contra o repo** — não herdar.

---

## A pergunta-linchpin (medir primeiro — ela reordena o resto)

**O chain-threading no introspect (que o F-5 ergueria) tem demanda de F-6 ou Marco G?**

Medir, com `file:line`:
- **F-6 passa pelo introspect?** As 3 folhas (`Text`/`MathText`/`MathIdent`) recebendo estilo via
  chain — esse caminho é só **layout** (que já lê a chain por merge, `layout/mod.rs:609-626`), ou
  também **introspect** (que não tem `StyleChain` hoje, `introspect.rs` walk)? Texto não é
  locatável (P353 mediu N/A para o `TextStyle` no introspect) — confirmar se o mesmo vale para as
  3 folhas, ou se alguma delas é consultável e precisaria da chain no walk.
- **Marco G precisa do introspect ler a chain?** Migrar os 65 nativos pela fronteira E1 (dinâmica)
  — o introspect, que hoje lê os nativos por match monomórfico, passaria a lê-los pela fronteira?
  Isso exige o `StyleChain` no walk? Medir o ponto exato (`introspect.rs`, o arm `Content::Dynamic`
  vs os arms monomórficos).

**O que a resposta decide:**
- **Se F-6 ou Marco G precisa do chain-threading** → erguer essa infra tem **demanda medida**; ela
  vira o **núcleo** do próximo lote (seja como F-5-mínimo-fundação, seja dentro do F-6/Marco G), e
  o de-bake pega carona como prova. A ADR-0107 é satisfeita (a infra serve a demanda real).
- **Se nenhum precisa** → a infra fica **sem demanda**; o F-5 (de-bake) continua limpeza, e o
  honesto é **registrá-lo como débito de limpeza disponível** (como já está, adiado no P353) e
  **não erguer à frente** (ADR-0107).

---

## Por item — o que medir

### F-5 (de-bake)
- Re-confirmar o P353: o de-bake é **limpeza** (caminho duplo chain ≡ assado, paridade-testada);
  os 3 numbering exigem o chain-threading no introspect (o walk não tem chain); o `TextStyle`
  arrasta o bold do heading (`markup.rs:85-86`). Confirmar que isso ainda vale pós-P356/P358/P359.
- **Resultado**: F-5 = **fundação medida** (se a linchpin disser que F-6/G precisam da infra) **ou**
  **limpeza-sem-demanda** (registrar como débito disponível; não erguer à frente).

### F-6 (3 folhas, DEBT-58)
- **Largura real**, `file:line`: os sítios de `Text`/`MathText`/`MathIdent` que receberiam estilo
  via chain. O DEBT-58 cita "Text 4/7 · MathText 6/5 · MathIdent 2/5" — **confirmar no repo, não
  herdar**.
- **Caminho**: F-6 é só layout (já lê a chain) ou também introspect (precisa do chain-threading —
  a linchpin)?
- **C1 (tampão)**: se o F-6 rotear pela chain **mantendo** o campo assado (content-preserving), o
  caminho duplo nasce com **gatilho de remoção** + **teste de paridade entre os dois caminhos**
  (P338). Medir se o F-6 é tampão ou remoção direta.
- **Demanda**: é **correção** (as 3 folhas divergem do vanilla observavelmente?) ou **limpeza**
  (paridade-testada como o F-5)? Medir contra o vanilla — não supor.

### Marco G (desacoplamento dos nativos)
- **Estado atual** (lente): `edges(content → elements::*)` = 66; `edges(elemento → elemento)` = 0.
  A métrica do marco é `content→elements → 0`.
- **Escopo**, `file:line` + grep: a migração dos 65 nativos pela fronteira E1 (F-1, construída).
  Quantos sítios, a forma (cada `*Elem` passa a implementar a fronteira dinâmica?), e se o núcleo
  deixa de importar cada um.
- **Dependências**: F-1 (fronteira) feita; o plano diz Marco G **pós-F-6** — medir se isso é
  ordem real ou herança. Precisa do chain-threading no introspect (a linchpin)?
- **Custo**: estimativa pela largura + a lente. É o **maior** item — medir se cabe num lote ou
  exige **sub-spec própria** (o plano diz que sim: "spec própria quando chegar a vez").

### DEBT-59 (flag na CLI)
- **O que falta**: a capacidade interna da flag de erro completo está implementada (P350); só a
  **exposição na CLI** é débito. Medir a **superfície**, `file:line`: onde a CLI registra flags,
  quantas linhas, se é isolado e pequeno.
- **Demanda**: a flag exposta é útil (é diagnóstico)? Medir o uso esperado — não supor.

### DEBT-60 (a) — confirmar selado
- Confirmar que (a) está registrado na DEBT-60 como **divergência consciente medida** (P335
  deliberado; confinado não-idiomático; gatilho de reabertura) — **não** é um passo pendente.

---

## A tabela de saída (o que decide) + o grafo

O recon entrega uma **tabela** — para cada item: **largura/custo** (medido), **demanda** (medida:
correção / feature / limpeza / nula), **dependências** (precisa do introspect-chain? precisa de
outro item antes?), **observável** (paridade com o doc/vanilla, ou interno/lente). E o **grafo de
dependência** (quem precisa de quem — em especial, quem cria a demanda da infra de introspect-chain).

A tabela responde a pergunta central: **a infra de introspect-chain tem demanda?** — porque a
resposta reordena a fila inteira. Mais a **recomendação marcada** (qual o próximo, e por quê) e a
**decisão proposta ao dono** (a escolha é dele).

---

## O que NÃO fazer

- **Não escrever código de produto.** Probes descartáveis para medir são permitidas (compiladas e
  **revertidas**, árvore limpa); produção, não.
- **Não editar L0.**
- **Não decidir o rumo.** O recon mede, monta a tabela e recomenda; o dono escolhe o próximo.
- **Não pré-classificar demanda** ("raro", "necessário", "limpeza") sem a medição — é a regra que
  o arco P351–P359 quebrou e recuperou.
- **Não erguer a infra de introspect-chain nesta probe.** Medir a demanda é leitura; pagá-la é o
  lote seguinte, se o dono escolher.
- **Não tocar a F-realização, o α / caso 2, o `morph_canon`/`==`, a flag P350c nem o Marco G.**
- **Não importar a quarentena.** `lab/` é leitura de semântica/doc, nunca import.

---

## Saída/relatório (`f-recon-fila-f-passo-360.md` + resumo no chat)

A resposta da **linchpin** (F-6/Marco G precisam do introspect-chain?) com `file:line`; por item
(F-5, F-6, Marco G, DEBT-59) a largura/custo, a demanda e as dependências medidas; a confirmação
do DEBT-60 (a) selado; a **tabela** custo×demanda×dependência e o **grafo**; a versão/commit da
lente; a recomendação marcada e a decisão proposta ao dono (qual dos candidatos é o próximo).
Suíte não re-rodada (read-only); árvore limpa; lint inalterado; caveat de stack nas probes.

## Fora de escopo (confirmado)

A **implementação** de qualquer item (F-5, F-6, Marco G, DEBT-59) — nasce da escolha do dono, no
lote seguinte; o de-bake do F-5 (continua adiado se a linchpin disser "sem demanda"); A2/A3 do
caso 1 (declinados P358); qualquer toque no α / `morph_canon` / `==` ou na flag de diagnóstico.
