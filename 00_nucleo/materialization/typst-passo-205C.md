# Passo 205C — `position_of` impl real + consumer migration

**Série**: 205 (sub-passo `C` = consumer integration após
P205B sealing infrastructure).
**Tipo**: implementação (integração + migração).
**Magnitude planeada**: S–M.
**Pré-condição**: P205B concluído; `SealedPositions` em
`01_core/src/entities/sealed_positions.rs`;
`#[comemo::track] impl SealedPositions { fn position_of }`
aplicado; `PagedDocument.extracted_positions` populated
em `Layouter::finish`; tests 1856 verdes; 0 violations;
19 sentinelas activas.
**Output**: 3 ficheiros (inventário + relatório +
alterações de código).

---

## §1 Propósito

Materializar a impl real de `Introspector::position_of`
(que actualmente retorna `None` em `TagIntrospector` per
P204D §C6a) e migrar consumers que precisem de Position
para usar a impl unificada.

A forma da impl é decidida no inventário inicial com
base em consumers reais e arquitectura existente. A
clarificação inicial fixou esta abordagem
explicitamente.

P205C respeita o padrão: começa com inventário empírico
antes de qualquer alteração.

---

## §2 Material de partida verificado em P205B

Antes de qualquer alteração, confirmar empíricamente:

- `SealedPositions` em
  `01_core/src/entities/sealed_positions.rs` com
  `#[comemo::track] impl` aplicado.
- `PagedDocument.extracted_positions: SealedPositions`
  campo público.
- `Layouter::finish` produz `extracted_positions`
  populated via `from_runtime`.
- Trait `Introspector::position_of` em
  `01_core/src/entities/introspector.rs` retorna
  `Option<Position>` (P204D migrou signature).
- `TagIntrospector::position_of` retorna sempre `None`
  (P204D documentou em §C6a do diagnóstico).
- `runtime.positions` continua populated single-pass via
  `advance_locator_if_locatable` (P204D).

Sem isto, recuar para P205B.

---

## §3 Cláusulas de execução (sem condicionais)

### C1 — Inventário empírico inicial

Antes de tocar em código, listar literalmente:

1. **Consumers reais de `position_of`** — confirmar:
   - Tests existentes em `01_core/`, `03_infra/`,
     `04_wiring/`, `lab/parity/`.
   - Stdlib expansions que invocam `here()`/`locate()`
     (esperado: ainda não materializadas, per P204F SKIP
     `here-locate.typ`).
   - Outros call sites empíricos.
2. **Arquitectura `Introspector` actual** — confirmar:
   - Trait declaration (1 trait, `#[comemo::track]` per
     P204B).
   - Impls existentes (`TagIntrospector` é a única
     produção; possíveis tests fixtures).
   - Como Layouter consume Introspector:
     `Tracked<'a, dyn Introspector + 'a>` per P204C.
3. **Pipeline pré vs pós-layout** — confirmar:
   - Pré-layout: `TagIntrospector` é construído via
     `from_tags`; sem acesso a `SealedPositions`.
   - Pós-layout: `PagedDocument` tem
     `extracted_positions: SealedPositions`.
   - Quem precisa de `position_of` em qual fase?
4. **Arquitectura `PagedTagIntrospector` candidata** —
   confirmar:
   - Existe já tipo similar?
   - Cristalino tem precedente para wrappers que
     combinam introspector + dados pós-layout?
   - Vanilla `PagedIntrospector` é referência empírica
     (per P203A A7 + P205A A9).
5. **Arquitectura "TagIntrospector enriquecido"
   candidata** — confirmar:
   - `TagIntrospector` pode ganhar field
     `Option<SealedPositions>` sem invadir invariantes
     de construção?
   - Construção pré-layout via `from_tags` continua a
     funcionar com field default `None`?
   - Sealing pós-layout pode injectar via `&mut self` ou
     método dedicado?

Output: 5 sub-secções com etiqueta CONFIRMADO ou
**AJUSTE NECESSÁRIO**.

Se C1.1 mostrar zero consumers reais (apenas tests
stub asserting `None`), C2 decide se faz sentido
materializar agora ou adiar para sub-passo dedicado
quando consumer real existir.

### C2 — Decisão sobre forma da impl real

Com base em C1.4 + C1.5 + C1.1, fixar:

- **Caminho A — `TagIntrospector` enriquecido** —
  campo `positions: Option<SealedPositions>` (default
  `None` na construção pré-layout); injectado pós-layout
  via método dedicado. `position_of` consulta o
  `Option`.
- **Caminho B — `PagedTagIntrospector` wrapper novo** —
  tipo dedicado que wrappa `TagIntrospector` +
  `SealedPositions`. Impl `Introspector` delega a maioria
  dos métodos ao inner; `position_of` usa o `SealedPositions`.
- **Caminho C — Adiar** — se C1.1 mostrar zero consumers
  reais, P205C fecha sem materializar. Documenta que a
  infraestrutura está pronta mas a impl real fica para
  sub-passo dedicado quando consumer real existir.

Critério para escolha:
- Se zero consumers: Caminho C honesto (não inflar).
- Se consumers existem em pipeline pré-layout: Caminho B
  natural (wrapper específico para fase pós-layout).
- Se consumers existem mas só pós-layout e arquitectura
  permite injecção: Caminho A simples.

C2 fixa **uma** alternativa.

### C3 — Implementação literal (se C2 = A ou B)

Se C2 = Caminho A:

```text
pub struct TagIntrospector {
    // 9 sub-stores existentes,
+   // P205C: populated post-layout via inject_positions().
+   pub positions: Option<SealedPositions>,
}

impl TagIntrospector {
+   pub fn inject_positions(&mut self, sealed: SealedPositions);
}

impl Introspector for TagIntrospector {
    fn position_of(&self, location: Location) -> Option<Position> {
-       None  // P204D §C6a stub
+       self.positions.as_ref()?.position_of(location)
    }
}
```

Se C2 = Caminho B:

```text
pub struct PagedTagIntrospector {
    inner: TagIntrospector,
    positions: SealedPositions,
}

impl Introspector for PagedTagIntrospector {
    fn position_of(&self, location: Location) -> Option<Position> {
        self.positions.position_of(location)
    }
    // 19 outros métodos delegam: self.inner.<método>(...)
}
```

Se C2 = Caminho C: skip C3.

C3 fixa forma concreta sem ramos.

### C4 — Sealing point + injecção (Caminho A) ou
construção (Caminho B)

Se C2 = Caminho A:
- Em `pub fn layout` ou `Layouter::finish`, após sealing
  produzir `extracted_positions`, invocar
  `intr.inject_positions(doc.extracted_positions.clone())`
  ou similar.
- Identificar local literal onde `intr` está disponível.

Se C2 = Caminho B:
- Construir `PagedTagIntrospector::new(tag_intr,
  sealed)` no fim do pipeline ou ponto de query.
- Identificar consumer que beneficia.

Se C2 = Caminho C: skip.

### C5 — Migração de consumers

Lista os consumers identificados em C1.1 e migra cada um:

- Tests stub asserting `None`: actualizar para asserting
  `Some(Position { ... })` quando location é locatable;
  manter `None` apenas para cases onde location não
  existe.
- Tests E2E novos (2–3) que exercem o caminho real.
- Stdlib `here()`/`locate()` consumers: out of scope se
  ainda não materializados (per P204F SKIP).

C5 não altera consumers que não invocam `position_of`.

### C6 — Tests dedicados

Adicionar 2–4 tests E2E:

- **Test 1** — `position_of` retorna `Some(Position)`
  para locatable em página específica.
- **Test 2** — `position_of` retorna `None` para
  location desconhecida.
- **Test 3** — pipeline completo: layout → seal →
  query → assert Position correcta.
- **Test 4 (opcional)** — múltiplas iterações fixpoint:
  `position_of` retorna Position consistente entre
  iterações.

Critério: tests verdes; 1856 → 1858+.

### C7 — Compilação

```
cargo build --workspace 2>&1 | tail -10
```

Critério: verde. Hipóteses prováveis de erro:

- (Caminho A) `inject_positions` precisa `&mut self` —
  caller adapta.
- (Caminho A) `Option<SealedPositions>` tem que satisfazer
  `Hash`/`Send`/`Sync` se trait `Introspector` tracked
  expõe via `&self`. Verificação no compile.
- (Caminho B) `PagedTagIntrospector` precisa derivar
  bounds para `#[comemo::track]` do trait — Send + Sync,
  Clone.

### C8 — Tests workspace

```
cargo test --workspace 2>&1 | tail -10
```

Critério: 1856+ tests verdes.

Caso testes existentes que assert `position_of(...) ==
None` falhem após migração, identificar:

- Tests que **deveriam continuar a retornar None**
  (location não-locatable): manter assert.
- Tests que **agora retornam Some** (location locatable
  + populated): actualizar assert.

### C9 — Linter

```
crystalline-lint .
```

Critério: 0 violations. `--fix-hashes` se necessário
para sincronizar L0 prompts (lição P204D/E/G).

### C10 — Documentação ADR-0074

ADR-0074 mantém PROPOSTO. Anotação cirúrgica em §P205C
com `✅ MATERIALIZADO` ou `✅ DEFERIDO` (se C2 = C) +
sumário (1–2 linhas).

Se C2 = C, registar honestamente que P205C fechou sem
materializar a impl real porque zero consumers reais.
Infra está pronta; impl real fica para quando consumer
existir.

### C11 — Sentinelas

Adicionar 1–2 sentinelas:

- (Caminho A) `p205c_tag_introspector_position_of_real`
  — falha se field for removido.
- (Caminho B) `p205c_paged_tag_introspector_existe` —
  falha se tipo for removido.
- (Caminho C) sem sentinelas novas; preserva 19
  existentes.

### C12 — Critério de fecho de P205C

P205C concluído quando:

- C1 inventário completo (5 sub-secções).
- C2 caminho fixado com justificação.
- C3 implementação aplicada (se C2 = A ou B).
- C4 sealing/construção aplicada (se C2 = A ou B).
- C5 consumers migrados.
- C6 tests dedicados (2–4).
- C7 compilação verde.
- C8 tests workspace verdes.
- C9 linter 0 violations.
- C10 ADR-0074 anotada.
- C11 sentinelas (mínimo 1 se C2 ≠ C).
- Inventário registado.
- Relatório escrito.

### C13 — Sem cláusulas condicionais

C1 produz dados. C2 fixa **uma** alternativa de impl.
C3–C11 executam decisões fixas.

A possibilidade de C2 = Caminho C (adiar) **não é ramo
condicional** — é resposta empírica honesta a "zero
consumers reais detectados". Se C1.1 mostrar consumers
existentes, C2 escolhe entre A e B. Se mostrar zero,
C2 = C registado honestamente. Decisão é empírica, não
ramificação na spec.

---

## §4 Outputs concretos

### Ficheiro 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-205C-inventario.md`.

Conteúdo:
- §1 C1 — inventário (5 sub-secções).
- §2 C2 — caminho fixado com justificação.
- §3 C3 — implementação literal (se A ou B).
- §4 C4–C5 — sealing point + migração consumers.
- §5 Decisões durante a leitura.

### Ficheiro 2 — Relatório

Localização:
`00_nucleo/materialization/typst-passo-205C-relatorio.md`.

Conteúdo:
- O que foi feito.
- Caminho escolhido (A / B / C).
- Tempo de execução.
- Métricas (tests pre/post; LOC delta).
- Decisões.
- Sugestão para próximo sub-passo (P205D).

### Ficheiro 3 — Alterações em código

Não é ficheiro discreto. Conjunto de:

- (Caminho A) `TagIntrospector` ganha field
  `positions`; impl `position_of` usa-o; sealing point
  invoca `inject_positions`. 1–2 ficheiros tocados.
- (Caminho B) `PagedTagIntrospector` ficheiro novo;
  consumers que precisam de Position usam-no. 1
  ficheiro novo + 1–2 tocados.
- (Caminho C) sem alterações de código além de
  documentação ADR.
- Tests dedicados.
- Possível L0 prompt actualização (Caminho B exigiria
  L0 novo).
- Anotação cirúrgica em ADR-0074.

---

## §5 Critério de progressão para P205D

P205C fechado quando C12 cumprido.

Em caso de divergência empírica relevante, registar em
`P205C.div-N` e:

- Resolver dentro de P205C (preferido).
- Recuar para P205A re-fixar C6 do diagnóstico (se
  obstrução for estrutural — improvável).

P205D só começa quando P205C fechado.

---

## §6 Convenções mantidas

- Sem condicionais estruturais.
- 3 outputs.
- Inventário empírico antes de implementação.
- Localização canónica:
  `00_nucleo/diagnosticos/` para inventário;
  `00_nucleo/materialization/` para relatório.
- Distinção fecho estrutural vs final mantida.
- Sem inflação retórica.

---

## §7 Não-objectivos

P205C não:

- Implementa `label_pages` trackable (P205D).
- Implementa stdlib `here()`/`locate()` — esses são
  consumers que invocam `position_of`, não são
  endereçados aqui.
- Transita ADR-0074 para ACEITE (P205E).
- Cria ADR nova além de ADR-0074 já PROPOSTO.
- Toca em loop fixpoint.
- Modifica `runtime.positions` populated single-pass.
- Modifica `SealedPositions` (esse foi P205B).
- Cria `PagedTagIntrospector` se C2 ≠ B.
- Endereça vanilla integration (DEBT-53/54).

---

## §8 Erro a não repetir

Da série P203 + P204 + P205A/B — pattern empírico:
inventário antes de decisão. P205C aplica-o em C1 com
foco específico em "consumers reais" — não assumir que
existem.

Risco específico: **inflar P205C materializando impl
real sem consumer real**. Se C1.1 mostrar zero
consumers, Caminho C (adiar) é honesto. Inflar para
demonstrar capacidade sem necessidade real é o erro
oposto à honestidade que a série P204 manteve.

Outro risco: **wrapper `PagedTagIntrospector` por
simetria com vanilla `PagedIntrospector`** sem
verificar empíricamente que cristalino precisa dessa
separação. Vanilla diverge intencionalmente
(post-layout fase 3); cristalino single-pass pode não
beneficiar do wrapper.

P205A documentou divergência intencional cristalino vs
vanilla (`P205A.div-1` — vanilla não tem Layouter
monolítico). P205C estende esse princípio: impl de
`position_of` segue arquitectura cristalina, não
arquitectura vanilla.

Hipótese mais provável: C1.1 mostra zero consumers
reais em produção (per P204A A3 — `position_of` stub
não tinha consumers; per P204F SKIP de `here-locate`).
Caminho C registado honestamente é resultado provável.

Mas isto é **hipótese**, não decisão da spec. C2
fixa-se com base em C1.1.

---

## §9 Particularidade — execução

P205C é trabalho de código focado:

- Investigação de consumers (C1.1) é grep simples.
- Decisão arquitectural (C2) é trivial se C1.1 for
  empírica.
- Implementação varia de S (C2 = C, sem código) a M
  (C2 = B, wrapper completo).
- Tests dedicados (~30–60 LOC).

Volume baixo a médio. Magnitude S–M.

Recomendado Claude Code dado:

- Volume de leitura para C1.1 + C1.2 (consumers +
  arquitectura).
- Decisão empírica que pode resultar em Caminho C
  (adiar) — honestidade exige não inflar.

Sessão actual viável se C1 não revelar surpresa.
