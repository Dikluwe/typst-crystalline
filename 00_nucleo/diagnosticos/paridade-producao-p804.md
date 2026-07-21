# Relatório de Verificação — Passo 804: `visualize` — `#line(length: ...)` rejeitado (achado P798 #9)

**Data:** 2026-07-21
**Status:** Concluído com Sucesso
**Proveniência da Medição:**
- **Commit Base:** `0661aef91` (HEAD)
- **Working tree na sonda "antes":** P799–P803 (zonas não relacionadas)
- **Working tree na validação "depois":** P799–P804
- **Hora da Medição:** 2026-07-21 ~16:50 (-0300)
- **Relatório de materialização:** `00_nucleo/materialization/typst-passo-804-relatorio.md`

---

## 1. O Problema Relatado

Achado #9 de P798: `#line(length: 3cm)` — cristalino `error: argumento nomeado inesperado em line(): 'length'`; vanilla aceita.

## 2. Diagnóstico e Medição

Sonda com três fontes: `#line(length: 3cm)`, `#line(length: 3cm, angle: 30deg)`, `#line(length: 3cm, end: (1cm, 1cm))` — as três rejeitadas pelo cristalino, as três aceites pelo vanilla. **Medição que refutou a previsão do prompt**: o vanilla **não rejeita** `length`/`angle` combinados com `end` — a documentação no código vanilla (`crates/typst-library/src/visualize/line.rs`) diz "only respected if `end` is none", e a compilação confirma que são ignorados. Logo não há combinação inválida a validar.

Geometria vanilla (`typst-layout/src/shapes.rs::layout_line`): sem `end`, `delta = (cos(angle)·length, sin(angle)·length)`; defaults `length: 30pt`, `angle: 0deg`. Cristalino (`native_line`): whitelist `["dx","dy","stroke","start","end"]` — scope-out de P739B.

## 3. A Solução Implementada

L0 `stdlib/shapes.md` (secção `native_line` reescrita com `length?`/`angle?`, semântica medida, scope-outs: `Ratio` em `length` sem região no native; `start` ≠ 0); hash corrigido (`shapes.rs` → `252a9a79`). Novo braço sem `end`: `dx = cos(angle)·length`, `dy = sin(angle)·length`; erro se combinado com `dx`/`dy` legado; com `end`, `length`/`angle` **ignorados** (paridade medida).

Validação por geometria (`mutool trace`, deltas moveto→lineto):

| Fonte | Cristalino | Vanilla |
|---|---|---|
| `length: 3cm` | (85.038, 0) | (85.039, 0) ✓ |
| `length: 3cm, angle: 30deg` | (73.645, 42.519) | (73.646, 42.520) ✓ |
| `length: 3cm, end: (1cm,1cm)` | (28.346, 28.346) — length ignorado | (28.346, 28.346) ✓ |

## 4. Testes Automatizados Persistidos (com nomeação explícita)

Os 4 falharam antes da implementação:
- `p804_line_length_sozinho` — `length: 3cm` → dx=85.04, dy=0.
- `p804_line_length_com_angle` — `length: 4cm, angle: 90deg` → dx≈0, dy=113.39.
- `p804_line_length_ignorado_com_end` — `length` + `end` → delta do `end`.
- `p804_line_length_nao_combinavel_com_dx` — `length` + `dx` → erro.

## 5. Verificação de Sucesso do Workspace

```
Suite 'typst-core' (lib):  ANTES 4324 passed; 1 ignored → DEPOIS 4328 passed; 1 ignored (total 4329 = +4 ✓)
crystalline-lint . → exit 0
```

Verificação final do workspace completo (2026-07-21 ~18:00): typst-core 4336/1i, typst-infra 657/5i, typst-shell 33, CLI bin 2, cli.rs 29, crystalline_lint 2 — zero falhas.
