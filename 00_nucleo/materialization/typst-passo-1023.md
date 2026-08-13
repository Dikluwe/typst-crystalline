# Passo 1023 — Finalizar `flow`/`sectioning` (pós-auditoria retroactiva) + registar regra de método

**Tipo**: Continuação directa do achado da auditoria retroactiva (mensagem do dono,
2026-08-13) — os dois L0s (`structural/flow.md`, `structural/sectioning.md`) já foram
corrigidos para declarar a fronteira como não medida. Este passo mede o que falta e
materializa a divisão, dentro do que a hierarquia do método já autoriza (critério 4 como
decisor quando o critério 3 está em silêncio, não em contradição).
**Pré-condição**: `git status` limpo. Nada foi commitado pela auditoria retroactiva —
confirmar que os dois L0s corrigidos ainda estão como os deixaram, antes de prosseguir.

---

## Parte 1 — `flow`: dividir em 3, sem ambiguidade a resolver

Critério 3 mudo (sem núcleo real), critério 4 separa (`par.rs`, `quote.rs`,
`footnote.rs` no vanilla). Autorizado directamente — materializar:

- `structural/flow/par.rs` (ou nome equivalente) — `native_par`.
- `structural/flow/quote.rs` — `native_quote`.
- `structural/flow/footnote.rs` — `native_footnote` (confirmar se existe aqui ou se já
  está tratado noutro nível — `compiler/layout/footnote.rs` já existe do Passo 1020;
  confirmar que não há duplicação de responsabilidade entre o nível stdlib/construção e o
  nível layout/render antes de mover).

L0 de cada um, sem referência a passo, corte e cola (mesma disciplina de sempre).

## Parte 2 — `sectioning`: medir antes de decidir a forma final

1. Confirmar `outline`+`lof`+`lot` como núcleo (já medido, `2ca61c873`) — materializar
   como nó `sectioning/outline.rs` (ou equivalente).
2. Para `heading`, `title`, `divider`: medir co-mudança **entre os três**, excluindo
   qualquer commit que os ligue a `outline`/`lof`/`lot` (esses já confirmados artefacto).
   Usar a versão corrigida da ferramenta (a mesma que fez a auditoria retroactiva).
3. Resultado da medição decide a forma:
   - Se os três co-mudam entre si, de forma real (mesma barra de prova usada no resto do
     método — `git show` a confirmar inserção vs alteração de corpo): um nó
     `sectioning/misc.rs` (nome a decidir) com os três.
   - Se nenhum dos três liga a nenhum outro: três nós individuais.
   - Se só dois ligam entre si e o terceiro fica sozinho: dois nós.
4. Aplicar critério 4 (vanilla) só se o critério 3 ficar em silêncio total — não usar o
   vanilla para desfazer um cluster que a medição confirmar.

## Parte 3 — Registar a regra de método nova

Actualizar `00_nucleo/prompts/auditar-fatiamento.md`:

- Nova entrada na tabela de proveniência: **"Par de co-mudança sem mecanismo plausível é
  para verificar, não para explicar" — origem: auditoria retroactiva pós-P1022, achado de
  segunda ordem (bug na própria ferramenta de auditoria, detectado porque `heading`+
  `native_table_vline` não tinha explicação plausível).**
- Nota sobre a auditoria retroactiva em si: mesmo fronteiras já materializadas em código
  (não só L0s por escrever) devem ser reconfirmadas quando a ferramenta de medição for
  corrigida — não presumir que "já está commitado" torna a decisão imune a revisão.

## Fase de validação (as duas partes)

```
crystalline-lint .
cargo test --workspace
```
Zero regressão. Confirmar contagem de testes antes/depois, mesma disciplina de sempre.

---

## Resultado esperado

`flow` e `sectioning` com fronteiras finais medidas (não por inspecção nem por artefacto),
`auditar-fatiamento.md` com a regra nova registada com proveniência. Com isto, a auditoria
retroactiva fica totalmente resolvida — não só diagnosticada, fechada.
