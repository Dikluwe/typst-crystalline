# P1301r2 — recibo do resselo de linhagem

## Veredito

**PASS.** O resselo ocorreu uma única vez, depois do GREEN interno e da matriz
pública candidata normal/invertida. A alteração pós-selo nos dois Prompts L0
ficou limitada exatamente à sua única linha `Hash do Código`; as identidades
semânticas congeladas permaneceram idênticas.

Regime: protocolo Tekt completo, **executado sem atestação de isolamento
técnico**.

## Entradas e prova anterior à escrita

- Selo P4-r2: `760846d4be24e6422b3284ae0853bc7647114cdfa25a92909207b1b10de9dcca`.
- Runner selado: `622a97b86121d8b7843710e0c54d9133c9cfe123fabcc82497cdad0fe7eac3fb`.
- Prompt `field_access.md`, full antes:
  `38d6f5cdee302491ec059577174d4f6dc6d3c28e3d2da2748f61c05853cc0c16`;
  semântico normalizado: `d6a3d67c1b7d2f23783cb85ec81a4520b50a2a39cb792d56fdc3663755656233`.
- Prompt `tests.md`, full antes:
  `a4ade7eda600891465c62c320d80b7c2182c30dbb5f456de210c4a823b3e609a`;
  semântico normalizado: `bf0d711419ebe33ce7bc7b54c6bd187798ef94c64747e6a50a0aa5674905924a`.
- Consumer `field_access.rs` antes do resselo:
  `e929aa9004ad74497c1bc40fbc64498877dd155423f88707231bed98ae2fdf81`.
- Consumer `tests.rs` antes do resselo:
  `6f2a17e97ae812728d8829ff3b69776970cfa02c09efe5c8e7e6720a82e429c0`.

O dry-run anterior à escrita informou somente:

```text
Would fix ./01_core/src/compiler/eval/bindings/field_access.rs prompt=00_nucleo/prompts/compiler/eval/bindings/field_access.md old=27bf3c60 hash-a=adcb180b hash-b=2d102890
Would fix ./01_core/src/compiler/eval/tests.rs prompt=00_nucleo/prompts/compiler/eval/tests.md old=ac42f559 hash-a=4afb0873 hash-b=343763db
```

## Escrita única e prova posterior

`crystalline-lint --fix-hashes .` foi executado uma vez. Produziu:

```text
Applied ./01_core/src/compiler/eval/bindings/field_access.rs prompt=00_nucleo/prompts/compiler/eval/bindings/field_access.md hash-a=adcb180b hash-b=2d102890
Applied ./01_core/src/compiler/eval/tests.rs prompt=00_nucleo/prompts/compiler/eval/tests.md hash-a=4afb0873 hash-b=343763db
Re-running analysis... ✅ 0 drift warnings remaining
```

Resultados:

- Prompt `field_access.md`, full depois:
  `de81ef3bf6572a1777f9057655e8433c6f2b393e8846f1fbdb66bcd1c8f81df9`;
  semântico normalizado ainda `d6a3d67c1b7d2f23783cb85ec81a4520b50a2a39cb792d56fdc3663755656233`;
  delta único `Hash do Código: d37b05cb` → `Hash do Código: 2d102890`.
- Prompt `tests.md`, full depois:
  `5ca2ad1e4bcf3f6fe30909be21f42bb2a2a5939a6efcbdfc136dba5b44e7a8d1`;
  semântico normalizado ainda `bf0d711419ebe33ce7bc7b54c6bd187798ef94c64747e6a50a0aa5674905924a`;
  delta único `Hash do Código: c82a31ba` → `Hash do Código: 343763db`.
- Consumer `field_access.rs` depois:
  `29abd27cc01b347da9fbc12a93f05265882c043d36cd9a03cff5e6487a96e024`,
  com `@prompt-hash adcb180b`.
- Consumer `tests.rs` imediatamente depois do resselo:
  `e32228e6cf9929f42ae4664c8c1bfb70bc69d7e126c809f376ca321789fd1d67`,
  com `@prompt-hash 4afb0873`.

Uma segunda execução de `crystalline-lint --fix-hashes --dry-run .` respondeu
`Nothing to fix`. `git diff --check` também passou.

## Proveniência

Medição encerrada em `2026-09-03T22:28:13,885217207-03:00`, HEAD
`1f082370e59939de7b57992e137a9f74bfb6758f`, working tree não commitida. O
`git diff HEAD --stat` naquele instante continha 8 ficheiros rastreados, 617
inserções e 24 remoções, incluindo mudanças anteriores do utilizador e P1300.
Nenhum ficheiro foi staged ou commitado.

## Reparo test-only posterior e estabilidade da derivação

O gate ampliado P1300 encontrou depois quatro expectativas antigas no helper
test-only, já contraditas pela obrigação P1301 do mesmo Prompt. O reparo está
registado em `p1301r2-candidate-repair-1.md`, SHA-256
`9fa774846c14f53eeb1077788921ae9cae0f3826dcb1b1b8f1ca7c2ff8c93233`:
P1300 passou `10/10` e P1301 passou `4/4`, sem mudança do Prompt L0, do
contrato ou do código produtivo.

O SHA-256 bruto final de `tests.rs` passou a
`5437bd761f48bef79b2eedd5c2e920bcba310e41c0e85346afaf6db6a2e68af9`.
Após esse reparo, antes de qualquer nova escrita de linhagem:

- o digest semântico normalizado de `tests.md` continuou
  `bf0d711419ebe33ce7bc7b54c6bd187798ef94c64747e6a50a0aa5674905924a`;
- o full hash de `tests.md` continuou
  `5ca2ad1e4bcf3f6fe30909be21f42bb2a2a5939a6efcbdfc136dba5b44e7a8d1`;
- `@prompt-hash 4afb0873` e `Hash do Código: 343763db` permaneceram;
- `crystalline-lint --fix-hashes --dry-run .` respondeu `Nothing to fix`.

Assim, o primeiro resselo não foi supersedido mecanicamente: houve exatamente
uma execução de escrita do linter e nenhuma segunda aplicação. A observação
"precisa ser supersedido" no recibo do autor do reparo era uma cautela anterior
ao dry-run e foi refutada pela medição do próprio linter.
