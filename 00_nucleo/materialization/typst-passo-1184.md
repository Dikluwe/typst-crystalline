# P1184 — separar o owner do hub L3 e da suíte de integração

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN EM 2026-08-25`
**Dependências:** ADR-0129; P1182 aprovado; P1183 GREEN
**Lote:** A-infra, segundo lote de saneamento da bijeção L0
**Classe ADR-0127:** correção interna de linhagem, sem mudança de contrato
público, default, fase ou compatibilidade; fluxo contínuo, L0 primeiro

## Objetivo

Eliminar exatamente uma das 23 colisões V15 restantes:

```text
00_nucleo/prompts/infra.md
├── 03_infra/src/lib.rs
└── 03_infra/src/integration_tests.rs
```

Estado final pretendido:

```text
00_nucleo/prompts/infra.md
↔ 03_infra/src/lib.rs

00_nucleo/prompts/infra/integration_tests.md
↔ 03_infra/src/integration_tests.rs
```

`infra.md` permanece no path atual e passa a especificar exclusivamente o hub
da crate. A história P844 e o contrato da suíte E2E migram para o novo owner de
testes.

Não criar Núcleo Tekt: o hub declara topologia de módulos; a suíte observa a
integração do pipeline. Não existe claim compartilhada que precise de categoria
1:N.

## Baseline medido

Sobre HEAD `00f402e875956304aa435f749a251f359287e2ba`, com working tree não
commitado após P1183 e linter SHA-256
`eb7494979040e70feb6ac3b738c86979488b8c26927480126746aae2ff707c9d`:

- V15=23 e V26=0;
- V5=419;
- `infra.md` tem exatamente dois consumers produtivos;
- `03_infra/src/lib.rs` possui 34 linhas, 19 declarações `pub mod`, dois módulos
  privados `#[cfg(test)]` e hash de código `4cc717a7`;
- `03_infra/src/integration_tests.rs` possui 5.537 linhas, 226 marcações
  `#[test]` e hash de código `841aa746`;
- ambos os consumers ainda apontam para `infra.md` com
  `@prompt-hash 8e65820e`;
- o L0 atual possui somente 10 linhas: título da crate, estado “Em migração” e
  uma adição histórica P844 pertencente à suíte de integração;
- o SHA-256 bruto atual de `infra.md` é `1db84ab1...`, mas o resselo final deve
  ser medido somente depois da atualização do L0 e de `Hash do Código`.

Os hashes de código foram calculados removendo apenas a própria linha
`@prompt-hash` dos sources, conforme o algoritmo efetivamente validado em P1183.

## Restrições

- não acessar/listar `00_nucleo/context/` ou `00_nucleo/materialization/`;
- não alterar corpos Rust, imports, visibilidade, módulos, testes ou fixtures;
- não criar, remover, ignorar ou renomear testes;
- não mover `infra.md` nem criar um `infra/lib.md` concorrente;
- não alterar os L0s dos módulos filhos de `03_infra`;
- não converter referências históricas em ownership adicional;
- não criar Núcleo Tekt;
- não executar `--fix-hashes` mutante global enquanto houver V15;
- preservar integralmente as alterações acumuladas P1181–P1183 e manter o
  índice vazio.

## 1. Proveniência e RED estrutural

Registrar:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
git diff --cached --stat
sha256sum /home/dikluwe/.cargo/bin/crystalline-lint
```

Executar duas vezes:

```text
crystalline-lint --checks v15,v26 --fail-on warning .
```

Exigir outputs byte-idênticos, V15=23, V26=0 e presença do grupo
`infra.md` com exatamente os dois consumers deste lote. Guardar logs em
`/tmp` e provar que a medição não alterou a árvore.

Se HEAD, árvore, binário, cardinalidade ou contagens divergirem, registrar a
nova proveniência antes de decidir. O V15 focal é o RED; não fabricar RED
funcional, pois não há mudança funcional.

## 2. Atualizar primeiro `infra.md` como owner exclusivo do hub

Reescrever:

```text
00_nucleo/prompts/infra.md
```

O L0 deve legitimar exclusivamente `03_infra/src/lib.rs` e conter:

- título, camada L3 e path do consumer proprietário;
- exatamente uma linha canônica `Hash do Código` no preâmbulo;
- responsabilidade de raiz da crate `typst-infra` e declaração estática dos
  módulos L3 existentes;
- a superfície pública atual dos 19 módulos `pub mod`, medida diretamente em
  `lib.rs`, sem prometer reexports ou APIs que o hub não contém;
- `integration_tests` e `p307b_snapshot_tests` como módulos privados somente
  sob `#[cfg(test)]`;
- a fronteira arquitetural: L3 materializa I/O e integra contratos L1, sem
  lógica user-facing de diagnóstico que migrou para L2;
- aceitação estrutural: a raiz expõe exatamente os módulos produtivos
  declarados no source e inclui as duas suítes somente em builds de teste;
- scope-out explícito: comportamento interno de cada módulo, pipeline, export,
  fontes, world, testes E2E e decisões futuras de API pertencem aos respectivos
  owners.

Remover de `infra.md` a história P844, pois ela não descreve `lib.rs`. Não
transformar o hub num índice que legitime os módulos filhos.

Documentar a superfície pública existente não autoriza alterá-la. Qualquer
adição/remoção futura de `pub mod` continua sujeita à classificação ADR-0127.

## 3. Criar o owner exclusivo da suíte E2E

Criar:

```text
00_nucleo/prompts/infra/integration_tests.md
```

O L0 deve legitimar exclusivamente
`03_infra/src/integration_tests.rs` e conter:

- camada L3, path proprietário e natureza `#[cfg(test)]`;
- exatamente uma linha canônica `Hash do Código` no preâmbulo;
- objetivo: exercitar o pipeline real com `SystemWorld`, filesystem temporário,
  eval, introspecção, layout e export, cobrindo fronteiras que mocks L1 não
  alcançam;
- helpers test-only vigentes para diretório temporário, criação de world,
  avaliação, travessia de frames e compilação/export;
- pureza corretamente classificada: filesystem, relógio e `SystemWorld` são
  permitidos neste consumer L3 test-only, não em L1;
- observáveis de língua e de produto verificados pelos testes: valores de eval,
  morfologia/layout, warnings/erros quando observáveis e artefatos de export;
- paridade segundo ADR-0107/0108: não elevar igualdade acidental de bytes ou
  mecânica a contrato, salvo quando bytes/estrutura são o próprio observável do
  formato testado;
- proveniência P844 preservada como história da suíte, incluindo seus helpers e
  achados #47–#54, sem alegar que esse lote esgota os testes atuais;
- aceitação estrutural: a suíte compila somente sob `cfg(test)` e seus testes
  selecionados permanecem verdes;
- scope-out: contratos dos módulos produtivos chamados, unit tests dos módulos,
  fixtures externas e comportamento novo do produto.

Não enumerar as 226 funções como contrato perene. A especificação deve definir
o papel e os limites da suíte, preservando P844 como proveniência, sem copiar
5.537 linhas de implementação para o L0.

## 4. Resselo restrito e independente

Após congelar o conteúdo dos dois L0s, calcular para cada par:

```text
code_hash   = SHA256(source sem a própria linha "@prompt-hash")[0..8]
prompt_hash = SHA256(prompt completo, incluindo "Hash do Código")[0..8]
```

Ordem obrigatória:

1. salvar em `/tmp` manifest com paths, hashes e cópias filtradas dos sources;
2. escrever `Hash do Código: <code_hash>` em cada L0;
3. somente então calcular SHA-256 do L0 completo;
4. conservar `@prompt 00_nucleo/prompts/infra.md` em `lib.rs` e atualizar seu
   `@prompt-hash`;
5. repointar `integration_tests.rs` para
   `00_nucleo/prompts/infra/integration_tests.md` e atualizar seu
   `@prompt-hash`;
6. recalcular os dois sentidos e validar com V5.

Usar `apply_patch`. Não usar reparo global, globs mutantes ou a fórmula
incorreta de P1183 que removia `Hash do Código` ao calcular `prompt_hash`.

Nenhuma linha Rust além de `@prompt`/`@prompt-hash` pode mudar. Em `lib.rs`, o
path de `@prompt` já é correto; portanto somente `@prompt-hash` deve variar.

## 5. GREEN focal

Executar:

```text
crystalline-lint --checks v15,v26 --fail-on warning .
crystalline-lint --checks v5 .
crystalline-lint --fix-hashes --dry-run .
```

Aceitação:

- V15 cai exatamente de 23 para 22;
- V26 permanece zero;
- o grupo compartilhado `infra.md` desaparece de V15;
- cada L0 deste lote possui exatamente um consumer;
- nenhum dos dois sources aparece em V5;
- no baseline imutável, V5 cai de 419 para 417; o gate obrigatório é a ausência
  focal, não apenas o total bruto;
- nenhum L0 novo aparece em V7;
- o dry-run continua bloqueado pelas 22 colisões V15 restantes, com exit não
  zero e zero writes.

As demais violações permanecem dívida fora do lote.

## 6. Testes e validação proporcional

Como nenhum corpo Rust muda, não escrever testes novos. Executar:

```text
cargo test -p typst-infra integration_tests
cargo build
git diff --check
git diff --cached --quiet
```

Registrar quantos testes o filtro realmente selecionou. Se selecionar zero,
executar `cargo test -p typst-infra` completo; não declarar cobertura por filtro
vazio.

Comparar cada source antes/depois ignorando somente:

```text
@prompt
@prompt-hash
```

Exigir bytes idênticos no restante. Para `lib.rs`, exigir adicionalmente que o
path `@prompt` permaneça idêntico.

## 7. Entregável de fechamento

Criar:

```text
00_nucleo/diagnosticos/typst-p1184-saneamento-infra-owners.md
```

Registrar:

- proveniência completa da medição;
- RED V15=23/V26=0 e grupo focal de dois consumers;
- conteúdo transferido de P844 e fronteiras dos dois owners;
- paths e hashes finais dos dois pares;
- GREEN V15=22/V26=0;
- V5 antes/depois e ausência focal;
- resultado do dry-run e prova de zero writes;
- testes selecionados e build;
- comparação dos sources fora da linhagem;
- zero Núcleos e zero alteração funcional/pública;
- índice vazio.

## Critérios de aceitação

- `infra.md` possui somente `03_infra/src/lib.rs` como consumer;
- `infra/integration_tests.md` possui somente
  `03_infra/src/integration_tests.rs` como consumer;
- o hub não reivindica comportamento interno dos módulos filhos;
- o owner da suíte não reivindica contratos produtivos dos módulos exercitados;
- história P844 preservada no owner correto;
- nenhum Núcleo criado;
- V15 23→22 e V26=0;
- os dois sources ausentes de V5;
- V5 esperado 419→417 se a árvore permanecer imutável;
- sources idênticos fora das linhas de linhagem;
- testes selecionados e build verdes;
- `git diff --check` limpo e índice vazio;
- nenhuma mudança funcional, pública ou de fase.

## Próximo passo condicionado

Após fechar P1184, regressar ao inventário P1182 e escrever um lote pequeno de
classe A sem Núcleo. Candidato preferencial: individualizar
`testing/math_oracle.md` entre `01_core/src/testing/math_oracle.rs` e
`01_core/src/testing/mod.rs`, condicionado a nova leitura do L0 e dos dois
consumers antes de confirmar P1185.

## Resultado da execução

Fechado em `00_nucleo/diagnosticos/typst-p1184-saneamento-infra-owners.md`.
O lote terminou com V15=22, V26=0, V5=417, 226 testes de integração verdes e
build verde, sem alteração de source fora das linhas de linhagem.
