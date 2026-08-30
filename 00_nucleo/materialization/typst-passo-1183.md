# P1183 — individualizar os owners de `Title` e reclassificar a metodologia

**Data:** 2026-08-25  
**Estado:** `EXECUTADO — GREEN EM 2026-08-25`  
**Dependências:** ADR-0129; P1182 aprovado  
**Lote:** D-title, primeiro lote de saneamento da bijeção L0  
**Classe ADR-0127:** correção interna de linhagem, sem contrato público,
default, fase ou compatibilidade; fluxo contínuo, L0 primeiro

## Objetivo

Eliminar exatamente uma das 24 colisões V15 atuais:

```text
00_nucleo/prompts/compiler/stdlib_audit_methodology.md
├── 01_core/src/entities/elements/title.rs
└── 01_core/src/compiler/layout/title.rs
```

Estado final pretendido:

```text
00_nucleo/prompts/entities/elements/title.md
↔ 01_core/src/entities/elements/title.rs

00_nucleo/prompts/compiler/layout/title.md
↔ 01_core/src/compiler/layout/title.rs

00_nucleo/diagnosticos/typst-stdlib-audit-methodology.md
   (processo histórico, sem ownership produtivo)
```

Não criar Núcleo Tekt: entidade, layout e metodologia possuem responsabilidades
distintas e não há claim compartilhada que justifique um quarto artefato.

## Baseline medido

Em P1182, sobre HEAD `00f402e875956304aa435f749a251f359287e2ba` e binário
SHA-256
`eb7494979040e70feb6ac3b738c86979488b8c26927480126746aae2ff707c9d`:

- V15=24, V26=0;
- o grupo `stdlib_audit_methodology.md` tem exatamente dois consumers;
- `title.rs` de entidade tem hash de código `ef2b41ab` no manifesto P1182;
- `title.rs` de layout tem hash de código `88f73a98`;
- o prompt atual tem hash efetivo `4b9ef0be` e zero metadata canônica;
- os dois consumers ainda têm `@prompt-hash 0683fad7`;
- V5 inclui exatamente esses dois consumers entre os 421 achados globais;
- os dois V7 independentes são `_convencoes.md` e `shell/custom-ca-cert.md`.

### Evidência da associação incorreta

`stdlib_audit_methodology.md:12-68` define uma metodologia de varredura,
declara em `:7` que não gera código e em `:51` que não legitima alterações de
código.

Em contraste:

- `entities/elements/title.rs:17-58` é a entidade pública `TitleElem`, seu body,
  constructor e implementação de `Element`;
- `compiler/layout/title.rs:16-41` é exclusivamente a materialização visual do
  título;
- `compiler/stdlib/structural/title.md` já possui ownership separado do
  constructor de linguagem `title(...)`; ele não deve absorver entidade ou
  layout.

## Restrições

- não acessar/listar `00_nucleo/context/` ou `00_nucleo/materialization/`;
- não alterar nenhum comportamento, assinatura, teste ou corpo Rust;
- não editar `compiler/stdlib/structural/title.md`;
- não criar Núcleo Tekt;
- não atualizar outros prompts compartilhados;
- não executar `--fix-hashes` mutante global: as outras 23 colisões V15 devem
  continuar bloqueando esse comando;
- não corrigir os demais 419 V5 esperados;
- não atualizar referências históricas em diagnósticos antigos;
- preservar a working tree P1181/P1182 e manter o índice vazio.

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

Exigir output byte-idêntico, V15=24, V26=0 e presença do grupo de dois
consumers. Esse V15 é o RED do lote. Se proveniência ou cardinalidade mudar,
atualizar a medição antes de decidir.

Guardar logs em `/tmp`. Provar que os comandos não alteraram a árvore.

## 2. Redigir primeiro os dois Prompts L0 proprietários

### 2.1 `entities/elements/title.md`

Criar:

```text
00_nucleo/prompts/entities/elements/title.md
```

O L0 deve legitimar exclusivamente
`01_core/src/entities/elements/title.rs` e conter:

- título e path do consumer proprietário;
- exatamente uma linha canônica `Hash do Código` no preâmbulo;
- `TitleElem { body: Content }` como entidade pública;
- `TitleElem::new(body)` sem default ou I/O;
- `Element::plain_text()` delegado ao body;
- `map_content` e `map_text` preservando a variante `Content::Title`;
- `get_field("body") → Value::Content` e campos desconhecidos → `None`;
- testes focais de texto e field access já existentes;
- scope-out explícito: parsing/constructor `title(...)`, fallback de
  `document.title`, layout 1.7em/bold e introspecção pertencem a outros owners;
- zero promessa nova além do código e dos L0s vigentes.

Aceitação em nível de linguagem: o título conserva identidade semântica e seu
body através das transformações de conteúdo.

### 2.2 `compiler/layout/title.md`

Criar:

```text
00_nucleo/prompts/compiler/layout/title.md
```

O L0 deve legitimar exclusivamente
`01_core/src/compiler/layout/title.rs` e conter:

- exatamente uma linha canônica `Hash do Código` no preâmbulo;
- forma B da ADR-0109: free function `title::layout`, chamada pelo match
  exaustivo do layout;
- entrada `&mut Layouter` e `&TitleElem`;
- escala `1.7 × style.size` medida em P765a;
- estilo bold, não italic, preservando os demais defaults vigentes do
  `TextStyle` usado pelo código;
- flush anterior somente quando o cursor já avançou além da margem esquerda;
- layout do body, flush posterior e restauração integral do estilo anterior;
- aceitação morfológica: título aparece em bloco próprio, maior e em negrito;
- scope-out: construção de `TitleElem`, resolução de argumento/metadado,
  entidade, HTML e qualquer mudança de estilo futura.

Não duplicar a metodologia P765 nem o contrato do constructor estrutural.

## 3. Reclassificar a metodologia como diagnóstico

Criar, preservando o conteúdo histórico substantivo:

```text
00_nucleo/diagnosticos/typst-stdlib-audit-methodology.md
```

Transformações documentais permitidas:

- trocar o título de “Prompt L0” para “Metodologia histórica”;
- remover `Hash do Código` e qualquer alegação de ownership/materialização;
- registrar o path anterior e a reclassificação P1183;
- manter decisões, critérios, exemplos e proveniência histórica;
- marcar que ADR-0107/0108 e regras atuais vencem em caso de divergência.

Depois de criar e verificar o diagnóstico, remover
`00_nucleo/prompts/compiler/stdlib_audit_methodology.md`. Não atualizar
diagnósticos históricos que citam o path antigo; essas referências descrevem
o estado da época.

Antes da remoção, `rg` deve provar que os únicos consumers produtivos do path
antigo são os dois headers deste lote. Referências em relatórios/diagnósticos
não bloqueiam a reclassificação.

## 4. Resselo restrito e independente

Como V15 global ainda contém 23 colisões após este lote, não usar o reparador
global. Calcular independentemente, conforme o algoritmo pinado do linter:

```text
prompt_hash = SHA256(prompt sem a linha canônica "Hash do Código:")[0..8]
code_hash   = SHA256(source sem a própria linha "@prompt-hash")[0..8]
```

Para cada par:

1. congelar prompt path, consumer path e hashes anteriores num manifest em
   `/tmp`;
2. escrever `Hash do Código: <code_hash>` no prompt proprietário;
3. escrever `@prompt <novo-path>` no consumer;
4. escrever `@prompt-hash <prompt_hash>` no consumer;
5. não modificar nenhuma outra linha do source;
6. recalcular independentemente os dois sentidos e exigir igualdade.

Usar `apply_patch` para as alterações explícitas. Não usar substituição global,
globs mutantes ou scripts que atravessem outros headers.

## 5. GREEN focal

Executar:

```text
crystalline-lint --checks v15,v26 --fail-on warning .
crystalline-lint --checks v5 .
```

Aceitação:

- V15 cai exatamente de 24 para 23;
- V26 permanece zero;
- o antigo grupo `stdlib_audit_methodology.md` desaparece;
- nenhum dos dois `title.rs` aparece em V5;
- os outros achados permanecem dívida fora do lote;
- nenhum prompt novo é V7;
- o diagnóstico reclassificado não é descoberto como prompt;
- `--fix-hashes --dry-run .` continua bloqueado pelas 23 colisões, com exit
  não zero e zero writes.

Não usar o total bruto de V5 como único gate. No baseline imutável espera-se
421→419, mas o critério obrigatório é ausência dos dois paths do lote e
preservação explicável dos restantes.

## 6. Testes e validação proporcional

Como nenhum corpo Rust muda, não escrever novos testes nem fabricar RED
funcional. Executar os testes focais existentes:

```text
cargo test -p typst-core entities::elements::title
cargo test -p typst-core compiler::layout::tests -- title
cargo build
git diff --check
git diff --cached --quiet
```

Se o filtro de layout não selecionar teste específico, registrar zero testes
selecionados e executar a suíte `typst-core` completa; não declarar cobertura
por um filtro vazio.

Comparar os dois sources antes/depois ignorando somente as linhas `@prompt` e
`@prompt-hash`. Exigir bytes idênticos no restante.

## 7. Entregável de fechamento

Criar:

```text
00_nucleo/diagnosticos/typst-p1183-saneamento-title-owners.md
```

Registrar:

- proveniência e estado da árvore;
- RED V15=24;
- paths e hashes dos dois pares novos;
- prova de reclassificação da metodologia;
- GREEN V15=23/V26=0;
- V5 antes/depois e ausência dos dois title paths;
- testes/build;
- comparação dos sources fora de lineage;
- confirmação de zero Núcleos e zero alteração funcional;
- índice vazio.

## Critérios de aceitação

- dois Prompts L0 novos, cada qual com exatamente um consumer;
- prompt de entidade não contém contrato de layout/constructor;
- prompt de layout não contém contrato de entidade/constructor;
- metodologia existe somente em `diagnosticos/`, sem `Hash do Código`;
- path antigo removido de `prompts/` e de todos os headers produtivos;
- nenhum Núcleo criado;
- V15 24→23 e V26=0;
- `title.rs` entidade/layout ausentes de V5;
- sources idênticos fora das duas linhas de lineage;
- testes focais e build verdes;
- `git diff --check` limpo e índice vazio;
- nenhuma mudança funcional ou pública.

## Próximo passo condicionado

Após fechar P1183, escrever P1184 para o próximo lote sem Núcleo e de baixo
risco: separar `infra.md` entre o owner do hub `03_infra/src/lib.rs` e o owner
dos testes `03_infra/src/integration_tests.rs`.

## Resultado da execução

Fechado em `00_nucleo/diagnosticos/typst-p1183-saneamento-title-owners.md`.
Durante o resselo, a medição do linter corrigiu uma premissa deste plano: o
`@prompt-hash` vigente é SHA-256 do L0 completo, incluindo a linha canônica
`Hash do Código`; a remoção dessa linha descrita na secção 4 não corresponde ao
algoritmo executado. Os hashes efetivos validados pelo V5 foram usados.
