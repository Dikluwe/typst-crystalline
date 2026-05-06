# Passo 204A — Diagnóstico-primeiro de M8 (comemo / paridade vanilla)

**Série**: 204 (sub-passo `A` = diagnóstico-primeiro
formal). 37ª aplicação consecutiva do padrão
diagnóstico-primeiro.
**Tipo**: diagnóstico-primeiro (zero código tocado) +
auditoria empírica de profundidade máxima.
**Magnitude planeada**: M (S–M para auditoria + S para
diagnóstico).
**Pré-condição**: P203 série fechada; tests 1824 verdes;
0 violations; pré-condição arquitectónica de M8 totalmente
cumprida (M5+M6+M7+M9 fechados; F1 fechado; F3 parcial;
zero lacunas residuais formalmente catalogadas).
**Output**: 4 ficheiros (auditoria + diagnóstico +
relatório + ADR-0073 PROPOSTO).

---

## §1 Propósito

Fixar as decisões estruturais de M8 antes de qualquer
alteração de código. M8 é o marco que completa o trabalho
deixado em aberto por M7 (fecho estrutural sem paridade
arquitectural) e cobre Position concrete naturalmente
(per ADR-0066 + P203 consolidado §13).

P204A produz:

1. Auditoria empírica de profundidade máxima — estado
   actual da trait `Introspector`, uso actual de
   `comemo`, pipeline vanilla typst, invariantes do crate
   `comemo` (tracking, lifetimes, performance).
2. Cláusulas de decisão (C1–Cn) sem condicionais — escopo
   de M8 fica para esta secção; não é pré-fixado pela
   spec.
3. ADR-0073 PROPOSTO sobre adopção de `comemo` no trait
   `Introspector`.
4. Plano de sub-passos `*B+` sem ramos — fixado **após**
   a auditoria, baseado no que ela revelar.

P204A não toca em código. Não escreve `.rs`. Não modifica
`Cargo.toml`. Não promove ADRs.

---

## §2 Material de partida verificado em P203

Antes de qualquer auditoria, confirmar empíricamente o
baseline pós-P203:

- Tests workspace: 1824 verdes.
- Crystalline-lint: 0 violations.
- Trait `Introspector`: 20 métodos.
- `TagIntrospector`: 9 sub-stores listados.
- `Layouter`: 22 fields, sem `counter`.
- Walk fn: 7 parâmetros.
- 2 loops fixpoint, MAX=5.
- `comemo` versão 0.4 declarada em `Cargo.toml` workspace.
- Position: stub `position_of() -> Option<()>`; 0
  consumers em produção.
- Lacunas residuais: zero formalmente catalogadas.
- ADRs ACEITES no ciclo M5/M6/M7: 6 estritas + 2 EM
  VIGOR + 3 PROPOSTAS.

Sem isto, a auditoria não tem fundamento. Recuar para
P203 se algum item não estiver confirmado.

---

## §3 Cláusulas de auditoria (A1–An)

Esta secção é executada **primeiro**. Output empírico
alimenta C1+ adiante. Cada item reporta CONFIRMADO /
DIVERGÊNCIA / NÃO APLICÁVEL com evidência.

### Bloco 1 — Trait `Introspector` cristalino

#### A1 — Listagem completa dos 20 métodos

```
grep -n "^    fn \|^    pub fn " 01_core/src/entities/introspector.rs
```

Para cada método: nome, assinatura, retorno, callers.

Critério: 20 métodos confirmados. Se a contagem divergir,
registar `P204A.div-N`.

#### A2 — Mutabilidade de cada método

Para cada método, classificar:

- **Read-only** — `&self` e retorna valor sem mutar
  state.
- **Mut-during-build** — `&mut self`, usado durante
  populate.
- **Misto** — usa `&self` mas internamente acede a
  estado mutável (raro mas possível).

Critério: cada método etiquetado. `comemo::track` requer
read-only; mut-during-build precisa de tratamento
distinto.

#### A3 — Consumers em produção

Para cada método, listar call sites:

```
grep -rn "\.<method_name>(" 01_core/ 02_shell/ 03_infra/ \
  04_wiring/
```

Critério: cada método com lista de callers. Métodos sem
callers em produção (stubs) marcados explicitamente.

#### A4 — Sub-stores `TagIntrospector` — granularidade

Para cada um dos 9 sub-stores (`labels`, `counters`,
`kind_index`, `figure_label_numbers`, `metadata`, `state`,
`bib_store`, `resolved_labels`, `headings_for_toc`):

- Tipo concreto.
- Quando é populado (walk-time / populate_intr / outro).
- Quando é lido (consumers).
- Granularidade de invalidação (sub-store inteiro vs
  per-key).

Critério: cada sub-store classificado. Granularidade de
invalidação é input para C5 (re-walks parciais).

### Bloco 2 — Uso actual de `comemo` no projecto

#### A5 — Uso existente de `comemo`

```
grep -rn "comemo::\|#\[comemo::\|use comemo" \
  01_core/ 02_shell/ 03_infra/ 04_wiring/
```

Critério: lista exaustiva de usos actuais. Pontos de
atenção:

- Onde está `Tracked<dyn TrackedWorld>` ou similar.
- Se há `#[comemo::track]` em alguma trait existente
  (referência: ADR-0005 padrão B3).
- Se há funções memoizadas com `#[comemo::memoize]`.

#### A6 — Versão exacta e API disponível

```
grep "^comemo" Cargo.lock
ls ~/.cargo/registry/src/*/comemo-*/src/ 2>/dev/null
grep -n "^pub fn\|^pub trait\|^pub macro\|#\[macro\|#\[proc_macro" \
  ~/.cargo/registry/src/*/comemo-*/src/lib.rs 2>/dev/null
```

Critério:

- Versão exacta confirmada (X.Y.Z).
- API disponível: `track`, `track_mut`, `evict`,
  `Tracked`, `TrackedMut`, `#[memoize]`, `#[track]`.
- Confirmar se a versão suporta `#[comemo::track]` em
  métodos de trait (questão crítica para C2).

### Bloco 3 — Vanilla typst — pipeline `Introspector`

#### A7 — Trait vanilla `Introspector`

```
grep -B 2 -A 50 "^pub trait Introspector\|^pub struct Introspector" \
  lab/typst-original/crates/typst-library/src/introspection/introspector.rs
```

Para cada método vanilla:
- Nome.
- Assinatura.
- Tem `#[comemo::track]` aplicado?
- Equivalente cristalino (correspondência 1:1, mapeamento
  many-to-one, ou ausente).

Critério: tabela `método cristalino ↔ método vanilla` com
cobertura. Métodos vanilla sem equivalente cristalino são
gaps potenciais (ou intencionalmente ausentes).

#### A8 — Pipeline vanilla — fluxo completo

```
grep -rn "Introspector::new\|#\[comemo::memoize\]" \
  lab/typst-original/crates/typst-layout/src/
grep -rn "Tracked<dyn Introspector>\|Tracked<.*Introspector>" \
  lab/typst-original/crates/typst-library/src/
grep -rn "Tracked<dyn Introspector>\|Tracked<.*Introspector>" \
  lab/typst-original/crates/typst-layout/src/
```

Critério: descrever o fluxo completo:

- Onde `Introspector` é construído.
- Onde é trackado pela primeira vez.
- Que consumers usam `Tracked<dyn Introspector>`.
- Como interage com fixpoint (vanilla tem fixpoint?).
- Quantas iterações típicas em corpus realista.

#### A9 — Cache invalidation no vanilla

Como vanilla decide quando invalidar entradas memoizadas?
Procurar:

- `comemo::evict` calls.
- Lifetime de `Tracked` instances.
- Strategy de cache (TTL? per-document? per-query?).

Critério: descrever política. Pode informar C6 (política
cristalina).

### Bloco 4 — Invariantes do crate `comemo`

#### A10 — Tracking constraints

Ler README e exemplos do crate:

```
ls ~/.cargo/registry/src/*/comemo-*/
cat ~/.cargo/registry/src/*/comemo-*/README.md 2>/dev/null | head -100
ls ~/.cargo/registry/src/*/comemo-*/examples/ 2>/dev/null
```

Identificar:

- Que tipos podem ser argumentos de função tracked?
  (Hash? Tracked? specific trait bounds?)
- Que limitações tem `#[comemo::track]` em traits?
- Como composição funciona (Tracked of Tracked)?
- Side effects são permitidos em métodos tracked?

Critério: lista literal de constraints. Cada uma com fonte
(README, exemplo, código fonte).

#### A11 — Lifetimes em `Tracked`

Investigar como `Tracked<'a, T>` se comporta:

- Lifetime relativo ao `T` original.
- Pode ser armazenado em struct?
- Pode atravessar fronteira de função?
- Como interage com mutable references?

Critério: documentar literalmente. Crítico para C3
(Layouter consumers).

#### A12 — Performance characteristics

Investigar:

- Custo de cache lookup (hash da entrada de tracking).
- Custo de invalidação (`evict`).
- Memory overhead (por entrada cached).
- Threading guarantees (Send/Sync).

Critério: tabela com ordem de magnitude (constante,
linear, depende). Informa C7 (decisão de adopção).

### Bloco 5 — Estado cristalino face a M8

#### A13 — Loops fixpoint actuais — interacção com comemo

Os 2 loops fixpoint cristalinos (TOC + run_fixpoint, MAX=5)
usam convergência por hash. Investigar:

- Hash actual: `compute_tags_hash` (introspect/fixpoint.rs)
  e `extracted_label_pages` (layout/mod.rs).
- Compatibilidade conceptual com tracking comemo:
  - Hash-based detecta convergência via igualdade;
    `comemo` invalida entradas via tracking de
    dependências.
  - São mecanismos ortogonais ou conflituosos?

Critério: declaração explícita "ortogonais" ou
"conflituosos" com justificação.

#### A14 — F3 parcial — fields ortogonais Layouter

21 fields ortogonais do Layouter pendentes (F3 parcial).
Investigar:

- Quais são candidatos a migração para sub-stores
  trackable (similar a `runtime: LayouterRuntimeState`).
- Quais são puros runtime de layout sem relação com
  introspecção.

Critério: classificação dos 21 fields em categorias
A/B/C:
- A — candidato a sub-store trackable.
- B — runtime puro (não migra).
- C — caso ambíguo (decisão fica para sub-passos M8
  posteriores).

#### A15 — Corpus de paridade actual

```
ls lab/parity/corpus/ 2>/dev/null
wc -l lab/parity/corpus/*.typ 2>/dev/null | tail -5
```

Critério:

- Tamanho do corpus actual.
- Cobertura de features que envolvem introspection
  (TOC, contadores, figuras, citações, equações
  numeradas).
- Casos de paridade que actualmente passam vs falham.
- Casos que precisariam de Position concrete.

#### A16 — Position concrete — escopo para M8

Per P203 consolidado §13: Position fica para M8. P204A
deve auditar:

- Se M8 deve materializar Position como parte do trabalho
  comemo.
- Ou Position fica para sub-passo separado dentro de M8.
- Ou Position requere sub-marco distinto (M8.5?).

Critério: recomendação fundamentada em A1–A15.

---

## §4 Cláusulas de decisão (C1–Cn)

Estas cláusulas são fixadas **depois** da auditoria, com
base no output empírico. Cada uma é fixada sem
condicionais.

### C1 — Escopo de M8

Decisão central. Range plausível:

- **Mínimo** — adopção `#[comemo::track]` em
  `Introspector`; consumers Layouter migram.
- **Médio** — Mínimo + sub-stores trackable
  selectivamente + queries location-aware re-emitidas
  com tracking granular.
- **Completo** — Médio + Position concrete + validação
  saída cristalino == vanilla (corpus paridade) +
  benchmarks.

C1 fixa-se com base em A1–A16. Não é pré-fixada pela
spec.

### C2 — Mecanismo de adopção `comemo` no trait

Padrão B3 (per ADR-0005) é candidato natural:

- `Introspector` puro (sem `comemo`).
- `TrackedIntrospector` com `#[comemo::track]`.
- Blanket impl `<T: Introspector> TrackedIntrospector for T`.

Mas decisão pode divergir se A6/A10 revelar limitações.
Alternativas:

- `#[comemo::track]` directamente em `Introspector` (se
  versão suportar).
- Funções livres memoizadas em vez de métodos.
- Sub-trait dedicada para subset de métodos trackable
  (deixar mut-during-build fora).

C2 fixa-se com base em A2 (mutabilidade) + A6 (API
versão) + A10 (constraints).

### C3 — Layouter consumers — assinatura

Como Layouter passa a aceitar `Introspector`?

Alternativas:

- `&dyn Introspector` (actual).
- `Tracked<'a, dyn Introspector>` (single-pass).
- `Tracked<'a, dyn TrackedIntrospector>` (padrão B3).

C3 fixa-se com base em A11 (lifetimes) + C2 (mecanismo).

### C4 — Sub-stores trackable selectivamente

Quais sub-stores ganham granularidade de tracking?

C4 lista literalmente:

- Sub-store X — trackable porque...
- Sub-store Y — não trackable porque...
- Sub-store Z — decisão adiada.

Baseado em A4 (granularidade) + A10 (constraints).

### C5 — Re-walks parciais

Re-walks parciais via invalidação cross-iteration são
parte de M8?

Alternativas:

- **Sim** — comemo invalida entradas tocadas; walk
  re-emite só o necessário.
- **Não** — walk continua single-pass; comemo apenas
  memoíza queries sobre output do walk.

C5 fixa-se com base em A8 (vanilla pipeline) + A13
(fixpoint actual).

### C6 — Política de invalidação

Quando entradas cached são invalidadas?

Alternativas:

- Per-document (lifetime do documento).
- Per-query (após cada query).
- Híbrido (per-document para sub-stores estáveis;
  per-query para volatile).

C6 fixa-se com base em A9 (vanilla policy) + A12
(performance).

### C7 — Loops fixpoint cristalinos — manter ou
substituir

Se A13 declarar "ortogonais": loops fixpoint coexistem
com comemo. Se A13 declarar "conflituosos": M8 substitui
fixpoint por comemo.

C7 fixa-se com base em A13. Sem condicionais — uma
decisão.

### C8 — Position concrete — escopo dentro de M8

Decisão entre 3 alternativas (per A16):

- **Sub-passo M8** — Position é trabalho concreto de
  `*B+` dentro de M8.
- **Sub-marco M8.5** — Position é marco separado depois
  de M8 base.
- **Adiada para pós-M8** — M8 não cobre Position; outro
  passo dedicado depois.

C8 fixa-se com base em A16 + C1.

### C9 — Validação de paridade

Validação cristalino == vanilla é parte de M8?

Alternativas:

- **Sim** — corpus de paridade estendido para
  introspection; sub-passo dedicado.
- **Sim em escala reduzida** — apenas sanity checks; full
  corpus fica para passo posterior.
- **Não** — M8 não inclui validação de paridade.

C9 fixa-se com base em A15 (corpus actual) + C1.

### C10 — Benchmarks

Benchmarks são parte de M8?

Alternativas:

- **Sim** — 3 configurações (cristalino sem comemo,
  cristalino com comemo, vanilla).
- **Sim em escala reduzida** — apenas measurements
  internos sem comparação vanilla.
- **Não** — sem benchmarks em M8.

C10 fixa-se com base em A12 + C1.

### C11 — Magnitude agregada

Soma das decisões C1–C10. Range plausível: M (escopo
mínimo) a XL (escopo completo + Position + validação +
benchmarks).

C11 é output. Não é pré-fixada.

### C12 — Sub-passos `*B+`

Plano de sub-passos `*B+` sem condicionais. Quantidade e
forma dependem de C1–C11.

**Não pré-defino sub-passos B–G nesta spec.** O erro de
P193A não é repetido. Plano `*B+` emerge de P204A após
A1–A16 e C1–C11.

### C13 — ADR-0073 PROPOSTO

ADR-0073 cobre adopção de `comemo` no trait
`Introspector`. Estrutura padrão (template-adr.md):

- Contexto (M5+M6+M7 estruturalmente fechados; baseline
  empírico reconciliado; lacunas zeradas).
- Decisão (mecanismo escolhido em C2).
- Consequências (positivas, negativas, neutras).
- Alternativas consideradas (de C2).
- Referências.

ADR-0073 transita PROPOSTO → ACEITE estrutural quando
sub-passo de adopção termina; ACEITE final quando M8
fecha (per snapshot §8).

### C14 — Sem cláusulas condicionais

C1–C13 fixadas com valores concretos, não com ramos.

---

## §5 Outputs concretos

Quatro ficheiros:

### Ficheiro 1 — Auditoria empírica

Localização:
`00_nucleo/diagnosticos/typst-passo-204A-auditoria-comemo.md`.

Conteúdo: A1–A16 com etiquetas e evidência. Sem decisões.

### Ficheiro 2 — Diagnóstico

Localização:
`00_nucleo/diagnosticos/typst-passo-204A-diagnostico.md`.

Conteúdo: cláusulas C1–C14 instanciadas. Cita auditoria.
Inclui plano `*B+` final (output de C12).

### Ficheiro 3 — Relatório

Localização:
`00_nucleo/materialization/typst-passo-204A-relatorio.md`.

Conteúdo: o que foi feito, tempo de execução, decisões
durante a leitura, magnitude calibrada, sugestão para
P204B.

### Ficheiro 4 — ADR-0073 PROPOSTO

Localização:
`00_nucleo/adr/typst-adr-0073-comemo-introspector.md`.

Estado: PROPOSTO. Estrutura per `template-adr.md`.

---

## §6 Critério de progressão para `*B`

P204A só transita para `*B` quando:

- A1–A16 todos com etiqueta CONFIRMADO ou DIVERGÊNCIA
  registada.
- C1–C14 instanciadas com valores concretos.
- ADR-0073 PROPOSTO escrito.
- Magnitude calibrada (C11).
- Plano `*B+` sem condicionais.

Em caso de divergência empírica relevante face ao
snapshot 2026-05-05 (ex: trait `Introspector` não ter 20
métodos, sub-stores divergirem), registar em
`P204A.div-N` e:

- Ignorar (divergência ortogonal).
- Corrigir snapshot (P205 administrativo).
- Ramificar (divergência bloqueia M8).

---

## §7 Convenções mantidas

- Sem código Rust nas specs.
- Sem `if`, sem ramos, sem condicionais.
- Cada passo começa com inventário empírico (per
  convenção P203 §9.1) — A1–A16 são inventário antes de
  C1+.
- 4 outputs (auditoria + diagnóstico + relatório + ADR).
- Localização canónica:
  `00_nucleo/diagnosticos/` e `00_nucleo/materialization/`.
- Distinção fecho estrutural vs arquitectural mantida.
- Sem inflação retórica.

---

## §8 Não-objectivos

P204A não:

- Toca em código.
- Aplica `#[comemo::track]` em qualquer ficheiro.
- Migra Layouter consumers.
- Materializa Position.
- Promove ADR-0067.
- Pré-define sub-passos `*B+` (esses emergem de C12 com
  base em A1–A16 + C1–C11).
- Decide o escopo de M8 antes da auditoria — C1 fixa-se
  com base em A1–A16, não por afirmação herdada.

---

## §9 Erro a não repetir

Da série P203 — duas detecções consecutivas de premissas
erradas em specs herdadas. Padrão correcto: cada passo
começa com inventário empírico.

P204A aplica isso com profundidade máxima — 16 cláusulas
de auditoria (A1–A16) cobrindo 5 blocos (trait
cristalino / comemo actual / vanilla pipeline /
invariantes comemo / estado pré-M8). Sem essa profundidade,
M8 corre risco de adoptar mecanismo errado por afirmação
herdada.

P204A spec não pré-define escopo de M8 (C1) — fica para
ser fixado com base em A1–A16. P204A spec não pré-define
mecanismo de adopção (C2) — fica para ser fixado com
base em A2 + A6 + A10. E assim por diante.

Sub-passos `*B+` emergem de C12 — não são pré-fixados
nesta spec.

---

## §10 Particularidade — execução

P204A é diagnóstico de profundidade máxima. Volume de
leitura é maior que P203A (que tinha foco em Position):

- 20 métodos do `Introspector` cristalino.
- Pipeline completo do vanilla `Introspector`.
- README + examples + source do crate `comemo`.
- 21 fields ortogonais do Layouter para classificação F3.

Se executado pela sessão actual (Opus, conversacional),
exige sessão dedicada com bash_tool. Se executado pelo
Claude Code, segue padrão dos diagnósticos anteriores.

Decisão entre sessão actual e Claude Code fica para o
humano. Recomendado Claude Code dado o volume e a
profundidade requerida.
