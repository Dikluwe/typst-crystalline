# P1315 — revisão anterior ao candidato

Revisor: agente `/root/p1315_review`. Regime recebido: ensaio A/B executado
sem atestação de isolamento. Filesystem compartilhado; a restrição de escrita
do papel é procedimental. Escritas limitadas a `p1315-review-*.md/json/cjs`
em diagnósticos. Não escreve L0, implementação ou oráculos. Contexto recebido:
escopo proposto, identidade alegada do baseline, caminhos da fonte e limitações
do ensaio. Foram lidos a skill de materialização segregada, suas duas
referências, instruções do repositório e ADRs 0127/0108. Pastas context e
materialization não foram lidas nem listadas.

## Evidência anterior à classificação

Inspeção em `2026-09-08T13:35:48Z`, HEAD
`bc8213f36b7a29b4fdc30cfc74ddc23586117c64`; `git diff HEAD --stat` vazio.
Este documento ainda não existia no instante da inspeção. Comandos de
proveniência: `git rev-parse HEAD`, `git diff HEAD --stat`, `date -u
+%Y-%m-%dT%H:%M:%SZ`, `sha256sum` dos caminhos abaixo, `nl -ba` e `sed` nas
faixas de fonte indicadas. Não houve medição comportamental independente
nesta revisão preliminar.

| Entrada | SHA-256 |
|---|---|
| `00_nucleo/prompts/compiler/stdlib/loading.md` | `6b717ccd9ddd4784407ca2b1d052f25c63c78198062748f449538a6ff85a0097` |
| `01_core/src/compiler/stdlib/loading.rs` | `1e911a53392ce900e482cf020973d5c78136977c032045fdc4f66e10ea8ec83a` |
| `lab/typst-original/crates/typst-library/src/loading/csv.rs` | `4d4c9a3ca69314904802f94f55d3a413e8f6d045de522e7c80c543f37e333fff` |
| `00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md` | `5e8581b5f9ebb0798d4213e59e39ee8dfcd4f41b34d4ca639b00b1f287699ad9` |
| `00_nucleo/adr/typst-adr-0108-disciplina-anti-deriva.md` | `31daec5ae9e84cb5bbdcb806e9a2b6cb9160b7e90519df53e6e0bd8809076405` |
| `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |

O baseline cristalino informado em
`/dev/shm/p1314-target.cswujn/release/typst` não estava presente no ambiente
deste revisor. Seu SHA recebido não foi verificado. A ratificação do alvo
`a51e02804` foi confirmada no adendo do dono de
`00_nucleo/diagnosticos/typst-retificacao-p990-p992-lab-sync.md:133`.

Na fonte vanilla `loading/csv.rs:54-77`, o número passado ao formatter é o
ordinal enumerado mais um offset inicial, acrescido pelo cabeçalho no modo
dictionary. O comentário nas linhas 73-75 declara explicitamente por que a
linha fornecida pelo erro deixou de ser usada. `csv.rs:150-153` usa esse
ordinal no fragmento `found X instead of Y fields in line N` de
UnequalLengths. `csv.rs:140-145` mantém separadamente a posição por byte para
o diagnóstico. Há, portanto, evidência de intenção do workaround e de
separação entre número do fragmento e âncora; não é necessário inferir
intenção apenas da saída.

O baseline cristalino `loading.rs:956-965` deriva N de `Position.line()`.
`loading.rs:970-985` já consome o cabeçalho como um registro no modo
dictionary. O L0 §3.2/P787 ainda exige `line do Position do erro`, de modo
que sua atualização é pré-condição real. A preservação de parsing em P1314
também precisa de exceção explícita limitada a N.

## Classificação e condição de avanço

O escopo proposto é coerente e útil: corrigir somente N de UnequalLengths
para o ordinal do registro, contando o cabeçalho e sem contar quebras dentro
de campos ou linhas vazias ignoradas pelo parser. Essa é uma correção de
paridade coberta pelo fluxo contínuo ADR-0127 §2, item 3. A ADR explicitamente
exclui convergência para vanilla da categoria de nova intenção de produto.
Não requer confirmação adicional, desde que permaneçam inalterados casts,
assinaturas, fases, parser, valores, UTF-8, formatos circundantes e spans.

O diagnóstico é observável público segundo a exceção ADR-0108; não se exige
copiar o mecanismo Rust do vanilla. É inferência que o consumer atual basta
para satisfazer esse recorte. Uma necessidade demonstrada de mudar API,
pipeline, parser ou outra superfície refutaria o enquadramento e exigiria
reclassificação antes do código.

Antes do candidato: atualizar L0 substituindo expressamente a regra de N
por Position, congelar o novo L0 e expectativas independentes, identificar
o baseline executável acessível e demonstrar RED no recorte. O texto novo
não pode deixar duas obrigações incompatíveis ativas. O veredito final fica
pendente de patch, recibos, preservações e gates arquiteturais. Esta revisão
não constitui selo nem prova de paridade geral CSV.

## Revisão do L0 atualizado e resolução de namespace

Recebido depois da inspeção anterior, o L0 atualizado tem SHA-256
`d84772e07c0cce407fcffe341afc4f18813cbde54f0b63fd14616a626bd24b4d`.
O diff substitui expressamente a regra P787 e limita a exceção de preservação
P1313/P1314 ao número N. A seção P1315 separa a medição da decisão, conserva
o formato circundante e exige comparação do fragmento com vanilla e do
diagnóstico integral com baseline alterando somente N. Não há bloqueio
preliminar para esse contrato.

A medição recebida `p1315-measurement.json`, SHA-256
`ee6535435927f5ea6bbe8dadcd05663636b8cb41324151bd3d59a2a0c4019fe0`,
registra before/after em `2026-09-08T13:35:16.946783+00:00` e
`2026-09-08T13:35:19.191007+00:00`, mesmo HEAD acima, diff vazio e scripts
de medição não rastreados identificados. A testemunha com primeiro registro
multilinha confirma N=3 no baseline e N=2 no vanilla, em ambos os modos.
São saídas recebidas e auditadas, não reexecutadas pelo revisor nesta etapa.

A ausência inicial do executável ocorreu no namespace do sandbox. Após
informação do coordenador, `exec_command` somente leitura com
`sandbox_permissions=require_escalated` tornou o caminho host acessível:
SHA-256 verificado `cf9b997ac399cfb95cf15063cc417484ed28a75be18f6e7d55a28300d792114f`
para `/dev/shm/p1314-target.cswujn/release/typst`. A identidade deixou de ser
Unknown; não houve remoção nem recriação do binário.
