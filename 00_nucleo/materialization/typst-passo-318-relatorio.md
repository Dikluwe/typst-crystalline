# Relatório P318 — Lote 3 (lista/termos) + Medições pré-F

**Pré-condição**: Lote 2 (P317) fechado — relatório existente, `crystalline-lint .`
= 0, suíte verde. ✅ Verificado antes de iniciar.

**Dois commits isoláveis**: `Passo 318 — medições pré-F` (Parte 2, primeiro,
tree limpo) e `Passo 318 — lote 3` (Parte 1).

---

## Parte 1 — Lote 3 (família lista/termos), instância do modelo

**Composição (confirmada no checkpoint): 5 variantes** element-shaped, ordem por
largura de uso crescente:
`EnumItem`(4) · `Link`(4) · `ListItem`(6) · `TermItem`(6) · `Terms`(11) = **31 sites**.

**`Set*` excluídas** (`SetFigureNumbering` 5, `SetEquationNumbering` 16, `SetPage`
8, `SetHeadingNumbering` 62): são marcadores de **set-rule** — a superfície da
StyleChain. O destino delas depende da decisão F (medida na Parte 2); migrá-las
agora desenharia o F por acidente. Registado.

**Locatabilidade**: confirmado que **nenhuma é locatável** (estão na lista
exaustiva não-locatável de `introspect/locatable.rs`; não são `ElementKind`) →
`element_kind`/`to_payload` no default `None`.

**Particularidade da família** (vs Lote 2 math): são **contentores de prosa** →
`map_text` **recurse** no(s) corpo(s) (precedente **Heading**, `e.map_text()`),
não terminal como math. `TermsElem`/`TermItemElem` **sobrepõem `is_empty`**
(`items.is_empty()` / ambos vazios); as outras 3 ficam no default `false`.
`Box<Content>` desboxado para `Content` no `…Elem` (convenção P316).

### Medições (métrica ADR-0104) — vs preditor

| medição | valor | comando |
|---|---|---|
| `content.rs` antes/depois | **5735 → 5700 (−35)** | `wc -l`; diff `+72 / −107` |
| parte atómica (5 módulos novos, c/ testes) | **402 linhas** | `wc -l elements/{list_item,enum_item,link,term_item,terms}.rs` |
| suíte typst-core | **2506 → 2521 (+15)** | só os 15 testes unitários novos (3×5) |
| `crystalline-lint .` | **0 violations** | gate binário |

Custo-por-módulo (linhas, c/ testes): TermItem 86 · Terms 85 · Link 81 ·
EnumItem 78 · ListItem 72. **Preditor validado numa família nova**: custo dominou
∝ largura de uso (31 sites externos), não ∝ tamanho do módulo (homogéneo
~72–86 linhas) — confirma o achado P316/P317 fora da família math.

**Plano de toque (realidade)**: os 31 sites caíram em `introspect.rs`
(materialize_time + walk + match exaustivo), `introspect/locatable.rs` (match
exaustivo + 1 construção), `layout/mod.rs` (5 handlers de layout destructuram
`Arc<Elem>`), `stdlib/structural.rs` (`terms()` construção), `eval/rules.rs`
(1 match já tuple), `eval/tests.rs` (extração Terms). Construções viraram
construtores (`Content::terms`/`term_item`/`list_item`/…); matches viraram
`Content::X(e)` com `e.campo`. **Nenhuma asserção alterada.**

**Construtores**: `list_item`/`enum_item`/`link` preservados (mesma assinatura);
novos `terms(items)`/`term_item(term, description)`.

### Proposta do Lote 4 (derivada da tabela P317 — decisão humana)

Esgotada a família lista/termos, os próximos element-shaped baratos por largura
crescente sugerem duas famílias coerentes (decisão do dono):

- **Decorações de texto** (`body` + cosméticos, recursam como Lote 3):
  `Overline`(10) · `Strike`(10) · `Underline`(31).
- **Quebras/espaços** (leaves/markers): `Linebreak`(11) · `Colbreak`(12) ·
  `Space`(13) · `VSpace`(14) · `HSpace`(18) · `Pagebreak`(22).

Excluir de novo as `Set*` (StyleChain, F) e os primitivos de DEBT-58
(`Sequence`/`Empty`/`Block`/math primitivos).

---

## Parte 2 — Medições pré-F (committada em separado)

Documento: **`00_nucleo/diagnosticos/medicao-pre-f-passo-318.md`** (números +
comandos, **sem recomendação**; alimenta o diagnóstico futuro do F / DEBT 99.E).
Corpus de baseline guardado: `…/medicao-pre-f-passo-318-corpus.typ`.

Os 4 números-síntese:

- **M1 — superfície de propriedades**: teto vanilla ~**283** campos settable
  (163 `#[elem]`); piso cristalino **10** (`Style`/`StyleDelta`) + 4 `Set*`
  (91 sites) + `Styled` (58 sites). Delta ~**273**.
- **M2 — sites a converter**: leitura de propriedade em produção pequena (5
  `Style::` + 10 `StyleDelta`); **104** sites de exaustividade de elemento (a
  trava de lint/teste que o F exige); 52 variantes ainda inline.
- **M3 — baseline de performance**: mediana **0.07 s** (5 runs) sobre corpus de
  7003 linhas (eval+layout+export; release; Ryzen 7 5800H). Corpus no repo para
  o "depois" ser comparável.
- **M4 — censo de scoping**: ~**100** testes fixam o comportamento global (38
  `numbering_active` + 41 `Set*` + 21 `Styled`).

---

## Encerramento

- `git log --oneline` (dois commits isoláveis):
  - `Passo 318 — lote 3`
  - `Passo 318 — medições pré-F`
- `crystalline-lint .` → **0 violations**.
- `git status` limpo (só resta cruft pré-existente de `lab/` quarentena).
- Caveat conhecida: stack default estoura em `recursao_infinita_*` — **não é
  regressão** (correr com `RUST_MIN_STACK=33554432`).

## Fora de escopo (confirmado)

Decidir o F ou o destino das `Set*` (Parte 2 mede; decisão do passo 99.E);
DEBT-58 (gatilho não disparou); Lotes 4+ (instância do modelo); otimização
sugerida pelo M3 (medir ≠ mexer).
