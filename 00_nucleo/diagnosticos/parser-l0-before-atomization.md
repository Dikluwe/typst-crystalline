# Prompt L0 — motor `Parser`
Hash do Código: 48d5eded

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/parse/parser.rs`
**ADRs:** ADR-0037, ADR-0107, ADR-0108, ADR-0129.

## Medição anterior à decisão

Este consumer é dono de `Parser`, token/lookahead, newline modes, markers,
checkpoints, memoização, profundidade, wrapping, recovery e ponte ao lexer. Os
campos `pub(super)` são usados pelas gramáticas e não constituem API pública.

## Contrato

- Manter um token corrente fora de `nodes`, com trivia e offsets preservados.
- `eat` avança; `marker` identifica início contíguo; `wrap` cria CST sem perder
  texto; erros esperados/inesperados preservam progresso.
- `enter_modes`/`with_nl_mode` restauram lexer e newline mode ao sair.
- Checkpoint/restore e memoização evitam reparsing exponencial.
- Profundidade máxima é 256 e falha de balanceamento permanece registrada.

## Aceitação

Texto integral, spans, trivia, recovery, modes e árvore permanecem observáveis
como antes; nenhuma mudança pública é introduzida.
