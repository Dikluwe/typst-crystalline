# Relatório consolidado — Série P182 (lacuna #4 fechada)

**Data**: 2026-05-02
**Série**: P182A → P182F (6 sub-passos)
**Resultado**: lacuna #4 (`is_numbering_active` / `numbering_active`) **resolvida**; M9 features **11/11** (completa); cumulativo +18 tests workspace.

---

## §1 Resumo executivo + pipeline final

Série P182 fecha a lacuna #4 do plano M3/M5/M9 — paridade `TagIntrospector` ↔ `CounterStateLegacy.numbering_active`. Pipeline materializado em 5 sub-passos de implementação (P182B–F) precedidos por 1 passo de diagnóstico (P182A); todos magnitude S; total cumulativo S-M conforme estimativa P182A.

**Pipeline final**:

```
Content::SetHeadingNumbering { active }
    │
    ├─► walk arm canonical (introspect.rs:455–457)         [legacy — intocado, M6 elimina]
    │   └─► state.numbering_active.insert("heading", active)
    │
    └─► extract_payload (P182C)
        └─► ElementPayload::StateUpdate {
                key:    "numbering_active:heading",
                update: StateUpdate::Set(Box::new(Value::Bool(active))),
            }
            │
            └─► from_tags::StateUpdate arm (P182C, com auto-init)
                └─► StateRegistry: init na 1ª ocorrência; update nas seguintes

Layouter consumers (P182D, substitution-with-fallback):
- mod.rs:301 (heading prefix):
    self.introspector.is_numbering_active("numbering_active:heading")    [P182B trait method]
    || self.counter.is_numbering_active("heading")                       [legacy fallback]
- equation.rs:24 (equation auto-numeração):
    self.introspector.is_numbering_active("numbering_active:equation")
    || self.counter.is_numbering_active("equation")
```

Para heading: ambos paths populados pelo mesmo `Content::SetHeadingNumbering` no documento; fallback é redundante mas inofensivo. Para equation: cristalino não tem variant `Content::SetEquationNumbering`; Introspector retorna sempre `false` para `numbering_active:equation`; fallback legacy é o caminho activo.

---

## §2 Sub-passos materializados

| Sub-passo | Escopo | Magnitude | Δ tests | Hashes L0 alterados |
|-----------|--------|-----------|---------|---------------------|
| P182A | Diagnóstico-primeiro / 6 cláusulas fixadas / 8-section diagnóstico + 14-section relatório | S (documental) | 0 | 0 (apenas diagnóstico/relatório criados) |
| P182B | Trait method `is_numbering_active(key)` + impl `TagIntrospector` + 5 tests unit | S | +5 | `entities/introspector.md` |
| P182C | `extract_payload` arm `Content::SetHeadingNumbering` + locatable promovido + auto-init em `from_tags::StateUpdate Set` + 5 tests | S | +5 | `rules/introspect/extract_payload.md`, `rules/introspect/locatable.md`, `rules/introspect/from_tags.md` |
| P182D | 2 consumers Layouter migrados (heading-arm `mod.rs:301` + equation-arm `equation.rs:24`) com substitution-with-fallback + 3 tests | S | +3 | `rules/layout.md` (efeito colateral em 9 ficheiros via `--fix-hashes`) |
| P182E | 5 tests E2E em submódulo `p182e_e2e_heading_numbering` | S | +5 | 0 |
| P182F | Fecho documental: `m1-lacunas-captura.md` (4 sítios) + `auditoria-fresh-projecto.md` F1 + relatório consolidado | S (documental) | 0 | 0 |
| **Total série** | **5 implementação + 2 documental** | **S-M cumulativo** | **+18** | **5 L0s** |

Baseline P181J: **1.738** tests workspace verdes. Após P182F: **1.756** tests verdes. Δ cumulativo: **+18** (5 unit em P182B + 5 unit em P182C + 3 E2E em P182D + 5 E2E em P182E).

---

## §3 Decisões arquiteturais — 6 cláusulas P182A fechadas

| # | Cláusula | Decisão | Justificação |
|---|----------|---------|--------------|
| 1 | Mecanismo | **M1** — reusar `StateRegistry` P171 com chave canónica `numbering_active:heading` | Sem novo `ElementPayload` variant; sem novo sub-store; replica P171/P173 |
| 2 | Default value | **OFF** | Paridade vanilla `Option<Numbering>` `None` + cristalino actual `unwrap_or(false)` |
| 3 | Consumers | **2 Layouter** (`mod.rs:301`, `equation.rs:24`) | 4 leituras totais, mas 2 intra-walk excluídas (consomem `state` local que o walk constrói) |
| 4 | Localização exacta | tabela `mod.rs:301` + `equation.rs:24` | Substitution-with-fallback P168/P181G |
| 5 | Forma da API | **A2** — helper `Introspector::is_numbering_active(key) -> bool` | Replica precedente P181F (`bib_*_for_key`); encapsula `Value::Bool` matching |
| 6 | Critério de fecho | **Opção 3** — infra pronta + consumer migrado; legacy permanece até M6 | Simétrico com lacuna #6 P181 |

Sem ADR criada. Sem DEBT aberto. M1 reusou infra P171; A2 replicou P181F.

---

## §4 Achados não-triviais durante execução

### §4.1 Vanilla não tem `numbering_active` em lado algum (P182A §2)

`grep -rn "numbering_active" lab/typst-original/crates/` retornou **zero matches**. Vanilla resolve via `pub numbering: Option<Numbering>` em `HeadingElem` (`heading.rs:134`) e `EquationElem` (`equation.rs:67`), com escopo léxico via StyleChain hierárquica. Cristalino diverge: `HashMap<String, bool>` global por chave, "última escrita ganha".

**Implicação**: lacuna #4 não é "feature implementada em vanilla mas ausente em cristalino" — é **divergência arquitectural** consciente, consequência da ausência de StyleChain location-aware em cristalino. P182 manteve a divergência (não materializou StyleChain) e migrou o boolean cristalino para o Introspector. Quando StyleChain for materializada (M+), cristalino pode revisitar a representação de `numbering_active`.

### §4.2 Variant é `{ active: bool }`, não `{ key, value }` (P182A §1, surpresa face ao texto da lacuna)

Texto original em `m1-lacunas-captura.md` linha 64 sugeria genéricamente `{ key, value }`. Forma real é apenas `Content::SetHeadingNumbering { active: bool }` — booleano, com `"heading"` hardcoded no walk arm + extract_payload. Não há equivalente para equation (`Content::SetEquationNumbering` não existe em cristalino). P182 emite `numbering_active:heading` mas mantém `numbering_active:equation` sem emitter — fallback legacy é o caminho activo para equation.

### §4.3 Auto-init em `from_tags::StateUpdate` foi gate trivial inesperado (P182C §5.1)

Diagnóstico P182A confiou que "`from_tags` arm `StateUpdate` (já existente) cobre". Verificou-se na execução de P182C que o `state_registry::update` é defensivo (P171 padrão "update sem init é silenciosamente ignorado"); `Content::SetHeadingNumbering` não tem `Content::State` antecedente, logo o update perdia-se.

Solução cirúrgica: arm `from_tags::StateUpdate Set` ganha verificação `value_at(key, loc).is_none()` — se `None`, chama `state.init`; senão segue `state.update`. Não toca `state_registry` (semântica P171 preservada para userspace `Content::State` + `Content::StateUpdate` sequence). Documentado em L0 `from_tags.md` + comentário inline.

Comportamento divergente para userspace: callers que escrevam `#state(...).update(...)` sem `#state(...)` antes ganham auto-init em vez de defensive ignore — divergência consciente face a P171 strict, ainda mais permissiva que vanilla (que gera erro). Inofensivo: testes que dependiam do silent ignore como heurística de erro testam estado não-inicializado via `state_value(key, loc)` directamente, que continua a retornar `None`.

### §4.4 Re-update revela que fallback é o caminho funcional, não Introspector (P182E §5.2)

Test E2E `re_update_active_true_then_false` confirma que, em sequência `SetHeadingNumbering(true) → H1 → SetHeadingNumbering(false) → H2`, o output observable correcto (H1 com prefixo, H2 sem) **vem do fallback `|| self.counter.is_numbering_active(...)`**, não do Introspector.

Razão: `Introspector::is_numbering_active` usa `state_final_value` (último update aplicado). Após o segundo `SetHeadingNumbering(false)`, `final_value` retorna `false` em ambos pontos — daria "false em ambos headings" se fosse o único caminho. Mas o fallback consulta `self.counter.numbering_active["heading"]`, que é mutável durante o layout walk e reflecte o valor **na altura** de cada heading ser processado.

**Implicação para M6 cleanup**: antes de remover o fallback `||`, o Introspector precisa ganhar semântica location-aware — ex. `is_numbering_active_at(key, location)` delegando a `state_value(key, location)` em vez de `final_value`. Caso contrário, regressões em re-update emergem. Trabalho substancial em M6+, **input para diagnóstico P185A** (cláusula 2 estratégia por arm ou cláusula 6 critério de fecho de M6).

---

## §5 Estado final M9 + M5

### M9 features — 11/11 ✅ **completo**

| # | Feature | Sub-passo | Estado |
|---|---------|-----------|--------|
| 1 | `query` flat | P165 | ✅ |
| 2 | `query_by_label` | P165 | ✅ |
| 3 | `query_first` / `query_unique` | P165 | ✅ |
| 4 | `figure_number_for_label` | P168 | ✅ |
| 5 | `query_metadata` | P169 | ✅ |
| 6 | `formatted_counter` | P170 | ✅ |
| 7 | `state_value` / `state_final_value` | P171 | ✅ |
| 8 | `query` via Selector | P175 | ✅ |
| 9 | `formatted_counter_at` | P177 | ✅ |
| 10 | `bib_entry_for_key` / `bib_number_for_key` | P181F | ✅ |
| 11 | `is_numbering_active` | **P182B** | ✅ |

M9 **completa**.

### M5 consumers Introspector migrados

| # | Consumer | Sub-passo |
|---|----------|-----------|
| 1 | figure-ref (`references.rs::layout_ref`) | P168 |
| 2 | cite-arm (`mod.rs:584-597`) | P181G |
| 3 | heading-arm (`mod.rs:301`) | P182D |
| 4 | equation-arm (`equation.rs:24`) | P182D |

Total: **4 consumers migrados** (heading e equation contam como 2 distintos por chave/arm). Fallback `||` legacy preservado em todos para janela compat M6.

Consumers ainda **não migrados**: outline, counter helpers, section-arm, layout_equation (parcialmente em P182D mas apenas para `is_numbering_active`), e leituras intra-walk em `introspect.rs:360, 378` (estas últimas estruturalmente não migráveis — consomem `state` local que o walk constrói antes do Introspector existir). Ver `inventario-consumers-counter-state-legacy.md` (P167).

---

## §6 Estado final lacunas

7 lacunas documentadas em `m1-lacunas-captura.md`:

| # | Estado | Sub-passo de fecho |
|---|--------|---------------------|
| 1 | Aberta — adiar | (intencional) |
| 2 | Aberta — adiar M3+ | (intencional) |
| 3 | Aberta — manter (intencional) | (sem fecho previsto) |
| 4 | ✅ Resolvida | **P182** |
| 5 | ✅ Resolvida | P170 |
| 6 | ✅ Resolvida | P181 |
| 7 | ✅ Resolvida | P178 |

**4 resolvidas / 3 abertas**. Nenhuma das 3 abertas bloqueia M5/M6/M7/M8.

---

## §7 Pendências cumulativas + janela compat M6

### Pendências M6 (F1 do `auditoria-fresh-projecto.md`)

Inalteradas face a P182A §3 cláusula 6 (Opção 3):

- Field `CounterStateLegacy.numbering_active` legacy continua até M6.
- Walk arm canonical em `introspect.rs:455–457` continua write legacy paralelo (sentinela `walk_continua_a_popular_legacy_apos_p182cd` em P182E protege contra regressão).
- Write paralelo em `layout/counters.rs:11–13` continua.
- Copy-sites em `mod.rs:1414, 1442` continuam.
- Leituras intra-walk em `introspect.rs:360, 378` continuam (consomem `state` local; não migráveis).
- Fallback `|| self.counter.is_numbering_active(...)` em ambos consumers Layouter.

### Pendências adicionais identificadas em P182

- **P182C §5.1**: `from_tags::StateUpdate` auto-init é divergência face a P171 strict (`update` defensivo). Documentada em L0 `from_tags.md`. Inofensiva para userspace; necessária para state interno.
- **P182E §5.2**: `Introspector::is_numbering_active` usa `state_final_value`; insuficiente para re-update sem fallback legacy mutável. M6 cleanup do fallback exige `is_numbering_active_at(key, location)` location-aware. Trabalho substancial; **input para P185A diagnóstico**.

### Janela compat M6 — alvos de eliminação

Quando F1 retomar (P185+, alvo M6):
1. Remover field `CounterStateLegacy.numbering_active` (e os outros 17 fields).
2. Remover walk arm canonical em `introspect.rs:455–457`.
3. Remover write paralelo em `layout/counters.rs:11–13`.
4. Remover copy-sites em `mod.rs:1414, 1442`.
5. Migrar leituras intra-walk em `introspect.rs:360, 378` (via state local construído durante o walk em vez de `CounterStateLegacy` campo).
6. **Adicionar `Introspector::is_numbering_active_at(key, location)` location-aware** (cf. §4.4) — pré-requisito para passos 7–8.
7. Migrar Layouter consumers de `is_numbering_active(key)` para `is_numbering_active_at(key, location)`.
8. Remover fallback `||` `self.counter.is_numbering_active(...)` em ambos consumers Layouter.

Pontos 6–8 são consequência directa de P182E §5.2.

---

## §8 Próximos passos sugeridos

- **P183A** (M4 série) — migrar consumers M5 restantes (outline, counter helpers, section-arm, layout_equation parcial). Ver `inventario-consumers-counter-state-legacy.md` para inventário detalhado. Bloqueios próprios em alguns (lacuna #3 outline body frozen vs hash; padrões mutação).
- **P184A ou similar** (M9 extensão) — hipotético; M9 está formalmente completa em 11/11. Features novas seriam adições, não fechamentos de lacunas.
- **P185A** (M6 série / F1 retomar) — diagnóstico-primeiro para eliminação de `CounterStateLegacy`. Deve ler P182E §5.2 e incorporar como cláusula obrigatória (Introspector location-aware antes de remover fallbacks). Magnitude **L** estimada — F1 é "grande" per `auditoria-fresh-projecto.md:123`.
- **Aproveitamento de `is_numbering_active_at`** (M+ ou M6+): quando StyleChain location-aware for materializada (separado de F1), a divergência face a vanilla (§4.1) pode ser revisitada — `numbering_active` deixaria de ser bool global por chave e passaria a `Option<Numbering>` por elemento via styles.

---

## §9 Conclusão

Série P182 fechou a última lacuna M9 (#4) seguindo o padrão diagnóstico-primeiro estabelecido em P154A/P181A. Magnitude S-M cumulativa, sem ADR, sem DEBT, sem regressão. Output observable em produção inalterado em todos os 6 sub-passos.

Dois achados não-triviais merecem registo arquitectural: (1) auto-init em `from_tags::StateUpdate` é necessário para state interno sem `Content::State` antecedente — divergência consciente face a P171 strict; (2) o caminho funcional para re-update é o fallback legacy mutável, não o Introspector — implica trabalho substancial location-aware em M6 cleanup (input para P185A).

Padrões P168/P181F/P181G replicados literalmente: helper trait method para encapsular tipos concretos, substitution-with-fallback para janela compat, tests E2E em submódulo dedicado para regressão visível.

P182 é a 8ª aplicação documentada do padrão diagnóstico-primeiro (P131A, P132A, P140A, P148, P154A, P181A, P181J consolidador, P182A; P182F como consolidador). O instrumento mantém a sua eficácia — evita planos monolíticos, força auditoria empírica, expõe surpresas (vanilla não tem `numbering_active`; auto-init é gate inesperado; fallback é caminho funcional) antes da escrita de código.

M9 features: **11/11** ✅ completa. Lacuna #4: ✅ resolvida. Próximo substantivo: P183A (M4 consumers restantes) ou P185A (M6 série / F1 retomar com input P182E §5.2).
