# P1292 — recibo final de preservação

**Veredito de preservação:** `PASS`
**Papel:** verificador final independente
**Regime:** materialização Tekt completa, segregada por papel/capacidade e
ordem; filesystem compartilhado, sem alegação de isolamento técnico
**Instante da captura anterior aos artefatos finais:**
`2026-09-01T12:45:56-03:00`
**HEAD:** `0eb39f8ecb48930515f2cadb6a378450855b5a72`
**Branch:** `Tekt`; working tree não commitada.

## Cadeia protegida

- contrato canônico v14:
  `0715876251f1ae4c19b5bbd881b4f8af3f331378d3f83ca573d56ec7f8fd46df`;
- seal v14:
  `fb7e5271f68ba992cd358384bd39ac827e30c1e438500d721b8d77fa2f1aca6d`;
- receipt contratual:
  `3e30fa643ef0d5bef8d1e4eb6f236b62b759bbb6c6259b448925fc455d02343c`;
- amendment-13:
  `a5787306b4a13fb99f60d2c0c63fb54164ef1713ff1c9c5237aec2c7c30619a0`;
- 27/27 L0s do v14 coincidiram byte a byte;
- objetos `lots` e `comparison_policy` permaneceram, respectivamente,
  `edeb8a195e15263dbfbc71de9d242657e79f9dd65e94c97e990b066e03f119bf`
  e `71eb4eafff8d88233aaa3df6b2666a651db75ebcdfd0fe2d470dc6620d190a46`;
- oráculo protegido `04_wiring/tests/p1292_contract.rs`:
  `fa8f8770ea188a6bfe4e3415a6e053356d38e945dc950be8b864424b24b983aa`;
- plano adversarial:
  `ccbaab3421a8d4bc4f7ca047ecdfc21a874c7f80fd4ad84ac5795503e294298f`;
- recibo adversarial:
  `b2b3ec632bb8a544ed73a0dae2ecf9e598beeb2f92a5d6e7a54228b85366673c`:
  23/23 mutantes mortos, score `1.0`, sobreviventes 0, `Unknown=0`.

O v12 foi corretamente invalidado durante a primeira verificação final: o
`crystalline-lint --fix-hashes .` pós-ataque reescreveu `Hash do Código` em
dois L0s. O autor contratual independente reconstruiu os bytes v12 e provou
que somente essa linha mudou; o v13 ressellou os hashes atuais sem alterar
obrigação, vetor ou política de comparação. V5 verde não foi usado como
substituto dessa preservação.

Uma auditoria pre-commit posterior normalizou o EOF de
`entities/elements/flush.md`. O arquivo atual tem 1.867 bytes, termina em uma
única LF e possui SHA
`75967e0915aa211ef297c7f88059c3ce675c892de450930137760605df223c37`;
acrescentar somente outra LF reproduz exatamente o pin v13 `7e1290bd...`.
Os outros 26 L0s ficaram byte-idênticos. O v14 ressellou esse whitespace e o
consumer foi sincronizado mecanicamente para `@prompt-hash 75967e09`, sem
alteração de corpo produtivo, obrigação ou observável.

## Reabertura test-only P1030

A primeira suíte global detectou um falso negativo no helper histórico
`p1030_set_vec_delim_aplica`: ele observava apenas `MathMatrix`, apesar de o
produto já construir corretamente `MathVec`. O autor independente de testes
alterou somente o observador para distinguir as duas entidades, sem mudar
expectativa ou produto. O arquivo atual
`01_core/src/compiler/eval/tests.rs` tem SHA-256
`7e5c362b1854214aaedb90e1e01949a78f812164da8fcf9f6a2cdcaa4b052017`;
o recibo RED/reabertura tem
`edabc574c2b8748e7b114230348aac3312b4acb66e70e5c587b04b6ca2ca5a61`.
P1030 terminou 7/7, P1292 core 17/17 e a workspace integral ficou verde.

## Superfície default

Comando:

```text
python3 lab/surface-inventory/run_probes.py --profile default --vanilla-bin /usr/local/bin/typst --crystalline-bin target/release/typst --output 00_nucleo/diagnosticos/p1292-surface-default.json --probes lab/surface-inventory/probes.json --inventory 00_nucleo/diagnosticos/p1284-inventory-default.json
```

Resultado: **111 total, 99 MATCH, 12 DIFFERENCE_OR_DISABLED, Unknown=0**.
Os quatro casos antes acionáveis — `math.cancel`, `math.underline`,
`math.vec` e `place.flush` — são `MATCH`. Os doze restantes são exatamente
as duas superfícies desligadas pelo perfil (`html`, `pdf.data-cell`) e as dez
extensões cristalinas já justificadas. Portanto os 95 MATCH do baseline fresco
foram preservados, com quatro ganhos e zero perdas. O JSON tem SHA-256
`a827b192fc38541f9b00baa3b666fc94aa2ff6f691ca7867bf28aa3a451e5f15`.

## Superfície HTML

O perfil HTML usa deliberadamente seu inventário canônico próprio
`p1284-inventory-html.json` (`2a3f5f7d...`), que seleciona **115** sondas, e
não o universo default de 111. Forçar 111/99/12 aqui adulteraria o runner.
O resultado bilateral correto foi **115 total, 89 MATCH, 26
DIFFERENCE_OR_DISABLED, Unknown=0**, SHA-256
`646884f8268f74e0acab79cf19e82fc81cceebf9e4258daeeb3ec2474d0f950a`.
Os quatro samples P1292 pertencem à amostragem default e não são selecionados
no sample HTML; sua obrigação nominal e funcional é coberta pela superfície
default e pelo oráculo protegido. Nenhum delta HTML não selado foi observado.

## Estado exato medido

Antes da escrita dos artefatos finais:

- `git status --short` SHA-256:
  `78cfbf0c5845198488fb0844c012342736ca3e2cf4a4432efaa67611979498b8`;
- `git diff HEAD --stat` SHA-256:
  `fdca3963bd5d1fc993a6e748ca03c2808459916fff992261f93c7d0a0975c02c`;
- stat: **48 arquivos rastreados**, 3.326 inserções e 515 remoções;
- binário candidato `target/release/typst`:
  `ed5f85e03e6fe473c1a7a9a42cceafe8c5fbe075de4dad56a7c88c07ad2c3b6f`;
- vanilla `/usr/local/bin/typst`:
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`,
  revisão ratificada `a51e02804`.

Os artefatos finais são não rastreados e, por definição do Git, não entram no
`git diff HEAD --stat`; seus caminhos e hashes são registrados no certificado.

Revalidação v14 em `2026-09-01T13:13:01-03:00`: `git status --short`
`89ff400a951a8e1ce52700b2c560910b098c43e4d40cf32159daa3e0f46b1063` e
`git diff HEAD --stat`
`2b5cf22b6ad633f43303fdf80e436d41256b2f561e0c30b38985fa91886d8ce2`.
