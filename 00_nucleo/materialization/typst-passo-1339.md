# Passo 1339 — fechar 10 rotas públicas fundamentais ausentes

## Estado do passo

- **Tipo:** materialização semântica de paridade.
- **Regime:** protocolo completo de materialização segregada.
- **Baseline de linguagem:** vanilla ratificado `a51e02804`.
- **Baseline cristalino inicial:** HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`
  mais a working tree certificada até P1338.
- **Universo principal de origem:** matriz global P1335, 4.718 probes em quatro
  perfis.
- **Escopo:** exatamente dez probes classificados `VANILLA_ONLY` em P1335.
- **Proibição:** este passo não autoriza corrigir qualquer uma das outras 93
  rotas `VANILLA_ONLY`, as 42 extensões cristalinas, as 10 divergências de
  valor/`repr` ou as 30 divergências diagnósticas.

Este é o primeiro lote da divisão **10 + 10 + 8** das 28 rotas não-HTML e
não-`math` que permaneceram ausentes no inventário P1335. Os dois lotes
seguintes são sucessores explícitos e não podem ser antecipados aqui.

**Retificação autorizada em 2026-09-09:** após a sonda e a revisão registradas
em `diagnosticos/p1339-preflight-blocker.md` e
`diagnosticos/p1339-review-preliminary.md`, o dono autorizou corrigir a
obrigação de ângulos e continuar. `angle.deg`/`angle.rad` são conversões
`Angle → float`, não os construtores numéricos internos Rust. O texto
anterior foi preservado em
`diagnosticos/p1339-step-before-angle-retification.md`; os recibos anteriores
mantêm seus hashes e a natureza focal/histórica. As dez rotas e os demais
gates não foram reduzidos.

## Resultado obrigatório

Ao final, estas dez rotas devem existir no cristalino e coincidir com o
vanilla ratificado em presença, tipo, nome público, `repr`, chamada estática,
chamada ligada quando pertencente à linguagem, valores, morfologia dos
resultados, argumentos, ordem observável de avaliação e diagnósticos:

1. `angle.deg`
2. `angle.rad`
3. `float.inf`
4. `float.nan`
5. `float.signum`
6. `float.from-bytes`
7. `float.to-bytes`
8. `function.with`
9. `function.where`
10. `version.at`

“Existir” não significa somente fazer
`repr((type(<rota>), repr(<rota>)))` passar. Cada rota deve executar o contrato
público que o valor descoberto promete. Um stub, função de nome correto que
sempre erra, alias aproximado ou implementação que só satisfaz a probe de
inventário é falha do passo.

## Evidência precedente — não substituir por narrativa

A matriz `00_nucleo/diagnosticos/p1335-matrix-normal.json`, produzida sobre o
estado não commitado registrado no próprio artefato em 2026-09-09, mediu as
dez rotas como `VANILLA_ONLY` nos quatro perfis `default`, `html`, `a11y` e
`html+a11y`. Para cada rota, o vanilla devolveu o valor público e o cristalino
devolveu `type <tipo> does not contain field ...`.

Os valores de descoberta medidos são:

| Rota | Tipo e `repr` vanilla |
|---|---|
| `angle.deg` | `(function, "deg")` |
| `angle.rad` | `(function, "rad")` |
| `float.inf` | `(float, "float.inf")` |
| `float.nan` | `(float, "float.nan")` |
| `float.signum` | `(function, "signum")` |
| `float.from-bytes` | `(function, "from-bytes")` |
| `float.to-bytes` | `(function, "to-bytes")` |
| `function.with` | `(function, "with")` |
| `function.where` | `(function, "where")` |
| `version.at` | `(function, "at")` |

Isto prova ausência e identidade superficial, mas não prova sozinho o contrato
de chamada. A primeira fase deste passo deve medir esse contrato antes de
alterar L0 ou código.

## Fase A.0 — congelar o estado real antes da spec

Produzir `00_nucleo/diagnosticos/p1339-a0.json` antes de qualquer edição.
Registrar:

- UTC inicial e final;
- HEAD, branch e `git status --short` integral;
- `git diff HEAD --stat` e hashes de todos os arquivos tracked modificados;
- caminhos e SHA-256 dos binários vanilla e cristalino usados;
- SHA-256 da matriz P1335 e dos artefatos P1338 tomados como antecedente;
- comandos completos e canais integrais, sem resumir stderr;
- `file:line` do owner atual de descoberta, semântica, dispatch e diagnóstico
  para cada família;
- L0 vigente, consumer único e hashes antes da edição;
- confirmação mecânica de V15/V26 antes de qualquer resselo.

Se o estado da árvore não for exatamente o sucessor certificado de P1338,
parar e produzir `p1339-baseline-mismatch.md`. Não construir sobre uma mistura
não explicada de estados.

## Fase A.1 — sonda bilateral obrigatória

Criar um manifesto explícito de casos e executar primeiro no vanilla, depois
no cristalino antecedente, em ordem normal e inversa. A sonda deve cobrir no
mínimo os casos abaixo; ampliar quando a fonte pinada revelar outra fronteira
pública.

### `angle.deg` e `angle.rad`

- descoberta, `type` e `repr` das duas funções;
- forma estática sobre ângulos zero, positivos, negativos e zero negativo,
  expressos em graus e radianos; o resultado é `float` na unidade pedida;
- equivalência semântica entre `angle.deg(a)` e `a.deg()`, e entre
  `angle.rad(a)` e `a.rad()`, com receiver `Angle`, sem exigir estrutura Rust
  igual; os métodos ligados são expostos pelo vanilla ratificado;
- inteiros e floats sem unidade, incluindo `-0.0`, são casos de rejeição de
  tipo, não entradas construtoras; `x * 1deg`/`x * 1rad` constroem os
  ângulos de teste e não são equivalentes às funções públicas de conversão;
- missing, positional extra, named desconhecido e tipo inválido;
- NaN/infinito apenas se a sintaxe pública bilateral conseguir construí-los;
  caso contrário classificar como `Unknown`, nunca como sucesso.

### `float.inf`, `float.nan` e `float.signum`

- constantes com `type`, `repr`, igualdade e uso aritmético observável;
- `signum` para positivo, negativo, `+0.0`, `-0.0`, `+inf`, `-inf` e NaN;
- forma estática e ligada de `signum`, inclusive obtenção como valor quando
  permitida pelo vanilla;
- distinguir o sinal de `+0.0` e `-0.0` por observável público adequado;
- missing, extra, named e tipos coercíveis/não coercíveis, com spans.

### `float.from-bytes` e `float.to-bytes`

- binary32 e binary64;
- endian little e big;
- round-trip para zero, `-0.0`, finito positivo/negativo, infinito e NaN;
- `from-bytes` com comprimentos 4 e 8;
- rejeição de comprimentos 0, 3, 5, 7 e 9;
- `to-bytes` com `size: 4` e `size: 8`, e rejeição de outros tamanhos;
- defaults de `endian` e `size` medidos, não presumidos;
- forma estática e ligada conforme o vanilla;
- bytes retornados são observável da própria API e, portanto, devem coincidir
  exatamente; isto é a exceção onde bytes são o resultado da linguagem, não
  mecânica interna de render;
- missing, positional extra, named desconhecido, enum endian inválido e tipos
  errados, incluindo origem do diagnóstico.

### `function.with` e `function.where`

- descoberta e nomes públicos curtos;
- `function.with(f, ..args)` e `f.with(..args)` devem usar a mesma semântica;
- posicionais, named, mistura e múltiplas aplicações de `with`;
- ordem preservada entre argumentos pré-aplicados e argumentos da chamada;
- closure, nativa e element function como receivers;
- `function.where(f, ..fields)` e `f.where(..fields)` quando válidas;
- `where` aceita somente element functions e produz Selector com os campos e
  morfologia medidos;
- closure, plugin/nativa não-elemento e valor não-função devem seguir os erros
  vanilla, sem promover namespace ou executar receiver para descobri-lo;
- duplicate named, spread, avaliação com `panic` e prioridade da primeira
  falha pública;
- preservar spans/ocorrências do carrier `Args`; não reconstruir ordem a partir
  dos mapas derivados.

### `version.at`

- descoberta estática `version.at` e método ligado já existente;
- `version.at(version(...), index)` e `version(...).at(index)` devem delegar à
  mesma semântica;
- índices 0, intermediário, além do comprimento, `-1`, limite negativo e
  versão vazia;
- zero-padding positivo e comprimento explícito para índice negativo;
- mensagem exata de out-of-bounds, aridade, named e tipo inválido, com spans;
- preservar `major`, `minor`, `patch`, `repr` e display já certificados.

Produzir:

- `p1339-probe-manifest.json`;
- `p1339-vanilla-runs.json`;
- `p1339-crystalline-before-runs.json`;
- `p1339-comparison-before.json`.

Cada caso deve ser pré-classificado como `Preserved`, `Violated` ou `Unknown`.
`Unknown` em caso obrigatório bloqueia o passo. Um caso criado deliberadamente
para provar opacidade deve permanecer `Unknown`.

## Fase A.2 — leitura da fonte e classificação

Depois da sonda, registrar `file:line` da fonte vanilla pinada que explica cada
resultado. Usar como pontos de partida, sem tratá-los como linhas eternas:

- `crates/typst-library/src/layout/angle.rs`;
- `crates/typst-library/src/foundations/float.rs`;
- `crates/typst-library/src/foundations/func.rs`;
- `crates/typst-library/src/foundations/version.rs`.

Classificar explicitamente:

- presença, chamada, valor, erro e morfologia como linguagem;
- enum, macro `#[scope]`, estrutura de `Func`, organização dos módulos e passos
  internos como mecânica;
- intenção normativa a partir de documentação/fonte declarativa, sem inferi-la
  apenas porque o vanilla se comporta de certa forma;
- toda inferência acompanhada da evidência que a refutaria.

## Fase B — atualizar L0 antes do código

Auditar a cardinalidade `1 Prompt L0 ↔ 1 consumer` antes de editar. Atualizar
os owners existentes ou criar owner individual somente quando o consumer
produtivo também for individual. Não apontar código para Núcleo Tekt.

Owners mínimos a auditar:

- `compiler/stdlib/foundations/float.md` ↔
  `01_core/src/compiler/stdlib/foundations/float.rs`;
- `compiler/eval/bindings/field_access.md` ↔ seu consumer;
- `compiler/eval/call_dispatch.md` ↔ seu consumer;
- `compiler/eval/bindings/value_methods.md` ↔ seu consumer;
- `compiler/stdlib/primitives-constructors/version.md` ↔ seu consumer;
- `entities/version.md` ↔ seu consumer vigente.

Para `angle`, preferir um owner semântico próprio em
`compiler/stdlib/foundations/angle.md` somente se a sonda confirmar que a
fórmula não pertence a consumer já individualizado. Se for criado
`01_core/src/compiler/stdlib/foundations/angle.rs`, ele deve ter exatamente
esse Prompt proprietário. `field_access` pode rotear a descoberta, mas não
possuir fórmulas de ângulo.

O L0 deve declarar explicitamente:

- as dez rotas completas e somente elas;
- semântica estática/ligada compartilhada, sem segunda fórmula;
- aridade, casts, named args, defaults, erros e spans medidos;
- preservações e scope-outs dos dois lotes seguintes;
- que P1339 não fecha paridade geral de `float`, função, versão ou ângulo;
- sucessão das cláusulas antigas que hoje declaram estes membros fora de
  escopo, sem apagar o histórico normativo ainda aplicável.

Resselar somente após V15/V26 limpos. Como estas são correções de paridade e
entradas de namespace, o modo esperado é fluxo contínuo ADR-0127. Porém, se o
desenho exigir campo público em entidade, método em trait público, assinatura
pública nova, mudança de default, quebra de compatibilidade ou mudança de fase,
**parar após L0 e pedir confirmação humana antes do código**.

## Segregação de autoridades

Produzir `p1339-authority-manifest.json` antes de criar contrato ou código.
Registrar executor, ambiente, allowlist de leitura, caminhos graváveis,
contexto herdado, hashes de entrada e saída e predecessor causal para cada
papel.

| Papel | Pode ler | Pode escrever | Não pode |
|---|---|---|---|
| Autor do contrato | L0 ressellado, baseline e sondas | contrato/oráculos | ler patch candidato |
| Autor dos oráculos | L0 e contrato | fixtures e expectativas A/B | adaptar expectativas ao patch |
| Adversário | L0, contrato, baseline | registro e patches mutantes isolados | corrigir implementação |
| Implementador | L0 e contrato selados | consumers produtivos | editar contrato, baseline ou oráculos |
| Testador A/B | L0 congelado e vanilla | testes independentes | ler candidato antes do freeze |
| Verificador | artefatos selados e saídas | recibos/veredito | corrigir o material que julga |

Um mesmo nome de agente em sessões distintas não prova segregação. Se o
ambiente não conseguir aplicar capacidades/allowlists, registrar
`executado sem atestação de isolamento`; nunca declarar `segregado e atestado`.

## Fase C — contrato, oráculos e selo antes da implementação

Criar:

- `p1339-contract.json` com observáveis obrigatórios por rota;
- `p1339-positive-oracles.json`;
- `p1339-negative-oracles.json`;
- `p1339-opaque-oracles.json`;
- `p1339-mutation-registry.json`;
- `p1339-discrimination-runs.json`;
- `p1339-seal.json`.

O contrato deve rejeitar, no mínimo, mutantes reais que:

1. troquem `deg` por `rad`;
2. convertam radianos como graus ou vice-versa;
3. façam `float.inf` finito;
4. façam `float.nan` comparar igual a si próprio;
5. percam o sinal de `-0.0` em `signum`;
6. façam `signum(NaN)` retornar zero;
7. invertam little/big endian;
8. aceitem comprimento diferente de 4/8 em `from-bytes`;
9. ignorem `size` em `to-bytes`;
10. retornem array de inteiros em vez de `bytes`;
11. implementem somente a descoberta, com função stub;
12. dupliquem a fórmula entre rota estática e ligada;
13. invertam a ordem preargs/new args de `with`;
14. permitam `where` em closure não-elemento;
15. descartem campos named de `where`;
16. implementem `version.at` estático com regra distinta do ligado;
17. usem zero-padding para índice negativo fora da lista explícita;
18. aceitem named args onde o vanilla rejeita;
19. alterem uma das 18 rotas restantes do plano 10+10+8;
20. convertam erro obrigatório em `Unknown`.

Todos os mutantes válidos precisam compilar e ser rejeitados por testemunha
específica. Exigir `mutation_score = 1.0`. Mutante sobrevivente, inválido sem
substituto ou `Unknown` inesperado impede o selo.

Budget inicial: até três revisões do contrato/oráculos e no máximo duas
execuções completas antes do selo. Rodar recorte focal após cada revisão.
Duas revisões consecutivas sem ganho discriminatório exigem parar e redesenhar
o contrato; não ajustar predicados indefinidamente ao corpus.

O selo deve conter hashes do manifesto, L0, baseline, contrato, oráculos,
fixtures e registro de mutações. Depois do selo, estas entradas são somente
leitura. Mudança de qualquer entrada protegida invalida a cadeia a partir da
primeira fase afetada.

## Fase D — RED independente

Antes da implementação, executar os testes A/B e registrar RED real:

- as dez probes de descoberta falham no cristalino antecedente;
- controles positivos preexistentes continuam verdes;
- os testes não falham por erro do runner, binário errado ou fixture inválida;
- cada falha RED identifica qual obrigação ainda não existe.

Salvar `p1339-red.json`. Teste escrito depois de ler o patch não conta como
teste independente, ainda que esteja correto.

## Fase E — implementação

Implementar somente após o selo e o RED válido.

Restrições arquiteturais:

- manter L1 puro;
- reutilizar as semânticas ligadas/estáticas existentes;
- descoberta de namespace não contém fórmula de domínio;
- não criar registry reflexivo, `dyn`, mapa genérico de propriedades ou
  fallback que faça tipos futuros ganharem campos implicitamente;
- não copiar macros, structs ou mecânica do vanilla;
- preservar `Args` e suas ocorrências para ordem e spans;
- não alterar `Value`, entidade pública, trait pública, default ou fase sem
  acionar o gate humano descrito na Fase B;
- não “aproveitar” para corrigir outras rotas;
- headers `@prompt` e `@prompt-hash` devem continuar 1:1 e canônicos.

### Estratégias de redesenho autorizadas

Se o primeiro desenho não fechar os oráculos, tentar nesta ordem, registrando
hipótese e refutação:

1. **Delegação ao owner semântico existente:** adicionar descoberta estática e
   transportar receiver para a mesma função usada pela forma ligada.
2. **Free function interna no owner da família:** extrair validação/fórmula
   única e fazer as duas superfícies delegarem a ela.
3. **Módulo proprietário novo:** somente quando a lógica hoje não possui owner
   1:1 legítimo; criar Prompt e consumer individuais, mantendo o roteamento
   magro.

Não avançar para a estratégia seguinte apenas porque um teste falhou: primeiro
demonstrar por medição por que a estratégia anterior não consegue expressar o
contrato. Se duas estratégias falharem pela mesma causa, parar para revisão
arquitetural antes da terceira.

## Fase F — GREEN, ataques e rebaseline

Executar:

1. suíte focal das dez rotas;
2. testes A/B independentes normal, repetido e ordem inversa;
3. quatro perfis: `default`, `html`, `a11y`, `html+a11y`;
4. todos os mutantes válidos;
5. controles negativos e scope-outs;
6. `cargo fmt --all -- --check`;
7. `git diff --check`;
8. `cargo build --workspace --release --locked`;
9. `cargo test --workspace --release --locked --no-fail-fast`;
10. `crystalline-lint .`;
11. gates estritos V5/V15/V26;
12. matriz global P1335 reconstruída com o mesmo manifesto/universo.

Não usar somente a matriz antiga com dez linhas editadas. O rebaseline deve
executar os binários e registrar proveniência nova.

Se não houver regressão e todas as dez rotas convergirem nos quatro perfis, o
delta esperado no universo principal é:

- probes iguais em todos os perfis: `4.533 → 4.543`;
- probes com alguma divergência: `185 → 175`;
- rotas `VANILLA_ONLY`: `103 → 93`;
- células `VANILLA_ONLY`: `324 → 284`;
- células iguais: `18.220 → 18.260`;
- células divergentes totais: `652 → 612`;
- igualdade bruta: `96,5451% → 96,7571%`;
- igualdade ajustada: `97,3499% → 97,5636%`.

Esses números são **expectativa derivada**, não critério que permite fabricar
resultado. Qualquer delta distinto deve ser explicado probe a probe. Não
arredondar contagens, não somar casos do suplemento funcional ao denominador
principal e não declarar as dez rotas fechadas se alguma só coincidir em parte
dos perfis.

## Fase G — veredito e fechamento

O verificador independente produz:

- `p1339-verification.json`;
- `p1339-final-report.md`;
- `p1339-closure.json`.

Vereditos possíveis:

- `PASS_SCOPED`: dez rotas fechadas, zero regressão no universo medido, todos
  os gates e mutantes verdes, entradas seladas intactas;
- `FAIL_SCOPED`: qualquer obrigação violada ou regressão atribuível;
- `INCONCLUSIVE`: infraestrutura/oráculo insuficiente ou `Unknown`
  obrigatório, sem converter incerteza em sucesso.

O relatório deve declarar regime e grau real de atestação, papéis, capacidades,
hashes, comandos, tempos, estado Git, RED→GREEN, mutações, rebaseline e limites.
`PASS_SCOPED` atesta somente estas dez rotas no fragmento medido; não significa
paridade total de Typst, das famílias afetadas ou dos 175 probes restantes.

## Critério final de conclusão

P1339 só fecha quando:

- as dez rotas estão presentes e funcionalmente equivalentes ao vanilla;
- estática e ligada compartilham causa semântica onde ambas existem;
- todos os casos obrigatórios estão classificados, sem `Unknown` indevido;
- mutation score é `1.0` com mutantes reais e testemunhas específicas;
- nenhum scope-out mudou;
- L0 e lineage estão ressellados com V5/V15/V26 limpos;
- workspace build/test, fmt, diff e lint passam;
- o rebaseline global reproduz ou explica integralmente o delta;
- o verificador, sem capacidade de corrigir, emite `PASS_SCOPED`.
