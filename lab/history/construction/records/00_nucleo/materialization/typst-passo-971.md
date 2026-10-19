# Passo 971 — limites de integral (`∫ᵇₐ`) não seguem a inclinação do símbolo

**Precede este passo**: achado 9.2 da auditoria externa (2026-08-05) — `∫` é desenhado inclinado;
no vanilla, o limite inferior ("a") acompanha a inclinação (6.1pt à direita do símbolo), o limite
superior ("b") fica mais deslocado ainda (11.0pt) — colunas diferentes, seguindo o ângulo do
traço. No cristalino, "a" e "b" ficam ambos a 11.0pt — mesma coluna, ignorando a inclinação.
Distância vertical (acima/abaixo) já bate nos dois (~9.5pt/~14.3pt) — só o deslocamento horizontal
do limite inferior está errado.

**Pré-condição de árvore**: `git status`. Confirmar P970 (se já executado) presente.

---

## Fase A — confirmar a fórmula real do vanilla

1. Ler o código do vanilla para posicionamento horizontal de limites de operadores inclinados
   (`∫`, `∮`, e variantes) — candidato: termo de itálico/inclinação (`italics_correction`/`skew`) já
   mencionado en passant em achados anteriores desta frente (P892 tinha investigado e refutado
   itálico como causa de outro problema — confirmar se este é um mecanismo relacionado mas
   diferente).
2. Confirmar os valores reais na fonte (`fontTools`) para o glifo do `∫` — itálico correction ou
   termo equivalente que produziria os 6.1pt/11.0pt medidos pela auditoria.
3. Confirmar a implementação actual do cristalino (`attach.rs`, o braço de limites de operador) e
   onde o termo de inclinação está ausente.

## Fase B — Implementação (TDD directo se for termo pontual ausente)

1. Teste com os deslocamentos horizontais esperados para limite superior e inferior de `∫`,
   derivados da fórmula real e dos valores da fonte.
2. Implementar.
3. Suíte verde. Confirmar que a distância vertical (já correta) não regride.
4. `cargo run -- .` — zero violations.

## Fase C — Revalidação

`compare.py` nas seções com integral (4, 25, entre outras). Benchmark completo, 7 cenários,
`depois/antes`.

## Resultado esperado

- Limites de `∫`/`∮` deslocados horizontalmente conforme a inclinação do símbolo, como o vanilla.
- Distância vertical preservada.
- Benchmark sem regressão.
