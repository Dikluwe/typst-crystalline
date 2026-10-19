# Lista A — o que falta migrar (top-down, Inventário 148) — Passo 386 eixo 1

**Tipo**: Diagnóstico (não materializa código L1–L4).
**Data**: 2026-06-21.
**Fonte**: Tabela A (vista user-facing, secções A.1–A.9) de
`typst-cobertura-vanilla-vs-cristalino.md` (Inventário 148), HEAD `a876d8d75`.
**Método**: extração **determinística** — `lab/parity/tools/falta_migrar.py --lista-A`
(2 corridas → output idêntico, critério 6.6).

> **Eixo 2 (i18n) adiado.** Por decisão do dono no início da execução (gatilho §156 do
> passo: superfície de strings grande — ~1062 sítios de erro/format só em L1), o eixo 2
> (Lista C + ADR i18n + DEBT) destaca-se como **Passo 387 dedicado**. Este passo entrega
> só o eixo 1 (Listas A e B).

---

## 1. Resumo

**44 entradas** `ausente`/`parcial` na vista user-facing — **16 ausente + 28 parcial**.
Nenhuma struck-through dentro da Tabela A (as resolvidas — smartquote/underline/footnote/
repeat — já saíram da Tabela A; aparecem riscadas só na Tabela C). Distribuição por categoria:

| Categoria | ausente | parcial | total |
|-----------|--------:|--------:|------:|
| Model (structural) | 2 | 10 | 12 |
| Foundations (stdlib) | 2 | 5 | 7 |
| Text features | 5 | 1 | 6 |
| Visualize | 4 | 1 | 5 |
| `#let`/`#set`/`#show` | 1 | 4 | 5 |
| Markup syntactic | 1 | 3 | 4 |
| Layout | 1 | 2 | 3 |
| Math | 0 | 1 | 1 |
| Introspection | 0 | 1 | 1 |
| **Total** | **16** | **28** | **44** |

A maior superfície de dívida user-facing é **Model** (12) — o cluster bibliografia/cite/
footnote/caption/par/table-header-footer — seguida de **Foundations** e **Text**.

---

## 2. Lista A — ausente (16)

Feature vanilla user-facing sem captura cristalina. Roadmap da coluna Tabela C / Referência.

| Categoria | Feature | Roadmap / bloqueante (Tabela C) |
|-----------|---------|----------------------------------|
| Model | `asset`, `title` | `Content`/elemento ausente — cluster document |
| Model | `document(...)` | metadados de documento; escopo M |
| Text | `lorem` | sem stdlib helper; escopo S |
| Text | `smallcaps` | `Content::SmallCaps`; OpenType features; DEBT-53 (shaping) |
| Text | `text.dir` (LTR/RTL) | bidi shaping ausente; DEBT-53 |
| Text | `text.region` | `Region` type ausente; XL com rustybuzz |
| Text | `text.script` | `Script` type ausente; XL com rustybuzz |
| Visualize | `gradient(...)` | `Value::Gradient` ausente; render gradient PDF; escopo M |
| Visualize | `tiling(...)` | `Value::Tiling` ausente; escopo M |
| Visualize | `cmyk`/`oklab` cores | color space não-RGB ausente; escopo S |
| Visualize | `square(...)` | `ShapeKind::Square` (derivável de Rect w=h); escopo XS |
| Layout | `pad`/`corners`/`sides` (inset modeling) | refino PageConfig (Fase 3 ADR-0061) |
| Foundations | `eval(string)` | runtime de re-eval ausente; escopo M |
| Foundations | `panic(msg)` | sem helper stdlib; escopo XS |
| `#show` | `#show <selector>` (regex/where) | `regex` em L1; ADR-0054bis condicional |
| Markup | Soft hyphen (`\u{00AD}`) | hyphenation espera literal `-` (P144); passo dedicado |

## 3. Lista A — parcial (28), com ressalva

Captura/estrutura existe; falta o que a coluna Nota do inventário regista.

| Categoria | Feature | Ressalva (o que falta para `implementado`) |
|-----------|---------|---------------------------------------------|
| Model | `bibliography(path)` | CSL parser; `Content::Bibliography`; `loading` module — escopo XL |
| Model | `cite(key)` | `Content::Cite` + bibliography parser — escopo XL |
| Model | `footnote(body)` | P295 marker materializado; sub-passos nota-rodapé + overflow pendentes |
| Model | `caption(...)` | parcial — refino figure |
| Model | `link(dest, body)` | capturado, sem render visual; escopo S |
| Model | `enum(items)` / `list(items)` (function form) | forma-função parcial vs markup |
| Model | `par` (paragraph element) | elemento parágrafo parcial |
| Model | `table.header` / `table.footer` | parcial — cell layout |
| Foundations | `repr(value)` | `repr` de cada Value variant parcial (subset materializado) |
| Foundations | `array.{push,pop,…}` / `dict.{at,keys,…}` / `str.{contains,…}` | métodos de coleção parciais |
| Foundations | `import/from math:` | namespacing parcial |
| Text | `linebreak` (function) | forma-função parcial |
| Visualize | `stroke(...)` (object) | forma-objeto parcial |
| Layout | `columns(n)` / `colbreak()` | variant + consumer graded; multi-region flow real ausente (Fase 4) |
| Markup | `- list item` / `+ enum` / `1. enum` | markup de lista/enum parcial |
| Markup | Quebra de linha (`\`, `linebreak`) | parcial |
| Math | `equation.numbering` | numbering de equação parcial |
| Introspection | `position(target)` | runtime de posição parcial |
| `#set` | `figure`/`heading`/`par`/`table`/`grid` | set-rules com subset de campos |

> A tabela completa com a coluna roadmap/ressalva integral sai de
> `python3 lab/parity/tools/falta_migrar.py --lista-A` (TSV).

---

## 4. Cruzamento com a Lista B

A Lista B (bottom-up, `typst-falta-migrar-lista-B-passo-386.md`) **confirma** esta lista:
os 19 candidatos do resíduo 385 que batem classe `ausente` mapeiam todos a entradas desta
Lista A (`lorem`, `smallcaps`, `panic`, `eval`, gradient, e os `Value::{Bytes,Decimal,
Duration,Version}` da Tabela C). A Lista B **acrescenta um achado**: o cluster de
**data-loading** (`csv`/`json`/`yaml`/`toml`/`cbor`/`xml`) — user-facing, ausente em L1 e
**não catalogado** nesta Lista A nem no Inventário (ver Lista B §3). Candidato a entrada
nova no Inventário 148.

---

## 5. Aviso de staleness (§153 do passo)

Esta lista reflete o Inventário 148 no HEAD. Entradas `ausente` podem já estar
implementadas e não reclassificadas. Conferência feita para os achados da Lista B
(data-loading confirmado ausente em código). A conferência exaustiva de cada `ausente`
da Lista A é trabalho dos passos de materialização que esta lista alimenta, não deste
diagnóstico.
