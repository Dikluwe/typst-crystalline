# Passo 354 — Diagnóstico/desenho: caso 1 (composição) — reconciliar com α, ou divergência declarada

> **O que faz.** **Recon read-only** (zero código, zero L0, árvore limpa). Mede a fundo a
> colisão que o P352 Fase A achou: a **composição fiel ao vanilla** (N regras sobre o mesmo
> elemento aplicam-se **todas, uma vez cada, innermost-first**) exige um **guard por-regra**
> que é **incompatível** com o **re-apply morfológico do loop α** (caso 2, recursão, **fechado**
> no P348). Entrega as **duas saídas** com custo medido: **(A) reconciliar** — um modelo de
> loop que dê composição **e** preserve o ponto-fixo morfológico a→b→c — ou **(B) divergência
> consciente declarada**. **Para na decisão do dono.** **NÃO decide o rumo** — produz a medição
> para o dono decidir (a escolha é arquitetural). Saída: `f-recon-caso1-passo-354.md`.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P354 (confirmar livre; **independente do P353** — read-only, roda a
qualquer hora; não depende do F-5).
**Pré-condição**: P352 fechado (a colisão do caso 1 foi medida e registrada na Fase A do P352,
§4). HEAD pós-P352, árvore limpa, lint **0/0**, suíte **2733**. Caveat de stack:
`RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: **diagnóstico/recon** — zero código de produto, zero edição de L0. A saída é um
documento de recon; nenhuma asserção tocada; suíte não re-rodada (read-only). O passo
**termina numa decisão proposta ao dono**, como os recons P337/P347d.

---

## O que medir (a fonte vence; `file:line`)

1. **ADR-0107 e ADR-0108** — reler e aplicar. A decisão do rumo é do dono; o recon **mede**,
   não decide (ADR-0108: afirmar só o medido; não decidir da narrativa).
2. **O modelo do vanilla, em cheio** (leitura da quarentena, nunca importar):
   - `lab/.../typst-realize/src/lib.rs:449-486` + `styles.rs:835` (`next_back`): a iteração
     innermost-first.
   - O **guard por `RecipeIndex`** (`elem.is_guarded(index)`, `lib.rs:472-474`) e o **bitset de
     lifecycle por-instância** (`content/mod.rs:148-156`).
   - **Como o vanilla faz recursão**: a regra transforma o elemento `E` em conteúdo que contém
     `E'`; medir se `E'` é uma **instância nova** (não guardada para aquela recipe) e se é
     **por isso** que a recursão funciona **sem** re-aplicar a mesma recipe ao mesmo nó.
     Esta é a pergunta-chave: o vanilla reconcilia composição e recursão **via guard
     por-instância + recursão por novas instâncias**?
3. **A decisão P347d/P348 — por que o guard por-instância foi recusado**:
   - Reler o relatório do P348 e o `f-recon-passo-347d`: o que exatamente "**GEROU**" (identidade
     nova? quebrou que invariante? custou o quê?) ao tentar o guard por-instância.
   - Medir se essa recusa era **específica do caso da recursão** (onde se **quer** o re-apply) ou
     se ela **bloqueia também** a composição (onde se quer o apply-uma-vez de regras distintas).
4. **O loop α atual**: `rules/eval/rules.rs:97-248`; o ponto-fixo morfológico; o teste
   `p348_show_recursao_converge_para_ponto_fixo` (`tests.rs:692`); e o ponto da divergência de
   ordem que o P352 mediu (`rules.rs:121-189`: aplica a 1ª regra que casa em ordem de declaração
   e `break`).

---

## A pergunta central (medir, não decidir)

A composição fiel **exige reabrir** a decisão P347d/P348 (trazer o guard por-instância que eles
recusaram), **ou** existe um modelo de loop que dê **composição** (regras distintas, uma vez
cada, innermost-first) **e** preserve o **ponto-fixo morfológico do α** para a recursão?

O vanilla tem os dois ao mesmo tempo — guard por-instância para as regras distintas, recursão
via novas instâncias. O cristalino tem o α morfológico e **não** tem o maquinário por-instância
(ou recusou-o no P347d/P348). O recon mede se essa recusa ainda vale e se há um caminho.

---

## Saída — o recon entrega (`f-recon-caso1-passo-354.md`)

Para cada opção, com `file:line` e custo medido:

### Opção (A) — reconciliar
- O **modelo de loop concreto** que daria composição **e** preservaria a→b→c (ex.: separar
  "regras distintas, guardadas por `RuleId`, uma vez cada" de "re-apply morfológico da mesma
  regra"; ou copiar o por-instância do vanilla; ou outro).
- **Onde tocaria** (`file:line`), quantos sítios, quais testes mexem.
- **Reabre P347d/P348?** Se sim, qual o invariante que o "GEROU" protegia e como o novo modelo o
  preserva — ou por que o "GEROU" não se aplica à composição.
- Se a Fase A **não achar** um modelo, dizer "**não achado**" e por quê (como o P352 Fase A fez
  — afirmar só o medido).

### Opção (B) — divergência consciente declarada
- O cristalino aplica **só a primeira regra que casa** (o comportamento atual) — declarado como
  **divergência consciente** vs vanilla (que aplica todas, innermost-first).
- **O custo**: que casos/documentos divergem (o canônico B1 do spike: 2 regras, ambas deveriam
  aplicar); a perda de paridade nomeada, não escondida.
- O **gatilho de reabertura concreto** (padrão SetPage/show): quando a cobertura da linguagem
  exigir composição real, os casos viram testes de paridade contra o vanilla medido, e o caso 1
  vira lote nessa hora.

### Recomendação do agente (marcada) + a DECISÃO é do dono
A recomendação fica marcada como recomendação; a escolha (A) ou (B) é do dono, e o próximo lote
(implementação ou divergência registrada) só nasce depois dela.

---

## O que NÃO fazer

- **Não escrever código de produto.** Probes descartáveis para medir comportamento são
  permitidas (compiladas e **revertidas**, árvore limpa — padrão do P352 Fase A); produção, não.
- **Não editar L0.** A edição só nasce depois da decisão do dono, no lote seguinte.
- **Não decidir o rumo.** O recon mede e recomenda; o dono decide.
- **Não tocar o loop α / caso 2, o caso 4, o `morph_canon`/`==` nem a flag P350c.** Read-only.
- **Não importar a quarentena.** `lab/` é leitura de semântica, nunca import.

---

## Saída/relatório (`f-recon-caso1-passo-354.md` + resumo no chat)

O modelo do vanilla medido com `file:line` (guard por-instância + recursão por novas
instâncias); a razão real do "GEROU" do P347d/P348 e se ela bloqueia a composição; as duas
opções (A reconciliar / B divergência declarada) com custo e `file:line`; a recomendação
marcada; a decisão proposta ao dono. Suíte não re-rodada (read-only); árvore limpa; lint
inalterado; caveat de stack se alguma probe foi compilada e revertida.

## Fora de escopo (confirmado)

A **implementação** do caso 1 (nasce do lote seguinte, após a decisão do dono); F-5 (P353);
F-6; Marco G; flag na CLI (DEBT-59); qualquer toque no loop α / `morph_canon` / `==` ou na flag.
