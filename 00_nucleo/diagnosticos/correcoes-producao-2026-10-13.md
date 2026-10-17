# Correções de produção e validação do CI

A linhagem bidirecional está íntegra e os workflows de CI, release e Docker
possuem gates de validação. As falhas funcionais conhecidas de HTML, bundle e
P1292 foram corrigidas. **A publicação continua bloqueada pelo Clippy**; os
warnings arquiteturais e as demais pendências da auditoria de produção também
permanecem abertos.

O saneamento posterior do Clippy está registrado em
[Saneamento dos testes de layout e das constantes de cor](qualidade-ci-2026-10-13.md).
Os números abaixo conservam a medição desta primeira rodada.

## Proveniência

Branch `Tekt`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**, com alterações preexistentes preservadas. Os números deste
relatório descrevem a árvore de trabalho, não o commit isolado.

Os recibos estão em `correcoes-producao-2026-10-13/`:

- `before.json.gz` registra o estado inicial desta rodada, em
  `2026-10-13T21:14:45.495438+00:00`, incluindo conteúdo, HEAD, status e
  `git diff HEAD --stat`.
- `pre-format.json.gz` preserva os arquivos antes da formatação global;
  `format-changes.json` identifica os 205 arquivos formatados e seus hashes.
- `pre-final-checks.json.gz` fixa o estado das verificações finais em
  `2026-10-13T21:33:31.957010+00:00`, com HEAD, status, diffstat e hashes.
- `validation-summary.json` reúne resultados e verifica se os arquivos mudaram
  durante os checks; apenas o README dos auxiliares recebeu um link documental.
  `evidence.json.gz` conserva logs, inventários e scripts de
  reprodução; os arquivos avulsos seguem a política de ignore dos diagnósticos.

O oráculo local usado nas novas medições é o build ratificado P1489 de
`lab/typst-original/target/release/typst`, pin
`9f2b6e8715237cb086899a42873660fe744622e8`, árvore upstream
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, SHA-256 verificado
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
A receita de origem está no diagnóstico P1489. Nenhum commit, push ou publicação
foi realizado nesta rodada.

## Correções funcionais e seus refutadores

Os L0 proprietários foram atualizados antes do código. Estas correções mantêm
contratos públicos, modos padrão e fases de pipeline, seguindo o fluxo contínuo
do ADR-0127.

### HTML e bundle

Whitespace, encoding e projeção semântica de bundle não atravessavam corretamente
os carriers `Source`, `ShowRecipe`, `Located` e `Realized`. Agora projetam o corpo
visual, preservando estilos onde necessários e sem renderizar a proveniência
original de `Realized` nem executar novamente receitas.

Os cinco testes causais novos falharam antes da correção
(`carriers-unit-red.log`) e passaram depois (`carriers-unit-green.log`). A família
CLI HTML passou em nove testes, o caso de bundle com introspecção global passou,
e a família unitária de exportação HTML passou em 33 testes. Esses resultados
cobrem as sete falhas anteriores de HTML/bundle, sem declarar paridade global.

### P1292

A medição bilateral em `math-bilateral.json` mostrou que ambos os compiladores
rejeitam `align:left` em math e aceitam `align:#left`. A fixture de vetores foi
corrigida e recebeu uma asserção negativa explícita para a sintaxe rejeitada;
as verificações de geometria foram mantidas.

No cenário `place.flush()`, o cristalino emitia uma página adicional porque
`block::layout` restaurava a altura reduzida da região anterior após uma mudança
de página. A restauração agora exige permanecer na mesma página e coluna. O
teste causal observou altura 20 em vez de 100 antes da correção
(`block-region-red-final.log`) e passou depois (`block-region-green.log`). O
contrato black-box completo passou em `flush-contract-after-block.log`; os 11
testes do alvo P1292 passaram na suíte ampla.

## Linhagem, qualidade e infraestrutura

O reparador oficial foi executado em cópia isolada, com preflight de ownership e
núcleos. Apenas os selos dos owners autorizados foram copiados de volta. A
formatação e os reparos de comentários Rustdoc tiveram os selos reversos
atualizados; nenhum selo foi usado como prova de conformidade semântica.

V1/V5/V15/V26 estritos passam, e o gate reverso verifica **965 consumers, zero
erros**. O relatório completo do linter publicado contém zero erros, **381
warnings** (V16: 328; V17: 28; V12: 19; V18: 6) e 1616 notas. Os dois novos
V16 estão nos helpers de transparência HTML que devolvem o conteúdo terminal;
permanecem visíveis para revisão. O critério global de zero violations ainda
não foi atingido.

`cargo fmt --check --all` passa. A formatação revelou uma comparação textual de
delegação estática que não aceitava a vírgula final inserida pelo rustfmt. O
observer agora normaliza essa vírgula e whitespace, mantendo variante, callee,
argumentos e braço completo. O RED está na primeira suíte ampla.

Rustdoc com `-Dwarnings` passa após corrigir links, URLs e tipos genéricos
interpretados como HTML. Isso não elimina warnings de compilação das
dependências do passo; esses continuam sendo bloqueados pelo gate Clippy.

O CI instala explicitamente Poppler, qpdf, librsvg e ImageMagick. O P1293 deixou
de usar `/usr/local/bin/typst`, que apontava para o pin antigo. Exige agora
`TYPST_VANILLA` fornecido após verificação independente. O CI faz checkout do
upstream pinado, verifica commit e árvore, compila com lockfile e toolchain
fixado, e conserva receita e SHA-256 em `oracle-build.json`. O P1293 passou em
dez testes com o build local ratificado do pin vigente.

O empacotador de release também recusa fontes não rastreadas pelo Git; antes,
uma árvore com arquivos rastreados limpos podia conter código adicional sem
proveniência. O novo teste falhou antes do reparo e passou depois. Os auxiliares
Python passaram em **16 testes**. Actionlint passou nos três workflows.

## Validações finais

| Gate | Resultado |
| --- | --- |
| Linhagem V1/V5/V15/V26 e reversa | Passou |
| Formatação e `git diff HEAD --check` | Passaram |
| Rustdoc `-Dwarnings` | Passou |
| Auxiliares Python e actionlint | Passaram |
| P1293 com oráculo vigente | 10 passaram |
| Build release nativo e smoke PDF/HTML | Passaram; PDF validado também por qpdf e extração de texto |
| Suíte ampla inicial desta rodada | 8335 passaram, 1 falhou, 7 ignorados preexistentes; falha estrutural de formatação corrigida |
| Suíte final com oráculo vigente | **8336 passaram, zero falhas, 7 ignorados preexistentes** |
| Clippy `-D warnings` | Falhou; inventário detalhado nos recibos |
| Linux i686 local | Parou na dependência `ring`, por ausência de headers C de 32 bits |

A instalação local de `gcc-multilib` não foi concluída porque `sudo` requer
senha. O workflow já instala essa dependência. Esse resultado não aprova nem
reprova o código Rust para i686. Windows, macOS, cross builds, build Docker e
execução remota dos workflows continuam sem validação nesta máquina.

O build otimizado desta árvore produziu o binário cristalino SHA-256
`bceeecf6d003fb09ff6758e378c3a14ea889e50635cbf7af96fbff30c4d3b0d0`.
Receita, fontes de smoke e observações estão em `release-build-final.log` e
`release-smoke-final.json`. Esse smoke comprova os casos exercitados, não a
conclusão das pendências de produção abaixo.

O Clippy mantém diagnósticos de APIs marcadas deprecated mas ainda usadas,
imports/variáveis não utilizados, documentação, métodos matemáticos proibidos e
outros casos. A execução falha já no core; a contagem observada não certifica
o inventário completo das outras camadas. Não foram introduzidas permissões
globais para tornar esse gate verde.

## Trabalho ainda aberto da auditoria de produção

A integridade formal da linhagem e as correções acima não encerram os seguintes
itens da auditoria original:

- Header/footer contextual, incluindo observação por página.
- Repetição de grupos de tabela/grid entre páginas, rowspans e bordas sem
  duplicação de efeitos.
- Continuação de notas de rodapé longas e linhas maiores que a região.
- Fragmentação contextual de parágrafos e aridade/ordem de callbacks de página.
- Clippy, análise dos warnings arquiteturais e testes sem refutador causal.
- Validação remota da matriz e dos pacotes/imagens de release.

O README passou a apontar para evidências atuais em vez de apresentar marcos
históricos como certificação de produção. A sequência de correção dos itens
restantes deve começar pela medição e pelo L0 proprietário; novos contratos
públicos continuam sujeitos ao gate ADR-0127.
