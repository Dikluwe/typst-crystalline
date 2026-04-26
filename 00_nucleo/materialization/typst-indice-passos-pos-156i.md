# Índice — passos pós-P156I

Sequência decidida na sessão pós-P156I: opção 1 (repeat) +
opção 5 (ADR meta) das 7 direcções candidatas no documento de
estado.

## Ordem de execução

| Ordem | Passo | Tamanho | Natureza | Target |
|------:|-------|:-------:|----------|--------|
| 1 | **P156J** | M | Código (variant + stdlib) | Layout 78% (14/18) |
| 2 | **P156K-meta** | S+ | Documental (2 ADRs) | 63 ADRs total |

## Justificação da ordem

**P156J primeiro** porque:
- Mantém momentum granular (1 feature; +6%).
- Adiciona uma sexta evidência empírica de helper reusado
  (`extract_length` N=6) que reforça ADR Smart→Option em
  P156K-meta.
- Adiciona uma quinta evidência empírica de inventariar-primeiro
  se sub-passo .1 for executado, reforçando ADR
  inventariar-primeiro em P156K-meta.

**P156K-meta segundo** porque:
- ADRs meta beneficiam de evidência mais robusta (N=6, N=5
  vs N=5, N=4).
- Custo baixo; pode ser intercalado com outras direcções sem
  bloquear progresso de cobertura.

## Ficheiros

- `typst-passo-156j.md` — enunciado de P156J.
- `typst-passo-156k-meta.md` — enunciado de P156K-meta.
- `typst-indice-passos-pos-156i.md` — este ficheiro.

## Estado actual relevante (resumo)

- Layout: 72% (13/18) — alvo P156J: 78% (14/18).
- ADRs: 61 total — alvo P156K-meta: 63 (+2 EM VIGOR).
- Tests: 1296 — alvo P156J: ≥1314 (Δ baseline série +18 a +25).
- Variants Content: 51 → 52 após P156J.
- Stdlib funcs: 41 → 42 após P156J.
- DEBTs em aberto: 14 (sem alteração esperada).
- Hash actual `entities/content.rs`: `b9ca52c4` (P156I).

## Direcções pós-P156K-meta

Não decididas. Candidatas remanescentes do documento de estado:
- 2 (columns L+ via DEBT-56) — quebra granularidade.
- 3 (footnote area) — desbloqueia Model Fase 2.
- 4 (Model Fase 2 P157 — table foundations) — equilíbrio entre
  domínios.
- 6 (promover ADR-0061 a IMPLEMENTADO) — depende de decisão
  humana sobre Fase 3.
- 7 (administrativo: discrepância DEBTs / paridade /
  Introspection 17%).

Decisão fica para após reporte de conclusão de P156K-meta.

## Padrões aplicados nesta sessão

- Granularidade 1-2 features/passo: P156J usa 1 feature.
- Inventariar primeiro pré-decisão: ambos os passos têm
  sub-passo .1 dedicado.
- Reuso de template de enunciado: estrutura idêntica à série
  P156C-I (estado actual / natureza / decisões / sub-passos /
  verificação / critério / cenários / notas / pós-passo).
