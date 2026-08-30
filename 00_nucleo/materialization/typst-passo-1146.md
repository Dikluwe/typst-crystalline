# P1146 — auditoria integral da superfície pública de `datetime`

**Data:** 2026-08-24  
**Estado:** `PLANEADO — não executar neste passo`  
**Baseline de linguagem:** vanilla ratificado `a51e02804`  
**Sentinela de origem:** `datetime.day`  
**Dependência:** P1145 materializado na working tree, ainda não commitado

## 1. Objetivo

Auditar como uma única família a superfície pública de `datetime`, em vez de
corrigir isoladamente a sentinela `datetime.day`. Medir primeiro o scope
completo no vanilla ratificado, comparar as formas estática e de instância com
o cristalino, nuclear os L0s necessários e só então decidir a materialização.

O passo deve separar três classes que partilham o namespace, mas não
necessariamente o mesmo risco arquitetural:

1. accessors puros de componentes e calendário;
2. `display`, que inclui defaults e uma linguagem de formato;
3. `today`, que depende do `World` e da fronteira de tempo/I/O.

Não assumir que a existência dos métodos Rust na entidade significa que os
membros existem na linguagem. Também não assumir que todos os membros devem
ser implementados no mesmo subpasso apenas por pertencerem ao mesmo scope.

## 2. Proveniência inicial e preservação da working tree

- HEAD medido em `2026-08-24T19:54:13-03:00`:
  `c65c839ff5544a66683b7e81edba923acb49760e`.
- Baseline vanilla: `a51e02804`.
- A working tree não está limpa: contém o lote P1145 ainda não commitado.
- Estado inicial: 12 ficheiros rastreados alterados, com 585 inserções e 28
  remoções segundo a medição anterior do lote, mais
  `typst-passo-1145.md` não rastreado.
- Os ficheiros P1145 incluem os três L0s de gradient, cinco ficheiros Rust,
  três snapshots PDF e testes de export. P1146 deve preservá-los integralmente
  e não os reformatar, ressellar ou reinterpretar.

Antes de executar P1146, registrar novamente:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
```

Se o estado diferir, a nova proveniência substitui estes números. Nenhum
número usado para fechar a frente vale sem HEAD, hora e estado exato da árvore.

## 3. Medição estática já estabelecida

A fonte ratificada
`a51e02804:crates/typst-library/src/foundations/datetime.rs:241-499`
contém, no bloco `#[scope] impl Datetime`, onze funções públicas:

```text
datetime, today, display,
year, month, weekday, day,
hour, minute, second, ordinal
```

O helper Rust `kind`, anterior ao scope público, não tem `#[func]` e não é
membro da linguagem. Operadores `Add`, `Sub`, `PartialOrd` e `Repr`, posteriores
ao fecho do scope, também não devem ser contados como fields sem medição
independente da sua exposição pela linguagem.

No cristalino, `01_core/src/entities/world_types.rs:119-194` já representa
data, hora ou ambas e expõe métodos Rust públicos para `year`, `month`, `day`,
`hour`, `minute`, `second` e `weekday`. Não foi encontrado `ordinal`. O
constructor `native_datetime` já existe em
`01_core/src/compiler/stdlib/foundations/cast.rs`, mas não foi encontrada uma
superfície `datetime_type_field` nem despacho de instância correspondente.

Estas observações são inventário inicial, não decisão de implementação.

## 4. Fase A — inventário literal do vanilla

Reler o ficheiro ratificado desde a definição da entidade até ao fim dos casts
auxiliares e produzir uma tabela:

```text
membro | assinatura Typst | posicional/nomeado | default | retorno |
date-only | time-only | datetime | erro | dependência de World | since
```

Extrair da fonte, antes de decidir:

- constructor: combinações válidas e incompletas, ranges, mensagens e hints;
- `today`: parâmetro `offset`, casts aceites, ausência de data no World e
  comportamento perante offsets que cruzam dias;
- `display`: `auto`, padrões customizados, componentes sem informação
  suficiente e erros de parsing/formatação;
- accessors: tipo numérico exato e matriz `valor`/`none` para data, hora e
  datetime;
- `weekday`: convenção Monday=1 até Sunday=7;
- `ordinal`: convenção, anos bissextos e limites do ano.

Não transportar para o contrato cristalino detalhes mecânicos do crate `time`.
Registrar quais comportamentos são semântica, sintaxe, morfologia ou mensagem
observável, conforme ADR-0107.

## 5. Fase B — sondas nos dois binários vanilla

Executar cada sonda em
`lab/typst-original/target/release/typst` e `/usr/local/bin/typst`. Só usar como
baseline resultados idênticos entre ambos.

### 5.1 Matriz mínima dos accessors

Criar valores discriminantes:

```typst
datetime(year: 2024, month: 2, day: 29)
datetime(hour: 23, minute: 58, second: 57)
datetime(year: 2023, month: 12, day: 31,
         hour: 1, minute: 2, second: 3)
```

Para cada um de `year`, `month`, `weekday`, `day`, `hour`, `minute`, `second` e
`ordinal`, medir:

- forma de instância `value.member()`;
- forma estática `datetime.member(value)`;
- `type`, `repr` e valor resultante;
- `none` quando falta a componente aplicável;
- chamada sem `self`, `self` de tipo errado, argumentos excedentes e named
  desconhecido;
- descoberta do field sem chamada, para distinguir presença de execução.

Adicionar casos de calendário: primeiro e último dia do ano, 29 de fevereiro
bissexto, datas em cada weekday e anos negativos/limites se o constructor os
aceitar.

### 5.2 `display`

Medir nas duas formas de chamada e nos três kinds:

- default `auto` e padrão explicitamente equivalente;
- componentes de ano, mês, dia, weekday, ordinal, hora, minuto e segundo;
- escape de colchetes e literais;
- padrão vazio;
- componente incompatível com date-only ou time-only;
- descrição inválida, bracket não fechado, component/modifier desconhecido;
- tipo errado, aridade e named desconhecido;
- texto exato dos erros quando a mensagem for o observável.

Não implementar uma aproximação parcial do minilanguage sem scope-out L0
explícito, nomeando o passo que completará o subconjunto omitido.

### 5.3 `today`

Primeiro confirmar por fonte e sonda como os binários recebem a data do World.
Medir, quando o harness permitir injeção determinística:

- offset omitido, inteiro positivo/negativo e duração equivalente;
- cruzamento de meia-noite e de limites de mês/ano;
- World sem data disponível;
- forma estática e campos/erros de aridade e tipo.

Não usar o relógio do processo em L1. Se o CLI vanilla não permitir fixar a
data de forma reproduzível, a ausência de sonda determinística deve ser
registrada como limite, e a decisão deve apoiar-se na fonte mais um teste com
World falso/injetado — nunca numa data corrente sem proveniência.

## 6. Fase C — auditoria cristalina e owners

Auditar com `file:line`:

- `Type::Datetime`, `Value::Datetime`, lookup de fields de tipo e de instância;
- constructor `native_datetime` e seus testes/L0 vigente;
- entidade `Datetime`, invariantes de `date`/`time` e métodos existentes;
- `World::today` ou equivalente e a implementação L3 que fornece a data;
- infraestrutura atual de `repr`, comparação e operações de datetime;
- suporte existente a parsing/formatting do crate `time` e sua whitelist L1;
- todos os consumers que já dependam diretamente dos métodos públicos Rust.

Produzir matriz obrigatória:

```text
caso | vanilla medido | cristalino atual | língua/mecânica |
owner | L0 afetado | contrato público necessário? | decisão
```

Auditar os L0s vigentes antes de propor owner. O conjunto candidato inclui:

- `00_nucleo/prompts/compiler/stdlib/foundations/cast.md`;
- `00_nucleo/prompts/compiler/stdlib/foundations.md`;
- `00_nucleo/prompts/compiler/eval/call_dispatch.md`;
- `00_nucleo/prompts/entities/value.md`;
- um novo L0 dedicado à entidade `Datetime`, se a entidade pública ou a sua
  API Rust tiver de mudar;
- o L0 de World/infra correspondente, somente se `today` o afetar.

Não criar um dispatcher genérico nem mover lógica para um hub por conveniência.
Escolher o owner depois da medição: accessors de linguagem podem delegar aos
métodos puros existentes; formatting e obtenção da data atual devem manter as
suas dependências nas camadas permitidas.

## 7. Decisão ADR-0108 e gate ADR-0127

Atualizar primeiro apenas os L0s comprovadamente afetados e ressellar os hashes.
Depois classificar cada subfrente:

- adicionar glue privado de fields sobre métodos públicos já existentes, sem
  mudar assinatura/default/fase: correção de paridade em fluxo contínuo;
- adicionar `ordinal` como método público Rust, alterar campos/invariantes da
  entidade ou qualquer assinatura pública: **parar no gate ADR-0127**;
- introduzir ou alterar o fornecimento de data pelo World/L3, comportamento por
  defeito de `today` ou fase do pipeline: **parar no gate ADR-0127**;
- materializar `display` com novo contrato público ou novo default:
  **parar no gate ADR-0127**;
- implementação privada com contrato e defaults já legitimados: documentar por
  que é interna e seguir com L0 primeiro + RED→GREEN.

Em dúvida, parar. O estado correto do documento nesse ponto é
`AGUARDA GATE ADR-0127`, com a proposta de contrato, alternativas e impacto
escritos antes de qualquer alteração L1/L3.

Se somente `ordinal`, `display` ou `today` exigir gate, não bloquear por
associação os accessors puros já legitimados: separar sublotes explicitamente,
sem declarar a família completa enquanto houver scope-out não resolvido.

## 8. Plano RED→GREEN após L0 e eventual aprovação

1. testes RED de presença nas formas estática e de instância;
2. RED da matriz date-only/time-only/datetime dos oito accessors;
3. RED de weekdays, ordinal e fronteiras bissextas;
4. RED de aridade, tipos, named args, mensagens e hints;
5. RED específico de `display`, se legitimado no lote;
6. RED determinístico de `today` com World falso, se legitimado no lote;
7. implementar no owner medido, com despacho fechado e estático;
8. ressellar headers de linhagem afetados;
9. repetir as sondas cristalinas e comparar no nível da linguagem;
10. executar regressões focadas de eval/stdlib/world e export apenas onde
    houver impacto medido;
11. executar `cargo check --workspace`, `cargo build --workspace`,
    `cargo fmt --all -- --check`, `git diff --check` e
    `crystalline-lint .` com zero violations.

Confirmar RED antes de escrever a implementação. Não contar um teste que já
passa como prova do delta.

## 9. Limites e encerramento

Ficam fora, salvo dependência inevitável medida:

- operações aritméticas e comparação de datetime;
- timezone e locale não presentes no scope ratificado;
- refactor geral de World, Args, dispatcher ou crate `time`;
- operações bitwise de `int` e restantes itens da fila pós-P1140;
- limpeza, commit ou alteração incidental do lote P1145.

P1146 encerra quando os onze membros do scope tiverem inventário reproduzível,
as duas formas de chamada e os três kinds estiverem classificados, os L0s
coincidirem com a decisão e o lote autorizado estiver verde. Membros adiados
devem ter scope-out e passo sucessor nomeados; não declarar “datetime completo”
enquanto `display` ou `today` permanecer sem decisão.

---

## 10. Execução até ao gate ADR-0127

Rebaseline em `2026-08-24T19:58:15-03:00`: HEAD
`c65c839ff5544a66683b7e81edba923acb49760e`; lote P1145 preservado, com 12
ficheiros rastreados alterados (585 inserções, 28 remoções) e P1145/P1146 não
rastreados durante a medição.

A varredura literal confirmou onze funções públicas no scope: constructor
mais dez fields. Os dois binários vanilla produziram saída idêntica para a
matriz de accessors, defaults/custom de `display` e offsets de `today`.
Resultados discriminantes: 2024-02-29 tem weekday 4 e ordinal 60; time-only
devolve `none` para accessors de data; date-only devolve `none` para accessors
de hora; os três defaults de display são `2024-02-29`, `23:58:57` e
`2023-12-31 01:02:03`.

A hipótese de duas formas de chamada foi refutada: `datetime.day(d)` existe,
mas `d.day` falha nos dois binários com `cannot access fields on type
datetime`. O cristalino falha na descoberta de `datetime.day` e
`datetime.today`.

Foram nuclearizados os L0s do novo owner de stdlib, hub, field access,
entidade, contrato World e SystemWorld. A implementação completa exige:

1. novo método público `Datetime::ordinal() -> Option<u16>`;
2. mudança pública de `World::today(Option<i64>)` para
   `World::today(Option<Duration>)`, além da semântica local para `None`.

Decisão: **parar no gate ADR-0127**. Nenhum teste RED nem código L1/L3 de
P1146 foi escrito. A continuação requer confirmação humana explícita destes
dois contratos públicos.

## 11. Continuação após aprovação e materialização

O dono respondeu `Continue`, aprovando explicitamente os dois contratos do
gate. A implementação começou somente depois dessa confirmação.

Materialização:

- `Datetime::ordinal() -> Option<u16>` delega ao calendário da data;
- `World::today` e todas as implementações/mocks recebem `Option<Duration>`;
- `SystemWorld::today(None)` usa data local; offset explícito usa UTC mais a
  duração exata, com overflow convertido em `None` sem panic;
- novo owner `foundations/datetime.rs`, com match fechado para os dez fields;
- oito accessors estáticos com matriz valor/`none`;
- `display` default e customizado pela format description do crate `time`;
- `today` aceita `auto`/ausente, inteiro em horas e `Value::Duration`;
- `Value::Type(Type::Datetime)` delega ao owner; nenhuma forma de instância foi
  criada, em conformidade com as sondas ratificadas.

RED confirmado: o teste de superfície falhou inicialmente em
`datetime.year` com `type datetime does not contain field`. GREEN focado: os
três testes P1146 passaram. O teste determinístico de World confirmou a
passagem de `None`, 2 horas e 90 minutos; a repetição em dois passes de eval é
mecânica e preservou os mesmos offsets.

Sonda cristalina após build:

```text
(year, ordinal, display-time) -> [2024, 60, "23:58:57"]
today(offset: 0)              -> datetime(year: 2026, month: 8, day: 24)
today(offset: duration(2h))   -> datetime(year: 2026, month: 8, day: 25)
```

Os dois resultados de `today` coincidem com os dois binários vanilla medidos
no mesmo dia. A subcommand `query` cristalina ainda não serializa metadata
genérico; a equivalência foi verificada por `typst eval` e pelos testes de
eval, não por bytes internos.

Validação executada: testes P1146 3/3, contratos World 6/6, infra `today` 2/2,
`cargo check --workspace`, `cargo build --workspace`, `cargo fmt --all --
--check`, `git diff --check` e `crystalline-lint .` sem violations. Warnings
históricos do compilador/linter permanecem fora do escopo.
