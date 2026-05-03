# Relatório P183D — caso de bloqueio

**Data**: 2026-05-03
**Passo**: P183D — auditoria semântica explícita C3 (figure auto-number per kind)
**Resultado**: **bloqueado em `.B` por gate substancial em eixo 2**. Zero código tocado.

---

## §1 Resumo

P183D tentou migrar o consumer C3
(`01_core/src/rules/layout/mod.rs:435–439`,
`self.counter.figure_numbers.get(kind_key).and_then(|v| v.get(idx))`)
para `Introspector::figure_number_at_index(kind, idx)` com fallback.
A auditoria semântica `.B` aplicou literalmente a regra operacional dos
2 eixos (consolidada em P183C §6).

**Resultado por eixo** — descoberta nova: os dois eixos podem falhar
**independentemente**.

| Eixo | C1 (P183B) | C2 (P183C) | **C3 (P183D)** |
|------|-----------|-----------|---------------|
| 1 — semântica temporal | ❌ snapshot-during-walk | ❌ snapshot-during-walk | **✅ snapshot-final adequado** |
| 2 — dados em sub-store | ✅ arm Heading popula | ❌ sem arm Equation | **❌ chave global, não per-kind** |

**C3 é o primeiro consumer onde eixo 1 passa mas eixo 2 falha.** A
categoria "snapshot-during-walk" foi consolidada em P183B/P183C como
problema temporal, mas P183D mostra que o problema dual — dados
ausentes — pode bloquear independentemente.

Causa raíz eixo 2: `from_tags` arm `Figure`
(`01_core/src/rules/introspect/from_tags.rs:71–95`) usa chave
**global** `"figure"` no `CounterRegistry` em vez da convenção
`figure:{kind}` documentada em `element_payload.rs:52` mas nunca
implementada. O campo `kind: Option<String>` do payload é
silenciosamente ignorado via `..` pattern no destructure.

Achado bonus: a leitura legacy em `mod.rs:435–438` é **dead code em
produção** — `figure_numbers` legacy nunca é copiado ao Layouter,
logo `unwrap_or(idx + 1)` é o caminho real. Independente do bloqueio
P183D mas digno de simplificação separada.

Diagnóstico completo em
`00_nucleo/diagnosticos/diagnostico-p183d-bloqueio.md`.

---

## §2 Sub-passos executados

| Sub-passo | Estado | Notas |
|-----------|--------|-------|
| `.A` Auditoria L0 | ✅ | C3 confirmado em `mod.rs:435-439`; `figure_numbers: HashMap<String, Vec<usize>>` populado em walk; sub-store candidato `CounterRegistry` mas sem entries per-kind. |
| `.B.1` Eixo 2 | ❌ **gate substancial** | `from_tags` ignora `kind` field; chave única `"figure"` global. |
| `.B.2` Eixo 1 | ✅ conceptualmente OK (não testado em pipeline porque eixo 2 já bloqueia) | Figures pós-walk fixos; snapshot-final apropriado. |
| `.C`–`.F` | — não executados | gate em `.B` impede |
| `.G` Escalada | ✅ | Diagnóstico escrito |
| `.H` Verificação | n/a (caso bloqueio) | — |
| `.I` Encerramento | ✅ | Este relatório |

---

## §3 Confirmação de "tudo revertido"

`.A` e `.B` foram apenas leituras + análise estática (Read tool e
grep); zero edições em ficheiros de código de produção. Confirmação
via `git status --short`:

```
?? 00_nucleo/diagnosticos/diagnostico-p183c-bloqueio.md   (P183C)
?? 00_nucleo/diagnosticos/diagnostico-p183d-bloqueio.md   (P183D, novo)
?? 00_nucleo/materialization/typst-passo-183b.md          (passo)
?? 00_nucleo/materialization/typst-passo-183c-relatorio.md (P183C)
?? 00_nucleo/materialization/typst-passo-183c.md          (passo)
?? 00_nucleo/materialization/typst-passo-183d.md          (passo)
?? 00_nucleo/materialization/typst-passo-183d-relatorio.md (este ficheiro)
```

- Únicos ficheiros novos introduzidos por P183D em L0: o diagnóstico
  e este relatório.
- Zero ficheiros L1–L4 modificados.
- Zero L0 prompts (`00_nucleo/prompts/`) modificados.
- Zero linhas em ficheiros de código.

---

## §4 Estado pós-P183D

- **Tests workspace**: baseline P183A mantido — **1.756 verdes**; zero
  violations linter.
- **Hashes L0**: inalterados.
- **Trait `Introspector`**: 15 métodos (inalterado).
- **P183 série**:
  - `A` ✅ (diagnóstico)
  - `B` ❌ (bloqueado — C1 heading prefix; eixo 1)
  - `C` ❌ (bloqueado — C2 equation counter; eixos 1+2)
  - `D` ❌ (bloqueado — C3 figure auto-number; eixo 2)
  - `E` pendente (C4 resolved label; categoria diferente — identidade)
  - `F` pendente (fecho; abre DEBT M4-residual cobrindo C1+C2+C3)
- **Progresso M5/M4**: 5 read-sites migrados (P168 + P181G ×2 +
  P182D ×2). C1, C2 e C3 permanecem legacy até DEBT M4-residual ser
  tratado.
- **37 passos executados** (P183A + P183B + P183C + P183D = 4).

---

## §5 Próximo passo — P183E

C4 (resolved label, `references.rs:53`,
`layouter.counter.resolved_labels.get(target)`).

**Categoria diferente** dos consumers C1–C3:

- Labels são **identidades estáveis** — uma vez resolvidas durante o
  walk de introspect (concatenando heading prefix + título, ou nome
  de equação, etc.), o texto resolvido é **fixo** e não muda durante
  o layout.
- Eixo 1 (semântica temporal): snapshot-final é adequado por natureza.
- Eixo 2 (dados em sub-store): a esperar — depende se há sub-store
  para `resolved_labels` ou se o LabelRegistry actual basta.

P183E aplicará a mesma regra dos 2 eixos por rigor (regra é
universal, não condicional ao tipo de consumer). Mas a expectativa é
**provável sucesso** — se o sub-store existir, ambos os eixos passam
naturalmente.

---

## §6 Aprendizado consolidado pós-P183D

Refino da regra operacional (extensão a P183C §6):

> A regra dos 2 eixos é uma **conjunção**, e os eixos são **ortogonais**:
>
> - **Falha só em eixo 1** (C1): solução é Layouter location-aware +
>   método `*_at(key, location)` no trait. Cross-cutting M6+.
> - **Falha só em eixo 2** (C3): solução é refinar o arm relevante em
>   `from_tags` + adicionar método trait dedicado. Localizado,
>   tratável isoladamente.
> - **Falha em ambos** (C2): solução requer trabalho em ambas as
>   frentes — pré-requisito de eixo 2 antes de eixo 1.
>
> A natureza do bloqueio dita a estratégia de desbloqueio futura.

---

## §7 Composição do DEBT M4-residual (a abrir em P183F)

Sumário antecipado para P183F documentar:

| Consumer | Site | Eixo bloqueado | Custo de desbloqueio |
|----------|------|---------------|----------------------|
| C1 heading prefix | `mod.rs:310` | 1 | M6+ (location-aware) |
| C2 equation counter | `equation.rs:97` | 1 + 2 | M6+ (location-aware) + emissão Tag Equation + arm `from_tags` |
| C3 figure auto-number | `mod.rs:435–439` | 2 | localizado: arm `from_tags` + método trait `figure_number_at_index` |

C3 é o consumer **mais barato** para desbloquear individualmente
(não exige cross-cutting M6+ change). Pode ser trabalhado isoladamente
quando a prioridade for justificada — independentemente do progresso
M6+ que C1 e C2 esperam.
