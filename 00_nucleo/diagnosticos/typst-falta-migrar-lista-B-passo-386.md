# Lista B — julgamento do resíduo 385 (bottom-up) — Passo 386 eixo 1

**Tipo**: Diagnóstico (não materializa código L1–L4).
**Data**: 2026-06-21.
**✅ Achado resolvido (Passo 387)**: o cluster `loading` (data import) do §3 foi
materializado no Passo 387 (ADR-0111): `read`+`csv`/`json`/`yaml`/`toml`/`cbor`/`xml` em
`01_core/src/rules/stdlib/loading.rs`; entrada nova no Inventário 148 (A.8). Lacuna fechada.
**Fonte**: balde 3 (resíduo genuíno, 2339 itens) do Passo 385, filtrado a candidatos de
língua e cruzado com o Inventário 148.
**Método**: `lab/parity/tools/falta_migrar.py --lista-B` (determinístico; o cruzamento é
script, o veredicto final é confirmado por julgamento — §B.3 do passo).

> A Lista B existe para **refutar** ou **completar** a Lista A, não para duplicá-la. O
> valor dela são os dois extremos: dívida que o inventário já tem (confirma A) e dívida
> que o inventário **não** tem (corrige A).

---

## 1. Filtro de candidatos (ADR-0107) e reconciliação (critério 6.3)

Do resíduo balde 3 (2339), o filtro de língua descarta a mecânica de execução
(`typst_layout::*`, `typst_eval::{vm,call,flow}`, `typst_realize::*`, `math::ir::*`,
maquinaria de introspecção) e mantém tipos de nível-de-língua (struct/enum/type/trait) e
free-fns. Sobram **608 candidatos**, cada um com **exatamente um veredicto** (nenhum órfão):

| Veredicto | Itens | Significado |
|-----------|------:|-------------|
| `lacuna-inventario` | 401 | sem entrada no Inventário → vai para revisão (não direto a dívida, §154) |
| `nao-divida-migrado` | 167 | bate `implementado` → migrado, mecanicamente divergente (chave K4 difere) |
| `parcial-ja-listado` | 21 | bate `parcial` → já na Lista A |
| `divida-confirmada` | 19 | bate `ausente` **e** é candidato de língua → dívida |
| **Total** | **608** | reconciliação fechada ✓ |

---

## 2. Dívida confirmada (19) — **confirma a Lista A**

Os 19 candidatos que batem classe `ausente` mapeiam **todos** a features já na Lista A ou
na Tabela C do Inventário — Lista B não encontra dívida user-facing **nova** entre os
matches diretos:

| Cluster | Itens do resíduo | Entrada Lista A / Tabela C |
|---------|------------------|----------------------------|
| `lorem` | `lorem#1`, `lorem#2` | Lista A · Text · ausente |
| `smallcaps` | `Smallcaps` | Lista A · Text · ausente |
| `panic` | `panic#1`, `panic#2` | Lista A · Foundations · ausente |
| `eval` | `Eval`, `eval#2` | Lista A · Foundations · ausente |
| `gradient` | `process_stops`, `sample_stops` | Lista A · Visualize · ausente |
| `repr` | `repr_variants` | Lista A · Foundations · parcial |
| `Value::Bytes` | `out_of_bounds*` | Tabela C · `Value::Bytes` ausente |
| `Value::Decimal` | `warn_on_float_literal` | Tabela C · `Value::Decimal` ausente |
| `Value::Duration` | `DurationDisplay`, `format_duration` | Tabela C · `Value::Duration` ausente |
| `Value::Version` | `version` | Tabela C · `Value::Version` ausente |
| `asset` | `AssetData`, `AssetElem` | Lista A · Model · ausente (`asset`) |
| `document` | `determine_format_from_path` | Lista A · Model · ausente (`document`) |

**Conclusão do match direto:** o Inventário 148 é **completo ao nível user-facing** — a
rede de segurança bottom-up não pega nenhuma feature catalogável que a Lista A já não tenha.

---

## 3. O achado — lacuna-do-inventário genuína (§B.3): **data-loading**

O veredicto `lacuna-inventario` (401) é uma **superfície de revisão**, não dívida (§154 do
passo: "o que passa e não bate o inventário vai para revisão"). Caracterização (julgamento,
**marca inferência**, refutável): a esmagadora maioria é **mecânica/interno** que o filtro
de língua não pegou ou **família já migrada** que o join não casou —

- `foundations::calc` (45): `cos`/`sin`/`abs`/… **migrados** em `make_calc_module` (P299);
  não batem porque o inventário não lista cada função calc individualmente. Não é dívida.
- `math::style` (32), `layout::grid::resolve` (24), `foundations::ops` (21),
  `image::{raster,svg}` (20): internos de algoritmo/decode — mecânica.
- `diag` (24): maquinaria de diagnóstico — **território do eixo 2 (i18n, Passo 387)**.
- `define` (×7), reflection `fields`/`scope`, `highlight` (syntax), `hash`/`encode`: mecânica.

**A exceção genuína — o cluster `loading` (data import).** O módulo `typst_library::loading`
inteiro aparece como lacuna **e** está confirmado **ausente em L1** (grep: zero hits em
`rules/stdlib` para `csv`/`json`/`yaml`/`cbor`/`xml`) **e** o Inventário 148 **não o
cataloga** (só o menciona como bloqueante de `bibliography`). É uma feature user-facing real:

| Função | Símbolo vanilla |
|--------|------------------|
| `read(path)` | `loading::read_::read` |
| `csv(path, …)` | `loading::csv_::csv` (+ `Delimiter`, `RowType`) |
| `json(path)` | `loading::json_::json` |
| `yaml(path)` | `loading::yaml_::yaml` |
| `toml(path)` | `loading::toml_::toml` |
| `cbor(path)` | `loading::cbor_::cbor` |
| `xml(path)` | `loading::xml_::xml` (+ `convert_xml`) |
| (infra) | `DataSource`, `LoadSource`, `Loaded`, `Readable`, `Encoding` |

**Recomendação:** abrir uma entrada nova no Inventário 148 — categoria Foundations/Loading,
classe `ausente`, escopo M (parsers `csv`/`json`/`yaml`/`toml`/`cbor`/`xml` + `read`). É o
único produto líquido da Lista B além de confirmar a Lista A.

---

## 4. Não-dívida (167) — migrado, mecanicamente divergente

Itens que batem `implementado` mas aparecem como só-vanilla porque a chave K4 difere
(módulo/assinatura diferente no cristalino). Concentram-se em `text` (22),
`visualize::color` (17), `text::font::color` (14), `grid` (10), `math::lr` (9),
`image` (8). **Não é dívida** (ADR-0107) — é a evidência viva de que a paridade é de
língua, não de mecânica. Reforça a leitura do Passo 385.

---

## 5. Reconciliação eixo 1 (critério 6.3)

Todo candidato de língua recebe exatamente um veredicto (608 = 401+167+21+19). A Lista B
**confirma** a Lista A (zero dívida user-facing nova nos matches diretos) e **corrige-a**
com um achado: o cluster `loading` (data import), a catalogar. Refutação: se a revisão
humana das 401 lacunas achar outra família user-facing ausente além de `loading`, sobe a
achado novo — a caracterização do §3 é proposta, não veredicto fechado.
