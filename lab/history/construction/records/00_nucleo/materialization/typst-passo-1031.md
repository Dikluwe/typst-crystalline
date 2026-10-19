# Passo 1031 — Bloco 3 (P1021): 68 achados graves fora da família math

**Tipo**: Mesmo processo do Passo 1029, aplicado ao resto do catálogo. Investigar cada
achado → citar/corrigir L0, ou classificar como decisão interna, ou escalar para passo de
gate próprio se revelar bug de código.
**Base**: catálogo do Passo 1021, os 68 achados graves de Bloco 3 fora da família math
(lista em `temp/p1024/bloco3_graves.md`, secção "68 restantes").
**Pode correr em paralelo com o Passo 1030** — não toca `eval/rules.rs` nem prompts de
`compiler/math/`/`entities/elements/math_*`; se algum achado desta lista cair
inesperadamente numa dessas áreas, sinalizar e não tocar, deixar para depois do 1030
fechar.
**Pré-condição**: `git status` limpo.

---

## Lições obrigatórias (herdadas de P1026/P1025/P1029, não repetir a descoberta)

1. **Antes de reler o vanilla, `grep` o mecanismo em `tests.rs`.** Se houver guarda com
   `file:line` do vanilla já citado, a resposta pode já lá estar.
2. **Confirmar o binário de referência correcto antes de medir** — vanilla ratificado
   (`--version`, hash confirmado), não presumir 0.15.0 nem aceitar `/usr/local/bin/typst`
   sem confirmar a versão.
3. **Ao medir um mecanismo do vanilla, varrer o pipeline inteiro**, não parar no primeiro
   call site plausível — erro já cometido uma vez (P1024, `resolve_skewed_frac` em vez do
   construto certo).
4. **Distinguir citação literal de citação contextual** (lição do P1029, Exemplo C): se a
   fonte só sustenta a intenção geral e quem prova o comportamento composto são os
   testes, dizer isso explicitamente no L0, não apresentar como se fosse a mesma força de
   prova de uma citação literal.

## Processo, por achado (igual ao P1029)

1. `grep` de guarda em `tests.rs` primeiro.
2. Se não houver guarda: verificar contra corpus de documentação existente para a área
   (se existir — só `math/` tem corpus dedicado por agora; para as outras áreas, ir
   directamente à documentação oficial `typst.app/docs/`).
3. Se nem a documentação cobrir: medição directa do vanilla, pipeline completo.
4. Classificar: citação em falta (adicionar) / L0 errado (corrigir) / código errado
   (**escalar, não implementar aqui**) / não verificável (decisão de implementação).

## Controlo de confound

Se algum achado envolver comparação visual/glifo, confirmar fontes de referência
idênticas nos dois binários antes de comparar píxeis (mesmo procedimento do P1026).

## Output

Tabela: achado, classificação, citação/`file:line`, achados a escalar (com prioridade
sugerida). Amostra de 3-4 citações mostrada lado a lado com a fonte (mesma exigência que
já se aplicou ao P1029 — não esperar que seja pedida outra vez).

## Validação

```
crystalline-lint .
cargo test --workspace
```
Zero regressão — só correcções de L0/citação; qualquer bug de código escala para passo
próprio.

---

## Resultado esperado

Os 68 achados fechados (citação, correcção, ou escalados com dono). Com isto, mais o
Passo 1029, o catálogo de graves do Bloco 3 fica processado por inteiro — restam só os
achados leves (58) e o Bloco 4 (referências a passo, tratamento à parte já decidido).
