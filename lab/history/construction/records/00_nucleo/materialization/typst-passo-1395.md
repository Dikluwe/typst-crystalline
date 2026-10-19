# Passo 1395 — precedência lexical de callees bare em math

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Corrigir a lacuna de língua priorizada por P1393: uma função lexical local
chamada como `abs(1)` ou `sqrt(1)` em modo math deve sombrear o despacho
nativo de mesmo nome. O passo também fixa a fronteira observável de binding
local não-função: exibir o valor seguido dos argumentos matemáticos, ou emitir
o diagnóstico próprio para named/spread, sem inventar coerção.

Aplica-se o protocolo completo de materialização segregada porque a alteração
é semântica e o contrato precisa ser congelado antes da implementação e dos
testes independentes. A execução ocorre num workspace compartilhado; portanto
deve ser relatada como **executado sem atestação de isolamento**, nunca como
isolamento fisicamente provado.

## Medição anterior à decisão

Baseline congelado: HEAD
`dd33334ca36e2d7abf79d16b200f07691d950c59`, árvore limpa antes de P1395.
Consumers produtivo/test-only:

- `01_core/src/compiler/eval/math.rs`, identificador administrativo
  [dado administrativo removido];
- `01_core/src/compiler/eval/tests.rs`, identificador administrativo
  [dado administrativo removido].

O L0 math anterior tinha identificador administrativo
[dado administrativo removido]
e ainda pinava `a51e02804`. Antes de autorizar código, este passo corrigiu o
pin para o vanila ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d` e acrescentou a obrigação P1395.
O L0 resultante, `compiler/eval/math.md`, tem identificador administrativo
[dado administrativo removido].
Como o harness E2E puro está no consumer test-only dedicado, seu L0 também foi
atualizado antes dos testes; `compiler/eval/tests.md` resultante tem identificador administrativo
[dado administrativo removido].
Ownership permanece 1:1; nenhum Núcleo novo é necessário.

Na fonte vanila ratificada:

- `typst-eval/src/call.rs:93-109` avalia o `MathIdent` callee antes de decidir
  se é função;
- `typst-eval/src/math.rs:45-54` usa `Scopes::get_in_math`;
- `typst-library/src/foundations/scope.rs:80-92` procura primeiro nos escopos
  lexicais e só depois em `base.math`;
- `call.rs:147-162,551-604` aplica função local; para não-função exibe valor +
  parênteses, e rejeita named/spread com diagnóstico específico.

No cristalino, `math.rs:514-600` extrai o nome e entra no `match` hardcoded
antes do lookup funcional de `:1282-1337`. Assim `abs` e `sqrt` são
interceptados. As sondas válidas desta sessão, sempre seriais e sem swap,
mediram:

| Witness | Vanila | Cristalino | RSS vanila/cristalino | Tempo vanila/cristalino |
|---|---|---|---:|---:|
| closure local `abs` | `LOCAL-ABS` | `|1|` | 31.636/39.104 KiB | 0,61/0,26 s |
| closure local `sqrt` | `LOCAL-SQRT` | raiz nativa | 31.428/39.272 KiB | 0,41/0,09 s |

Fixtures: [dado administrativo removido]
e [dado administrativo removido].
Uma tentativa instrumental vanila com `--color` na posição errada terminou
exit 2/RSS 10.080 KiB; foi descartada e não sustenta conclusão alguma.
Binários congelados: vanila
[dado administrativo removido];
cristalino [dado administrativo removido].

## Classificação e decisão

O que a chamada resolve e a forma textual resultante são semântica e
morfologia da língua (ADR-0107), não bytes ou estrutura Rust. A ordem explícita
na fonte e o ramo dedicado `NonFunc` mostram intenção. A hipótese generaliza
a prioridade a todo `MathIdent` bare; seria refutada por exceção nominal no
mesmo pin ou sonda bilateral contrária.

É correção interna de paridade: nenhuma assinatura pública, default, fase ou
compatibilidade muda. Segue fluxo contínuo ADR-0127: L0 primeiro, depois RED,
implementação, resselo e revalidação, sem gate humano intermediário.

## Delta autorizado

1. Em `math.rs`, antes do despacho nativo/hardcoded de callee `MathIdent`,
   consultar exclusivamente `Scopes::get_local`.
2. `Value::Func` local usa o builder de Args math vigente, avalia cada argumento
   uma vez, aplica a função e mantém o contrato Content-only já existente.
3. Binding local não-função não cai em native: exibe o valor pela conversão
   pública vigente e preserva posicionais/pontuação entre parênteses; named e
   spread produzem os diagnósticos vanila descritos no L0.
4. Ausência de binding local conserva integralmente `abs`, `sqrt` e todos os
   outros branches atuais.
5. R3 revoga a alteração de `eval/tests.rs`: o oráculo independente fica em
   `[artefato laboratorial removido]`, fora do lib-test monolítico.
6. Resselar somente a relação `compiler/eval/math.md` ↔ `math.rs`.

Fora do escopo: parser/AST, `Scopes`, Args público, field access, namespace
qualificado, layout, stdlib, funções nativas, assinatura pública, outro owner,
rebuild release completo, matriz completa ou alegação de paridade geral. R3
autoriza apenas o rebuild incremental explicitamente limitado abaixo.

## RED, ataques e `Unknown`

Os testes devem ser escritos por autoridade independente sem ler o patch e
falhar no baseline ao menos nos dois witnesses de closure. Positivos:
precedência local e controles sem binding. Negativos: lookup depois do
hardcode; correção só de um nome; `get` global em vez de `get_local`; avaliação
duplicada; queda em native para não-função; descarte de argumentos; aceitação
silenciosa de named/spread; regressão do caminho nativo. O adversário deve
atingir `mutation_score = 1.0`; sobrevivente impede selo.

Identidade divergente, parse/build inconclusivo, timeout, OOM, swap, ausência
de output ou evidência insuficiente permanecem `Unknown`; `Unknown` nunca
conta como verde. Diagnóstico é observável mecânico apenas porque a mensagem é
a superfície pública neste caso.

## Disciplina de memória e gates

Tudo é serial (`--test-threads=1`). Primeiro executar apenas testes `p1395_`
sob 512 MiB virtuais e 120 s. Se o próprio rustc/linker provar que 512 MiB é
insuficiente, registrar a falha de recurso e repetir uma única vez sob 1 GiB;
o teto absoluto é 2 GiB. Não elevar limite para corrigir falha semântica.

```bash
bash -lc 'ulimit -v 524288; exec timeout 120s /usr/bin/time -v cargo test -p typst-core p1395_ -- --nocapture --test-threads=1'
```

Depois do RED e GREEN focal: repetir/inverter casos, executar bilateralmente
somente as duas fixtures sob 512 MiB/60 s por processo, `cargo check -p
typst-core` sob no máximo 2 GiB/180 s e `crystalline-lint .` sob 2 GiB/120 s.
Não rodar testes workspace nem processos em paralelo. Registrar comando, HEAD,
`git diff HEAD --stat`, instante, exit, duração, RSS e swap. Qualquer swap ou
RSS acima de 85% do teto interrompe a sequência; duas falhas com mesma causa
reabrem o desenho.

### Revisão causal R1 — duas falhas instrumentais antes do RED

Antes de existir implementação candidata, o autor independente dos oráculos
escreveu os testes P1395. A tentativa focal inicial sob 512 MiB não chegou a
executá-los: o `rustc` abortou, com RSS máximo de 153.388 KiB e zero swap. A
única repetição autorizada sob 1 GiB também abortou antes da suíte, ao tentar
alocar 122.973.653 bytes; RSS máximo 220.020 KiB e zero swap. Portanto ainda
**não existe RED semântico** e nenhum selo pode ser emitido a partir dessas
duas execuções.

Esta R1 sucede o passo R0 identificador administrativo
[dado administrativo removido]
e o contrato R0 identificador administrativo
[dado administrativo removido].
A causa observada é limite de recurso do processo de compilação, não falha dos
asserts e não motivo para afrouxar qualquer expectativa de língua.

Autoriza-se **exatamente uma terceira e última tentativa** do filtro focal
`p1395_`, serial, sob o teto absoluto já congelado de 2 GiB e timeout de 180 s:

```bash
bash -lc 'ulimit -v 2097152; exec timeout 180s /usr/bin/time -v cargo test -p typst-core p1395_ -- --nocapture --test-threads=1'
```

Essa autorização não inclui outro filtro, suíte completa, teste workspace,
paralelismo ou nova repetição. Se a terceira tentativa não executar os testes
até obter um vetor RED semanticamente discriminante, a execução encerra e o
desenho é reaberto; é proibido elevar o teto novamente ou promover o resultado
a `Unknown` satisfatório. Os hashes finais do recibo de oráculos e de
`eval/tests.rs` ainda estão pendentes enquanto o autor independente conclui;
uma R2 deve registrá-los antes do selo. Até lá, nenhum hash transitório da
suíte constitui entrada protegida final.

### Revisão causal R2 — reabertura com `RLIMIT_DATA`

O caminho R1 baseado em `RLIMIT_AS` não produziu RED semântico. A tentativa de
substituí-lo por cgroup transitório também não é contenção disponível neste
ambiente: `cgroup.subtree_control` devolveu `EBUSY` e `memory.max` devolveu
`Permission denied`. Essa tentativa é descartada; não prova limite de memória
e não autoriza execução.

Uma sonda nova mediu outro mecanismo do kernel:

```bash
bash -lc 'ulimit -d 65536; exec /usr/bin/time -v python3 -c "bytearray(128*1024*1024)"'
```

Ela terminou com `MemoryError`, exit 1, RSS máximo 30.496 KiB e zero swap.
Logo `RLIMIT_DATA` é efetivo para heap/dados anônimos neste host sem impor o
limite global sobre `mmap` file-backed que impediu o `rustc` sob `RLIMIT_AS`.
Isto reabre o desenho com mecanismo materialmente diferente; não é quarta
repetição do mesmo limite nem elevação do teto de 2 GiB.

Ficam agora protegidos, antes de qualquer candidato:

- `01_core/src/compiler/eval/tests.rs`, identificador administrativo
  [dado administrativo removido];
- relatório consolidado da limpeza;
- relatório consolidado da limpeza.

Autoriza-se uma única tentativa de obter o RED, sem `RLIMIT_AS`, com
`RLIMIT_DATA` de 2 GiB, uma só job Cargo, timeout de 180 s e testes P1395
seriais:

```bash
bash -lc 'ulimit -d 2097152; export CARGO_BUILD_JOBS=1; exec timeout 180s /usr/bin/time -v cargo test -p typst-core p1395_ -- --nocapture --test-threads=1'
```

Registrar exit, duração, RSS máximo e swap. O teste não recebe crédito se
houver swap ou RSS acima de 1.782.579 KiB (85% de 2 GiB); qualquer desses
sinais exige aborto. Se essa única tentativa não chegar a um RED semântico
discriminante, P1395 fica bloqueado e nenhuma nova contenção, repetição ou
elevação de teto é autorizada sem nova decisão humana.

### Revisão arquitetural R3 — oráculo CLI sem lib-test monolítico

O dono autorizou explicitamente redesenhar o teste guloso. A medição causal
mostrou que o filtro `cargo test ... p1395_` só é aplicado **depois** de o Cargo
compilar o lib-test monolítico: `01_core/src/compiler/eval/tests.rs` ultrapassa
20 mil linhas, o `rustc` aproxima-se de 1,6 GiB e zero testes chegam a ser
executados. A causa não é loop, recursão nem fixture gigante; é a compilação do
harness monolítico antes do filtro. Repetir esse caminho não agrega evidência.

R3 invalida como oráculos os artefatos e hashes R2 ligados a
`eval/tests.rs`. A cláusula P1395 foi removida de
`00_nucleo/prompts/compiler/eval/tests.md`, restaurado exatamente ao baseline
HEAD, identificador administrativo
[dado administrativo removido].
O próprio `eval/tests.rs` deve voltar byte-a-byte ao baseline identificador administrativo
[dado administrativo removido]
antes do selo, sem mudança produtiva ou de ownership. O L0 math P1395
permanece autoritativo e inalterado.

O novo autor independente de oráculos pode escrever somente
`[artefato laboratorial removido]` e seu recibo R3. O script usa apenas standard
library Python, recebe `--binary PATH` e `--output PATH`, cria e remove fixtures
e outputs num `TemporaryDirectory`, executa um processo por vez com timeout
interno, extrai texto com `pdftotext` e retorna exit 0 somente se todos os casos
contratados forem preservados. Deve registrar expected/actual, exit, stderr,
duração e razão pública por caso; ferramenta ausente, timeout, crash ou output
ilegível é `Unknown` fail-closed, nunca sucesso.

O corpus cobre closures locais `abs`/`sqrt`, não-função local posicional,
diagnósticos named/spread, controles nativos sem binding e avaliação única na
medida observável por CLI. Expected vem do vanila ratificado e do L0, não do
binário candidato. O RED usa exclusivamente o binário cristalino baseline já
existente, sob `RLIMIT_AS` 512 MiB/60 s; o script não invoca Cargo nem rustc:

```bash
bash -lc 'ulimit -v 524288; exec timeout 60s /usr/bin/time -v python3 [artefato laboratorial removido] --binary target/release/typst --output /tmp/p1395-red.json'
```

O implementador recebe o oráculo R3 selado e pode editar somente `math.rs`.
Depois do candidato, reconstruir incrementalmente o binário release com uma
job, `RLIMIT_DATA` de 2 GiB e timeout 240 s:

```bash
bash -lc 'ulimit -d 2097152; export CARGO_BUILD_JOBS=1; exec timeout 240s /usr/bin/time -v cargo build --release --bin typst'
```

Abortar se houver swap ou RSS acima de 1.782.579 KiB. O GREEN repete o mesmo
script/oráculo contra o binário reconstruído, sob `RLIMIT_AS` 512 MiB/60 s,
sem recompilar testes. Os hashes finais do script, recibo R3 e novo catálogo
adversarial ficam pendentes até seus autores concluírem; qualquer selo R2 está
inválido e não pode ser reutilizado.

### Revisão focal R4 — tokenização do controle `sqrt` pelo extrator

O primeiro RED CLI R3 executou com segurança em 2,28 s, RSS máximo 39.364 KiB
e zero swap. O vetor direto foi seis `Difference` e um `Preserved`, estável no
escopo executado. O único controle que deveria estar preservado mas apareceu
como diferença foi `native-sqrt-control`: `pdftotext -layout` extrai a
morfologia nativa como `√ 1`, enquanto o expected instrumental dizia `√1`.

A hipótese focal é que o espaço pertence à tokenização do extrator, não a uma
diferença do produto: o próprio output contém o radical e o radicando esperados
e o caso não envolve binding local nem código candidato. Seria refutada se a
alteração afetasse qualquer outro caso, aceitasse ausência/troca do radical ou
mudasse o vetor fora desse controle.

Antes do selo, autoriza-se somente trocar o expected literal do caso
`native-sqrt-control` em `[artefato laboratorial removido]`, de `√1` para
`√ 1`, e atualizar o recibo do oráculo. Nenhuma fixture, comando, timeout,
classificador, outro expected ou caso pode mudar. Autoriza-se exatamente uma
reexecução focal do mesmo RED sob `RLIMIT_AS` 512 MiB/60 s:

```bash
bash -lc 'ulimit -v 524288; exec timeout 60s /usr/bin/time -v python3 [artefato laboratorial removido] --binary target/release/typst --output /tmp/p1395-red-r4.json'
```

O delta discriminatório esperado é de `6 Difference / 1 Preserved` para
`5 Difference / 2 Preserved`: somente o controle nativo `sqrt` muda de classe.
Qualquer outro vetor, `Unknown`, timeout, OOM, swap ou mudança além do literal
reabre o desenho e impede selo. Os hashes finais do script e recibo calibrados
continuam pendentes e devem ser protegidos numa R5 antes de qualquer candidato.

### R5 — entradas finais congeladas, contrato pronto para selo

A única reexecução R4 produziu exatamente o delta discriminatório previsto:
cinco `Difference`, dois `Preserved` e zero `Unknown`, com RSS máximo de
39.328 KiB e zero swap. Somente `native-sqrt-control` mudou para `Preserved`.
O caso de avaliação única permaneceu `NotObserved`: o caminho CLI disponível
não oferece testemunha pública suficiente sem ampliar a linguagem exercitada,
portanto ele fica explicitamente fora do denominador e não recebe crédito.

O gate adversarial possui 14 mutações aplicáveis e rejeitou as 14;
`mutation_score = 14/14 = 1.0`. Essa pontuação cobre apenas o fragmento
observável do contrato. As entradas finais, conferidas antes do candidato, são:

- script `[artefato laboratorial removido]`, identificador administrativo
  [dado administrativo removido];
- recibo relatório consolidado da limpeza;
- catálogo relatório consolidado da limpeza;
- `eval/tests.rs` restaurado ao baseline, identificador administrativo
  [dado administrativo removido];
- L0 de testes inalterado, identificador administrativo
  [dado administrativo removido];
- L0 math P1395, identificador administrativo
  [dado administrativo removido].

Com essas entradas o contrato está pronto para selo por autoridade distinta.
Após o selo, o implementador pode alterar exclusivamente `math.rs` e a
metadata de linhagem/hash necessária para essa relação 1:1. Build incremental
e GREEN seguem exatamente os comandos e limites R3. Nenhum teste monolítico,
oráculo, fixture, contrato, mutante ou outro código pode ser adaptado ao
candidato. Esta R5 não emite selo nem veredito.

### R6 — baseline direto do vanila refuta três expected instrumentais

Após a existência do candidato, uma medição direta obrigatória executou o
mesmo script contra o vanila ratificado sob `RLIMIT_AS` 512 MiB/60 s. O recibo
`/tmp/p1395-vanila-r4.json`, identificador administrativo
[dado administrativo removido],
terminou em 4,55 s, RSS máximo 32.288 KiB e zero swap. Ela refutou três
expectativas do oráculo pré-selado:

- `local-abs-non-func-positional`: actual `7(1, 𝑥)`, não `7(1, x)`;
- `local-abs-non-func-named`: `public_diagnostics` contém somente o primary
  e o hint público de escape do `:`;
- `local-abs-non-func-spread`: `public_diagnostics` contém somente o primary
  e o hint público que adiciona espaço depois de `..`.

O stderr vanila contém ainda a anotação de span `abs is not a function`, mas
sem prefixo `hint:`. O script captura somente linhas `error:`/`hint:` e o
`SourceDiagnostic` simplificado não representa essa anotação como categoria
distinta. Ela fica registrada como observável não representável e dívida fora
do escopo; P1395 não pode alegar `MATCH` diagnóstico integral.

Como uma entrada de intenção protegida mudou depois do candidato, o estado
“pronto para selo” R5 está invalidado. O novo L0 math R6 tem identificador administrativo
[dado administrativo removido].
Nenhum resultado do candidato recebe crédito antes de novo selo independente.
O autor deste contrato não leu o candidato.

Autoriza-se o autor independente do oráculo a alterar somente:

1. expected textual do posicional: `7(1, x)` → `7(1, 𝑥)`;
2. expected diagnóstico named: remover apenas a linha instrumental
   ``hint: `abs` is not a function``, preservando primary e repair-hint;
3. expected diagnóstico spread: a mesma remoção única, preservando primary e
   repair-hint;
4. atualizar somente o recibo correspondente.

Nenhum parser, capturador, fixture, comando, timeout, closure/native expected,
classificador ou código pode mudar. Se necessário, autoriza-se uma única
repetição RED contra o binário cristalino baseline **somente após confirmar**
identificador administrativo
[dado administrativo removido],
sob o mesmo limite:

```bash
bash -lc 'ulimit -v 524288; exec timeout 60s /usr/bin/time -v python3 [artefato laboratorial removido] --binary target/release/typst --output /tmp/p1395-red-r6.json'
```

Se a identidade baseline não estiver disponível, não substituir pelo candidato:
registrar `Unknown/INPUT_IDENTITY_MISMATCH` e reabrir a preparação. Depois da
calibração, uma nova revisão deve proteger os hashes do script/recibo e emitir
novo selo antes de verificar ou adaptar implementação. As obrigações principais
de closures locais e controles nativos permanecem intactas.

### R7 — retomada incremental com extensão somente temporal

Após o ajuste final R6, o rebuild incremental autorizado chegou a compilar
`typst-core`, mas o timeout externo de 240 s encerrou o comando com exit 124
durante o downstream `typst-infra`. Não houve falha semântica do core nem sinal
reportável de OOM ou swap; como o timeout encerrou também `/usr/bin/time`, RSS
e swap finais dessa tentativa são `Unknown`, não zero presumido.

Nenhuma entrada semântica, L0, contrato observável, oráculo ou mutante mudou.
Logo R7 é amendment estritamente executivo: não invalida o conteúdo de um selo
R6 independente já emitido e não cria selo se ele ainda estiver pendente.
Memória e paralelismo permanecem exatamente os mesmos.

Autoriza-se uma única retomada incremental, aproveitando os artefatos já
compilados, com `RLIMIT_DATA` de 2 GiB, uma job Cargo e timeout externo ampliado
somente de 240 para 360 s:

```bash
bash -lc 'ulimit -d 2097152; export CARGO_BUILD_JOBS=1; exec timeout 360s /usr/bin/time -v cargo build --release --bin typst'
```

Registrar exit, duração, RSS máximo e swap; abortar se houver swap ou RSS acima
de 1.782.579 KiB. Não aumentar memória, jobs ou escopo de build. Se essa única
retomada falhar ou terminar inconclusiva, P1395 fica bloqueado e exige nova
decisão humana; nenhuma repetição adicional é autorizada.

## Critérios de aceitação

- contrato, L0s, baseline, oráculos e mutantes permanecem nos hashes selados;
- oráculo CLI R3 independente e selado distingue o binário baseline em RED;
  o mesmo script passa em GREEN contra o binário incremental reconstruído;
- `abs` e `sqrt` locais produzem somente `LOCAL-ABS`/`LOCAL-SQRT`;
- não-função local coincide com a morfologia CLI medida; named/spread preservam
  primary + repair-hint, com a anotação de span explicitamente fora do crédito;
- sem binding local, a morfologia nativa é preservada;
- argumentos são avaliados uma vez, sem alteração de field/namespaced calls;
- `mutation_score = 1.0`, repetição e ordem inversa concordam;
- rebuild incremental, check aplicável e linter terminam sem violações, OOM
  ou swap;
- o verificador independente emite o veredito; este passo não o antecipa.

Este documento é Passo de Execução, não Prompt L0, selo ou certificado.
