# Relatório de Paridade de Produção — P543

**Data:** 2026-07-03  
**Passo:** 543  
**Prompt L0:** `00_nucleo/prompts/infra/export-fixtures.md` (hash `9228792a`) para testes de shaper; `00_nucleo/prompts/rules/layout.md` (hash `9c9b7122`) para tipos de layout  
**Dependências:** P534 (fallback multi-script), P538e (fallback quando a fonte default não existe)

## Objectivo

Corrigir o mecanismo global de fallback de fonte por carácter (DEBT-65),
que duplicava texto quando o `FontBook` começava por fontes especializadas de
cobertura parcial (ex.: `"Hello"` → `"Helloello"`).

## Sonda

1. **`split_run_by_font`** (`03_infra/src/shaper.rs:408`): iterava caractere a
caractere e chamava `CandidateSet::covering(c)`, que devolvia a **primeira**
fonte (na ordem do `FontBook`) que cobria aquele caractere isolado.
2. **Causa da duplicação**: se `MathJax_AMS` cobria `'H'` mas não `'e'`, o
run era partido em `"H"` + `"ello"`. Cada sub-run carregava o campo `text`
com o texto **original** completo, e o export ToUnicode / `pdftotext`
duplicava o conteúdo.
3. **Decisão de estratégia**: a correcção correcta é escolher a fonte que
cobre o **maior trecho contíguo** a partir de cada posição, mantendo as
fontes primárias (declaradas ou lista fixa de P538e) com prioridade sobre o
fallback global do `FontBook`.

## Implementação

`03_infra/src/shaper.rs`:

- **`CandidateSet::covering_all`** — devolve todos os candidatos que cobrem
  um dado caractere, em ordem de prioridade.
- **`CandidateSet::best_covering_run`** — dado um conjunto de candidatos,
  escolhe o que cobre o maior trecho contíguo a partir de `start`.
- **`CandidateSet::covering_run`** — primeiro tenta as primárias; só recai no
  fallback global se nenhuma primária cobrir o primeiro caractere.
- **`split_run_by_font`** — reescrito para usar `covering_run` e fundir
  sub-runs consecutivos da mesma fonte. A divisão por script Unicode
  continua a ter prioridade.
- **`try_shape`** — o campo `text` de cada `FrameItem::TextShaped` passou a
  ser `subrun.text` (apenas o sub-run), e os `cluster`/`char_code` dos glifos
  são relativos a esse texto.

## Testes

### Teste unitário

`03_infra/src/shaper.rs` —
`p543_fallback_global_escolhe_maior_trecho_e_nao_duplica`:

- FontBook com `MathJax_AMS-Regular.otf` (cobre 'H', não cobre 'e'/'l'/'o')
  seguido de `DejaVuSans.ttf` (cobre "Hello").
- Fonte declarada: `"Helvetica"` (não existe), forçando fallback global.
- Verifica que o resultado é `"Hello"` (sem duplicação) e um único
  `TextShaped`.

### Testes existentes

- `p534_split_run_by_font_respects_script_boundaries` — passa.
- `p534_shape_mixed_script_system_fallback` — passa.
- `p534_latin_only_stays_single_textshaped` — passa.
- `p538e_fonte_default_ausente_usa_fallback_padrao` — passa.

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Resultado: todos os testes passam; linter limpo.

## Ficheiros alterados

- `03_infra/src/shaper.rs` — `covering_run`, `best_covering_run`,
  `covering_all`, `split_run_by_font`, `try_shape`, e teste P543.
- `00_nucleo/diagnosticos/debt/DEBT.md` — DEBT-65 marcado como fechado
  (P543).
- `00_nucleo/diagnosticos/paridade-producao-p543.md` — este relatório.

## Conclusão

P543 está concluído. O mecanismo global de fallback de fonte deixou de
fragmentar texto desnecessariamente e de duplicar conteúdo quando o
`FontBook` contém fontes especializadas de cobertura parcial. O caso comum
(uma das cinco fontes padrão existe) permanece sem regressão. DEBT-65 está
fechado.
