# Relatório P158 — Diagnóstico Model figure-kinds

Passo arquitectural de diagnóstico precedendo materialização.
**Não materializa código**. **Segunda aplicação concreta de
ADR-0065 critério #5** (scope determinado por inventário) após
P157 — auto-validação cumulativa do ADR meta P156K. **Décima
terceira aplicação consecutiva** do padrão diagnóstico-primeiro.

**Política nova explícita**: P158 NÃO cria reservas para passos
pós-P158A — decisões ficam para sessões futuras com informação
acumulada (resposta a crítica humana sobre reservas pré-existentes
travarem decisões).

---

## 1. Resumo do diagnóstico (síntese das 5 secções)

### 1.1 §1 — ADR-0060 sobre figure-kinds

ADR-0060 status `IMPLEMENTADO`. **figure-kinds declarado em
§"Decisão 2" Fase 2** como "extension" tamanho M. **Sem detalhes
concretos** na ADR — scope deixado para diagnóstico determinar.
Dependência declarada: P157 para figure-table (já fechada
P157A/B/C).

ADR-0041 referencia `#show figure: ...` (show rules futuras
scope-out per ADR-0054 graded).

### 1.2 §2 — Estado factual em código

**Toda a infraestrutura já existe**:
- `Content::Figure { body, caption, kind: String, numbering:
  Option<String> }` (Passo 75/ADR-0041) — field `kind: String`
  arbitrário aceita image/table/raw/custom.
- `Content::Image` (Passo 71), `Content::Table` (P157A),
  `Content::Raw` (P156C ou anterior) — todos implementados.
- **Counters por kind funcionais** em `introspect.rs:279-292`
  via `local_figure_counters.entry(kind).or_insert(0)` —
  counters independentes por kind já trabalham.
- `native_figure` aceita `kind: Str` arbitrário; default `"image"`.
- **Sem auto-detecção** baseada no body — user passa kind
  manualmente.

### 1.3 §3 — Scope determinado por inventário

3 subsets avaliados:

- **Minimal §3.2** — auto-detecção apenas: helper privado
  `infer_kind_from_body(body) -> Option<String>` + modificação
  trivial de `native_figure` para fallback chain `kind explícito
  > infer > "image"`. ~10 linhas de helper; sem alteração a
  variant ou layout; tests ~6-8.
- **Máximo §3.3** — auto-detecção + supplement automático
  ("Figure"/"Table"/"Listing" prefix por lang): exige modificação
  em `introspect.rs` para mapear `kind → prefix_localizado(lang)`;
  refactor moderado; tests ~12-18.
- **Intermédio §3.4** — auto-detecção + supplement com lang
  fallback: viola "1 feature/passo".

**Recomendação adoptada**: subset MÍNIMO §3.2.

Justificação:
- Granularidade preservada N=13.
- Auto-detecção sozinha é melhoria substancial (`figure(image(...))`
  activa counter automaticamente).
- Subsets máximo/intermédio podem ser materializados em passos
  seguintes se prioritários, **mas NÃO pré-acordados como
  reservas**.

### 1.4 §4 — Dependências bloqueantes

**Zero bloqueios hard** para subset minimal P158A:
- Toda a infraestrutura (Image/Table/Raw/Figure + counters) já
  existe.
- ADR-0017 (Introspection runtime adiada) **não bloqueia** —
  counters resolvem em walk single-pass.
- DEBTs relevantes (DEBT-14 numbering, DEBT-15 kind discriminator)
  já resolvidos em P75.

ADR-0064 NÃO directamente aplicável em subset minimal (kind
continua String directo). Caso A potencialmente aplicável se
refactor `kind: String → Option<String>` for materializado em
passo posterior — **NÃO reservado**.

### 1.5 §5 — Esboço de P158A

**Identificador**: P158A (sufixo letra após número base).

**Tamanho**: S+ ou M- (1 feature simples; helper ~10 linhas;
sem refactor de variant; sem nova ADR).

**Subset concreto**:
- Helper privado novo `infer_kind_from_body(body: &Content) ->
  Option<String>` em `stdlib/figure_image.rs`.
- Modificação `native_figure` para fallback chain.
- Sem alteração a `Content::Figure` (estrutura inalterada).
- Sem alteração a `introspect.rs` ou layout.

**Granularidade preservada**: N=13 (cresce de N=12 pós-P157C).

**Risco estimado**: baixo. Reusos significativos; sem dependência
nova.

### 1.6 Subset NÃO reservado

Per nota operacional explícita do passo:
- **Supplement automático** (prefix por lang): refino M futuro
  se prioritário; **NÃO reservado**.
- **Show selectors `figure.where(kind:)`**: scope-out per
  ADR-0041 + ADR-0054 graded; **NÃO reservado**.
- **Refactor `kind: String → Option<String>`**: candidato per
  ADR-0064 Caso A; **NÃO reservado**.

---

## 2. Decisão final de scope para P158A

**Subset MÍNIMO §3.2** (auto-detecção apenas):

```rust
fn infer_kind_from_body(body: &Content) -> Option<String> {
    match body {
        Content::Image { .. } => Some("image".to_string()),
        Content::Table { .. } => Some("table".to_string()),
        Content::Raw { .. }   => Some("raw".to_string()),
        _ => None,
    }
}

// native_figure modificação:
let kind = args.named.get("kind")
    .and_then(|v| if let Value::Str(s) = v { Some(s.to_string()) } else { None })
    .or_else(|| infer_kind_from_body(&body))  // P158A novo
    .unwrap_or_else(|| "image".to_string());
```

**Características**:
- 1 feature granular (auto-detecção).
- Helper privado ~10 linhas.
- Modificação trivial em `native_figure`.
- Tests ~6-8 (auto-detecção por kind + fallback + override
  manual + Sequence handling se aplicável).
- Sem alteração a variant `Content::Figure`.
- Sem alteração a layout ou introspect.
- **Granularidade preservada N=13**.

**Subsets rejeitados**:
- §3.3 máximo (auto-detecção + supplement) — refactor moderado;
  candidato a passo dedicado se prioritário.
- §3.4 intermédio (mistura concerns) — viola "1 feature/passo".

---

## 3. Dependências identificadas a tratar antes de P158A

**Zero pré-requisitos hard**. P158A pode iniciar sem trabalho
prévio.

Notas operacionais:
- **Sequence handling**: decisão sobre se `figure(Sequence([
  Image, ...]))` deve detectar `Image` recursivamente fica para
  diagnóstico .1 de P158A.
- **Fallback default `"image"`** preservado para compatibilidade
  com tests existentes.

---

## 4. Análise de risco (padrão N=10 → 11; passo diagnóstico)

P158 é **passo diagnóstico** sem alteração de código. **Décima
primeira aplicação consecutiva** de §análise de risco
(P156F-P157C + P158) preservando precedente.

### 4.1 Riscos identificados

| Risco | Avaliação | Mitigação aplicada |
|-------|-----------|---------------------|
| ADR-0060 não cobrir figure-kinds com detalhe esperado | Confirmado | Inventário §1.2 documentou — scope decidido por diagnóstico em vez de pré-existente em ADR |
| `Content::Figure` ter estrutura não-trivial | Não — já existe completo desde P75 | Inventário §2.1 confirmou; refactor minimal possível |
| `Content::Image` não existir | Não — implementado P71 | Inventário §2.2 confirmou |
| Counters de figure dependerem de ADR-0017 | Não — resolvem em walk single-pass | Inventário §4 confirmou |
| Subset "figure-kinds" forçar quebra de granularidade | Não — subset minimal preserva N=13 | §3 documenta 3 subsets; minimal recomendado |

### 4.2 Riscos não-aplicáveis

- **Refactor de código**: zero (passo puramente documental).
- **Quebra de contrato API**: zero (sem código alterado).
- **Drift de hashes L0/L1**: zero (sem código → sem hash a
  propagar).

### 4.3 Conclusão de risco

**Risco residual: muito baixo.** Padrão "passo diagnóstico
documental + scope determinado por inventário (ADR-0065 #5) +
política sem novas reservas" replica tratamento bem-sucedido
de P157 (table foundations diagnóstico) e P156B (Layout
diagnóstico).

**Auto-validação ADR-0065 critério #5**: este passo é segunda
aplicação concreta de critério #5 (scope) após P157. Padrão
consolida-se com aplicação cross-feature: P157 (table)
multi-passo divisão M+ → 3xM; P158 (figure-kinds) single-passo
divisão entre subsets minimal/intermédio/máximo. Critério #5
demonstra flexibilidade — aplica-se tanto a divisão quanto a
selecção de subset.

**Política "sem novas reservas"**: P158 estabelece precedente
explícito documentado em §"Notas operacionais" — resposta a
crítica humana sobre reservas pré-existentes (P158 + P159 +
ADR-0062) travarem decisões. Reservas existentes mantêm-se
respeitadas; novas decisões ficam para sessões futuras com
informação acumulada.

---

## 5. Verificações (numeradas per spec do passo)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | Diagnóstico produzido com 5 secções | **✓** `diagnostico-model-figure-kinds-passo-158.md` (5 secções §1-§5) |
| 2 | ADR-0060 §"Decisão" relevante a figure-kinds lida e resumida em §1 | **✓** §"Decisão 2" Fase 2 + §"Plano de materialização" linha P158 citados literalmente |
| 3 | Estado de `Content::Figure` em crystalline determinado factualmente em §2 | **✓** estrutura completa documentada; counters por kind confirmados |
| 4 | Estado de `Content::Image` (e dependências) em §2 | **✓** Image (P71) + Table (P157A) + Raw confirmados |
| 5 | Subset concreto definido em §3 com recomendação para P158A | **✓** subset minimal §3.2 (auto-detecção apenas) recomendado; §3.3 máximo + §3.4 intermédio rejeitados com justificação |
| 6 | Dependências bloqueantes listadas em §4 | **✓** zero bloqueios hard documentados; ADRs em vigor + pendentes |
| 7 | Esboço de P158A em §5 | **✓** identificador, tamanho S+/M-, subset, sub-passos, granularidade preservada N=13 |
| 8 | Sem novas reservas criadas para passos pós-P158A | **✓** §1.6 + §"Notas operacionais" + relatório §1.6 explícitos |
| 9 | ADR-0061 §"Aplicações cumulativas" actualizada com linha P158 | **✓** linha P158 com slope "—"; padrões inventariar primeiro N=10 → 11; §análise risco N=10 → 11 |
| 10 | `crystalline-lint`: zero violations | **✓ No violations found** |
| 11 | Sem alteração de hashes (passo documental) | **✓** zero código modificado; hash `entities/content.rs` mantém-se `ec58d849` (preservado P156L → P158 — 6 passos consecutivos) |

---

## 6. Confirmação: ADR-0065 critério #5 segunda aplicação concreta

P158 é **segunda aplicação concreta** de ADR-0065 critério #5
(scope: atributos a incluir/diferir per ADR-0054 graded) após P157.

Auto-validação cumulativa do ADR meta P156K:
- **ADR-0064**: NÃO aplicável directamente em P158 (passo
  documental); aplicação futura potencial em P158A se refactor
  `kind: String → Option<String>` for materializado.
- **ADR-0065**:
  - Critério #1 (naming) — aplicado P157A/B/C.
  - Critério #3 (expansão variant) — aplicado P156L.
  - **Critério #5 (scope) — aplicado P157 + P157A reforçado +
    agora P158 (segunda aplicação concreta)**.
  - Critério #6 (divergência da spec) — aplicado P157B/C.

**Padrão emergente**: cada passo documental valida cumulativamente
critérios distintos de ADR-0065 e expande a base empírica.
Critério #5 demonstra flexibilidade cross-feature (table foundations
multi-passo divisão M+ → 3xM em P157; figure-kinds single-passo
divisão entre subsets em P158).

---

## 7. Estado pós-P158

- **Cobertura Layout**: **78%** (inalterada — escopo Model
  documental).
- **Cobertura Model agregada**: ~50% (inalterada — passo
  documental).
- **Cobertura arquitectural total**: **80%** (inalterada).
- **Variants Content**: **56** (inalterada).
- **Stdlib funcs**: **46** (inalterada).
- **Tests**: **1141** typst-core lib (inalterada). Workspace:
  **1401** (inalterada).
- **Lint**: zero violations.
- **DEBTs**: zero criados ou fechados.
- **ADR-0061** §"Aplicações cumulativas": tabela slope ganha
  linha P158 (slope "—"); padrões N actualizados.
- **README ADRs**: entrada P158 adicionada antes de P157C.
- **Reservas pré-existentes**: P159 + ADR-0062 mantidas (não
  reforçadas; **política nova "sem novas reservas"** estabelecida).
- **Hash `content.rs`**: `ec58d849` (preservado — passo
  documental; **6 passos consecutivos** P156L → P158 com
  hash inalterado).
- **Total ADRs**: **63** (inalterado).

### 7.1 Próxima decisão

**P158A** redigido como spec separada com base no diagnóstico
§5. Ponto de validação humana antes de redigir spec.

Se P158A for aprovado:
- Materializa auto-detecção de kind.
- Cadência granular preservada (N=13).
- Refino qualitativo de `figure` existente — sem novo variant.

Se P158A for redirigido:
- §3.3/§3.4 documentam alternativas (supplement automático,
  intermédio).

Outras direcções pendentes (sem reservas reforçadas):
- Continuar Fase 3 Layout (columns/colbreak).
- Footnote area.
- Promover ADR-0061 a IMPLEMENTADO.
- Promover `extract_length` a helper público (N=7 patamar
  forte).
- Atacar Introspection (17% cobertura).
- Fechar DEBT-34e e DEBT-56 (refactor multi-region).
- Promover ADR-0060 a R1 com confirmação Fase 2 sub-passo 3
  fechado.
- ADR meta XS de "ADR-0064 caso completion" (saturação atingida
  em P157C).

---

## 8. Fechamento

P158 fecha como **passo diagnóstico documental** sem alteração
de código. **Auto-validação ADR-0065 critério #5** — segunda
aplicação concreta após P157 reforça padrão "scope determinado
por inventário factual" cross-feature (table multi-passo;
figure-kinds single-passo).

**Política nova "sem novas reservas"** estabelecida explicitamente
em §"Notas operacionais" — resposta a crítica humana. Reservas
existentes (P159 + ADR-0062) respeitadas; futuros refinos
(supplement, show selectors, refactor kind) **não reservados**;
decisões ficam para sessões futuras.

**Padrões pós-P158**:
- Granularidade N=12 (inalterada — diagnóstico).
- Inventariar primeiro N=10 → **11** (segunda aplicação concreta
  critério #5).
- §análise risco N=10 → **11** (passo diagnóstico baixo risco;
  estabelece precedente "sem novas reservas").

**ADR-0060 mantém `IMPLEMENTADO`** (P158 lê, não modifica).
**ADR-0061 mantém `PROPOSTO`** (Layout não tocado).

**ADRs meta P156K** continuam a ganhar evidência empírica:
ADR-0064 inalterada (não aplicável directamente); ADR-0065 ganha
segunda aplicação concreta de critério #5.

**Pausa natural após P158 — diagnóstico estruturado completo;
P158A pronto para spec; padrão "sem novas reservas" estabelecido.
Decisão humana sobre P158A (ou outras direcções pendentes) tem
máxima informação acumulada.**
