# Passo 108.D — Candidatos a sub-escopo da primeira materialização

**Data**: 2026-04-23
**Input**: 108.A (vanilla), 108.B (cristalino), 108.C (dependências).
**Propósito**: listar 3-5 candidatos a primeira materialização de
Introspection, com tamanho estimado, DEBTs desbloqueados e
limitações.

**Escala de tamanho** (referência):
- **Passo 104** (Sink materializado): ~200 linhas L1, 8 testes L1,
  0 testes L3. **Médio-pequeno**.
- **Passo 107** (propagação sink + DEBT-49): ~60 linhas L1, +6
  testes L3, 7 ficheiros tocados. **Médio**.
- **Passo 100** (StyleChain + Content::Styled):
  muitas linhas, refactor profundo. **Grande**.
- **Passo 102** (`#set text(fill)` activação): ~80 linhas L1, ~5
  testes. **Pequeno**.

O spec 108 pede pelo menos 1 candidato "≤ Passo 104 em tamanho".

---

## Candidato 1 — `Location` mínima como tipo opaco

### O que

Criar `entities/location.rs` com:

```rust
pub struct Location(u128);

impl Location {
    pub fn new(hash: u128) -> Self { ... }
    pub fn hash(&self) -> u128 { ... }
    pub fn variant(&self, n: usize) -> Self { ... }
}
```

Sem integração com Content, sem Locator, sem Introspector.

### Tipos a materializar

- `Location` — 1 struct, ~5 métodos.

### DEBTs que desbloqueia

**Nenhum aberto hoje.** Infra-estrutura pura.

### Tamanho estimado

**Muito pequeno** — ~80-100 linhas L1 + testes. Metade de Passo 102.

### Dependências necessárias

- Nenhuma (`u128` de stdlib).

### O que o candidato NÃO resolve

- Nenhum user-facing behavior.
- Nenhuma chamada de eval nova.
- Nenhum DEBT-1 residual.

### Avaliação

**Não recomendado como sub-escopo único.** Location sozinha não
tem valor. Serve apenas como "tijolo" que seria consumido pelo
próximo passo. Violaria o padrão dos últimos passos: cada passo
paga valor visível.

**Alternativa**: incluir Location **dentro** de outro candidato
(combinado) quando ele precisar.

---

## Candidato 2 — `Introspector` wrapping `CounterState`

### O que

Criar `entities/introspector.rs` que **wrapea** o `CounterState`
actual e expõe API limpa:

```rust
pub struct Introspector {
    state: CounterState,
}

impl Introspector {
    pub fn from_content(content: &Content) -> Self {
        Self { state: introspect::introspect(content) }
    }
    pub fn headings(&self) -> &[(Label, Content, usize)] { ... }
    pub fn figure_number(&self, label: &Label) -> Option<usize> { ... }
    pub fn resolved_label(&self, label: &Label) -> Option<&str> { ... }
    pub fn label_page(&self, label: &Label) -> Option<usize> { ... }
    // etc. — encapsula os campos hoje públicos do CounterState.
}
```

**Nenhuma** funcionalidade nova para o utilizador. Apenas melhor
API interna — `CounterState::resolved_labels` deixa de ser campo
público; acesso vai por `introspector.resolved_label(lbl)`.

### Tipos a materializar

- `Introspector` — 1 struct.
- Opcional: `Selector` estendido para suportar `Elem(NodeKind)`
  + filtros básicos.

### DEBTs que desbloqueia

**Nenhum**. Mas prepara terreno para:

- Candidato 3 (Query) — usa `Introspector` directamente.
- Candidato 4 (Counter.at) — Introspector ganha `count_before`.

### Tamanho estimado

**Pequeno/médio** — essencialmente refactor. O código já existe
em `CounterState`; muda-se a superfície. Estimativa: ~150 linhas
novas + ~30 sítios de consumidor a actualizar. Equivalente a Passo
107 em esforço.

### Dependências necessárias

- Nenhuma. `CounterState` já existe.

### O que NÃO resolve

- `query(heading)` função stdlib (precisa de Candidato 3).
- `counter.at(here())` (precisa de Candidato 4).
- Multi-pass.

### Avaliação

**Preparação boa, valor pequeno.** Esforço razoável, sem entregar
funcionalidade visível. Candidato "ponte" — faz sentido se o
próximo passo vai ser um dos candidatos 3/4/5.

---

## Candidato 3 — Função `query(selector)` sobre o `Introspector`

### O que

Adicionar à stdlib uma função `query` que retorna os elementos
matching via `Introspector`:

```typst
#query(heading)        // → array de Content
#query(heading.where(level: 1))  // se Selector suportar filtros
```

Implementação:

1. Construir `Introspector` via `Introspector::from_content` após
   a fase de introspecção.
2. Função stdlib `query` recebe um `Selector`; consulta
   `introspector.query(selector) -> Vec<Content>`.
3. Resultado convertido em `Value::Array(Vec<Value::Content>)`.

### Tipos a materializar

- Candidato 2 (Introspector) como dependência.
- `Selector` mais rico (já tem `NodeKind` base).
- Função nativa `query` em `stdlib`.

### DEBTs que desbloqueia

**Nenhum aberto hoje** fala de `query` explicitamente.

### Tamanho estimado

**Médio** — Candidato 2 (~150 linhas) + `Selector` genérico (~100
linhas) + stdlib function (~50 linhas) + testes + migrar um
consumidor = ~300-400 linhas novas, 10-15 testes novos. Tamanho
entre Passo 104 e Passo 100.

### Dependências necessárias

- Candidato 2 (Introspector wrapper) — implícito.
- Ponto de call: a função `query` precisa de receber o
  `Introspector` de algures. Hoje o eval não tem acesso a ele
  (Introspection corre depois do eval, em L3).

**→ Encontramo-nos com o problema arquitectural do 108.C**: para
`query` funcionar em eval, o Introspector tem de ser construído
**antes** do eval terminar. No vanilla, isto é resolvido por
multi-pass. No cristalino single-pass, o eval não vê o documento
todo ainda.

**Mitigação**: `query` só funciona **após** um pass preliminar.
Requer re-invocar eval a seguir ao primeiro introspect. **É o
início do multi-pass**, limitadíssimo.

### O que NÃO resolve

- `counter.at(here())` (Candidato 4).
- `@label` em eval (Objectivo 3 do 108.C — descartado).

### Avaliação

**Funcionalidade real, mas custo arquitectural alto**. Introduz a
primeira forma de multi-pass. Risco de propagação: uma vez aceita
segunda passada, surge pressão para `counter.at()`,
`state.final_()`, etc.

---

## Candidato 4 — `Location` + `here()` + `context` + `counter.at()`

### O que

Stack mínimo para a expressão vanilla
`#counter(heading).at(here())`:

1. `Location` (Candidato 1) como chave opaca.
2. `here()` função stdlib que retorna `Value::Location(loc)`.
3. `context` → no vanilla, `context` é um Contextual functor. No
   cristalino, seria preciso primeiro **produzir** `Location`s
   durante eval/layout para que `here()` tenha algo que retornar.
4. `Counter::at(loc)` → requer `Introspector.count_before(selector,
   loc)`.

### Tipos a materializar

- `Location` (Candidato 1).
- Atribuição de `Location` a variantes de Content relevantes
  (Heading, Figure, CounterUpdate, …). Forma provável: campo
  `loc: Option<Location>` nas variantes introspectáveis.
- Introspector estendido com `count_before(selector, loc)`.
- `here()` + `Value::Location` tipo novo.

### DEBTs que desbloqueia

- **DEBT-1 residual** (parcialmente) — a pendência
  "propriedades adicionais bloqueadas por tipos não materializados"
  menciona `counter.at(here())`.

### Tamanho estimado

**Grande** — toca Content (campo novo em N variantes), eval
(produzir Location), introspect (preservar Location), stdlib
(here/query), novas funções, novo Value variant. Compareável a
Passo 100 (StyleChain).

### Dependências necessárias

- **Multi-pass eval** (implícito): `counter.at(here())` só pode
  ser avaliado quando o Introspector tiver visto todo o documento.
- Decisão arquitectural sobre onde vive Location (em Content ou
  paralelo).

### O que NÃO resolve

- Convergência completa (fixpoint de hash).
- `query` estruturado com filtros complexos.

### Avaliação

**Alto valor** (expressão user-facing real), **alto custo**
(arquitectural e de código). Candidato "grande". Viola o
"pequeno" mas paga o "destrava DEBT-1".

---

## Candidato 5 — `Engine<'a>` com Introspection stub interno

### O que

Materializar `Engine<'a>` como struct coesa que **absorve** os 10+
parâmetros actuais de `eval_*` (route, styles, show_rules,
active_guards, current_file, figure_numbering, sink, world, ...)
**e** um campo `introspector: StubIntrospector` que serve como
ponto de extensão futuro.

O `StubIntrospector` devolve `CounterState` actual. Nada muda para
o utilizador. O **valor** é:

1. `eval_*` passa de 10+ params para 1 param (`engine: &mut Engine`).
2. Próxima extensão de introspection é adicionar método a
   `Engine`/`Introspector` — sem propagar 11º parâmetro.

### Tipos a materializar

- `Engine<'a>` — struct com todos os campos actuais dos `eval_*`.
- `StubIntrospector` — hoje é `CounterState`; wrapper trivial.

### DEBTs que desbloqueia

- **Nenhum aberto**. Mas **remove a pressão** identificada no
  Passo 107 ("10 params + ctx é visualmente pesado").

### Tamanho estimado

**Médio/grande** — refactor de todos os `eval_*` (24 funções da
5ª aplicação ADR-0036 do Passo 107). Pouco código novo, muito
refactor.

### Dependências necessárias

- Nenhuma nova de Introspection. Apenas reorganização.

### O que NÃO resolve

- `query`, `counter.at()`, multi-pass — continua adiado.

### Avaliação

**Não resolve Introspection**, mas é **precondição para
resolver bem**. Passo 107 já documentou a pressão de 10 params.
Se o próximo passo for Candidato 2, 3 ou 4, fazer Engine primeiro
simplifica cada um deles.

**Custo**: médio. **Valor**: indirecto (facilita futuros).

---

## Resumo ranqueado

Ranking pelos critérios (pequeno ≤ 104 em tamanho; desbloqueia
DEBT aberto; viável com o que existe):

| # | Candidato | Tamanho | Desbloqueia DEBT? | Viável hoje? | Valor visível ao utilizador |
|---|-----------|---------|:-----------------:|:------------:|-----------------------------|
| 1 | Location opaca | XS | Não | Sim | Nenhum |
| 2 | Introspector wrapping CounterState | S-M | Não | Sim | Nenhum (refactor) |
| 3 | `query(heading)` stdlib | M-L | Não | **Exige multi-pass** | Pequeno/médio |
| 4 | Location + here + counter.at | L | DEBT-1 (parcial) | **Exige multi-pass + decisão arq.** | Alto |
| 5 | Engine<'a> stub | M-L | Não (remove pressão) | Sim | Nenhum (refactor) |

**Nenhum** candidato satisfaz as três condições simultaneamente
(pequeno + desbloqueia DEBT + viável).

### Combinações viáveis como passo único

Nenhum candidato isolado tem os três.

- Cand.1 sozinho: **falha "desbloqueia DEBT"** (não desbloqueia
  nada).
- Cand.2 sozinho: **falha "desbloqueia DEBT"** (apenas refactor).
- Cand.3 sozinho: **falha "viável hoje"** (exige multi-pass).
- Cand.4 sozinho: **falha "pequeno"** + exige decisão arq.
- Cand.5 sozinho: **falha "desbloqueia DEBT"** (remove pressão,
  não destranca DEBT).

---

## Recomendação (para 108.E)

**Recomendação primária: Candidato 5 (Engine<'a> com stub
interno)**.

Razões:

1. **Precondição para tudo o resto**: Candidatos 2, 3, 4 todos
   sofrem com a explosão de parâmetros (10 → 11 → 12 à medida que
   Introspector é adicionado). Fazer Engine primeiro absorve e
   facilita.
2. **Remove pressão documentada**: Passo 107 registou
   explicitamente os 10 params como evidência empírica.
   Engine paga essa pressão.
3. **Sem decisão arquitectural forte**: não exige decidir onde
   vive `Location`, nem se multi-pass é aceite, nem se vão haver
   novas variantes de Content. Refactor mecânico, mesurado.
4. **Medida do custo é conhecida**: 24 funções, 7 ficheiros, ~10
   params absorvidos numa struct. Não há "mystery box".
5. **Falha o critério "desbloqueia DEBT aberto"**, mas a spec 108
   reconhece esta alternativa:

   > "Se nenhum candidato fica em tamanho ≤ Passo 104, isto é
   > informação valiosa: significa que Introspection não se
   > materializa por partes triviais. A recomendação em 108.E
   > passa a ser 'fazer Engine<'a> primeiro com Introspection
   > stub interno' ou 'dividir em sub-candidatos ainda menores'
   > — registar a conclusão em vez de forçar."

**Recomendação secundária, para passo seguinte a Engine**:
Candidato 2 (Introspector wrapping CounterState). Com Engine no
lugar, adicionar o campo `introspector: Introspector` é trivial; a
API interna melhora; prepara Candidatos 3 e 4 com superfície clara.

**Candidato 4 (Location + counter.at)** fica para quando a
arquitectura estiver pronta (Engine + Introspector wrapping).

**Candidato 3 (query)** depende de decidir se aceita multi-pass
ou não. Deixar para depois.

---

## Se a decisão for "não fazer Engine agora"

Alternativas aceitáveis:

- **Cand.2 isolado**: pequeno refactor, sem Engine. Funcional.
  Prepara 3/4. Aceita "não desbloqueia DEBT aberto" como
  trade-off explícito.
- **Adiar Introspection**: trabalhar noutros DEBTs abertos
  (DEBT-45, DEBT-8, …) enquanto a decisão arquitectural cozinha.
  Registar em 108.E como opção.
