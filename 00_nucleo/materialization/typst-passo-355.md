# Passo 355 — caso 1 (composição): divergência declarada (rumo B) + ordem innermost-first

> **O que faz.** Fecha o caso 1 (composição same-kind) pelo **rumo (B)** do recon
> (`f-recon-caso1-passo-354.md`): **declara a divergência** numa **ADR nova** e aplica a
> **sub-melhoria barata** (inverter a iteração de `node_rules` para **innermost-first**). A
> divergência: para N regras `#show` de mesmo kind sobre um elemento, o crystalline aplica
> **uma regra efetiva** (innermost-first / última-declarada, após o conserto) + a cascata
> cross-kind + o α morfológico — e **NÃO acumula** todas as regras same-kind como o vanilla
> (que o faz via guard por-instância — o mesmo maquinário cujo papel de terminação o P347d/P348
> substituíram pelo α). É **mecânica de realização**, classe que o projeto diverge de propósito
> (ADR-0107), **sem demanda medida** (0 testes de B1; raro em docs reais — ADR-0108). **A
> declaração carrega o peso**; o conserto de ordem só melhora a polaridade do subconjunto de
> uma-regra-efetiva e **não pode ser apresentado como "composição funciona"**. **Não toca** o
> loop α / caso 2 (o conserto é no-op para regra única), o caso 4, o `morph_canon`/`==`, a flag
> P350c nem o Marco G.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P355 (confirmar livre). *(O diagnóstico do contador `1.1`≠`0.1`
desloca para o P356.)*
**Pré-condição**: P354 fechado (recon do caso 1; a colisão composição↔α medida; rumo (B)
escolhido pelo dono). HEAD pós-P354 (sem código novo desde o P352; o Estágio 0 do P353 — rig
de perf — commitado isolado se já foi). Suíte **2733**, lint **0/0**, árvore limpa. Caveat de
stack: `RUST_MIN_STACK=33554432`. Se algo não bater, parar e reportar.
**Tipo**: caso 1, rumo (B) — **declaração de divergência** (ADR/L0) + um conserto de ordem que
**não é content-preserving** para o subconjunto same-kind multi-regra. Esse subconjunto tem
**0 testes hoje** (recontagem P352 Fase A), então **nenhuma asserção existente muda**; os testes
deste lote são novos, e a mudança de comportamento é **declarada e justificada** contra o
vanilla (a polaridade move-se em direção ao vanilla).

---

## Leituras da Fase A (a fonte vence; `file:line`)

1. **ADR-0107 e ADR-0108** — reler e aplicar. A divergência é classificada **da fonte** como
   mecânica de realização (ADR-0107 — composição acumulativa não é semântica/sintaxe/morfologia
   da língua; é o algoritmo de passes). O conserto de ordem move a polaridade observável em
   direção ao vanilla; a aceitação é ao nível da língua, não do booleano.
2. **O recon** — `f-recon-caso1-passo-354.md`: confirmar que (a) o conserto de ordem é **zero
   risco ao α/caso 2** (os testes de caso 2 usam **1 regra** → a reversão é no-op para eles;
   `show_rule_encadeamento_duas_regras` é cross-kind → inalterado); (b) o subconjunto B1 (duas
   regras same-kind, ambas deveriam acumular) **continua divergente** após o conserto.
3. **L0** — `entities/show.md` (`ShowRule`/`Selector`/modelo α) e `entities/f_fronteira_e1.md`
   (§3a/§3c, o caso 1). Sincronizar hashes antes do código (critério 5).
4. **O ponto do conserto** — `rules/eval/rules.rs:121-189` (o loop que aplica a 1ª regra que
   casa em ordem de declaração e `break`) e a iteração de `node_rules` a inverter.
5. **As travas a confirmar verdes**: `p348_show_recursao_converge_para_ponto_fixo`
   (`tests.rs:692`) e os demais testes de caso 2 — devem ficar **inalterados** (no-op).

---

## Limites duros

- **Não tocar o loop α / caso 2 (recursão).** O conserto inverte só a **ordem** de iteração de
  `node_rules`; para regra única (todos os testes de caso 2) é **no-op**. O `morph_canon`/`==`
  (P345) ficam intactos.
- **Não reabrir o guard por-instância ("GEROU").** Acumular todas as regras same-kind exigiria
  o maquinário por-instância que o P347d/P348 dispensaram; este lote **declara a divergência**,
  não a reconcilia (isso seria A2/A3, outro rumo).
- **A declaração carrega o peso, não o conserto.** O conserto de ordem **não** fecha o B1 e
  **não pode** ser apresentado (em teste, L0 ou ADR) como "composição funciona". Sem a
  declaração explícita, o conserto mascararia o buraco (a falha do S5b) — por isso a ADR é o
  Estágio com Trava e o conserto vem depois, amarrado a ela.
- **Não tocar o caso 4, a flag P350c, o DEBT-59 nem o Marco G** (`edges content→elements` = 66).

---

## Estágios

### Estágio ADR/L0 — a declaração (Trava; PARA aqui para o dono)
Escrever, sincronizar hashes, e **parar** para aprovação do dono antes de qualquer `.rs`:

1. **ADR nova** (`ADR-0109-divergencia-composicao-same-kind.md`, numerar livre), **EM VIGOR**
   após o dono selar:
   - **Decisão**: para N regras `#show` de mesmo kind sobre um elemento, o crystalline aplica
     **uma regra efetiva** (innermost-first / última-declarada) + a cascata cross-kind + o α
     morfológico; **não acumula** todas as same-kind (comportamento do vanilla via guard
     por-instância).
   - **Classificação (ADR-0107)**: a acumulação é **mecânica de realização** (ordem/quantidade
     de passes), não língua (semântica/sintaxe/morfologia) — classe que o projeto diverge de
     propósito. O guard por-instância que daria a acumulação é o mesmo cujo papel de terminação
     o P347d/P348 substituíram pelo α; trazê-lo "à letra" reabriria o caso 2 fechado.
   - **Custo nomeado**: o caso B1 do spike (duas regras same-kind, ambas deveriam aplicar) é o
     caso divergente; **0 testes hoje**; raro em docs reais.
   - **Gatilho de reabertura concreto** (padrão SetPage/show): quando a cobertura da linguagem
     exigir composição same-kind real, os casos B1 viram **testes de paridade contra o vanilla
     medido**, e o caso 1 vira lote de implementação (A2 ou A3) nessa hora — não antes.
2. **L0** — `show.md` + `f_fronteira_e1.md`: registrar a divergência com cross-reference à
   ADR-0109; anotar que o conserto de ordem é polaridade do subconjunto de uma-regra-efetiva,
   não acumulação.

**TRAVA**: o passo termina aqui no chat — ADR + L0 + hashes para o dono aprovar. Nenhum código
antes.

### Estágio 1 — o conserto de ordem (após aprovação)
Inverter a iteração de `node_rules` para **innermost-first** (última-declarada primeiro), de
modo que, no subconjunto onde **uma** regra é efetiva, a regra que vence case com a polaridade
do vanilla ("última-declarada vence"). **Sem** guard por-recipe; **sem** tocar o α.

### Estágio Teste
- **Novo**: no subconjunto de uma-regra-efetiva, a **última-declarada** vence (polaridade do
  vanilla) — ex.: `#show heading: upper` ⨁ `#show heading: it=>[X:]+it.body` → o efeito da
  **segunda** declarada, casando a polaridade medida no P352 Fase A.
- **Novo (documenta a divergência)**: B1 — duas regras same-kind onde ambas deveriam acumular
  → o crystalline aplica **o comportamento declarado** (uma efetiva), **marcado no teste como
  divergência consciente vs vanilla** (com referência à ADR-0109). O teste assere o
  comportamento **declarado**, não finge acumulação.
- Confirmar por construção/teste que `p348`/caso 2, o caso 4, o `morph_canon`/`==` e a flag
  P350c **ficaram intactos** (o conserto é no-op para regra única).

### Estágio F — linhagem
`@updated`; `--fix-hashes`; V7 limpa.

---

## Verificação (gates)

```
build: limpo por estágio.
suíte (RUST_MIN_STACK=33554432): 2733 + os testes novos (polaridade + a divergência documentada).
  ZERO asserção existente alterada (o subconjunto same-kind multi-regra tinha 0 testes).
lint: crystalline-lint . = 0/0.

ACEITAÇÃO (observável, nível da língua — ADR-0107/0108):
  - subconjunto de uma-regra-efetiva: a última-declarada vence (polaridade do vanilla).
  - B1 (acumulação): DIVERGÊNCIA DECLARADA — o teste documenta o comportamento do crystalline
    com referência à ADR-0109; NÃO é apresentado como paridade.

INTACTOS (confirmar): loop α / caso 2 (p348 verde, no-op), caso 4 (P340), morph ==/morph_canon
  (P345), flag P350c, DEBT-59, Marco G (edges content→elements = 66).

lente (critério 3): edges(content→elements::*) = 66 INALTERADO; edges(elemento→elemento) = 0.
perf (critério 4): antes = o baseline do P353 Estágio 0 (1.1991 s ± 0.0742, mesma sessão);
  reportar o depois — o conserto é reordenação O(regras), esperado ~nulo; afirmar só o medido.
L0 (critério 5): ADR-0109 + show.md + f_fronteira_e1.md com hashes sincronizados ANTES do
  código, Trava cumprida (aprovação do dono registrada).
```

---

## O que NÃO fazer

- **Não acumular as regras same-kind** (isso é A2/A3 — outro rumo; reabriria o α ou divergiria
  na ordem). Este lote **declara**, não reconcilia.
- **Não apresentar o conserto de ordem como "composição funciona".** A ADR-0109 é o que mantém
  o lote honesto; o conserto sem a declaração mascararia o B1.
- **Não tocar o loop α / caso 2, o caso 4, o `morph_canon`/`==`, a flag P350c nem o Marco G.**
- **Não pular a Trava.** ADR + L0 + hash do dono antes de qualquer `.rs`.
- **Não importar a quarentena.** `lab/` é leitura de semântica, nunca import.

---

## Relatório (`typst-passo-355-relatorio.md` + resumo no chat)

A ADR-0109 (a divergência, a classificação ADR-0107, o gatilho de reabertura) + o L0 com hashes
e a Trava aprovada; o conserto de ordem com `file:line`; os testes novos (polaridade + a
divergência B1 documentada com referência à ADR); a prova de que `p348`/caso 2, caso 4, morph
`==` e a flag ficaram intactos; os números da lente (edges 66) e da perf (antes/depois);
`git status` limpo por estágio fora de `lab/` e docs; lint 0/0; o caveat de stack.

## Fora de escopo (confirmado)

A acumulação same-kind (A2/A3 — nasce do gatilho de reabertura, com demanda medida); o
diagnóstico do contador `1.1`≠`0.1` + supplement "Secção" (P356); o de-bake do F-5 (limpeza sem
demanda — adiado no P353); F-6; Marco G; flag na CLI (DEBT-59).
