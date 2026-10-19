# Passo 385 — Diagnóstico: decomposição do só-vanilla da lente e proposta de marcador de correspondência

**Tipo**: Diagnóstico (não materializa código de produção L1–L4)
**Data**: 2026-06-21
**Padrão**: diagnóstico-primeiro (precedentes 131A / 132A / 140A / 148 / 154A / 159 / 160)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0034 (diagnóstico obrigatório / estrutura canónica), ADR-0054 (perfil observacional graded / scope-out), ADR-0065 (inventariar primeiro), ADR-0075 (vanilla integration)
**Entrada externa**: relatório "só-vanilla" da **lente** (projeto `tekt-cargo-dsm`) corrido contra `lab/typst-original/`

> **Nota de numeração**: o número de ADR proposto (§"O que produzir", item 3) fica como `ADR-NNNN` deliberadamente. Antes de criar, confirmar o próximo número livre no README dos ADRs — precedente P160A (conflito 0017/0066) mostrou que assumir número é erro.

---

## 1. Contexto

A lente (DSM) corrida contra o vanilla vendorizado em `lab/typst-original/` produz um conjunto "só-vanilla" — símbolos que existem no vanilla e a lente não conseguiu parear no cristalino. Esse conjunto é tratado, no relatório da lente, como aproximação de "o que falta migrar". O número agregado reportado foi ~10745.

Esse número **mistura três coisas diferentes**, e tratá-las como uma só infla a leitura de dívida:

1. **Renomeado-com-registro** — símbolo vanilla que existe no cristalino sob outro nome (o renome dominante é `*Elem` → `Content::*`, por ADR-0026). A correspondência **já está escrita** na construção: na seção "Sobre paridade" de cada prompt L0 e nas entradas do Inventário 148. A lente não a lê porque pareia por nome; o pareamento existe, só não está num campo que a máquina consome.
2. **Fora-de-escopo-com-ADR** — itens que o cristalino não persegue de propósito (perfil graded de ADR-0054; backends de render/svg/html; atributos scope-out por feature). Não são dívida; são exclusão declarada.
3. **Resíduo genuíno** — equivalências semânticas sem âncora escrita: uma feature vanilla partida em duas cristalinas, ou fundida, sem entrada "Sobre paridade" nem ADR de divergência. É o único subconjunto onde falta migrar de verdade, ou onde a correspondência ainda não foi declarada.

Este passo **mede** esse corte (hoje é estimativa) e **propõe** a convenção que torna o pareamento legível por máquina no futuro. Não materializa a convenção nem mexe em código de produção.

Princípio que rege o método deste passo, literal: a medição é um **script determinístico**, não um julgamento de LLM. Pôr o LLM dentro do loop de contagem torna o número não-reprodutível, que é o oposto do objetivo. O LLM entra só onde a forma é irregular (propor marcador) e onde o resíduo exige julgamento — sempre como proposta a confirmar, nunca como autoridade da contagem.

---

## 2. Objectivo

Produzir três coisas:

- A **decomposição factual** do conjunto só-vanilla nos três baldes acima, com contagens.
- A **lista explícita do resíduo genuíno** (balde 3), que é o único insumo legítimo para trabalho de migração futuro.
- Uma **ADR PROPOSTA** que fixe o marcador de correspondência legível por máquina (`@vanilla`), com roadmap de materialização em passos dedicados posteriores.

Tudo isto é diagnóstico. Zero código L1–L4 tocado.

---

## 3. Entradas

- Relatório só-vanilla da lente (lista de símbolos vanilla não-pareados). Produzido externamente em `tekt-cargo-dsm`; consumir como ficheiro de entrada (sugestão: colocar em `00_nucleo/diagnosticos/entrada-lente-so-vanilla.<data>.txt`).
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` (Inventário 148) — autoridade de cobertura por feature.
- Todos os prompts L0 em `00_nucleo/prompts/**` — especificamente as seções `## Sobre paridade`.
- ADRs de divergência e scope-out: ADR-0026 (Content enum fechado), ADR-0033, ADR-0054, e quaisquer ADRs de scope-out por feature.
- `lab/typst-original/` — o vanilla vendorizado, para resolver caminhos de símbolo quando o registro nomeia um caminho.

---

## 4. Método

### 4.1. Medição (script determinístico — não LLM)

Escrever um script (Python + ripgrep) em `lab/` (arena; não é produção). O script:

1. Lê a lista só-vanilla da lente.
2. Varre todos os prompts L0 e extrai, das seções `## Sobre paridade`, os símbolos/caminhos vanilla nomeados. Produz o conjunto **pareado-por-literatura**.
3. Varre o Inventário 148 e extrai as entradas com referência vanilla (coluna/menção de "Vanilla path" ou equivalente). Une ao conjunto pareado-por-literatura.
4. Varre os ADRs de scope-out e extrai os itens/atributos declarados fora de escopo. Produz o conjunto **scope-out-declarado**.
5. Classifica cada item da lista só-vanilla:
   - se bate com pareado-por-literatura → balde 1 (renomeado-com-registro);
   - senão, se bate com scope-out-declarado → balde 2 (fora-de-escopo-com-ADR);
   - senão → balde 3 (resíduo genuíno).
6. Emite contagens por balde + a lista completa do balde 3.

O script é a medição. Rodar de novo tem de dar o mesmo número (critério de aceitação 6.2). Ele é também o esqueleto do futuro check de linter (passo posterior), por isso escrevê-lo de forma limpa e testável.

### 4.2. Exploração de apoio (Haiku)

Para acelerar a varredura e listar quais L0 têm/ não têm a seção "Sobre paridade" e em que forma (símbolo, caminho, lista, prosa), usar um subagente de busca read-only em Haiku (ou o Explore embutido). Saída: inventário de cobertura e forma da literatura. **Esta saída é candidata, não autoridade** — a contagem que vale sai do script de 4.1.

### 4.3. Julgamento do resíduo (Sonnet ou humano)

Os itens do balde 3 que forem casos não-1:1 (feature partida/fundida) exigem julgamento. Encaminhar para Sonnet ou para revisão humana. Haiku não decide aqui. Para cada item do balde 3, classificar: (a) correspondência existe mas não-1:1 e não-registrada → candidato a marcador novo; (b) falta migrar de verdade → insumo de roadmap de feature; (c) scope-out não-declarado → candidato a ADR de scope-out.

### 4.4. Proposta de marcador

Com base no que 4.2 revelar sobre a irregularidade de forma, especificar o marcador na ADR proposta (§3 dos deliverables). Forma mínima a fixar:

- `@vanilla <caminho::Símbolo>` — correspondência 1:1.
- `@vanilla-agrega [<A>, <B>, ...]` — um construto cristalino agrega N vanilla.
- `@vanilla-nenhum` — sem equivalente vanilla (construto cristalino próprio).
- `@scope-out <ADR>` — vanilla deliberadamente não perseguido.

O marcador fica ao lado da prosa "Sobre paridade", não a substitui: a prosa é para o humano, o marcador é o que a ferramenta lê.

---

## 5. O que produzir

1. **Diagnóstico** em `00_nucleo/diagnosticos/typst-decomposicao-so-vanilla-passo-385.md`, com: contagens por balde; tabela de cobertura/forma da literatura "Sobre paridade" por domínio (entities/model/layout/math/introspect); lista completa do resíduo (balde 3) classificada per 4.3.
2. **Script de medição** em `lab/` (sugestão `lab/parity/tools/decompor_so_vanilla.py` + um pequeno teste). Determinístico, re-rodável.
3. **ADR PROPOSTA** `ADR-NNNN` (número a confirmar) fixando o marcador de correspondência `@vanilla` (formas de 4.4), com **roadmap de materialização** em sub-passos dedicados:
   - Passo de semeadura: preencher o marcador nos L0 a partir das seções "Sobre paridade" existentes (Haiku propõe, humano confirma).
   - Passo de regra: tornar o marcador obrigatório no fecho de qualquer ficheiro L1–L4 (análogo ao cabeçalho de linhagem `@prompt`).
   - Passo de linter: adicionar check ao `crystalline-lint` (nova regra V) que falha sem marcador.
   - Passo de ferramenta: só depois disto, a lente (ou outra ferramenta) consome o campo direto.

---

## 6. O que NÃO fazer (scope-out do passo)

- **Não** adicionar marcadores a todos os ficheiros. Isso é o passo de semeadura, posterior.
- **Não** escrever o check de linter. Posterior.
- **Não** tocar em código de produção L1–L4. Este passo é diagnóstico.
- **Não** pôr LLM no loop de medição. A contagem é do script.
- **Não** assumir o número "~800 backends" do material anterior: ele não foi confirmado contra os arquivos. Medir o scope-out de fato (balde 2), não herdar a estimativa.
- **Não** abrir reservas novas fora da ADR proposta (política "sem novas reservas").

---

## 7. Critérios de aceitação

1. As contagens dos três baldes somam o total da lista só-vanilla de entrada (reconciliação fechada; qualquer item não classificado é erro a resolver, não a ignorar).
2. O script é reprodutível: duas corridas sobre a mesma entrada dão contagens idênticas.
3. A lista do resíduo (balde 3) está explícita e classificada (a/b/c per 4.3).
4. A ADR de marcador está PROPOSTA, com as quatro formas e o roadmap de sub-passos.
5. Zero ficheiros de produção L1–L4 alterados (confirmável por diff vazio fora de `lab/` e `00_nucleo/diagnosticos/` e `00_nucleo/adr/`).
6. O número "~800 fora-de-escopo" do material anterior é confirmado ou corrigido com a contagem real do balde 2.

---

## 8. O que pode sair errado

- **Lista da lente desatualizada vs. repo** (sem push, o cristalino local está à frente). O script pode marcar como resíduo coisas já construídas depois da corrida da lente. Mitigação: registrar a data/commit da corrida da lente no cabeçalho do diagnóstico; re-correr a lente se a defasagem for grande.
- **Forma irregular quebra o extrator ingénuo.** As seções "Sobre paridade" misturam símbolo, caminho, lista e prosa. O script de 4.1 deve falhar de forma visível (listar o que não conseguiu extrair), não silenciosamente. O não-extraível vira insumo da exploração 4.2 e da proposta de marcador.
- **Casos não-1:1 mal classificados.** Um agregado pode parecer resíduo se o extrator só procura 1:1. Por isso o balde 3 passa por julgamento (4.3) antes de virar "falta migrar".
- **Tentação de ajustar o número.** Se o balde 3 vier maior que o esperado, a resposta é registrar e investigar, não recalibrar o script para encolher o número. Uma previsão que falha é dado.

---

## 9. Referências

- ADR-0026 — Content enum fechado (`*Elem` → `Content::*`, o renome dominante).
- ADR-0033 — paridade vanilla (âncora universal nas seções "Sobre paridade").
- ADR-0034 — diagnóstico obrigatório / estrutura canónica.
- ADR-0054 — perfil observacional graded / scope-out.
- ADR-0065 — inventariar primeiro.
- ADR-0075 — vanilla integration (lente contra `lab/typst-original/`).
- Inventário 148 — `typst-cobertura-vanilla-vs-cristalino.md`.
- Precedentes diagnóstico-primeiro: P148, P154A, P159, P160.

---

## 10. Nota sobre o Tekt

A convenção `@vanilla` é descoberta aqui, na typst-crystalline, mas não é específica dela: é uma propriedade do método orientado por prompt — o registro de construção como **oráculo de correspondência** (o "terceiro oráculo", distinto do sistema-fonte e do julgamento humano). Se a ADR proposta se confirmar útil na semeadura, é candidata a **lição v1.4 do Tekt**, promovida a partir desta bancada per o padrão "lab como evidência viva". Registrar essa possibilidade no diagnóstico, sem materializá-la no Tekt neste passo.
