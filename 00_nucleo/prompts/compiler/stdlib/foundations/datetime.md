# Prompt L0 — `stdlib/foundations/datetime` — superfície pública de `datetime`
Hash do Código: 9dbe969d

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/foundations/datetime.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/foundations.md`
**Origem**: P1146
**Baseline**: vanilla ratificado `a51e02804`
**ADRs**: ADR-0107, ADR-0108, ADR-0127

---

## 1. Medição anterior à decisão

A varredura literal de
`a51e02804:crates/typst-library/src/foundations/datetime.rs:241-499`
encontra onze funções no `#[scope] impl Datetime`: constructor, `today`,
`display`, `year`, `month`, `weekday`, `day`, `hour`, `minute`, `second` e
`ordinal`. O helper Rust `kind` está fora do scope e não é field da linguagem.

Em `2026-08-24T19:58:15-03:00`, HEAD
`c65c839ff5544a66683b7e81edba923acb49760e`, com o lote P1145 não commitado,
os dois binários vanilla ratificados devolveram resultados idênticos:

```text
datetime.day(date-2024-02-29)       -> 29
datetime.weekday(date-2024-02-29)   -> 4
datetime.ordinal(date-2024-02-29)   -> 60
datetime.year(time-only)            -> none
datetime.hour(date-only)            -> none
datetime.ordinal(2023-12-31 + time) -> 365
datetime.display(date-only)         -> "2024-02-29"
datetime.display(time-only)         -> "23:58:57"
datetime.display(full)              -> "2023-12-31 01:02:03"
```

`type(datetime.day) == function`. A forma `value.day` falhou nos dois
binários com `cannot access fields on type datetime`; nesta baseline, a
superfície medida é estática no valor-tipo, apesar de a função Rust receber
`&self` e do exemplo documental usar sintaxe de método.

`datetime.today` aceita `offset: auto`, inteiro interpretado em horas ou
`duration`; a fonte converte ambos para `Option<Duration>` antes de chamar
`World::today`. O World vanilla escolhe data local para `None` e data UTC com
o offset fornecido para `Some`. Ausência de data produz
`unable to get the current date`.

O cristalino, no mesmo estado, falha em `datetime.day` e `datetime.today` com
`type datetime does not contain field ...`. A entidade já implementa sete dos
oito accessors puros; falta `ordinal`. O contrato cristalino de World ainda
usa `Option<i64>` e perde offsets com precisão inferior a uma hora.

## 2. Decisão de superfície

O valor-tipo `datetime` expõe exatamente:

```text
today, display, year, month, weekday, day,
hour, minute, second, ordinal
```

O constructor continua pertencendo a `foundations/cast.rs`. Os dez fields
acima pertencem a este nó. Não criar dispatch de instância enquanto a
baseline ratificada continuar a rejeitá-lo.

Cada accessor recebe `self: Datetime` como primeiro positional e nenhum outro
argumento. Retorna `Int` quando a componente existe e `None` caso contrário:

- data: `year`, `month`, `weekday`, `day`, `ordinal`;
- hora: `hour`, `minute`, `second`;
- `weekday` usa Monday=1 até Sunday=7;
- `ordinal` usa 1 até 365/366.

`display(self, pattern: auto)` usa os defaults:

```text
date      -> [year]-[month]-[day]
time      -> [hour]:[minute]:[second]
datetime  -> [year]-[month]-[day] [hour]:[minute]:[second]
```

Padrões customizados seguem a linguagem de format description do crate
`time`; componente incompatível retorna
`failed to format datetime (insufficient information)`. Erros de descrição
preservam a mensagem pública da baseline. Não aceitar uma aproximação parcial
sem scope-out L0 nomeado.

`today(offset: auto)` chama `World::today(None)`. Inteiro converte para
`Duration::from_hours`; `Value::Duration` é passado integralmente. Outros
tipos, positional em vez de named, named desconhecido e aridade inválida são
erros. `None` do World torna-se `unable to get the current date`.

## 3. Estrutura

- O nó contém somente glue/semântica de stdlib; zero I/O e zero relógio.
- `datetime_type_field` é match fechado e devolve as funções nativas acima.
- Accessors delegam à entidade `Datetime`; não leem campos privados nem
  duplicam calendário.
- `today` recebe o World já injetado pelo ABI de função nativa.
- `display` usa o suporte `time` já permitido em L1.
- Não criar registry, reflexão ou despacho dinâmico.

## 4. Gate P1146

A materialização requer `Datetime::ordinal() -> Option<u16>` e a mudança de
`World::today` de `Option<i64>` para `Option<Duration>`. Ambos são contrato
público Rust. Este L0 deve ser guardado/ressellado e a execução deve parar no
gate ADR-0127 antes de alterar L1 ou L3.

## 5. Verificação

- RED da presença dos dez fields estáticos;
- RED da matriz date-only/time-only/full dos oito accessors;
- RED de bissexto, weekday e ordinal;
- RED dos três defaults e padrões customizados de `display`;
- RED determinístico de `today` com World falso e offsets inteiro/duration;
- sondas equivalentes aos dois binários vanilla;
- workspace build/fmt/check, `git diff --check` e `crystalline-lint .` verdes.
