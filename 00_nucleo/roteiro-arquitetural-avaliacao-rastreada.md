# Roteiro arquitetural — avaliação rastreada (o que "comemo/TrackedWorld" deve ser no cristalino)

Pedido: desenhar a infraestrutura que DEBT-2 (e os consumidores de introspecção)
diz precisar de `comemo`/`TrackedWorld`, já que o `TrackedWorld` não existe.

Este documento é um roteiro arquitetural, não uma spec. Ele fixa a forma e a
ordem; cada passo concreto é escrito depois, com a sonda própria (ADR-0114).

---

## 0. Correção da premissa (ler antes de tudo)

Três factos do material mudam o enquadramento. Confirmá-los é a primeira sonda.

1. **`comemo` já existe.** `typst-core` depende de `comemo 0.4` (DEBT.md:1492).
   Não é uma crate a adicionar.
2. **`comemo` já é usado.** Os guards de profundidade são funções livres
   `world_types::check_*(Tracked<Route>)`, não métodos (DEBT-44/45, P92/P93). O
   projeto já sabe consumir tipos `Tracked` dentro da forma B (ADR-0109): um
   tipo `Tracked` é consumido por função livre, porque `Tracked<T>` só expõe os
   métodos do tipo rastreado.
3. **O oráculo de introspecção já está a nascer.** Em pedaços: fixpoint da TOC
   dentro de `layout()` em L1 (DEBT-17, P65, com `known_page_numbers`);
   `BibStore` com `style_for_key` por chave determinística (P429);
   `SealedPositions`/`position_of` (P205C); `CounterRegistry`/`counter_values_at`
   (P451); `TagIntrospector`; `query_by_kind`/`query`
   (`introspector.rs:46/132`, P388).

Portanto, o que falta **não** é construir um `comemo`. O nome "comemo/TrackedWorld"
junta duas necessidades distintas que têm de ser separadas:

- **Correção** — avaliação em 2 passagens com um oráculo de introspecção que vê
  o estado final do documento. É disto que DEBT-2, introspecção real (ADR-0066)
  e `measure()` precisam. O substrato já existe, espalhado.
- **Performance** — memoização incremental (recompilar só o que mudou). É o outro
  papel do `comemo`. O projeto compila documentos inteiros; esta necessidade é
  provável que seja prematura, e é uma decisão de benchmark (P441), não de agora.

O roteiro entrega a correção agora e difere a performance.

---

## 1. Fundação — as decisões da refatoração em que isto assenta

| Decisão | O que impõe a este roteiro |
|---|---|
| ADR-0108 medir antes de decidir; ADR-0114/0117 sonda antes da spec | A arquitetura abaixo é provisória até a Fase 0 confirmar o substrato. Nenhuma fase começa sem a sua sonda. |
| ADR-0107 paridade é de saída; mecânica livre | O mecanismo de rastreio não precisa de espelhar o `comemo`. Só a saída renderizada tem de bater com o vanilla. Isto liberta o desenho. |
| Tekt — separar substratos e fasear pelo mais fraco (P388) | Fasear pelos consumidores, do mais fraco (introspecção, ADR-0066) ao mais forte. |
| ADR-0017 — abrir o portão do tipo, consumidores fluem (P395) | Consolidar o oráculo primeiro (substrato); só depois ligar consumidores. |
| ADR-0029 L1 puro; fronteira com seam explícito (P450) | O oráculo e o rastreio são computação pura (L1). O World concreto é L3. O seam é declarado, não implícito. |
| ADR-0109 forma B; sem `dyn`/vtable no código próprio (precedente DEBT-44/45) | Tipos `Tracked` consumidos por funções livres. Sem trait objects no código do projeto. |
| Reaproveitar em vez de reescrever (P429/P451/P454) | Generalizar `BibStore`/`SealedPositions`/`CounterRegistry` num oráculo único, não criar um paralelo. |
| ADR-0085 diagnóstico imutável | A saída de cada sonda é um ficheiro imutável em `00_nucleo/diagnosticos/`. |

---

## 2. Fase 0 — Sonda de viabilidade (read-only, zero código)

Estabelece o estado real e decide A vs B (Fase 1). Produz um diagnóstico
imutável. Perguntas factuais, cada uma com `file:line`:

1. Versão e uso atual de `comemo`: que tipos `Tracked` são usados hoje, e onde
   (`Route` confirmado; mais algum?). `grep -rn "Tracked\|comemo\|memoize" 01_core/src/`.
2. Forma do trait `World`: que métodos expõe, qual a fronteira L1/L3, se já é
   passado a `layout_with_introspector` (`mod.rs:1484`).
3. Mapa do oráculo disperso: `BibStore`, `SealedPositions`, `CounterRegistry`,
   `known_page_numbers`, `TagIntrospector`, `query_by_kind`. Quais campos, quais
   chaves, quem sela e quando.
4. Sítio da captura de closures (DEBT-2): `ClosureRepr::captured` (`Arc<Scope>`),
   `apply_closure`, `Scopes::with_parent`.
5. Estado de `measure()`: é stub transparente hoje? Onde.
6. **Teste de paridade da premissa de DEBT-2.** Correr em `lab/parity`
   (vanilla, DEBT-53/54) o caso `#let x=1; #let f()=x; #let x=2; #f()` e
   registar o que o vanilla devolve de facto. O DEBT.md afirma que o vanilla
   devolve `2` (lazy); isto **não está verificado**. Se o vanilla devolver `1`,
   a divergência de DEBT-2 não existe e a Fase 4c desaparece.

**Saída:** `typst-sonda-avaliacao-rastreada-passo-NNN.md`, com o mapa do oráculo
disperso, o resultado da paridade de DEBT-2, e a recomendação A vs B.

> Razão de a paridade entrar na sonda: este roteiro existe para destravar DEBT-2,
> mas a premissa de DEBT-2 nunca foi medida contra o vanilla. Construir
> infraestrutura sobre uma premissa não verificada é exatamente a deriva que a
> ADR-0114 persegue. A sonda mede primeiro.

---

## 3. Fase 1 — ADR de decisão (correção vs performance; nativo vs comemo)

Escrever a ADR que regista duas decisões, com base na Fase 0:

**Decisão 1 — separar correção de performance.** A correção (oráculo 2-pass) é
materializada agora. A memoização incremental fica diferida e só entra por
benchmark (P441), nunca por suposição. Registar como dois caminhos distintos.

**Decisão 2 — substrato do oráculo: nativo vs `comemo`.**

| Opção | Descrição | Recomendação |
|---|---|---|
| A — Oráculo nativo selado | Generalizar `BibStore`/`SealedPositions`/`CounterRegistry`/`known_page_numbers` num oráculo único, imutável após selagem, consultado por chave. Fixpoint já em `layout()` (DEBT-17). | **Recomendada para a correção.** Estende o que já existe; zero dependência nova; controlo total (padrão P450/P429). |
| B — `TrackedWorld` via `comemo` | Embrulhar o World em `Tracked`, memoizar a avaliação. | Diferida para a Fase 5 (performance). Não é necessária para a correção. |

A recomendação A vale porque o oráculo já está 80% construído em pedaços; o
trabalho é consolidá-lo, não inventá-lo. O `comemo` (Opção B) resolve um problema
de performance que o projeto ainda não tem.

---

## 4. Fase 2 — Consolidar o oráculo selado de introspecção (o portão)

É o passo-substrato, no padrão ADR-0017: abrir o portão, consumidores fluem
depois. Pura computação L1.

Unificar os fragmentos existentes num único `Introspector` selado:

- **Entradas seladas por chave determinística** — o padrão que P429 já provou com
  `BibStore::style_for_key`. Generalizar a chave (`elem.key()`) para qualquer
  elemento que precise de transportar resultado computado de eval para layout,
  sem poluir o struct de domínio (lição de DEBT-63: tabela lateral, não campo).
- **Posições** (`SealedPositions`/`position_of`), **contadores**
  (`CounterRegistry`/`counter_values_at`), **páginas** (`known_page_numbers`) e
  **query por tipo/ordem** (`query_by_kind`) entram como vistas do mesmo oráculo.
- **Selagem explícita:** o oráculo é mutável durante a coleta (1ª passagem) e
  imutável depois (2ª passagem). Quem lê na 2ª passagem nunca muta. Isto torna o
  2-pass uma invariante de tipo, não uma convenção.

Critério de fecho do portão: um `Introspector` selado, consultável por chave,
com as quatro vistas (posição, contador, página, query) sobre a mesma estrutura,
e o fixpoint de `layout()` (DEBT-17) a alimentá-lo. Zero `dyn` no código próprio;
tipos `Tracked` (se entrarem) consumidos por função livre (forma B, DEBT-44/45).

---

## 5. Fase 3 — Fronteira: a entrada do oráculo

Decidir de onde o oráculo é alimentado, com seam L1/L3 explícito (padrão P450).

- O oráculo e o rastreio são puros → L1.
- O World concreto (I/O, leitura de ficheiros) → L3 (`SystemWorld`,
  `World::read_bytes`, P450).
- O seam é o `Introspector` selado: L3 corre a coleta, sela, e passa o oráculo
  imutável a `layout_with_introspector` (`mod.rs:1484`). L1 lê do oráculo, nunca
  do World diretamente para introspecção.

Se a Fase 5 (performance) entrar mais tarde, é aqui que um `Tracked<World>`
encaixa, sem mudar os consumidores — eles falam com o oráculo, não com o World.

---

## 6. Fase 4 — Consumidores, faseados pelo mais fraco

Cada um é um passo próprio, com sonda própria. Ordem pelo substrato mais fraco
primeiro (ADR-0066 é o módulo mais fraco, P388).

### 4a — Introspecção real (ADR-0066)
A 2ª passagem vê o documento final: `query` ordenada, `position_of` estável,
back-references. Estende o oráculo da Fase 2. Desbloqueia a bibliografia Fase 2
e o `label`/`ref` do roteiro de conclusão.

### 4b — `measure()` genuíno (ADR-0066)
Layout de um fragmento para obter o tamanho, usado de volta no documento. Usa a
1ª passagem do oráculo para o tamanho medido. Hoje é stub transparente.

### 4c — DEBT-2: captura de closures — **só se a Fase 0 confirmar a divergência**
Se a paridade da Fase 0 mostrar que o vanilla devolve `2`, então a captura passa
a resolver através do scope rastreado pelo oráculo, em vez do snapshot
`Arc<Scope>` eager (DEBT-2). Se a paridade mostrar `1`, **este passo não existe**
— o cristalino já bate com o vanilla, e DEBT-2 fecha como "divergência inexistente,
premissa corrigida". Não construir 4c antes de saber qual dos dois é.

### Nota — o que já consome o oráculo e não precisa de infra nova
A bibliografia Fase 2 (estilos numéricos, ordem, back-refs) e a numeração
(heading/figure/equation) já consomem `query_by_kind`/`CounterRegistry`. Com o
oráculo consolidado (Fase 2), são materialização, não infraestrutura nova.

---

## 7. Fase 5 — Memoização incremental (diferida; só por benchmark)

`TrackedWorld` via `comemo` para recompilação incremental. Critérios para sequer
considerar:

- Existe um caso de uso de edição incremental (LSP, watch-mode) que o justifique?
  Hoje o projeto compila documentos inteiros.
- O benchmark (infra P441) mostra que a avaliação completa é um gargalo real?

Se nenhum dos dois, esta fase não se faz. A decisão é de número medido (padrão
P443), não de "o vanilla usa comemo, logo nós também". A paridade é de saída
(ADR-0107); o vanilla usar comemo é mecânica, não contrato.

---

## 8. Regras transversais (gates em cada passo)

- Sonda A.0 real antes de cada spec; evidência `file:line` + commit (ADR-0114/0117).
- Verificar a fronteira/ADR vigente antes de propor estrutura (ADR-0117 Cláusula 4).
  Em especial: não pôr cache/estado computado em struct de domínio — tabela
  lateral selada (lição DEBT-63/P429).
- Tipos `Tracked` consumidos por função livre, sem `dyn` no código próprio
  (forma B, precedente DEBT-44/45).
- L1 puro, zero I/O; World concreto em L3; seam no oráculo (P450).
- Paridade verificada contra `lab/parity` onde a saída muda (ADR-0033/0107).
- Cada sonda produz diagnóstico imutável (ADR-0085).

---

## 9. O que pode sair errado

- **A premissa de DEBT-2 estar errada.** É o risco central e por isso a paridade
  está na Fase 0. Se o vanilla também for eager, metade do motivo deste roteiro
  desaparece — e isso é um ganho (menos trabalho), não um problema.
- **Conflito de versão `comemo` 0.4 vs 0.5** (DEBT.md:1482-1499) — só relevante
  se a Fase 5 acontecer; as duas versões coexistem sem trocar tipos `Tracked`.
- **Selagem mal definida.** Se a fronteira mutável/imutável do oráculo não for de
  tipo, o 2-pass volta a ser convenção e regride para os bugs de fixpoint que
  DEBT-17 já resolveu. A selagem tem de ser uma invariante verificável.
- **Tentação de fazer a Fase 5 junto.** A memoização é o pedaço mais complexo e o
  menos necessário. Mantê-la diferida é a decisão certa até haver número.
