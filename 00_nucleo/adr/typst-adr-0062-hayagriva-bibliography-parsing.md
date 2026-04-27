# ⚖️ ADR-0062: Autorização de crate `hayagriva` para bibliography + cite (CSL parsing)

**Status**: `PROPOSTO`
**Data**: 2026-04-27

---

## Contexto

Bibliography + Cite exigem CSL (Citation Style Language)
parsing para paridade observable per **ADR-0033**. CSL define
formatos de citation académicos (APA, IEEE, MLA, Chicago,
ABNT, etc.) através de XML — parsing e style engine não-triviais.

**Vanilla typst integra `hayagriva` profundamente**:
- `lab/typst-original/crates/typst-library/src/model/bibliography.rs`
  (1226 linhas) importa `hayagriva::{archive, io, Entry, ...}`
  directamente.
- `Bibliography` interno usa `Arc<ManuallyHash<IndexMap<Label,
  hayagriva::Entry, FxBuildHasher>>>`.
- `CslStyle::load(...)` integra hayagriva ArchivedStyle + CSL
  parser.
- Autoridade de manutenção: hayagriva é mantida pela mesma
  organização que typst (`typst/hayagriva` GitHub).

**P159A materializou subset minimal cristalino sem hayagriva**:
- `Content::Bibliography { entries: Vec<BibEntry>, title }` com
  input cristalino literal.
- `Content::Cite { key, supplement }` com placeholder render
  `[key]`.
- Cobre subset minimal (cite/bibliography movem `ausente →
  parcial`) mas **insuficiente para paridade ADR-0060 declarado
  ~68%**.

**DEBT-55** documenta plano XL com hayagriva — 4/10 itens
pendentes pós-P159A:
- ADR-0062 criar (este ADR).
- Cargo.toml + crystalline.toml configurados.
- Pipeline introspect com resolução cruzada (depende ADR-0017).
- Render layout completo (CSL).

**P159B §3 categoria C (crate externa)** identifica hayagriva
como dependência hard para CSL parsing/styles, Bibliography
parsing externo, e Cite.style override.

---

## Decisão

**Autorizar uso da crate `hayagriva` em L1** para:
1. Parsing de entries CSL (formatos `.bib` BibLaTeX e `.yaml`
   Hayagriva nativo).
2. CSL style parsing (XML CSL files).
3. Geração de citation strings formatadas conforme style
   activo.
4. Style engine para resolução de citações (autor-ano,
   numérico, etc.).

**Status PROPOSTO** — autorização concedida em princípio mas
**não em vigor** até passo de materialização real (P159G ou
equivalente que adiciona dependência ao `Cargo.toml` + `crystalline.toml`).

**Promoção a `IMPLEMENTADO`** ocorre quando:
1. `Cargo.toml` adiciona `hayagriva = "0.9.1"` ou versão
   actualizada.
2. `crystalline.toml` adiciona `hayagriva` a
   `[l1_allowed_external]`.
3. Pelo menos um uso real em código L1
   (`stdlib/structural.rs::native_bibliography` ou similar
   integra parsing).

---

## Análise de pureza

| Propriedade | Estado |
|-------------|--------|
| Zero I/O | ✓ — hayagriva opera sobre strings/bytes em memória; ficheiros lidos por wrapper L3 antes de chamar |
| Zero estado global mutável | ✓ — API funcional |
| Determinismo total | ✓ — mesmo input + mesmo style produz output idêntico |
| Dependências transitivas | A confirmar em passo de materialização (`indexmap`/`citationberg`/`unscanny` esperados; `unscanny` já documentado em ADR-0014) |

`hayagriva` é **estrutura de dados + parser puro** —
conceptualmente equivalente a uma serialização especializada
(BibLaTeX/YAML/CSL XML). Nenhum I/O directo; carregamento de
ficheiros fica em L3.

---

## Precedentes citáveis

Autorização de crate externa em L1 com precedentes existentes:

| ADR | Crate | Motivo |
|-----|-------|--------|
| **ADR-0023** | `indexmap` | `Scope` precisa de map com ordem de inserção preservada (semanticamente significativa em Typst) |
| **ADR-0024** | `ecow` | `Value::Str` precisa de `EcoString` (clone O(1) por refcounting) |
| **ADR-0057** | `hypher` | `text.lang` hyphenation — funcionalidade significativa do compilador |

**Padrão consolidado**: cada crate externa em L1 tem ADR
dedicada com análise de pureza + justificação técnica + critério
de promoção. ADR-0062 segue este padrão.

`hayagriva` é o caso mais forte de "crate complementar mantida
pela mesma organização que typst" — autoridade de manutenção
máxima.

---

## Crate hayagriva — informação técnica

- **Versão**: `0.9.1` (já em cache local per probe P152).
- **Repositório**: `github.com/typst/hayagriva`.
- **Licença**: MIT (compatível com L1).
- **Mantedor**: typst organization.
- **Dependências críticas a verificar antes de promoção**:
  `citationberg` (CSL parser), `serde`, `unscanny`.

---

## Consequências

### Positivas

- **Paridade vanilla** — bibliography + cite renderizam
  consistentemente com vanilla.
- **Reuso de CSL compliance** existente — sem duplicar trabalho
  de implementação CSL.
- **Manutenção partilhada** — bug fixes em hayagriva propagam
  automaticamente.
- **Cobertura Model significativa** — paridade ADR-0060
  declarado ~68% atingível com hayagriva integrada.

### Negativas

- **Dependência externa em L1** — precedente documentado em
  ADR-0024/0023/0057; aceitável.
- **Aumento de tempo de compilação** — hayagriva traz
  citationberg + ecosistema CSL.
- **Potencial conflito de versão** com typst-original em
  `lab/typst-original/`. Mitigação: pin versão em
  `Cargo.toml`.
- **API surface** maior que cristalino actualmente cobre —
  subset de uso a definir em passo de materialização.

### Neutras

- **DEBT-55 fica desbloqueada** (4/10 itens restantes
  alcançáveis com hayagriva integrada).
- **ADR-0017 continua adiada** — Introspection runtime é
  problema separado; hayagriva não resolve cite cross-document
  forward refs.
- **Reservas pré-existentes preservadas** — política "sem
  novas reservas" P158 não viola.

---

## Alternativas consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| **Adoptar hayagriva** ✓ | Paridade vanilla; reuso CSL; manutenção partilhada | Dependência externa (precedente coberto) |
| Implementar CSL parsing cristalino do zero | Sem dependência externa | Trabalho desproporcionado (CSL é spec extensa); reinvenção da roda |
| Manter subset minimal sem hayagriva (P159A) | Sem dependência externa; já implementado | Insuficiente para paridade ADR-0060 (~50% vs ~68% target) |
| Usar outra crate (e.g. `biblatex`) | Alternativa | `biblatex` é menos mainstream; hayagriva é default vanilla; quebra de paridade |
| Adiar Bibliography + Cite total | Adia decisão | Bloqueia ADR-0060 Fase 2 indefinidamente; DEBT-55 fica em aberto |

**Escolha**: adoptar hayagriva (paridade + manutenção
partilhada). Subset minimal cristalino (P159A) preservado para
casos sem dependência externa.

---

## Plano de promoção (futuro)

Status `PROPOSTO → IMPLEMENTADO` em passo de materialização
real (P159G ou equivalente):

1. Adicionar `hayagriva = "0.9.1"` (ou versão actualizada) a
   `01_core/Cargo.toml`.
2. Adicionar `hayagriva` a `[l1_allowed_external]` em
   `crystalline.toml`.
3. Verificar dependências transitivas via `cargo tree`;
   actualizar `[l1_allowed_external]` para cobrir todas.
4. Implementar `extract_bib_entries_from_string` ou similar
   para parsing.
5. Tests com inputs reais BibLaTeX/CSL.
6. Inventário 148 reclassifica `cite`/`bibliography` de
   `parcial → implementado`.
7. ADR-0062 transita `PROPOSTO → IMPLEMENTADO`.
8. DEBT-55 critério de fecho parcialmente cumprido (CSL parsing
   resolvido).

---

## Referências

- **ADR-0023** — `indexmap` autorização (precedente crate
  externa L1).
- **ADR-0024** — `ecow` para `Value::Str` (precedente).
- **ADR-0057** — `hypher` para hyphenation (precedente).
- **ADR-0014** — `unscanny` inlinado (precedente quarentena
  de scanner; hayagriva pode usar internamente).
- **ADR-0033** — Paridade observable vanilla (motivação CSL).
- **ADR-0034** — Diagnóstico obrigatório (cumprido em P159A
  + P159B).
- **ADR-0054** — Perfil graded (subset minimal P159A aceite).
- **ADR-0060** — Model roadmap §"Decisão 2" (Bibliography +
  Cite com hayagriva; reserva ADR-0062 documentada).
- **ADR-0017** — Introspection runtime adiada (problema
  separado; cite cross-document forward refs continua adiado).
- **DEBT-55** — Bibliography + Cite XL (plano completo;
  4/10 itens dependem desta autorização).
- **Passo P152** — probe que confirmou `hayagriva 0.9.1` em
  cache local.
- **Passo P159** — diagnóstico Bibliography + Cite confirmou
  ADR-0062 reserva sem ficheiro.
- **Passo P159A** — materialização subset minimal sem
  hayagriva (Vec<BibEntry> literal).
- **Passo P159B** — diagnóstico amplo identificou Bloco B
  (refinos com hayagriva ADR-0062).
- **Passo `ADR-0062-create`** (este passo) — formalização da
  reserva como ADR PROPOSTO.
