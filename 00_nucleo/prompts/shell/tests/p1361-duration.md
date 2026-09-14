# Prompt L0 — teste de integração do formatter de duração P1361
Hash do Código: 3659e29b

**Camada:** L2 — teste de integração
**Ficheiro alvo exclusivo:** `02_shell/tests/p1361_duration.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Propriedade

Este Prompt possui exclusivamente o teste público binding-free do formatter de
duração. A obrigação produtiva permanece em `shell/duration.md`; este owner não
legitima outro código nem acrescenta comportamento.

## Entrada selada

O teste deriva do contrato e selo P1361. Recebe `Duration` literais e observa
somente `format_duration(duration).to_string()`. Não lê relógio, não cria
processos, não executa `watch`/`w`, não faz I/O e não usa tempo de execução como
dado.

## Aceitação

- cobre os 22 casos positivos congelados, incluindo zero `0 s` e 300.006 s;
- repete a ordem canônica e inversa três vezes;
- distingue as fronteiras de ns/µs/ms/s, cinco minutos e um dia;
- é executado apenas por nome exato com um job e timeout limitado;
- recibo com zero testes, timeout ou OOM não recebe crédito.

P1362 acrescenta ao mesmo teste público as quatro variantes de `WatchStatus`
com duração literal 2.8492 s, além de controles que provam que Error ignora
duração e que Success/PartialSuccess não trocam os prefixos. O teste continua
sem relógio, processo ou I/O e nunca chama `watch`/`w`.

P1363 acrescenta um teste separado de `format_watch_line` com componentes
literais `03:04:05` para as quatro variantes. Exige padding de dois dígitos,
um único espaço após `]`, ausência de newline e preservação integral do corpo.
Não consulta hora local nem aceita valor de execução como expected.

Este prompt foi formalizado durante a correção de linhagem posterior ao primeiro
GREEN porque o teste de integração nasceu sem header. A ordem imperfeita deve
ser registrada como limitação de atestação, nunca ocultada como isolamento
completo.
