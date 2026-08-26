# P1207 — sanear metadata canônica e os dois prompts órfãos

**Estado:** ESCRITO — NÃO EXECUTADO
**Baseline:** P1206 com V15=0, V26=0, V7=2, V5=312; índice Git vazio.
**Classe ADR-0127:** higiene de linhagem/documentação, sem contrato público,
default ou fase; fluxo contínuo. Se a auditoria de `custom-ca-cert` revelar
mudança funcional ainda não materializada, parar antes de código.

## Objetivo

Desbloquear o preflight transacional do linter sem aplicar o reparo global de
V5 e sem misturar os avisos heurísticos V16–V21. O resultado esperado é:

- metadata canônica exatamente uma vez em todos os consumers;
- V7=0, V15=0 e V26=0;
- `crystalline-lint --fix-hashes --dry-run .` executável e determinístico em
  duas passagens, sem escrita;
- nenhuma alteração funcional e índice Git vazio.

## 1. Congelar e provar o inventário dos 21 headers

Registrar HEAD, hora, `git diff HEAD --stat`, `git status --short`, digest do
diff e estado do índice. Confirmar mecanicamente que cada ficheiro abaixo tem
exatamente um `@prompt`, zero `@prompt-hash`, owner existente e relação 1:1:

```text
01_core/src/entities/layouter_runtime_state.rs
01_core/src/entities/rel.rs
01_core/src/entities/numbering.rs
01_core/src/entities/elements/curve.rs
01_core/src/entities/elements/grid_hline.rs
01_core/src/entities/elements/grid_vline.rs
01_core/src/entities/elements/label.rs
01_core/src/entities/elements/ref.rs
01_core/src/entities/elements/table_hline.rs
01_core/src/entities/elements/table_vline.rs
01_core/src/compiler/eval/cast.rs
01_core/src/compiler/layout/footnote_flush.rs
01_core/src/compiler/stdlib/collections.rs
01_core/src/compiler/stdlib/gradients.rs
01_core/src/compiler/stdlib/layout.rs
01_core/src/compiler/stdlib/numbering.rs
01_core/src/compiler/stdlib/transforms.rs
01_core/src/compiler/stdlib/shapes.rs
01_core/src/compiler/stdlib/ref.rs
01_core/src/compiler/stdlib/label.rs
03_infra/src/fontdb.rs
```

Antes de selar, ler integralmente os 21 owners. Não alterar L0 nem código
funcional para fazer o hash “caber”. Inserir somente uma linha
`@prompt-hash <hash efetivo>` imediatamente após `@prompt`, obtendo o valor do
próprio V5 mediante placeholder focal quando necessário. Provar os 21 sources
byte-idênticos ao estado anterior após remover linhas `@prompt-hash`.

## 2. Reclassificar `_convencoes.md`

`00_nucleo/prompts/_convencoes.md` não legitima consumer: registra a decisão
de governança P1062 sobre caminhos canônicos. Removê-lo de `prompts/` e
preservar integralmente como ADR vigente, com número livre confirmado por
inventário de `00_nucleo/adr/`, título sobre referências canônicas de L0 e
estado da decisão original. Não criar `@prompt` fictício nem exceção V7.

Atualizar somente referências documentais que apontem ao path antigo. Se uma
ADR vigente já possuir integralmente a decisão, mover o texto para diagnóstico
histórico e referenciar a ADR dona, evitando duas decisões normativas.

## 3. Dissolver `shell/custom-ca-cert.md` sem owner representativo

Medir primeiro, sem confiar no campo histórico “implementado”:

1. auditar `02_shell/src/cli.rs`, `03_infra/src/package_downloader.rs`,
   `03_infra/src/world.rs` e `04_wiring/src/main.rs` para flag `--cert`, env
   `TYPST_CERT`, transporte, leitura PEM e configuração TLS;
2. ler integralmente os quatro Prompts L0 proprietários vigentes e confirmar
   seus hashes;
3. classificar cada cláusula do órfão como materializada, backlog ou refutada.

Se o contrato estiver materializado, editar primeiro cada owner 1:1 com apenas
a responsabilidade local e extrair as invariantes realmente compartilhadas
para `00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml`: precedência
flag/env, validação TLS preservada, segredo não exposto e aplicação aos
downloads controlados. Pinar somente os prompts aplicáveis e mover o Markdown
coletivo integral para diagnóstico histórico. Resselar apenas os consumers
tocados. Não apontar código diretamente ao núcleo.

Se o contrato não estiver materializado, mover o documento para diagnóstico de
backlog, corrigindo o estado falso; não criar núcleo órfão nem cláusulas L0 que
afirmem comportamento inexistente. Se a materialização exigir código ou mudar
opção/default/segurança pública, parar no gate ADR-0127 e fechar P1207 apenas
com a classificação, sem fingir V7 resolvido por consumer arbitrário.

## 4. Gates do preflight

Após as correções documentais:

1. `crystalline-lint --checks v7,v15,v26 .` → zero violações;
2. varredura de metadata → todo consumer tem exatamente um `@prompt` e um
   `@prompt-hash`;
3. `crystalline-lint --fix-hashes --dry-run .` duas vezes;
4. os dois dry-runs devem terminar com a mesma lista/digest de reparos e não
   alterar o digest da working tree;
5. não executar `crystalline-lint --fix-hashes .` neste passo: os V5 globais
   serão um lote transacional posterior próprio;
6. `cargo test -p crystalline-lint` ou a suíte equivalente disponível,
   `cargo build` e `git diff --check` GREEN;
7. `git diff --cached --quiet` confirma índice vazio.

V16–V21 permanecem inventariados, mas fora do escopo: são heurísticas de código
e não bloqueiam este preflight documental.

## 5. Fechamento

Criar
`00_nucleo/diagnosticos/typst-p1207-saneamento-metadata-preflight.md` com:

- matriz dos 21 source → owner → hash efetivo;
- prova de byte-identidade fora de metadata;
- destino e justificativa dos dois antigos órfãos;
- resultado da auditoria funcional de custom CA;
- contagens V5/V7/V15/V26 antes/depois;
- outputs e digests dos dois dry-runs;
- testes, build, diff-check, índice e proveniência temporal.

P1207 termina GREEN somente se o preflight duplo alcançar a fase de preview
sem escrita e V7=V15=V26=0. Qualquer necessidade de código público descoberta
em custom CA é gate explícito, não autorização implícita para materializar.
