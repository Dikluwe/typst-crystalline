# Relatório consolidado — Série P194

**Período**: 2026-05-04 (P194A diagnóstico + P194B implementação)
**Magnitude agregada**: S
**Estado**: ✅ Série fechada (A ✅ B ✅) — **passo 2 da
sequência §9 P189 consolidado**
**ADR vinculada**: nenhuma
**DEBT**: M5-residual avança 1 dos 3 pré-requisitos

---

## §1 Resumo executivo

Migração C4 (resolved label resolution) materializada em
`layout/references.rs:53-67::layout_ref`. Replica padrão
P184D/P187B/P188B substitution-with-fallback com primitiva
`resolved_label_for` (P193B) na variante **Opção C**:

```rust
let display_text = match layouter.introspector
    .resolved_label_for(target)
    .or_else(|| layouter.counter.resolved_labels.get(target).map(String::as_str))
{
    Some(text) => text.to_string(),
    None       => format!("@{}", target.0),
};
```

**Estado temporário** (não permanente face a P188B):
sub-store P193B vazio em produção até P195+ activar
populate via Tag. Output observable inalterado por
construção — fallback legacy chamado consistentemente.

Δ tests cumulativo: **+4** (1821 → 1825) com **zero
regressões**.

P194 é **passo 2 dos 7** identificados em P189 §9 para
fechar M5 universalmente. **2 dos 4 pré-requisitos**
cumpridos.

---

## §2 Sub-passos materializados

| Passo | Magnitude planeada | Magnitude real | Δ tests | L0s tocados |
|-------|---------------------|-----------------|---------|-------------|
| **P194A** | S (diagnóstico) | S | 0 | nenhum |
| **P194B** | S (agregado) | S | **+4** | `rules/layout.md` |
| **Total** | — | — | **+4** | 1 L0 |

P194B agregou em sub-passo único:
- `.A` auditoria (site C4 confirmado).
- `.B` migração consumer C4 + comentário inline curto.
- `.C` actualização L0 `rules/layout.md` (secção C4 +
  estado temporário documentado).
- `.D` 4 tests E2E em `mod p194b_c4_resolved_label`.
- `.E` verificação estrutural (14/14).
- `.F` actualização nota DEBT M5-residual.
- `.G` relatório consolidado P194 (este ficheiro).

---

## §3 Decisões arquiteturais

### 6 cláusulas P194A fechadas

| # | Cláusula | Decisão | Sub-passo |
|---|----------|---------|-----------|
| 1 | Forma da expressão | **Opção C** match com `or_else` propagando `Option<&str>` | P194B `.B` |
| 2 | `None` do Introspector | Aceitar; comentário leve | P194B `.B` |
| 3 | Copy-sites Layouter | **Manter** durante janela compat M5 | (não tocados) |
| 4 | Acesso `layouter.introspector` | Cenário α directo | P194B `.B` |
| 5 | Documentação inline | Comentário curto referenciando sequência §9 P189 | P194B `.B` |
| 6 | Critério fecho | Consumer migrado + tests E2E + DEBT actualizado | P194B `.G` |

### Sem ADR — replicação de padrão

Padrão P184D/P187B/P188B replicado com variante Opção C
(`Option<&str>` propagado, único `to_string()` no Some
arm).

---

## §4 Achados não-triviais durante execução

### P194A §11.1 — Site C4 está depois de figure-ref early-returns

`layout_ref` tem 3 caminhos em ordem:
1. Linha 44: figure-ref via Introspector (P168 já migrado).
2. Linha 49: figure-ref via legacy fallback (P168 mantido).
3. **Linha 53: resolved-labels** (este — C4) — atinge-se
   apenas se label não corresponder a figure.

C4 cobre Heading auto-toc + Labelled explicit (não
figures). Migration não interage com P168.

### P194A §11.3 — Layouters secundários têm Introspector próprio

`mod.rs:1472, 1511` mostram que cada Layouter recebe a sua
cópia do Introspector. Em P194B, sub-store
`intr.resolved_labels` está vazio (P193B abre infra;
populate em P195+) → fallback legacy via
`l.counter.resolved_labels` é caminho funcional.

Copy-sites `mod.rs:1481, 1512` continuam necessários.

### P194A §11.4 — Diferença chave face a P188B

| | P188B (C2 Equation) | P194B (C4 resolved label) |
|---|---|---|
| Estado | **Permanente** dormente | **Temporário** dormente |
| Razão | `SetEquationNumbering` ausente | Walks E2/E4 não migrados |
| Activação | Passo dedicado SetEquationNumbering | P195 + P196 (sequência §9 P189) |
| Documentação | 4 pontos obrigatórios | Comentário inline curto + secção L0 |

### P194A §11.5 — Forma Opção C idiomática

`Option<&str>` propagado através da chain (sem clone
intermediário). Único `to_string()` no `Some` arm.
Idiomático em Rust — preferível a Opções A/B que
introduziam clones em ambos os braços ou usavam
`unwrap_or_else` quando `or_else` puro funciona.

### P194B `.D.2` — Test caso central produção valida fallback legacy

Test `c4_resolved_label_via_fallback_legacy_caso_atual`
exercita pipeline real:
- Walk legacy popula `state.resolved_labels` via arm
  Labelled (E4 P189B excepção).
- Re-walk em `layout()` (P181H) popula Introspector mas
  sub-store `resolved_labels` permanece vazio (E2/E4
  ainda mutam legacy).
- Consumer C4 (`references.rs:53`) consulta Introspector
  → `None` → `or_else` cai em legacy → output "Secção 1".

Confirma empiricamente que cenário de produção real é
coberto pelo fallback legacy.

---

## §5 Estado temporário (secção dedicada — paralela a §5 P188)

### Diferença observable

P194 introduz Introspector path em `layout_ref:53` mas
**Introspector está vazio em produção**:
- `intr.resolved_labels` field aberto em P193B (struct
  + método trait).
- **Walks Labelled/Heading não modificados** — continuam
  a mutar `state.resolved_labels` directamente (E2/E4
  excepções P189B).
- `from_tags` não tem arm para popular sub-store
  resolved_labels via Tag (P195 adiciona).

Resultado: `intr.resolved_label_for(target)` retorna
sempre `None` em produção; `or_else` cai em legacy
consistentemente.

### Razão

Walk arms Labelled (`introspect.rs:Content::Labelled`) e
Heading (`introspect.rs:Content::Heading`) continuam a
popular `state.resolved_labels` legacy. Para activar
caminho Introspector, walks precisam:
1. Emitir Tag para Labelled (passo 3 da sequência §9
   P189 — P195).
2. Heading auto-toc gerar Tag também (passo 4 — P196).
3. `from_tags` arm popular `intr.resolved_labels` via
   Tag.

Até esses passos, sub-store fica vazio.

### Activação após P195 + P196

Após P195 + P196 sequenciais:
- Walk arms emitem Tag em vez de mutar state.
- `from_tags` arm popula `intr.resolved_labels`.
- Consumer C4 (este — P194) recebe `Some(text)` do
  Introspector → caminho Introspector activa.
- Fallback legacy redundante (mas mantido durante janela
  compat M5).

### Janela compat M6

Após excepções E2-E6 (P189B) todas fecharem:
- `CounterStateLegacy.resolved_labels` removível.
- Copy-sites `mod.rs:1481, 1512` removíveis.
- Fallback legacy em `references.rs:53-57` removível.
- Forma final: apenas Introspector path.

### Diferença vs P188B (Equation)

P194 é **temporário**; P188B é **permanente**:
- P188B: `SetEquationNumbering` não existe; sem como
  popular state → gate em `from_tags` arm Equation
  (P186E) sempre dorme.
- P194: walks Labelled/Heading **existem**; só falta
  migrá-los (P195/P196). É trabalho identificado, não
  bloqueador estrutural.

Documentação P194 mais leve que P188B em consequência —
comentário inline curto + secção L0 + secção §5 do
consolidado, sem 4 pontos obrigatórios formais.

---

## §6 Estado final M9 e M5

### M9 (counter-feature) — inalterado: 11/11

P194 não introduz feature M9 nova.

### M5 — incremental (inalterado em arms migrados)

Cadeia de pré-requisitos para fechar excepções E2-E6:

**Antes de P194**: 3 pré-requisitos pendentes.
**Após P194B**: **2 pré-requisitos** restantes:
1. ~~Sub-store `resolved_labels`~~ ✅ P193B.
2. ~~C4 migration (consumer Ref-arm)~~ ✅ P194B.
3. Sub-store `headings_for_toc` — passo dedicado.
4. `Content::SetEquationNumbering` — passo independente.

**Excepções E2-E6 continuam activas** — P194 desbloqueia
consumer mas activação completa requer walks Labelled/Heading
migrados (P195/P196).

### Trait `Introspector` — 19 métodos (inalterado)

### `TagIntrospector` sub-stores — 8 (inalterado)

### M5/M4 (read-sites) — interpretação

Per P189A §11.4-§11.6, C4 era originalmente contado como
parte de M4-residual mas ficou pendente em P183E não
corrido. Este relatório adopta a contagem 8/12 + 1 (C4
migrado em P194B) = **9/12 read-sites** num sentido
ampliado, ou 8/12 inalterado se contado estritamente
M4-residual original.

Para evitar confusão, próximos passos contam apenas
M5/M4 estrito = 8/12; M5 work é trabalho separado.

---

## §7 Estado final lacunas

- **Lacuna #3** (`headings_for_toc` sub-store): activa
  ainda. Independente de P194.
- Outras lacunas: inalteradas.

---

## §8 Pendências cumulativas + DEBT M5-residual

### DEBT M5-residual (Cenário B)

**Sem DEBT formal aberto**. Nota actualizada per P193 §7:

> Antes P194: 3 pré-requisitos pendentes para fechar
> cadeia E2-E6.
>
> **Após P194B: 2 pré-requisitos restantes** (1 dos 3
> avançado).
> 1. ~~Sub-store `resolved_labels`~~ ✅ P193B.
> 2. ~~C4 migration (consumer Ref-arm)~~ ✅ P194B.
> 3. Sub-store `headings_for_toc` — passo dedicado.
> 4. `Content::SetEquationNumbering` — passo independente.
>
> **Excepções E2-E6 continuam activas** — P195+ destranca.
> P194 desbloqueia consumer; populate via Tag activa em
> P195+.

DEBT M5-residual continua em Cenário B.

---

## §9 Próximos passos sugeridos

### Sequência continua (per P189 §9)

1. **P195 — migrar walk arm Labelled**: emite Tag em vez
   de mutar `state.resolved_labels` directamente.
   `from_tags` arm popula `intr.resolved_labels`. Magnitude
   **S–M** (depende de decisão arquitectural sobre payload —
   pode exigir nova variant `ElementPayload::Labelled`
   similar a P186 Equation, ou pode aproveitar mecanismo
   P171 StateUpdate). **E4 fecha**.

2. **P196** — migrar walk arm `Heading` (auto-toc gera
   labels que precisam de chegar ao sub-store). **E2
   fecha residual**.

3. **P197** — migrar walk arm `Figure`. **E3 fecha**.

4. **P198** — migrar walks `SetHeadingNumbering` +
   `CounterUpdate`. **E5 + E6 fecham**.

5. (Independente) — `Content::SetEquationNumbering`
   materialização. **E1 fecha**.

Após sequência: M5 universalmente fechado; segue M6
(eliminação `CounterStateLegacy`).

### Independente

- Sub-store `headings_for_toc` (lacuna #3) — passo
  dedicado paralelo.

---

## §10 Conclusão

P194 fechou em 2 sub-passos (A diagnóstico + B
implementação agregada) com magnitude correctamente
estimada (S em ambos). Replicação literal de padrão
P184D/P187B/P188B com variante Opção C idiomática.

Achados centrais:
- **Replica padrão substitution-with-fallback** com
  primitiva location-aware-equivalente
  (`resolved_label_for` retorna `Option<&str>` sem
  Location parameter).
- **Estado temporário (não permanente)** distingue P194
  de P188B. Documentação mais leve.
- **Output observable preservado por construção** —
  sub-store vazio; fallback consistente.
- **2 dos 4 pré-requisitos cumpridos** após P193B + P194B
  para fechar cadeia E2-E6.

P194 é **passo 2 dos 7** da sequência §9 P189 consolidado.
Próximo passo desbloqueado: **P195** (walk arm Labelled
migration).

**64 passos executados** após P194B. Padrão
diagnóstico-primeiro mantido — 16/16 acertaram a
magnitude planeada ±1 nível
(P131A/132A/140A/148/154A/181A/182A/183A/184A/185A/186A/
187A/188A/189A/193A/194A).
