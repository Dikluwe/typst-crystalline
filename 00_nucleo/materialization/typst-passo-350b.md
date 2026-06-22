# Passo 350b — verificar o ponto de leitura e a origem da flag, antes de fixar C

> **Por que este passo.** A trava do P350 ofereceu C (capacidade no `EvalContext`,
> injeção só-de-teste). Eu recomendei C, mas por **raciocínio** ("o `EvalContext` já é o
> lugar certo") — e a ADR-0108 (regra 1) exige que a **medição produza** a decisão, não o
> raciocínio. Este passo **mede**, da fonte e **sem a CLI**, se C é o ponto de **leitura**
> certo e onde fica a **origem** (escrita) do valor — separando o que é verificável agora
> do que só a CLI revela. **Não implementa** a flag; decide a forma de C.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P350b (confirmar livre).
**Pré-condição**: P350 parado na trava (M-canal: **não** existe canal de config L4→L1).
HEAD pós-P349; suíte **2726** / **3245**, lint 0/0, árvore limpa. Se não bater, parar.
**Tipo**: **verificação / medição** — content-preserving estrito: **zero código, zero
teste, zero `.rs`/`.toml`**. Read-only na fonte do `tekt` (L1–L4). Termina na **TRAVA**
com a forma de C decidida. **Não** implementar a flag.
**Objetivo**: distinguir, da fonte, três coisas que decidem se C é seguro e qual a sua
forma — o ponto de **leitura** (onde a classificação lê o booleano), a **origem**
(de onde o valor entra), e a disponibilidade do **histórico** para classificar. A
distinção-chave (ADR-0108): **ler ≠ escrever**. C escolhe o ponto de leitura (verificável
agora); a CLI escolhe a origem (futura) — mas a origem pode já ter um lugar natural (um
`CompileOptions` existente) que C deve apontar agora em vez de inventar.
**Fontes**: a fonte do `tekt` — o `EvalContext` (criação, propagação, onde vive no ponto
do erro de recursão do P348, `apply_show_rules`/`apply_all`), o pipeline de compilação
(L3/L4: `SystemWorld`, a função que chama `eval`), qualquer struct de opções de
compilação existente (procurar `*Options`/`*Config`/`*Flags` em L2–L4). Relatórios P350
(M-canal: sem canal L4→L1; M-classif: o histórico), P348 (o ponto do erro), **ADR-0108**.

---

## As três verificações (da fonte; `file:line`; sem CLI, sem código)

### V1 — o `EvalContext` é o ponto de LEITURA certo? (verificável agora)
1. No ponto onde o teto de recursão dispara (o loop em `apply_show_rules`/`apply_all`,
   P348), o `EvalContext` está **disponível**? (`file:line` do ponto do erro e do acesso
   ao `EvalContext` ali.)
2. O `EvalContext` é o **portador único** da compilação — criado uma vez e propagado por
   todo o eval — ou é **recriado/clonado/fragmentado** por escopo (o que faria um campo
   nele não chegar, ou chegar inconsistente, ao ponto do erro)? Rastrear a criação e a
   propagação (`file:line`).
   - **disponível + portador único** → o `EvalContext` é o ponto de **leitura** certo (um
     campo nele é lido no ponto do erro). C é sólido na leitura.
   - **fragmentado / indisponível no ponto do erro** → C está errado no ponto de leitura;
     reportar onde a flag teria de ser lida em vez disso.

### V2 — já existe um struct de opções que deva ser a ORIGEM? (verificável agora)
"Não há canal **que desce** a L1" (M-canal do P350) **não** é "não existe struct de opções
**nenhum**". Procurar, em L2–L4, um `CompileOptions`/`*Config`/`*Settings`/`*Flags` (ou
equivalente) que já configure a compilação — mesmo que hoje **não** carregue nada que
chegue a L1:
- **existe** → ele é a **origem natural** do campo. C deve pôr o campo **lá** (a origem
  futura) e fazer o `EvalContext` **recebê-lo** — mesmo que, por ora, só o teste o preencha.
  Assim a escrita já tem o lugar certo, e a CLI depois só liga o parsing nesse struct, sem
  refatorar C. (`file:line` do struct.)
- **não existe** → não há origem natural; C põe o campo só no `EvalContext` (leitura) e a
  **origem inteira** (o struct + o caminho até o `EvalContext` + a CLI) vira débito. O
  débito é maior, mas honesto.

### V3 — o histórico de morfologias está disponível/barato no ponto do erro?
Confirmar (do P348/M-classif) que, no ponto do erro, dá para reconstruir/consultar o
histórico de morfologias do caminho de recursão para classificar (cíclico via repetição de
`morph_canon`; divergente; converge-fundo) — **só quando a flag está ligada**. Se não está
disponível, medir o custo de mantê-lo sob a flag (não no caminho quente).

---

## TRAVA — a forma de C decidida (parar; não implementar)

Emitir, com `file:line`:
- **V1**: o `EvalContext` é o ponto de leitura certo (disponível + portador único) — sim/
  não. Se não, onde a flag deve ser lida.
- **V2**: existe um struct de opções que é a origem natural? Se **sim**, C põe o campo lá +
  `EvalContext` recebe (origem apontada agora); se **não**, campo só no `EvalContext` +
  origem inteira como débito.
- **V3**: o histórico para a classificação está disponível/barato sob flag.
- **A forma de C, decidida pela medição** (não pelo raciocínio):
  - **C-com-origem** (V1 sim + V2 existe): campo no struct de opções (origem) → `EvalContext`
    recebe (leitura) → classificação no erro+flag; teste preenche o struct; **só** o parsing
    da CLI fica como débito. A forma mais completa, sem abrir canal especulativo (o struct já
    existe).
  - **C-enxuto** (V1 sim + V2 não): campo só no `EvalContext` (leitura), injeção de teste;
    origem inteira (struct + caminho + CLI) = débito. A variante que o P350 ofereceu.
  - **C inválido** (V1 não): reportar; reabrir a decisão (o ponto de leitura não é o
    `EvalContext`).
**Parar** para o dono confirmar a forma antes do P350c implementar.

---

## Verificação (gates)
```
content-preserving: zero código/teste/.rs/.toml. Árvore intacta (git status limpo). Suíte
  2726/3245 não re-rodada. Read-only (grep/leitura).
lint: crystalline-lint . = 0/0.
evidência: V1 (disponibilidade + propagação do EvalContext, file:line), V2 (o struct de
  opções, file:line, ou a ausência confirmada por busca em L2-L4), V3 (o histórico).
  Inferências marcadas. Zero "~".
distinção ler≠escrever: o relatório separa explicitamente o ponto de LEITURA (verificável
  agora) da ORIGEM/escrita (struct existente ou débito) — não os confunde.
fronteira: não implementou; decidiu a forma de C e parou.
```

---

## O que NÃO fazer
- **Não implementar a flag** — este passo mede e decide a forma; o P350c implementa.
- **Não abrir canal de config especulativo** — se V2 não acha struct, a origem é débito,
  não trabalho agora (a flag é melhoria, não paridade — não construir infraestrutura sem
  demanda).
- **Não confundir ler com escrever** — C decide a leitura (agora); a origem é struct
  existente (apontar) ou débito (nomear).
- **Não assumir o `EvalContext`** — V1 mede; se fragmentado, C cai.
- **Não tocar a CLI** — fora do escopo (débito do P350).
- **Não estimar com "~"**.

---

## Relatório (`typst-passo-350b-relatorio.md`)
- V1: o `EvalContext` no ponto do erro — disponível? portador único? (`file:line`).
- V2: o struct de opções de compilação — existe (origem natural, `file:line`) ou não
  (origem = débito)?
- V3: o histórico de morfologias — disponível/barato sob flag?
- **A forma de C decidida**: C-com-origem / C-enxuto / C-inválido, com a razão medida.
- O que fica como débito (a CLI sempre; a origem inteira se V2=não).
- **Mapa de filtro (campo):** o lugar lógico — "a escolha de C foi medida, não raciocinada:
  separou-se o ponto de **leitura** (verificável agora, o `EvalContext`) da **origem**
  (struct existente ou débito) — segunda aplicação prática da ADR-0108, agora a uma decisão
  do **assistente** (eu recomendei C por raciocínio; a medição confirma ou corrige a forma)"
  — com o rastro (P350 mediu que não há canal L4→L1; P350b mede o ponto de leitura e a
  origem; P350c implementa a forma decidida).
- Item: `content→elements` aponta para o Marco G (P346).
```
