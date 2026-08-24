# Passo 1140.5 — `math.equation.alt` e fechamento do contrato público

**Data:** 2026-08-24  
**Estado:** gate L0 P1140.5-A; aguardando confirmação  
**Origem:** continuação atomizada das frentes D–E do P1140.4  
**Vanilla ratificado:** upstream/main `a51e02804`  
**Gate:** ADR-0127 — obrigatório antes de código

## 1. Objetivo

Completar o campo público `alt` de `math.equation` e fechar a integração dos
cinco named args públicos (`block`, `numbering`, `number-align`, `supplement`,
`alt`) sem confundir três resultados diferentes:

1. aceitar e representar o campo na linguagem;
2. transportar a descrição sem perda até uma fronteira acessível;
3. consumi-la num formato tagueado que Assistive Technology realmente leia.

O passo só declara paridade de acessibilidade se o terceiro observável for
exercitado. Campo aceito mas descartado não fecha a frente.

## 2. Estado inicial medido

Proveniência da escrita: `2026-08-24T09:30:36-03:00`, HEAD
`ffd527c85dd7d547413d33cbc2d27a80e32a3f8c`, working tree não commitada;
`git diff HEAD --stat` registava 51 ficheiros, 1549 inserções e 112 remoções
acumuladas de P1140.4-A–C e documentação relacionada.

### 2.1 Vanilla

- `lab/typst-original/crates/typst-library/src/math/equation.rs:112-126`
  declara `alt: Option<EcoString>`: descrição natural da equação para
  Assistive Technology.
- `equation.rs:167-170` sintetiza também `locale`, usado na descrição
  alternativa automática quando aplicável.
- `lab/typst-original/crates/typst-pdf/src/tags/resolve/mod.rs:303-306`
  transforma `GroupKind::Formula` em tag semântica `Formula`, copia `alt` e
  preserva placement block/inline.
- Portanto, o observável não é texto visual nem `plain_text`: pertence à
  árvore estrutural do documento/PDF tagueado.

### 2.2 Cristalino

- `native_math_equation` ainda rejeita `alt` como argumento inesperado.
- O braço especial de `#set math.equation` ainda não avalia `alt`.
- `EquationElem` permanece atomizado em `{ body, block }`; P1140.4-A–C usa
  style chain e stores por Location para campos públicos não estruturais.
- `FrameItem` contém geometria/desenho e links, mas a varredura inicial não
  encontrou um equivalente a `GroupKind::Formula`, structure tree ou tag
  semântica de fórmula.
- ADR-0126 separa explicitamente export PDF verboso/compacto do eixo de PDF
  tagueado (`00_nucleo/adr/typst-adr-0126-modo-verboso-primeiro.md:150`).

**Inferência:** aceitar `alt` hoje é menor que implementar PDF/UA. Esta
inferência é refutada se a medição detalhada da frente A encontrar um canal
semântico já existente e consumido pelo exportador.

## 3. Pergunta arquitetural

Qual é a menor unidade durável que preserva `alt` do constructor até o
consumidor acessível sem assar metadados semânticos em glifos ou texto visual?

Antes de escolher representação, medir:

- tipos semânticos existentes entre `Content → layout → PagedDocument → PDF`;
- tratamento atual de `Image.alt`, links e qualquer metadado de acessibilidade;
- se HTML ou outro exportador cristalino já consome descrições alternativas;
- se `alt` explícito participa de `repr`, fields, selectors, query e show;
- comportamento vanilla de ausência, `none`, string vazia, set-rule e locale;
- requisitos do vanilla para PDF normal versus PDF/UA.

Não escolher `FrameItem::Text`, `/ActualText`, annotation, tooltip ou metadado
PDF global por conveniência. Cada um precisa de prova de que corresponde ao
observável da linguagem.

## 4. Frentes atomizadas

### P1140.5-A — medição e gate L0

Executar probes no vanilla pinado e no cristalino:

```typst
repr(math.equation(alt: "x squared", [x^2]))
repr(math.equation(alt: none, [x]))
repr(math.equation(alt: "", [x]))
math.equation(alt: [x], [x])
math.equation(alt: 1, [x])
#set math.equation(alt: "description")
#set math.equation(alt: none)
```

Medir também `fields()`, acesso `it.alt`, `query(math.equation)` e combinação
com os quatro campos já implementados. Para o exportador vanilla, comparar
artefato sem `alt`, com `alt` e com `alt: ""`, em modo padrão e no padrão de
acessibilidade disponível; inspecionar a structure tree, não bytes totais.

Atualizar os L0 donos somente depois da medição:

- `compiler/stdlib/structural/math.md`;
- `compiler/eval.md` e bindings de field access;
- `entities/elements/equation.md`;
- os L0 do transporte semântico e do exportador que a medição identificar.

Como há contrato público e possível nova fase de pipeline, ressellar hashes e
**parar para confirmação humana** antes de RED/código.

### P1140.5-B — contrato de linguagem e morfologia

Após o gate:

- constructor e set-rule aceitam exatamente o domínio medido;
- erros de avaliação e cast propagam sem `.ok()` ou descarte;
- named omitido herda; valor explícito preserva sua semântica;
- `repr` mantém ordem pública `block`, `numbering`, `number-align`,
  `supplement`, `alt`, `body`;
- callbacks/show/query veem o valor efetivo somente se a linguagem vanilla o
  expuser nesse ponto;
- `alt` não altera `plain_text`, largura, baseline, paginação ou número.

Escrever testes RED antes da implementação. Não adicionar `alt` a
`EquationElem` automaticamente: decidir dado versus style chain a partir dos
consumidores medidos, preservando atomização e ADR-0107.

### P1140.5-C — transporte semântico até a fronteira de exportação

Introduzir ou reutilizar uma unidade semântica explícita para fórmula que
carregue:

- descrição alternativa opcional;
- placement block/inline quando o consumidor precisar;
- descendentes visuais já produzidos, sem relayout;
- associação estável à equação/Location quando necessária.

A unidade deve nascer na camada dona e atravessar L1→L3 por contrato, não por
import reverso. `04_wiring` apenas compõe. Não duplicar toda a árvore de
`Content`, não colocar callbacks em L3 e não executar eval no exportador.

Gate de aceitação de C: uma probe demonstra que o valor chega intacto à API de
exportação, mesmo que o consumidor PDF tagueado seja faseado.

### P1140.5-D — consumidor acessível ou fase nomeada

Se já existir infraestrutura tagueada suficiente, implementar o consumidor e
validar semanticamente:

- fórmula aparece como tag `Formula` ou equivalente normativo;
- `alt` não vazio aparece como descrição alternativa da fórmula;
- ausência/`none` não inventa descrição explícita;
- block/inline preserva placement quando observável;
- conteúdo visual continua idêntico com e sem `alt`.

Se a medição provar que isso exige uma nova árvore estrutural/PDF-UA, **não
expandir silenciosamente este passo**. Fechar B–C como transporte comprovado,
abrir um passo sucessor nomeado para PDF tagueado e registrar que a paridade de
acessibilidade permanece aberta. ADR-0126 proíbe misturar esse eixo com o modo
de stream verboso/compacto.

### P1140.5-E — fechamento transversal de `math.equation`

Revalidar todos os cinco named args em constructor e set-rule:

- nenhum argumento reconhecido é ignorado;
- combinação de campos preserva todos os deltas;
- `$...$`, show/selectors/query, labels e referências não regridem;
- `repr`, `fields`, hash/eq/map refletem somente os campos realmente donos de
  cada unidade;
- inventário P1140 deixa de marcar metadata de `math.equation` como não
  verificada somente após probes reproduzíveis;
- atualizar diagnóstico final em `00_nucleo/diagnosticos/`.

## 5. Testes obrigatórios

### Linguagem

- constructor: ausência, string, `none`, vazio e tipos inválidos medidos;
- set-rule: string/`none`, erro de variável indefinida e escopo léxico;
- `repr` isolado e combinado com numbering/align/supplement;
- acesso a fields/show/query conforme vanilla;
- `plain_text` e geometria invariantes com/sem `alt`.

### Pipeline

- API pública de compilação recebe documento com `alt` sem bypass;
- transporte mantém Unicode e string vazia sem normalização acidental;
- callbacks permanecem em L1 e L3 recebe somente dados materializados;
- teste do consumidor acessível inspeciona estrutura semântica do artefato;
- se o consumidor for faseado, teste de fronteira prova até onde o dado chega.

### Regressão

- testes P1140.4-A–C;
- duas falhas P862 preexistentes continuam registradas, não absorvidas;
- suíte L1, suíte L3, `cargo build`, `crystalline-lint .`,
  `cargo fmt --check` e `git diff --check`.

## 6. Critérios de aceitação

- [ ] Medição precede toda decisão e contém `file:line` + proveniência.
- [ ] L0 de cada módulo tocado existe, foi atualizado e ressellado.
- [ ] Gate ADR-0127 confirmado antes de código.
- [ ] `alt` tem domínio/default/repr compatíveis com o vanilla pinado.
- [ ] `alt` não muda o render visual nem `plain_text`.
- [ ] Nenhum campo de `math.equation` é ignorado silenciosamente.
- [ ] O dado chega a uma fronteira de exportação comprovada.
- [ ] Paridade acessível só é declarada com inspeção da estrutura tagueada.
- [ ] Se PDF tagueado faltar, há passo sucessor explícito e estado aberto.
- [ ] Relatório final fica em `00_nucleo/diagnosticos/`.
- [ ] Travas finais passam ou falhas preexistentes/ambientais são registradas.

## 7. Fora de escopo

- inferir automaticamente descrição natural de matemática quando `alt` está
  ausente, sem prova no vanilla;
- usar `plain_text` da fórmula como substituto inventado para `alt`;
- implementar toda conformidade PDF/UA dentro desta frente por arrasto;
- misturar tagging com `StreamMode::Verbose`/`Compact`;
- alterar algoritmos de layout matemático, numbering ou referências;
- corrigir P862 ou falhas ambientais de rede do sandbox;
- reestruturar `EquationElem` por igualdade mecânica com Rust vanilla.

## 8. Resultado esperado

Ao final, `math.equation.alt` deixa de ser argumento ausente e passa a ter
linhagem verificável desde a linguagem até a fronteira acessível. O projeto
saberá, com prova, se a descrição já é consumida por um artefato tagueado ou se
o consumidor pertence a um passo PDF/UA separado — sem chamar transporte de
paridade completa e sem transformar geometria em semântica por acidente.

## 9. Estado de execução — gate P1140.5-A

Medição concluída em 2026-08-24 contra o pin `a51e02804`. O contrato confirmou
`string | none`, string vazia distinta, repr/fields/query realizados e erros
de cast exatos. O vanilla gera PDF tagueado por padrão com tag Formula e Alt;
o cristalino gera `Tagged: no` e não possui structure tree.

Decisão: P1140.5 fecha linguagem + query + transporte semântico explícito até
L3, sem tagging parcial. O consumidor PDF tagueado foi nomeado P1140.6. Os L0
donos foram atualizados para `equation.alt`, visão realizada por Location e
`FrameItem::Semantic(Formula)` transparente ao render. Diagnóstico completo:
`00_nucleo/diagnosticos/typst-p1140.5-alt-e-tagging.md`.

Como há mudança de contrato público e introdução de uma nova fase semântica
L1→L3, a execução para aqui no gate ADR-0127. Ainda não existem teste RED nem
código de P1140.5.

## 10. Execução após confirmação do gate — 2026-08-24

O dono confirmou a continuação. Foram implementados o contrato `string | none`,
repr e set-rule de `alt`; a captura por Location; a visão realizada usada por
query; e o envelope `FrameItem::Semantic { kind: Formula, placement, alt,
items }`. Shaping, métricas, layout, raster, SVG e PDF atravessam o envelope
recursivamente sem emitir tagging. P1140.6 continua dono de MCID, ParentTree,
StructTreeRoot e PDF/UA.

Proveniência da validação: working tree não commitado sobre
`ffd527c85dd7d547413d33cbc2d27a80e32a3f8c`, medido em
`2026-08-24T09:54:34-03:00`; `git diff HEAD --stat` registrava 76 ficheiros,
2363 inserções e 133 remoções, incluindo o lote P1140.4 já presente.

- `cargo check --workspace`: passou.
- `cargo test -p typst-core p11405 -- --nocapture`: 4 passaram.
- `crystalline-lint .`: zero violations (avisos informativos existentes).
- suíte L1 completa: 5106 passaram e 38 falharam. Duas falhas são P862
  preexistentes; as outras 36 são testes de inspeção antigos que só percorrem
  `page.items` no primeiro nível e, portanto, não veem os mesmos glifos agora
  contidos em `Semantic.items`. O próprio output de P994 mostra os filhos
  visuais preservados dentro do envelope.

Estado: implementação funcional integrada, mas o passo permanece aberto até
os 36 testes consumidores migrarem para percurso recursivo e as suítes L1/L3
e travas finais serem repetidas.

Correção e fechamento delegados ao passo
`00_nucleo/materialization/typst-passo-1140.7.md`, que distingue walkers de
inspeção, geometria e export e tem como gate retornar ao baseline exclusivo das
duas falhas P862.
