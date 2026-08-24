# Passo 1140.4 — completar o contrato público de `math.equation` por eixos observáveis

**Data:** 2026-08-23  
**Origem:** scope-out explícito de P1140.3-B  
**Vanilla ratificado:** `upstream/main a51e02804`  
**Natureza:** paridade pública de linguagem, dividida por consumidor  
**Gate:** ADR-0127 antes de cada campo que amplie contrato/default/pipeline

## 1. Objetivo

Completar os quatro named args públicos que P1140.3-B mediu e deixou
explicitamente incompletos em `math.equation`:

1. `numbering`;
2. `number-align`;
3. `supplement`;
4. `alt`.

O passo não trata os quatro nomes como uma única alteração de struct. Cada um
tem consumidores e observáveis diferentes e deve fechar em RED → GREEN
separado. A função mínima `math.equation(body, block:)` entregue por P1140.3-B
permanece a base comum.

## 2. Proveniência inicial

Estado medido no fechamento de P1140.3:

- HEAD: `a8959bd184871d72f470ee4dd06d829e6ff0e483`;
- working tree não commitada;
- hora: `2026-08-23T22:45:32-03:00`;
- `git diff HEAD --stat`: 25 ficheiros, 331 inserções, 43 remoções; ficheiros
  novos ainda não rastreados não entram nessa contagem;
- inventário release: `WRONG_KIND == 0`; `math.equation` passou para
  `UNVERIFIED_METADATA`;
- vanilla: `lab/typst-original/target/release/typst`, baseline `a51e02804`.

Repetir HEAD, hora e `git diff HEAD --stat` antes de usar qualquer número para
fechar este passo.

## 3. Medição anterior à decisão

### 3.1 Contrato vanilla já confirmado

```text
repr(math.equation(numbering: "(1)", block: true, [x]))
→ equation(block: true, numbering: "(1)", body: [x])

repr(math.equation(number-align: bottom, [x]))
→ equation(number-align: bottom, body: [x])

repr(math.equation(supplement: [Eq.], [x]))
→ equation(supplement: [Eq.], body: [x])

repr(math.equation(alt: "x", [x]))
→ equation(alt: "x", body: [x])
```

Também foi confirmado:

- `body` é obrigatório;
- segundo positional é `unexpected argument`;
- named desconhecido é `unexpected argument: <nome>`;
- default de `block` é `false`;
- default de `number-align` é `end + horizon` e é omitido do `repr`;
- numbering só conta e renderiza número para equação de bloco.

### 3.2 Estado cristalino

- `EquationElem` contém somente `body` e `block`;
- `numbering` de set-rule vive apenas em
  `StyleChain::custom("equation.numbering")`, divergência mecânica já
  autorizada pela ADR-0107;
- `layout/equation.rs` lê esse custom para decidir/formatar a numeração;
- o braço dedicado de `#set math.equation(...)` em `eval/rules.rs` trata
  somente `numbering` e retorna cedo; `block`, `number-align`, `supplement` e
  `alt` são atualmente ignorados por esse caminho;
- `number-align` não chega ao posicionamento do número;
- `supplement` não chega ao payload/ref de equação;
- `alt` não chega à árvore acessível/exportadores;
- a nativa P1140.3-B rejeita os quatro campos, conforme o L0 faseado.

Essas diferenças são semântica/morfologia da linguagem, não obrigação de
copiar a estrutura Rust vanilla.

## 4. Auditoria L0 obrigatória

Antes de qualquer teste RED, ler integralmente e confirmar os hashes dos L0
dos módulos efetivamente afetados, no mínimo:

- `00_nucleo/prompts/compiler/stdlib/structural/math.md`;
- `00_nucleo/prompts/compiler/stdlib/structural.md`;
- `00_nucleo/prompts/entities/elements/equation.md`;
- `00_nucleo/prompts/compiler/eval.md`;
- L0 de `eval/rules.rs`/set-rules;
- L0 de `compiler/layout/equation.rs`;
- L0 de introspecção, referências e acessibilidade somente quando cada eixo
  realmente os tocar.

Se a auditoria decidir criar `compiler/stdlib/structural/math/equation.rs`,
redigir antes o L0 próprio
`00_nucleo/prompts/compiler/stdlib/structural/math/equation.md`. Não criar o
arquivo de código apenas para reduzir LOC: a fronteira precisa ser sustentada
pela unidade pública `math.equation` e por seus testes.

## 5. Atomização do trabalho

### P1140.4-A — `numbering` no construtor

Aceitar no construtor o mesmo domínio medido no vanilla: padrão, função ou
`none`. Preservar a decisão vigente de transportar o valor pela style chain;
não adicionar `numbering` ao `EquationElem` apenas para copiar a mecânica
vanilla.

O conteúdo produzido pelo construtor deve transportar o estilo junto da
equação sem alterar o estilo exterior. Inline com numbering continua sem
contagem/render de número; block com numbering participa do contador,
referências e layout. `repr` deve revelar o named arg na morfologia do elemento,
mesmo que o dado esteja mecanicamente em `Content::Styled`.

Medir antes de decidir:

```typst
repr(math.equation(numbering: "(1)", [x]))
repr(math.equation(numbering: none, [x]))
repr(math.equation(numbering: n => str(n), block: true, [x]))
math.equation(numbering: 1, [x])
#set math.equation(numbering: "(I)")
```

### P1140.4-B — `number-align`

Materializar o tipo de alinhamento aceito e o default `end + horizon`. Medir
combinações horizontais/verticais, direção RTL e o que ocorre quando a equação
não tem número. O consumidor dono é o posicionamento da numeração em
`compiler/layout/equation.rs`; não mover lógica de layout para a entidade.

Probes mínimos:

```typst
repr(math.equation(number-align: bottom, [x]))
repr(math.equation(number-align: left + top, [x]))
math.equation(number-align: 1, [x])
#set math.equation(number-align: bottom)
```

Comparar posições por observável de língua/render, com proveniência de páginas
ou coordenadas; não usar igualdade de frames Rust como critério.

### P1140.4-C — `supplement`

Medir e implementar `auto`, `none`, conteúdo e função conforme o cast vanilla.
O supplement é observável em referências a equações numeradas; não basta
armazená-lo ou fazê-lo aparecer no `repr`. Definir fonte única entre named arg,
set-rule, introspecção e `@label`.

Probes mínimos:

```typst
repr(math.equation(supplement: auto, [x]))
repr(math.equation(supplement: none, [x]))
repr(math.equation(supplement: [Eq.], [x]))
#set math.equation(numbering: "(1)", supplement: [Eq.])
$ x $ <eq-x>
@eq-x
```

Medir locale/default `auto` antes de decidir sua representação. Não inventar
string fixa global.

### P1140.4-D — `alt`

Aceitar `str` ou `none` conforme o vanilla e transportar a descrição até o
consumidor de acessibilidade. `alt` não substitui `plain_text`, não altera o
layout visual e não deve ser descartado pelo exportador tagueado/HTML quando
esses pipelines suportarem o observável.

Probes mínimos:

```typst
repr(math.equation(alt: "x squared", [x^2]))
repr(math.equation(alt: none, [x]))
math.equation(alt: [x], [x])
#set math.equation(alt: "description")
```

Se o pipeline acessível necessário ainda não existir, separar claramente:
armazenamento/morfologia agora e consumo num passo nomeado. Não declarar
paridade de acessibilidade com um campo morto.

### P1140.4-E — set/show, reflexão e fechamento

Eliminar o retorno antecipado que silencia campos diferentes de `numbering`
em `#set math.equation`. Todos os cinco named args devem ser ou aplicados ou
rejeitados por diagnóstico correto; nenhum pode ser ignorado.

Revalidar:

- `#set math.equation(...)` por campo e em combinações;
- `#show math.equation: ...`;
- `query(math.equation)` e selectors;
- `repr`, `type` e metadados de parâmetros do binding;
- sintaxe `$...$` inline/block;
- numbering, referências e labels;
- map/hash/eq da entidade para qualquer campo que nela seja armazenado.

## 6. Forma arquitetural

- O match exaustivo sobre `Content` permanece.
- Construção/validação da função fica na unidade stdlib de `math.equation`.
- Dados semânticos pertencem à entidade ou style chain conforme a decisão L0;
  lógica de layout permanece em `compiler/layout/equation.rs`.
- Não introduzir `dyn`, registry, `PropMap` ou import reverso
  `entities → compiler`.
- `numbering` continua na chain salvo nova decisão explícita que revogue a
  divergência mecânica vigente.
- Atualizar `map_content`, `map_text`, `Hash`, `PartialEq`, payload e `repr`
  somente para campos realmente armazenados na entidade.

## 7. Gates ADR-0127

Cada frente altera contrato público, default ou consumidor do pipeline. Para
A–D:

1. medir vanilla e o cristalino;
2. atualizar o L0 dono;
3. ressellar hashes;
4. **parar para confirmação humana**;
5. somente depois escrever RED e código.

Uma confirmação de A não autoriza B–D. Correções internas descobertas durante
uma frente podem seguir em fluxo contínuo apenas se couberem expressamente nas
classes dispensadas pela ADR-0127.

## 8. Sequência de execução

1. Repetir proveniência e probes completos.
2. Auditar a representação de alinhamento, numbering, supplement e alt já
   disponível em L1.
3. Decidir se a unidade stdlib merece arquivo próprio e escrever seu L0.
4. Executar P1140.4-A em L0 → gate → RED → GREEN.
5. Repetir o ciclo separadamente para B, C e D.
6. Executar E para integração set/show/reflexão.
7. Regenerar inventário e probes; `WRONG_KIND` deve permanecer zero e
   `math.equation` só deixa `UNVERIFIED_METADATA` com prova dos metadados.
8. Escrever relatório em `00_nucleo/diagnosticos/`.
9. Rodar testes focados, suíte L1, `cargo build`, `crystalline-lint .`,
   `cargo fmt --check` e `git diff --check`.

## 9. Critérios de aceitação

- [ ] L0 medido e ressellado antes de cada frente.
- [ ] Gates A–D confirmados separadamente.
- [ ] Os cinco named args públicos do vanilla são aceitos com tipos/defaults
      medidos.
- [ ] Nenhum named arg de constructor ou set-rule é ignorado silenciosamente.
- [ ] `numbering` mantém gate block-level e fonte única.
- [ ] `number-align` altera a posição do número nos casos observáveis.
- [ ] `supplement` participa de referências e respeita `auto`/locale.
- [ ] `alt` chega ao consumidor acessível ou fica em fase nomeada sem alegação
      de paridade completa.
- [ ] `repr` omite defaults e expõe valores customizados como o vanilla.
- [ ] `$...$`, show/selectors/query, labels e referências não regridem.
- [ ] `WRONG_KIND == 0` permanece.
- [ ] Metadados só são marcados verificados a partir de probes reproduzíveis.
- [ ] Relatório final fica em `00_nucleo/diagnosticos/`.
- [ ] Testes/travas finais passam; falhas preexistentes são registradas.

## 10. Fora de escopo

- `math.root` e demais `MISSING_MEMBER`;
- mudanças no algoritmo geral de layout matemático sem relação com a posição
  da numeração;
- copiar tipos/traits internos do vanilla por igualdade mecânica;
- corrigir as duas falhas preexistentes P862;
- declarar acessibilidade completa se o consumidor final não for exercitado;
- iniciar nova família do inventário P1140 antes de fechar ou fasear
  explicitamente A–E.

## 11. Resultado esperado

`math.equation` deixa de ser apenas uma função de kind correto e passa a ter o
contrato público completo, com cada campo ligado ao consumidor que lhe dá
significado. O passo preserva a atomização: constructor, estilo, layout,
referência e acessibilidade cooperam por contratos explícitos, sem concentrar
toda a lógica no `EquationElem` nem no hub `structural/math.rs`.
