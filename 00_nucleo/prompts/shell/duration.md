# Prompt L0 — formatter público de duração
Hash do Código: b8867f4e

**Camada:** L2
**Ficheiro alvo exclusivo:** `02_shell/src/duration.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Propriedade e propósito

Este Prompt possui exclusivamente o formatter humano de duração. Ele converte
um `std::time::Duration` já fornecido em representação textual pública; não lê
relógio, não mede execução, não dorme e não conhece `watch`, pipeline ou I/O.

## Medição anterior à decisão

O recibo `diagnosticos/p1361-duration-preflight.md` congela o formatter e o
teste do vanila ratificado antes desta decisão. A fonte vanila seleciona peças
de dia, hora e minuto, interrompe antes de segundos quando há dia ou hora,
arredonda frações a duas casas e escolhe a unidade pelas fronteiras de um
segundo, um milissegundo e um microssegundo. O cristalino não possui símbolo
comparável; P1357 classificou S11 como limitação do harness, não como prova de
gap semântico.

Classificação ADR-0107: a string apresentada ao utilizador é morfologia pública;
a macro, o wrapper Display e a mecânica de arredondamento são livres desde que
o vetor observável seja preservado. Inferência: uma função pura em L2 basta para
criar o adaptador binding-free. Refutariam isso divergência no vetor medido,
necessidade de relógio ou dependência numa camada de execução.

## Contrato público

```rust
pub fn format_duration(duration: std::time::Duration) -> impl std::fmt::Display;
```

P1362 acrescenta a representação pública fechada dos corpos de status:

```rust
pub enum WatchStatus {
    Compiling,
    Success(std::time::Duration),
    PartialSuccess(std::time::Duration),
    Error,
}

impl WatchStatus {
    pub fn message(&self) -> String;
}

pub fn format_watch_line(
    hour: u8,
    minute: u8,
    second: u8,
    status: &WatchStatus,
) -> String;
```

`Compiling` produz `compiling ...`; `Success(d)` produz `compiled successfully
in {format_duration(d)}`; `PartialSuccess(d)` produz `compiled with warnings in
{format_duration(d)}`; `Error` produz `compiled with errors`. A função não lê
relógio e não emite I/O. Success versus PartialSuccess é decisão do consumidor
a partir de warnings tipados, nunca inferência por texto.

P1363 envolve cada corpo em `[{hour:02}:{minute:02}:{second:02}] {body}`.
Hora, minuto e segundo são componentes válidos fornecidos pelo consumidor;
L2 não lê relógio, timezone ou ambiente. Para `03:04:05`, as quatro variantes
produzem as quatro linhas literais congeladas no Passo 1363. A função devolve
o texto sem newline; o consumidor decide o canal e a terminação.

O valor retornado escreve:

- dias, horas e minutos não nulos nesta ordem;
- somente dias/horas/minutos quando dia ou hora for não zero;
- segundos inteiros quando a duração for pelo menos cinco minutos;
- segundos com arredondamento decimal para durações de pelo menos um segundo e
  menos de cinco minutos;
- milissegundos, microssegundos ou nanossegundos nas fronteiras inclusivas
  correspondentes;
- `0 s` para zero, pois o ramo de zero é deliberadamente o de segundos.

Cada peça usa um espaço entre número e unidade e as peças usam um espaço entre
si. O arredondamento decimal é a duas casas, e a apresentação normal de `f64`
não força zeros finais.

## Vetor obrigatório

Os 18 pares exatos do Passo 1361 são obrigatórios. Além deles, controles de
fronteira devem distinguir 999 ns/1 µs, 999 µs/1 ms, 999 ms/1 s,
`299.999 s -> 4 min 60 s`, `300 s -> 5 min 0 s`,
`300.006 s -> 5 min 0 s`, e a passagem de 23 h 59 min para 1 d. Alterar apenas
`>= 5 min` para `> 5 min` é equivalente no ponto inteiro de 300 s e não conta
como mutante válido; um deslocamento para 300.010 s é discriminado por
300.006 s. O adaptador usa durações literais;
tempo de parede, monotônico, scheduler ou duração do próprio teste são entradas
proibidas.

## Restrições

- zero I/O e zero estado global;
- zero dependência externa nova;
- não importar L3/L4 nem lógica privada de repr de L1;
- não iniciar ou controlar `watch`;
- não afirmar equivalência fora das strings e fronteiras congeladas.

Cor, fullscreen, limpeza de terminal, canal e ordenação perante diagnósticos
pertencem ao consumidor/envelope. O formatter P1363 cobre somente o timestamp
`HH:MM:SS` sem data/offset/fração; a fonte do instante continua fora de L2.

## Aceitação

- vetor obrigatório e controles de fronteira passam em teste unitário exato;
- determinismo por repetição e ordem inversa;
- mutantes de unidade, limite inclusivo, omissão, arredondamento e relógio são
  rejeitados pelo contrato selado;
- `cargo fmt --check`, `git diff --check` e `crystalline-lint .` passam.
