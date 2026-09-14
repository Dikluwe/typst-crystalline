# P1359 — causa raiz do crescimento de memória no verificador

## Achado

O contrato continha casos de parsing para `watch` e `w`, mas não declarava o
modo de observação obrigatório. O verificador pós-implementação tentou observar
esses casos como processos de CLI. `watch` é deliberadamente não terminante;
uma chamada bloqueante que captura stdout/stderr aguarda o encerramento e pode
acumular saída sem limite.

Isto não é uma propriedade necessária do produto nem do teste de parsing. Os
testes independentes em `02_shell/src/cli.rs` usam `Args::try_parse_from` para
`compile`, `watch` e `w` e retornam imediatamente. Os quatro testes de processo
em `04_wiring/tests/cli.rs` invocam somente `compile`, que é finito.

## Correção do protocolo

- V05–V07 são observáveis exclusivamente por parser in-process;
- é proibido lançar `watch`/`w` como subprocesso no harness P1359;
- a matriz bilateral de processo usa somente `compile`;
- ajuda de `watch` pode ser observada apenas por `--help`, que termina;
- o selo anterior é inválido porque contrato/oráculos não continham esta
  restrição de capacidade.

## Estado

Não há processo `watch`, cargo ou verificador restante após a interrupção. A
implementação candidata não muda por esta correção. Nenhum filtro `p1359_` será
reexecutado até a cadeia ser ressellada com o modo finito explícito.

