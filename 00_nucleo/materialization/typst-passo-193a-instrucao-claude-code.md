# Passo 193A — Instrução Claude Code

## Contexto mínimo

Typst Cristalino é re-implementação atómica do projecto
`typst/typst` em Rust com arquitectura camadas L0–L4.
Vanilla original está em quarentena em `lab/typst-original/`.
Cristalino vive em `01_core/`, `02_shell/`, `03_infra/`,
`04_wiring/`. ADRs em `00_nucleo/adr/`.

**Snapshot de partida** (a confirmar empiricamente em
sub-passo .A):

- Tests workspace 1.815 verdes; zero violations.
- M9 ✅ 11/11.
- M4-residual fechado funcionalmente (P188B).
- M5 incremental (P189B): 1 arm migrado (Outline) + 6
  excepções declaradas.
- M5 universal bloqueado por 7 passos sequenciais
  pré-requisito (P189 §9 sequência).

**P193 é o passo 1 dessa sequência**: abrir sub-store
`resolved_labels` no Introspector. Trabalho **mais
arquitectural** desde P181 (BibStore) ou P185 (Layouter
location-aware) — introduz nova estrutura de dados +
novo método no trait.

P193 não migra consumer C4 (Ref-arm em Layouter) — esse
é passo 2 da sequência (provável P194). P193 não migra
walk arms Labelled/Heading — esses são passos 3-4 (P195+).

**Cadeia de desbloqueio**:

```
P193 (sub-store resolved_labels)
  ↓
P194 (C4 migration — consumer Ref-arm)
  ↓
P195 (migrar walk arm Labelled) — E2+E4 fecham
  ↓
P196 (migrar walk arm Heading) — E2 residual
  ↓
P197 (migrar walk arm Figure) — E3 fecha
  ↓
P198 (migrar walks SetHeadingNumbering + CounterUpdate)
       — E5+E6 fecham
  ↓
M5 universal fecha (excepto E1 — Equation)
  ↓
P199 ou paralelo (SetEquationNumbering) — E1 fecha
  ↓
M5 universal completamente fechado
  ↓
P200 (M6 eliminar CounterStateLegacy)
```

P193A é o passo de diagnóstico que precede a
implementação. Magnitude esperada **S–M** — replicação de
padrão BibStore (P181B) com adaptação a estrutura
diferente.

Material de partida verificado:

- `00_nucleo/materialization/typst-passo-189-relatorio-consolidado.md`
  §9 — sequência de 7 passos identificada; P193 é
  passo 1.
- `00_nucleo/materialization/typst-passo-181b-relatorio.md`
  — relatório criação BibStore; padrão de referência.
- ADR-0026 Content enum fechado — não-aplicável (P193
  não toca enum).
- ADR-0068 ACEITE — Layouter location-aware (impacta
  P194 mas não P193).

P193A é diagnóstico-primeiro. Sem decisões fixadas, P193B+
herda problema do plano monolítico.

---

## Postura do auditor / executor

P193A é passo **L0-puro / diagnóstico-primeiro**, padrão
estabelecido em 14 aplicações
(P131A/132A/140A/148/154A/181A/182A/183A/184A/185A/186A/
187A/188A/189A).

- **Zero código tocado** em camadas cristalinas.
- **Zero testes** modificados.
- **Pode criar** ADR `PROPOSTO` — possivelmente para
  formalizar interface de `LabelStore` se a decisão for
  arquitectural. Avaliar em `.A`.
- **Pode abrir DEBT** se trabalho identificado for adiado.
- **Não modifica** trait `Introspector`, walk, `from_tags`,
  Layouter consumer — P193B+.

**Magnitude diagnóstico**: S. Decisões esperadas são
predominantemente arquiteturais (forma do sub-store,
método trait, tipos de retorno). Possível ADR.

**Regra dos 2 eixos aplicável** (P183C §6) — para auditar
qual é o tipo de dados real necessário: o consumer C4 lê
"durante walk" (mutável) ou "snapshot final" (location-aware
opcional)?

---

## Escopo

**Primário**: desenhar abertura do sub-store
`resolved_labels` no Introspector — estrutura de dados,
API, integração com `from_tags`, interface no trait
`Introspector`.

**Confirmação**: validar inventário factual — campo legacy
`state.resolved_labels`, consumer C4 actual, contracto
semântico que precisa de preservar.

**Decisões a tomar** — 8 cláusulas:

1. **Forma estrutural do sub-store** —
   `LabelRegistry`/`ResolvedLabelStore`/sub-mapa em
   estrutura existente. Tipo de chave (`Label` vs
   `String`) e tipo de valor (`String` vs estrutura
   mais rica).

2. **Localização do sub-store no `TagIntrospector`** —
   field novo paralelo a `BibStore`/`StateRegistry`/etc.
   ou aninhado em estrutura existente.

3. **Forma de populate em `from_tags`** — receber payload
   `ElementPayload::Labelled { label, ... }` (existe?
   ou precisa ser criado em P195?). P193 abre só sub-store
   ou também adiciona arm de populate?

4. **API exposta no trait `Introspector`** — método novo
   `resolved_label_for(&Label) -> Option<&str>` ou
   `resolved_label_for(&Label) -> Option<String>` ou
   variante location-aware.

5. **Decisão sobre location-awareness** — `resolved_labels`
   é semântica snapshot final (label → text fixo) ou
   pode mudar com Location? Auditor decide via regra dos
   2 eixos.

6. **Compatibilidade com `state.resolved_labels` legacy**
   — P193 abre sub-store novo; legacy continua a ser
   populated por walk arms (E2/E4). Bridge em `from_tags`
   ou independência total?

7. **Tipo de chave** — `Label`, `String`, ou outro?
   `state.resolved_labels` legacy usa qual?

8. **Critério de fecho de P193** — sub-store + método
   trait + tests unit. Sem migração de consumer (P194) e
   sem migração de walk arms (P195+).

**Fora de escopo**:

- Migração consumer C4 (P194).
- Migração walk arms Labelled/Heading/Figure
  (P195/196/197).
- Eliminação `CounterStateLegacy.resolved_labels` (M6).
- `SetEquationNumbering` (passo independente).

---

## Critérios objectivos

### O1 — Inputs verificáveis

`grep -rn "resolved_labels\|resolved_label" 01_core/src/`.
Para cláusula 1, confirmar tipo de
`state.resolved_labels`. Para cláusula 7, confirmar
chave usada nos `insert`s legacy.

### O2 — Alternativas

Mínimo 2 quando há margem real. Para cláusula 1 (forma
estrutural): sub-store dedicado vs sub-mapa em
`StateRegistry` vs reuse de outro sub-store. Para
cláusula 4 (API): variante simples vs location-aware.

### O3 — Critério de escolha

Padrão BibStore (P181B) replica para estruturas paralelas.
Padrão `StateRegistry` (P171) para estruturas que evoluem
com Location. Decisão depende de §5.

### O4 — Magnitude

P193 sub-passo único agregado é **S–M**. Sub-store + API
+ tests é coeso mas pode escalar para M se decisão de
location-awareness exigir mais infraestrutura.

### O5 — Reversibilidade

Adicionar sub-store é reversível (remover field +
método). API trait é reversível mas exige cuidado se
consumers já usam.

---

## Critérios qualitativos

### Q1 — Consistência com padrão estabelecido

Replica P181B (BibStore) literalmente? Diferença:
`resolved_labels` é mapa Label→String simples; BibStore
tem mais estrutura (entries, numbers).

### Q2 — Honestidade de magnitude

P193A diagnóstico é S. P193B+ implementação:
- Provável passo único agregado: sub-store + API + tests.
- Magnitude S–M (depende de decisão `.A`).

Total agregado: ~80-150 LOC produção + ~80 LOC tests ≈ S
ou M.

### Q3 — Cobertura sem regressão

P193 **não toca**:
- Walk arms (continuam a popular legacy).
- Consumer C4 (continua a ler legacy).
- `from_tags` arms existentes.

P193 adiciona infra **sem activar caminho**. Activação
em P194/P195+. Sem regressão esperada.

### Q4 — Honestidade sobre M5 incremental

P193 abre 1 sub-store dos 2 que faltam (`resolved_labels`
e `headings_for_toc`). P193 não fecha lacuna #3
(`headings_for_toc`). E2 E3 E4 E5 E6 ficam parcialmente
desbloqueados — "depende deste sub-store" passa a
"depende de P194/195+".

### Q5 — Granularidade

Sub-passo único P193B agregado:
- Adicionar struct (sub-store).
- Integrar em `TagIntrospector`.
- Adicionar método ao trait.
- Tests unit.
- L0 actualizada.

Pode ser dividido em 2 sub-passos (B = struct + integração;
C = trait + tests) se magnitude empírica em `.A` o
justificar.

---

## Sub-passos de P193A

### Sub-passo 193A.A — Validação do estado actual

Auditor confirma empiricamente:

1. Campo legacy `state.resolved_labels`:
   - Localização em `counter_state_legacy.rs` (ou
     similar).
   - Tipo: `HashMap<Label, String>`? `HashMap<String,
     String>`? Outro?
   - Como é populated: walk arms (per P189A §2.7).
   - Como é lido: consumer C4 em Layouter (per P189A
     §2.6).

2. Confirmar arms que populam:
   - `Content::Labelled` walk (per P189A §11.2).
   - `Content::Heading` walk auto-toc (per P189A §11.2).
   - Possivelmente outros — `grep -rn "resolved_labels" 01_core/src/rules/introspect.rs`.

3. Confirmar consumer C4:
   - Localização exacta (`mod.rs` Ref-arm).
   - Forma da leitura: `state.resolved_labels.get(&label)`?
   - Tipo de retorno usado downstream.

4. Confirmar uso em vanilla typst:
   - `grep -rn "resolved_label\|resolve_label" lab/typst-original/`.
   - Como vanilla resolve cross-references?

5. Confirmar trait `Introspector` actual:
   - 18 métodos existentes per P185-consolidado.
   - Onde adicionar método novo (ordem cronológica per
     P185B convenção).

6. Confirmar `TagIntrospector` struct actual:
   - Sub-stores existentes (`StateRegistry`,
     `CounterRegistry`, `BibStore`, `MetadataStore`,
     `kind_index`, ...).
   - Onde adicionar field novo.

7. Confirmar tests existentes:
   - `grep -rn "resolved_label" 01_core/src/`.
   - Tests unit do legacy `state.resolved_labels`.
   - Tests E2E que cobrem cross-references.

8. Aplicar regra dos 2 eixos:
   - **Eixo 1**: consumer C4 precisa do valor "durante
     walk" (mutável) ou snapshot final?
   - **Eixo 2**: dados estão presentes em produção
     (state.resolved_labels é populated por walks que
     existem em produção)?

   Esperado: eixo 1 = snapshot final (label → text
   resolvido é determinístico após walk completo); eixo 2
   = sim em produção (walk arms são de Heading/Labelled
   que existem). **Sub-store não precisa de variante
   location-aware** (cláusula 5).

Output: tabela com item + estado confirmado / linha
actual / observação. Inclui análise eixos.

**Critério de saída**:
- Tipo de `state.resolved_labels` confirmado.
- Arms que populam identificados.
- Consumer C4 localizado.
- Análise dos 2 eixos completa.
- Decisão sobre location-awareness preliminar.

### Sub-passo 193A.B — Decisão cláusula 1 (forma estrutural)

Avaliar a forma do sub-store.

**Opção α** — Struct dedicado `ResolvedLabelStore`:
```
pub struct ResolvedLabelStore {
    labels: HashMap<Label, String>,
}
```

**Opção β** — Struct paralelo a `BibStore` (mais
estrutura):
```
pub struct ResolvedLabelStore {
    by_label: HashMap<Label, String>,
    by_location: HashMap<Location, Label>, // se útil
}
```

**Opção γ** — Sub-mapa em `StateRegistry` ou outro
sub-store existente:
- Reuso de infra; menor pegada.
- Mas semântica diferente (state value-at vs label
  resolution determinístico).

Critério: padrão P181B usou Opção α (struct dedicado
com mapa simples). Replica.

Sugestão: **Opção α**. Struct dedicado mínimo.

Output: decisão fixada.

### Sub-passo 193A.C — Decisão cláusula 2 (localização)

`TagIntrospector` ganha field novo:
```
pub struct TagIntrospector {
    // existing
    pub state: StateRegistry,
    pub counters: CounterRegistry,
    pub bib: BibStore,
    pub kind_index: HashMap<ElementKind, Vec<Location>>,
    // NEW
    pub labels: ResolvedLabelStore,
}
```

**Opção 1** — Field directo, paralelo aos outros.

**Opção 2** — Field aninhado dentro de outro sub-store
(ex.: `MetadataStore.labels`). Reuso de infra.

Critério: paralelismo com BibStore (Opção 1) é mais
claro arquiteturalmente.

Sugestão: **Opção 1**.

Output: decisão fixada.

### Sub-passo 193A.D — Decisão cláusula 3 (populate em `from_tags`)

P193 abre sub-store **mas não popula em produção** —
walks ainda mutam legacy directamente (excepções E2/E4
de P189B). P193 pode:

**Opção A** — Apenas declarar sub-store; sem arm de
populate em `from_tags`. P195 (migrar walk Labelled)
adiciona arm.

**Opção B** — Adicionar arm em `from_tags` para
`ElementPayload::Labelled` (que precisa ser criado em
P195). Bloqueia P193 a P195 de forma cíclica.

**Opção C** — Adicionar bridge em `from_tags`: copiar
`state.resolved_labels` (já populated por walks legacy)
para sub-store novo.

Critério: Opção A é mais limpa (separação clara entre
"abrir infra" e "popular"). Opção C cria duplicação de
estado durante janela compat.

Sugestão: **Opção A**. P193 abre sub-store com tests
unit (populate manual em tests); P195 adiciona arm de
populate via Tag.

Output: decisão fixada.

### Sub-passo 193A.E — Decisão cláusula 4 (API trait)

**Opção α** — Método simples:
```
fn resolved_label_for(&self, label: &Label) -> Option<&str>;
```

**Opção β** — Retornar String (clone):
```
fn resolved_label_for(&self, label: &Label) -> Option<String>;
```

**Opção γ** — Retornar referência ao registry inteiro:
```
fn resolved_labels(&self) -> &ResolvedLabelStore;
```

Critério: Opção α (referência) evita clone desnecessário.
Opção γ expõe muito interno. Padrão BibStore P181F usou
métodos específicos (`bib_entry_for`, `bib_number_for`),
não exposição directa.

Sugestão: **Opção α** com nome
`resolved_label_for(&self, label: &Label) -> Option<&str>`.

Output: decisão fixada.

### Sub-passo 193A.F — Decisão cláusula 5 (location-awareness)

Per `.A.8` análise dos 2 eixos:
- Eixo 1: snapshot final (resolução determinística após
  walk completo).
- Eixo 2: dados presentes em produção.

Conclusão: **sem necessidade de variante location-aware**.
Diferente de P185B (`is_numbering_active_at` /
`flat_counter_at`) que precisaram porque consumer
location-aware Layouter (heading prefix re-update).

`resolved_labels` é write-once durante walk; cada label
tem texto único; consumer lê snapshot final.

Output: decisão fixada — sem variante `*_at`.

### Sub-passo 193A.G — Decisão cláusula 6 (compat com legacy)

Durante janela compat:
- Walks (E2/E4) continuam a mutar `state.resolved_labels`.
- Consumer C4 continua a ler `state.resolved_labels`.
- Sub-store novo é vazio em produção (sem arm de populate
  per cláusula 3 Opção A).

Quando P195 (migrar Labelled) executar:
- Walk Labelled emite Tag.
- `from_tags` arm Labelled popula sub-store novo.
- Legacy também é populated (write paralelo durante janela
  compat).

Quando P196 (migrar Heading) executar:
- Idem.

Quando consumer C4 migrar (P194):
- Ler do sub-store novo via `intr.resolved_label_for(label)`.
- Fallback a `state.resolved_labels.get(label)` durante
  janela compat (substitution-with-fallback per padrão
  P184D/P187B/P188B).

Output: cenário de compat documentado.

### Sub-passo 193A.H — Decisão cláusula 7 (tipo de chave)

Confirmar empiricamente em `.A.1`:
- `state.resolved_labels: HashMap<Label, String>`? ou
- `state.resolved_labels: HashMap<String, String>`?

Replicar tipo no sub-store novo (cláusula 1).

Output: tipo de chave fixado per `.A.1`.

### Sub-passo 193A.I — Decisão cláusula 8 (critério de fecho)

P193 fecha quando:
- Struct `ResolvedLabelStore` adicionado.
- Field em `TagIntrospector` adicionado.
- Método trait `resolved_label_for` adicionado.
- Tests unit cobrem populate manual + lookup.
- L0s actualizados (`entities/introspector.md` ou
  similar).

**Não exige**:
- Walk arms migrados (P195+).
- Consumer C4 migrado (P194).
- Populate em produção real (vazio).

Output: critério literal verificável.

### Sub-passo 193A.J — Validação do plano de sub-passos

Tabela esperada:

| Sub-passo | Escopo | Magnitude |
|-----------|--------|-----------|
| `.B` | Adicionar `ResolvedLabelStore` struct + field em `TagIntrospector` + método trait + tests unit + L0s + relatório consolidado P193 | S–M |

Sub-passo único agregado. **Alternativa**: dividir em 2
sub-passos (B = struct + field; C = trait + tests) se
magnitude empírica em `.A` justificar.

Output: tabela final.

### Sub-passo 193A.K — ADR

Avaliar:

- Sub-store paralelo a BibStore — replicação P181B.
- Método trait simples — replicação P181F.
- Sem semântica nova; sem decisão arquitectural disruptiva.

Conclusão esperada: **não cria ADR**.

**Excepção**: se `.A.4` (vanilla typst) revelar que
resolução de cross-references tem nuances que afectam
forma do sub-store (ex.: ambiguidade label → multiple
locations), ADR `PROPOSTO`.

### Sub-passo 193A.L — DEBT

P193 não abre DEBT novo. Mas **actualiza nota DEBT
M5-residual** per P189 §8:
- Antes P193: 6 excepções (E1–E6) bloqueadas por 4
  pré-requisitos.
- Após P193: 6 excepções; 1 dos 4 pré-requisitos avançou
  (sub-store `resolved_labels` aberto). Restantes 3:
  C4 migration, sub-store `headings_for_toc`,
  `SetEquationNumbering`.
- Estado actualizado em relatório consolidado P193.

Sem DEBT formal aberto — Cenário B per P189A §8.

Output: cenário identificado.

### Sub-passo 193A.M — Outputs

Produzir 3 ficheiros (padrão P181A–P189A):

1. **`00_nucleo/diagnosticos/diagnostico-resolved-labels-store-passo-193a.md`**
   — diagnóstico com 8 secções:
   - §1 Validação estado actual + análise dos 2 eixos.
   - §2 Decisões cláusula 1–8 (formato O1–O5).
   - §3 Plano de sub-passos sem condicionais.
   - §4 Magnitude consolidada.
   - §5 ADR avaliação.
   - §6 DEBT avaliação (M5-residual progresso).
   - §7 Relação com P189 §9 sequência (P193 = passo 1).
   - §8 Próximo sub-passo (P193B com escopo concreto).

2. **`00_nucleo/materialization/typst-passo-193a-relatorio.md`**
   — relatório com 14 secções (padrão P181A/etc.).

3. **Sem ADR e sem DEBT formal esperados**.

---

## Restrições

- **Zero código tocado** em qualquer ficheiro fora de
  `00_nucleo/`.
- **Zero testes** modificados.
- **Não criar reservas** de identificadores.
- **Não modificar trait `Introspector`** — P193B.
- **Não modificar `TagIntrospector`** — P193B.
- **Não criar struct `ResolvedLabelStore`** — P193B.
- **Não modificar walk arms** — P195+.
- **Não modificar consumer C4** — P194.
- **Não modificar `from_tags`** — P195+.
- **Não inflar linguagem**: sem "patamar", "limiar",
  "consolidação", "deriva", "subpadrão", "cumulativo",
  "cross-domínio", "paridade observable" como bandeira
  retórica.
- **Honestidade obrigatória**: P193 abre infra **sem
  activar caminho**. Sub-store fica vazio em produção
  até P195+ adicionar arm de populate. Esta é janela
  compat por design — registar honestamente.
- **Sem cláusulas condicionais nos sub-passos `.B`+ do
  plano**.

---

## Critério de conclusão

- Diagnóstico em
  `00_nucleo/diagnosticos/diagnostico-resolved-labels-store-passo-193a.md`
  com 8 secções produzido.
- Relatório em
  `00_nucleo/materialization/typst-passo-193a-relatorio.md`
  com 14 secções produzido.
- 8 cláusulas fechadas com decisão literal.
- Plano de 1 sub-passo (B agregado) sem condicionais.
- Magnitude S–M agregada confirmada.
- Critério de fecho P193 fixado.
- ADR avaliada (esperado: não criada; possível PROPOSTO se
  vanilla revelar nuance).
- DEBT M5-residual progresso registado (1 dos 4
  pré-requisitos avançado).
- Análise dos 2 eixos aplicada empiricamente.
- Nenhum ficheiro de cristalino tocado.
- Tests workspace 1.815 inalterados.
- `crystalline-lint .` zero violations.

P193A é instrumento. Abertura concreta de sub-store
começa em P193B.
