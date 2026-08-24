# Relatório do Passo 1139 — paridade dos diagnósticos públicos

**Medição final:** 2026-08-23T23:53:14.591043+00:00  
**HEAD:** `fcbc9763f8925d5c27b3670e35597b9adc0412a0`  
**Estado:** working tree não commitada  
**Vanilla ratificado:** `upstream/main a51e02804`

## Resultado

Os três casos que abriram P1139 passaram de `DIFF` a `MATCH` byte a byte:

- `P1138-S-001` — erro sintático com bloco fonte e caret;
- `P1138-S-002` — nome desconhecido, quoting e hint;
- `P1138-S-003` — erro em include com trace cross-file.

A matriz integral terminou com **15 `MATCH` e 4 `DIFF`**. Os doze `MATCH`
anteriores foram preservados. A lista integral ainda não-MATCH é:

- `P1138-X-001` — SVG;
- `P1138-L-001` — layout;
- `P1138-X-002` — raster/PNG;
- `P1138-X-003` — PDF.

Todos os 19 casos satisfizeram o estado esperado do manifesto. Reprodução:

```sh
python3 lab/parity/matrix/runner.py --output /tmp/p1139-matrix.json
```

## Validações

- 8 testes unitários P1139 em L2: aprovados;
- testes de mensagem `unknown_variable` em L1: aprovados;
- testes CLI de L4: aprovados;
- `cargo build`: aprovado;
- `cargo test --manifest-path lab/parity/Cargo.toml`: aprovado;
- `crystalline-lint .`: zero violações bloqueantes;
- `git diff --check`: aprovado.

O teste colorido remove ANSI e compara o resultado integral com o modo sem
cor. Há também cobertura de tab + Unicode, múltiplos hints, span detached,
trace multi-linha e trace cuja fonte difere da fonte principal.

## Proveniência da working tree

No instante da medição anterior à criação deste relatório,
`git diff HEAD --stat` registou **26 ficheiros rastreados, 515 inserções e 398
remoções**. Os ficheiros não rastreados eram:

- `00_nucleo/diagnosticos/auditoria-diagnosticos-p1139.md`;
- `typst-passo-1139.md`.

Este próprio relatório foi acrescentado depois dessa fotografia e, por isso,
é o terceiro ficheiro não rastreado no estado final. A lista nominal dos 26
ficheiros rastreados está preservada pelo `git diff HEAD --stat` do estado de
trabalho; não se usou o número para fechar frentes fora de P1139.

## Decisão materializada

O adendo de P1139 mantém a ADR-0045 como decisão incremental correta no seu
contexto original e substitui apenas o default atual pelo formato humano
vanilla-espelhado. L2 continua pura; L4 resolve previamente `FileId → Source +
path`; L1 produz mensagens e tracepoints sem regex de apresentação. A API
multi-source usa `codespan-reporting 0.11.1`, a mesma versão do vanilla
ratificado.
