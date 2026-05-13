# Relatório do passo P215 — Diagnóstico Layout Fase 3 (DEBT-56)

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-215.md`.
**Tipo**: diagnóstico arquitectural amplo + ADR PROPOSTO column
flow algorithm + roadmap sub-passos.
**Magnitude planeada**: M (~2-3h). **Magnitude real**: M (~2h).
**Marco**: nenhum (terceiro passo pós-M9c; primeiro a abrir
nova trajectória de materialização — Layout Fase 3).

---

## §1 O que foi feito

P215 produziu diagnóstico-primeiro (11ª aplicação cumulativa)
para Layout Fase 3 / DEBT-56 (column flow). Inventário
empírico Layouter revelou **135 call-sites** afectados
por refactor multi-region (>100 → registou-se **P215.div-1**
decompondo sub-fase (a) em P216A+P216B). 6 entradas
restantes mapeadas em §A.5 + cross-ref §A.6 com dependências
sub-fases. **ADR-0078 PROPOSTO** criada com decomposição
sub-fases (a) decomposta + (b) consumer; paridade vanilla
`regions.rs` documentada. Roadmap 6 sub-passos núcleo
(P216A-B + P217-P221) + 3 opcionais (P222-P224) sem reservas.
Política "sem novas reservas" P158 preservada. Marca blueprint
**diferida** per Opção β (paridade P156B/P159B
diagnóstico-amplo sem marca cirúrgica).

---

## §2 Inventário Layouter face a multi-region (C1)

Auditoria empírica `01_core/src/rules/layout/mod.rs`:

| Componente | Estado actual | Multi-region? | Call-sites |
|------------|---------------|----------------|------------|
| `Layouter` struct | single-page write-target | Não | (struct) |
| `current_items: Vec<FrameItem>` | escrita directa | Não — sem `Region` | incluído nos 102 |
| `current_line: Vec<FrameItem>` | acumulador de linha | Não | incluído nos 102 |
| `cursor_x`/`cursor_y` | escalares globais | Não — sem iteração regions | incluído nos 102 |
| `page_config.width`/`height` | dimensões directas | Não — sem `width / count - gutter` | 33 separado |
| `flush_line` / `flush_page` | escreve em `pages` | Não — sem iteração de colunas | (helpers) |

**Empírico**:
- `grep -c "current_items|current_line|cursor_x|cursor_y"
  01_core/src/rules/layout/mod.rs` = **102 matches**.
- `grep -c "page_config\."
  01_core/src/rules/layout/mod.rs` = **33 matches**.
- **Total: ~135 call-sites** afectados pelo refactor
  multi-region.

**P215.div-1 registada**: 135 > 100 limiar do spec §5 risco 1.
Sub-fase (a) decomposta em **P216A+P216B** (vs P216 monolítico
hipotético):
- P216A: introduzir `Region` type; substituir cursor_x/y/items/
  line/page_config (~80 call-sites).
- P216B: introduzir `Regions` (Vec<Region>) wrapper +
  `Layouter::with_regions` helper (~30-40 call-sites).

Magnitude sub-fase (a) decomposta: **2× M+ (~3-5h cada
sub-passo)** vs M+ original (~3-5h num passo). Risco
mitigado por decomposição.

---

## §3 Inventário entradas Fase 3 restantes (C2)

| Entrada | Magnitude | Dep. sub-fase (a)? | Dep. sub-fase (b)? | Notas |
|---------|-----------|---------------------|---------------------|-------|
| `columns(n)` | M | sim | sim | core feature DEBT-56; §A.5 ausente |
| `colbreak()` | S+ | sim | sim | depende columns; §A.5 ausente |
| `place` float/clearance | S+ | parcial (column scope) | não | refino DEBT-37 fechado P84.6; §A.5 parcial |
| `measure(body)` stdlib | S+ | não (helper L1 existe) | não | ADR-0066 Bloco C; isolado; §A.5 parcial |
| `grid` header/footer real | M | sim | sim | DEBT-56 difere repetição em page breaks; §A.6 Model parcial |
| `TableHeader.repeat` algoritmo | S+ | sim | sim | P157C diferiu em DEBT-56; §A.6 Model |

**Distribuição dependências**:
- 4 entradas dependem de DEBT-56 sub-fases: `columns`, `colbreak`,
  `grid` header/footer, `TableHeader.repeat`.
- 2 entradas isoladas: `place` refino (parcial sub-fase a),
  `measure` stdlib (independente).

**Sub-fase (a) desbloqueia 4 entradas em cascata**; sub-fase
(b) materializa 2 directas (`columns` + `colbreak`).

`measure` stdlib **NÃO depende de DEBT-56**; pode ser
materializado em paralelo ou primeiro (P222 opcional Bloco C).

---

## §4 ADR-0078 PROPOSTO column flow algorithm (C3)

ADR-0078 criada em
`00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md`
status PROPOSTO 2026-05-12. Estrutura paridade ADR-0061:
Status / Diagnóstico prévio / Contexto / Decisão / Não-objectivos
/ Plano materialização / Alternativas Consideradas / Reservas /
Análise paridade vanilla / Referências / Histórico.

**Decisão arquitectural fixada**:
- Tipos `Region` + `Regions` em L1 (paridade vanilla
  `typst-layout/src/regions.rs` simplificado).
- Sub-fase (a) decomposta P216A+B (per P215.div-1).
- Sub-fase (b) consumer multi-column (P217-P220).
- 6 condições para transição PROPOSTO → IMPLEMENTADO em P221.

**Divergências cristalino vs vanilla documentadas**:
- `Region` simplificado (sem `expand` axes; sem `full`).
- `Regions` owned `Vec<Region>` (vs vanilla `&'a [Abs]` borrow).
- Sem `root` flag explícito.
- Auto-expand controlado por flush helpers em vez de field.

Justificação per ADR-0054 graded — refinos vanilla adiados
se consumer real necessitar.

**Cross-references**: ADR-0061 (PROPOSTO; Layout Fase X),
ADR-0066 (PROPOSTO; Introspection runtime — Bloco C),
ADR-0054 (graded), DEBT-56 (EM ABERTO), DEBT-37 (FECHADO),
P156B (precedente metodológico), P215 (este).

---

## §5 Roadmap granular P216-P224 (C4)

**Núcleo (6 sub-passos)**:

| Sub-passo | Trabalho | Magnitude | Cumulativo |
|-----------|----------|-----------|------------|
| **P216A** | sub-fase (a) parte 1 — `Region` type + cursor_x/y/items/line/page_config refactor | M+ | ~80 call-sites; tests inalterados |
| **P216B** | sub-fase (a) parte 2 — `Regions` wrapper + `Layouter::with_regions` helper | M | ~30-40 call-sites; preparação sub-fase (b) |
| **P217** | `Content::Columns { count, gutter, body }` variant + arms exhaustivos | S+ | enum 56→57 variants |
| **P218** | `native_columns` stdlib + `extract_count` helper + scope register | S | stdlib 53→54 funcs |
| **P219** | sub-fase (b) — Consumer multi-column no Layouter; iteração N regions; `width / count - gutter` | M+ | tests `columns(2)`/`columns(3)` |
| **P220** | `Content::Colbreak { weak: bool }` + `native_colbreak` + tests mixing pagebreak | S+ | enum 57→58; stdlib 54→55 |
| **P221** | encerramento Fase 3 — ADR-0061 + ADR-0078 PROPOSTO → IMPLEMENTADO; DEBT-56 fecha; inventário 148 actualiza | XS | documental |

**Opcionais Bloco C (3 sub-passos paralelos não-bloqueantes)**:

| Sub-passo opcional | Trabalho | Magnitude | Dependência |
|---------------------|----------|-----------|--------------|
| **P222** | `measure(body)` stdlib expose (Bloco A) | S+ | nenhuma |
| **P223** | `place` float/clearance refino | S+ | sub-fase (a) parcial |
| **P224** | `grid` header/footer real + `TableHeader.repeat` algoritmo | M+ | sub-fases (a)+(b) completas |

**Custo agregado núcleo**: M+M+S+S+M+S+XS = **L cumulativo
(~6-9h)** vs DEBT-56 estimado L+ original (~5-8h L+ incluindo
todas as 6 entradas dependentes — i.e. L+ era para resolver
DEBT-56 + cascata; L pelo presente decompõe pendência mais
honestamente).

**Total candidatos**: 7 sub-passos núcleo (P216A-B + P217-P221)
+ 3 opcionais (P222-P224) = **10 candidatos**.

---

## §6 Decisões substantivas

- **Sub-fase decomposta vs big-bang refactor**: decomposta
  fixada em P215.div-1 (135 call-sites > 100). Risco
  mitigado; tests existentes funcionam como regression suite
  para sub-fase (a).
- **Política "sem novas reservas" P158 preservada**:
  P216-P224 documentados como **opções identificadas**, NÃO
  reservas. Análoga a P213/P214 candidatos Bloco A/B.
  Distinção crítica: ADR-0078 PROPOSTO é decisão
  arquitectural (não reserva de número); paridade ADR-0061
  PROPOSTO em P156B.
- **Marca blueprint diferida — Opção β**: paridade P156B
  diagnóstico amplo (que não criou marca blueprint per pattern
  P204H+ "fora-de-escopo reescrita ampla"). P159B também
  diagnóstico sem marca. P215 segue mesmo precedent —
  reduz inflação documental. Marca pode ser adicionada em
  P221 (encerramento Fase 3) se útil.
- **`measure` (P222 opcional) em paralelo a DEBT-56**:
  primeira identificação explícita de sub-passo
  paralelizável em série Layout. Permite materialização
  isolada sem bloquear DEBT-56.
- **Divergências cristalino vs vanilla `Region/Regions`
  documentadas**: simplificações justificadas per ADR-0054
  graded; refinos (lifetime, axes, root) adiados se consumer
  real necessitar.
- **P215.div-1 registada**: 135 > 100 call-sites empíricos
  → sub-fase (a) decomposta em P216A+P216B. Pattern
  emergente "decomposição empírica de magnitude" — magnitude
  ajustada baseada em count empírico, não estimativa
  abstracta.

---

## §7 Marca §3.0duodecies blueprint (decisão Opção β — sem marca)

**Decisão fixada em C6 P215**: **Opção β — sem marca
blueprint adicionada em P215**.

Justificação:
- P156B diagnóstico Layout amplo não criou marca blueprint.
- P159B diagnóstico Model amplo idem.
- Pattern emergente P204H+ "fora-de-escopo reescrita ampla"
  preserva blueprint sem inflação documental.
- Marca cirúrgica é para **fechos** (séries, marcos,
  recálculos administrativos como P213/P214). Diagnósticos
  amplos abrem trabalho — fechamento ocorre noutro passo.

**Reconsideração registada**: se humano fixar Caminho 1
(prosseguir P216A imediatamente), marca blueprint pode ser
útil em P221 (encerramento Fase 3) cumprindo padrão
"encerramento série" §3.0quater-§3.0septies. Decisão
deferida ao executor de P221 se Caminho 1 for prosseguido.

Estado pós-P215: 11ª marca cumulativa blueprint (§3.0undecies
P214) preservada; P215 não adiciona §3.0duodecies. Próxima
marca será §3.0duodecies se P221 for executado em pattern
encerramento série.

---

## §8 Decisão humana — caminhos 1-4 (C8)

P215 fecha diagnóstico. Decisão humana sobre próxima sessão:

| Caminho | Trabalho | Magnitude | Estado pós-execução |
|---------|----------|-----------|----------------------|
| **Caminho 1** | Prosseguir série materialização P216A imediatamente (sub-fase a parte 1) | M+ (~3-5h por sessão) | Layout 78% → ~85% após P217-P218; ~95% após P219-P220; 100% após P221 |
| **Caminho 2** | Começar pelos Bloco C isolados — P222 `measure` stdlib expose primeiro | S+ (~1-2h) | Layout 78% → 89% (1 entrada parcial → impl; sem afectar DEBT-56) |
| **Caminho 3** | Adiar DEBT-56; voltar a outro módulo (Model hayagriva DEBT-55; outro recálculo de categoria; etc.) | varia | DEBT-56 mantém-se aberto |
| **Caminho 4** | Diagnóstico mais profundo se C1 revelou complexidade inesperada (sub-passo P215-bis) | XS-S (~30min-1h) | refino cálculo magnitude |

**Recomendação metodológica subjectiva** (não fixação):
humano fixou "focar no Layout até onde der" — sugere
Caminho 1 (prosseguir P216A imediatamente). Caminho 2
(`measure` primeiro) é alternativa "win rápido" se humano
quer fechar §A.9 estricto antes de mergulhar em DEBT-56.

**Considerações**:
- P215.div-1 (135 call-sites) já mitigou risco original.
  Caminho 1 é tractável.
- Caminho 2 mantém momentum sem grande compromisso.
- Caminho 3 deixa Layout em 78% indefinidamente.
- Caminho 4 só se P215 inventário revelou surpresa
  (não revelou — 135 era provavelmente próximo do
  esperado).

**Decisão humana fica em aberto literal**. P215 não
compromete trabalho subsequente per política "sem novas
reservas".

**Estado final pós-P215**:
- Marco M9c: ✅ ACEITE 2026-05-12 (preservado).
- ADRs ACEITES M9c: 2 (preservadas).
- ADR-0078 PROPOSTO: criada (column flow algorithm).
- DEBT-56: EM ABERTO (decomposição P215.div-1 mitiga
  custo).
- Layout categoria: 78% (preservada via P214 sync).
- 6 entradas restantes Fase 3 mapeadas com dependências.
- Roadmap 7 sub-passos núcleo (P216A-B + P217-P221) + 3
  opcionais (P222-P224) documentados como opções.
- Tests workspace: **1939 verdes**; `crystalline-lint`: **0
  violations** (preservados; sem código tocado).
- Trajectória aberta: 4 caminhos para próxima sessão;
  decisão humana.
- 11ª aplicação consecutiva diagnóstico-primeiro pattern
  (P148+P154A+P156B+P157+P158+P159+P159B+P160+P213+P214+
  **P215**).
