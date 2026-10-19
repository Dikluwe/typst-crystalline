# Relatório — Passo 969 (módulo oráculo de fórmulas do vanilla)

**Data:** 2026-08-05 · **Gate:** desenho de Fase A confirmado pelo dono em
2026-08-05 ("Continue" após `typst-passo-969-faseA.md`).
**Proveniência**: HEAD no início da Fase B = `7c6577ce9` (P972).

## Fase B — o que foi construído

- **L0**: `00_nucleo/prompts/testing/math_oracle.md` (o rascunho da Fase A,
  com um ajuste — ver abaixo).
- **Módulo**: `01_core/src/testing/math_oracle.rs` (+ `testing/mod.rs`),
  declarado `#[cfg(test)] pub(crate) mod testing;` em `lib.rs` — nunca
  compilado em produção. Funções puras sobre primitivos em pt, zero
  imports do motor. 4 funções do lote inicial:
  - `large_operator_upper_shift` / `large_operator_lower_shift`
    (`scripts.rs:290-313`, P959);
  - `grid_total_descent` (`table.rs:103-106`, P945);
  - `grid_axis_baseline` (`table.rs:188`, P919).
- **Ajuste ao rascunho confirmado**: `grid_cell_dy` foi **excluída** — a
  forma que o rascunho codificava (`baseline_offset + row_ascent −
  cell_ascent`) foi revogada por P952b (a forma vigente é trivial:
  `dy = baseline_offset`). Registado no próprio prompt.
- **Migração** (os testes existentes passam a referenciar o oráculo):
  - `p945_grid_total_descent_sem_dupla_contagem` — o valor esperado
    hardcoded (36.0) virou `grid_total_descent(&[(10,4)×3], 2.0)`.
  - Os 6 asserts de shift de `p959_tests` (4 testes + 2 asserts do teste
    combinado) — hardcoded (−10.9/−14.0/+9.2/+10.004) viraram chamadas
    `large_operator_upper_shift`/`large_operator_lower_shift` com os mesmos
    inputs.
- **Self-tests do oráculo**: 4 testes unitários (selecção do ramo do
  `max`, forma do `total_descent`, axis). Suite: **5733 testes, 0 falhas**
  (+4). Linter: 0 violations — o prompt novo é referenciado pelos 2
  ficheiros do módulo (sem órfão V7 novo; o V7 pré-existente de
  `package_version_resolution.md` permanece, alheio).

## Fase C — avaliação honesta da utilidade

O passo pedia usar o oráculo num dos achados 9.1–9.3 e avaliar se acelera.
Os três achados já estavam investigados (P970/971/972) antes do oráculo
existir — a avaliação possível é sobre o trabalho restante desses mesmos
achados (as Fases B de P970-parte-2 e P971, executadas a seguir):

- **A investigação não acelerou.** O custo de P970-parte-2/P971 foi a
  leitura do vanilla (`radical.rs`, `scripts.rs`), a medição da fonte
  (fontTools) e a refutação de proxies — trabalho que o oráculo não faz.
  Cada fórmula nova continua a exigir leitura extensa antes de poder ser
  portada — o cenário que o passo antecipou.
- **A consolidação acelera.** Os asserts migrados de P959/P945 deixam de
  ter aritmética solta em comentários (que já falhou uma vez: P952 teve de
  ser corrigido retroativamente) e passam a apontar para a fórmula com
  `file:line`. Novos testes de P970-parte-2/P971 vão usá-lo — o valor
  esperado deixa de ser um número mágico.
- **Risco registado**: o oráculo pode dar uma falsa sensação de verdade —
  ele espelha a nossa *leitura* do vanilla, não o vanilla. A disciplina
  que o mantém honesto é a regra 4 do prompt (sem `file:line`, não entra)
  + a medição end-to-end contra o PDF do vanilla, que continua a ser a
  prova decisiva (lição de P949/P972: medir no documento real).

**Veredicto**: ganho de organização e rastreabilidade, não de velocidade
de investigação. Útil como repositório de leituras confirmadas; não
substitui a leitura da fonte nem a medição end-to-end.

## Resultado

- Módulo oráculo com escopo contido (não é motor paralelo), criado e
  referenciado pelos testes migrados.
- Lote inicial de 4 fórmulas (P959 ×2, P945, P919) + avaliação honesta da
  utilidade.
- Suíte verde; linter limpo.
