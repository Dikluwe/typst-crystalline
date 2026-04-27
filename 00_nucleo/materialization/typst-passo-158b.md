# Passo P158B — Supplement automático por lang em figure (Model figure-kinds sub-passo 2)

Segundo sub-passo substantivo de Model figure-kinds per
recomendação primária diagnóstico P159B §6. Materializa
**supplement automático por lang** — figure renderiza prefix
localizado ("Figure"/"Figura"/"Abbildung"/etc.) consoante
lang activo. **Décima quinta aplicação consecutiva de
materialização** desde início da série granular P156C.

Refino comportamental análogo a P158A (auto-detect kind), mas
agora sobre prefix de label em vez de detecção de kind. Reuso
do padrão consolidado `localize_quotes(lang)` em
`rules/lang/quotes.rs` (P155).

**Sem dependência cruzada hard**. **Hash `content.rs` preservado**
(11º passo consecutivo se confirmado). **Sem alteração a variants
Content**.

---

## Estado actual antes de começar

- 63 ADRs após P159B (28 EM VIGOR; ADR-0060 IMPLEMENTADO;
  ADR-0062 reserva sem ficheiro).
- Layout: 78% (inalterado). Cobertura arquitectural total 82%.
- Cobertura Model agregada: ~50% (impl + impl⁺ inalterada).
- Hash actual `entities/content.rs`: `ec58d849` (preservado em
  10 passos consecutivos com interpretação L0-baseada).
- Hash `bib_entry.rs`: `5a2c0ebd` (P159A).
- 1412 tests (lib+integ+diagnostic; workspace 1434); zero
  violations linter.
- 58 variants Content; 48 stdlib funcs.
- Padrões consolidados pós-P159B: granularidade N=14;
  inventariar N=15; Smart→Option Caso A patamar N=5; §análise
  risco N=15; estabilidade hash L0 N=10.

**Diagnóstico P159B** (recomendação primária §6 + listagem
completa Bloco A §3.1):
- Refino: `infer_kind_from_body` produz kind (image/table/raw);
  novo helper `lookup_supplement(kind, lang)` produz prefix
  localizado.
- Sem dependência cruzada hard.
- Hash `content.rs` preservado (módulo novo + modificação
  introspect).
- Tests Δ: +12-15.
- Granularidade: M preservado.

**Política "sem novas reservas" preservada** — refinos
pós-P158B permanecem candidatos NÃO-reservados.

**Leituras prévias obrigatórias**:
- `00_nucleo/diagnosticos/diagnostico-expansao-159-passo-159b.md`
  — §3.1 esboço P158B.
- `00_nucleo/materialization/typst-passo-158a-relatorio.md` —
  precedente direto (auto-detect kind; padrão de refino
  comportamental).
- `00_nucleo/materialization/typst-passo-155-relatorio.md` —
  padrão `localize_quotes(lang)` reusável.
- `01_core/src/rules/lang/quotes.rs` (P155) — padrão de
  helper de localização a replicar.
- `01_core/src/rules/stdlib/figure_image.rs` — `native_figure`
  + `infer_kind_from_body` actuais (P158A).
- `01_core/src/rules/introspect.rs` — counters por kind +
  formato de label.
- `lab/typst-original/crates/typst-library/src/model/figure.rs`
  + supplement automático em vanilla — referência para
  comportamento.

---

## Natureza do passo

**Tamanho**: M.

**Justificação**: 1 feature (supplement por lang). Helper novo
~20 linhas + modificação trivial em formato de label em
`introspect.rs`. Tests ~12-15. Sem alteração a variants Content.

Granularidade preservada: 1 feature → mantém N=15 do padrão.

**Risco baixo**:
- Reuso de padrão consolidado (P155 `localize_quotes`).
- Sem alteração estrutural.
- Hash `content.rs` preservado.
- Sem decisões arquiteturais-chave (decisões delegadas a .1
  são comportamentais, não estruturais).

---

## Decisões já tomadas

- **Helper novo `figure_supplement_for_lang`**:
  ```rust
  // 01_core/src/rules/lang/figure_supplement.rs (ficheiro novo)
  pub fn figure_supplement_for_lang(kind: &str, lang: &str) -> String {
      match (kind, lang) {
          ("image", "pt") => "Figura".to_string(),
          ("image", "en") => "Figure".to_string(),
          ("image", "de") => "Abbildung".to_string(),
          ("image", "fr") => "Figure".to_string(),
          ("image", "es") => "Figura".to_string(),
          ("image", "it") => "Figura".to_string(),
          ("table", "pt") => "Tabela".to_string(),
          ("table", "en") => "Table".to_string(),
          ("table", "de") => "Tabelle".to_string(),
          ("table", "fr") => "Tableau".to_string(),
          ("table", "es") => "Tabla".to_string(),
          ("table", "it") => "Tabella".to_string(),
          ("raw", "pt") => "Listagem".to_string(),
          ("raw", "en") => "Listing".to_string(),
          ("raw", "de") => "Listing".to_string(),
          ("raw", "fr") => "Listing".to_string(),
          ("raw", "es") => "Listado".to_string(),
          ("raw", "it") => "Listato".to_string(),
          _ => figure_supplement_for_lang(kind, "en"),  // fallback EN
      }
  }
  ```
  Mapeamento mínimo: 3 kinds × 6 langs = 18 entradas + fallback
  EN. Outros kinds e outros langs caem no fallback.

- **Localização**: `rules/lang/figure_supplement.rs` ficheiro
  novo per padrão `quotes.rs` (P155). `pub mod figure_supplement`
  em `rules/lang/mod.rs`.

- **Modificação em `introspect.rs`**: formato de label de
  figure passa a usar `figure_supplement_for_lang(kind, lang)`
  em vez de prefix hardcoded ou ausente.

- **Lang resolution**: lang vem do contexto activo (provavelmente
  `ctx.lang()` ou equivalente — confirmar em .1).

- **Sem alteração a variants Content**: Figure variant inalterado;
  refino vive em `introspect.rs` no momento de gerar label.

- **Sem alteração a stdlib `native_figure`**: `kind` continua
  resolvido como antes (P158A); supplement aplica-se em walk.

- **Fallback EN**: kinds desconhecidos ou langs desconhecidos
  caem em EN per ADR-0033 (paridade observable mínima).

## Decisões diferidas

- **Mais langs além das 6 minimais**: NÃO reservadas. Candidato
  a refino futuro (XS por par lang/kind adicional).

- **Kinds custom além de image/table/raw**: caem no fallback
  EN. Suporte explícito NÃO reservado.

- **Numbering format integrado com supplement** (e.g. "Figura
  3:" vs "Figura 3."): a decidir em .1 conforme padrão vanilla
  e tests existentes.

- **Promoção de `figure_supplement_for_lang` a helper público**:
  diferida per política consistente N=3-4 (sem reuso até agora).

---

## Sub-passos

### .1 Inventário pré-materialização (obrigatório per ADR-0065)

Diagnóstico em
`00_nucleo/diagnosticos/diagnostico-figure-supplement-passo-158b.md`
com 7 itens canónicos (ADR-0034) + 2 itens específicos para
i18n e padrão de localização:

1. Assinatura vanilla `FigureElem.supplement` — confirmar se
   é Smart<Content> ou String localizada; comportamento
   default.
2. Comportamento observável (label "Figure 1" / "Figura 1" /
   "Abbildung 1" consoante lang; numbering format integrado
   com supplement).
3. ADR-0064 caso aplicável: NÃO directamente em P158B (kind
   continua String; lang é parâmetro contextual).
4. Variants Content existentes a estender (nenhuma; refino
   comportamental apenas).
5. Helpers stdlib reusáveis: padrão `localize_quotes` (P155);
   helper novo `figure_supplement_for_lang`.
6. Limitações aceites (mapeamento mínimo 6 langs × 3 kinds;
   custom kinds caem no fallback EN; numbering format
   diferido se complexo).
7. Tests planeados (lookup por par lang/kind + fallback +
   integração com label format — range 12-15 per esboço P159B
   §3.1).
8. **(Específico i18n)** Confirmar como lang é resolvido no
   contexto: `ctx.lang()` directo? `Style::lang` cascade?
   Verificar em código.
9. **(Específico padrão P155)** Confirmar estrutura exacta
   de `localize_quotes(lang)` e replicar pattern para
   `figure_supplement_for_lang(kind, lang)`.

### .2 Adicionar helper `figure_supplement_for_lang`

`01_core/src/rules/lang/figure_supplement.rs` (ficheiro novo):
- Adicionar helper per assinatura em §"Decisões já tomadas".
- 18 entradas hardcoded (3 kinds × 6 langs) + fallback EN.
- `pub mod figure_supplement;` em `rules/lang/mod.rs`.

### .3 Modificar formato de label em `introspect.rs`

`01_core/src/rules/introspect.rs`:
- Localizar formato de label actual de figure (provavelmente
  hardcoded ou prefix-less).
- Substituir por `figure_supplement_for_lang(kind, lang)`
  seguido do número counter.
- Preservar formato exacto de separador (`:`, `.`, ` `) per
  decisão em .1.

### .4 Tests

- **Helper tests** em `rules/lang/figure_supplement.rs`
  (~5):
  - Lookup `image/pt` → "Figura".
  - Lookup `table/de` → "Tabelle".
  - Lookup `raw/it` → "Listato".
  - Fallback `image/zh` → "Figure" (EN fallback).
  - Fallback `custom-kind/en` → "Figure" (EN fallback de kind
    desconhecido — confirmar comportamento em .1; pode ser
    string vazia ou kind capitalizado).

- **Integração tests** em `stdlib/mod.rs` ou
  `introspect_tests.rs` (~7-10):
  - `figure_label_pt_image` — `figure(image(...))` com
    lang=pt produz label "Figura 1".
  - `figure_label_en_table` — `figure(table(...))` com
    lang=en produz label "Table 1".
  - `figure_label_de_raw` — `figure(raw(...))` com lang=de
    produz label "Listing 1".
  - `figure_label_fallback_lang_unknown` — lang=zh produz
    label EN.
  - `figure_label_explicit_kind_override` — `figure(image(...),
    kind: "custom")` produz label EN (fallback para custom).
  - `figure_multiple_figures_numbering_independente` — counter
    por kind continua a funcionar (regression P157A).
  - `figure_explicit_supplement_override_diferido` — verificar
    que field explícito `supplement:` ainda não é suportado
    (decisão NÃO reservada).

- **Δ esperado**: +12-15 tests (alinhado com esboço P159B
  §3.1).

### .5 Propagação de hashes

`crystalline-lint --fix-hashes .` para gerar hash inicial de
`figure_supplement.rs` novo.

`entities/content.rs` NÃO modificado em P158B — hash
`ec58d849` deve permanecer (11º passo consecutivo a preservar).

`introspect.rs` é modificado — verificar se há prompt L0
correspondente que precise de hash refresh.

---

## Verificação

Numerada para reporte de conclusão:

1. `cargo test` em workspace: **1412 + Δ** tests, zero falhas
   (Δ esperado +12-15).
2. `crystalline-lint`: zero violations.
3. Contagem variants Content: **58** (inalterada — refino).
4. Contagem stdlib funcs: **48** (inalterada — refino; helper
   é privado/módulo, não stdlib func nova).
5. Cobertura Model agregada: ~50% (inalterada — refino
   qualitativo). Tabela cobertura pode ganhar nota qualitativa
   em entrada `figure` se aplicável.
6. Hash `figure_supplement.rs` gerado.
7. **Hash `entities/content.rs` permanece `ec58d849`** —
   **11º passo consecutivo** a preservar contrato L0 do variant
   Content.
8. Lang resolution decidido em .1 documentado no relatório.
9. Padrão P155 `localize_quotes` confirmado como reusado em
   estrutura.
10. **Sem novas reservas** criadas (paridade política P158).
11. Tests pré-existentes de figure label continuam a passar
    inalterados (regression P157A counters por kind).

---

## Critério de conclusão

- Verificações 1-11 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-158b-relatorio.md`
  produzido com:
  - Resumo do executado.
  - Confirmação de cada verificação.
  - **§análise de risco** (padrão N=15 → 16; primeira
    aplicação i18n em domínio Model figure).
  - Slope cumulativo Model (mesa P155-P158B).
  - ADR-0061 §"Aplicações cumulativas" anotada com P158B
    (slope Layout "—"; nota cross-domínio refino qualitativo).
  - **Confirmação**: padrão P155 `localize_quotes` reusado
    cross-feature (quotes → figure supplement); estabilidade
    hash content.rs N=10 → 11.

---

## O que pode sair errado

**Cenários gerais**:
- Inventário .1 revela que vanilla supplement é
  `Smart<Content>` (não String) → ajustar tipo de retorno do
  helper para `Content`; documentar.
- Inventário .1 revela que lang resolution é não-trivial (e.g.
  Style::lang cascade complexo) → simplificar para `ctx.lang()`
  ou fallback default; documentar como graded per ADR-0054.

**Cenários específicos**:
- Tests pré-existentes de figure label esperarem prefix
  específico hardcoded → ajustar tests para usar lang
  explícito; documentar regression.
- Numbering format vanilla integrar supplement de forma
  complexa (e.g. "Figure 1:" com colon obrigatório) → seguir
  paridade vanilla per ADR-0033; ajustar formato.
- Formato de label em `introspect.rs` ser usado em múltiplos
  sítios além de figure (e.g. heading, equation) → limitar
  modificação a figure path; documentar.
- Helper `figure_supplement_for_lang` colidir com estrutura
  existente em `rules/lang/` → renomear ou ajustar
  organização; .1 confirma.

---

## Notas operacionais

- **Décima quinta aplicação de materialização**. Patamar
  empírico forte. Sem reformulação esperada.
- **§análise de risco no relatório** com peso baixo (refino
  comportamental sem refactor estrutural). Décima sexta
  aplicação consecutiva — preserva precedente.
- **Helper `figure_supplement_for_lang` N=1**. Sem candidato
  a reuso até agora; promoção diferida.
- **Estabilidade hash content.rs N=10 → 11**: 11º passo
  consecutivo a preservar contrato L0 do variant Content.
  Padrão consolida-se a maturidade.
- **Política "sem novas reservas" preservada**. P158B não
  cria reservas para passos pós-P158B. Refinos i18n adicionais
  (mais langs, mais kinds) permanecem candidatos NÃO-reservados.
- **Reuso padrão P155 cross-feature**: primeiro reuso explícito
  de `localize_quotes` pattern em diferente feature (quotes
  → figure). Subpadrão emergente "padrão P155 i18n reusado"
  candidato a registo se outro feature reusar (e.g. table
  caption supplement futuro).
- **Refino qualitativo, não materialização nova**. Funcionalmente
  importante (figure ganha label localizado) mas estruturalmente
  é só helper privado + uma linha modificada em introspect.

---

## Pós-passo

Após conclusão de P158B:

**Layout fica em 78% inalterado**. **Model agregado fica em
~50% inalterado** (refino qualitativo). **Hash
`entities/content.rs` provavelmente preservado** (11º passo
consecutivo).

**Próxima decisão** (sem candidata pré-acordada — política
"sem novas reservas"):

- Continuar Bloco A do diagnóstico P159B (P159C/D/F
  candidatos).
- Passo administrativo XS: criar ADR-0062 PROPOSTO
  (independente — desbloqueia Bloco B futuro a qualquer
  momento).
- Mudar de módulo (Introspection P160).
- Outras direcções pendentes (continua lista das ~12
  candidatas remanescentes).

ADR-0060 mantém-se `IMPLEMENTADO`. ADR-0061 mantém-se
`PROPOSTO`. ADR-0062 mantém-se reserva sem ficheiro.

Padrão granularidade 1-2 features/passo (N=15 com P158B se
fechar sem reformulação) **NÃO** é formalizado em ADR. Continua
candidato.

**Pausa natural após P158B — refino i18n de figure materializado;
padrão P155 reusado cross-feature; estabilidade hash content.rs
atinge 11 passos consecutivos. Decisão humana sobre próxima
direcção tem máxima informação.**
