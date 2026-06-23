# Relatório — Passo 415

**Título**: ADR-0113 "Stub Transparente vs Fallback Software" + Atualização Inventário Cobertura P391–P414  
**Data**: 2026-06-22  
**Tipo**: Administrativo-documental (zero código Rust; zero tipo novo; zero I/O).

## Resumo executivo

Formalizado o princípio emergente da **honestidade epistêmica** em ADR canônica
(ADR-0113), citando os passos históricos que já o aplicavam (P295, P408, P414,
P157B, P224.B, P223, P156G, P231). O inventário de cobertura vanilla vs
cristalino foi sincronizado com o cluster de materializações P391–P414,
refletindo o estado real do código.

## Ficheiros criados / alterados

| Ficheiro | Alteração |
|----------|-----------|
| `00_nucleo/adr/typst-adr-0113-stub-transparente-vs-fallback-software.md` | ADR canônica (NOVO) |
| `00_nucleo/prompts/adr/adr-stub-vs-fallback.md` | Prompt L0 da ADR (NOVO) |
| `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` | Atualização P415 no header; Tabela A.3 (`smallcaps`, `text.font` dict); Tabela A.8 (`bytes`, `lorem`); Tabela B.2 (`SmallCaps`); Tabela C (resolvidos); nota de ajuste ⁸⁷ no final |

## Decisões de engenharia

1. **ADR-0113** estabelece:
   - Stub transparente é preferido quando infraestrutura está ausente.
   - Fallback software só é aceite se bit-exact com vanilla ou ADR-0054 graded com ressalva.
   - Razão: evolução por adição pura vs refactor destrutivo; honestidade epistêmica.

2. **Inventário atualizado**:
   - `smallcaps`: `ausente` → `implementado` (stub transparente per ADR-0113).
   - `text.font` (dict): `scope-out` → `implementado` (P407 forma legado + P414 named fields; variant-aware selection continua scope-out).
   - `bytes(...)` e `lorem(n)` documentados em Foundations.
   - `SmallCaps` adicionado a Content variants.
   - `smallcaps` e `text.font` dict movidos para resolvidos na Tabela C.
   - Contagens: user-facing total 73/27/24/15/2=141 → **75/27/24/14/1=141**.
   - Cobertura user-facing factual (impl + impl⁺): **~72.3%** (nota: diferente da estima de ~81% no material do passo, que usava outro denominador/aproximação).

## Scope-out mantido

- Variant-aware font selection (variant/weight/style do dict `text.font`).
- Shaping OpenType real para `smallcaps`.
- ADR-0113 não altera código histórico — apenas cita.

## Validação

```bash
crystalline-lint .
# → 0 drift; 2 warnings de prompt órfão:
#   - show-regex.md (pré-existente)
#   - adr-stub-vs-fallback.md (novo; prompt de ADR puramente documental sem arquivo de código L1–L4 para referenciá-lo)
```

O warning do prompt `adr-stub-vs-fallback.md` é esperado porque o passo é
documental puro (zero código Rust). O prompt existe como artefacto L0 da ADR,
mas não há arquivo de produção que o referencie.

## Notas epistêmicas

- A sonda A.0 encontrou ADR-0054 (referência para graded) e confirmou ausência de ADR duplicada sobre stub/fallback.
- O inventário estava localizado em `00_nucleo/diagnosticos/` (não em `00_nucleo/` como sugerido no material do passo).
- Nenhum código de produção foi alterado neste passo.
