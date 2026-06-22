# Passo 375 — Marco G: medir os três eixos (atomização líquida, leitura, custo-IA) em valores

> **O que faz.** **Read-only.** Responde **em valores** as três perguntas do dono sobre o Marco G,
> que hoje só têm inferência (o P361 mediu a estrutura, não os saldos):
> 1. **Atomiza o núcleo?** — o saldo **líquido** de código central: os arms/linhas/arquivos que o
>    Marco G **remove** do hub (`content.rs`) vs a **tabela de handlers** que ele **cria** e que
>    re-centraliza (importa os elementos). Atomiza, reloca, ou as duas coisas — e qual o saldo.
> 2. **Ganho de leitura ou algorítmico?** — arms colapsados (menos repetição) vs a superfície nova
>    (a tabela + o teste/lint de exaustividade que **repõe** o que o `match` dava de graça). E o
>    saldo algorítmico (`match` estático O(1) vs despacho dinâmico).
> 3. **Melhora para a IA?** — o **custo de adicionar um elemento** hoje (quantos pontos toca) vs
>    depois (implementar o trait + registrar), **menos** a perda da exaustividade do compilador (a
>    rede que hoje pega o caso esquecido). Os dois efeitos, medidos.
>
> **Mede os SALDOS, não a estrutura solta** — separa o que **atomiza** do que **reloca** (a lição:
> `content→elements→0` reloca o acoplamento, não o elimina — P361). **Não desenha a execução, não
> decide, não recomenda fazer** — só dá os valores para o dono pensar nas formas elegantes depois.
> A lente é **instrumento**. **Zero código de produto, zero L0.** Saída:
> `00_nucleo/diagnosticos/marco-g-tres-eixos-passo-375.md`.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P375 (confirmar livre).
**Pré-condição**: F fechado (P373; P373 commitado — confirmar, senão registrar). Suíte verde, lint
0/0. HEAD pós-P373/P374. Lente `tekt-cargo-dsm` (`98d8f9e`) disponível. Caveat de stack:
`RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: **medição/recon read-only** — lente + leitura + grep + probes mentais (nenhuma alteração).
Termina no relatório com os valores dos três eixos. **Não é desenho de execução** — o desenho das
"formas elegantes e precisas" é o passo seguinte, **se** o dono decidir, com estes valores na mão.

---

## O que medir (a fonte vence; `file:line` + os números)

### Eixo 1 — atomização líquida do núcleo
1. **O que o Marco G removeria do hub** (`content.rs`): os ~320 arms monomórficos
   (`plain_text`/`is_empty`/`map_content`/`map_text`/`get_field`/`eq`) que colapsariam em
   `Content::Dynamic(e) => e.dyn_*()`. **Contar**: arms, linhas, e em quantos `match` distintos.
   Quanto de `content.rs` (linhas) sai.
2. **O que o Marco G criaria/re-centralizaria**: a **tabela kind→handler de layout** (os 48 arms de
   `layout/mod.rs` que leem campos concretos viram entradas de tabela com downcast) — **contar** a
   superfície estimada (entradas + os 48 handlers + o módulo novo) e **se importa os elementos** (a
   relocação do acoplamento). Idem o arm `Dynamic` no walk do introspect.
3. **O saldo líquido**: linhas/arms removidos do núcleo **menos** os criados na tabela. Atomiza (o
   núcleo encolhe), reloca (muda de arquivo), ou as duas — **com o número**, não a impressão.
4. **Arquivos**: quantos arquivos o núcleo toca hoje para os nativos vs depois. O Marco G aumenta,
   diminui, ou mantém a contagem de arquivos do núcleo?

### Eixo 2 — leitura e algoritmo
5. **Leitura — o que melhora**: a repetição removida (320 arms → 1) — quantificar.
6. **Leitura — o que piora**: a **perda da exaustividade do `match`** (o compilador deixa de garantir
   que todo kind tem handler) + a superfície que a **repõe** (a ADR-0105 cl.3 exige um teste-varre-
   tabela ou regra de lint) — quantificar a superfície dessa reposição. **O saldo de leitura é
   menos-repetição vs menos-garantia-estática — declarar os dois, não só o lado bom.**
7. **Algoritmo**: confirmar (medir/raciocinar com `file:line`) que o despacho passa de `match`
   estático (O(1), jump table) para despacho dinâmico (vtable α / lookup PropMap β) — **igual ou
   pior** em runtime; **sem ganho algorítmico**. Registrar a direção; a magnitude (perf) só se a
   lente/um micro-bench der barato — senão, marcar [não-medido, direção inferida].

### Eixo 3 — custo para a IA (adicionar/manter um elemento)
8. **Custo de adicionar um elemento HOJE**: contar os pontos que um elemento novo toca — os match
   arms do hub, o layout, o introspect, etc. (medir pelo que o `callout`/`badge` precisou, ou pelo
   diff de um elemento nativo). **Quantos pontos, quantos arquivos.**
9. **Custo de adicionar um elemento DEPOIS do Marco G**: implementar o `trait Element` + registrar +
   **a entrada na tabela de handlers de layout** (que o Marco G cria). **Quantos pontos** — incluir
   a entrada na tabela (não esquecer o lado que re-centraliza).
10. **A perda de rede para a IA**: hoje o compilador pega o caso esquecido (exaustividade); depois,
    um kind sem handler é falha de runtime até o teste/lint pegar. **Pesar os dois**: o padrão único
    (ganho para IA — um molde só) vs a perda da garantia estática (perda para IA — menos rede). Não
    declarar só o lado bom (o critério "economia para IA" do P332 era o argumento pró; o contra é
    igualmente real).

---

## A saída — a tabela dos três eixos, em valores

O recon entrega, para cada eixo, o **saldo** com número e `file:line`:
- **Eixo 1**: removido do núcleo (arms/linhas) − criado na tabela = saldo líquido; arquivos
  antes/depois; **atomiza vs reloca, com o número**.
- **Eixo 2**: repetição removida vs superfície da exaustividade reposta (saldo de leitura);
  algoritmo (sem ganho, direção).
- **Eixo 3**: pontos para adicionar elemento hoje vs depois; o padrão-único (pró-IA) vs a perda de
  exaustividade (contra-IA).
- **A leitura honesta**: em quais eixos o Marco G é ganho **líquido**, em quais é trade, em quais é
  perda — sem inflar (o erro do P346) nem descartar por reflexo. Marcado [medido]/[inferido].

---

## Limites duros

- **Medir saldos, não a estrutura solta.** "320 arms colapsam" sozinho é meia-verdade; o saldo
  inclui a tabela de handlers que re-centraliza. Os dois lados, sempre.
- **Não declarar só o lado bom** de nenhum eixo (a Trava 7 — o enquadramento cômodo). Cada eixo tem
  um pró e um contra medidos.
- **Não desenhar a execução nem decidir** — o passo dá os valores; o desenho (formas elegantes) e a
  decisão são do dono, depois.
- **A lente é instrumento** — `content→elements` é o alvo do Marco G, mas a pergunta aqui é o
  **benefício** (os três eixos), não o número da lente por si.
- **Zero código de produto, zero L0.** Probes/lente revertidas; árvore limpa.
- **Não importar a quarentena.**

---

## Verificação (gates)

```
read-only: nenhum código de produto, nenhum L0; lente/grep/leitura; suíte não re-rodada.
saída: marco-g-tres-eixos-passo-375.md — os 3 eixos em valores (saldo líquido com file:line),
  a leitura honesta (ganho/trade/perda por eixo), marcado medido/inferido.
lente (instrumento): versão registrada; usada para o eixo 1 (acoplamento) se ajudar.
INTACTOS: nada tocado (sem código).
```

---

## O que NÃO fazer

- **Não medir só o lado que atomiza** (o hub) sem o lado que reloca (a tabela de handlers).
- **Não declarar só o pró** de leitura/IA — cada eixo tem o contra medido.
- **Não desenhar a execução** (as formas elegantes) — é o passo seguinte, se o dono decidir.
- **Não recomendar fazer/não-fazer o Marco G** — dar os valores; a decisão é do dono.
- **Não confundir o número da lente (`content→elements`) com o benefício** — o benefício são os 3
  eixos.
- **Não escrever código nem L0.**
- **Não emendar nem iniciar o passo seguinte** (Trava 5) — termina no relatório.

---

## Relatório (`marco-g-tres-eixos-passo-375.md` + resumo no chat)

A **tabela dos 3 eixos** em valores: (1) atomização líquida — removido do núcleo − criado na tabela,
arquivos antes/depois, atomiza vs reloca; (2) leitura — repetição removida vs exaustividade reposta,
e o algoritmo (sem ganho, direção); (3) custo-IA — pontos para adicionar elemento hoje vs depois, o
padrão-único vs a perda de exaustividade. Cada número com `file:line` e [medido]/[inferido]. A
**leitura honesta** (ganho líquido / trade / perda por eixo, sem inflar nem descartar). A versão da
lente. Read-only; árvore limpa; lint inalterado; o caveat de stack. **Termina aqui — não desenha a
execução nem decide; isso é o passo seguinte, se o dono escolher, com estes valores na mão.**

## Contexto (não escopo)

O P361 mediu que o Marco G é decisão de modelo (α-vtable vs β-PropMap, a 0026 rejeita o vtable),
grande (~640 arms), fora do F. O P374 mediu `content→elements = 68` (alvo do Marco G, não-F). **Este
passo não decide o modelo nem executa** — mede o **benefício** (os 3 eixos) para o dono julgar se
vale, e só então pensar nas formas elegantes e precisas de executar.

## Fora de escopo (confirmado)

O desenho/execução do Marco G (passo seguinte, se o dono decidir); a decisão de modelo α/β; DEBT-59;
DEBT-60; qualquer código ou L0.
