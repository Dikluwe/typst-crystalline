# Relatório do passo P204A

**Data de execução**: 2026-05-06.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-204A.md`.
**Natureza**: diagnóstico-primeiro de profundidade máxima
+ ADR PROPOSTO. **37ª aplicação consecutiva** do padrão
diagnóstico-primeiro.
**Magnitude planeada**: M (S–M auditoria + S diagnóstico).
**Magnitude real**: M (≈80 min; auditoria 16 cláusulas em
5 blocos + diagnóstico 14 cláusulas + ADR PROPOSTO).

---

## §1 O que foi feito

P204A produziu **4 ficheiros** (zero código tocado):

### Output 1 — Auditoria empírica

Localização:
`00_nucleo/diagnosticos/typst-passo-204A-auditoria-comemo.md`.

Conteúdo:
- §1 Estado de partida verificado.
- §2-§6 Blocos 1-5 (A1-A16) com etiquetas + evidência.
- §7 Resumo dos achados (tabela 16 linhas).
- §8 Divergências: nenhuma.
- §9 Referências.

Tamanho: ~22 KB.

### Output 2 — Diagnóstico

Localização:
`00_nucleo/diagnosticos/typst-passo-204A-diagnostico.md`.

Conteúdo:
- §1 Sumário das decisões (tabela 14 linhas).
- §2-§15 detalhe C1-C14 instanciadas com valores
  concretos.
- §16 Critério de progressão.
- §17 Referências.

Tamanho: ~13 KB.

### Output 3 — ADR-0073 PROPOSTO

Localização:
`00_nucleo/adr/typst-adr-0073-comemo-introspector.md`.

Conteúdo (per template-adr.md):
- Contexto (M5+M6+M7+M9 fechados; baseline reconciliado).
- Decisão (mecanismo paridade vanilla literal +
  Layouter migração + Position concrete +
  política invalidação + fixpoint preservados).
- Alternativas consideradas (B/C/D/E rejeitadas).
- Consequências (positivas/negativas/neutras).
- Plano de validação (9 condições para transição
  PROPOSTO → ACEITE em P204H).
- Plano de materialização (7 sub-passos B-H).
- Cross-references.
- Pattern emergente.

Tamanho: ~10 KB.

### Output 4 — este relatório

---

## §2 Tempo de execução

Sessão única; ordem grosseira:

- Leitura do spec P204A: ~3 min.
- Bloco 1 (A1-A4) — Trait Introspector: ~10 min (leitura
  trait + sub-stores + grep consumers).
- Bloco 2 (A5-A6) — comemo actual + versão: ~5 min.
- Bloco 3 (A7-A9) — Vanilla pipeline: ~10 min.
- Bloco 4 (A10-A12) — Invariantes comemo: ~10 min
  (leitura crate source).
- Bloco 5 (A13-A16) — Estado pré-M8: ~10 min (fixpoint +
  Layouter fields + corpus + Position).
- Análise integrativa + redacção auditoria: ~15 min.
- Diagnóstico C1-C14: ~10 min.
- ADR-0073 PROPOSTO: ~5 min.
- Este relatório: ~3 min.

**Total**: ~80 min.

---

## §3 Decisões tomadas durante a leitura

### 3.1 Inventário em profundidade — Bloco 1

A1-A4 confirmou:
- Trait Introspector com 20 métodos read-only.
- 9 sub-stores TagIntrospector com granularidade per-key
  (HashMap-based) dominante.
- ~10 dos 20 métodos com consumers Layouter activos.
- `position_of` stub sem consumers em produção.

**Sem divergências** face ao snapshot 2026-05-05 §5.

### 3.2 comemo já estabelecido — Bloco 2

A5 revelou que comemo já é infraestrutura activa:
- 3 `#[comemo::track]` em world_types.rs.
- Engine usa `Tracked` e `TrackedMut`.
- Eval (markup, modules, closures) usa TrackedMut.

**`Introspector` é a última peça grande** com comentário
explícito "deferido para M7+" (linha 10). M8 fecha esta
pendência.

A6 confirmou criticamente que **comemo 0.4.0 suporta
`#[track]` em traits não-genéricos** — paridade vanilla
literal disponível.

### 3.3 Paridade vanilla literal — escolha estratégica

A7 mostrou que vanilla declara exactamente:

```rust
#[comemo::track]
pub trait Introspector: Send + Sync { ... }
```

Cristalino tem trait Introspector que satisfaz **todos**
os requisitos (não-genérico, métodos `&self` com
`ToOwned`/`Hash`).

**Decisão**: optar pelo **padrão A** (paridade literal)
em C2, em vez de B (B3 ADR-0005), C (funções livres) ou
D (sub-trait dedicada). Justificação detalhada em
diagnóstico §3.

### 3.4 Fixpoint loops ortogonais — A13

Vanilla também usa hash-based convergence (MAX_ITERS=5
em `convergence.rs:17`). Cristalino tem 2 loops
hash-based MAX=5 paralelos. comemo não substitui
fixpoint; adiciona granularidade dentro de cada
iteração.

**Decisão C7**: manter loops fixpoint cristalinos
inalterados. Sem complicação adicional em M8.

### 3.5 Position concrete — sub-passo M8 — A16

Vanilla tem `Introspector::position()` no trait. M8 inclui
Position naturalmente:
- Tipo `Position { page, point }` em L1.
- `runtime.positions` (LayouterRuntimeState).
- Layouter feedback single-pass populates.

**Decisão C8**: sub-passo M8 dedicado (P204D). Não M8.5
nem pós-M8.

### 3.6 Validação reduzida — A15

Corpus paridade actual tem 0 ficheiros que exercitam
introspection. Validação completa requer expansão XL.

**Decisão C9**: 5-7 ficheiros novos (escala reduzida)
em sub-passo dedicado (P204F). Cobertura completa fica
para pós-M8.

### 3.7 Magnitude L cross-modular — C11

Soma das decisões = M+M+S-M+S+S+M+S+S documental = **L
cross-modular**, análoga a M6 série.

**Decisão**: 7 sub-passos B-H em C12, sem condicionais.

### 3.8 ADR-0073 PROPOSTO criado em P204A

Per spec C13 + §5 Output 4. Estrutura standard com
cross-references explícitas:
- ADR-0066 superseded em P204H (não revogada).
- ADR-0067 ortogonal (permanece PROPOSTO).
- ADR-0072 mantido (loops fixpoint preservados).

ADR transita PROPOSTO → ACEITE em P204H quando 9
condições cumpridas.

---

## §4 Magnitude calibrada

**Magnitude planeada (spec)**: M (S-M auditoria + S
diagnóstico).
**Magnitude real (P204A)**: M (~80 min).

**Magnitude estimada para M8 série completa** (per C11):

| Sub-passo | Magnitude |
|-----------|-----------|
| P204B | S-M |
| P204C | M |
| P204D | S-M |
| P204E | S |
| P204F | M |
| P204G | S |
| P204H | S documental |
| **Total agregado** | **L cross-modular** |

Comparável a M6 série (P190 9 sub-passos + P191 ramo
paralelo 3 sub-passos = 12 sub-passos efectivos).

---

## §5 Sugestão para próximo sub-passo (não-vinculativa)

**Recomendação**: **P204B** — `#[comemo::track]` em trait
`Introspector`.

Justificação:
- C12 fixou ordem B → C → D → E → F → G → H.
- P204B é foundational — sem ele, nada avança.
- Magnitude S-M (manejável em sessão única).

**Trabalho concreto P204B**:
1. Inventário empírico inicial (per convenção P203 §9.1):
   - Confirmar trait `Introspector` ainda com 20
     métodos read-only.
   - Confirmar `TagIntrospector` impl pode ganhar
     `Send + Sync`.
   - Confirmar nenhum consumer está bloqueado por
     mudança de assinatura.
2. Aplicar `#[comemo::track]` ao trait + bound `: Send +
   Sync`.
3. Verificar compilação.
4. Tests workspace verdes.
5. Crystalline-lint 0 violations.
6. Relatório P204B (relatório individual).

Magnitude esperada P204B: S-M.

---

## §6 Critério de progressão respeitado

Per spec §6, P204A está concluído quando:

- [x] A1-A16 todos com etiqueta CONFIRMADO ou DIVERGÊNCIA
  registada (todos CONFIRMADO).
- [x] C1-C14 instanciadas com valores concretos.
- [x] ADR-0073 PROPOSTO escrito (4º output).
- [x] Magnitude calibrada (C11: L cross-modular).
- [x] Plano `*B+` sem condicionais (7 sub-passos B-H em
  C12).

**Sem `P204A.div-N`** registadas — todos os 16 itens
CONFIRMADO; snapshot 2026-05-05 reflecte realidade
empírica.

---

## §7 Não-objectivos respeitados

Per spec §8, P204A não:

- [x] Não tocou em código.
- [x] Não aplicou `#[comemo::track]` em qualquer ficheiro.
- [x] Não migrou Layouter consumers.
- [x] Não materializou Position.
- [x] Não promoveu ADR-0067.
- [x] Não pré-definiu sub-passos `*B+` (7 sub-passos
  emergiram de C12 baseado em A1-A16 + C1-C11; não foram
  definidos a priori na spec).
- [x] Não decidiu o escopo de M8 antes da auditoria —
  C1 fixado com base em A1-A16, não por afirmação
  herdada.

---

## §8 Achados resumo

| Achado | Implicação para M8 |
|--------|---------------------|
| Trait Introspector cristalino: 20 métodos read-only | Compatível trivialmente com `#[comemo::track]` |
| comemo 0.4.0 suporta `#[track]` em traits não-genéricos | Padrão A (paridade vanilla) viável |
| Vanilla declara `#[comemo::track] pub trait Introspector: Send + Sync` | Paridade literal disponível |
| comemo já estabelecido (World, Engine, eval); só Introspector pendente | M8 é trabalho focado, não overhaul |
| Layouter tem 22 fields; 2 categoria A trackable | Migração concentrada em `introspector` field |
| Loops fixpoint ortogonais a comemo | Mantidos em M8 sem alteração |
| Position concrete: 0 consumers + 0 corpus pressure mas é parte natural da paridade | Sub-passo M8 dedicado (P204D) |
| Corpus paridade actual: 0 ficheiros introspection | Validação reduzida (5-7 novos em P204F) |
| ADR-0066 → "intermediário até M8" + M8 chega | ADR-0073 PROPOSTO supersede em P204H |

---

## §9 Notas operacionais

### 9.1 Diagnóstico-primeiro funcionou (de novo)

P204A spec §10 advertiu: "Sem essa profundidade, M8 corre
risco de adoptar mecanismo errado por afirmação herdada."

Ao verificar empíricamente que comemo 0.4.0 suporta
`#[track]` em traits (A6.3), evitou-se assumir Padrão B3
desnecessário. Padrão A (literal) ficou disponível.

### 9.2 Trabalho útil cumulativo

P204A consolida pré-condições para M8:
- Auditoria empírica em profundidade máxima.
- Diagnóstico com decisões fixadas (não condicionais).
- ADR-0073 PROPOSTO referenciável por P204B-H.
- Plano de 7 sub-passos com magnitudes individuais.

P204B pode iniciar imediatamente.

### 9.3 Localização canónica respeitada

Per convenção P203 §9.3:
- `00_nucleo/diagnosticos/` — auditoria + diagnóstico.
- `00_nucleo/adr/` — ADR-0073.
- `00_nucleo/materialization/` — relatório.

### 9.4 Volume de leitura

Per spec §10 ("Volume de leitura é maior que P203A"):
- 20 métodos do `Introspector` cristalino + impl
  TagIntrospector.
- Pipeline completo do vanilla `Introspector`
  (introspector.rs + typst-layout/src/introspect.rs +
  engine.rs + convergence.rs).
- README + lib.rs + track.rs + macros do crate `comemo`
  + exemplo calc.
- 22 fields do Layouter para classificação F3.
- Corpus paridade (30 .typ files) — confirmação
  empírica de zero introspection.

Volume comparável a P201/P202 administrativos (sem
chegar ao ~100 ficheiros desses).

---

**Fim do relatório P204A.**
