# Tekt — linhagem e CI/release, 2026-10-13

Este registro conserva a medição inicial. Correções funcionais, formatação,
Rustdoc e ajustes posteriores de CI estão em
[Correções de produção e validação do CI](correcoes-producao-2026-10-13.md).

## Estado e proveniência

Trabalho solicitado: corrigir a linhagem e a infraestrutura de CI/release da
branch `Tekt`. Base: `06848cdcb5450f2c555626bf116bc7acd82bd82b`, com **working tree
não commitado preexistente**. A captura inicial foi feita em
`2026-10-13T19:55:43.915117+00:00`.

Todos os números abaixo pertencem a essa árvore com as alterações desta tarefa,
não ao commit isolado. Em `linhagem-ci-release-2026-10-13/`:

- `before.json.gz`: HEAD, UTC, `git status`, `git diff HEAD --stat` e hashes dos
  arquivos antes do trabalho.
- `before-files.json.gz`: conteúdo inicial dos arquivos auditados, para distinguir
  nossas mudanças das alterações que já estavam na branch.
- `after.json.gz`: estado final e hashes; `evidence.json.gz`: resultados, logs e
  inventários reunidos para conservação no Git.
- `scope-verification.json`: comparação contra a captura inicial. Corpos Rust e
  texto normativo dos L0 foram preservados; nenhum núcleo foi alterado.

Os logs avulsos são locais, conforme o `.gitignore` de diagnósticos. Os recibos
comprimidos conservam o conteúdo necessário para revisão. Não houve commit, push,
execução remota dos workflows ou publicação.

## Medição e reparo de linhagem

A auditoria inicial encontrou 152 consumers com V5: 151 arquivos de `src` e um
teste externo. A verificação reversa ampliou o inventário para 208 pares
L0/consumer, incluindo selos reversos obsoletos que o linter publicado não acusa
quando o selo direto ainda coincide. Havia também metadados fora do preâmbulo
canônico em 37 prompts.

Foram lidos os contratos atuais e examinadas diferenças de código/histórico nos
casos com selo reverso divergente. O inventário registra separadamente selos já
coerentes, mudanças só de header, mudanças de corpo, placeholders e selos cuja
origem não foi recuperada. Ausência de origem histórica não foi apresentada como
prova de conformidade semântica. As cláusulas que já declaravam pendências nos
L0 foram preservadas.

V15/V26 passaram antes do reparo. O reparador oficial foi aplicado em cópia
isolada, e somente mudanças permitidas foram copiadas de volta: `@prompt-hash`
nos headers e `Hash do Código` nos prompts, incluindo reposicionamento no
preâmbulo. O resultado alterou 144 headers e metadados de 115 prompts. Nenhum
comportamento, contrato público ou corpo de código L1–L4 foi modificado.

Resultado medido:

- Linter publicado pinado, `--checks v1,v5,v15,v26 --fail-on warning`: passou.
- Verificação reversa: 965 consumers, zero erros.
- Linter completo instalado: zero erros; permanecem 379 warnings de outras regras
  (V16: 326; V17: 28; V12: 19; V18: 6) e 1616 notas. Portanto, não se declara o
  gate global de zero violations do projeto encerrado.

O fechamento é da integridade formal bidirecional da linhagem. Ele não encerra
dívidas semânticas ou comprova paridade com o vanilla.

## CI implementado

`.github/workflows/ci.yml` agora usa o workspace cristalino real:

- Build de todos os alvos em Linux e Windows; suíte completa serial em Linux com
  fixtures isoladas em `/dev/shm`, como exigem os L0 P1465/P1467/P1468/P1469.
  Windows compila inclusive os testes e executa a CLI com `--version`; não executa
  a suíte dependente de tmpfs Linux. `cargo testit` aponta para
  `cargo test --workspace --locked`, sem dependência do `test-wrapper` ausente.
- Compilação Linux i686; esta checagem não equivale à execução de testes em 32 bits.
- Clippy, formatação e documentação com falhas bloqueantes.
- Relatório SARIF completo e gates estritos de linhagem direta/reversa, ownership
  e núcleos. Os testes dos auxiliares de CI também são executados.
- Check agregado `Tests` exige sucesso de todos os jobs. Cancelamento e jobs
  pulados não permitem aprovação.

O linter do CI está fixado no commit **publicado**
`49c6bb0e584104a0c89270431d8402f5c6c61150` de `Dikluwe/tekt-linter`, repositório
público. O commit local mais novo não publicado não foi usado como dependência do
CI. O auxiliar `tools/ci/check_reverse_lineage.py` cobre a lacuna reversa dessa
versão. A severidade V5 também foi elevada na configuração; o argumento
`--fail-on warning` garante o bloqueio independentemente da interpretação dessa
configuração pela versão pinada.

O walker exclui o diretório `diagnosticos` da descoberta de consumers. Isso evita
classificar cópias históricas de código como produção; L1–L4, L0 e núcleos
produtivos continuam verificados. As demais regras mantêm seu relatório.

## Release e Docker

Os dois workflows reutilizam o CI antes de compilar/publicar. O build passou a
selecionar `-p typst-wiring --bin typst --locked`, removendo pacote/features do
upstream que não existem neste workspace. Rust foi fixado em `1.92.0`.

O empacotador gera tar.xz/zip, checksum SHA-256 e `BUILD.json`, contendo commit,
árvore Git, hash do binário, lockfile, toolchain, versão do compilador e receita de
build. Ele rejeita alterações rastreadas e identidade de commit divergente. A
receita é declarada pelo workflow; o auxiliar não certifica sozinho a origem de
um binário fornecido arbitrariamente.

A compilação de todos os alvos também revelou três benchmarks desatualizados.
As fixtures de `pdf_builder_bench.rs` e `stream_bench.rs` receberam `bidi: None`;
o classificador de `benches/content/coverage.rs` passou a atravessar os carriers
`Source`, `ShowRecipe`, `Realized` (conteúdo original) e `Located`. São ajustes de
infraestrutura fora de L1–L4 (ADR-0115). O erro inicial está em
`build-all-targets-red.log`, e a captura anterior dessas três fontes está em
`benchmarks-before.json.gz`.

`Cross.toml` repassa `TYPST_COMMIT_SHA` ao container, como previsto pela
[documentação oficial do cross](https://github.com/cross-rs/cross/wiki/Configuration#buildenv).
O commit do cross foi expandido para o SHA completo
`085092ca01d6ac9f5a0b612d9e68bcdf1e50f7ee`.

O Dockerfile usa o mesmo pacote e Rust, imagens-base fixadas por digest e contexto
filtrado por `.dockerignore`. Os manifests dos pins foram consultados e incluem
amd64/arm64 (`container-tags.json`). Foi corrigido o SHA malformado de
`docker/build-push-action` e incluída uma tag por commit para execução manual.

O daemon Docker local recusou acesso ao socket. Build das imagens, cross builds,
Windows/macOS e os runners GitHub ainda precisam de validação remota. A validação
de YAML/actionlint e dos manifests não substitui esses builds.

## Validação local e bloqueios restantes

| Verificação | Resultado |
| --- | --- |
| `cargo build --offline --locked` | Passou |
| `cargo build --offline --workspace --all-targets --locked` | Passou após atualizar as fixtures dos benchmarks |
| Build release de `typst-wiring`/`typst` | Passou |
| CLI release `--version` e compilação de documento simples para PDF | Passaram |
| Auxiliares Python de linhagem/empacotamento | 11 testes passaram |
| Integração com o linter publicado pinado | 2 testes passaram |
| `actionlint` dos workflows | Passou |
| Gates V1/V5/V15/V26 e reverso | Passaram |
| `cargo testit --offline --no-fail-fast -- --test-threads=1` | 8285 passaram, 45 falharam, 7 já estavam ignorados; diagnóstico de ambiente abaixo |
| Clippy com `-D warnings` | Falhou em pendências existentes |
| `cargo fmt --check --all` | Falhou em formatação existente |
| Rustdoc com `-Dwarnings` | Falhou em links e tags de documentação existentes |

As falhas funcionais observadas incluem os sete casos já conhecidos de
HTML/bundle em `04_wiring/tests/cli.rs` e dois casos de
`04_wiring/tests/p1292_contract.rs`: `vec_gap_finite_auto_absolute_and_mixed`
(erro `unknown variable left`) e `flush_prefix_suffix_clearance_nested_and_noop`
(asserção 4 versus 3). O diagnóstico registra o conflito entre teste e execução;
a correção exige medir os contratos e o oráculo pinado, sem presumir qual lado
está errado.

Dois testes HTTP da suíte inicialmente falharam por `Operation not permitted`
ao abrir sockets. Reexecutados fora do sandbox, ambos passaram. Essa limitação
do ambiente está separada dos defeitos funcionais.

Outras 34 falhas da primeira execução vieram de `TMPDIR` ausente nos contratos de
exportação P1465/P1467/P1468/P1469. Uma sonda com `/tmp` confirmou que esses testes
exigem `/dev/shm` literalmente. O CI foi corrigido para criar uma raiz isolada
nesse tmpfs e executar a suíte serialmente, conforme seus L0. A reexecução
dirigida passou nos 36 testes desses quatro alvos (`tmpfs-contract-tests.log`),
incluindo os 34 que antes falhavam. A contagem inicial não deve ser interpretada
como 45 defeitos funcionais. Restam nove conflitos funcionais confirmados pela
suíte: sete de HTML/bundle e dois do P1292. Essa separação deriva da execução
completa seguida de reexecuções dirigidas, não de uma nova execução completa.

O smoke de release produziu um PDF válido e registrou o binário SHA-256
`bf0aab91cc0dc8bfc8b740845b4f267855d7e2aca4adab89e38b337e150ebb2d` em
`release-smoke.json`. Esse é o binário cristalino da árvore de trabalho, não o
oráculo vanilla. Nenhuma medição de paridade foi feita nesta tarefa.

**A publicação continua bloqueada pelos testes e verificações de qualidade.**
Os workflows corrigidos mantêm essas falhas visíveis. Próxima etapa: resolver os
conflitos funcionais e o baseline de qualidade sob seus respectivos L0, depois
validar toda a matriz remota. Instruções operacionais em `tools/ci/README.md`.
