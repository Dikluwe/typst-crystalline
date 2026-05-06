# Passo 203A — Diagnóstico + auditoria empírica do estado actual de Position

**Série**: 203 (sub-passo `A` = diagnóstico-primeiro
formal). 36ª aplicação consecutiva do padrão
diagnóstico-primeiro.
**Tipo**: diagnóstico-primeiro (zero código tocado) +
auditoria empírica do estado actual.
**Magnitude planeada**: S–M.
**Pré-condição**: P202 concluído; snapshot 2026-05-05
reconciliado (`00_nucleo/snapshot-2026-05-05.md`); lacunas
#1 (Position) e #1b (Position-related) confirmadas como
únicas residuais; Introspector trait com método
`position_of` retornando `Option<()>` (stub estrutural).
**Output**: 3 ficheiros (diagnóstico + auditoria + relatório).

---

## §1 Propósito

Fixar as decisões estruturais de P203 (sub-passos `*B+`)
antes de qualquer alteração de código. P203 endereça
lacunas #1 e #1b (Position concreto) que o snapshot
classifica como **provavelmente bloqueante para M8**
("queries location-aware completas").

P203A produz:

1. Auditoria empírica do estado actual de Position no
   cristalino (cláusulas de auditoria A1–An).
2. Cláusulas de decisão (C1–Cn) sem condicionais.
3. Decisão sobre se vanilla typst Position é referência.
4. Plano de sub-passos `*B+` sem ramos — fixado **após**
   a auditoria, baseado no que ela revelar.

P203A não toca em código. Não escreve `.rs`. Não modifica
`Cargo.toml`. Não promove ADRs.

---

## §2 Material de partida verificado em P202

Antes de fixar cláusulas, P203A confirma:

- `Introspector::position_of` existe como método e retorna
  `Option<()>` (stub estrutural; snapshot §5).
- Comentário interno em `TagIntrospector` declara
  `// positions: HashMap<Location, Position> — adiado para M5/M9`
  (snapshot §5).
- Lacuna #1 último passo: P165 (TagIntrospector criada com
  `position_of` stub).
- Lacuna #1b: depende de #1 (não materializada
  isoladamente).
- Tipo `Position` pode ou não existir em L1 — P203A
  determina empíricamente.

Sem isto, a auditoria não tem fundamento. Recuar para P202
se algum item não estiver confirmado.

---

## §3 Cláusulas de auditoria (A1–An)

Esta secção é executada **primeiro**. Output empírico
alimenta C1+ adiante. Cada item reporta CONFIRMADO /
DIVERGÊNCIA / NÃO APLICÁVEL com evidência.

### A1 — Existência do tipo `Position`

```
grep -rn "^pub struct Position\b\|^pub enum Position\b" \
  01_core/src/entities/ 01_core/src/contracts/
grep -rn "Position " 01_core/src/entities/introspector.rs
```

Critério: confirmar se `Position` existe como tipo
nominal em L1, ou se é apenas referido em comentários e
assinaturas de stub.

### A2 — Forma vanilla de `Position`

```
grep -rn "^pub struct Position\b\|^pub enum Position\b" \
  lab/typst-original/crates/
```

Output esperado: localização e definição de `Position` no
vanilla. Campos do struct (provavelmente `page: usize`
+ `point: Point` ou similar).

A2 é **leitura informativa** — saber o que está disponível
como referência potencial. Não decide adopção.

### A3 — Locais que consomem `position_of`

```
grep -rn "\.position_of(" 01_core/ 02_shell/ 03_infra/ 04_wiring/
grep -rn "position_of" 01_core/src/entities/introspector.rs
```

Critério: lista de call sites. Para cada call site:
- Quem invoca.
- Que faz com o `Option<()>` retornado.
- Se o tipo `()` é actualmente suficiente ou se há código
  que tenta extrair informação que não existe.

### A4 — Stores existentes em `TagIntrospector`

```
grep -B 2 -A 30 "^pub struct TagIntrospector" \
  01_core/src/entities/introspector.rs
```

Critério: confirmar 9 sub-stores actuais (per snapshot §5).
Identificar onde um sub-store `positions` se encaixaria
(antes ou depois de `kind_index`?).

### A5 — Walk fn — onde Position seria emitido

```
grep -B 2 -A 20 "^pub(crate) fn walk" \
  01_core/src/rules/introspect.rs
```

Critério: identificar pontos do walk onde Position seria
calculado. Hipóteses:
- Walk fn sabe `current_location` mas não `current_page`.
- Walk fn não tem acesso a layout state.
- Position pode requerer feedback do Layouter (ciclo
  layout↔introspect).

A5 é o ponto crítico. Se Position depender de feedback do
Layouter, P203 cruza com M8 (comemo); se for pure walk-time,
é ortogonal.

### A6 — Layouter — onde Position é determinada

```
grep -n "page\|current_page\|page_number" \
  01_core/src/rules/layout/mod.rs | head -30
grep -n "Location" 01_core/src/rules/layout/mod.rs | head -20
```

Critério: identificar onde o Layouter conhece página
correntemente em layout. Confirmar se há mecanismo
existente para reportar essa informação ao `TagIntrospector`.

### A7 — Vanilla typst — `position_of` pipeline

```
grep -rn "fn position\b\|positions:" \
  lab/typst-original/crates/typst-library/src/introspection/
grep -rn "\.position(" \
  lab/typst-original/crates/typst-library/src/introspection/ | head -10
```

Critério: como vanilla calcula Position. Hipóteses
prováveis:
- Vanilla calcula Position no Layouter e emite tag/marker.
- Vanilla mantém store de Position acumulado durante layout.
- Vanilla usa fixpoint para resolver Position.

A7 é também leitura informativa — alimenta C5.

### A8 — Lacuna #1b — Position-related

Identificar empíricamente a que se refere "#1b
Position-related":

```
grep -rn "#1b\|lacuna 1b\|Position-related" 00_nucleo/
```

Critério: localizar a definição original da lacuna #1b.
Sem isto, P203 não pode endereçá-la — está mal definida.

### A9 — Tests que tocam Position actualmente

```
grep -rn "position_of\|Position " 01_core/src/ 03_infra/src/ \
  | grep -E "test|assert" | head -20
```

Critério: tests existentes que invocam o stub. Quantos.
Que asserções fazem (provavelmente que retorna `None` ou
`Some(())`).

### A10 — Corpus de paridade — casos que precisam de Position

```
ls lab/parity/ 2>/dev/null
grep -rn "position\|location" lab/parity/ 2>/dev/null | head -10
```

Critério: identificar se há casos no corpus de paridade que
falhariam por falta de Position concreta. Se sim, são
input importante para P203.

---

## §4 Cláusulas de decisão (C1–Cn)

Estas cláusulas são fixadas **depois** da auditoria, com
base no output empírico de A1–A10. Cada uma é fixada sem
condicionais.

### C1 — Definição de Position (forma concreta)

Forma do tipo `Position` em L1. Decisão entre:

- **Réplica vanilla** — mesmo struct (page + point).
- **Forma cristalina mínima** — apenas `page: usize`
  inicialmente; `point` adiado para sub-lacuna.
- **Forma cristalina alternativa** — não documentada nesta
  spec; emerge da auditoria se houver razão.

A decisão é fixada com base em A2 (forma vanilla) + A3
(consumers actuais) + A10 (corpus de paridade).

### C2 — Localização do tipo

`Position` vai para `01_core/src/entities/position.rs`,
seguindo padrão dos outros tipos de domínio.

Esta cláusula é estrutural — não depende da forma
escolhida em C1.

### C3 — Sub-store `positions` em `TagIntrospector`

Adicionar `positions: HashMap<Location, Position>` como
10º sub-store, removendo o comentário "adiado para M5/M9".

A posição na ordem dos fields é fixada com base em A4
(provavelmente após `kind_index` ou junto a `state`).

### C4 — Mecanismo de cálculo de Position

Decisão entre três mecanismos, fixada com base em A5–A7:

- **Walk-time puro** — Position calculado durante walk
  com base em `current_location` + heurística de página.
  Viável apenas se A5 mostrar que walk tem informação
  suficiente.
- **Layouter feedback (single-pass)** — Layouter emite
  Position para `TagIntrospector` durante layout; sem
  fixpoint adicional.
- **Layouter feedback (fixpoint)** — Position depende de
  fixpoint análogo ao TOC fixpoint existente.

C4 é a decisão central de P203. Determina magnitude
agregada e relação com M8.

### C5 — Relação com vanilla typst

Decisão entre:

- **Adoptar pipeline vanilla** — replicar mecanismo
  observado em A7.
- **Pipeline cristalino próprio** — divergir e justificar.

C5 fixa-se com base em A7 + C4. Documentação da decisão
fica no relatório do passo.

### C6 — Lacuna #1b — definição operacional

Com base em A8, fixar o que "#1b Position-related" cobre
operacionalmente. Hipóteses:

- Refere queries que dependem de Position (ex:
  `query_at_position`).
- Refere consumers actuais que ignoram Position por estar
  ausente.
- Refere conversão entre Location e Position.

C6 não pode ser fixada sem A8.

### C7 — Magnitude agregada de P203

Magnitude estimada após C1–C6 fixadas. Range plausível:
**M** (se C4 escolher walk-time puro) a **L cross-modular**
(se C4 escolher Layouter feedback com fixpoint).

C7 é output de P203A — não pré-fixada.

### C8 — Sub-passos `*B+`

Plano de sub-passos `*B+` sem condicionais. Quantidade e
forma dependem de C1–C7.

**Não pré-defino sub-passos B–G nesta spec.** O erro de
P193A é não ser repetido. Plano `*B+` emerge de P203A após
A1–A10 e C1–C7.

### C9 — Compatibilidade com M8

P203 fechado é pré-condição para M8 ou ortogonal a M8.
Decisão fixada com base em C4:

- C4 = walk-time puro → P203 ortogonal a M8.
- C4 = Layouter feedback single-pass → P203 leve, antes
  de M8.
- C4 = Layouter feedback fixpoint → P203 antes de M8;
  fixpoint Position interage com fixpoint comemo.

### C10 — ADR dedicada

P203 produz ADR dedicada se C4 escolher mecanismo não
trivial (Layouter feedback ou fixpoint). Caso contrário,
documentação fica no relatório consolidado de série P203.

Decisão sobre ADR fica para output de P203A. Se afirmativa,
ADR proposta ganha número (provavelmente 0073 ou
posterior).

### C11 — Sem cláusulas condicionais

Cláusulas C1–C10 são fixadas com valores concretos
(uma forma, um mecanismo, um plano), não com ramos. Casos
que dependem de auditoria são marcados como "output de
P203A", não como `if`.

---

## §5 Outputs concretos de P203A

Três ficheiros:

### Ficheiro 1 — Auditoria empírica

Localização:
`00_nucleo/diagnosticos/typst-passo-203A-auditoria-position.md`.

Conteúdo: A1–A10 com etiquetas e evidência (output de
comandos, citação de ficheiros). Sem decisões — apenas
factos.

### Ficheiro 2 — Diagnóstico

Localização:
`00_nucleo/diagnosticos/typst-passo-203A-diagnostico.md`.

Conteúdo: cláusulas C1–C11 instanciadas com valores
concretos. Cita auditoria como evidência. Inclui plano
`*B+` final (output de C8).

### Ficheiro 3 — Relatório

Localização:
`00_nucleo/materialization/typst-passo-203A-relatorio.md`.

Conteúdo: o que foi feito, tempo de execução, decisões
tomadas durante a leitura, magnitude calibrada,
recomendação de número ADR (se C10 afirmativa), sugestão
não-vinculativa para próximo sub-passo (provavelmente
`P203B` mas determinado por C8).

---

## §6 Critério de progressão para `*B`

P203A só transita para `*B` quando:

- A1–A10 todos com etiqueta CONFIRMADO ou DIVERGÊNCIA
  registada.
- C1–C11 instanciadas com valores concretos.
- Magnitude calibrada e dentro do range S–L cross-modular.
- Plano `*B+` sem condicionais.
- ADR dedicada decidida (criar ou não criar).

Em caso de divergência empírica relevante face ao snapshot
2026-05-05 (ex: `position_of` não retornar `Option<()>`,
sub-stores não serem 9), registar em `P203A.div-N` e
decidir entre:
- Ignorar (divergência ortogonal).
- Corrigir snapshot (P204 administrativo).
- Ramificar (divergência bloqueia P203).

---

## §7 Convenções mantidas

- Sem código Rust nas specs.
- Sem `if`, sem ramos, sem condicionais nos sub-passos.
- 3 outputs padrão.
- Distinção fecho estrutural vs arquitectural mantida.
- Honestidade sobre dead code, gate dormente, magnitude
  real.
- Preservação histórica de relatórios anteriores.
- Sem inflação: sem "patamar", sem "limiar", sem
  "consolidação", sem "deriva", sem "subpadrão", sem
  "cumulativo", sem "cross-domínio", sem "paridade
  observable" como bandeira retórica.

---

## §8 Não-objectivos

P203A não:

- Toca em código.
- Cria o tipo `Position`.
- Adiciona sub-store `positions`.
- Modifica `position_of`.
- Promove ADRs.
- Pré-define sub-passos `*B+` (esses emergem de C8 com base
  em A1–A10 + C1–C7).
- Decide M8.

---

## §9 Erro a não repetir

P193A (versão anterior, antes da correcção em P201/P202)
definia auditoria + diagnóstico + plano `*B–G` num só
pacote, com cláusulas pré-fixadas para sub-passos que
ainda não tinham fundamento empírico.

P203A separa **auditoria** (factos empíricos, A1–A10) de
**diagnóstico** (decisões fixadas, C1–C11). O plano `*B+`
emerge da combinação dos dois — não é pré-definido na spec.

Próximo sub-passo concreto (`P203B`+) é fixado pelo
relatório de P203A, não por esta spec.

---

## §10 Particularidade — execução

Pode ser executado pela sessão actual (Opus, conversacional)
**ou** pelo Claude Code, à escolha do humano. P203A é
diagnóstico de feature, não passo administrativo
exaustivo — volume de leitura é menor que P201/P202
(focado em Position; não corpus completo).

Se executado pela sessão actual, validação empírica
passa pelo bash_tool. Se executado pelo Claude Code, segue
o padrão de delegação dos passos administrativos
anteriores.
