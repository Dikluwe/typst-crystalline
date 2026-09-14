# Passo 1400 — classificação semântica das 93 rotas fora da família `format`

## Objetivo

Transformar as 93 rotas públicas fora da família `format` que aparecem
somente no inventário do vanila ratificado em casos bilaterais executáveis,
classificá-las no nível da linguagem e produzir uma fila causal por owner L0.

Este passo é exclusivamente diagnóstico sobre o produto. Pode criar harness,
fixtures e artefatos em `lab/parity/` e `00_nucleo/diagnosticos/`, mas não pode
alterar Prompt L0, Núcleo Tekt, código L1–L4, manifesto permanente da matriz ou
expectativas históricas. Nenhuma ausência nominal autoriza implementação.

A família `format`, com a raiz e 123 descendentes `format.*`, fica fora deste
passo. Ela exige
nucleação e decisão de contrato público separadas. As diferenças instrumentais
de PNG e nome de fonte PDF também são carry-over, não entradas do denominador
das 93 rotas.

## Medição anterior à decisão

Baseline observado em `2026-09-14T18:40:50-03:00`:

- HEAD `2ded754d01f36294536d0d0feeea63c31209eba9`;
- working tree limpa;
- vanila ratificado
  `586e1bd43fae6c9a973218163d3165c53ab8d16d`;
- binário vanila
  `lab/typst-original/target/release/typst`, SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- binário cristalino `target/release/typst`, SHA-256
  `508842d9e0f3cf7684d1acea4f3850c616afdf38e3aa0fac1c0ab68d7e026fcc`;
- enumerador vanila efetivamente executado pelo inventário, em
  `lab/typst-original/target/release/p1140-inventory`, SHA-256
  `11f3eab4ccfadc8239cdfc7fecdc87822238e625ff5bf4f755f3eb3b4ef8da13`;
- enumerador cristalino reconstruído do HEAD, SHA-256
  `0552b34e4b585b6869f4b453be72ebfaaddf78da1a868d6098f1f9c16597ede9`.

O inventário bilateral atual produziu SHA-256
`b1d1ed0f827df3e2ad1efdb12305c12c20fb7df772e4b0b8322545606d9c16e2`.
As quatro execuções canônicas/reversas produziram o mesmo hash
`dd1eab1f9ab7801a72ecc89236615fcb5871a47f663525dda25ae76bd032f8ae`.

### Retificação de proveniência anterior ao congelamento

Em `2026-09-14T18:59:56-03:00`, ainda antes da criação do manifesto e do
contrato, a auditoria encontrou que o hash
`6cbc7dfc6591f95e61ebe74cd9fa6de32ed23b8a203f9fbf6f231185ca3081d1`
atribuído acima ao enumerador vanila não corresponde ao executável que gerou
o inventário. O primeiro `argv` de `/tmp/p1400-current-inventory.json` nomeia
literalmente
`/repos/Antigravity/typst-crystalline/lab/typst-original/target/release/p1140-inventory`;
`sha256sum` desse ficheiro produz
`11f3eab4ccfadc8239cdfc7fecdc87822238e625ff5bf4f755f3eb3b4ef8da13`
e `stat` registra `mtime` `2026-09-14 18:37:40.576278087 -0300`. A busca
`find /tmp /repos/Antigravity/typst-crystalline -type f -name p1140-inventory`
encontrou somente esse executável; a busca textual pelo hash `6cbc7df...`
encontrou somente a declaração incorreta deste passo. Portanto não há
evidência de que `6cbc7df...` identifique outro artefato que participou da
medição, e a identidade foi corrigida para o hash do executável efetivamente
registrado no `argv`.

No instante da auditoria, o predecessor causal era HEAD
`2ded754d01f36294536d0d0feeea63c31209eba9`; `git status --short` continha
somente `?? 00_nucleo/materialization/typst-passo-1400.md` e
`git diff HEAD --stat` estava vazio porque o único ficheiro de trabalho era
novo e ainda não rastreado. O inventário tinha SHA-256
`b1d1ed0f827df3e2ad1efdb12305c12c20fb7df772e4b0b8322545606d9c16e2`
e `mtime` `2026-09-14 18:38:26.781275862 -0300`. Nenhuma lista ou contagem foi
alterada por esta retificação.

Contagens por perfil:

| Perfil | Ambos | Somente vanila | Somente cristalino |
|---|---:|---:|---:|
| `default` | 1966 | 55 | 42 |
| `html` | 2037 | 214 | 42 |
| `a11y-extras` | 1969 | 58 | 42 |
| `html+a11y` | 2040 | 217 | 42 |

As 544 células `VANILLA_ONLY` correspondem a 217 paths únicos. A família
`format` contém exatamente 124 rotas: a raiz `format` e 123 descendentes
`format.*`. Excluída essa família inteira, restam exatamente 93 rotas para
este passo:

### Cor e HTML — 46

```text
color.spot
color.spot.tint
html.area
html.audio
html.base
html.blockquote
html.canvas
html.caption
html.colgroup
html.data
html.del
html.details
html.dialog
html.embed
html.fieldset
html.form
html.frame
html.hr
html.img
html.input
html.ins
html.label
html.link
html.map
html.meta
html.meter
html.object
html.optgroup
html.option
html.output
html.progress
html.q
html.script
html.slot
html.source
html.style
html.table
html.tbody
html.td
html.textarea
html.tfoot
html.th
html.thead
html.time
html.tr
html.track
```

### Math — 31

```text
math.abs
math.accent
math.bold
math.cal
math.cases
math.display
math.frac
math.italic
math.limits
math.lr
math.mat
math.mid
math.norm
math.overbrace
math.overbracket
math.overline
math.overparen
math.overshell
math.primes
math.root
math.round
math.sans
math.scr
math.sscript
math.stretch
math.text
math.underbrace
math.underbracket
math.underparen
math.undershell
math.upright
```

### Elementos, introspecção e membros auxiliares — 16

```text
enum.item
figure.caption
footnote.entry
list.item
outline.entry
outline.entry.body
outline.entry.indented
outline.entry.inner
outline.entry.page
outline.entry.prefix
par.line
polygon.regular
raw.line
selector.after
selector.before
terms.item
```

O inventário mede presença de paths, não equivalência funcional. Logo, as 93
rotas formam o conjunto de investigação, não 93 defeitos já demonstrados.

## Decisão após a medição

Executar uma sonda pública mínima para cada rota antes de decidir se existe um
gap. A classificação deve distinguir:

| Estado | Significado |
|---|---|
| `MATCH` | a mesma forma pública existe e o observável da linguagem coincide |
| `ABSENT` | o vanila expõe a forma pública e o cristalino a rejeita por ausência |
| `PARTIAL` | operação relacionada existe, mas a forma pública ou um subconjunto obrigatório falta |
| `DIFFERENCE` | ambos executam a mesma forma, porém divergem em semântica, sintaxe, morfologia ou diagnóstico público |
| `UNKNOWN` | identidade, construção, observador, timeout ou evidência não permitem decidir |
| `SCOPE_OUT` | uma ADR ou L0 vigente exclui explicitamente a obrigação |

Uma rota alternativa não transforma um membro ausente em `MATCH`. Ela pode
demonstrar `PARTIAL` e reduzir a causa, desde que o relatório conserve o fato de
que a forma pública do vanila continua indisponível.

## Regime de materialização segregada

Usar o protocolo completo para os instrumentos e o veredito, pois os casos
produzidos por este passo poderão fundamentar alterações semânticas futuras.
O produto permanece read-only.

- **Autor da obrigação:** este passo; congela pin, conjunto de 93 rotas,
  observáveis e política de `Unknown`.
- **Autor do contrato:** escreve manifesto e contrato sem ler implementação
  candidata futura e sem classificar pelo resultado atual do cristalino.
- **Autor dos oráculos:** constrói witnesses positivos, negativos e opacos a
  partir do vanila e do contrato; não adapta expected ao candidato.
- **Adversário:** cria mutações do classificador e do corpus; não corrige harness
  nem escolhe implementação.
- **Implementador do instrumento:** escreve somente harness/fixtures de
  `lab/parity/`; não edita manifesto, contrato, oráculos ou mutantes selados.
- **Verificador:** executa os artefatos protegidos, confere identidades e produz
  recibo, ledger e relatório; não corrige o que verifica.

Se as autoridades compartilharem este workspace ou contexto não isolado, o
relatório deve dizer **“executado sem atestação de isolamento”**. Nomes de
papéis, hashes e sessões distintas não bastam para alegar isolamento material.

## Fase A — congelar manifesto e contrato

Antes de escrever o runner candidato, produzir:

1. `00_nucleo/diagnosticos/p1400-manifest.json`;
2. `00_nucleo/diagnosticos/p1400-contract.json`;
3. `00_nucleo/diagnosticos/p1400-oracles.json`;
4. `00_nucleo/diagnosticos/p1400-mutants.json`;
5. `00_nucleo/diagnosticos/p1400-preseal.json`.

O manifesto registra pelo menos:

- versão do protocolo;
- hash deste passo;
- HEAD, árvore, status e instante;
- pin completo e hashes dos produtos;
- hashes dos dois enumeradores e do inventário regenerado;
- lista literal e cardinalidade das 93 rotas;
- perfis válidos e features exatas;
- observáveis obrigatórios;
- política de `Unknown`;
- allowlists de leitura/escrita e contexto herdado por papel;
- limites de memória, tempo, streams e revisões.

Qualquer mudança de passo, pin, lista de rotas, contrato ou oráculos após o
preseal invalida a cadeia a partir da primeira fase afetada.

## Fase B — regenerar e validar o inventário

Reconstruir os enumeradores somente se suas identidades não corresponderem às
fontes declaradas. Executar o inventário P1357 nos quatro perfis, duas vezes na
ordem canônica e duas vezes na ordem reversa, mantendo brutos em `/tmp`.

O gate de entrada exige simultaneamente:

- 544 células `VANILLA_ONLY`;
- 217 paths únicos `VANILLA_ONLY`;
- 124 paths da família `format` — a raiz `format` e 123 descendentes
  `format.*`;
- 93 paths fora da família `format` literalmente iguais à lista congelada
  acima;
- quatro hashes de determinismo idênticos;
- identidades dos produtos iguais às declaradas no manifesto.

Se qualquer cardinalidade ou path mudar, não ajustar a lista silenciosamente.
Emitir `Unknown(BASELINE_DRIFT)`, registrar o delta nominal e reabrir o escopo
antes de escrever novos expected.

## Fase C — projetar witnesses públicos

Cada rota recebe um caso primário e, quando aplicável, controles de fronteira.
Nenhum caso pode depender de reflexão Rust, endereço, ordem de `HashMap`, bytes
internos do PDF ou presença textual no código-fonte.

Para cada caso registrar:

- ID estável `P1400-<família>-NNN`;
- path investigado e perfil mínimo;
- fonte Typst mínima ou expressão de CLI;
- comando bilateral completo;
- resultado bruto de stdout, stderr, exit e artefato quando houver;
- projeção pública comparada;
- justificativa ADR-0107 do observável;
- classificação e `reason_code`;
- controle que separa ausência da rota de falha do parser, target ou fixture;
- hipótese causal marcada como inferência e condição que a refutaria.

Regras por família:

1. **`math.*`:** testar o acesso público qualificado e uma chamada válida
   mínima. Executar também a forma math/bare equivalente quando existente.
   Sucesso da forma bare só pode produzir `PARTIAL`, nunca quitar a ausência do
   membro qualificado. Preservar os fechamentos P1395–P1397 como controles.
2. **`html.*`:** usar somente perfil com feature e target HTML corretos. Cada
   witness deve distinguir tag/função ausente de argumento inválido, conteúdo
   proibido ou target errado. Não extrapolar de uma tag para as outras 43.
3. **`color.spot/tint`:** separar existência do constructor, validade de args,
   representação e comportamento do método. Não fundir com `luma` nem com
   serialização de cor.
4. **membros de elementos/introspecção:** construir uma instância pública
   válida antes do acesso. Se a construção depender de contexto ou layout não
   observável por `eval`, usar documento mínimo e projeção pública apropriada;
   falha de construção é `UNKNOWN`, não ausência do membro.

Executar em lotes de no máximo 12 rotas, serialmente. Cada lote contém um
controle positivo conhecido e, quando disponível, um sentinela de diferença.
Resultados de irmãos não executados não recebem crédito.

## Fase D — gate discriminatório

O adversário deve incluir ao menos mutações que:

1. removem uma rota do denominador;
2. acrescentam a raiz `format` ou um descendente `format.*` ao escopo;
3. tratam acesso alternativo como `MATCH` da rota ausente;
4. ignoram o perfil ou ativam features assimétricas;
5. aceitam erro de argumento como prova de existência da função;
6. aceitam falha de construção como `ABSENT`;
7. descartam stderr ou exit em caso diagnóstico;
8. normalizam texto/morfologia que constitui observável público;
9. convertem timeout, OOM ou identidade divergente em sucesso;
10. extrapolam o resultado de uma tag HTML ou wrapper math para seus irmãos;
11. atribuem owner L0 por semelhança de nome ou diretório;
12. alteram expected depois de observar o candidato.

Casos positivos válidos devem resultar `Preserved`; mutações negativas,
`Violated`; casos deliberadamente opacos, `Unknown`. Exigir
`mutation_score = 1.0`, repetição idêntica e invariância à ordem. Mutação válida
sobrevivente impede o selo.

Budget de calibração: duas revisões focais por causa e uma execução integral
depois que todos os recortes passarem. Duas revisões consecutivas sem ganho no
vetor ou com a mesma causa dominante exigem redesenho; não permitem relaxar o
contrato nem converter `Unknown` em sucesso.

## Fase E — execução bilateral

Criar `lab/parity/p1400_route_probe.py`. O runner deve:

- validar hashes e lista antes de iniciar subprocessos;
- executar uma rota por processo quando um diagnóstico anterior puder ocultar
  o seguinte;
- limitar cada stream a 64 KiB e marcar truncamento como `UNKNOWN`;
- limitar cada processo a 512 MiB e 15 s;
- manter paralelismo 1;
- executar ordem direta, repetida e reversa;
- registrar duração, RSS máximo, swap, comando e instante;
- conservar artefatos volumosos e outputs brutos em `/tmp`.

Falha do runner, falta de ferramenta, target indisponível, OOM, swap, timeout,
saída truncada ou identidade ambígua devem permanecer `UNKNOWN` com razão
literal. Não aumentar memória para transformar falha semântica em sucesso.

## Fase F — ownership e fila causal

Somente após a medição, ler integralmente o Prompt L0 vigente de cada módulo
candidato a owner e conferir seu consumer exato e hash. Não inferir ownership
por símbolo, nome de arquivo ou proximidade arquitetural.

Para cada item não-`MATCH`, registrar:

- menor witness bilateral reproduzível;
- estado e perfil afetado;
- causa observada versus inferência;
- owner L0 e consumer, quando provados;
- estado do owner: `CURRENT`, `STALE`, `MISSING` ou `AMBIGUOUS`;
- classe ADR-0127 provável e condição que a refutaria;
- dependências e risco de regressão;
- agrupamento mínimo de rotas da mesma causa.

Rotas sem owner único ficam na fila como dívida de nucleação e não podem ser
encaminhadas diretamente para implementação. Se a solução futura criar ou
alterar contrato público, default, fase do pipeline ou compatibilidade, o passo
de produto deverá redigir o L0 e parar para confirmação humana.

## Artefatos finais

Produzir:

- `lab/parity/p1400_route_probe.py`;
- `00_nucleo/diagnosticos/p1400-manifest.json`;
- `00_nucleo/diagnosticos/p1400-contract.json`;
- `00_nucleo/diagnosticos/p1400-oracles.json`;
- `00_nucleo/diagnosticos/p1400-mutants.json`;
- `00_nucleo/diagnosticos/p1400-preseal.json`;
- `00_nucleo/diagnosticos/p1400-results.json`;
- `00_nucleo/diagnosticos/p1400-owner-ledger.tsv`;
- `00_nucleo/diagnosticos/p1400-backlog.json`;
- `00_nucleo/diagnosticos/p1400-report.md`;
- `00_nucleo/diagnosticos/p1400-verification.json`.

O relatório separa obrigatoriamente:

1. rotas públicas realmente ausentes;
2. implementações parciais com forma alternativa;
3. diferenças de semântica/sintaxe/morfologia;
4. lacunas do harness ou da fixture;
5. scope-outs explicitamente documentados;
6. matches atuais;
7. itens sem owner L0 provado.

Não publicar uma “percentagem de toda a linguagem”. Contagens são válidas
somente para o denominador congelado de 93 rotas.

## Gates finais

Executar, sob limites registrados e sempre serialmente:

```sh
python3 -m py_compile lab/parity/p1400_route_probe.py
python3 lab/parity/p1400_route_probe.py --validate-only ...
python3 lab/parity/p1400_route_probe.py --gate ...
python3 lab/parity/p1400_route_probe.py --run ...
cargo fmt --all -- --check
crystalline-lint .
git diff --check
```

Não executar `cargo test --workspace`, rebuild global ou testes monolíticos.
Como o produto é read-only, build só é permitido se a identidade do binário
cristalino estiver obsoleta; nesse caso usar build release serial, teto de
4 GiB e timeout de 180 s, registrando RSS e swap.

O verificador final deve confirmar:

- hashes de todos os inputs protegidos;
- cardinalidade literal 93 e exclusão integral da família `format`;
- execução bilateral dos quatro perfis aplicáveis;
- estabilidade direta/repetida/reversa;
- `mutation_score = 1.0`;
- nenhuma alteração em L0, Núcleos ou L1–L4;
- zero `Unknown` promovido implicitamente;
- zero V5, V15 ou V26;
- fila agrupada por causa e owner provado, não por conveniência nominal.

## Critério de fecho

P1400 fecha somente quando cada uma das 93 rotas possuir exatamente um estado,
evidência atual reproduzível e owner provado ou razão literal para sua ausência.
Todos os números decisórios devem registrar HEAD, working tree, timestamp,
comando e hashes dos produtos que os geraram.

O relatório recomenda exatamente um próximo passo de produto: a menor causa
`ABSENT`, `PARTIAL` ou `DIFFERENCE` com owner L0 único e witness completo. Ele
não implementa esse passo e não mistura a futura nucleação da família `format`
com as 93 rotas deste denominador.

O veredito deve ser proporcional: cobre somente os observáveis executados e
nunca declara paridade funcional geral da linguagem Typst.
