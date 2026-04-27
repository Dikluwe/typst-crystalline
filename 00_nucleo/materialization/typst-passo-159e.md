# Passo P159E — `url` + `doi` em `BibEntry` (refino família 159 fora Bloco A)

Refino estrutural de tipo entity `BibEntry` adicionando 2 fields
adicionais (`url`, `doi`) — par natural identificado em P159D §9
como candidato a sub-passo futuro. **Vigésima aplicação consecutiva
de materialização** desde início da série granular P156C.

**Primeiro sub-passo família 159 fora do Bloco A** do diagnóstico
P159B. Preenche slot P159E reservado fracamente após P158C ter
ocupado o identificador alternativo. Refino paralelo a P159D
(estrutura idêntica; +2 fields Optional<String>).

---

## Estado actual antes de começar

- 63 ADRs após P159F (28 EM VIGOR; ADR-0060 IMPLEMENTADO;
  ADR-0062 reserva sem ficheiro mantida).
- Layout: 78% (inalterado). Cobertura arquitectural total 82%.
- Cobertura Model agregada: ~50% (impl + impl⁺ inalterada).
  Cobertura ampla 77% inalterada.
- Hash actual `entities/content.rs`: `ec58d849` (preservado em
  **15 passos consecutivos** P156L → P159F via L0-baseline).
- Hash `entities/bib_entry.rs`: `5a2c0ebd` (P159A; preservado
  P159D; **quebra esperada em P159E** se interpretação
  L0-baseline mantém — campos novos via doc-comment).
- Hash `entities/citation_form.rs`: `677849cb` (P159C).
- Hash `entities/counter_state.rs`: `4b8e4f02` (P158B/P159C/F).
- 1453 tests (lib+integ+diagnostic; workspace 1482 pós-P159F);
  zero violations linter.
- 58 variants Content; 48 stdlib funcs.
- Padrões consolidados pós-P159F: granularidade N=19;
  inventariar N=21; Smart→Option Caso A patamar N=7
  (43/57 Layout/Model); §análise risco N=21; estabilidade
  hash L0 content.rs N=15; tipo entity em ficheiro próprio
  N=5; **infraestrutura state lookup N=3** (limiar formalização);
  P155 cross-feature N=1; refino tipo entity sem alteração
  Content N=1; refactor de field para Option N=1.

**Decisão de identificador P159E**: slot reservado fracamente
após P158C ter ocupado o alternativo (anteriormente "P158C ou
P159E" no diagnóstico P159B §3.4). Decisão humana pós-P159F:
manter sequência alfabética sem reservar scope; preencher P159E
com próximo sub-passo família 159 que surja.

**Decisão de scope P159E**: url + doi em BibEntry. Par natural
identificado em P159D §9 como "candidato a sub-passo M futuro".

**Política "sem novas reservas" preservada** — P159E não cria
reservas para passos pós-P159E. Outros fields BibEntry restantes
(`editor`/`series`/`note`/`isbn`/`location`/`organization`)
permanecem candidatos NÃO-reservados.

**Leituras prévias obrigatórias**:
- `00_nucleo/materialization/typst-passo-159d-relatorio.md` —
  precedente directo (4 fields adicionais; pattern a replicar).
- `00_nucleo/materialization/typst-passo-159f-relatorio.md` —
  contexto pós-Bloco A.
- `00_nucleo/diagnosticos/diagnostico-bibentry-fields-passo-159d.md`
  §9 — decisão prévia url/doi como par natural.
- `00_nucleo/adr/typst-adr-0054-perfil-graded.md` — fundamento
  para subset minimal (sem URL parsing; sem DOI validation).
- `00_nucleo/adr/typst-adr-0033-paridade-observavel.md` —
  fundamento para plaintext em layout (sem hyperlinks).
- `01_core/src/entities/bib_entry.rs` — struct actual P159A+P159D
  com 8 fields (4 obrigatórios + 4 opcionais).
- `01_core/src/rules/stdlib/structural.rs` — `extract_bib_entries`
  helper actual (P159A+P159D extendido).
- `01_core/src/rules/layout/mod.rs` — `format_bib_entry` helper
  privado P159D.
- `lab/typst-original/crates/typst-library/src/model/bibliography.rs`
  + `hayagriva::Entry` (vanilla, quarentena) — referência para
  url/doi semântica.

---

## Natureza do passo

**Tamanho**: M.

**Justificação**: 1 feature (par natural url+doi). Pattern
idêntico a P159D — 2 fields Optional<String>. Modificação
trivial em `extract_bib_entries` para parse de fields novos
(reuso do helper inline `optional_str` já criado em P159D).
Modificação em `format_bib_entry` para concatenação condicional
APA-like. Tests ~5-8.

**P159D era S+** com 4 fields; **P159E é M** com 2 fields. Por
quê M e não S+? Porque url/doi têm decisões arquitecturais-chave
deferidas a .1:
- Ordem no layout (antes ou depois de pages? antes ou depois
  de publisher?).
- Separadores (URL: `https://...` literal? `[URL]` placeholder?
  itálico em alguns styles?).
- DOI prefix (`doi:10.1234/...` vs `https://doi.org/10.1234/...`
  vs `10.1234/...` literal).

Decisões cosméticas mas com paridade vanilla a confirmar.

Granularidade preservada: 1 feature (par natural) → mantém
N=19 do padrão.

**Risco baixo**:
- Pattern idêntico a P159D (validado N=1).
- ADR-0064 NÃO aplicável (Optional<String> directo).
- Tests pré-existentes (P159D) preservam-se via fields novos
  default `None`.
- Decisões cosméticas em .1 sem impacto estrutural.

---

## Decisões já tomadas

- **2 fields adicionais Optional<String>**:
  ```rust
  pub struct BibEntry {
      // P159A:
      pub key:       String,
      pub author:    String,
      pub title:     String,
      pub year:      u32,
      // P159D:
      pub volume:    Option<String>,
      pub pages:     Option<String>,
      pub journal:   Option<String>,
      pub publisher: Option<String>,
      // P159E:
      pub url:       Option<String>,
      pub doi:       Option<String>,
  }
  ```
  Total 10 fields (4 obrigatórios + 6 opcionais).

- **Tipos `Option<String>`**: paridade P159D. Sem URL parsing
  per ADR-0054 graded; sem DOI validation per ADR-0054 graded.

- **Default `None`**: paridade P159D. Backwards compat trivial
  via fields novos default `None` — `BibEntry::new(...)`
  original com 4 args + builder fluente P159D continua a
  funcionar.

- **Builder pattern fluente extendido**:
  ```rust
  impl BibEntry {
      pub fn with_url(mut self, url: impl Into<String>) -> Self {
          self.url = Some(url.into());
          self
      }
      pub fn with_doi(mut self, doi: impl Into<String>) -> Self {
          self.doi = Some(doi.into());
          self
      }
  }
  ```
  Paridade pattern P159D.

- **Helper `extract_bib_entries` extendido**:
  - Reuso do helper inline `optional_str` privado (P159D).
  - Adicionar 2 fields opcionais `url` e `doi` à parsing
    chain.
  - Validação: `Value::Str` esperado se presente; outros
    tipos rejeitados.

- **Layout `format_bib_entry` extendido**:
  - Concatenação condicional APA-like.
  - **Ordem deferida a .1**. Pré-decisão: url depois de
    publisher; doi depois de url.
  - **Formato deferida a .1**. Pré-decisão: url plaintext
    (`https://...`); doi prefixo `doi:` (`doi:10.1234/...`).

## Decisões diferidas

- **Ordem url/doi no layout**: a decidir em .1. Pré-decisão:
  `[key] author. title journal vol. volume, pp. pages.
  publisher (year). url, doi:DDDD.`.

- **Formato url**: plaintext `https://...` vs `[URL]`
  placeholder. Pré-decisão: plaintext.

- **Formato doi**: prefixo `doi:` vs `https://doi.org/...`
  vs literal. Pré-decisão: prefixo `doi:` per paridade APA.

- **Hyperlinks no output**: NÃO suportados em P159E per
  ADR-0054 graded. Plaintext simples. Hyperlinks dependem
  de Layout/PDF infrastructure cross-módulo (provavelmente
  Bloco C).

- **URL validation**: NÃO em P159E (Value::Str literal aceite).
  Validation seria refino futuro (Bloco B com hayagriva ou
  refactor dedicado).

- **DOI validation**: NÃO em P159E (formato DOI tem regex
  específica `10.\d{4,}/...`; validation seria refino).

- **Restantes fields BibEntry vanilla** (`editor`/`series`/
  `note`/`isbn`/`location`/`organization`): NÃO reservados.
  Candidatos a refinos futuros se prioritários.

---

## Sub-passos

### .1 Inventário pré-materialização (obrigatório per ADR-0065)

Diagnóstico em
`00_nucleo/diagnosticos/diagnostico-bibentry-url-doi-passo-159e.md`
com 7 itens canónicos (ADR-0034) + 2 itens específicos para
ordem layout + formato:

1. Assinatura vanilla `hayagriva::Entry` para `url` e `doi` —
   confirmar que são fields Optional<String> (não estruturas
   complexas).
2. Comportamento observable vanilla (URL e DOI como hyperlinks
   em PDF; plaintext em outros formatos — confirmar paridade
   ADR-0033).
3. ADR-0064 caso aplicável: NÃO directamente (Optional<String>
   directo, paridade P159D).
4. Variants Content existentes a estender: nenhum. `BibEntry`
   refino estrutural (paridade P159D).
5. Helpers stdlib reusáveis: `optional_str` inline P159D
   (helper privado parametrizado N=2 usos no mesmo passo
   pattern P159D — agora N=4 usos cumulativos).
6. Limitações aceites (sem URL/DOI validation; sem hyperlinks;
   plaintext simples).
7. Tests planeados (constructor com fields novos + parse stdlib
   + render layout extendido + regression P159D sem fields —
   range 5-8 per padrão P159D).
8. **(Específico ordem layout)** Confirmar paridade vanilla
   APA para ordem de url/doi:
   - Pré-decisão: url depois de publisher; doi depois de url.
   - Alternativa: doi antes de url (alguns styles).
   - Decisão final em .1 com justificação.
9. **(Específico formato)** Confirmar prefixos:
   - URL: plaintext `https://...` (paridade vanilla minimal).
   - DOI: `doi:10.1234/...` per paridade APA (alternativa:
     `https://doi.org/10.1234/...` URL completa).
   - Decisão final em .1 com justificação.

### .2 Expandir struct `BibEntry`

`01_core/src/entities/bib_entry.rs`:
- Adicionar 2 fields Optional<String> (`url`, `doi`).
- Builder pattern extendido com `with_url()` e `with_doi()`.
- Constructor `new(...)` original inalterado (paridade P159D).
- Derives mantidos: `Debug`, `Clone`, `PartialEq`, `Eq`.

### .3 Extender `extract_bib_entries`

`01_core/src/rules/stdlib/structural.rs`:
- Helper `optional_str(field)` reusado para `url` e `doi`.
- Validação tipo `Value::Str`; outros tipos rejeitados com
  diagnóstico claro mencionando field específico.
- Constructor `BibEntry::new(...).with_url(...).with_doi(...)`
  via builder fluente (ou atribuição directa per padrão P159D).

### .4 Refinar layout `format_bib_entry`

`01_core/src/rules/layout/mod.rs`:
- Concatenação condicional para url e doi:
  - Se `url` presente: append `" {url}"` (formato decidido em .1).
  - Se `doi` presente: append `", doi:{doi}"` (formato decidido
    em .1).
- Ordem APA-like: depois de publisher, antes de `(year)`?
  Ou antes do final? Decidido em .1.

### .5 Tests

- **Unit tests `BibEntry`** em `entities/bib_entry.rs` (~3):
  - Constructor com url/doi via builder.
  - PartialEq cobre 10 fields agora.
  - Backwards compat: `new(4 args)` continua a funcionar
    (regression P159A+P159D).

- **Stdlib tests** em `stdlib/mod.rs` (~3):
  - Parse com url/doi.
  - Parse sem url/doi (regression P159D).
  - Tipo errado em url/doi rejeitado.

- **Layout E2E tests** em `layout/tests.rs` (~2):
  - Bibliography com entry completa (incluindo url/doi)
    renderiza formato extendido.
  - Bibliography com entry mínima (sem url/doi) renderiza
    formato P159D original (regression).

**Δ esperado**: +5-8 tests (paridade P159D Δ=+8).

### .6 Propagação de hashes

`crystalline-lint --fix-hashes .`:
- `bib_entry.rs` hash: per regra L0-baseline preservado se
  prompt `bib_entry.md` não mencionar fields individualmente
  (paridade P159D resultado).
- Outros ficheiros: refactor interno; preserva L0.

**Esperado "Nothing to fix" se interpretação L0 mantém** —
lição P159A/C/D/P158C/F internalizada e aplicada
conscientemente. Refactor de tipo entity sem alteração de
prompt L0 preserva hash.

---

## Verificação

Numerada para reporte de conclusão:

1. `cargo test` em workspace: **1482 + Δ** tests, zero falhas
   (Δ esperado +5-8).
2. `crystalline-lint`: zero violations.
3. Contagem variants Content: **58** (inalterada — refino
   tipo entity).
4. Contagem stdlib funcs: **48** (inalterada — `native_bibliography`
   modificada via helper extendido).
5. **Hash `entities/content.rs` preservado** `ec58d849` —
   **16º passo consecutivo** via L0-baseline interpretation.
6. Hash `entities/bib_entry.rs` per L0-baseline (preservado
   se prompt L0 não modificado, paridade P159D).
7. Decisão sobre ordem layout (url/doi antes/depois de
   publisher/year) documentada no relatório §"Decisões
   tomadas em .1".
8. Decisão sobre formato (URL plaintext; DOI `doi:` prefix)
   documentada com justificação per paridade APA.
9. **Sem novas reservas** criadas (paridade política
   P158/P159).
10. Tests pré-existentes Bibliography (P159A+P159D) passam
    inalterados — fields novos default None produz output
    P159D original.
11. `layout_grid` original NÃO modificado (paridade P157A/B/C
    + P159A/C/D/F).
12. Restantes fields BibEntry vanilla (`editor`/`series`/
    `note`/`isbn`/`location`/`organization`) NÃO materializados
    (NÃO reservados; candidatos futuros).

---

## Critério de conclusão

- Verificações 1-12 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-159e-relatorio.md`
  produzido com:
  - Resumo do executado.
  - Confirmação de cada verificação.
  - **§análise de risco** (padrão N=21 → 22).
  - Slope cumulativo Model (mesa P155-P159E).
  - ADR-0061 §"Aplicações cumulativas" anotada com P159E
    (preenche slot P159E reservado fracamente).
  - **Confirmação**: pattern P159D replicado fielmente; helper
    `optional_str` N=2 → 4 usos cumulativos.
  - **Decisão de ordem layout + formato registadas** com
    justificação para refinos futuros (e.g. P159 outro
    sub-passo adiciona editor/series/etc.).

---

## O que pode sair errado

**Cenários gerais**:
- Inventário .1 revela que vanilla usa `url: Option<Url>`
  estruturado (não `Option<String>`) → simplificar para
  `Option<String>` per ADR-0054 graded; documentar.
- Inventário .1 revela que DOI vanilla é tipo dedicado
  (`Doi`) com validation regex → simplificar para
  `Option<String>` literal; documentar.

**Cenários específicos**:
- Layout formato extendido produzir output muito longo em
  uma linha → refactor multi-line diferido (depende
  multi-region DEBT-56).
- Tests pré-existentes (P159D) esperarem formato exacto
  conflitante → backwards compat preservada via fields novos
  default None; sem ajuste de tests existentes esperado.
- `optional_str` helper precisar de variante alargada para
  validation URL/DOI → manter trivial; validation diferida
  per ADR-0054 graded.
- L0-baseline NÃO preservar hash `bib_entry.rs` se prompt
  for actualizado por inadvertência → reconhecer e
  documentar; quebra excepcional não bloqueante.

---

## Notas operacionais

- **Vigésima aplicação de materialização**. Patamar empírico
  forte. Sem reformulação esperada.
- **§análise de risco no relatório** com peso baixo (refino
  pattern idêntico a P159D validado). Vigésima segunda
  aplicação consecutiva preserva precedente.
- **Pattern P159D replicado**: 2º refino estrutural de tipo
  entity sem alteração ao variant Content. Subpadrão #16
  cresce N=1 → 2 (P159D BibEntry 4 fields + **P159E BibEntry
  2 fields**). Precedente consolida-se.
- **Helper `optional_str` cumulativo**: P159D N=2 usos no mesmo
  passo + **P159E N=2 usos no mesmo passo** = N=4 cumulativos.
  Promoção a `pub(super)` ou helper público diferida per
  política consistente N=3-4 (atinge limiar; reavaliar em
  passo administrativo XS futuro NÃO reservado).
- **ADR-0064 NÃO aplicável**: fields são `Option<String>`
  directos sem mapping `Smart<T>`. Pattern Optional trivial.
- **P159E preenche slot reservado fracamente** após P158C
  ter ocupado identificador alternativo (decisão diagnóstico
  P159B §3.4 "P158C ou P159E"). Identificador preserva sequência
  alfabética sem violar política "sem novas reservas" (que se
  refere a scope, não identificadores).
- **Sem decisão arquitectural-chave**: refino paralelo a P159D;
  decisões cosméticas (ordem/formato) em .1 sem impacto
  estrutural.

---

## Pós-passo

Após conclusão de P159E:

**Layout fica em 78% inalterado**. **Model agregado fica em
~50% inalterado** (refino tipo entity). **Hash `entities/content.rs`
provavelmente preservado** (16º passo consecutivo via
L0-baseline).

**Próxima decisão** (sem candidata pré-acordada — política
"sem novas reservas"):

- **Restantes fields BibEntry vanilla** (`editor`/`series`/
  `note`/`isbn`/`location`/`organization`): NÃO reservados.
  Candidatos a refino M futuro se prioritário.
- **ADR-0062-create** — XS administrativo; desbloqueia Bloco B.
- **Bloco B (hayagriva)**: P159G (Cargo.toml + crystalline.toml
  hayagriva) após ADR-0062 PROPOSTO.
- **Bloco C (cross-módulo)**: refactor multi-region L+
  (DEBT-34e + DEBT-56) ou Introspection P160.
- **Refinos Model fora Bloco A continuação** (mais langs em
  `figure_supplement_for_lang`; refactor `kind` em outros
  variants Content; etc.).
- **Mudança de módulo**: Layout Fase 3 (columns/colbreak) ou
  Introspection P160.
- **Passos administrativos XS**: actualizar L0 prompt
  `content.md` mencionando variants Bibliography/Cite/Cite.form;
  promover ADR-0060 a R1; ADR meta saturação ADR-0064; **ADR
  meta subpadrão #15 (infraestrutura state lookup; agora N=3
  atinge limiar)**; promoção `optional_str` a helper público
  (N=4 cumulativos).

ADR-0060 mantém-se `IMPLEMENTADO`. ADR-0061 mantém-se
`PROPOSTO`. ADR-0062 mantém-se reserva sem ficheiro.

Padrão granularidade 1-2 features/passo (N=19 com P159E se
fechar sem reformulação) **NÃO** é formalizado em ADR. Continua
candidato.

**Pausa natural após P159E — BibEntry com 10 fields (4
obrigatórios + 6 opcionais); slot P159E preenchido com par
natural url+doi; pattern P159D replicado fielmente. Decisão
humana sobre próxima direcção tem máxima informação.**
