# Passo 1401 — contexto `eval --in` e recortes `selector.before/after`

## Objetivo

Fechar três lacunas públicas medidas pelo P1400:

1. aceitar `typst eval --in INPUT EXPRESSION` e avaliar a expressão contra o
   documento compilado indicado por `INPUT`;
2. materializar `selector.before(end, inclusive: true)`;
3. materializar `selector.after(start, inclusive: true)`.

O passo não cobre `eval --target`, outros argumentos comuns da CLI, as 44
rotas `html.*`, as demais 22 rotas `math.*` inconclusivas nem os outros
resultados do P1400. `--in` serve aqui como capacidade pública por si e como
pré-condição para voltar a observar `outline.entry.body`; não autoriza
implementar esse membro neste passo.

## Medição anterior à decisão

Estado medido em `2026-09-14T20:59:32-03:00`:

- predecessor causal: HEAD
  `4c1d7bb5ca13a0a40e2f423a7de9c00089da4c20`;
- árvore: `06a9df79e8aa9b3bb199cdc3602ff1ed5ca913c7`;
- working tree limpa;
- vanila ratificado:
  `586e1bd43fae6c9a973218163d3165c53ab8d16d`;
- binário vanila `lab/typst-original/target/release/typst`, SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- binário cristalino `target/release/typst`, SHA-256
  `508842d9e0f3cf7684d1acea4f3850c616afdf38e3aa0fac1c0ab68d7e026fcc`;
- resultado P1400, SHA-256
  `9da156aafbe572a0680f14a6c4b93aeff2b93ef63dde7bca6056a787feb62542`;
- relatório P1400, SHA-256
  `a0829f6771b9583b7dfee50794c3ae6b16d472564c9da3f55499fa34219b14a0`.

### `eval --in`

O P1400 executou, nas três ordens, a construção:

```text
typst eval --format raw --in fixture.typ \
  'repr(query(outline.entry).first())'
```

O vanila compilou a fixture e devolveu uma instância pública `entry(...)`,
exit 0. O cristalino terminou no parser de argumentos, exit 2:

```text
error: unexpected argument '--in' found
Usage: typst eval --format <FORMAT> <EXPRESSION>
```

A fonte pinada confirma a intenção: `typst-cli/src/args.rs:195-224` declara
`EvalCommand::in: Option<Input>` e `typst-cli/src/eval.rs:24-89` compila o
documento e avalia a expressão com o introspector do resultado. Logo não basta
aceitar a flag e ignorar o ficheiro.

No cristalino, `02_shell/src/cli.rs:439-453,511-519,599-606` não transporta
input em `EvalArgs`/`EvalIntent`; `04_wiring/src/main.rs:844-887` sempre cria
`SystemWorld::for_eval(".")`; e
`03_infra/src/pipeline.rs:75-89` chama avaliação isolada sem introspector do
documento. A ausência atravessa L2, L4, L3 e a entrada pública de avaliação
L1.

Classificação: a opção, diagnóstico e exit são `PUBLIC_CLI`; `query(...)`
contra o documento é semântica da linguagem. Inferência: o bloqueio observado
é de fio contextual, não de parsing Typst. Refutação: depois de transportar
`--in`, uma fixture compilável continuar sem fornecer suas ocorrências à
expressão.

### `selector.before` e `selector.after`

O P1400 demonstrou bilateralmente que `selector(heading)` é construível. O
vanila produziu:

```text
repr(selector(heading).before(selector(figure)))
  -> heading.before(figure)
repr(selector(heading).after(selector(figure)))
  -> heading.after(figure)
```

O cristalino respondeu, respectivamente, `type selector has no method
\`before\`` e `type selector has no method \`after\``. Portanto as duas formas
públicas estão ausentes; o agrupamento P1400 como
`UNKNOWN(UNSUPPORTED_CONSTRUCTION:INSTANCE_INVOCATION)` foi conservador, mas a
construção da instância não falhou.

Uma medição focal adicional no mesmo estado, com fixture compilável contendo
`#outline()`, numbering e uma heading, confirmou no vanila:

- `before(boundary)` omite `inclusive` na representação e inclui o primeiro
  match da fronteira por defeito;
- `before(boundary, inclusive: false)` exclui esse match;
- `after(boundary)` inclui o primeiro match da fronteira por defeito;
- `after(boundary, inclusive: false)` começa depois desse match;
- a fonte pinada em `foundations/selector.rs:204-242` fixa default `true`;
- `introspection/introspector.rs:246-274` usa o primeiro match da fronteira e
  conserva a seleção inteira quando a fronteira não tem match;
- as variantes são de introspecção e não casam show rules
  (`foundations/selector.rs:145-153`).

No cristalino, `01_core/src/entities/selector.rs:28-61` não possui variantes
`Before`/`After`; `compiler/introspect.rs:365-455` não as consulta; e os L0s
de `entities/selector`, `field_access` e `call_dispatch` as bloqueiam
explicitamente pelo gate ADR-0127.

Classificação: nome, argumentos, default, `repr` e conjunto ordenado retornado
por `query` são semântica/sintaxe/morfologia da linguagem (ADR-0107). `Box` ou
`Arc`, layout do enum e algoritmo de busca são mecânica livre. A inferência de
que uma fronteira pode ser decidida pela ordem canónica de `Location` seria
refutada por witness público no qual essa ordem divergir da ordem do documento;
o contrato deve exigir ordem documental, não uma representação numérica
incidental.

## Decisão — duas obrigações públicas independentes

Embora caibam no mesmo passo por compartilharem o witness P1400, `--in` e os
dois métodos têm owners e gates separados. Falha de um lote não permite
declarar o outro concluído.

### A. Contrato público de `eval --in`

Após confirmação humana, a família de L0s deve legitimar:

```rust
pub struct EvalIntent {
    // campos vigentes
    pub input: Option<PathBuf>,
}
```

- `typst eval EXPRESSION --in INPUT` e a ordem equivalente com a opção antes
  da expressão são aceitas;
- ausência de `--in` preserva integralmente a avaliação isolada vigente;
- `--in PATH` constrói o mesmo `SystemWorld` paginado do documento, compila o
  main até obter introspecção estável e só então avalia `EXPRESSION` com esse
  introspector;
- imports, packages, fontes, `sys.inputs`, features e CA usam o mesmo world da
  compilação; não se cria um segundo universo sintético para a expressão;
- `--in -` lê o documento de stdin uma única vez e mantém a expressão vinda do
  argumento; stdin não pode alimentar ambos ambiguamente;
- falha de leitura ou compilação impede a avaliação da expressão e preserva
  diagnóstico público, stderr e exit não-zero;
- `--in` não ativa HTML, não adiciona `--target` e usa target paginado neste
  recorte;
- a CLI não aceita a flag de forma decorativa: `query(...)`, `here()` e membros
  contextuais têm de observar o documento indicado;
- não expor introspector, módulos ou detalhes de pipeline em L2.

O desenho L3/L1 deve oferecer uma entrada explícita para avaliar a expressão
com um introspector já materializado. Se isso exigir nova assinatura pública
entre crates, ela deve aparecer no L0 antes do código. Não usar estado global,
cache mutável estático nem segunda compilação oculta por probe.

### B. Contrato público de `selector.before/after`

Após confirmação humana, `Selector` ganha duas variantes fechadas. A mecânica
concreta pode usar a indirection já adotada pelo enum, mas o significado é:

```rust
Before {
    base: Box<Selector>,
    end: Box<Selector>,
    inclusive: bool,
}
After {
    base: Box<Selector>,
    start: Box<Selector>,
    inclusive: bool,
}
```

- formas ligadas:
  `base.before(end, inclusive: true)` e
  `base.after(start, inclusive: true)`;
- formas não ligadas do valor-tipo:
  `selector.before(base, end, inclusive: true)` e
  `selector.after(base, start, inclusive: true)`;
- `base` e fronteira aceitam somente valores convertíveis no contrato vigente
  de selector locatável; tipo inválido e cardinalidade errada falham, nunca
  retornam selector vazio;
- exatamente uma fronteira positional; `inclusive` é named, bool e default
  `true`; positional excedente e named desconhecido falham;
- `query(Before)` parte de `query(base)`, encontra o primeiro match de `end` em
  ordem documental e conserva somente ocorrências anteriores; inclui a
  ocorrência fronteira quando ela também pertence a `base` e `inclusive` é
  true;
- `query(After)` é simétrico a partir do primeiro match de `start`; inclui a
  ocorrência fronteira quando aplicável e `inclusive` é true;
- fronteira sem match conserva `query(base)` integralmente;
- os resultados preservam ordem documental e não duplicam `Location`;
- `repr` omite `inclusive: true` e escreve literalmente
  `, inclusive: false` quando falso;
- igualdade e hash incluem discriminante, `base`, fronteira e `inclusive`;
- `Before` e `After` permanecem deliberadamente não suportados como seletores
  de show rule; tentativa nesse contexto produz o diagnóstico vigente de
  selector não suportado, nunca falso positivo silencioso;
- consumers fechados — introspecção, repr, igualdade semântica, detecção de
  elemento, validação de locatabilidade e chave de counter — recebem braços
  explícitos; wildcard não quita exaustividade.

Não simular as variantes com `And`, `Or`, `Within` ou `Where`. Isso perderia a
fronteira, o default inclusivo e a representação pública.

## Gate L0 obrigatório — parar antes de código

As duas obrigações alteram contrato público:

- `EvalIntent` recebe campo público e a CLI ganha uma opção utilizador-face;
- `Selector` recebe duas variantes públicas e novos métodos da linguagem.

Logo incidem ADR-0127 itens 1, 2 e 4. Antes de qualquer teste ou código, editar
e auditar integralmente os seguintes owners 1:1:

### Owners de `--in`

- `00_nucleo/prompts/shell/cli.md` → `02_shell/src/cli.rs`;
- `00_nucleo/prompts/wiring.md` → `04_wiring/src/main.rs`;
- `00_nucleo/prompts/infra/pipeline.md` → `03_infra/src/pipeline.rs`;
- `00_nucleo/prompts/compiler/eval.md` → `01_core/src/compiler/eval/mod.rs`.

### Owners de `before/after`

- `00_nucleo/prompts/entities/selector.md` →
  `01_core/src/entities/selector.rs`;
- `00_nucleo/prompts/compiler/introspect.md` →
  `01_core/src/compiler/introspect.rs`;
- `00_nucleo/prompts/compiler/eval/repr.md` →
  `01_core/src/compiler/eval/repr.rs`;
- `00_nucleo/prompts/compiler/eval/bindings/field_access.md` →
  `01_core/src/compiler/eval/bindings/field_access.rs`;
- `00_nucleo/prompts/compiler/eval/bindings/value_methods.md` →
  `01_core/src/compiler/eval/bindings/value_methods.rs`;
- `00_nucleo/prompts/compiler/eval/call_dispatch.md` →
  `01_core/src/compiler/eval/call_dispatch.rs`;
- `00_nucleo/prompts/compiler/eval/operators/equality.md` →
  `01_core/src/compiler/eval/operators/equality.rs`;
- `00_nucleo/prompts/compiler/eval/selector_matching.md` →
  `01_core/src/compiler/eval/selector_matching.rs`;
- `00_nucleo/prompts/compiler/stdlib/counter.md` →
  `01_core/src/compiler/stdlib/counter.rs`.

Auditar sem edição, salvo se o código realmente mudar,
`compiler/stdlib/foundations/selector.md`: o constructor deve preservar as
novas variantes por identidade, mas seu consumer vigente já clona qualquer
`Value::Selector`. Não aumentar artificialmente o conjunto de owners.

Depois de redigir esses L0s, **PARAR** e pedir confirmação explícita do dono.
Não executar `--fix-hashes`, escrever RED, alterar L1–L4 ou aproveitar o pedido
de implementação desta sessão como confirmação retroativa: a confirmação deve
ocorrer sobre o contrato acima já visível.

## Regime segregado pós-confirmação

Usar protocolo completo, executado sem atestação de isolamento se os papéis
compartilharem este workspace:

1. autor da obrigação congela os L0s confirmados, baseline, perfis e política
   de `Unknown`;
2. autor do contrato cria manifest/contrato sem ler patch candidato;
3. autor dos oráculos mede vanila, incluindo positivos, negativos e opacos;
4. adversário cobre pelo menos flag aceita mas ignorada, expressão avaliada sem
   introspector, fronteira errada, inversão de inclusive, fronteira ausente
   retornando vazio, extrapolação de bound para unbound e show rule aceitando
   `Before/After`;
5. preseal exige mutation score `1.0` antes do código;
6. implementador e testador independente partem dos mesmos L0s selados;
7. verificador não corrige candidato nem expected e emite certificado
   proporcional.

Qualquer mudança em L0, baseline, contrato ou oráculo depois do selo invalida
a cadeia a partir da primeira fase afetada.

## Testes RED→GREEN pós-confirmação

### `--in`

- help de `eval` anuncia `--in <INPUT>`; outros subcomandos não o herdam;
- ausência de flag preserva todos os casos de eval isolado;
- fixture com headings: `query(heading)` devolve as ocorrências reais;
- fixture com outline: `query(outline.entry).first().body` é observável;
- `--in -` lê stdin uma vez;
- input inexistente, documento inválido e expressão inválida mantêm fases e
  exits distinguíveis;
- imports relativos resolvem a partir do documento, não do cwd incidental;
- repetição direta/inversa produz observável idêntico.

### `before/after`

- descoberta ligada e não ligada, `type == function`, nome e `repr`;
- chamada com default e `inclusive: true/false` explícito;
- base e fronteira coincidentes, disjuntos e fronteira sem match;
- múltiplos matches de fronteira usam o primeiro em ordem documental;
- boundary que não pertence ao base ainda corta na posição correta;
- argumentos ausentes/excedentes, `inclusive` não bool e named desconhecido;
- igualdade/hash distinguem método, fronteira e inclusive;
- query real via `eval --in` prova conteúdo e ordem, não apenas forma Rust;
- show rule com `Before/After` permanece rejeitada;
- `counter(selector.before/after)` não colide com a chave do base.

Os casos A/B independentes devem existir antes de o testador ler o patch e
devem incluir os witnesses públicos exatos do vanila ratificado.

## Limites de execução e memória

- proibir `cargo test --workspace`, testes monolíticos e execução de watch;
- executar um processo por vez;
- probes públicos: RLIMIT_AS 512 MiB, timeout 15 s e 64 KiB por stream;
- testes Rust: filtrar pelo prefixo P1401, `--test-threads=1`, teto externo de
  2 GiB e timeout de 120 s por binário;
- build release, somente se necessário: serial, teto 4 GiB, timeout 180 s;
- lotes públicos com no máximo 12 casos;
- registrar RSS máximo, swap, timeout, truncamento, comando, HEAD, árvore,
  working tree e instante;
- OOM, swap, timeout ou truncamento permanecem `Unknown`; não aumentar memória
  para converter falha em sucesso.

## Gates finais pós-confirmação

```text
cargo fmt --all -- --check
cargo test -p typst-core p1401_ -- --test-threads=1
cargo test -p typst-shell p1401_ -- --test-threads=1
cargo test -p typst-infra p1401_ -- --test-threads=1
cargo test -p typst-crystalline p1401_ -- --test-threads=1
python3 lab/parity/p1401_probe.py --validate-only
python3 lab/parity/p1401_probe.py --gate
python3 lab/parity/p1401_probe.py --run
crystalline-lint .
git diff --check
```

Filtros inexistentes ou que executem zero testes falham o gate. Testes devem
rodar serialmente e registrar quantos casos foram realmente executados.

## Critério de fecho

P1401 fecha apenas se:

- os L0s públicos tiverem confirmação humana registrada e hashes resselados;
- `eval --in` compilar a fixture e avaliar com sua introspecção real;
- `selector.before` e `selector.after` existirem nas formas ligada e não
  ligada, com default inclusivo, repr e query em paridade com o vanila;
- o gate adversarial tiver score `1.0` e nenhum negativo sobrevivente;
- as três rotas P1400 forem reexecutadas: `selector.before/after` deixam de
  `Unknown` e `outline.entry.body` deixa de falhar por ausência de `--in`;
- zero `Unknown` for promovido implicitamente;
- zero V5, V15 ou V26;
- não houver alteração fora dos owners e artefatos explicitamente selados.

O certificado cobre somente essas três obrigações e seus controles. Não
declara paridade geral de eval, selector, introspecção nem da linguagem Typst.
