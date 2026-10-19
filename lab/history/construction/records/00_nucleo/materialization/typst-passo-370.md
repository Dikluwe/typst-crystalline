# Passo 370 — F-5b: por que a ADR-0026 rejeita o vtable, e a solução que satisfaz a razão dela

> **O que faz.** **Read-only.** O F-5b está bloqueado por um conflito de ADRs: a **0107** exige a
> distinção semântica `strong ≠ emph ≠ styled` do vanilla (a paridade é com a linguagem), e a
> **0026** rejeita o caminho que o vanilla usa para tê-la (elementos distintos via **vtable**). A
> pergunta central deste passo **não** é "qual ADR vence" — é **por que a 0026 rejeita o vtable**:
> entender a **razão** dela na fonte, porque a solução boa tem de **satisfazer essa razão**, não
> contorná-la. Se a razão da 0026 for "sem vtable como mecanismo de dispatch dinâmico", talvez a
> distinção semântica de tipo (um **discriminante de enum**) a satisfaça — discriminante **não é**
> vtable. Se a razão for outra, a solução é outra. **O passo mede a razão, depois desenha a solução
> que a respeita, e termina numa decisão de modelo proposta ao dono.** **Não escreve código, não
> decide o conflito, não toca** o α/caso 2, o `morph_canon`/`==`, o caso 4, a flag, o Marco G. Saída:
> `00_nucleo/diagnosticos/f-recon-f5b-adr-0026-0107-passo-370.md`.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P370 (confirmar livre).
**Pré-condição**: F completo pelos princípios exceto o F-5b (P369 Veredito B). O F-5b registrado
como DEBT-61 (lote arquitetural: modelo strong/emph/styled + α-fixpoint). A medição do P366: o
de-bake do `TextStyle` toca o α (o `morph_canon` serve `rules.rs:229`) e o modelo (o P101 colapsou
strong/emph em `Content::Styled`). Suíte **2742**, lint **0/0**, lente **66/0**, árvore limpa. HEAD
pós-P369. Caveat de stack: `RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: **recon/resolução de conflito de ADR** — read-only (leitura das ADRs, do P101, do vanilla
em `lab/`; probes revertidas). Termina numa **decisão de modelo/ADR proposta ao dono**. Não escreve
código de produto, não decide o conflito.

---

## A pergunta central (o que o dono pediu): POR QUE a 0026 rejeita o vtable

A solução tem de **satisfazer a razão** da 0026, não contorná-la. Então a Fase A começa por
**extrair a razão**, com `file:line`:

1. **Ler a ADR-0026 inteira** e extrair, textualmente, **por que** ela rejeita o vtable. Os indícios
   já vistos: "replicar o original exigiria (1) `typst_macros` (proc macros) como dependência de L1,
   (2) código `unsafe` em L1 (violação do domínio puro), (3) toda a cadeia `NativeElement → Styles →
   StyleChain` antes de ter texto básico". E: "enum linear declarativo... pode crescer linearmente
   **sem vtable**, mantendo a arquitetura clara". **Decompor a razão** em cláusulas verificáveis:
   - é "sem **dispatch dinâmico por vtable**" (a mecânica)?
   - é "sem **proc-macro/`unsafe`** em L1" (a dependência/pureza)?
   - é "**exaustividade do compilador**" (o `match` que o enum dá e o `dyn` tira — o que a ADR-0105
     cl.3 depois protege)?
   - é "**clareza/declaratividade** de L1" (legibilidade)?
   Provavelmente é mais de uma. **Marcar cada cláusula [medido] com a citação.** Esta decomposição é
   o coração do passo: a solução do F-5b é avaliada contra **cada** cláusula.

2. **Ler a ADR-0107** e extrair o que ela exige no caso strong/emph/styled, com `file:line`: a
   distinção semântica é com a **linguagem**; o vanilla é o oráculo. Confirmar (P366 já mediu) que o
   vanilla trata `StrongElem`/`EmphElem` como **tipos distintos** (`Packed::eq` compara id), logo
   `*bold* ≠ _italic_` e `#set text X ≠ X` — semântica observável.

3. **Ler o P101 na fonte** (o colapso) e o critério da época: ele foi decidido por **consolidação +
   ADR-0026** (paridade **funcional**, não de implementação). Confirmar que, sob a 0026, o colapso
   era **válido**; sob a 0107 (posterior), virou divergência semântica. Datar as duas (a 0026 é
   anterior; a 0107 posterior) com `file:line`.

---

## A análise: a distinção semântica satisfaz a razão da 0026?

Com a razão da 0026 decomposta em cláusulas, medir se há uma forma de dar a **distinção semântica**
que a 0107 exige **sem violar nenhuma cláusula**:

- **Candidato: discriminante de tipo no modelo** (ex.: `Content::Styled` ganha um `kind:
  StyleKind { Strong, Emph, Text, … }`, ou strong/emph voltam a ser variantes do enum). Avaliar
  contra **cada** cláusula da 0026:
  - dispatch dinâmico por vtable? **Não** — um discriminante de enum é estático, o `match` continua.
  - proc-macro/`unsafe`? **Não**.
  - exaustividade do compilador? **Preserva** (o `match` sobre o discriminante é exaustivo) — talvez
    até **melhore** vs o estado atual (distinguir pelo campo de estilo).
  - clareza? medir.
  Se o candidato satisfaz todas as cláusulas → **não há conflito real**: a 0026 nunca proibiu a
  distinção semântica, só o vtable como mecânica; a distinção por discriminante é compatível, e a
  0026 talvez nem precise de emenda (ou só de uma nota).
- **Se algum candidato viola uma cláusula** → o conflito é real nessa cláusula; aí sim é uma
  **emenda de ADR** (qual cede), decisão do dono.

4. **O impacto no α-fixpoint** (o limite duro do P366): para **cada** candidato, medir se distinguir
   strong/emph/styled no modelo muda o que o `morph_canon` entrega ao α (`rules.rs:229`, o caso 2).
   Existe uma forma de dar a distinção ao **`==` da linguagem** (fiel ao vanilla) **sem** mudar o
   canon morfológico que o α consome? Se os dois consumidores do `morph_canon` puderem ser separados
   → o F-5b não reabre o caso 2. Se não → o custo é maior. **Medir, não assumir** (Trava 1/6/8).

---

## A saída — a decisão de modelo proposta ao dono

O recon entrega:
- **A razão da 0026 decomposta** em cláusulas, com citação (`file:line`).
- **O que a 0107 exige** no caso strong/emph/styled (`file:line`, oráculo vanilla).
- **A tabela candidato × cláusula da 0026 × impacto no α**: para cada candidato de modelo
  (discriminante, variantes próprias, separar os consumidores do `morph_canon`), se satisfaz cada
  cláusula da 0026 e se toca o α.
- **O veredito sobre o conflito**: é **aparente** (a distinção por discriminante satisfaz a 0026 →
  sem emenda, ou nota) ou **real** (algum candidato viola uma cláusula → emenda de ADR, qual cede).
- **A recomendação marcada** e a **decisão proposta ao dono**: (i) o modelo que dá a fidelidade à
  0107 satisfazendo a 0026 (se existe) + a spec multi-lote; ou (ii) se todo candidato fiel viola a
  0026 ou reabre o caso 2, registrar o F-5b como **divergência consciente medida** (DEBT-61, como o
  DEBT-60 (a)) com o porquê. **A decisão é do dono.**

---

## Limites duros

- **Não decidir o conflito** — o passo mede a razão e desenha os candidatos; a escolha (emendar a
  0026, ou aceitar a divergência) é do dono.
- **Não escrever código de produto, não editar L0** (exceto registrar o recon/veredito no L0 se o
  dono já tiver decidido na leitura — senão, só o documento de diagnóstico).
- **Não tocar o α / caso 2, o `morph_canon`/`==`, o caso 4, a flag P350c nem o Marco G.** Medir o
  impacto é leitura; mudá-los é o lote seguinte, se o dono escolher.
- **Não assumir que o conflito é real nem aparente** antes de decompor a razão da 0026 (Trava 1/6).
- **Não importar a quarentena.** `lab/` é leitura de semântica, nunca import.

---

## Verificação (gates)

```
read-only: nenhum código de produto, nenhum L0 funcional tocado (só o documento de diagnóstico, e o
  L0 só se o dono decidir na leitura). Probes revertidas; árvore limpa; suíte 2742 não re-rodada.
saída: f-recon-f5b-adr-0026-0107-passo-370.md — a razão da 0026 decomposta, o que a 0107 exige, a
  tabela candidato × cláusula × α, o veredito (conflito aparente/real), a recomendação.
INTACTOS: α/caso 2, morph ==/morph_canon, caso 4, flag P350c, Marco G, os 3 numbering, o #set de
  props de usuário (P368) — não tocados (sem código).
lente (instrumento): content→elements = 66 (registrar; não é gate).
```

---

## O que NÃO fazer

- **Não decidir o conflito de ADR** (é do dono).
- **Não contornar a razão da 0026** — a solução tem de satisfazê-la; medir a razão primeiro.
- **Não escrever código** nem tocar o α/`morph_canon`/caso 4/flag/Marco G.
- **Não assumir o veredito** (conflito aparente vs real) antes da decomposição.
- **Não emendar nem iniciar o passo seguinte** (Trava 5) — termina no recon; a decisão é do dono.

---

## Relatório (`f-recon-f5b-adr-0026-0107-passo-370.md` + resumo no chat)

A **razão da 0026 decomposta em cláusulas** com citação (`file:line`); o que a **0107** exige no caso
strong/emph/styled (`file:line`, vanilla); o **P101** e o critério da época (válido sob 0026, virou
divergência sob 0107 — datado); a **tabela candidato × cláusula da 0026 × impacto no α** (cada
candidato de modelo avaliado); o **veredito** (conflito aparente — a distinção por discriminante
satisfaz a 0026 — ou real — emenda de ADR); a **recomendação marcada** e a **decisão proposta ao
dono** (o modelo + spec multi-lote, ou o F-5b como divergência consciente registrada); marcado
[medido] vs [inferido] em cada afirmação. Read-only; árvore limpa; lint inalterado; o caveat de
stack. **Termina aqui — não emenda o passo seguinte.**

## Fora de escopo (confirmado)

A **execução** do F-5b (nasce da decisão do dono sobre o conflito); o **Marco G** (não é F; também
toca a 0026, mas é decisão separada); DEBT-59 (flag CLI); DEBT-60 (contador); qualquer toque no α /
`morph_canon` / `==` ou na flag.
