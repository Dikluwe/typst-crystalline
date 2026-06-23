# ADR-0113 — Stub Transparente vs. Fallback Software

**Status**: IMPLEMENTADO  
**Data**: 2026-06-22  
**Decisão**: Stub transparente preferido sobre fallback software quando infraestrutura está ausente.  
**Decisor**: Passo 415 (administrativo-documental).  

## 1. Contexto

O princípio da **honestidade epistêmica** — "não fingir que se sabe fazer algo que não se sabe fazer bem" — já opera no cristalino desde vários passos históricos, mas carecia de formalização arquitetural canônica. Sem ADR, cada passo futuro que enfrentasse a mesma escolha (stub vs fallback) teria de re-discutir o critério do zero.

## 2. Decisão

**Stub transparente é preferido.** Quando um consumer requer infraestrutura que ainda não existe (shaping, runtime layout, CSL parser, variant-aware font selection, gradient render, etc.), o elemento existe no pipeline completo (parse → eval → Content → walk → layout), mas o consumer emite o body inalterado ou vazio. Não finge funcionalidade que não existe.

**Fallback software é rejeitado** salvo duas exceções:
- (a) O fallback é **bit-exact com vanilla** (ex.: `lorem` é helper puro, não fallback).
- (b) O fallback é **ADR-0054 graded com ressalva documentada** — aproximação aceitável com plano de remoção ou ativação futura.

**Razão**: "quase funciona" é mais difícil de remover do que adicionar. Stub permite evolução por adição pura; fallback exige refactor destrutivo no futuro.

## 3. Consequências

- **Positiva**: evolução por adição pura; nenhum refactor destrutivo no futuro.
- **Positiva**: honestidade epistêmica — o sistema não engana o usuário.
- **Negativa**: valor visual imediato pode ser zero para features stubbed.
- **Negativa**: testes E2E de features stubbed precisam ser escritos com expectativas ajustadas (body inalterado, não erro).

## 4. Passos que aplicam este princípio

| Passo | Elemento | Infra ausente | Comportamento do stub |
|-------|----------|---------------|----------------------|
| P295 | `footnote` | Layout-time two-pass | Emite `[N]` marker apenas |
| P408 | `smallcaps` | OpenType shaping (rustybuzz) | Emite `body` inalterado |
| P414 | `text.font` dict | Variant-aware font selection | Parseia campos mas não ativa selection |
| P157B | `table.cell` | Placement algorítmico | Armazena `colspan`/`rowspan` mas ignora em layout |
| P224.B | `GridHeader`/`GridFooter` | Multi-region flow | Armazena `repeat` mas ignora em layout |
| P223 | `place` | Float consumer geometric | Armazena `float`/`clearance` mas ignora |
| P156G | `block` | Breakable layout | Armazena `breakable` mas ignora até consumer real |
| P231 | `block`/`box` | Radius/clip infrastructure | Armazena `radius`/`clip`/`outset` mas ignora |

Lista não exaustiva; novos passos podem ser adicionados via PR.

## 5. Anti-padrões

- **Não** implementar "quase funciona" sem documentar como scope-out.
- **Não** criar fallback visual (uppercase+scale, fake small caps, etc.) sem ADR-0054 graded explícito.
- **Não** deixar o usuário achar que a feature está completa quando é stub.

## 6. Referências

- ADR-0054 — Graded (aproximações aceitáveis com ressalva).
- P295 — `footnote` marker only.
- P408 — `smallcaps` stub transparente.
- P414 — `text.font` dict honestidade epistêmica.
