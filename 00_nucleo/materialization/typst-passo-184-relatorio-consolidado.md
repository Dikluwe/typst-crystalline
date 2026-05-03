# Relatório consolidado P184 — desbloqueio C3 (figure auto-number per kind)

**Período**: 2026-05-03 (P184A → P184F, sequência única)
**Série**: P184A diagnóstico → P184B refinamento arm → P184C trait method
+ helper → P184D migração consumer → P184E tests E2E → P184F fecho.
**Resultado agregado**: C3 desbloqueado e validado em pipeline real;
DEBT M4-residual reduz para C1+C2; M5/M4 progresso 6/12 read-sites
migrados; tests workspace 1.764 → 1.769 (+5 E2E + 8 unit em P184C);
zero violations linter.

---

## §1 Resumo executivo + pipeline final desbloqueio C3

P184 atacou o eixo 2 do bloqueio P183D (consumer C3 figure
auto-number per kind, `mod.rs:435–439`). A regra dos 2 eixos
(P183C §6) classificou C3 como "**eixo 1 OK** (semântica
snapshot-final adequada para figures pós-walk fixos), **eixo 2
falha** (chave global `"figure"` em vez de per-kind `figure:{kind}`)".
P183D §7 declarou C3 como o consumer mais barato individualmente
para desbloquear — não exige cross-cutting M6+ change.

P184 materializou esse desbloqueio em 6 sub-passos (5 implementação +
1 fecho documental):

| Passo | Escopo | Magnitude |
|-------|--------|-----------|
| P184A | Diagnóstico (6 cláusulas + plano sub-passos) | S documental |
| P184B | Arm Figure populates `figure:{kind}` | S |
| P184C | Trait method `figure_number_at_index` + helper | S |
| P184D | Consumer migrado (substitution-with-fallback) | S |
| P184E | 5 tests E2E paridade C3 | S |
| P184F | Relatório consolidado + nota DEBT preventiva | S documental |

Pipeline final:

1. **Walk** (`introspect.rs:391–399`) continua a popular legacy
   `state.figure_numbers[kind]` paralelamente (dead code em
   produção; M6 elimina junto com `CounterStateLegacy`).
2. **`from_tags` arm Figure** (`from_tags.rs:71–112`) emite duas
   chamadas `apply_at` ao `CounterRegistry`:
   - `apply_at(format!("figure:{kind_key}"), Step, loc)` — chave
     per-kind (P184B).
   - `apply_at("figure", Step, loc)` — chave global mantida em
     paralelo durante janela compat M6 (dead code factual).
3. **Trait `Introspector`** ganha `figure_number_at_index(&str, usize) -> Option<usize>`
   (P184C) que constrói `format!("figure:{}", kind)` e delega a
   `CounterRegistry::value_at_index(&str, usize) -> Option<&[usize]>`
   (helper novo P184C).
4. **Layouter** (`mod.rs:435–439`) consulta Introspector primeiro;
   fallback legacy + heurística `unwrap_or(idx + 1)` final
   defensiva (P184D).

Output observable preservado por construção (counter flat com
`apply_at(Step)` produz snapshots `[1], [2], [3], …` — `.last()` no
trait extrai número 1-based; coincide com sequência que legacy
geraria e que `idx + 1` heurística replica). Tests E2E (P184E)
confirmam paridade empiricamente.

---

## §2 Sub-passos materializados — métricas

| Passo | LOC produção | LOC tests | Tests Δ | L0s editados | Hashes sync |
|-------|--------------|-----------|---------|--------------|-------------|
| P184A | 0 | 0 | 0 | 0 | n/a |
| P184B | ~10 (arm Figure) | 0 | 0 | `from_tags.md` | 1 file |
| P184C | ~20 (helper + trait + impl) | ~80 (3+5 unit) | +8 | `introspector.md`, `counter_registry.md` | 2 files |
| P184D | ~10 (consumer C3) | 0 | 0 | `layout.md` | 9 files (todos os módulos `rules/layout/`) |
| P184E | 0 | ~150 (5 E2E + helper) | +5 | 0 | 0 |
| P184F | 0 | 0 | 0 | 0 | 0 |
| **Σ** | ~40 | ~230 | **+13** | 4 L0s | 12 files |

Workspace tests: 1.756 (P183A baseline) → 1.769 (pós-P184F). Δ +13:
5 E2E em P184E + 5 unit `figure_number_at_index` (P184C trait method)
+ 3 unit `value_at_index` (P184C helper).

---

## §3 Decisões arquitecturais — 6 cláusulas P184A fechadas

| Cláusula | Decisão | Justificação |
|----------|---------|--------------|
| 1 — Convenção de chave | `figure:{kind}` (default `figure:image`) | Replica `numbering_active:heading` (P182A) com mesmo separador `:`. Default `"image"` já estabelecido em `introspect.rs:391` e `mod.rs:431`. |
| 2 — Método trait | `figure_number_at_index(&self, kind: &str, idx: usize) -> Option<usize>` | `_at_index` documenta posição (não Location). Variação clara de `formatted_counter_at` (P177). Opção γ (location-aware) misturaria escopo com P185. |
| 3 — Sub-store alvo | `CounterRegistry` (não 6º sub-store dedicado) | Já desenhado para counters por kind. Helper `value_at_index` adicionado para acesso por idx. |
| 4 — Forma migração | Substitution-with-fallback | Replica P168/P181G/P182D. Reversível; padrão estabelecido. |
| 5 — Legacy paralelo | Manter até M6 (Opção 1) | Simetria com P181/P182. **Registo honesto**: legacy é dead code factual, não "redundância defensiva". Cleanup orgânico em M6. |
| 6 — Critério de fecho | Opção 3 (infra + consumer + tests E2E) | Replica P181/P182. P184E satisfez literalmente. |

---

## §4 Achados não-triviais

### §4.1 "Dead code em produção" ratificado três vezes

P184A §3.6 descobriu (via auditoria empírica) que `state.figure_numbers`
legacy é populado durante `introspect_with_introspector` mas **nunca
copiado** ao Layouter (copy-sites em `mod.rs:1414–1430` no-TOC e
`mod.rs:1444–1460` TOC fixpoint não copiam o campo). Em produção,
`self.counter.figure_numbers.get(kind_key)` retorna sempre `None`,
e `unwrap_or(idx + 1)` heurística é o caminho real.

P184B §1, P184D §1, P184E `.C` ratificaram este achado em três
contextos progressivos (refinamento arm sem regressão; migração
consumer sem regressão; pipeline E2E sem regressão). Honestidade
obrigatória respeitada — não rebaptizado como "redundância
defensiva".

Cleanup: orgânico em M6 junto com eliminação geral de
`CounterStateLegacy`.

### §4.2 Inversão Introspector vs fallback legacy

P182 (heading numbering) fechou lacuna #4 mas Introspector ficou
**redundante** (fallback legacy mutável durante walk é o caminho
funcional para casos limite). P184 fecha C3 com **Introspector
como caminho funcional** (legacy é dead code → heurística).

Esta inversão é a primeira na série M4/M5 e ratifica empiricamente
a regra dos 2 eixos: eixo 1 OK + eixo 2 atendido = consumer
realmente migrado, não apenas estruturalmente. C3 é o primeiro
consumer onde Introspector populado é o caminho activo, não
redundância paralela.

### §4.3 Ajuste empírico em P184E `.E`

A spec P184E sugeria asserir "Tabela 1:" para figures de kind
table. Inspecção empírica em `mod.rs:440` revelou que o Layouter
formata sempre `format!("Figura {}: ", figure_number)` independente
do kind. Cláusula gate trivial activada: asserções `.E` ajustadas
para observar captions únicos + contar ocorrências de "Figura 2:"
(esperado 2× para image[1] + table[1]).

A formatação genérica "Figura" é provavelmente bug pré-existente
(uma `Tabela 1:` ou `Listing 1:` seria mais informativa); fora do
escopo P184. Pode ser tratado em passo dedicado de DEBT-cleanup ou
como parte do alinhamento i18n futuro.

### §4.4 Helper P184C `value_at_index` foi necessário

P184A previu cláusula gate trivial em `.A.4` para decidir entre
delegar directamente ao `CounterRegistry` ou adicionar helper.
Inspecção empírica confirmou que o `history` HashMap é privado e
os métodos públicos existentes (`value`, `value_at`, `format`) não
suportam acesso por posição. Helper `value_at_index` adicionado em
`.C`, com 3 tests unit. Magnitude continuou S.

---

## §5 Estado final M9 e M5/M4

### M9 — features

11/11 (inalterado pós-P184F).

| # | Feature | Estado |
|---|---------|--------|
| 1 | Metadata | ✅ P169 |
| 2 | format_hierarchical / hierarquia | ✅ P170 (lacuna #5) |
| 3 | State runtime mutable | ✅ P171 |
| 4 | Func eval (StateUpdate::Func) | ✅ P173 |
| 5 | Selector::Kind query | ✅ P175 |
| 6 | formatted_counter_at history | ✅ P177 |
| 7 | Outline | ✅ P178 (lacuna #7) |
| 8 | Bibliography | ✅ P181 (lacuna #6) |
| 9 | numbering_active StateRegistry | ✅ P182 (lacuna #4) |
| 10 | figure_number_at_index per kind | ✅ **P184** |
| 11 | (slot livre — futuro M9 ext) | — |

P184 não adiciona M9 feature nova porque `figure_number_at_index` é
extensão de feature existente (counter access por idx, ortogonal a
P170/P177 que cobrem por hierarquia / por Location).

### M5/M4 — read-sites migrados

6/12 read-sites (P183A §1):

| # | Site | Estado migração |
|---|------|------------------|
| 1 | `equation.rs:33` (`is_numbering_active`) | ✅ P182D |
| 2 | `equation.rs:97` (`get_flat("equation")`) | ❌ **C2** (espera P185+) |
| 3 | `mod.rs:308` (`is_numbering_active`) | ✅ P182D |
| 4 | `mod.rs:310` (`format_hierarchical`) | ❌ **C1** (espera P185+) |
| 5 | `mod.rs:435–439` (`figure_numbers`) | ✅ **P184D** |
| 6 | `mod.rs:601` (`bib_entries.find`) | ✅ P181G |
| 7 | `mod.rs:609` (`bib_numbers.get`) | ✅ P181G |
| 8 | `mod.rs:1072` (`label_pages` move) | n/a (fixpoint side-channel; fora M4) |
| 9 | `outline.rs:24` (`headings_for_toc`) | ❌ bloqueado lacuna #3 (separado) |
| 10 | `outline.rs:35` (`known_page_numbers`) | n/a (fixpoint side-channel) |
| 11 | `references.rs:49` (`figure_label_numbers`) | ✅ P168 |
| 12 | `references.rs:53` (`resolved_labels`) | pendente P183E (não corrido) |

Σ migrados M4/M5: 6 (P168 + P181G ×2 + P182D ×2 + **P184D**).
DEBT M4-residual: 2 (C1 + C2).

---

## §6 Estado final lacunas M1

| # | Lacuna | Estado |
|---|--------|--------|
| 1 | `figure.kind` None vs "image" default | aberta — adiada (não bloqueia) |
| 2 | Auto-labels só em state | aberta — adiada (não bloqueia) |
| 3 | `headings_for_toc` body frozen | aberta — manter intencional (separada) |
| 4 | `is_numbering_active` | ✅ P182 |
| 5 | `format_hierarchical` | ✅ P170 |
| 6 | `bib_entries`/`bib_numbers` | ✅ P181 |
| 7 | `has_outline` | ✅ P178 |

3 abertas; nenhuma bloqueia M5/M6/M7/M8.

---

## §7 Pendências cumulativas + janela compat M6

### Pendências activas

- **C1 + C2 (DEBT M4-residual)**: heading prefix counter +
  equation counter. Esperam **P185+ location-aware Layouter**
  (pendência paralela P182E §5.2). Cross-cutting M6+ change.
- **C5 TOC entries**: `outline.rs:24` (`headings_for_toc`).
  Bloqueado por lacuna #3 (body frozen vs hash em tags).
  Separado da série P183/P184.
- **Lacunas #1, #2, #3**: abertas; decisão "adiar/manter
  intencional"; nenhuma bloqueia milestones M5–M8.

### Janela compat M6 — dead code legacy a eliminar

Quando F1 retomar e M6 começar, o cleanup geral de
`CounterStateLegacy` deve eliminar:

- `state.figure_numbers: HashMap<String, Vec<usize>>` —
  populado em walk mas nunca copiado ao Layouter (P184A §3.6).
- `state.local_figure_counters: HashMap<String, usize>` —
  auxiliar interno do walk, sem consumer externo.
- Chave global `"figure"` no `CounterRegistry` populada em
  paralelo (`from_tags` arm Figure linha P184B; sem consumers).
- `figure.rs:16` doc comment factualmente desactualizado
  ("introspecção pré-computou os números").
- Fallback `or_else(...legacy...)` em `mod.rs:435–439` deixa de
  ter rede de segurança útil (Introspector populado é
  suficiente).
- Fallback final `unwrap_or(idx + 1)` pode ser preservado como
  defesa contra Introspector vazio (caller pass-through), ou
  eliminado se entry points exigirem Introspector populado.

---

## §8 Próximos passos sugeridos

### Imediato — P185A

Diagnóstico-primeiro: location-aware Layouter para desbloquear
C1+C2. Pendência P182E §5.2 finalmente atacada. 6 cláusulas
prováveis:

1. Mecanismo de propagação Location ao Layouter (parâmetro
   propagado vs `figure_progress`-like com Locator dedicado vs walk
   sincronizado).
2. Trait method novo `formatted_counter_at` ou `flat_counter_at` —
   já existe `formatted_counter_at` (P177). Ver se cobre C1
   (heading prefix) directamente.
3. Mecanismo para Layouter conhecer a sua Location no ponto da
   consulta.
4. Forma de migração (substitution-with-fallback vs alternativa).
5. Legacy paralelo durante M6.
6. Critério de fecho.

### Sequencial pós-P185A

- **P186**: `Content::Equation` locatable. Ainda não tem variant
  `ElementPayload::Equation`; emissão de Tag em `from_tags`
  depende disto. Pré-requisito para C2.
- **P187**: migrar C1 (heading prefix). Pré-requisito: P185+
  location-aware.
- **P188**: migrar C2 (equation counter). Pré-requisito: P185+
  + P186.
- **P189**: walk puro (M5 finalização). Eliminar mutações em
  `CounterStateLegacy` durante walk.
- **P190**: M6 — eliminar `CounterStateLegacy`.

### Padrão diagnóstico-primeiro

P184A é a 9ª aplicação consecutiva do padrão diagnóstico-primeiro:
P131A, P132A, P140A, P148, P154A, P181A, P182A, P183A, **P184A**.
P185A continuará. Padrão consolidado.

---

## §9 Conclusão

P184 fechou C3 (figure auto-number per kind) em **6 sub-passos**
de magnitude S agregada (~40 LOC produção + ~230 LOC tests).
Resolveu eixo 2 do bloqueio P183D isoladamente, sem cross-cutting
M6+ change. **Primeiro consumer onde Introspector populado é o
caminho activo, não redundância paralela** — ratifica empiricamente
a regra dos 2 eixos (eixo 1 OK + eixo 2 atendido = consumer
realmente migrado).

**Estado final pós-P184F**:
- P184 série: A ✅ B ✅ C ✅ D ✅ E ✅ **F ✅**. Fechada.
- Tests workspace: 1.769 verdes (Δ +13 vs P183A baseline 1.756);
  zero violations linter.
- C3 desbloqueado e validado em pipeline real.
- DEBT M4-residual: cobre apenas **C1 + C2** (cenário B
  confirmado; nota preventiva registada em
  `m1-lacunas-captura.md` anexo).
- M9: 11/11 (inalterado).
- M5/M4 progresso: 6/12 read-sites migrados.
- Lacunas: 3 abertas (#1, #2, #3 — adiadas intencionalmente;
  nenhuma bloqueia M5/M6/M7/M8).
- Próximo passo substantivo: **P185A** — diagnóstico
  location-aware Layouter para desbloquear C1+C2.
- 43 passos executados (P184E = 42 + P184F = 43).

P184 é instrumento. Encerra a série; não toca código de produção
em P184F (apenas documentação consolidada + nota preventiva).
