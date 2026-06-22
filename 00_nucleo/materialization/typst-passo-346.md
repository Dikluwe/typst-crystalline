# Passo 346 — decisão de planejamento: o destino do `content → elements → 0`

> **O que é.** Fechar o item aberto que o relatório carrega há vários passos sem
> dono: o baseline da lente mede `content → elements::* = 66` e espera `target = 0`,
> mas **nenhum lote** da fila F entrega esse corte. Este passo **não é código** — é a
> decisão do dono entre três saídas, materializada como o registro que ela escolher.
> Documentação pura.

**Repositório de trabalho**: typst-crystalline (raiz).
**Número do passo**: P346 (confirmar livre).
**Pré-condição**: P345 fechado (`==` morfológico, ADR-0107 exercida; suíte 2723 /
3242, lint 0/0). HEAD pós-P345; árvore limpa. Se não bater, parar.
**Tipo**: **decisão de planejamento / documentação** — content-preserving estrito:
**zero `.rs`/`.toml`**. Toca só o(s) documento(s) que a decisão escolher. **Termina
na fronteira de revisão do dono**: apresenta as três saídas com a consequência de
cada uma e **para** — a escolha é do dono, não do agente.
**Objetivo**: o dono escolher uma das três saídas e o passo materializar **só** o
registro correspondente, para o item parar de ser carregado em todo relatório.
**Fontes**: o baseline da lente (`baseline-estrutural-lente-passo-338.md` — o
`content→elements = 66`, `target = 0`, o modelo D de enum fechado), os relatórios
P339–P345 (o item carregado), `f-plano-lotes-passo-333.md` (a fila F), os limites
R1–R5 do baseline (o R3 module-level que mede a aresta).

---

## O fato, fixado (para a decisão não ser sobre suposição)

Medido e repetido em todos os baselines desde P333: `content → elements::* = 66`
(era 65; +1 por um elemento novo), `target = 0`. **Nenhum lote da fila** F-1…F-6 +
F-realização entrega esse corte — confirmado lote a lote: o F-1 foi aditivo (o gate
do F-1 dizia explicitamente que `content→elements` fica inalterado), e os demais
(F-2 canal, F-3/F-realização `#show`, F-4 backing, F-5 de-bake, F-6 folhas) também
não removem os imports `use elements::*Elem` do núcleo. A fila torna os elementos
**extensíveis** e unifica o modelo de estilo/realização; ela **não** desacopla
`content` dos 65 tipos de elemento concretos. O baseline, do lado da lente, espera
um `target = 0` que a fila nunca prometeu — é uma **expectativa órfã**, não uma
regressão.

---

## As três saídas (o dono escolhe UMA; o passo materializa só ela)

### Saída 1 — reconciliar o baseline (aceitar `≠ 0` por desenho)
O modelo D (enum fechado) tem `content → elements ≠ 0` **por desenho** — o hub
despacha para cada elemento, então importa cada um. Ação: editar o
`baseline-estrutural-lente-passo-338.md` (e a nota de contrato de verificação do F)
para o `target` deixar de ser `0` e passar a ser **"≠ 0 esperado; a métrica-gate do
F é outra"** (ex.: atomização preservada, 0 arestas elemento→elemento). O `target=0`
some como expectativa.
- **Consequência**: o item fecha sem prometer trabalho futuro. O baseline para de
  reportar uma falha que não é falha. **Custo**: uma edição de doc. **Implica**: o
  projeto aceita que `content` depende dos nativos concretos — o que é coerente com o
  modelo D, mas fecha a porta do desacoplamento sem nomeá-la.

### Saída 2 — nomear um marco pós-F-6 (o desacoplamento como objetivo futuro)
O corte `content → elements → 0` é um objetivo **maior** que a fila atual habilita
mas não entrega (converter os 65 nativos a passarem pela fronteira de extensão, para
`content` parar de importar cada `*Elem`). Ação: registrar um **marco nomeado**
(ex.: "G — desacoplamento dos nativos") em `f-plano-lotes-passo-333.md`, **fora** da
fila F, **pós-F-6**, com a sua dependência (a fronteira E1, que o F-1 construiu) e o
`target=0` como a sua métrica — não a do F.
- **Consequência**: o `target=0` deixa de ser órfão (ganha dono: o marco G) sem
  comprometer trabalho nesta branch. **Custo**: uma edição de doc. **Implica**: o
  desacoplamento vira objetivo declarado do projeto, a fazer depois — uma migração
  grande (mover 65 elementos pela fronteira), com spec própria quando chegar a vez.

### Saída 3 — registrar como lacuna do plano
Registrar que a fila F-1…F-6 + F-realização **não inclui** o corte, como **lacuna
explícita** — sem decidir se será reconciliada (saída 1) ou virará marco (saída 2).
Ação: uma nota em `f-plano-lotes-passo-333.md` declarando a lacuna e adiando a
decisão entre 1 e 2.
- **Consequência**: honesto, mas **não fecha** o item — só o nomeia. **Custo**: uma
  nota. **Implica**: o item para de parecer esquecido, mas volta a pedir decisão
  depois. É a saída de menor compromisso (útil se você quer fechar o F antes de
  decidir o destino do corte).

---

## Recomendação (do agente, marcada como tal)

**Saída 2.** Razão: o desacoplamento dos nativos é coerente com o norte do projeto
(o que tornou os elementos extensíveis foi feito justamente para um dia o núcleo não
precisar conhecê-los um a um) — é um objetivo real, não ruído do baseline. Nomeá-lo
como marco pós-F-6 dá dono ao `target=0` sem comprometer esta branch, e deixa a fila
F terminar limpa. A saída 1 fecha a porta cedo demais (aceita `≠ 0` como permanente
quando a fronteira E1 já abriu o caminho para `=0`); a saída 3 só adia. Mas é decisão
sua — a 1 é defensável se você considera o desacoplamento custo alto demais para o
ganho, e a 3 é defensável se você quer fechar o F antes de pensar nisso.

---

## Fronteira de parada — a escolha do dono
Apresentar as três saídas (este texto) e **parar**. Não editar nenhum documento até o
dono escolher **uma**. Após a escolha — na continuação — materializar **só** o
registro da saída escolhida (a edição de doc correspondente), confirmar lint 0/0, e
fechar o item (remover o "item aberto carregado" dos relatórios futuros, ou trocá-lo
pela referência ao marco/à reconciliação).

---

## Verificação (gates)
```
content-preserving: zero .rs/.toml. Suíte 2723 / 3242 intacta (não re-rodada — nada
  de código). Árvore de produto não tocada.
lint: crystalline-lint . = 0/0.
fronteira: nada editado antes da escolha do dono.
escopo: materializar SÓ a saída escolhida — não as três.
```

---

## O que NÃO fazer
- **Não tocar código** — é planejamento/doc.
- **Não materializar mais de uma saída** — o dono escolhe uma.
- **Não decidir pelo dono** — apresentar e parar; a recomendação é marcada como do
  agente, não como a escolha.
- **Não confundir com o corte em si** — este passo **não** executa o desacoplamento
  (isso seria o marco G, se a saída 2 for escolhida); só registra o destino do item.

---

## Relatório (`typst-passo-346-relatorio.md`)
- A saída escolhida pelo dono e o registro materializado (o diff do doc).
- Confirmação de que o item `content→elements→0` foi fechado/redirecionado (não volta
  como "item aberto carregado"; vira referência ao marco ou à reconciliação, ou — na
  saída 3 — a nota de lacuna com a decisão 1-vs-2 ainda pendente).
- **Mapa de filtro (campo):** o lugar lógico — "o destino do corte content→elemento é
  uma decisão de fundação da fila F (o que a fila entrega vs o que ela habilita), que
  na execução real ficou órfã do P339 ao P346 porque o baseline media um alvo que a
  fila nunca prometeu" — com o rastro (baseline P338 mede target=0; carregado sem dono
  P339–P345; decidido aqui).
```
