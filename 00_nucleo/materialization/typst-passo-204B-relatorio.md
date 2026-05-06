# Relatório do passo P204B

**Data de execução**: 2026-05-06.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-204B.md`.
**Natureza**: implementação foundational pós-diagnóstico
P204A. **Sub-passo `B` da série M8** — primeiro de 7
(B-H) per ADR-0073.
**Magnitude planeada**: S–M.
**Magnitude real**: **M** (Hash impls em 3 tipos +
trait modificação + 3 sentinel tests; ~70 LOC líquido).

---

## §1 O que foi feito

P204B aplicou `#[comemo::track]` ao trait `Introspector`
+ bounds `Send + Sync` per Padrão A (paridade vanilla
literal, fixado em C2 de P204A; ADR-0073 PROPOSTO).

Inventário empírico (C1) revelou divergência relevante
— 3 tipos retornados por métodos do trait não
implementavam `Hash`. **Resolvido dentro de P204B** per
spec §5 (preferido).

### Output 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-204B-inventario.md`.

Conteúdo:
- §1 C1 inventário (20 métodos × bounds).
- §2 Divergência `P204B.div-1` registada.
- §3 C3+C4 Send+Sync confirmado.
- §4 C5 alterações literais (4 ficheiros).
- §5 C6+C7+C8 verificações.
- §6 C9 sentinel tests.
- §7 Decisões.
- §8 Métricas.
- §9 Critério de fecho.
- §10 Referências.

Tamanho: ~10 KB.

### Output 2 — Alterações em código

4 ficheiros modificados em `01_core/src/entities/`:

#### 2.1 `introspector.rs`

```text
+ #[comemo::track]
- pub trait Introspector {
+ pub trait Introspector: Send + Sync {
      // 20 métodos inalterados.
  }
```

+ 3 sentinel tests (`p204b_*` em módulo `tests`).

#### 2.2 `bib_entry.rs`

```text
- #[derive(Debug, Clone, PartialEq, Eq)]
+ #[derive(Debug, Clone, PartialEq, Eq, Hash)]
  pub struct BibEntry { ... }
```

#### 2.3 `value.rs`

```text
+ impl std::hash::Hash for Value {
+     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
+         format!("{:?}", self).hash(state);
+     }
+ }
```

#### 2.4 `content.rs`

```text
+ impl std::hash::Hash for Content {
+     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
+         crate::entities::content_hash::hash_content(self).hash(state);
+     }
+ }
```

### Output 3 — este relatório

---

## §2 Tempo de execução

Sessão única; ordem grosseira:

- Leitura do spec P204B: ~3 min.
- Inventário C1 (greps + reads de trait + impl + tipos
  auxiliares): ~10 min.
- Probe `#[comemo::track]` para detectar erros
  empíricamente: ~5 min.
- Análise de divergência `P204B.div-1` + decisão
  resolver dentro de P204B: ~5 min.
- Implementação Hash impls (BibEntry derive; Value
  manual; Content manual via hash_content): ~10 min.
- Verificações cargo build + test + lint: ~5 min.
- 3 sentinel tests + verificação: ~10 min.
- Inventário interno (Output 1): ~10 min.
- Este relatório: ~5 min.

**Total**: ~65 min.

---

## §3 Métricas

| Métrica | Pré-P204B | Pós-P204B | Δ |
|---------|-----------|-----------|---|
| Tests workspace | 1824 | **1827** | +3 |
| Crystalline-lint violations | 0 | 0 | = |
| LOC produção (Hash impls + trait edits) | baseline | +~30 | +30 |
| LOC tests (sentinel) | baseline | +~30 | +30 |
| ADRs criadas/modificadas | — | 0 | = |
| Ficheiros código alterados | — | 4 | introspector.rs, bib_entry.rs, value.rs, content.rs |

---

## §4 Decisões tomadas durante a leitura

### 4.1 Probe-then-resolve em vez de inventário ex ante

Spec §3 C1 critério permite tabela inventário pré-aplicação,
mas é mais rigoroso aplicar e ler erros do compilador
para classificação precisa.

**Decisão**: aplicar `#[comemo::track]` antes de inventário
final, ler 3 erros distintos
(`Value`/`BibEntry`/`Content`: Hash não satisfeito), e
registar como `P204B.div-1`.

### 4.2 Resolver `P204B.div-1` dentro de P204B

Per spec §5 "Resolver dentro de P204B (preferido)" —
3 Hash impls são trabalho bounded (~30 LOC) e mantêm
magnitude P204B em S-M (mesmo que original do spec).

Recuar para P204A teria custo desproporcional (re-fixação
não traria alternativas melhores que Caminho A).

### 4.3 Hash via Debug formatting (pragmático)

Para `Value` e `Content`, manual `impl Hash` via Debug
formatting é a estratégia já usada em `hash_content` (P162).
Trade-off:
- Vantagem: 1 linha por impl; sem necessidade de match
  exhaustivo.
- Trade-off: colisões teóricas se Debug idêntico para
  estruturas distintas — improvável (Debug recursivo
  estrutural). Comemo trata colisões como cache miss
  (sem prejuízo correção).

### 4.4 Content delega a `hash_content`

`hash_content -> u128` já existe e é usado em
`extract_payload`. `impl Hash for Content` delega:

```rust
fn hash<H>(&self, state: &mut H) {
    hash_content(self).hash(state);
}
```

Sem duplicação de lógica. `hash_content` u128 hashado
no estado genérico.

### 4.5 BibEntry: derive trivial

Todos os fields são tipos Hash (String, Option<String>,
u32). `derive(Hash)` directo.

### 4.6 3 sentinel tests cobrindo aspectos distintos

- **`p204b_trait_e_send_sync`**: bounds estruturais.
- **`p204b_dyn_trait_implementa_track`**: macro
  `#[comemo::track]` aplicado.
- **`p204b_tagintrospector_pode_ser_tracked_via_dyn`**:
  pipeline runtime end-to-end.

Falhas distintas → problemas distintos. Sentinel coverage
multi-nível.

### 4.7 Sem alterações em consumers Layouter

P204B é foundational. Layouter consumers continuam a usar
`&dyn Introspector` directamente — `Tracked` é opt-in
e P204C migra para `Tracked<'a, dyn Introspector + 'a>`.

Compilação continua a passar porque adicionar
`#[comemo::track]` ao trait não invalida usos existentes
de `&dyn Introspector` directos.

---

## §5 Sugestão para próximo sub-passo (não-vinculativa)

**P204C — Layouter consumers via `Tracked`** (per ADR-0073
plano de materialização).

Trabalho concreto P204C (per diagnóstico §13.1):
1. Layouter ganha `'a` lifetime parameter.
2. Field `introspector: TagIntrospector` →
   `Tracked<'a, dyn Introspector + 'a>`.
3. Migração ~10 call sites em Layouter consumers
   (mod.rs, equation.rs, references.rs, outline.rs).
4. Wrapper API público mantém retrocompatibilidade.
5. Tests adaptados (construir Tracked em fixtures).

Magnitude esperada P204C: **M** (mais invasivo que P204B
porque toca em Layouter struct + consumers + tests).

Pré-condição cumprida por P204B:
- Trait `Introspector: Send + Sync` ✅.
- `#[comemo::track]` aplicado ✅.
- 3 Hash impls confirmados ✅.
- Tests sentinela activos para regressão ✅.

---

## §6 Critério de progressão respeitado

Per spec §3 C11, P204B está concluído quando:

- [x] C1 inventário completo (20 métodos × bounds —
  inventário §1).
- [x] C2 alteração aplicada (`#[comemo::track] pub trait
  Introspector: Send + Sync`).
- [x] C3+C4 `Send + Sync` confirmado (auto-trait sobre 9
  sub-stores).
- [x] C5 edições literais aplicadas (4 ficheiros).
- [x] C6 compilação verde.
- [x] C7 tests workspace verdes (1827; +3).
- [x] C8 linter 0 violations.
- [x] C9 sentinel tests (3 adicionados).
- [x] Inventário registado (Output 1).
- [x] Relatório escrito (este).

**Divergência `P204B.div-1`** registada e resolvida
internamente (3 Hash impls em Value/BibEntry/Content).

---

## §7 Não-objectivos respeitados

Per spec §7, P204B não:

- [x] Não migrou Layouter consumers (P204C).
- [x] Não adicionou lifetime parameter ao Layouter
  (P204C).
- [x] Não materializou Position (P204D).
- [x] Não adicionou `evict()` wrapper (P204E).
- [x] Não adicionou ficheiros ao corpus de paridade
  (P204F).
- [x] Não adicionou benchmarks (P204G).
- [x] Não transitou ADR-0073 para ACEITE (P204H).
- [x] Não criou ADR nova.
- [x] Não modificou ADR-0066.
- [x] Não tocou em outros traits que não `Introspector`.

**Excepção**: tocou em 3 tipos auxiliares (`Value`,
`BibEntry`, `Content`) para adicionar `Hash` impls.
Justificada empíricamente como pré-requisito de
`#[comemo::track]` (per `P204B.div-1`).

---

## §8 Achados resumo

| Achado | Implicação |
|--------|-----------|
| Trait Introspector já satisfaz restrições comemo (não-genérico, &self, args ToOwned) | C2 (Padrão A) viável literalmente |
| Trait fica `Send + Sync` automaticamente quando todos os 9 sub-stores são | TagIntrospector C3 confirmado sem trabalho |
| 4 métodos retornam Value/BibEntry/Content sem Hash | `P204B.div-1` — resolvido internamente com 3 Hash impls |
| Hash impls via Debug-based formatting (mesmo padrão hash_content P162) | Pragmático; reuso de lógica existente; sem inflação |
| 3 sentinel tests cobrem bounds + macro + runtime | Protecção contra regressão multi-nível |
| Layouter consumers continuam a compilar com &dyn Introspector | P204C pode migrar incrementalmente; P204B foundational sem invadir |

---

## §9 Notas operacionais

### 9.1 Magnitude real M (não S-M)

Spec planeou S-M; real foi M devido a `P204B.div-1`
(3 Hash impls). Ainda dentro do range aceitável; sem
inflação.

### 9.2 Diagnóstico-primeiro funcionou (de novo)

P204B C1 detectou empíricamente que 3 tipos não tinham
Hash — não previsto pela auditoria P204A (que só verificou
restrições no trait, não nos tipos retornados).

A convenção P203 §9.1 ("mesmo `*B+` começam com inventário
empírico") preveniu adopção cega que teria gerado erros
de compilação opacos.

### 9.3 Hash impls são cleanup útil para futuro

3 tipos (Value, BibEntry, Content) ganham `Hash` mesmo
que comemo desactivasse. Beneficiam:
- Uso futuro em HashMap/HashSet.
- Hashing genérico em tests / debugging.
- Paridade com vanilla (que provavelmente tem Hash em
  tipos análogos).

### 9.4 Trabalho útil cumulativo

P204B + P204A juntos:
- Auditoria empírica em profundidade máxima (16
  cláusulas).
- Diagnóstico fixado com decisões concretas (14
  cláusulas).
- ADR-0073 PROPOSTO redigido com plano de 7 sub-passos.
- Trait `Introspector` agora trackable + `Send + Sync`.
- 3 tipos auxiliares ganham Hash.
- 3 sentinel tests activos.

P204C pode iniciar imediatamente.

---

**Fim do relatório P204B.**
