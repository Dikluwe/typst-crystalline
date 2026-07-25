# Relatório — Passo 903: espaçamento ausente em torno de texto entre aspas em modo matemático

**Data:** 2026-07-25
**Commit de partida:** `86e8f28eb` (P902)

---

## Fase A — diagnóstico

### Ponto 1 — confirmar a hipótese de P897 (mecanismo tipo P891)

**Parcialmente certa, mas incompleta**: confirmado por leitura de código que `Content::Text`
(texto literal entre aspas — produzido por `value_to_display_content(Value::Str)`, distinto de
`Content::MathText`) caía no braço catch-all `_ => MathClass::Normal` de `base_math_class`
(`01_core/src/engine/math/layout/spacing.rs`), e que o par `(Alphabetic, Normal)` não tem regra
explícita em `spacing_between` — cai no catch-all `_ => 0.0`. **Mas simplesmente corrigir a classe
para `Alphabetic` não bastava**: o par `(Alphabetic, Alphabetic)` (ex.: identificadores adjacentes
`a`/`b`, que legitimamente NÃO têm espaço entre si — justaposição = multiplicação implícita)
**também** cai no mesmo catch-all `_ => 0.0`. Confirmado por leitura do vanilla
(`math/ir/item.rs::TextItem::create`, comentário "The resulting item is spaced and has alphabetic
math class") que o mecanismo real tem **dois componentes independentes**: a classe (`Alphabetic`,
para as regras de Punctuation/Relation/Binary/Large) **e** uma flag `spaced: bool` separada, que
força um espaço de texto normal quando nenhuma regra de classe explícita se aplica
(`math/ir/process.rs::spacing()`, ramo de prioridade mais baixa: `_ if (l.is_spaced() ||
r.is_spaced()) => return space`).

### Ponto 2 — classe actual vs classe correcta

Cristalino (antes): `Content::Text` → `MathClass::Normal` (via catch-all).
Vanilla: `Alphabetic` **+** flag `spaced: true` (dois mecanismos, não um).

### Ponto 3 — caso mínimo isolado

`$ a "texto" b $` reproduz o sintoma isoladamente, sem depender de numeração de equação ou
contexto adicional: cristalino `𝑎texto𝑏`, vanilla `𝑎 texto 𝑏`.

## Mecanismo vanilla completo (`process.rs::spacing()`, linhas 277-319)

O `match (l.rclass(), r.lclass())` resolve as regras de classe (Punctuation/Opening-Closing/
Relation/Binary/Large) **por ordem de prioridade**, incluindo os casos onde o resultado **é**
explicitamente 0.0 (ex.: antes de pontuação, depois de abertura). O fallback "item espaçado" só é
consultado quando **nenhuma** dessas regras corresponde ao par — não quando o resultado é
simplesmente 0.0. Esta distinção é crítica: `"texto",` (texto literal seguido de vírgula) não deve
ganhar espaço antes da vírgula só por o texto ser "spaced" — a regra de pontuação continua a
dominar.

## Fase B — Implementação

### Mudança 1 — classificação

`base_math_class`: novo braço `Content::Text(_) => MathClass::Alphabetic` (antes de `_`).

### Mudança 2 — distinguir "regra explícita = 0.0" de "nenhuma regra"

`spacing_between` (assinatura pública preservada, usada por 9 testes/chamadas pré-existentes)
passou a ser um wrapper fino sobre uma nova função privada `spacing_between_class(l, r, size) ->
Option<f64>` — `Some(v)` quando uma regra explícita corresponde (incluindo `Some(0.0)`), `None`
quando cai no catch-all. `spacing_between = spacing_between_class(...).unwrap_or(0.0)` preserva o
comportamento antigo bit-a-bit para todos os call sites que só querem o valor final.

### Mudança 3 — fallback de item espaçado em `compute_gaps`

`compute_gaps` ganhou um 4º parâmetro `text_space_pt: f64` (largura de um espaço de texto normal
no estilo/tamanho actual) e passou a rastrear, por nó, `is_text = matches!(node, Content::Text(_))`.
Quando `spacing_between_class` devolve `None` **e** um dos dois nós adjacentes é `Content::Text`,
usa `text_space_pt` em vez de `0.0` — replica a prioridade exacta do vanilla (regras explícitas
sempre ganham, incluindo as que "querem" 0.0).

`text_space_pt` é medido pelo caller (`layout_sequence`, `mod.rs`) via
`self.metrics.advance(" ", style.size, style).val()` — **não hardcoded**, consistente com a
disciplina do projecto de medir valores reais via `FontMetrics` em vez de aproximar por constante.
Medido via `mutool trace` num PDF real do vanilla: ≈3.65pt a 11pt — mesma ordem de grandeza já
registada em P825 (`spacing.md`, achado "spaced" nunca implementado antes deste passo).

### TDD

4 testes novos em `spacing.rs`:
- `texto_literal_entre_identificadores_recebe_espaco_dos_dois_lados` — o caso central do bug.
- `texto_literal_sozinho_nao_produz_gaps` — sem vizinhos, sem gaps.
- `texto_literal_e_alphabetic` — confirma a classificação.
- `texto_literal_antes_de_virgula_continua_sem_espaco` — confirma que a regra de Punctuation
  continua a dominar sobre o fallback (a distinção crítica do mecanismo).

Vermelho confirmado por reversão temporária em duas partes separadas (uma para cada mudança —
classe sozinha vs fallback de `compute_gaps` sozinho), confirmando que ambas as peças são
necessárias e cada uma tem cobertura própria:
- Só a classe revertida: `texto_literal_e_alphabetic` falha (`(Normal, Normal) != (Alphabetic,
  Alphabetic)`), os outros continuam a passar (confirma que o fallback funciona independentemente
  da classe).
- Só o fallback de `compute_gaps` revertido: `texto_literal_entre_identificadores_recebe_espaco_
  dos_dois_lados` falha (`[0.0, 0.0] != [4.2, 4.2]`).

### Suíte completa

```
typst-core:    4740 passed; 0 failed; 3 ignored
typst-infra:    734 passed; 0 failed; 5 ignored
typst-shell:     41 passed; 0 failed
```

Zero regressões (os 29 testes pré-existentes de `spacing.rs`/`compute_gaps` continuam a passar
inalterados, confirmando a preservação de assinatura/comportamento de `spacing_between`).

### `crystalline-lint`

`--fix-hashes .`: 1 ficheiro (`spacing.rs`, L0 `math/layout/spacing.md` actualizado com o
mecanismo P903, marcando o item `Content::Text` do scope-out de P825 como resolvido). 0 drift.

## Confirmação visual

Caso mínimo: `$ a "texto" b $` → `𝑎 texto 𝑏` — **byte-idêntico** ao vanilla.

Casos catalogados nas secções 24 e 30:
```
$ Q = rho A v + "time offset" $        → 𝑄 = 𝜌𝐴𝑣 + time offset       (igual)
$ f(x) = x^2 "for all" x in RR $        → 𝑓(𝑥) = 𝑥2 for all 𝑥 ∈ ℝ    (igual)
$ f(x) "sujeito a" g_i(x) <= 0 $        → 𝑓(𝑥) sujeito a 𝑔𝑖(𝑥) ≤ 0    (igual)
```

Todos os 3 casos catalogados agora produzem espaçamento idêntico ao vanilla à volta do texto
literal.

## Achados incidentais, registados fora de âmbito (não corrigidos)

1. **`RR | x` (fence/"mid")**: gap ≈0 entre `ℝ`/`|`/`x` no cristalino, vanilla usa ≈3.65pt dos dois
   lados. **Já registado como scope-out em P825** (`spacing.md`) — reconfirmado ainda presente
   após P903 (não corrigido por este passo, que se limitou ao caso `Content::Text` do mecanismo
   "spaced"). Reproduzível sem qualquer texto literal envolvido — mecanismo distinto (Fence class,
   não Text).
2. **`min_(x) f(x)`**: `min` (função com limite) seguido de `𝑓(𝑥)` sem espaço no cristalino
   (`min𝑓(𝑥)`), vanilla mostra `min 𝑓(𝑥)`. Confirmado reproduzível sem qualquer texto literal
   envolvido — mecanismo distinto, não investigado. Candidato a passo dedicado futuro.

## Benchmark (Fase C)

7 cenários, `hyperfine --warmup 5 -N -m 20`. Todas as leituras dentro da baseline estabelecida
(`04-math` 156.4ms, `01-hello` 95.1ms). Sem regressão.

## Fora de âmbito adicional

`.typ` de 30 secções: hash confirmado igual aos passos anteriores
(`9ae95a8d892103afc0c82c505a7f490a856acdad828577b7351013ad72691f29`), recompila sem regressão
(`exit=0`, `(1)`...`(44)` completo).
