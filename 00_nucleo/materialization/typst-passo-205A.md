# Passo 205A — Diagnóstico-primeiro de F3 (refactor 21 fields ortogonais via sub-stores trackable)

**Série**: 205 (sub-passo `A` = diagnóstico-primeiro
formal). 38ª aplicação consecutiva do padrão.
**Tipo**: diagnóstico-primeiro (zero código tocado) +
auditoria empírica.
**Magnitude planeada**: M (S–M auditoria + S diagnóstico).
**Pré-condição**: P204H concluído; M8 estruturalmente
fechado; ADR-0073 ACEITE 2026-05-07; ADR-0066
SUPERSEDED-BY 0073; tests 1852 verdes; 0 violations; 17
sentinelas activas em M8 preservadas; blueprint anotado
cirurgicamente.
**Output**: 4 ficheiros (auditoria + diagnóstico +
relatório + ADR-0074 PROPOSTO se C9 for afirmativa).

---

## §1 Propósito

Fixar as decisões estruturais de F3 antes de qualquer
alteração de código.

F3 — refactor dos 21 fields ortogonais do Layouter
(per snapshot 2026-05-05 §5; pós-P190I). O snapshot
consolidado §6 do diagnóstico P204A C9 declarou que
"alguns dos 21 fields ortogonais do Layouter são
candidatos a migrar para sub-stores trackable se isto
reduzir aliasing entre estado de layout e estado de
introspecção". P205A materializa essa análise.

**Forma do trabalho** (per clarificação inicial):

- F3 endereça sub-stores trackable via comemo (sinónimo
  arquitectural a M8 mas para fields layout, não para
  trait Introspector).
- Vanilla typst Layouter é referência empírica — ler
  typst-layout para identificar que fields equivalem a
  sub-stores trackable lá.

P205A produz:

1. Auditoria empírica dos 21 fields ortogonais com
   classificação face a comemo.
2. Auditoria do Layouter vanilla — quais sub-stores
   estão trackable lá.
3. Cláusulas de decisão (C1–Cn) sem condicionais.
4. ADR-0074 PROPOSTO **se C9 for afirmativa** (decisão
   sobre criar ADR fica para o diagnóstico).
5. Plano de sub-passos `*B+` sem ramos — fixado **após**
   a auditoria.

P205A respeita o padrão: cada passo começa com
inventário empírico antes de qualquer decisão.

---

## §2 Material de partida verificado em P204H

Antes de qualquer auditoria, confirmar empíricamente o
baseline pós-M8:

- Tests workspace: 1852 verdes.
- Crystalline-lint: 0 violations.
- Layouter ganha `'a` lifetime parameter (P204C).
- Field `introspector` é `Tracked<'a, dyn Introspector +
  'a>` (P204C).
- Position concrete em runtime (P204D).
- 17 sentinelas activas (3+2+2+2+6+2).
- ADR-0073 ACEITE; ADR-0066 SUPERSEDED-BY 0073.
- Layouter tem 22 fields totais (per snapshot
  2026-05-05 §5; pós-P204C).

Sem isto, recuar para P204H.

---

## §3 Cláusulas de auditoria (A1–An)

Esta secção é executada **primeiro**. Output empírico
alimenta C1+ adiante. Cada item reporta CONFIRMADO /
DIVERGÊNCIA / NÃO APLICÁVEL com evidência.

### Bloco 1 — Layouter cristalino — inventário dos 22 fields

#### A1 — Listagem completa dos 22 fields

```
grep -B 1 -A 30 "^pub struct Layouter" \
  01_core/src/rules/layout/mod.rs
```

Para cada field: nome, tipo, visibilidade, comentário se
existir.

Critério: 22 fields confirmados. O field `introspector:
Tracked<...>` (P204C) é 1 deles; outros 21 são
"ortogonais" para efeitos de F3.

#### A2 — Classificação inicial dos 21 ortogonais

Para cada field ortogonal (excluindo `introspector` que
já é Tracked), classificar em categorias preliminares:

- **Categoria A — runtime puro de layout** (cursor_x,
  cursor_y, current_page, etc.): mutável durante layout,
  sem relação com queries.
- **Categoria B — runtime de introspecção
  (LayouterRuntimeState)** (label_pages,
  known_page_numbers, is_readonly, positions): mutável
  durante layout, consumido por queries.
- **Categoria C — config de layout** (FontMetrics,
  ImageSizer params): set na construção, imutável
  durante layout.
- **Categoria D — fronteira ambígua** (current_location,
  pages, locator): pode ser A ou B dependendo da
  semântica.

Output: tabela 21 linhas × categoria preliminar +
justificação.

#### A3 — Mutabilidade actual de cada field

Para cada field:

- Onde é set?
- Onde é mutado durante layout?
- Onde é lido?
- Quantas vezes (estimativa de hot path)?

Critério: identificar fields com padrão write-once vs
mutate-frequently.

#### A4 — Aliasing entre fields

Identificar pares de fields que representam o mesmo
conceito ou são derivados um do outro. Exemplo
provável:
- `pages.len() + 1` derivar `current_page`.
- `cursor_x, cursor_y` agrupados em conceito Point.

Critério: lista de pares com sugestão de consolidação.

### Bloco 2 — Vanilla typst Layouter — referência

#### A5 — Forma do Layouter vanilla

```
grep -B 1 -A 50 "^pub struct PageLayouter\|^pub struct DocumentLayouter\|pub struct FlowLayouter" \
  lab/typst-original/crates/typst-layout/src/
```

Para cada Layouter vanilla relevante: fields,
classificação A/B/C/D análoga.

Critério: tabela paralela à de A2.

#### A6 — Sub-stores trackable em vanilla

```
grep -rn "#\[comemo::track\]\|Tracked<" \
  lab/typst-original/crates/typst-layout/src/
```

Identificar:

- Que sub-stores em vanilla têm `#[comemo::track]`
  aplicado?
- Que tipos `Tracked<...>` aparecem na assinatura do
  Layouter?
- Vanilla aplica padrão B3 (trait + blanket impl) ou
  Padrão A (literal) para layout state?

Critério: tabela de sub-stores trackable em vanilla com
forma de trackagem.

#### A7 — Mapeamento cristalino ↔ vanilla

Para cada field cristalino (A2), identificar correspondência
em vanilla (A5):

- 1:1 — mesmo nome, mesma semântica.
- 1:N — field cristalino corresponde a múltiplos vanilla.
- N:1 — múltiplos cristalino correspondem a 1 vanilla.
- ausente — sem correspondência (decisão do cristalino).

Critério: tabela de mapeamento. Fields ausentes
documentados (podem ser intencionais ou candidatos a
re-arquitectura).

### Bloco 3 — Compatibilidade com comemo

#### A8 — Bounds satisfeitos por categoria

Para fields da Categoria B (runtime de introspecção):

- `Send + Sync` automaticamente?
- Tipos retornados satisfazem `Hash`?
- Padrão de mutação é compatível com tracking imutável?

Critério: cada field B classificado quanto a
compatibilidade com `#[comemo::track]`. Hipóteses
específicas:
- `LayouterRuntimeState` actualmente acessível via
  `&self.runtime` — precisa Tracked?
- Fields populated single-pass durante layout não podem
  ser tracked durante o mesmo layout (paradox: tracking
  exige imutabilidade; population exige mutabilidade).

#### A9 — Modelo de tracking pós-layout

Investigar: vanilla aplica tracking em Layouter durante
layout ou apenas pós-layout?

Hipótese: vanilla tem `PagedIntrospector::new(pages)`
post-layout (per P203A A7) — sub-stores são populated
durante layout, sealed pós-layout, e tracked apenas
post-sealing.

Cristalino diverge intencionalmente (single-pass).
Verificar se F3 mantém divergência ou converge para
modelo vanilla.

### Bloco 4 — Loops fixpoint e F3

#### A10 — Loops fixpoint cristalinos vs F3

Os 2 loops fixpoint (TOC + run_fixpoint, MAX=5) usam
convergência por hash. F3 sub-stores trackable
afectam-nos?

- Hash actual continua válido?
- Tracking comemo + hash convergence coexistem?
- F3 reduz iterações esperadas?

Critério: declaração explícita de impacto.

#### A11 — Position concrete (P204D) e F3

P204D criou `runtime.positions`. F3 tornaria isto
trackable?

Hipótese: positions é populated durante layout e lido
em queries. Padrão é trackable se sealing pós-layout
existir, não-trackable se single-pass apenas.

### Bloco 5 — Estado pós-M8 e oportunidades

#### A12 — Sub-stores cristalinos elegíveis

Listar sub-stores em Layouter cristalino (não em
TagIntrospector — esse fica em M8) que satisfaçam:

- Categoria B (runtime introspecção).
- Sealable pós-layout (sem mutação após fim de pages).
- Tipos com Hash impl ou facilmente adicionáveis.

Critério: candidatos concretos para F3 com magnitude
estimada de migração por candidato.

#### A13 — Sub-stores ineligíveis

Lista sub-stores que NÃO são candidatos:

- Categoria A (runtime puro layout).
- Categoria C (config; Tracked irrelevante).
- Categoria B mas sem sealing point.

Documentar razão por candidato.

#### A14 — Performance e benefício

Estimativa qualitativa: F3 reduz overhead de queries
repetidas durante layout? Vanilla mostra ganho mensurável
com Layouter sub-stores trackable?

Critério: declaração honesta sem benchmarks (esses ficam
para sub-passo dedicado se F3 prosseguir).

---

## §4 Cláusulas de decisão (C1–Cn)

Estas cláusulas são fixadas **depois** da auditoria, com
base no output empírico. Cada uma é fixada sem
condicionais.

### C1 — Escopo de F3

Decisão central. Range plausível:

- **Mínimo** — 1–2 sub-stores trackable em
  `LayouterRuntimeState` (provavelmente `positions` +
  `label_pages`).
- **Médio** — todos os fields Categoria B sealable
  trackable.
- **Completo** — Categoria B + reorganização de
  Categoria D + alinhamento com modelo vanilla
  (post-layout sealing).

C1 fixa-se com base em A2 + A5 + A12 + A13.

### C2 — Modelo de tracking

Com base em A9:

- **Single-pass** — manter divergência intencional do
  cristalino; fields B trackable apenas após population
  via sealing explícito.
- **Post-layout vanilla-like** — `LayoutResult` ou
  similar gera sub-stores sealed que são tracked
  posteriormente.
- **Híbrido** — alguns fields single-pass, outros
  post-layout.

C2 fixa **uma** alternativa.

### C3 — Mecanismo de tracking

Com base em A6 + A8:

- **Padrão A (literal)** — `#[comemo::track]` em trait
  específica de `LayouterRuntimeState` (ou similar).
- **Padrão B3** — trait pura + trait Tracked + blanket
  impl.
- **Padrão C — sub-trait** — múltiplas traits para
  granularidade fina.

Padrão A é favorito (paridade com M8). Decisão fixa-se
com base em A8 (bounds satisfeitos).

### C4 — Sealing point

Se C2 = single-pass ou híbrido, identificar o ponto
literal onde `LayouterRuntimeState` (ou candidato) é
sealed:

- Após `pages` finalizadas?
- No fim de `pub fn layout`?
- Outro?

Fixa-se com base em A3 + A11.

### C5 — Compatibilidade com fixpoint

Com base em A10 — fixar uma:

- **Coexistência** — F3 sub-stores tracked em paralelo
  com hash convergence (preferido).
- **Substituição** — F3 substitui hash convergence por
  tracking-based.

C5 não tem ramos.

### C6 — Position e F3

Com base em A11:

- **Position trackable** — `runtime.positions` ganha
  tracking; `position_of` retorna `Some(Position)` real
  via TagIntrospector com sub-store sincronizado.
- **Position permanece em runtime** — F3 não toca
  Position; consumers continuam a aceder via
  `layouter.runtime.positions.get(loc)` (Padrão C6a do
  diagnóstico P204D).

C6 fixa **uma** alternativa.

### C7 — Lacunas residuais e F3

P204H confirmou zero lacunas formalmente catalogadas.
F3 abre lacunas novas? Se sim, registar.

Hipótese: fields Categoria D ambíguos podem virar
lacuna se F3 não as resolver.

### C8 — Magnitude agregada

Soma das decisões C1–C7. Range plausível: M (escopo
mínimo, single-pass) a XL (escopo completo,
post-layout, todos os fields B).

C8 é output. Não pré-fixada.

### C9 — ADR-0074 PROPOSTO?

Decisão: criar ADR-0074 PROPOSTO para F3 ou não.

Critério:

- **Sim, criar** — F3 é decisão arquitectural com
  alternativas reais (modelo de tracking, padrão de
  adopção).
- **Não, documentação inline** — F3 é continuação
  natural de ADR-0073 e fica documentada como sub-passo
  (Padrão emergente de M8).

Hipótese mais provável: criar ADR-0074. M8 estabeleceu
padrão para `Introspector`; F3 estende para Layouter
sub-stores com decisões análogas. ADR dedicada permite
cross-reference.

C9 fixa decisão. Se afirmativa, ADR-0074 é Output 4.

### C10 — Sub-passos `*B+`

Plano sem ramos. Quantidade depende de C1–C9.

**Não pré-defino sub-passos B–G nesta spec.** Plano `*B+`
emerge do diagnóstico.

### C11 — Sem cláusulas condicionais

C1–C10 fixadas com valores concretos.

---

## §5 Outputs concretos

Quatro ficheiros (3 sempre + 1 condicional em C9):

### Ficheiro 1 — Auditoria empírica

Localização:
`00_nucleo/diagnosticos/typst-passo-205A-auditoria-f3.md`.

Conteúdo: A1–A14 com etiquetas e evidência. Sem
decisões.

### Ficheiro 2 — Diagnóstico

Localização:
`00_nucleo/diagnosticos/typst-passo-205A-diagnostico.md`.

Conteúdo: cláusulas C1–C11 instanciadas. Plano `*B+`
final.

### Ficheiro 3 — Relatório

Localização:
`00_nucleo/materialization/typst-passo-205A-relatorio.md`.

### Ficheiro 4 — ADR-0074 PROPOSTO (condicional em C9)

Localização (se C9 = afirmativa):
`00_nucleo/adr/typst-adr-0074-f3-layouter-substores-trackable.md`.

Estado: PROPOSTO. Estrutura per `template-adr.md`.

---

## §6 Critério de progressão para `*B`

P205A só transita para `*B` quando:

- A1–A14 todos com etiqueta CONFIRMADO ou DIVERGÊNCIA
  registada.
- C1–C11 instanciadas com valores concretos.
- ADR-0074 PROPOSTO escrito (se C9 afirmativa).
- Magnitude calibrada (C8).
- Plano `*B+` sem condicionais.

Em caso de divergência empírica relevante face ao
snapshot 2026-05-05 (ex: Layouter não tem 22 fields,
LayouterRuntimeState divergir do esperado, vanilla
typst-layout reorganizado), registar em `P205A.div-N`
e:

- Ignorar (divergência ortogonal).
- Corrigir snapshot (sub-passo administrativo separado).
- Ramificar (divergência bloqueia F3).

---

## §7 Convenções mantidas

- Sem código Rust nas specs.
- Sem condicionais.
- Cada passo começa com inventário empírico.
- 4 outputs (3 sempre + 1 condicional).
- Localização canónica:
  `00_nucleo/diagnosticos/` para auditoria/diagnóstico;
  `00_nucleo/materialization/` para relatório;
  `00_nucleo/adr/` para ADR.
- Sem inflação retórica.

---

## §8 Não-objectivos

P205A não:

- Toca em código.
- Aplica `#[comemo::track]` em qualquer Layouter
  sub-store.
- Migra fields para sub-stores.
- Materializa Position trackable (decisão fica para C6).
- Promove ADR-0067 (PROPOSTO mantém-se).
- Pré-define sub-passos `*B+` (esses emergem de C10).
- Decide escopo de F3 antes da auditoria — C1 fixa-se
  com base em A1–A14.
- Endereça vanilla integration (DEBT-53/54) — fica para
  série dedicada P206 ou similar.
- Toca em ADR-0073 (já ACEITE) ou ADR-0066 (já
  SUPERSEDED-BY).

---

## §9 Erro a não repetir

Da série P204 — 6 sub-passos sem inflação. P205 segue
mesmo padrão.

Risco específico de P205A: pode haver tentação de
**adoptar Padrão B3** (per ADR-0005) por simetria
arquitectónica, sem verificar empíricamente que Padrão A
funciona (per lição de M8 onde Padrão A literal foi
escolhido após verificação empírica).

C3 não pré-fixa padrão. Fica para a auditoria decidir.

Hipótese específica: vanilla pode usar Padrão diferente
para Layouter sub-stores vs Introspector. Per A6, P205A
verifica empíricamente antes de C3.

Outro risco: F3 pode ter escopo XL se C1 = "completo" +
C2 = "post-layout vanilla-like". Magnitude L
cross-modular semelhante a M8 série inteira. **Não
pré-fixei escopo nesta spec** — fica para C1 com base
em auditoria.

---

## §10 Particularidade — execução

P205A é diagnóstico de profundidade média a alta:

- 22 fields do Layouter cristalino para classificar.
- Pipeline completo do Layouter vanilla (typst-layout
  crate inteiro).
- Mapeamento empírico cristalino ↔ vanilla.
- Análise de compatibilidade comemo para sub-stores
  Categoria B.

Volume médio. Magnitude M.

Recomendado Claude Code dado:

- Volume de leitura para A5–A7 (vanilla typst-layout).
- Iteração sobre fields ambíguos (Categoria D).
- Decisão arquitectural em C2 que beneficia de comparação
  detalhada vanilla.

Sessão actual viável se C1 não revelar obstrução
estrutural.
