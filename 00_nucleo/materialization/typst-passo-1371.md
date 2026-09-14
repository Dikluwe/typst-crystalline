# Passo 1371 — reclassificação executável do S11

## Objetivo

Substituir o `Unknown(PARSER_UNSUPPORTED)` histórico de S11 por observação
executável do adaptador cristalino P1361, sem usar relógio ou executar watch.

## Entradas congeladas

- fonte vanila do formatter: SHA-256 `8c4aebfcb63da6b6df1cbd9c539ba3142dbd9496b6e23d4c8f9fd10104019a07`;
- verificação P1361: SHA-256 `72f83b6d0864b857982fc217043330157a7d68b129eb2a58f8cd2c74f2cf608b`;
- fonte cristalina atual `02_shell/src/duration.rs`: SHA-256 `1b879bc834d7e309bd0582c752540a7ad4229b5c36de410b52ed09ea4338fd08`;
- teste atual `02_shell/tests/p1361_duration.rs`: SHA-256 `d7fcf84b2d2bbad018bd9309eec8130e07c5b581ce055897da74fda6db79b5e4`.

## Contrato

O desenho inicial executaria Cargo, mas duas tentativas limitadas terminaram
antes do teste porque `RLIMIT_FSIZE=8 MiB` foi herdado pelo `rustc`, que recebeu
`SIGXFSZ` ao criar o artefato. A revisão R2 não aumenta o limite nem tenta
compilar outra vez. Ela usa o executável focal preexistente, posterior às
fontes por timestamp e inventariado por nome, lista e SHA-256:

```text
target/debug/deps/p1361_duration-785cf7dc0d2800f9
  p1361_duration_exact_vector_is_deterministic_and_boundary_safe --exact
```

SHA-256 do executável: `06c91f0646fff5c4263935851e8d2899ea2cc3b8eb86af5beab6964daa75e1be`.

`Preserved` exige exit 0, exatamente um teste aprovado, zero falhas e zero
timeout/sinal/limite. Qualquer execução incompleta é `Unknown`; falha completa
é `Violated`. Somente então S11 muda para `MATCH`, pois o teste compara o
formatter cristalino com as strings autoritativas congeladas do vanila.

## Segurança

Processo único, sem Cargo/rustc, timeout 120 s,
`RLIMIT_AS=3 GiB`, `RLIMIT_FSIZE=8 MiB`, `RLIMIT_CPU=120 s`, canais em arquivos
temporários e leitura máxima de 2 MiB cada. Sem retry, watch, binário Typst,
rede deliberada ou suíte ampla.

Durante a primeira calibração, o recibo conservou apenas o início de stderr e
não tornou observável o diagnóstico final após warnings. Uma revisão focal
acrescenta caudas limitadas a 8 KiB e autoriza uma segunda e última execução;
ela revelou `SIGXFSZ`. A troca arquitetural para o executável já materializado
é permitida uma vez e não constitui uma terceira compilação.

## L0 e regime

Nenhum código L1–L4 ou Prompt L0 muda. Protocolo completo executado sem
atestação de isolamento; o fechamento cobre somente o vetor S11 congelado.
