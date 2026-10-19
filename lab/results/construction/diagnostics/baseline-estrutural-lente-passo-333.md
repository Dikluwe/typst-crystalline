# Baseline Estrutural — Lente `tekt-cargo-dsm` (Passo 333, Parte 5)

> Medição estrutural por computação (DSM / grafo de dependências resolvido),
> não por narrativa. É o análogo estrutural de um baseline de performance:
> mede **atomização** (independência dos 65 módulos de elemento) e
> **separação de camadas** (acoplamento do hub `content`) por número, não por
> argumento.
>
> **Natureza desta parte:** medição de leitura. Zero código de produto alterado;
> zero fix à lente. Apenas leitura do produto + escrita deste diagnóstico.

---

## 1. A lente — versão, build, comandos exatos

### Identidade da lente (registrada)

| Item | Valor |
|------|-------|
| Repo da lente | `/home/dikluwe/Documentos/Antigravity/tekt-cargo-dsm` |
| Commit | `98d8f9e` — `feat: --comparar a nível de item — trilha 0075→0078 (workspace, third-party, chave K4)` |
| Branch | `main` |
| `git describe --tags` | *(sem tags — "fatal: No names found")* |
| Working tree | limpo (sem alterações) |
| Binário | `target/release/lente` (crate `lente_app`) |
| Fonte do grafo | fork `cargo-modules` `0.27.0`, commit `ddcd3ca` (`feat(export-json): posição no fonte`), instalado em `~/.cargo/bin/cargo-modules` |

### Produto medido

| Item | Valor |
|------|-------|
| Repo | `/home/dikluwe/Documentos/Antigravity/typst-crystalline` |
| Commit HEAD | `f83f78a13` — `Passo 332 — relatório` |
| Crate analisado | `typst-core` (L1, `01_core/`) |

### Build da lente (no repo da lente)

```bash
cd /home/dikluwe/Documentos/Antigravity/tekt-cargo-dsm
cargo build --release -p lente_app
# Finished `release` profile — EXIT 0 (artefatos ficam no repo da lente, não poluem o produto)
```

### Comandos de medição (executados de dentro de `01_core/`)

```bash
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline/01_core
LENTE=/home/dikluwe/Documentos/Antigravity/tekt-cargo-dsm/target/release/lente

# (A) Estrutura — acoplamento de tipo genuíno (so-referencia), só o nosso código
RUST_MIN_STACK=33554432 $LENTE --pacote typst-core --estrutura --so-referencia --filtrar-stdlib
RUST_MIN_STACK=33554432 $LENTE --pacote typst-core --estrutura --so-referencia --filtrar-stdlib --text

# (B) Estrutura — TODAS as uses (inclui imports de nível de módulo), p/ comparar real vs inflado
RUST_MIN_STACK=33554432 $LENTE --pacote typst-core --estrutura --filtrar-stdlib

# (C) Ranking de impacto
RUST_MIN_STACK=33554432 $LENTE --pacote typst-core --ranking --top 15 --filtrar-stdlib --text

# (D) DSM em HTML autocontido
RUST_MIN_STACK=33554432 $LENTE --pacote typst-core --estrutura --so-referencia --filtrar-stdlib --html --saida /tmp/typst-dsm.html

# (E) Modo --diff (mapeia diff git → nós tocados) — testado, opera na raiz do repo
RUST_MIN_STACK=33554432 $LENTE --diff --repo /home/dikluwe/Documentos/Antigravity/typst-crystalline --vista resumo --text
```

Observações de execução:
- O default de saída é **JSON**; `--text` comuta para texto. Não existe flag `--json`.
- Sem `--pacote` a lente exige `--grafo` ou `--pacote` (erro: *"Informe --grafo ou --pacote"*) — ver R1.
- Todas as execuções terminaram com EXIT 0. `RUST_MIN_STACK=33554432` foi aplicado por precaução; não houve overflow.

---

## 2. Os números estruturais (o que faltava no balanço de P332)

### 2.1 Métricas globais do crate `typst-core`

| Métrica | so-referencia (tipo real) | todas-as-uses (inflado) |
|---------|---------------------------|--------------------------|
| Módulos | **217** | 217 |
| Arestas módulo→módulo | **667** | 970 |
| Ciclos (SCC não-triviais) | **3** | 5 |
| Tamanhos dos ciclos | **[81, 4, 8]** | [83, 4, 8, 5, 7] |

O recorte `--so-referencia` (acoplamento de tipo genuíno, descartando `use` de
topo de módulo) é o número honesto: 667 arestas, 3 ciclos. As 303 arestas extras
do modo "todas-as-uses" são acoplamento aparente vindo de imports declarativos.

### 2.2 Hub `entities/content.rs` (o enum central, ~4960 linhas)

Medido como o módulo `typst_core::entities::content` (a lente é module-level: o
ficheiro inteiro é **um nó** — ver R3).

| Métrica do hub | Valor (so-referencia) |
|----------------|------------------------|
| Fan-in (módulos que dependem DE content) | **85** |
| Fan-out (módulos de que content depende) | **82** |

Decomposição do fan-out (82):
- **65 → módulos de elemento** (`content` importa cada `*Elem` concreto)
- 17 → não-elementos: `bib_entry, citation_form, color, counter_update, dir, func, geometry, label, layout_types, math_style, parity, ptr_eq_arc, sides, source_result, state_update, style, value`

Decomposição do fan-in (85):
- **65 ← módulos de elemento** (cada elemento importa `Content`)
- 20 ← não-elementos (ex.: `element_payload`, `introspector`, `module`,
  `page_store`, `value`, e vários submódulos de `rules::eval / introspect /
  layout / math / stdlib`).

**Leitura:** `content` é simultaneamente o maior emissor e o maior receptor do
crate. O acoplamento com os elementos é **bidirecional e total**: content→elem
(65) e elem→content (65). Essa reciprocidade é a assinatura estrutural do
**modelo de enum fechado (D)**: o núcleo conhece cada elemento concreto e cada
elemento conhece o núcleo.

### 2.3 Independência dos 65 módulos de elemento (`entities/elements/*`)

| Métrica | Valor |
|---------|-------|
| Módulos de elemento medidos | **65** (de 66 ficheiros; `mod.rs` não é nó de elemento) |
| Arestas **elemento → elemento** | **0** |
| Ciclos *entre* elementos | **0** |
| Fan-out médio de um elemento | **3.29** módulos distintos |
| Maior fan-out de elemento | `block`, `boxed` = 7 deps |

**Atomização horizontal: confirmada por cálculo.** Nenhum dos 65 elementos
referencia outro elemento — zero arestas mútuas, zero ciclos entre eles. Cada
elemento é uma folha lateral que aponta para o núcleo e tipos compartilhados,
nunca para um par. Este é exatamente o número que o balanço narrativo de P332
afirmava sem medir.

### 2.4 Ciclos — o achado central

A lente reporta **3 ciclos (SCCs)** em modo so-referencia. Tamanhos: 81, 4, 8.

| Ciclo | Tamanho | content dentro? | nº de elementos dentro |
|-------|---------|-----------------|------------------------|
| 0 | **81** | **sim** | **65** (todos) |
| 1 | 4 | não | 0 — é `ast::{code, expr, markup, math}` |
| 2 | 8 | não | 0 — é `geometry, gradient, layout_types, paint, position, sealed_positions, style, style_chain` |

**O paradoxo aparente, resolvido:** os 65 elementos são mutuamente independentes
(§2.3, 0 arestas entre si) **mas os 65 estão todos dentro do ciclo de 81 módulos**
(ciclo 0). O ciclo não passa elemento→elemento; passa **elemento → content →
elemento**. É `content` quem fecha o anel: ele importa cada `*Elem`, e cada
`*Elem` importa `Content`. Os 16 não-elementos do ciclo 0 são:
`args, content, element_payload, engine, func, introspector, metadata_store,
module, page_store, scope, show, state_registry, state_update, value,
rules::eval, rules::scopes`.

**Consequência para o F:** o ciclo de 81 não é causado pela falta de atomização
dos elementos (que já é perfeita) — é causado pela **direção content→elemento**.
Se o F remove os 65 imports `use elements::*Elem` de `content.rs` (núcleo deixa
de conhecer o elemento concreto, conhecendo só o trait `Element` + tipos
públicos), o fan-out de content cai de 82 → 17, os elementos deixam de ser
alcançáveis de volta a partir de content, e o **ciclo de 81 colapsa**. Os ciclos
1 (ast, 4) e 2 (geometria/estilo, 8) são ortogonais ao F e permanecem.

### 2.5 Ranking de impacto (top — `--ranking`)

Os nós de maior alcance transitivo NÃO são os elementos nem `content`, e sim os
tipos sintáticos de base:

| # | Impacto | Classe | Path |
|---|---------|--------|------|
| 1 | 2245 | Base | `entities::span::Span` |
| 2 | 2190 | Base | `entities::syntax_text::SyntaxText` |
| 3 | 2171 | Base | `entities::syntax_kind::SyntaxKind` |
| 4–9 | ~2116–2166 | Intermediário | nós de `syntax_node::*` |
| 10 | 1376 | Base | `entities::color::Color` |
| 11–13 | ~1322–1340 | Base/Interm | `layout_types::{Pt, Abs, Length}` |
| 14 | 1316 | Base | `ecow::string::EcoString` |

(`content` não entra no top-15 de impacto por item — o ranking é por símbolo, não
por módulo; o peso de content está distribuído pelos 65 `*Elem` que ele expõe.)

### 2.6 Artefato visual

DSM HTML autocontido gerado em `/tmp/typst-dsm.html` (fora do produto, descartável).

---

## 3. Requisitos numerados R* (limites da lente → realimentação futura)

> Material para uma futura sessão de `tekt-cargo-dsm`. **Não corrigir a lente
> aqui.** Apenas registar. Cada R é uma lacuna entre o que este corpus precisa e
> o que a lente entrega hoje.

- **R0 — (não disparou).** A lente **buildou e rodou** neste ambiente. Baseline
  não está pendente. R0 fica registado como "sem ocorrência".

- **R1 — Análise multi-crate / workspace-wide ausente.** A lente exige um
  `--pacote <nome>` único (ou `--grafo`); sem ele aborta com *"Informe --grafo ou
  --pacote"*. A topologia Tekt do produto tem 5 camadas em crates separados
  (`typst-core`, `02_shell`, `03_infra`, `04_wiring`); a lente mede uma de cada
  vez. **Não há, hoje, uma vista DSM que cruze L1↔L2↔L3↔L4** para verificar a
  topologia de imports inter-camada por cálculo. O `crystalline-lint` faz isso por
  regra; a lente faria por grafo. Requisito: modo workspace que una os crates num
  só grafo rotulado por camada.

- **R2 — Sem métrica de instabilidade / camadas explícita.** A lente entrega
  fan-in, fan-out, ranking de impacto, SCCs e ordem topológica (DSM). **Não**
  computa I = fan-out / (fan-in+fan-out) (instabilidade de Martin) nem detecta
  "violação de camada" (aresta que sobe na hierarquia) como métrica de primeira
  classe. Para o F, a métrica desejada — "núcleo não conhece o elemento" — é
  derivável manualmente do fan-out de content (§2.2), mas não é um número que a
  lente nomeie. Requisito: métrica de instabilidade + flag de aresta-que-viola-
  ordem-DSM.

- **R3 — Granularidade module-level (não intra-módulo).** `content.rs` (~4960
  linhas) é **um único nó**. A lente não decompõe o ficheiro em itens
  (variantes do enum, blocos `impl`, funções de dispatch). O ranking por símbolo
  (§2.5) existe, mas a vista de estrutura/ciclos é por módulo. Para o F, seria
  útil ver *quais variantes/handlers* dentro de content acoplam a cada elemento
  (a granularidade que o lock de ADR-0105 precisa — ver §4). Requisito: DSM
  intra-módulo (item-level) opcional para um nó-hub.

- **R4 — Sem distinção entre import-de-trait e import-de-tipo-concreto.** O F
  promete: "o elemento importa só o trait + tipos públicos; o núcleo não conhece
  o concreto". A lente conta a aresta `content → elements::heading`, mas não
  classifica se essa aresta é por **trait/tipo público** (legítima sob F) ou por
  **struct concreta `HeadingElem`** (a aresta que o F elimina). Hoje a separação
  é feita por leitura manual do `use` (§ confirmado: `content.rs` faz
  `use ...elements::heading::HeadingElem`). Requisito: rotular cada aresta `Uses`
  por *kind* do alvo (trait vs struct vs fn), para que o critério do F seja um
  filtro de aresta e não uma inspeção de texto.

- **R5 — `--comparar` testado só de leitura; não validado para o ciclo F
  antes/depois.** A lente tem `--comparar --antes/--depois` (estrutura de duas
  raízes) e `--diff` (diff git → nós tocados). Ambos rodaram (diff: 0 tocados na
  árvore atual, esperado). **Não foi exercido** um par antes/depois real de um
  lote F (não existe ainda). Requisito futuro: protocolo de uso de `--comparar`
  como gate do F (ver §4), validando que o delta esperado é "fan-out de content
  −65; ciclo de 81 → desaparece ou encolhe".

---

## 4. A lente no contrato de verificação do F (avaliação — não implementação)

### 4.1 "Separação de camadas" do F: medível pela lente, antes/depois? — **SIM (parcial).**

A promessa da fronteira F (E1) — *"um elemento de usuário importa só o trait +
tipos públicos; o núcleo não o conhece"* — tem uma **assinatura estrutural
direta e computável** nesta baseline:

- **Hoje (modelo D):** `content` fan-out = 82, dos quais **65 → elementos**;
  ciclo 0 = 81 módulos com os 65 elementos dentro (§2.2, §2.4).
- **Sob F (esperado):** ao núcleo deixar de importar cada `*Elem`, espera-se
  `content` fan-out → ~17 (−65); a aresta content→elemento desaparece; o ciclo de
  81 **colapsa** (elementos viram folhas verdadeiras, fan-in de content cai).

Portanto o critério "núcleo não conhece o elemento" **pode ser verificado por
número**, lote a lote, em vez de por argumento:
`(arestas content → entities::elements::*) == 0` após cada lote F migrado.
A lente produz esse número via `--estrutura --json` (campo `dependencias`,
filtrando `de==content & para∈elements`). O `--comparar` daria o delta direto.

**Ressalva (parcial → vira R4):** a lente conta a aresta mas não distingue se ela
é por *trait legítimo* ou por *struct concreta*. Como sob F a aresta correta é
zero (núcleo não importa nem o trait dos elementos concretos individualmente, só
o `Element` genérico), a condição-zero ainda é verificável; mas para lotes
intermediários (alguns elementos migrados, outros não) a contagem bruta basta.
A medição é **direcional e honesta** — supera o balanço narrativo de P332.

### 4.2 Lente como o lock de verificação da cláusula 3 da ADR-0105? — **NÃO (hoje); candidata a R3+R4.**

A ADR-0105 cláusula 3 manda um lock que **varre a tabela const de handlers** e
detecta um *acesso de campo/propriedade* que o F substitui por *lookup*. Avaliação:

- **O que a lente vê:** acoplamento estrutural **module-level** — "content
  referencia elements::heading". Vê a *aresta*, não o *sítio*. Não localiza, dentro
  de content.rs, a entrada da tabela const nem o acesso de campo específico.
  Confirmado na fonte: content.rs tem **292** construções `match self / =>`
  (sítios de dispatch por-elemento) e `element_payload.rs` tem o `ElementKind` de
  12 variantes — exatamente o tipo de tabela que o lock precisa varrer, e que a
  lente **não** desce a ler.
- **Por que não serve como lock hoje:** o lock exige granularidade **item-level
  intra-módulo** (qual variante, qual handler, qual campo) — precisamente o que a
  R3 diz que falta — e exige **kind de aresta** (campo vs lookup) — o que a R4 diz
  que falta. A lente é estrutural, não comportamental (limite declarado no seu
  README): vê o que um item *referencia*, não o que o código *faz na tabela em
  runtime*.
- **Veredito:** **não substitui** o grep/test mão-mantido do lock da ADR-0105
  cláusula 3 — opera na camada errada de granularidade. O que a lente **pode**
  fazer, como complemento, é o lock *grosso* de camada (§4.1): "fan-out de content
  para elements == 0". Os dois locks são ortogonais: lente = lock de separação de
  camada (módulo); ADR-0105 c.3 = lock de eliminação de handler (item). Para a
  lente cobrir também o item-level, ver **R3** (DSM intra-módulo) e **R4**
  (kind de aresta). Até lá, o lock de handler permanece mão-mantido.

### 4.3 Síntese para o contrato F

| Critério F | Medível pela lente hoje? | Como |
|------------|--------------------------|------|
| "núcleo não conhece o elemento concreto" | **Sim (grosso)** | `arestas content→elements::* == 0` por `--estrutura --json`; delta por `--comparar` |
| "elemento importa só trait + tipos públicos" | **Parcial** | aresta contável, mas kind não distinguido → R4 |
| ciclo de 81 colapsa após F | **Sim** | tamanho do SCC contendo content, lote a lote |
| lock da tabela const (ADR-0105 c.3) | **Não** | granularidade item-level + comportamental ausente → R3, R4 |

---

## Apêndice — proveniência dos números

| Número | Origem (comando / artefato) |
|--------|------------------------------|
| 217 módulos, 667 arestas, 3 ciclos [81,4,8] | comando (A), JSON `modulos`/`dependencias`/`ciclos` |
| 970 arestas / 5 ciclos (todas-uses) | comando (B) |
| content fan-in 85 / fan-out 82 (65 elementos cada lado) | comando (A), filtro sobre `dependencias` |
| 65 elementos, 0 arestas elem→elem, 0 ciclos entre elementos | comando (A), filtro `de,para ∈ elements` |
| 65 elementos todos no ciclo 0 (81) | comando (A), interseção SCC × elements |
| ranking top (Span 2245, …) | comando (C) |
| 292 sítios `match/=>` em content.rs; ElementKind=12 variantes | leitura direta da fonte (sem alteração) |
| DSM HTML | comando (D), `/tmp/typst-dsm.html` |
| `--diff` 0 tocados | comando (E) |

Lente: `tekt-cargo-dsm@98d8f9e` (main) · fork `cargo-modules@ddcd3ca` (v0.27.0).
Produto: `typst-crystalline@f83f78a13`. Nenhum ficheiro de produto tocado.
