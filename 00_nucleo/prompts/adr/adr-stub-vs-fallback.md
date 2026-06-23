# Prompt L0 — ADR: Stub Transparente vs. Fallback Software

**Camada**: ADR (arquitetural, transversal a L1–L4)
**Ficheiro alvo**: `00_nucleo/adr/typst-adr-0113-stub-transparente-vs-fallback-software.md`
**ADRs relevantes**: ADR-0054 (graded — aproximações aceitáveis com ressalva),
ADR-0107 (paridade linguagem), ADR-0108 (medir-antes-de-decidir),
ADR-0093 (metodologia de evolução de ADRs).

## Contexto

O princípio da **honestidade epistêmica** — "não fingir que se sabe fazer
algo que não se sabe fazer bem" — já opera no cristalino em vários passos
históricos, mas carece de formalização arquitetural canônica. Sem ADR, cada
passo futuro que enfrente a mesma escolha (stub vs fallback) terá de
re-discutir o critério do zero.

## Decisão

> **Quando um consumer requer infraestrutura ausente** (shaping, runtime
> layout, CSL parser, variant-aware font selection, gradient render, etc.),
> **prefere-se stub transparente** — o elemento existe no pipeline completo
> (parse → eval → Content → walk → layout), mas o consumer emite o body
> inalterado ou vazio. **Nunca fallback software** a menos que o fallback
> seja bit-exact com vanilla ou explicitamente justificado como ADR-0054
> graded com ressalva documentada.

**Razão**: "quase funciona" é mais difícil de remover do que adicionar. Stub
permite evolução por adição pura; fallback exige refactor destrutivo no
futuro.

## Exceções

1. **Fallback bit-exact com vanilla** — ex.: `lorem` é helper puro, não
   fallback; resulta no mesmo valor que vanilla independentemente da infra.
2. **ADR-0054 graded com ressalva documentada** — aproximação aceitável com
   plano de remoção ou ativação futura.

## Passos históricos que aplicam este princípio

| Passo | Elemento | Infra ausente | Comportamento do stub |
|-------|----------|---------------|-----------------------|
| P295 | `footnote` | Layout-time two-pass | Emite `[N]` marker apenas |
| P408 | `smallcaps` | OpenType shaping (rustybuzz) | Emite `body` inalterado |
| P414 | `text.font` dict | Variant-aware font selection | Parseia campos mas não ativa selection |
| P157B | `table.cell` | Placement algorítmico | Armazena `colspan`/`rowspan` mas ignora em layout |
| P224.B | `GridHeader`/`GridFooter` | Multi-region flow | Armazena `repeat` mas ignora em layout |
| P223 | `place` | Float consumer geometric | Armazena `float`/`clearance` mas ignora |
| P156G | `block` | Breakable layout | Armazena `breakable` mas ignora até consumer real |
| P231 | `block`/`box` | Radius/clip infrastructure | Armazena `radius`/`clip`/`outset` mas ignora |

Lista não exaustiva; novos passos podem ser adicionados via PR.

## Anti-padrões

- Não implementar "quase funciona" sem documentar como scope-out.
- Não criar fallback visual (uppercase+scale, fake small caps, etc.) sem
  ADR-0054 graded explícito.
- Não deixar o usuário achar que a feature está completa quando é stub.

## Consequências

- **Positiva**: evolução por adição pura; nenhum refactor destrutivo no
  futuro.
- **Positiva**: honestidade epistêmica — o sistema não engana o usuário.
- **Negativa**: valor visual imediato pode ser zero para features stubbed.
- **Negativa**: testes E2E de features stubbed precisam ser escritos com
  expectativas ajustadas (body inalterado, não erro).

## Critérios de Verificação

```
Dado um passo futuro que necessite infraestrutura ausente
Quando o autor consulta ADR-0113
Então a decisão default é "stub transparente"
E fallback só é aceite se bit-exact ou ADR-0054 graded
```
