# Passo 1046 — V16 Classe B: hubs de despacho com erro/fallback a jusante

**Tipo**: Classificar e decidir caso a caso, mesma disciplina do P1041 Fase A/B — nenhum
`_ =>` se resolve por suprimir o lint, decisão semântica por caso, com evidência.
**Base**: lista de 23 ocorrências (confirmar contagem exacta na Fase 0 — o relatório de
origem usou "22" e "22/23" em pontos diferentes, sem coincidir com a soma da própria
lista) em: `bindings/field_access.rs` (2), `bindings/method_dispatch.rs` (1),
`bindings/value_methods.rs` (2), `call_dispatch.rs` (1), `control_flow.rs` (2), `math.rs`
(5), `eval/mod.rs` (3), `operators/arithmetic.rs` (1), `operators/ordering.rs` (1),
`rules.rs` (2), `selector_matching.rs` (1), `stdlib/counter.rs` (1),
`stdlib/foundations/query.rs` (1).
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1045.

---

## Fase 0 — Confirmar a contagem exacta antes de começar

```
crystalline-lint --checks v16 . | grep -E 'field_access|method_dispatch|value_methods|call_dispatch|control_flow|math\.rs|eval/mod\.rs|arithmetic|ordering|rules\.rs|selector_matching|counter\.rs|query\.rs'
```
Confirmar se são 22 ou 23 (ou outro número) antes de dimensionar o resto do passo — mesma
disciplina que já se aplicou ao V16 remanescente no P1045.

## Fase A — Para cada caso, confirmar que é mesmo Classe B, não reclassificar às cegas

"Classe B" foi definida no P1045 como "hub com erro a jusante" — antes de decidir a
correcção, confirmar que a classificação está certa: o wildcard realmente desvia para um
diagnóstico tipado/erro formatado, ou é na verdade um neutro simples (Classe A) mal
classificado? Não presumir que a triagem anterior foi perfeita — é rápido reconfirmar,
caro presumir errado.

## Fase B — Decisão por caso, mesma taxonomia do P1041

Para cada um dos ~23 casos:
1. **(a)** o fallback para erro/diagnóstico a jusante já está correcto — anotar inline
   com a razão e, se aplicável, a mensagem de erro exacta que produz (mesmo padrão do
   P1043, item 4 — citar a mensagem literal, não parafrasear).
2. **(b)** devia ter tratamento próprio em vez de cair no fallback genérico — implementar.
   **Muda comportamento observável — gate `ADR-0127` antes de codificar.**
3. **(c)** o fallback devia ser um erro mais específico do que o actual — mesma exigência
   de gate.

Prestar atenção especial aos 5 casos de `math.rs` (delimitadores e escopos) — dado que já
encontrámos bugs reais em código de delimitador matemático nesta frente (P1026, P1042),
não presumir que estes 5 são triviais só por estarem na mesma categoria dos outros.

## Fase C — Validar

```
crystalline-lint --checks v16 .
cargo build --workspace --release
cargo test --workspace
```
Zero regressão nos casos (a). Qualquer (b)/(c) implementado só depois de gate aprovado,
com teste próprio antes/depois.

---

## Resultado esperado

~22-23 casos de Classe B decididos individualmente, com evidência e mensagens exactas
onde aplicável. Fica para depois: os 84 casos de Classe A (projecções neutras), próximo
passo natural após este.
