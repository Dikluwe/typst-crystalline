# P1293 — recibo de implementação do lote A

**Estado atual:** `IMPLEMENTED_GREEN_OWN_TESTS_PENDING_INDEPENDENT_REVALIDATION`.
O lote A está implementado e GREEN nos testes próprios; este recibo não o
aprova nem substitui o gate independente.

**Regime:** protocolo Tekt completo, papel segregado de implementador.
**Atestação:** segregado por capacidades e artefatos; sem isolamento técnico de
leitura no filesystem compartilhado.

## Entradas congeladas do selo original

- `HEAD`: `7dd25ff0e222b6c7c640d6bc7957b98f94227507` (`Tekt`), working tree
  não commitida;
- manifesto: `00_nucleo/diagnosticos/p1293-manifest.json`, SHA-256
  `7fd0b482c23ddd639f317a74bf9e07275d61ab5924a322e8fc39f224ba685404`;
- selo: `00_nucleo/diagnosticos/p1293-contract-seal.json`, SHA-256
  `5996cd712dd33def5873e207a0fe3067b9f6c701c12398ca09bd67aa81619402`;
- L0 proprietário: `00_nucleo/prompts/compiler/stdlib/foundations/float.md`,
  SHA-256
  `b07ba6be955e28f01e9e12f3f41d3977e5728c13942b381c13428375aff3b57e`;
- lineage consumida: `@prompt-hash 996763cf`.

O implementador não leu nem executou
`04_wiring/tests/p1293_contract.rs`, o recibo RED protegido ou outputs privados
do testador. Não editou L0, contrato, oráculos, selo, manifesto ou veredito.

## Capacidade e escrita no selo original

Escrita deste papel limitada exatamente a:

1. `01_core/src/compiler/stdlib/foundations/float.rs`;
2. `00_nucleo/diagnosticos/p1293-implementation-receipt-a.md`.

O consumer já possuía os três pontos fechados necessários (`float_type_field`,
`is_float_instance_method`, `dispatch_float_method`). Portanto não houve owner
gap e nenhum outro consumer precisou ser alterado.

## RED próprio anterior ao código produtivo

Com somente o teste próprio de presença/identidade/formato ligado acrescentado
ao consumer e antes de `native_is_nan`, foi executado:

```text
cargo test -p typst-core p1293_is_nan_tem_nome_publico_curto_e_forma_ligada
```

Resultado: `1` teste executado, `0` passou, `1` falhou. Causa semântica:
`float_type_field("is-nan")` devolveu ausência e o teste parou em
`float.is-nan deve existir como função estática`. O build completou; warnings
preexistentes não foram usados como RED. A sequência causal foi preservada na
sessão: esse comando terminou antes do primeiro patch produtivo.

## Implementação

O consumer recebeu uma única `native_is_nan`, com:

- fórmula única `f64::is_nan`;
- `Float` e coerção estática de `Int`;
- `true` somente para NaN e `false` para finitos e `+/-infinito`;
- nome público curto `is-nan` no match estático;
- aridade, named e tipo fechados no mesmo padrão de `is-infinite`;
- dispatch ligado que insere o receiver `Float` e chama a mesma nativa.

Não foi criado trait, registry, fallback, segundo payload ou segunda fórmula.
`float.inf`, `float.nan`, `signum` e bytes não foram tocados.

## GREEN e gates do implementador

Estado medido em `2026-09-01T15:04:33-03:00`, sobre o mesmo `HEAD` e working
tree não commitida. O conjunto exato de caminhos escrito por este papel é o
allowlist de dois caminhos acima.

| Comando | Resultado |
|---|---|
| `cargo test -p typst-core tests_p1293_is_nan` | PASS: `4` passaram, `0` falharam |
| `cargo fmt --all -- --check` | PASS |
| `crystalline-lint --quiet --checks v5 --fail-on warning .` | PASS |
| `crystalline-lint --quiet --checks v15 --fail-on warning .` | PASS |
| `crystalline-lint --quiet --checks v26 --fail-on warning .` | PASS |
| `crystalline-lint --fix-hashes --dry-run .` | PASS: `Nothing to fix` |
| `git diff --check` | PASS |

Os quatro testes próprios cobrem presença/nome curto, semântica de NaN contra
finitos/inteiro/infinidades, delegação ligada e erros fechados de
missing/extra/named/tipo. Eles são evidência do implementador e não substituem
o oráculo protegido nem o veredito independente.

SHA-256 do consumer produzido após formatação:
`406988d2522b78e286d54766eb22e08c7a8842987f0be6ba7a873e0b72d5f6de`.

## Limite do recibo

Este recibo atesta apenas a implementação candidata do lote A e seus testes
próprios. Não alega execução do contrato selado, mutation score pós-candidato,
equivalência funcional geral, aprovação dos lotes B-D ou fecho de P1293. O
implementador para aqui antes do lote B.

## Reabertura — spans diagnósticos

Em `2026-09-01T15:13:15-03:00`, sobre `HEAD`
`7dd25ff0e222b6c7c640d6bc7957b98f94227507` e working tree não commitida, o
coordenador devolveu o lote A exclusivamente por cinco spans diagnósticos. O
implementador continuou sem ler nem executar o oráculo protegido ou recibos
privados.

Sondas públicas próprias foram executadas bilateralmente com
`target/release/typst` e o vanilla ratificado `/usr/local/bin/typst`, usando
cinco ficheiros isolados em `/tmp/p1293-a-span-*.typ`. Elas confirmaram mensagens
textuais iguais e as seguintes âncoras de língua:

| Família | Expressão em modo code | Candidato | Vanilla/esperado |
|---|---|---:|---:|
| static missing | `float.is-nan()` | `12..14` | `0..14` |
| static extra | `float.is-nan(0.0, 1.0)` | `12..22` | `18..21` |
| bound named | `float("NaN").is-nan(other: true)` | `19..32` | `20..31` |
| static cast | `float.is-nan("x")` | `12..17` | `13..16` |
| bound access sem chamada | `float("NaN").is-nan` | `0..19` | `13..19` |

Os ranges são offsets UTF-8 half-open na expressão sem o marcador de markup
`#`. Os cinco números decisórios foram novamente materializados como testes
end-to-end próprios no consumer, usando `eval_expression` e
`Source::span_byte_range`.

Comando RED:

```text
cargo test -p typst-core tests_p1293_is_nan
```

Resultado: `9` executados; os `4` testes anteriores permanecem GREEN e os `5`
novos testes de span ficam RED. Cada falha discrimina exatamente um par da
tabela, sem crash, timeout ou falha de build alheia.

### Causa produtiva e owner gap

`Args` preserva apenas `args.span`, o span agregado da lista sintática de
argumentos. `native_is_nan` recebe valores já avaliados e não possui o span de
cada `Arg`; por isso não pode escolher causalmente a chamada inteira no missing,
o positional excedente, o named completo ou o primeiro positional do cast.

Os spans individuais ainda existem em
`01_core/src/compiler/eval/call_dispatch.rs`: esse consumer já constrói
`CollectionCallSpans` para coleções e já faz override pontual para `eval`, mas o
caminho de float descarta os spans ao converter a AST em `Args`. O quinto caso
nem chega ao consumer float:
`01_core/src/compiler/eval/bindings/field_access.rs` passa `access.span()` ao
erro genérico, enquanto a âncora requerida é `access.field().span()`.

Logo, um reparo causal mínimo exige dois owners produtivos adicionais:

1. `compiler/eval/call_dispatch.rs`, para encaminhar chamada/positional/named
   ao caminho estático e bound de `float.is-nan`;
2. `compiler/eval/bindings/field_access.rs`, para ancorar o acesso não chamado
   no identificador do campo.

O L0 selado do lote A autoriza somente o consumer
`compiler/stdlib/foundations/float.rs`. Derivar ranges a partir de texto dentro
da nativa duplicaria parsing, contaminaria a função pura e continuaria incapaz
de corrigir o acesso sem chamada. Esse atalho foi rejeitado.

Conforme a condição de paragem, nenhum dos dois owners adicionais foi lido em
nível L0, editado ou ressellado. Os cinco REDs próprios ficam preservados em
`float.rs`; o lote A aguarda nucleação desses owners e novo selo antes da
implementação. O SHA-256 do consumer com os REDs é
`a11e9d30a25898642f0f1e49aa8d3bbb79016b3fc14c963a785ed29761dd4203`.

## Retomada sob o selo substituto

O owner gap acima foi nucleado e o coordenador autorizou a retomada somente
sob estas entradas resselladas, verificadas antes da nova escrita:

| Entrada | SHA-256 |
|---|---|
| `00_nucleo/diagnosticos/p1293-contract-seal.json` | `6797a97c4bf75ef3d01261d5023f26412c1817b0bfbf50fc181eefd351df2c8d` |
| `00_nucleo/diagnosticos/p1293-contract-reopen-span-receipt.md` | `61070b61854bf752ca0dacb6c399ea4f97a35997198eb989469fe708244e2bdf` |
| `00_nucleo/diagnosticos/p1293-manifest.json` | `68386dd0481a58584ea0e7047e39f7e259398135a1e8a818028219afb5c14bd3` |
| `00_nucleo/prompts/compiler/stdlib/foundations/float.md` | `f28cb58e46ced68ddb29335a35e8ade4f03a7c30a573c7e79a86b1ef3bc99107` |
| `00_nucleo/prompts/compiler/eval/call_dispatch.md` | `bff7a205e96855e1df8106201d8fb55dd3102f81f8dfe47159d934255d3bda60` |
| `00_nucleo/prompts/compiler/eval/bindings/field_access.md` | `3e5b96f5d62a693cb8291f63cd656a73ba19ebd69e29446858d36e475bff4a60` |

O selo substituto expandiu a capacidade de escrita do implementador para os
três consumers 1:1 abaixo e para este recibo, sem autorizar nenhum outro path:

1. `01_core/src/compiler/stdlib/foundations/float.rs`;
2. `01_core/src/compiler/eval/call_dispatch.rs`;
3. `01_core/src/compiler/eval/bindings/field_access.rs`;
4. `00_nucleo/diagnosticos/p1293-implementation-receipt-a.md`.

O implementador continuou sem ler nem executar
`04_wiring/tests/p1293_contract.rs`, o RED protegido, o recibo do gate ou
outputs privados do testador. Nenhum L0, contrato, oracle, manifesto, selo ou
veredito foi editado por este papel.

## Reparação causal dos cinco spans

Antes do patch de reparação, sob o selo substituto, o comando próprio foi
reexecutado:

```text
cargo test -p typst-core tests_p1293_is_nan
```

Resultado RED reproduzido: `9` testes executados; `4` passaram e os `5` testes
discriminatórios de span falharam com os pares já registados na tabela de
reabertura.

O reparo mínimo foi então aplicado nos dois owners ressellados:

- `call_dispatch.rs` captura, antes da avaliação, metadados sintáticos privados
  e efémeros somente para a identidade nativa `is-nan`: span da chamada,
  posicionais explícitos, named completos e presença de spread. Depois da
  única avaliação existente, escolhe o anchor exigido e o entrega pela
  propriedade interna `Args::span` já existente;
- `field_access.rs` seleciona `access.field().span()` somente para acesso sem
  chamada de `is-nan` sobre `Value::Float`; todos os outros acessos conservam
  `access.span()`;
- a validação permanece única em `native_is_nan`, em `float.rs`, sem segunda
  fórmula, trait, registry ou parsing textual.

Não houve mudança em `entities::Args`, API pública, ordem ou quantidade de
avaliações, mensagens, defaults ou fase do pipeline. Named continua a ter a
precedência de validação da nativa; spreads mantêm o fallback agregado quando
não existe um span explícito causalmente identificável.

## GREEN, regressões e gates após a retomada

Estado medido em `2026-09-01T15:37:40-03:00`, sobre `HEAD`
`7dd25ff0e222b6c7c640d6bc7957b98f94227507` e working tree compartilhada não
commitida. Os números abaixo provêm dos três consumers com os hashes de saída
registados na secção seguinte. Alterações paralelas fora do allowlist deste
papel já existiam na working tree e não foram editadas nem atribuídas a este
implementador.

| Comando | Resultado |
|---|---|
| `cargo test -p typst-core tests_p1293_is_nan` | PASS: `9` passaram, `0` falharam |
| filtros `p1215_`, `p815_controlo_field_access_sem_chamada_intocado`, `p829b_controlo_field_access_sem_chamada_intacto`, `p829c_controlo_math_field_access_intacto`, `p731_field_access_sem_regressao`, `funcall_` | PASS: `10` testes relevantes passaram, `0` falharam |
| `cargo fmt --all -- --check` | PASS |
| `crystalline-lint --quiet --checks v5 --fail-on warning .` | PASS |
| `crystalline-lint --quiet --checks v15 --fail-on warning .` | PASS |
| `crystalline-lint --quiet --checks v26 --fail-on warning .` | PASS |
| `crystalline-lint --fix-hashes --dry-run .` | PASS: `Nothing to fix` |
| `git diff --check` | PASS |

Os cinco REDs próprios passaram sem alterar os textos já corretos:

| Família | Range GREEN |
|---|---:|
| static missing | `0..14` |
| static extra | `18..21` |
| bound named | `20..31` |
| static cast | `13..16` |
| bound access sem chamada | `13..19` |

### Hashes de saída após formatação

| Consumer | SHA-256 |
|---|---|
| `01_core/src/compiler/stdlib/foundations/float.rs` | `b47b42e83b0aa56c07af41dab207ea1dd886a1f6f75d5b0f6ff54a811ad31e5c` |
| `01_core/src/compiler/eval/call_dispatch.rs` | `ceb021c1f9ec0edc5480005e61dc0cb24dcfd8f2ed0efc6e4c8c1499afeb54d5` |
| `01_core/src/compiler/eval/bindings/field_access.rs` | `7873e47635ad2e99df9132bae9b13472581f5026a3f3a671f2f48de8a3b15497` |

## Limite final do lote A

Este papel entrega apenas o candidato GREEN próprio do lote A para
revalidação independente. Não alega ter executado o contrato protegido, não
emite veredito e não avança aos lotes B-D.

## Revalidação independente do coordenador

O coordenador, autoridade distinta do implementador e autorizada a executar o
oráculo protegido sem o expor ao produto, revalidou o lote em
`2026-09-01T15:41:14-03:00`. Estado: `HEAD`
`7dd25ff0e222b6c7c640d6bc7957b98f94227507`, working tree não commitida com
os `18` caminhos e o stat integral registados pelo comando
`git diff HEAD --stat`; binário candidato `target/release/typst` SHA-256
`865bdf085082673e3875f29a31934fa667800f27e9d221a4eed28154163e4a81`.

| Comando | Resultado independente |
|---|---|
| `cargo build --release` | PASS |
| `cargo test -q -p typst-wiring --test p1293_contract -- --exact p1293_a_static_bound_values_and_closed_errors --nocapture` | PASS: `1` passou, `0` falhou, `8` filtrados |

O teste A protegido cobre conjuntamente valores, identidade pública, formas
estática e ligada, argumentos fechados, mensagens e os cinco spans. Portanto o
checkpoint independente do lote A é `APPROVED`, limitado a A. B-D continuam
fora deste recibo e não recebem aprovação implícita.
