# Lacunas vanilla→cristalino via lente — 2026-07-15 (diagnóstico)

**Tipo**: Diagnóstico (não materializa código L1–L4; não é prompt L0).
**Data**: 2026-07-15.
**Motivo**: pedido direto do utilizador nesta sessão — "usar a lente para verificar o
que falta no projeto", com o aviso explícito de que há diferenças por causa da nova
arquitetura. Não corresponde a um Passo numerado da sequência P590–P762 (handoff
`00_nucleo/handoff-novo-chat-p762.md`); é diagnóstico ad-hoc desta sessão de chat,
reaproveitando a metodologia já fechada em P385/P386.
**Padrão**: diagnóstico-primeiro; medir-antes-de-decidir (ADR-0108); proveniência de
cada número registada (regra "registar a proveniência de cada medição", CLAUDE.md).

---

## 1. Proveniência da medição (reprodutível)

| Item | Valor |
|------|-------|
| Lente | binário `/home/dikluwe/.cargo/bin/lente` (mtime 2026-06-12; sem `--version` disponível para hash) |
| Produto medido (cristalino) | HEAD `82e84356fb0c80a58f0da21c28a5a0c476937d32` (P762) + working tree com 176 ficheiros não commitados (todos `.md`/diagnóstico — nenhum toca `01_core`/`02_shell`/`03_infra`/`04_wiring`) |
| Vanilla (`lab/typst-original`) | commit `032a33e4037bd46d5b4ecccf59461c6c2310e4ee`, 2026-06-29 17:28 -03 — "sincroniza com upstream/main v0.15.0" |
| Comando | `lente --comparar --antes lab/typst-original --depois .` (escopo `seu-codigo`); symlink `Cargo.toml.original→Cargo.toml` criado e removido logo a seguir (diff de produção vazio, confirmado por `git status --porcelain` antes/depois) |
| Campo consumido | `itens.sem_par_antes` |
| Entrada congelada | `00_nucleo/diagnosticos/entrada-lente-so-vanilla.2026-07-15.txt` (TSV, 11325 linhas) |
| Scripts reusados (sem alteração) | `lab/parity/tools/decompor_so_vanilla.py` (P385), `lab/parity/tools/falta_migrar.py` (P386, lógica replicada num script de scratch a apontar para a entrada de hoje — `candidatos_lingua()`/`lista_B()`; o ficheiro do repo não foi tocado) |
| Total só-vanilla | **11325** |

### 1.1. Por que os números divergem do portão do README (`lab/mapa-migracao/README.md`)

O README espera (laudo 0078) `pareados≈1474`, `sem-par≈10910/1203`. Esta corrida deu
`pareados=1738`, `sem_par_antes=11325`, `sem_par_depois=2643`, `ambíguos=134`. Regra do
próprio README — "se divergir muito, parar e reportar, não improvisar" — por isso este
número **não fecha nenhum portão**, só é reportado com a causa identificada: o lado
vanilla foi **resincronizado com o upstream v0.15.0 em 2026-06-29** (commit `032a33e4`),
**depois** do laudo 0078 e depois do snapshot de 2026-06-21 usado no P386. O total de
itens do lado vanilla mudou de versão, não é regressão da lente nem do cristalino. Não
recalibrei o portão do README — isso é decisão de dono, não deste diagnóstico.

---

## 2. Decomposição balde 1/2/M/3 (metodologia P385, script inalterado)

| Balde | Significado | Itens | % |
|-------|-------------|------:|---:|
| **1 — renomeado-com-registro** | símbolo migrado sob outro nome, âncora escrita na literatura/Inventário 148 | 393 | 3.5% |
| **2 — fora-de-escopo-com-ADR** | backends/tooling (topologia) + vtable ADR-0026 + atributos graded | 2051 | 18.1% |
| **M — mecânica-não-língua (ADR-0107)** | impl de trait (5164) + método de tipo registado (1026) + método Rust convencional (213) + macro (12) | 6415 | 56.6% |
| **3 — resíduo-genuíno** | sem âncora e sem scope-out | 2466 | 21.8% |
| | **SOMA** | **11325** | 100% |

Reconciliação fechada (`decompor_so_vanilla.py` reafirma `soma == total`). Leitura
central igual à do P385: **mais de metade** do "só-vanilla" é mecânica pura do Rust
(trait-impl/método/macro), que ADR-0107 exclui explicitamente do critério de paridade.

### 2.1. Balde 2 — topologia por crate (medido)

| Crate vanilla | Itens | Âncora |
|---|---:|---|
| `typst_pdf` | 509 | ADR-0033 / ADR-0075 (PDF backend externo) |
| `typst_html` | 500 | ADR-0075 (HTML backend externo) |
| `typst_docs` | 229 | topologia: tooling de docs |
| `typst_kit` | 169 | topologia: I/O kit |
| `typst_svg` | 149 | ADR-0033 / ADR-0075 (SVG renderer externo) |
| `typst_ide` | 127 | topologia: sem camada IDE (CLAUDE.md L1–L4) |
| `typst_bundle` | 67 | topologia: bundling de assets/fonts |
| `typst_render` | 53 | ADR-0033 (PNG/raster backend) |
| `typst_timing` | 22 | topologia: instrumentação |
| `test_wrapper` | 14 | topologia: harness de teste |
| `typst_fuzz` | 9 | topologia: harness de fuzz |
| `typst_macros` | 6 | ADR-0026 + CLAUDE.md: sem proc-macros vtable |
| vtable ADR-0026 (`foundations::cast`/`content::{element,field,vtable,packed,raw}`) | 167 | scope-out — Content enum fechado |
| feature graded (ADR-0054/0082/0083/0097) | 30 | atributos cosméticos scope-out |

---

## 3. Balde 3 → candidatos de nível-de-língua (filtro P386, ADR-0107)

Do resíduo bruto (2466), separando nível-de-língua (struct/enum/type/trait = 404) de
nível-fn (2062), e dentro do nível-de-língua removendo mecânica-de-execução (módulos
`typst_layout::*`, `typst_eval::{vm,call,flow,access,math}`, `typst_realize::*`,
`math::ir`, introspecção — divergem **de propósito**, ADR-0107):

| Categoria | Itens |
|---|---:|
| (c) mecânica-de-execução (não-dívida) | 150 |
| (a/b) candidato genuíno — tipos (struct/enum/type/trait) | 254 |
| candidato genuíno — free-fn (sem `::`, fora de EXEC_MOD) | 387 |
| **total candidatos cruzados com Inventário 148** | **641** |

### 3.1. Cruzamento com o Inventário 148 (`typst-cobertura-vanilla-vs-cristalino.md`)

| Veredicto | Itens | Leitura |
|---|---:|---|
| `nao-divida-migrado` | 190 | inventário marca `implementado` — falso-positivo mecânico da lente |
| `parcial-ja-listado` | 47 | já rastreado como `parcial` no inventário — não é lacuna nova |
| `lacuna-inventario` | 400 | **fora do índice** do inventário (granularidade — ver §4) |
| `divida-confirmada` | 4 | inventário marca `ausente` explicitamente |

Lista completa (641 linhas, TSV) congelada em
`00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt`.

---

## 4. Verificação manual — os 4 "confirmados" e o achado de granularidade

Diferente de P385/P386 (que pararam na proposta de julgamento), esta sessão **abriu o
código** dos 4 itens `divida-confirmada` e de uma amostra do maior grupo
`lacuna-inventario`, porque o pedido do utilizador foi especificamente para checar
lacunas **dado que a arquitetura nova causa diferenças** — e essas diferenças afetam os
dois lados (falsos-positivos de dívida E falsos-negativos de cobertura).

### 4.1. Os 4 `divida-confirmada` — 3 reais, 1 falso-positivo

| Item | Veredicto após leitura do código | Evidência |
|---|---|---|
| `typst_library::model::title::TitleElem` (`#title()`) | **✅ confirmado ausente** | Nenhuma função `title` registada no scope do stdlib (`01_core/src/engine/stdlib/`); `document(title:…)` só guarda metadado — não existe o elemento de markup `#title()` que o vanilla expõe para renderizar o título no corpo do documento |
| `typst_library::foundations::symbol::repr_variants` | **✅ confirmado incompleto** | `01_core/src/engine/eval/repr.rs:95` — `Value::Symbol(s) => s.ch.to_string()` — imprime só o char; não reproduz `symbol("α")`/`symbol(("bold","α"),…)` para símbolos compostos/modificados |
| `typst_library::visualize::gradient::process_stops` | **❌ falso-positivo** | Lógica presente, reestruturada inline em `Gradient::sample()` (`01_core/src/entities/gradient.rs:147` e variantes radial/cónica) — exatamente o efeito de arquitetura nova (free-fn vanilla → método inline cristalino) |
| `typst_library::visualize::gradient::sample_stops` | **❌ falso-positivo** | mesma causa que o item acima |

Sinal real e específico desta corrida: **`#title()` ausente** e **`repr()` de `Symbol`
incompleto para variantes/modificadores**. Não sistémico.

### 4.2. Achado adicional: granularidade do Inventário 148 infla `lacuna-inventario`

O maior grupo por módulo dentro de `lacuna-inventario` (400 itens) é
`typst_library::foundations` (93), quase todo `calc.*` (`abs`, `sin`, `gcd`, `clamp`,
`binom`, …). Verificado: **já implementados** em `01_core/src/engine/stdlib/calc.rs`
(registo `dict.insert("abs".into(), Value::Func(Func::native("calc.abs", calc_abs)))`,
etc., com testes em `01_core/src/engine/stdlib/mod.rs`). O Inventário 148 trata `calc`
como módulo único, não função-a-função — por isso o índice de features não tem entrada
para `abs`/`sin`/etc. individualmente, e o cruzamento cai em "fora do índice" em vez de
"implementado". **Não é dívida — é lacuna de granularidade do inventário**, não do
código.

Extrapolando (não verificado item-a-item): o segundo maior grupo,
`typst_library::diag` (24 itens — `At`, `Hint`, `Trace`, `HintedStrResult`,
`PackageError`, …), é infraestrutura de propagação de erro do Rust (traits de
extensão, aliases de `Result`), não símbolos de língua Typst. Isto expõe um limite do
script P385: `classificar()` só desvia para o balde M quando o campo `trait` do item
está preenchido (impl-de-trait); a **definição** de um trait como `At`/`Hint` tem
`trait=""` e cai no resíduo como "candidato genuíno" indevidamente. Fica registado como
limitação do script (não corrigido aqui — ver §6), não como dívida de língua.

Dito isto, o texto de mensagens de erro **é** um observável legítimo de paridade
(CLAUDE.md: "aceitação no nível da língua, nunca mecânica — exceto onde a mecânica é o
observável — mensagem de erro"). Se `PackageError`/`LoadError` do vanilla produzem texto
de erro que o cristalino não reproduz palavra-por-palavra, isso seria dívida real — mas
**não foi verificado** nesta sessão; fica como pergunta em aberto, não como achado.

---

## 5. Síntese

- Da massa bruta de 11325 itens "só-vanilla", **78,2%** é mecânica de Rust (56,6%),
  fora-de-escopo arquitetural (18,1%) ou renome com registo (3,5%) — não é dívida de
  língua, por construção (ADR-0107/CLAUDE.md).
- Do resíduo restante filtrado a candidatos de língua (641), cruzado com o Inventário
  148: só **4** batem como `ausente` explícito, e a leitura manual do código confirma
  **2 lacunas reais e distintas**: `#title()` (elemento de markup ausente) e `repr()` de
  `Symbol` (incompleto para variantes/modificadores) — as outras 2 linhas do mesmo grupo
  eram a mesma função (`process_stops`/`sample_stops`) já migrada sob forma diferente.
- Os **400** itens fora do índice do inventário não são, na amostra verificada
  (`calc.*`), dívida — são lacuna de granularidade do documento de inventário. Não foi
  feita verificação item-a-item das 400 linhas; a amostra (`calc`, `diag`) é indicativa,
  não exaustiva.
- Confirma-se o aviso do utilizador: a arquitetura nova produz diferenças nos dois
  sentidos — **falsos-positivos de dívida** (renome, reestruturação inline como
  `process_stops`→`Gradient::sample`, calc como módulo vs função) e um **falso-negativo
  do próprio script** (definições de trait sem impl caem no resíduo indevidamente).

---

## 6. O que NÃO foi feito (scope-out explícito)

- Não se verificou item-a-item as 400 linhas `lacuna-inventario` nem as 387 free-fns —
  só a amostra `calc`/`diag` citada em §4.2.
- Não se corrigiu `decompor_so_vanilla.py` para desviar definições-de-trait puras
  (`trait=""`, kind=`trait`, sem impl) para um balde próprio — ficou registado como
  limitação (§4.2), não corrigido.
- Não se tocou código L1–L4 nem se escreveu prompt L0 — este documento é diagnóstico,
  não legitima código (regra de ouro do CLAUDE.md).
- Não se recalibrou o portão do README do mapa de migração (`lab/mapa-migracao/README.md`)
  para os novos números — decisão de dono.
- Não se abriu ADR nem passo de implementação para `#title()`/`repr(Symbol)` — fica
  como insumo de roadmap, não como decisão tomada aqui.

---

## 7. Artefactos

- Entrada congelada: `00_nucleo/diagnosticos/entrada-lente-so-vanilla.2026-07-15.txt` (11325 linhas TSV)
- Veredictos completos (Lista B): `00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt` (641 linhas TSV)
- Scripts reusados sem alteração: `lab/parity/tools/decompor_so_vanilla.py`, `lab/parity/tools/falta_migrar.py` (lógica de `candidatos_lingua()`/`lista_B()` replicada num script de scratch só para apontar à entrada de 2026-07-15; nenhum ficheiro do repo foi alterado)
- Reproduzir a decomposição: `python3 lab/parity/tools/decompor_so_vanilla.py --entrada 00_nucleo/diagnosticos/entrada-lente-so-vanilla.2026-07-15.txt`
