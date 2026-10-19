# Passo 374 — medição da atomização após o F (recon read-only, em valores)

> **O que faz.** **Read-only.** Com o F fechado (P373: item (1) 4/4, DEBT-61 fechado), mede **em
> valores** o quanto a atomização melhorou ao longo do arco. **Roda a lente `tekt-cargo-dsm`** no
> HEAD atual e traz os números **antes/depois**, mas **separa duas coisas que não são a mesma** (a
> lição central do arco — a ferramenta mede, o princípio define):
> - **(a) a atomização que o F entregou, medida pelo PRINCÍPIO** (fonte única de verdade): os
>   caminhos duplos eliminados, os arms de `morph_canon` colapsados, o parâmetro de ABI removido,
>   a distinção semântica restaurada. **Isto é o que o F perseguiu** — é contagem de estruturas, com
>   `file:line`, não um único número da lente.
> - **(b) as métricas da lente** (`content→elements`, `elemento→elemento`), medidas como
>   **INSTRUMENTO**, com a ressalva explícita: o `content→elements` é o alvo do **Marco G** (órfão
>   da lente, P346), **não gate do F**; e ele **subiu** (66→~68) por **variantes prescritas** pela
>   0026/0105 (Strong/Emph, P371), o que **não é regressão de atomização** — é o modelo prescrito.
>
> **Não estima — mede** (Trava 1/8: número sem `file:line`/medição é suposição). **Zero código de
> produto, zero L0.** Termina no relatório. Saída:
> `00_nucleo/diagnosticos/atomizacao-medida-pos-f-passo-374.md`.

**Repositório de trabalho**: typst-crystalline (raiz), branch `tekt`.
**Número do passo**: P374 (confirmar livre).
**Pré-condição**: P373 fechado (F completo pelos princípios; item (1) 4/4; suíte verde, pipeline
completo). HEAD pós-P373, árvore limpa. A lente `tekt-cargo-dsm` (commit `98d8f9e` nos relatórios
anteriores; **confirmar e registrar a versão usada**). Caveat de stack: `RUST_MIN_STACK=33554432`.
Se algo não bater, parar e reportar.
**Tipo**: **medição/recon read-only** — roda a lente (ferramenta de medição) + leitura/grep; zero
código de produto, zero L0. Termina no relatório com os valores. **A lente é instrumento; os
números não são gate** — são o retrato pedido pelo dono.

---

## O que medir (a fonte/lente vence; `file:line` + os números)

### Parte (a) — a atomização que o F entregou (medida pelo PRINCÍPIO)
Contar, com `file:line`, o que o F removeu/colapsou ao longo do arco (P331→P373). A fonte são os
relatórios + o repo no HEAD atual (confirmar que o que os relatórios dizem feito está no código):

1. **Caminhos duplos eliminados** (fonte única — o item (1) da auditoria): os 4 campos assados que
   coexistiam com a chain e foram removidos —
   - `HeadingElem.numbering_active` (P364);
   - `EquationElem.numbering_active` (P364);
   - `FigureElem.numbering` (P365);
   - o `TextStyle` assado de `Content::Text` (P373).
   Confirmar que cada campo **não existe mais** no HEAD (`file:line` da ausência / do construtor
   simplificado). **Valor: 4 → 0 caminhos duplos.**
2. **Arms de `morph_canon` colapsados**: os arms dedicados que foram subsumidos pelo arm `Styled`
   transparente — heading, equation, figure (P364/P365). Confirmar no HEAD. **Valor: N arms → o arm
   único.**
3. **ABI removido**: o parâmetro partilhado `figure_numbering: Option<&str>` (141 ocorrências, 16
   ficheiros, P365). Confirmar a ausência. **Valor: 141 → 0 ocorrências.**
4. **Distinção semântica restaurada** (a fidelidade à 0107): `strong`/`emph`/`#set text` agora
   distintos (P371/P373) — `strong X ≠ emph X ≠ #set text X ≠ X`. Confirmar (o teste
   `f5b_strong_distinto_de_set_text_bold` + o `==`).
5. **Canal único de estilo**: o `StyleDelta.custom` agora serve numbering + `#set` de props de
   usuário + render `#set text` (P368/P373), com o transporte aninhado correto (P373). Confirmar os
   consumidores.

### Parte (b) — as métricas da lente (medidas como INSTRUMENTO)
Rodar a lente `tekt-cargo-dsm` no HEAD atual e registrar, com o número e a versão da lente:

6. **`edges(elemento→elemento)`** — a atomização no sentido estrito (elementos independentes). Os
   relatórios trazem **0** o arco todo. **Confirmar = 0** no HEAD.
7. **`edges(content→elements::*)`** — o acoplamento de import. **Medir o valor atual** e comparar
   com o baseline (66 no início do arco; ~68 esperado após as variantes Strong/Emph da P371; o
   de-bake do P373 pode ter mexido — **medir, não inferir**). **Registrar com a ressalva dupla:**
   - é o alvo do **Marco G**, **não gate do F** (P346 — órfão da lente);
   - subiu por **variantes prescritas** (0026/0105), **não regressão** — decompor o delta (quantas
     arestas são as variantes novas vs quantas o de-bake removeu).
8. **Qualquer outra métrica que a lente reporte** (se houver) — registrar como instrumento.

---

## A saída — a tabela antes/depois, em valores

O recon entrega:
- **A tabela da Parte (a)**: caminhos duplos (4→0), arms de `morph_canon` colapsados, ABI removido
  (141→0), a distinção restaurada — cada um com `file:line` e o valor. **Esta é a atomização que o F
  entregou.**
- **A tabela da Parte (b)**: `elemento→elemento` (0), `content→elements` (baseline → atual, com o
  delta decomposto: variantes prescritas vs de-bake), a versão da lente. **Com a ressalva de que
  `content→elements` é alvo do Marco G, não gate do F.**
- **A leitura honesta**: o quanto a atomização melhorou **pelo princípio** (a Parte a — o número
  real do F) vs o que a lente mostra em `content→elements` (a Parte b — que sobe por desenho, e cujo
  alvo 0 é o Marco G, fora do F). **Não confundir as duas** (o erro do arco).

---

## Limites duros

- **Não estimar nenhum número** — rodar a lente e contar no repo, com `file:line` (Trava 1/8). Se a
  lente não buildar/rodar, **registrar isso** e dar a Parte (a) por contagem no repo, marcando a
  Parte (b) como não-medida.
- **Não apresentar `content→elements` como "a atomização do F"** — é o alvo do Marco G (não-F); a
  atomização do F é a Parte (a). Registrar a distinção (a lição central do arco).
- **Não tratar o 66→68 como regressão** — decompor: as variantes Strong/Emph são o modelo prescrito
  (0026/0105).
- **Zero código de produto, zero L0.** Probes/lente revertidas; árvore limpa.
- **Não importar a quarentena.**

---

## Verificação (gates)

```
read-only: nenhum código de produto, nenhum L0; a lente rodada (ferramenta); suíte não re-rodada.
saída: atomizacao-medida-pos-f-passo-374.md — a tabela (a) princípio + a tabela (b) lente, antes/
  depois, com file:line e os números.
lente: a versão/commit registrada; os valores (elemento→elemento, content→elements) com o delta
  decomposto.
INTACTOS: nada tocado (sem código).
```

---

## O que NÃO fazer

- **Não estimar** — medir (rodar a lente, contar no repo).
- **Não confundir `content→elements` (Marco G, não-F) com a atomização do F** (a Parte a).
- **Não ler o 66→68 como piora** — é variante prescrita.
- **Não escrever código nem L0.**
- **Não emendar nem iniciar o passo seguinte** (Trava 5) — termina no relatório; a decisão é do dono.

---

## Relatório (`atomizacao-medida-pos-f-passo-374.md` + resumo no chat)

A **tabela (a)** — a atomização que o F entregou, pelo princípio: os 4 caminhos duplos eliminados
(`file:line` da ausência), os arms de `morph_canon` colapsados, o ABI removido (141→0), a distinção
semântica restaurada, o canal único; cada um com o valor antes/depois. A **tabela (b)** — a lente
como instrumento: `elemento→elemento` (= 0), `content→elements` (baseline → atual, com o delta
decomposto em variantes-prescritas vs de-bake), a versão da lente. A **leitura honesta** separando
as duas (a atomização do F vs o `content→elements` do Marco G). Marcado [medido] em cada número (com
o `file:line` ou a saída da lente); se a lente não rodar, [não-medido] com o porquê. Read-only;
árvore limpa; lint inalterado; o caveat de stack. **Termina aqui — não emenda o seguinte.**

## Nota sobre o Marco G (para contexto da leitura, não escopo)

O `content→elements` que a Parte (b) mede é o alvo do **Marco G** — desacoplar o núcleo dos nativos
para `content→elements → 0`. O P361 mediu que o Marco G é uma decisão de modelo (α-vtable vs
β-PropMap), grande, fora do F. **Este passo não o executa nem o recomenda** — só registra o número
atual de `content→elements` para o dono ver onde está, com a ressalva de que reduzi-lo é o Marco G,
um trabalho separado e não-F.

## Fora de escopo (confirmado)

A execução do Marco G (decisão de modelo separada, não-F); DEBT-59 (flag CLI); DEBT-60 (contador);
qualquer código ou L0.
