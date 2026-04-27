# Passo P158A — auto-detecção de `kind` em `native_figure` (Model figure-kinds sub-passo 1)

Primeiro sub-passo substantivo de Model figure-kinds per scope
decidido em diagnóstico P158 §3.2. Materializa subset MÍNIMO:
helper privado `infer_kind_from_body` + modificação trivial de
`native_figure` para fallback chain (`kind explícito > infer >
"image"`). **Décima terceira aplicação consecutiva de
materialização** desde início da série granular P156C.

Refino qualitativo de infraestrutura existente (`Content::Figure`
e counters por kind já funcionais desde P75/ADR-0041 + P157A).
Sem alteração a variants Content; sem alteração a layout ou
introspect.

Target pós-passo: cobertura Model agregada ~50% inalterada
(refino qualitativo). Funcionalmente: `figure(image(...))`
activa counter de imagem automaticamente sem `kind:` manual.

---

## Estado actual antes de começar

- 63 ADRs após P158 (28 EM VIGOR; ADR-0060 IMPLEMENTADO).
- Layout: 78% (inalterado). Cobertura arquitectural total 80%.
- Cobertura Model agregada: ~50% (inalterada em P158
  documental).
- Hash actual `entities/content.rs`: `ec58d849` (preservado
  P156L → P157 → P157A → P157B → P157C → P158 — 6 passos
  consecutivos).
- 1379 tests (lib+integ+diagnostic; workspace 1401); zero
  violations linter.
- 56 variants Content; 46 stdlib funcs.
- Padrões consolidados pós-P158: granularidade N=12;
  inventariar N=11; Smart→Option N=9 (saturação cross-domínio
  cross-caso); §análise risco N=11.

**Diagnóstico P158** confirmou:
- `Content::Figure { body, caption, kind: String, numbering:
  Option<String> }` existente desde P75/ADR-0041.
- `Content::Image` (P71), `Content::Table` (P157A),
  `Content::Raw` todos implementados.
- Counters por kind funcionais via
  `local_figure_counters.entry(kind).or_insert(0)`.
- `native_figure` aceita `kind: Str` arbitrário; default
  `"image"`.
- **Falta**: auto-detecção de kind baseada no body.

**Política "sem novas reservas" estabelecida em P158**
mantida — P158A não cria reservas para passos pós-P158A.

**Leituras prévias obrigatórias**:
- `00_nucleo/diagnosticos/diagnostico-model-figure-kinds-passo-158.md`
  — §§1-§5 (esboço P158A em §5 + subset minimal §3.2).
- `00_nucleo/adr/typst-adr-0041-*.md` — ADR base de
  `Content::Figure` (P75).
- `01_core/src/entities/content.rs` — variant `Content::Figure`
  para confirmação estrutural pré-execução.
- `01_core/src/rules/stdlib/figure_image.rs` (módulo declarado
  no diagnóstico §5) — `native_figure` actual.
- `01_core/src/rules/introspect.rs` linhas 279-292 — counters
  por kind para confirmação de não-impacto.
- `lab/typst-original/crates/typst-library/src/model/figure.rs`
  (vanilla, quarentena) — código de referência para auto-detecção.

---

## Natureza do passo

**Tamanho**: S+ ou M- (a confirmar em sub-passo .1 conforme
descobertas).

**Justificação**: 1 feature granular (auto-detecção). Helper
privado ~10 linhas. Modificação trivial em `native_figure`
(uma linha de fallback chain). Tests ~6-8. Sem alteração a
variants, layout, introspect.

Granularidade preservada: 1 feature → mantém N=13 do padrão.

**Risco baixo**:
- **Baixo** porque é refino qualitativo aditivo sem refactor
  de tipo.
- Reuso de infraestrutura existente (Figure variant, counters
  por kind, kind: String arbitrário).
- Sem decisões arquitecturais-chave (decisões delegadas a .1
  são comportamentais, não estruturais).

---

## Decisões já tomadas

- **Helper privado novo**:
  ```rust
  fn infer_kind_from_body(body: &Content) -> Option<String> {
      match body {
          Content::Image { .. } => Some("image".to_string()),
          Content::Table { .. } => Some("table".to_string()),
          Content::Raw { .. }   => Some("raw".to_string()),
          _ => None,
      }
  }
  ```
  Localização: `01_core/src/rules/stdlib/figure_image.rs`
  (privado, mesmo módulo de `native_figure`).

- **Fallback chain em `native_figure`**:
  ```rust
  let kind = args.named.get("kind")
      .and_then(|v| if let Value::Str(s) = v { Some(s.to_string()) } else { None })
      .or_else(|| infer_kind_from_body(&body))  // P158A novo
      .unwrap_or_else(|| "image".to_string());
  ```
  Precedência preservada: `kind` explícito > inferência >
  default `"image"`.

- **Default `"image"` preservado** para compatibilidade com
  tests existentes.

- **Sem alteração a `Content::Figure`**: estrutura inalterada
  (`kind: String` continua directo, não `Option<String>`).
  Refactor para Option diferido (NÃO reservado per política).

- **Sem alteração a layout ou introspect**: counters por kind
  continuam a funcionar inalterados — o refino é só na origem
  do valor `kind` antes de o passar a `Content::Figure`.

- **Promoção de helper a `pub(super)` ou público**: diferida
  per política consistente N=3-4 (helper privado N=1 actual).

## Decisões diferidas

- **Sequence handling**: comportamento de
  `figure(Sequence([Image, ...]))` — detectar `Image`
  recursivamente vs limitar a body directo. **Decisão delegada
  a sub-passo .1** com inventário do que vanilla faz e do que
  os tests existentes esperam.

- **Custom kinds detectáveis**: vanilla pode suportar custom
  kinds via `kind: "minha-coisa"` mas auto-detecção limita-se
  a Image/Table/Raw. Sem suporte para auto-detecção custom em
  P158A.

- **Refactor `kind: String → Option<String>`** per ADR-0064
  Caso A: **NÃO reservado**. Candidato a passo futuro se
  prioritário.

- **Supplement automático** (prefix por lang): **NÃO reservado**
  per política. Candidato a passo M futuro.

---

## Sub-passos

### .1 Inventário pré-materialização (obrigatório per ADR-0065)

Diagnóstico em
`00_nucleo/diagnosticos/diagnostico-figure-auto-detect-passo-158a.md`
com 7 itens canónicos (ADR-0034) + 2 itens específicos para
refino comportamental:

1. Assinatura vanilla `FigureElem` para auto-detecção — confirmar
   se vanilla detecta kind do body e como (recursive vs direct;
   Image/Table/Raw; outros).
2. Comportamento observável do fallback chain (`kind` explícito
   tem precedência absoluta; auto-detecção activa só se `kind`
   ausente; default `"image"` se nenhum dos anteriores).
3. ADR-0064 caso aplicável — **não aplicável** em P158A
   (kind continua String directo). Aplicação futura potencial
   em refactor não reservado.
4. Variants Content existentes a estender (nenhuma; refino
   stdlib apenas).
5. Helpers stdlib reusáveis: nenhum directo (parse trivial);
   helper novo `infer_kind_from_body` privado.
6. Limitações aceites (custom kinds não auto-detectados;
   supplement automático fora de scope; refactor kind: String
   diferido).
7. Tests planeados (auto-detecção por kind + fallback +
   override manual + Sequence handling se decidido em .1 §8 —
   range 6-8 per esboço P158 §5).
8. **(Específico Sequence handling)** Decisão sobre se
   `figure(Sequence([Image, ...]))` detecta `Image`. Verificar:
   - Comportamento vanilla.
   - Comportamento esperado pelos tests existentes.
   - Implicações de simplicidade vs paridade.
   - **Decisão registada com justificação**.
9. **(Específico tests existentes)** Verificar se algum test
   existente passa `kind:` explicitamente quando o body permite
   auto-detecção. Se sim, esses tests continuam válidos
   (precedência preservada). Documentar inventário.

### .2 Adicionar helper privado `infer_kind_from_body`

`01_core/src/rules/stdlib/figure_image.rs`:
- Adicionar helper privado per assinatura em §"Decisões já
  tomadas".
- Cobertura `match` em pelo menos 3 variants
  (Image/Table/Raw); decisão sobre Sequence handling per .1.
- Wildcard `_ => None` para kinds não-detectáveis (default
  `"image"` aplica-se posteriormente em fallback chain).

### .3 Modificar `native_figure` para fallback chain

`01_core/src/rules/stdlib/figure_image.rs`:
- Substituir extracção actual de `kind` (provavelmente
  `unwrap_or_else(|| "image".to_string())` directo) pela
  fallback chain de 3 níveis.
- Preservação semântica: zero mudança de comportamento quando
  `kind:` explícito é passado.
- Mensagem de erro inalterada (kind tem default — não há erro
  associado a kind ausente).

### .4 Tests

- **Auto-detecção tests** em `stdlib/mod.rs` ou `stdlib/figure_image.rs`
  (~5):
  - `figure_auto_detect_image` — `figure(image("a.png"))` →
    `kind == "image"`.
  - `figure_auto_detect_table` — `figure(table(...))` →
    `kind == "table"`.
  - `figure_auto_detect_raw` — `figure(raw("foo"))` →
    `kind == "raw"`.
  - `figure_kind_explicit_override_image` —
    `figure(image("a.png"), kind: "custom")` → `kind == "custom"`.
  - `figure_default_image_quando_body_nao_detectavel` —
    `figure([texto qualquer])` → `kind == "image"`.
- **Sequence handling test** (1 ou 2 conforme decisão .1):
  - Se Sequence detecta recursivamente: `figure_auto_detect_image_dentro_de_sequence`.
  - Se Sequence NÃO detecta: `figure_default_image_para_sequence_com_image_dentro`.
- **Regression** (1):
  - `figure_kind_explicit_continua_funcionar` — todos os tests
    pré-existentes que passam `kind:` explícito continuam a
    passar.

**Δ esperado**: +6 a +8 tests (alinhado com esboço P158 §5).

### .5 Propagação de hashes

`crystalline-lint --fix-hashes .` para verificar se há prompt L0
correspondente a `stdlib/figure_image.rs`.

Esperado: "Nothing to fix" se a estrutura de prompts L0 não
cobrir este módulo individualmente, OU hash actualizado se
cobrir.

`entities/content.rs` NÃO modificado em P158A — hash
`ec58d849` deve permanecer (sétimo passo consecutivo a preservar).

---

## Verificação

Numerada para reporte de conclusão:

1. `cargo test` em workspace: **1379 + Δ** tests, zero falhas
   (Δ esperado +6 a +8).
2. `crystalline-lint`: zero violations.
3. Contagem variants Content: **56** (inalterada — refino,
   não adição).
4. Contagem stdlib funcs: **46** (inalterada — refino, não
   adição).
5. Cobertura Model agregada: ~50% (inalterada — refino
   qualitativo). Tabela cobertura pode ganhar nota qualitativa
   em entrada `figure` se aplicável (decisão em .1).
6. Hash actualizado em prompts L0 (`crystalline-lint --check-hashes`
   passa).
7. **Hash `entities/content.rs` permanece `ec58d849`** —
   sétimo passo consecutivo (P156L → P158A) a preservar
   contrato L0 do variant Content.
8. Sequence handling decidido em .1 documentado no relatório
   §"Decisões tomadas em .1".
9. **Sem novas reservas** criadas em P158A (paridade política
   P158).
10. Tests pré-existentes que passam `kind:` explícito continuam
    a passar inalterados (regression).

---

## Critério de conclusão

- Verificações 1-10 passam.
- Relatório separado em
  `00_nucleo/materialization/typst-passo-158a-relatorio.md`
  produzido com:
  - Resumo do executado.
  - Confirmação de cada verificação.
  - **§análise de risco** (padrão N=11 → 12; décima segunda
    aplicação consecutiva — primeiro passo Model com refino
    comportamental sem alteração estrutural).
  - Slope cumulativo Model (mesa P155-P158A).
  - ADR-0061 §"Aplicações cumulativas" anotada com P158A
    (slope Layout "—"; nota cross-domínio).
  - **Confirmação**: Sequence handling decidido em .1;
    helper privado `infer_kind_from_body` N=1 (sem promoção);
    política "sem novas reservas" preservada.

---

## O que pode sair errado

**Cenários gerais**:
- Inventário .1 revela que vanilla detecta Sequence
  recursivamente mas tests cristalinos esperam comportamento
  diferente → decisão consciente registada com justificação;
  documentar como precedente para futura promoção a paridade
  total.
- `Content::Figure` ter estrutura ligeiramente diferente do
  documentado em P158 §2 (e.g. campo extra não previsto) →
  ajustar wildcard `_` no helper para preservar default
  "image"; sem impacto.

**Cenários específicos**:
- Helper `infer_kind_from_body` precisar de variants adicionais
  além de Image/Table/Raw (e.g. Listing como kind separado em
  vanilla?) → expandir match; documentar em .1.
- Tests pré-existentes terem assumpções sobre `kind == "image"`
  default que falhem com auto-detecção (e.g. teste passa
  `figure(image(...))` esperando `kind == "image"` por default,
  mas auto-detecção também devolve "image" — coincidente sem
  problema; verificar) → coincidência beneficia regression.
- Sequence handling ter implicações para `Content::Sequence`
  como wrapper transparente em outros stdlib funcs → limitar
  decisão a `figure` apenas; outros containers fora de scope.
- `figure_image.rs` não ser o módulo correcto (e.g. `native_figure`
  vivar em `stdlib/structural.rs` por reorganização recente) →
  ajustar localização em .1.

---

## Notas operacionais

- **Décima terceira aplicação de materialização**. Patamar
  empírico forte. Sem reformulação esperada.
- **§análise de risco no relatório** com peso baixo (refino
  comportamental sem refactor). Décima segunda aplicação
  consecutiva — preserva precedente.
- **Helper `infer_kind_from_body` N=1**. Sem candidato a
  reuso até agora; promoção diferida.
- **Sétimo passo consecutivo a preservar `entities/content.rs`
  hash**. Padrão de "passos aditivos / refino sem alteração
  de variant Content" estabilizado — relevante para análise
  de prompt L0 estabilidade.
- **Política "sem novas reservas" preservada**. P158A não
  cria reservas para P158B/futuros. Decisão sobre passo
  seguinte fica para sessão posterior com informação acumulada.
- **Refino qualitativo, não materialização nova**. Funcionalmente
  importante — `figure(image(...))` activa counter de imagem
  automaticamente — mas estruturalmente é só um helper privado
  + uma linha modificada.

---

## Pós-passo

Após conclusão de P158A:

**Layout fica em 78% inalterado**. **Model agregado fica em
~50% inalterado** (refino qualitativo). **Hash
`entities/content.rs` provavelmente preservado** (oitavo passo
consecutivo).

**Próxima decisão** (sem candidata pré-acordada — política
"sem novas reservas"):

- Continuar refino figure-kinds (subset máximo §3.3 do
  diagnóstico P158: supplement por lang; M; **NÃO reservado**).
- Continuar Fase 3 Layout (columns/colbreak — DEBT-56;
  quebra granularidade).
- Footnote area.
- Atacar Introspection (17% cobertura; módulo mais fraco).
- Promover ADR-0061 a IMPLEMENTADO.
- Promover `extract_length` a helper público (N=7 patamar
  forte).
- Fechar DEBT-34e e DEBT-56 (refactor multi-region L+).
- Bibliography + cite (P159 reserva pré-existente; ADR-0062
  hayagriva).
- Promover ADR-0060 a R1 com confirmação Fase 2 sub-passo 3
  fechado.
- ADR meta XS de "ADR-0064 caso completion" (saturação
  atingida em P157C).

ADR-0060 mantém-se `IMPLEMENTADO`. ADR-0061 mantém-se
`PROPOSTO`.

Padrão granularidade 1-2 features/passo (N=13 com P158A se
fechar sem reformulação) **NÃO** é formalizado em ADR. Continua
candidato.

**Pausa natural após P158A — refino qualitativo de figure
materializado; auto-detecção activada; padrões cross-domínio
consolidam-se. Decisão humana sobre próxima direcção tem
máxima informação sem reservas que travem escolha.**
