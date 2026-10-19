# Passo 1485 — `asset(path, data)` e target bundle funcional

**Estado:** P1485 confirmado e parcialmente materializado na working tree;
novo gate ADR-0127 P1485.2 aberto para continuidade global de Locations e
contextos entre documentos. Não fechar P1485 nem conceder paridade total ao bundle
enquanto o caso PDF `1. Alpha`/`2. Beta` e os controles cruzados não passarem.

**Baseline:** commit limpo `90aad760ba472cffb70063baa684c461a240e447`
em 2026-09-22, comparado ao vanilla pinado `586e1bd43`; hashes dos
binários em P1480 §1.

## 1. Medição anterior à decisão

No vanilla, `asset` pertence a `format.bundle` e o prelude o expõe quando
`bundle` está ativo (`lab/typst-original/crates/typst-library/src/lib.rs:430`).
`lab/typst-original/crates/typst-bundle/src/format.rs:135-240` define
`asset(path: BundlePath, data: AssetData)`; dados são bytes, ou string
convertida a UTF-8, e o elemento só produz arquivo no target bundle.
`--format bundle` é o destino documentado na mesma fonte. O cristalino atual
tem `native_asset(path, kind:?)`, apenas placeholder sem dados
(`01_core/src/compiler/stdlib/structural/document.rs:76-110`). Seu L0
`compiler/stdlib/structural/document.md:10-35` chama-o de extensão; esta
premissa ficou obsoleta com o vanilla ratificado e deve ser retificada.

Ainda **não** foram medidos bilateralmente todos os casos de erro, paths,
colisões e múltiplos documentos. A primeira fase deste passo executa essas
sondas; nenhuma assinatura além de `path,data` é inferida como completa.
Classificação: arquivos emitidos, nomes, bytes de payload e diagnósticos de
entrada são observáveis; organização interna do bundle não é. Inferência:
substituir o placeholder exige modelar o target, não somente a função;
refutação seria uma prova de que `asset` produz saída equivalente em outro
target, contradita pela fonte acima.

## 2. Contrato a redigir após as sondas

1. `format.bundle` e sua projeção `asset` têm kind, `repr`, namespace e
   gate correspondentes ao vanilla. Nenhum target ativa a feature por si.
2. `asset(path, data)` aceita os tipos e normalização de `BundlePath`/
   `AssetData` medidos; rejeita `kind:` como contrato vanilla, salvo decisão
   explícita de extensão compatível. Preserva bytes, não interpreta extensão
   nem tenta ler arquivo em L1. Leitura `read(...)` continua responsabilidade
   do world/infra já existente.
3. `compile --format bundle --features bundle` emite os arquivos de assets
   e documentos definidos no `.typ`, com paths e payloads equivalentes ao
   vanilla. Medir e cobrir duplicatas, diretórios, path inválido, ausência
   de documento, mistura de PDF/HTML/SVG/PNG e interação com watch antes de
   decidir o escopo concreto. Se watch exigir outro passo, registrar
   explicitamente a fronteira em L0 e manter P1485 aberto para paridade
   total de bundle.
4. Fora do target bundle, `asset(...)` retorna o diagnóstico vanilla; não
   materializar arquivos como efeito colateral de eval L1.

## 3. Owners e gate

Auditar `compiler/stdlib/format.md`,
`compiler/stdlib/structural/document.md`, `entities/content.md`,
`entities/compiler_features.md`, `shell/cli.md`, `infra/pipeline.md`,
`wiring/main.md` e os L0s de export realmente afetados. A divisão em
owners continua 1:1, núcleo de feature pinado. L1 descreve conteúdo puro;
L3 faz I/O; L4 compõe. Não importar `lab` para produção.

Mudança de assinatura/contrato de `asset`, target e semântica de CLI exigem
L0 causal, hashes e **confirmação humana após a especificação** (ADR-0127).
Testes RED devem provar incompatibilidade atual em chamada e arquivo de
saída; GREEN exige corpus bilateral funcional (não apenas `type(asset)`),
controles de feature ausente e demais targets, `cargo build`, testes
afetados, `crystalline-lint .`, V5/V15/V26. Registrar estado Git, hora,
binários e resultados de cada sonda. Se o tamanho demandar subdivisão,
redigir a divisão **antes** do código e nomear no L0 o passo que completa
cada subconjunto; não declarar P1485 fechado com um bundle só nominal.

## 4. Auditoria pré-gate (2026-09-23T21:55:59-03:00)

P1484 foi confirmado e materializado na working tree não commitada;
HEAD permanece `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`.
As sondas com vanilla pinado `586e1bd43`, comandos, fixtures,
diagnósticos, bytes de assets e snapshot Git exato estão em
`00_nucleo/diagnosticos/typst-p1485-bundle-sondas.md`.
O alvo medido exige `format.bundle.asset` e global `asset` com
identidade compartilhada, path/data posicionais, path virtual seguro,
payload byte-exato, `document` com path/body, saída diretório,
PDF/SVG/PNG/HTML mistos, erros de duplicata/top-level/target e
warning experimental. O dono incluiu HTTP/live reload no escopo de
P1485; runtime confirmou HTTP 200, script SSE por omissão,
`--no-reload` sem script e `--no-serve` sem listener local.
Fonte, comandos e provenance estão no diagnóstico P1485.
No cristalino, `document()` direto ainda devolve Content de
metadata, enquanto no vanilla é `missing argument: path`:
além de novo target há quebra pública deliberada a confirmar.

L0s propostos (SHA-256 completo, após resselo V5/V15/V26):

```text
compiler/stdlib/format.md                    0d55b01c60d60d2194e951f207b68f53a0a0013f1c3ec2b2ab545319f4989f91
compiler/stdlib/structural/document.md      3395831455ce3582196999d014afc89ecce5d1e42b1ec0a3b3e15e4d411c131a
compiler/stdlib/structural.md               772e0e38862608f4674ac6e4581e1ab47f68b578a2ed980a1a65a87ee5f3c262
entities/content.md                          755c659585276c6d88b5afdb5d55e192b940a34958b9e705fbf899d87389c473
compiler/eval.md                             f6a7a77f40ad833a4cca7ee1501f4c72d19a8e021db8e36d3c7bf9175a9763c9
compiler/eval/bindings/field_access.md       b27a969d34d3f3ea6eb45dfd992ba0fff7771f2f10b1fd7181adb370fe26b9ec
compiler/eval/tests.md                       11d7514453e8db9c42f3e5cdfb0b36478cfd24c9ae339dbf05c635d8df4c191f
shell/cli.md                                 e69a740a3a85d8f7794d33b2a2438baecd156501f6d699225a5822a5b98151e9
infra/pipeline.md                            b6b9e5a6663cbf248d3130556fbb8a7ebabdfb91ab534c8bf80753cc28cc5c69
shell/watch.md                               64d147960f8027d1c35f619bbd8f33959542ca0b5d2cdea9eb9a1682b7c13664
wiring.md                                    781976d40bab93ed60c99f87216915a51943cd55d1734ff835d210edbfa577a6
wiring/tests/cli.md                         a581e23645383b141db98b9d8eb3069e1b5722c512e1f70eeb32107864132bd0
infra/integration_tests.md                   029088dfcd13468a5fbef5f541094ce2be86adf1e10784d9960e9a8e5796320b
```

Os arquivos Rust correspondentes receberam **somente**
`@prompt-hash` novo pelo linter; nenhum teste ou código
funcional P1485 foi escrito. `git diff --check` e
`crystalline-lint --quiet .` passaram; o resselo específico
V5/V15/V26 terminou com zero drift. A lista exata de
arquivos rastreados na working tree inclui os 15 da
proveniência P1484/P1485 no diagnóstico mais estes dez:
`00_nucleo/prompts/compiler/stdlib/format.md`,
`00_nucleo/prompts/compiler/stdlib/structural.md`,
`00_nucleo/prompts/entities/content.md`,
`00_nucleo/prompts/infra/pipeline.md`,
`00_nucleo/prompts/wiring.md`, e os respectivos
`01_core/src/compiler/stdlib/format.rs`,
`01_core/src/compiler/stdlib/structural/mod.rs`,
`01_core/src/entities/content.rs`,
`03_infra/src/pipeline.rs`,
`04_wiring/src/main.rs`. Este passo e o diagnóstico P1485
são não rastreados; os rascunhos P1486–P1488 permanecem intactos.

**Gate ADR-0127 ainda aberto:** o dono escolheu incluir servidor
HTTP/live reload agora. Os L0s de CLI, pipeline, watch, wiring e
testes foram complementados com APIs e defaults explícitos; os
hashes finais aparecem na lista acima. P1484 não
autoriza por arrasto as novas mudanças de contrato, default,
fase e compatibilidade de P1485. Nenhum código/teste funcional
P1485 começa até confirmação humana desta especificação final.

## 5. Revisão final pré-gate

O dono incluiu HTTP/live reload no P1485. Os L0s de CLI,
pipeline, watch, wiring e teste externo foram fechados com
assinaturas, defaults e aceitação; hashes completos estão acima.
O resselo V5/V15/V26 terminou com zero drift; `git diff --check`
e `crystalline-lint --quiet .` passaram. Nenhum código ou teste
funcional P1485 foi escrito antes desta confirmação final.

O gate ADR-0127 permanece aberto: a escolha de incluir servidor
não confirmou o L0 completo nem a quebra de contrato de `document`.


## 6. Materialização segregada após o gate e novo limite

O dono confirmou o L0 P1485 com “Pode continuar”. Pela skill
`tekt-materializacao-segregada`, A escreveu produção a partir do
prompt, B escreveu testes independentemente no mesmo baseline e C
integrou/validou. `asset(path, data)`, target bundle com arquivos
mistos, watch HTTP/live reload e proteção de publicação foram
materializados. Os resultados, fixtures, mutations, horários,
proveniência Git e o único RED restante constam em
`00_nucleo/diagnosticos/typst-p1485-integracao-gate2.md`.

A numeração global entre PDFs ainda diverge: o vanilla emite
`1. Alpha`/`2. Beta`, o cristalino `1. Alpha`/`1. Beta`.
A estabilização contextual selecionada também precisa revalidar
o conjunto de documentos, não um único body. Os L0s P1485.2 dos
owners Locator, introspecção, layout, estabilização e pipeline
foram redigidos e ressellados; seus hashes completos estão no
diagnóstico. `crystalline-lint --quiet .` e `git diff --check`
passaram. **Novo gate ADR-0127 aberto:** as APIs públicas L1
propostas não foram implementadas e exigem confirmação humana.
P1485 permanece aberto, sem commit.
## 7. P1485.2 materializado; fronteira Page P1485.3 aguardando gate

Após confirmação humana do primeiro gate P1485.2, o código de seed,
introspecção global, estabilização por conjunto e page views locais
foi materializado. Os testes de bundle anteriores à nova sonda passaram
15/15; L1 passou 6142/6142 e infra lib 1051/1051 no recorte registrado.
A suíte Rustdoc de watch teve allowlist ajustado e teste focal 1/1.
O diagnóstico de integração anexa logs, fixtures, resultados, horário
e o snapshot Git exato. Não houve commit.

Um novo teste RED mostrou que o HTML posterior a PDF com
`counter(page).update(5)` herda 5; a linguagem ideal deve mostrar 1,
sem impedir `counter(page).update(7)` dentro do próprio HTML.
O vanilla ratificado foi usado como medição semântica, não como
algoritmo a copiar. A correção exige uma nova assinatura pública L1
de introspecção runtime por bodies, além de replay causal da chave Page.
Os L0s P1485.3 de introspect, from_tags, counter_registry, counter,
pipeline e context_stabilization foram redigidos e ressellados;
`crystalline-lint --quiet .` passou. Seus SHA-256 completos estão
no diagnóstico.

**Gate ADR-0127 P1485.3 aberto:** nenhuma implementação funcional da
nova entrypoint foi feita. A confirmação anterior abrangia P1485.2,
não este novo contrato. P1485 permanece aberto, incluindo
`counter(page).final()` multi-documento em scope-out explícito.


## 8. P1485.3 materializado

Após confirmação ADR-0127 para o restante desta seção, a fronteira
Page por body e o replay causal runtime foram implementados. Os testes
independentes e a validação final passaram; prova e proveniência estão
em `00_nucleo/diagnosticos/typst-p1485-integracao-gate2.md`. O HTML
continua sem layout paginado e os demais contadores permanecem globais.
Não houve commit. `counter(page).final()` multi-documento permanece
scope-out explícito do L0, sem alegação de paridade nessa superfície.
