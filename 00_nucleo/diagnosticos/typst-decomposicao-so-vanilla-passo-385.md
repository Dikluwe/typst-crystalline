# Decomposição do conjunto só-vanilla — Passo 385 (diagnóstico)

**Tipo**: Diagnóstico (não materializa código L1–L4).
**Data**: 2026-06-21.
**Passo fonte**: `typst-passo-385.md` (o cabeçalho tinha "Passo 285" — typo corrigido
para 385 neste passo; a data e os precedentes citados eram já os do 385).
**Padrão**: diagnóstico-primeiro; medir-antes-de-decidir (ADR-0108).

> **Nota de método (literal do passo §1).** A medição é um **script determinístico**,
> não um julgamento de LLM. Pôr o LLM no loop de contagem torna o número
> não-reprodutível. O LLM entrou só onde a forma é irregular (propor o marcador) e
> onde o resíduo exige julgamento — sempre como **proposta a confirmar**, nunca como
> autoridade da contagem.

---

## 1. Proveniência da medição (reprodutível)

| Item | Valor |
|------|-------|
| Lente | `tekt-cargo-dsm`, binário `target/release/lente`, commit `98d8f9e` |
| Produto medido | `typst-crystalline` HEAD `b17525ef0` (Passo 384) |
| Comando | `lente --comparar --antes lab/typst-original --depois .` (escopo `seu-codigo`) |
| Campo consumido | `itens.sem_par_antes` (itens presentes no vanilla, sem par no cristalino) |
| Entrada congelada | `00_nucleo/diagnosticos/entrada-lente-so-vanilla.2026-06-21.txt` (TSV, 10745 linhas) |
| Script de medição | `lab/parity/tools/decompor_so_vanilla.py` (determinístico; 2 corridas → output idêntico) |
| Total só-vanilla | **10745** — confirma exatamente o "~10745" reportado (§1 do passo) |

> Para correr `--comparar` foi preciso restaurar temporariamente o
> `lab/typst-original/Cargo.toml` (quarentena renomeia-o para `.original`) e excluir
> `lab` do workspace cristalino. Ambas as alterações foram **revertidas**; o diff de
> produção é vazio (critério 6.5). A entrada congelada torna o resto reproduzível sem
> re-correr a lente.

---

## 2. A decomposição (reconciliação fechada — critério 6.1)

O conjunto só-vanilla mistura quatro coisas. Tratá-las como uma só infla a leitura de
dívida em ~25×. O script classifica cada item por **ordem documentada** (primeiro match
vence): crate-fora-de-escopo → vtable ADR-0026 → renomeado-com-registro → método/feature →
mecânica-Rust → resíduo.

| Balde | Significado | Itens | % |
|-------|-------------|------:|---:|
| **1 — renomeado-com-registro** | o símbolo migrou sob outro nome; correspondência escrita na literatura "Sobre paridade"/Inventário 148 | **384** | 3.6% |
| **2 — fora-de-escopo-com-ADR** | backends/tooling (topologia), sistema de elementos vtable (ADR-0026), atributos graded (ADR-0054) | **1753** | 16.3% |
| **M — mecânica-não-língua (ADR-0107)** | item Rust (impl de trait, método convencional, macro) cuja paridade **não se mede a este nível** | **6269** | 58.3% |
| **3 — resíduo-genuíno** | símbolo de nível-de-língua sem âncora escrita e sem scope-out | **2339** | 21.8% |
| | **SOMA** | **10745** | 100% |

**A leitura central.** **58.3%** do "só-vanilla" é **mecânica do Rust** (5031 impl de
trait + 1227 métodos + 11 macros), não língua Typst. A lente pareia por chave **K4
mecânica** `(kind, trait, pai::nome)`; por isso conta como "só-vanilla" todo `fn`,
método e `impl` não-pareado **mesmo quando a feature de língua está migrada**. Por
ADR-0107 a paridade é com a **língua**, nunca com a mecânica de execução ou a igualdade
do Rust — logo o balde M **não é dívida**. Somado ao scope-out declarado (16.3%) e aos
renomes (3.6%), sobram **21.8%** de resíduo a julgar — e mesmo esse encolhe (§4).

---

## 3. Balde 2 — fora-de-escopo **medido**, não herdado (critério 6.6)

O material anterior estimava "~800 backends". A medição refuta o número: os **backends
de render/export** sozinhos somam **1039**, não ~800.

### 3.1. Topologia por crate (1568) — cada crate com âncora escrita

| Crate vanilla | Itens | Âncora |
|---------------|------:|--------|
| `typst_pdf` | 500 | ADR-0033 §res. visual (PDF diverge) / ADR-0075 (PDF backend externo) |
| `typst_html` | 359 | ADR-0075 (HTML backend externo) |
| `typst_svg` | 131 | ADR-0033 / ADR-0075 (SVG renderer externo) |
| `typst_render` | 49 | ADR-0033 §res. visual (PNG/raster) |
| **subtotal render/export** | **1039** | **(corrige o "~800" herdado)** |
| `typst_kit` | 148 | topologia: I/O kit (download/fonts CLI); L3 reimplementa só o necessário |
| `typst_docs` | 142 | topologia: tooling de docs, não-compilador |
| `typst_ide` | 122 | topologia: cristalino não tem camada IDE |
| `typst_bundle` | 66 | topologia: bundling de assets/fonts |
| `typst_timing` | 22 | topologia: instrumentação/timing |
| `test_wrapper` | 14 | topologia: harness de teste (não-produção) |
| `typst_fuzz` | 9 | topologia: harness de fuzz (não-produção) |
| `typst_macros` | 6 | ADR-0026 + CLAUDE.md: sem proc-macros vtable |
| **subtotal tooling/vtable** | **529** | |

### 3.2. Sistema de elementos por vtable — ADR-0026 (166)

Itens em `foundations::cast` e `foundations::content::{element,field,vtable,packed,raw}`
(`NativeElement`, `Synthesize`, `Construct`, `ShowSet`, `Packed`, `ContentVtable`,
`Reflect`, `IntoValue`/`FromValue`, `CastInfo`, …). É o sistema `#[elem]`/`#[cast]`
vtable que ADR-0026 (Content enum fechado) e CLAUDE.md ("sem proc-macros vtable")
declaram scope-out. A literatura `locatable.md`/`element_kind.md`/`tag.md` já o
classifica como scope-out — só não num campo que a máquina lê (motivo do marcador, §5).

### 3.3. Atributos/feature graded — ADR-0054 (19)

Tokens snake-case de atributos cosméticos declarados scope-out graded (`radius`,
`clip`, …) extraídos das ADRs 0054/0082/0083/0097 e das linhas `scope-out` do
Inventário 148. Número pequeno porque a maioria dos atributos graded é método de tipo
(cai no balde M) e não símbolo próprio.

---

## 4. Balde 3 — o resíduo (2339) e o seu julgamento (§4.3 do passo)

A classificação **per kind** (determinística):

| kind | itens |
|------|------:|
| `fn` | 1943 |
| `struct` | 219 |
| `enum` | 123 |
| `trait` | 38 |
| `type` | 11 |
| `const` | 5 |

O resíduo **não é** "falta migrar 2339 features". É a superfície que precisa de
**julgamento** (4.3), e o julgamento encolhe-a. Abaixo, a proposta de classificação —
**marcada como inferência**, refutável item-a-item; a contagem fina é trabalho de um
passo dedicado de julgamento (Sonnet/humano), não deste diagnóstico.

### 4.1. Resíduo de nível-de-língua (struct/enum/type/trait) = 385

- **(c) mecânica-de-execução — 145.** Módulos de algoritmo cujo cristalino diverge
  **de propósito** (ADR-0107): `typst_layout::*` (`GridLayouter`, `Distributor`,
  `Composer`, `FlowMode`…), `typst_eval::{vm,call,flow}` (`Vm`, `Eval`, `FlowEvent`),
  `typst_realize::*`, o **IR de math** `typst_library::math::ir::*` (`MathItem`,
  `FractionItem`, `MathResolver`…) e a maquinaria de introspecção
  (`QueryCache`, `Introspection`, `*Introspection`). Não é dívida de língua — é a
  **mecânica de execução** que o cristalino substitui por single-pass/enum-fechado.
- **(a/b) candidato genuíno de língua — 240**, concentrado em `typst_library`:
  layout 39 · text 32 · foundations 29 · visualize 19 · diag 18 · math 18 · loading 15
  · pdf 10 · model 6 (resto < 6). Esta é a **superfície real de decisão** — mas ainda
  contém scope-out não-declarado e divergência mecânica residual (ver 4.3).

### 4.2. Resíduo de nível-fn (1943): método vs free-fn

- **Métodos (`Tipo::método`) — 1279**: maioria mecânica de tipos in-scope cujo nome de
  tipo não bateu a literatura. Predominantemente balde-M-não-capturado pela regra
  conservadora.
- **Free-fns — 664**: candidatos a **função stdlib user-facing**. **Mas** a verificação
  cruzada com o Inventário 148 refuta a maioria como dívida: `cos`/`sin`/`abs`/`ceil`/
  `clamp`/… aparecem aqui e **estão migradas** (Inventário: Math 92%; `make_calc_module`
  + 42 operadores math registados em P299) — mostram-se só-vanilla apenas porque a chave
  K4 do cristalino difere (módulo/assinatura diferente). Mecânica-divergente, não dívida.
  Um subconjunto menor (alguns color-maps `cividis`/`coolwarm`/`crest`, formatos de
  dados, `lorem`/`smallcaps` per linhas `ausente` do Inventário) **é** dívida real.

### 4.3. Síntese do julgamento (proposta, marca inferência)

Peladas as quatro camadas, a dívida **genuína de língua** ("falta migrar de verdade")
é da ordem de **dezenas a baixas centenas**, não 10745 — e parte dela já está
implementada-mas-mecanicamente-divergente. O número exato exige o passo de julgamento
4.3 (cruzar cada candidato com o Inventário 148: `implementado`→balde M; `ausente`→
dívida; `scope-out`→balde 2). **O que refutaria esta síntese:** um candidato genuíno
(4.1-a/b ou 4.2-free-fn) que, cruzado com o Inventário, esteja marcado `ausente` **e**
seja user-facing — esse é dívida confirmada e deve subir a insumo de roadmap de feature.

---

## 5. Cobertura/forma da literatura "Sobre paridade" (§5 item 1)

Só **33 de 214** L0 têm a seção `## Sobre paridade`. A forma é **irregular** (símbolo
em backtick, caminho `module/file.rs`, lista, prosa) — é o que quebra o extrator ingénuo
(§8 do passo) e o que motiva o marcador legível por máquina (ADR proposta).

| Domínio | tem/total | Nota |
|---------|----------:|------|
| `entities` (raiz) | 27/70 | melhor cobertura |
| `rules/introspect` | 5/5 | completo |
| `rules` (raiz) | 1/15 | |
| `entities/elements` | **0/66** | **lacuna crítica** — é onde vive o renome dominante `*Elem`→`Content::*` |
| `rules/math` | 0/11 | |
| `rules/stdlib` | 0/6 | |
| `entities/ast` | 0/5 | |
| `infra/export` | 0/14 | (esperado: backend scope-out) |
| restantes | 0/… | |

A **lacuna `entities/elements` (0/66)** explica diretamente por que o balde 1
(renomeado-com-registro) é só 3.6%: a correspondência `HeadingElem`→`Content::Heading`
existe na **construção** (cada elemento foi migrado num passo com âncora vanilla no
corpo do prompt), mas **não** numa seção "Sobre paridade" que o extrator leia. O renome
dominante está escrito — só não num campo que a máquina consome. **Este é o achado que
a ADR proposta ataca.**

---

## 6. Critérios de aceitação — estado

| # | Critério | Estado |
|---|----------|--------|
| 6.1 | Baldes somam o total (reconciliação fechada) | ✓ 384+1753+6269+2339 = 10745 |
| 6.2 | Script reprodutível (2 corridas idênticas) | ✓ `diff` vazio |
| 6.3 | Resíduo explícito e classificado (a/b/c) | ✓ §4 + listas no output `--lista-residuo` |
| 6.4 | ADR de marcador PROPOSTA (4 formas + roadmap) | ✓ ADR-0110 (PROPOSTO) |
| 6.5 | Zero ficheiros de produção L1–L4 alterados | ✓ diff de produção vazio (alterações em lab/diagnosticos/adr só) |
| 6.6 | "~800 fora-de-escopo" confirmado/corrigido | ✓ **corrigido**: render/export = **1039** (§3.1) |

---

## 7. O que NÃO foi feito (scope-out do passo, §6 do passo)

- Não se adicionou marcador `@vanilla` a nenhum ficheiro (passo de semeadura, posterior).
- Não se escreveu check de linter.
- Não se tocou código L1–L4.
- Não se pôs LLM no loop de contagem.
- O julgamento fino do resíduo (4.3, item-a-item) é passo dedicado posterior, não este.

---

## 8. Limitações honestas (§8 do passo)

- **Defasagem lente↔repo.** A lista foi medida contra HEAD `b17525ef0`; é a corrida atual,
  sem defasagem relevante (entrada congelada com proveniência).
- **Heurística de match coarse.** `literatura-modulo` (321) é o sinal mais fraco (match
  por módulo inteiro, não por símbolo); `EXEC_MOD`/`VTABLE_MOD` são listas de prefixo
  auditáveis, extensíveis num passo futuro. Cada regra tem âncora escrita no corpo do
  script. Mis-classificações vão para o resíduo (4.3), não escondidas.
- **Não se ajustou o número para encolher** (§8 do passo). As regras adicionadas
  (vtable-ADR-0026, math::ir como execução) são **categorias arquiteturais ancoradas**
  (ADR-0026, ADR-0107), não tuning a um alvo. O resíduo caiu como consequência, não como
  objetivo.

---

## 9. Nota sobre o Tekt (§10 do passo)

A convenção `@vanilla` (ADR-0110 proposta) é descoberta aqui mas não é específica da
typst-crystalline: é o **registro de construção como oráculo de correspondência** — o
"terceiro oráculo", distinto do sistema-fonte e do julgamento humano. Se a semeadura
confirmar a sua utilidade, é candidata a **lição v1.4 do Tekt**, promovida a partir desta
bancada per o padrão "lab como evidência viva". Registrada a possibilidade; não
materializada no Tekt neste passo.

---

## 10. Artefactos

- Entrada: `00_nucleo/diagnosticos/entrada-lente-so-vanilla.2026-06-21.txt`
- Script: `lab/parity/tools/decompor_so_vanilla.py` (+ `test_decompor_so_vanilla.py`)
- ADR proposta: `00_nucleo/adr/typst-adr-0110-marcador-vanilla.md` (PROPOSTO)
- Reproduzir: `python3 lab/parity/tools/decompor_so_vanilla.py [--lista-residuo]`
