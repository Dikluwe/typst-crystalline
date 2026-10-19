# P1153 — revalidar e corrigir as 10 expectativas semânticas remanescentes

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN; baseline integral limpo`
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Dependência:** P1152 GREEN na working tree não commitada

## 1. Objetivo

Medir no vanilla ratificado e resolver as 10 falhas integrais restantes sem
transformar nomes históricos de testes em contrato. O lote divide-se em:

- 1 expectativa de `int(float)`;
- 3 expectativas P269 com named args radiais em underscore;
- 6 expectativas P273 que constroem gradients com um único stop.

A hipótese inicial é que as 10 são testes obsoletos, não defeitos de produção.
Essa hipótese deve ser confirmada por sondas de linguagem antes de qualquer
edição. Se uma sonda a refutar, atualizar primeiro o L0 owner e classificar o
gate ADR-0127.

## 2. Proveniência

- Hora da auditoria: `2026-08-25T07:08:17-03:00`.
- HEAD: `1e4a98e615f58348ad28065dfd1f2b17c9fcd6fc`.
- Estado: working tree não commitada dos lotes P1149–P1152, a preservar.
- Entrada pós-P1152: **5.213 aprovados, 10 falhas, zero ignorados**.

Antes de executar, rebaselinear e registrar:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
cargo test -p typst-core --lib
```

Não atribuir a P1153 alterações anteriores da working tree.

## 3. Medição estática anterior à decisão

### 3.1 `int(float)`

- Vanilla ratificado,
  `lab/typst-original/crates/typst-library/src/foundations/int.rs:83-105`:
  a documentação pública inclui `int(2.7)` e declara truncamento para zero.
- A conversão em `int.rs:448-469` aceita `f64` via
  `convert_float_to_int`.
- L0 vigente `compiler/stdlib/foundations/cast.md` exige explicitamente
  `int(3.7) -> 3`.
- Produção cristalina `compiler/stdlib/foundations/cast.rs:73-80` já implementa
  truncamento e overflow.
- O único RED, `native_int_float_retorna_err`, ainda espera erro.

Classificação provisória: expectativa de teste revogada. O que a refutaria:
os dois binários vanilla rejeitarem `int(3.7)` ou produzirem valor distinto de
`3`.

### 3.2 Named args focais do radial

- Vanilla ratificado,
  `visualize/gradient.rs:332-451`, expõe `focal-center:` e `focal-radius:`;
  os identificadores Rust com underscore são mecânica do macro, não sintaxe.
- L0 vigente `compiler/stdlib/gradients.md`, anotação P1145, mede hífen como
  nome público e underscore como argumento inesperado.
- Produção cristalina `compiler/stdlib/gradients.rs:784-858` aceita somente
  `focal-center` e `focal-radius`.
- Os três testes P269 inserem `focal_center`/`focal_radius` diretamente em
  `Args.named` e fazem `unwrap()`.

Classificação provisória: os três testes exercem grafia não pública. O que a
refutaria: vanilla aceitar underscore como alias público ou rejeitar hífen.

### 3.3 Quantidade mínima de gradient stops

- Vanilla ratificado `visualize/gradient.rs:307-320`, `:414-421` e `:511-518`
  rejeita menos de dois stops nas três variantes.
- L0 `compiler/stdlib/gradients.md`, anotação P1145, registra mínimo de dois e
  supera as cláusulas históricas anteriores que ainda dizem “pelo menos 1”.
- Produção cristalina valida `stops.len() < 2` antes de `relative` em Linear,
  Radial e Conic.
- Os seis testes P273 fornecem um único stop e esperam construção bem-sucedida.

Classificação provisória: fixtures P273 obsoletas; a validação de aridade tem
precedência legítima. O que a refutaria: qualquer constructor vanilla aceitar
um stop, ou `relative` modificar a quantidade mínima.

## 4. Sondas obrigatórias no vanilla

Executar o mesmo documento em:

```text
lab/typst-original/target/release/typst
/usr/local/bin/typst
```

Confirmar HEAD e proveniência; não usar `./target/release/typst` como vanilla.
As sondas devem observar linguagem, não representação Rust:

1. `int(3.7)` e limites relevantes: resultado, tipo e diagnóstico de overflow;
2. `gradient.radial(red, blue, focal-center: ..., focal-radius: ...)`;
3. as mesmas chamadas com `focal_center:` e `focal_radius:`;
4. `gradient.linear(red, relative: self/parent/auto)`;
5. `gradient.radial(red, relative: parent)`;
6. `gradient.conic(red, relative: self)`;
7. controles positivos equivalentes com dois stops para as três variantes.

Registrar stdout/stderr, exit status, hash pinado, hora e ficheiro exato da
sonda. Para erros, a mensagem é observável e pode ser comparada; para valores,
comparar semântica e morfologia, não bytes internos.

## 5. Decisão após as sondas

Se as duas instalações vanilla coincidirem com a medição estática:

- renomear `native_int_float_retorna_err` para refletir truncamento e esperar
  `Value::Int(3)`;
- mudar os três testes P269 para `focal-center`/`focal-radius`;
- adicionar um segundo stop aos seis testes P273, preservando exatamente a
  intenção de testar `relative` e não a aridade do constructor;
- manter ou criar controles negativos separados para underscore e um stop,
  somente se ainda não houver cobertura equivalente;
- não alterar produção nem entidade.

Não “corrigir” o parser para aceitar underscore: isso criaria uma extensão de
compatibilidade não medida. Não reduzir o mínimo para um stop: isso contrariaria
o vanilla e a anotação P1145.

Se qualquer sonda divergir, parar a fatia correspondente, registrar a
contradição e atualizar primeiro o L0 owner. Não misturar a correção confirmada
de uma fatia com uma decisão ainda aberta de outra.

## 6. Nucleação e ADR-0127

L0s owners auditados:

- `00_nucleo/prompts/compiler/stdlib/foundations/cast.md`;
- `00_nucleo/prompts/compiler/stdlib/gradients.md`;
- `00_nucleo/prompts/entities/gradient.md` somente se a entidade for tocada.

Com as sondas confirmatórias, o reparo é exclusivamente test-only e já está
legitimado pelos L0s vigentes: fluxo contínuo RED→GREEN, sem gate.

Parar no gate ADR-0127 antes de:

- mudar assinatura ou tipo público Rust;
- aceitar um novo alias público como underscore;
- mudar o mínimo/default público dos constructors;
- alterar o default de `relative`, `space` ou geometria radial;
- mover comportamento entre eval, layout ou render;
- introduzir quebra de compatibilidade.

Uma correção interna de paridade medida que não caia nessas classes exige L0
primeiro e resselo, mas não paragem. Em dúvida sobre a classe, parar.

## 7. RED→GREEN por sublote

### A — `int(float)` (1)

1. Correr isoladamente o teste RED e guardar diagnóstico.
2. Executar/registrar a sonda vanilla.
3. Corrigir nome e expectativa para truncamento a zero.
4. Cobrir pelo menos positivo fracionário; preservar testes existentes de
   bool, string, base e overflow.
5. Confirmar GREEN do módulo de cast.

### B — radial focal names (3)

1. Confirmar os três REDs e o argumento inesperado real.
2. Executar sondas hífen/underscore nos dois vanilla.
3. Trocar somente as keys dos três fixtures para hífen.
4. Confirmar valores de focal center/radius e defaults existentes.
5. Confirmar que underscore continua rejeitado por teste negativo existente
   ou novo, se necessário.

### C — relative com stops válidos (6)

1. Confirmar que todos falham primeiro por mínimo de stops.
2. Executar sondas de um e dois stops.
3. Acrescentar um segundo `Value::Color` a cada fixture positiva.
4. Preservar assertions de `Self_`, `Parent` e `None/auto` sem relaxamento.
5. Manter o teste de `relative` inválido focado; se ele usa um stop, fornecer
   dois para que alcance o parser de `relative` e verificar o diagnóstico.
6. Confirmar controles negativos próprios de zero/um stop.

## 8. Critérios de aceitação

- as 10 falhas nominais ficam GREEN sem skip, ignore ou relaxamento;
- `int(3.7)` resulta em `3`, conforme vanilla e L0;
- radial aceita hífen e rejeita underscore, conforme linguagem medida;
- Linear/Radial/Conic continuam a exigir pelo menos dois stops;
- os testes P273 atingem e verificam `relative`, não falham antes por aridade;
- nenhum ficheiro de produção ou contrato público muda no caminho esperado;
- suite integral termina com **5.223 aprovados, zero falhas e zero ignorados**,
  se o rebaseline não alterar o total;
- nenhuma falha nova substitui uma das dez antigas.

O número 5.223 deriva do estado não commitado P1152: 5.213 + 10. Deve ser
recalculado se o rebaseline inicial mudar.

## 9. Validação

```text
cargo test -p typst-core native_int --no-fail-fast
cargo test -p typst-core p269_stdlib_radial_focal --no-fail-fast
cargo test -p typst-core p273_stdlib --no-fail-fast
cargo test -p typst-core gradient --no-fail-fast
cargo test -p typst-core --lib
cargo check --workspace
cargo build --workspace
cargo fmt --all -- --check
git diff --check
crystalline-lint .
```

Warnings preexistentes não contam como violations, mas nenhum warning novo do
lote deve ser introduzido.

## 10. Handoff

Se P1153 terminar GREEN, o baseline integral do `typst-core` fica limpo e a
sequência P1141–P1153 pode ser encerrada com um diagnóstico consolidado de
paridade/path/location. Se sobrar qualquer falha, o próximo passo deve ser
escrito a partir da lista nominal nova, sem chamar o baseline de verde.

## 11. Execução e resultado

### 11.1 Rebaseline e RED

- Hora: `2026-08-25T07:12:11-03:00`.
- HEAD: `1e4a98e615f58348ad28065dfd1f2b17c9fcd6fc`.
- Working tree não commitada P1149–P1152 preservada.
- Os 10 REDs foram reproduzidos nominalmente.
- P269 devolvia argumento inesperado para underscore; os seis P273 devolviam
  `a gradient must have at least two stops` antes de alcançar `relative`.

### 11.2 Sondas ratificadas

O mesmo conjunto de sondas foi executado em
`lab/typst-original/target/release/typst` e `/usr/local/bin/typst`. Os dois
binários coincidiram em todos os observáveis:

- `int(3.7)` devolveu `3`;
- radial com `focal-center:` e `focal-radius:` construiu o gradient e `repr`
  preservou os dois nomes com hífen;
- `focal_center:`/`focal_radius:` devolveu `unexpected argument:
  focal_center`;
- Linear/Radial/Conic com dois stops preservaram `relative: "self"` ou
  `"parent"` em `repr`;
- um stop devolveu `a gradient must have at least two stops`, com o hint para
  preencher a shape com uma única cor.

Ficheiros exatos das sondas: `/tmp/p1153-good.typ`,
`/tmp/p1153-underscore.typ` e `/tmp/p1153-one-stop.typ`. A fonte ratificada
consultada permanece `a51e02804`; o HEAD e a hora acima identificam o estado
cristalino usado na comparação.

### 11.3 Correção test-only

Somente `01_core/src/compiler/stdlib/mod.rs` foi alterado por P1153:

- `native_int_float_retorna_err` tornou-se
  `native_int_float_trunca_para_zero` e espera `Value::Int(3)`;
- os três testes positivos P269 passaram a usar nomes com hífen;
- o controle de `focal-radius > radius` também passou a usar hífen, deixando
  de passar acidentalmente por argumento desconhecido;
- os seis fixtures positivos P273 receberam um segundo stop;
- o controle de `relative` inválido também recebeu dois stops, de modo que
  agora alcança o parser que pretende testar.

Nenhum ficheiro de produção, contrato público, default ou fase do pipeline foi
alterado. O gate ADR-0127 não foi acionado.

### 11.4 GREEN e validação

- `native_int_float_trunca_para_zero`: GREEN;
- P269 focal: GREEN;
- P273 relative: 7 aprovados, zero falhas;
- suite integral: **5.223 aprovados, zero falhas, zero ignorados**.

Concluíram com sucesso:

```text
cargo test -p typst-core --lib
cargo check --workspace
cargo build --workspace
cargo fmt --all -- --check
git diff --check
crystalline-lint .
```

P1153 encerra as 10 expectativas obsoletas e deixa o baseline integral do
`typst-core` limpo. O linter terminou com exit status zero e apenas os
warnings/informações preexistentes. O próximo artefato é o diagnóstico
consolidado da sequência P1141–P1153.
