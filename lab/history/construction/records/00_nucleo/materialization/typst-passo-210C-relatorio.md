# Relatório do passo P210C

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-210C.md`.
**Tipo**: encerramento série documental puro.
**Magnitude planeada**: S (~20-30min). **Magnitude real**: S (~25min).
**Marco**: M9c (encerramento série P210; M9c continua com
P211+P212).

---

## §1 O que foi feito

Encerramento da série P210 (3 sub-passos materializados:
A diagnóstico + B counter_step + C encerramento). Trabalho
**100% documental** — P210A C3 já fixou Caminho 3 subset;
P210B materializou; P210C apenas formaliza encerramento +
adiciona bloco "Agregado série P210" em ADR-0076 e marca
§3.0septies em blueprint. Distinto de P207E/P208D/P209E:
sem decisões C1 a tomar (Caminho 3 já fixado em P210A).

---

## §2 Forma do encerramento — documental puro

P210C **não tem decisões C1 a tomar**. Distinção face a
encerramentos anteriores:

| Encerramento | Decisões C1 | Conteúdo |
|--------------|-------------|----------|
| P207E | Caminho 1 vs 2 (captura page-meta deferred) | ADR + blueprint + relatório |
| P208D | Caminho 1 vs 2 (`Content::Context` block deferred) | ADR + blueprint + relatório |
| P209E | C1.1 ADR-0077 transição + C1.2 Caminho 1 vs 2 | ADR-0077 → ACEITE + ADR-0076 + blueprint + relatório |
| **P210C** | **Nenhuma** — Caminho 3 fixado em P210A C3; P210B implementou | ADR-0076 + blueprint + relatório |

P210C limita-se a:
- Anotar encerramento série em ADR-0076 (+bloco agregado +
  deferreds documentados).
- Adicionar marca blueprint §3.0septies (7ª marca cirúrgica
  cumulativa).
- Verificar tests/lint preservados.
- Escrever relatório.

---

## §3 Alterações documentais

**Zero código tocado.**

| Ficheiro | Edição |
|----------|--------|
| `00_nucleo/adr/typst-adr-0076-introspector-completion.md` | §Plano de materialização: série P210 transita "EM CURSO" → "✅ MATERIALIZADO 2026-05-12"; P210C anotado como documental puro; bloco "Agregado série P210" adicionado com sumário 3 sub-passos + métricas + deferreds explícitos com critério de reabertura + pattern emergente "Caminho 3 honest subset" 8ª aplicação + distinção qualitativa stdlib funcs M9c. |
| `00_nucleo/diagnosticos/blueprint-projecto.md` | §3.0septies Marca de actualização adicionada (paralelo a §3.0sexies [P209E]): regista série P210 fechada + Caminho 3 honest subset + deferreds + estado pós-P210 (4 séries fechadas; restam P211+P212). |
| `00_nucleo/materialization/typst-passo-210C-relatorio.md` | Este ficheiro (novo). |

`crystalline-lint --fix-hashes`: "Nothing to fix" (L0
preservados; hashes intactos).

---

## §4 Decisões substantivas

- **Sem decisões materiais**: P210C é puramente
  formalização. Pattern "Caminho 3 honest subset" foi
  fixado em P210A C3 e implementado em P210B.
- **Critério de reabertura para deferreds**: registado
  explícitamente no bloco agregado P210 da ADR-0076.
  `counter.display`/`state.get` ficam adiados com gatilho:
  "quando walk advance for implementado pós-M9c, OU quando
  consumer real emergir". Sem critério, deferreds ficariam
  sem trigger; com critério, sub-passo dedicado tem entrada
  óbvia.
- **Marca blueprint §3.0septies**: 7ª marca cirúrgica
  cumulativa (P204H §3.0, P205E §3.0bis, P206E §3.0ter,
  P207E §3.0quater, P208D §3.0quinquies, P209E §3.0sexies,
  **P210C §3.0septies**). Pattern formaliza-se: cada
  encerramento série acumula marca; reescrita ampla do
  blueprint mantém-se fora-de-escopo M9c.
- **Sem ADR nova**: trabalho P210 inteiro coberto por
  ADR-0076 (Bloco V — Counter/State extras minimal).

---

## §5 Métricas

| Métrica | Antes (P210B) | Depois (P210C) | Δ |
|---------|---------------|----------------|---|
| Trait `Introspector` métodos | 26 | 26 | 0 |
| Stdlib funcs registadas | ~53 | ~53 | 0 |
| Tests workspace | 1939 | 1939 | 0 |
| `crystalline-lint` violations | 0 | 0 | 0 |
| L0 prompts modificados (em P210C) | — | 0 | — |
| L1 ficheiros modificados (em P210C) | — | 0 | — |
| Documentação modificada (em P210C) | — | 3 | +3 |

**Agregado série P210** (A diagnóstico + B counter_step + C
encerramento):

| Métrica | Pré-P210 | Pós-P210C | Δ série |
|---------|----------|-----------|---------|
| Stdlib funcs registadas | ~52 | ~53 | +1 (`counter_step`) |
| Tests workspace | 1935 | 1939 | +4 |
| L0 prompts novos | — | 0 | 0 |
| L0 prompts modificados | — | 0 | 0 |
| L1 ficheiros modificados | — | 3 (em P210B) | +3 |
| Sub-store novos | — | 0 (reusa pre-existentes) | 0 |
| ADRs novas | — | 0 (sob ADR-0076) | 0 |
| Caminho 3 honest subset aplicações | 0 | 1 (P210 inteira) | +1 |
| Deferreds documentados com critério | — | 2 (`counter.display`, `state.get`) | +2 |

---

## §6 Encerramento série P210 — sumário literal

Série P210 fechou em 3 sub-passos (A + B + C). Pattern
emergente do projecto (P204A-H, P205A-E, P206A-E, P207A-E,
P208A-D, P209A-E) replicado em forma compacta: diagnóstico-primeiro
reduzido (A) → materialização subset (B) → encerramento
documental (C). Sem P210D/E necessárias.

| Sub-passo | Tipo | Magnitude real | Output principal |
|-----------|------|---------------|------------------|
| P210A | Diagnóstico-primeiro reduzido | S (~30min) | Auditoria A1-A5 + decisões C1-C5 + plano P210B-C. Caminho 3 subset fixado: só `counter.step` materializável; `counter.display`/`state.get` deferred. Relatório `00_nucleo/diagnosticos/typst-passo-210A-relatorio.md`. |
| P210B | Materializar `counter_step` | S (~30min) | `native_counter_step(key)` stdlib (~40L em `foundations.rs`); scope register em `eval/mod.rs`; 4 tests em `stdlib/mod.rs`. Sem L0 novo; sem trait extension. Reusa `Content::CounterUpdate` + `CounterAction::Step` pre-existentes. |
| P210C | Encerramento série | S documental (~25min) | ADR-0076 anotada com agregado P210 + deferreds com critério; blueprint §3.0septies marca; relatório este. Zero código tocado. |

**Custo agregado real**: ~1.5h (estimado ~1.5-2h per P210A
C5). Magnitude **S-M** confirmada empíricamente — abaixo
do estimado L original em P207A para Bloco V completo (Q1=β
escopo full). Caminho 3 honest subset reduziu o custo.

**Padrões emergentes consolidados em P210**:

1. **Caminho 3 honest subset** (pattern novo, distinto):
   materializa parte do escopo Q1=β; defere parte com critério
   explícito de reabertura. P210 é a 1ª aplicação completa;
   anti-inflação cumulativa 8ª aplicação.
2. **Distinção qualitativa stdlib funcs M9c**:
   - Sem `current_location` dependência: materializáveis
     trivialmente (`counter_step`, `query`, `locate`, `here`).
   - Com `current_location` dependência funcional: deferred
     até walk advance estar implementado (`counter.display`,
     `state.get`).
3. **Convenção L0 stdlib funcs P169+ inline-documentadas**:
   reusada em P210B (4ª aplicação consolidada — P208B, P208C,
   P209D C6, P210B). Sem L0 prompt novo em toda a série
   P210.

---

## §7 Próximo sub-passo

**P211 série** — Outline configurável. Aguarda spec próprio
para diagnóstico-primeiro (P211A) que decidirá Caminho 1/2/3
empíricamente.

Antecipações baseadas em padrão M9c (zero consumers reais
+ outline `()` cristalino actual já cobre needs M5+P200):
- **Caminho 1 puro** provável (skip total se outline existente
  cobre needs M9c sem configuração adicional).
- **Caminho 3 subset** alternativa (materializar 1-2 features
  triviais; deferir outras).

**P212** — encerramento M9c (auditoria 7 condições ADR-0076
§Plano de validação + transição PROPOSTO → ACEITE).
Magnitude S documental.

**Estado actual M9c**: 4 séries fechadas (P207 + P208 + P209
+ P210). Restam P211 + P212. ADR-0076 mantém `PROPOSTO`.
ADR-0077 já transitou ACEITE (P209E).
