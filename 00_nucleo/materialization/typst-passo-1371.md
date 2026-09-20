# Passo 1371 — reclassificação executável do S11

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Substituir o `Unknown(PARSER_UNSUPPORTED)` histórico de S11 por observação
executável do adaptador cristalino P1361, sem usar relógio ou executar watch.

## Entradas congeladas

- fonte vanila do formatter: identificador administrativo [dado administrativo removido];
- verificação P1361: identificador administrativo [dado administrativo removido];
- fonte cristalina atual `02_shell/src/duration.rs`: identificador administrativo [dado administrativo removido];
- teste atual `02_shell/tests/p1361_duration.rs`: identificador administrativo [dado administrativo removido].

## Contrato

O desenho inicial executaria Cargo, mas duas tentativas limitadas terminaram
antes do teste porque `RLIMIT_FSIZE=8 MiB` foi herdado pelo `rustc`, que recebeu
`SIGXFSZ` ao criar o artefato. A revisão R2 não aumenta o limite nem tenta
compilar outra vez. Ela usa o executável focal preexistente, posterior às
fontes por timestamp e inventariado por nome, lista e identificador administrativo:

```text
target/debug/deps/p1361_duration-785cf7dc0d2800f9
  p1361_duration_exact_vector_is_deterministic_and_boundary_safe --exact
```

identificador administrativo do executável: [dado administrativo removido].

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
