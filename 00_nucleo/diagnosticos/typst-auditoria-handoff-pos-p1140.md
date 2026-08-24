# Auditoria de handoff após P1140

**Data:** 2026-08-24  
**Objetivo:** preparar a continuação dos passos num novo chat  
**Estado auditado:** P1140 tecnicamente concluído; pendências transferidas

## 1. Proveniência

- HEAD: `a3e72f35b3fc0f5cd765d0dc0ca89990ff1314a5`.
- Branch: `Tekt`, alinhada a `origin/Tekt` no início da auditoria.
- Hora: `2026-08-24T18:25:14-03:00`.
- Working tree no início: limpa; `git diff HEAD --stat` sem saída e
  `git status --short` com zero entradas.
- Baseline vanilla: upstream/main ratificado `a51e02804`.
- Fonte quantitativa:
  `00_nucleo/diagnosticos/superficie-linguagem-p1140.26.json` e
  `superficie-linguagem-p1140.26-probes.json`.

## 2. Estado que deve ser levado ao novo chat

P1140 fecha no escopo que executou: correção da superfície pública inicial e
conclusão da frente `page`. `page` e `std.page` são funções, page-run é lexical,
os 18 deltas suportados chegam ao layout e as probes correspondentes coincidem
com o vanilla.

As divergências abaixo não reabrem P1140. Elas formam a fila a partir de P1141.

Há uma inconsistência documental conhecida: o diagnóstico e o passo P1140.26
foram escritos antes da decisão do dono e ainda dizem que “a série permanece
aberta”. A decisão posterior é encerrá-la e transferir as pendências. O novo
chat deve corrigir esses dois documentos somente se receber os paths completos,
por causa da restrição de leitura de `materialization`.

## 3. Quantidade remanescente

O inventário estrutural mede:

| Classe | Contagem |
|---|---:|
| MATCH | 799 |
| UNVERIFIED_METADATA | 178 |
| MISSING_BINDING | 2 |
| MISSING_MEMBER | 1.155 |
| EXTRA_BINDING | 45 |

Os 1.155 membros ausentes dividem-se em:

| Kind vanilla | Contagem |
|---|---:|
| symbol | 768 |
| function | 334 |
| color | 18 |
| array | 15 |
| alignment | 8 |
| module | 5 |
| direction | 4 |
| float | 2 |
| type | 1 |

Esses totais medem tamanho, não prioridade. Não criar um passo por entrada nem
implementar tabelas manualmente sem comparação mecânica.

As probes observáveis têm 18 coincidências em 29 e 11 divergências:

- globais: `html`, `path`;
- membros: `array.all`, `str.clusters`, `color.black`, `gradient.kind`,
  `datetime.day`, `int.bit-and`, `counter.get`, `content.fields`;
- alias: `emoji.heart`.

## 4. Classificação técnica das 11 probes

### 4.1 `path` — nova entidade e novo contrato

O vanilla define `path` como tipo público em
`foundations/path.rs:136-178`. O constructor resolve strings relativamente ao
`FileId` chamador, preserva raiz de projeto/pacote e impede escape da sandbox.

No cristalino não há `Type::Path`, `Value::Path`, entidade `RootedPath` nem L0
específico. `entities/geometry::PathItem` é geometria vetorial e não pode ser
reutilizado: a homonímia não representa o mesmo domínio.

Essa frente cruza:

- domínio puro do path/root em L1;
- contexto do `FileId` durante eval;
- casts `path | str` em consumidores de arquivos;
- I/O somente nos owners L3 já existentes.

É mudança de contrato público e requer L0 + gate ADR-0127. Não deve começar
pela simples inserção de `scope.define("path", ...)`.

### 4.2 `array.all` e `str.clusters` — semântica de instância já existente

Ambos já funcionam como métodos de instância em
`compiler/stdlib/collections.rs`: `array_all` e `str_clusters` possuem consumers
reais. A divergência medida é a ausência do membro na superfície do valor-tipo
(`array.all`, `str.clusters`).

O trabalho é criar funções não ligadas que recebem `self` explicitamente e
reutilizam os owners existentes. Ainda é contrato público e exige L0 primeiro,
mas o risco algorítmico é baixo. Os dois pertencem ao mesmo owner
`collections.rs` e podem formar um passo atomizado único.

### 4.3 `counter.get` e `content.fields` — implementação de instância existente

`counter_get` já existe em `compiler/stdlib/counter.rs`; `content.fields()` já
é implementado em `eval/bindings/field_access.rs`. Faltam as formas estáticas
que recebem `self` como primeiro argumento.

Apesar do padrão semelhante, os owners são distintos. Devem ser passos
separados ou um passo estritamente dividido por owner, sem mover lógica para
um hub genérico.

### 4.4 `color.black` — constante existente, namespace incompleto

`black` já existe na tabela global de cores em `compiler/stdlib/color.rs`, mas
`color_type_field` só expõe constructors e operadores. O valor pode ser
reutilizado da mesma fonte tabelada; não duplicar os bytes RGB em outro match.

A probe `color.black` é apenas sentinela: o inventário mede 18 cores ausentes
no namespace `color`. A correção adequada é tabelada para todas as cores
ratificadas, não somente `black`.

### 4.5 `gradient.kind` — método ainda ausente

`gradient_type_field` expõe apenas `linear`, `radial` e `conic`. Não foi
encontrado consumer cristalino de `kind`. A entidade `Gradient` já é enum
fechado, portanto o algoritmo tende a ser pequeno, mas é semântica nova e deve
ser medido para todas as variantes e para a forma estática/instância antes da
decisão.

### 4.6 `datetime.day` — accessor ainda ausente

O domínio `Datetime` e seu constructor existem, porém a superfície de
accessors do tipo não está materializada. `day` é só a sentinela; antes de
implementar, inventariar o scope completo de `datetime` para evitar corrigir um
único campo de uma família homogênea.

### 4.7 `int.bit-and` — família bitwise ainda ausente

`Type::Int` atualmente expõe somente `min` e `max` por field access. Não foi
encontrada implementação de `bit-and`. Medir toda a família bitwise vanilla e
materializá-la como uma unidade do owner de inteiros, com casos negativos e
overflow definidos pela linguagem, não por comportamento acidental do Rust.

### 4.8 `emoji.heart` — bloqueio de morfologia, não simples alias

O L0 vigente de emoji limita `Symbol` a um único `char` e deixa sequências
multi-codepoint/variation selectors fora do escopo. `emoji.heart` pertence a
essa classe; adicionar apenas `♥` pode perder a morfologia emoji observável.

Essa correção exige primeiro decidir se `Symbol` passa de `char` para conteúdo
capaz de preservar sequências. É mudança pública e de morfologia; gate
ADR-0127 obrigatório. Não inserir um alias aproximado na tabela.

### 4.9 `html` — frente feature-gated separada

O vanilla só expõe a probe com feature `html`. O repositório já possui L0 de
infra para export HTML, mas isso não legitima automaticamente o módulo público
da linguagem. Manter fora da fila principal até uma auditoria de feature,
defaults de build e superfície completa. Habilitá-lo por defeito seria mudança
de comportamento e requer gate.

## 5. Pendência fora das 11 probes: numbering por função

`page` aceita string/none no cristalino. A fonte vanilla também documenta
numbering por função, inclusive diferenças de aridade entre referências e
numeração visível. Suportar callback exige ampliar o domínio de numbering,
transportá-lo por `PageConfig`, snapshots e `PageStore`, e invocá-lo em contexto
de eval/layout sem introduzir estado global.

Não é correção pequena do constructor `page`; é uma nova frente de callback e
possível interação de fase. Deve ter passo próprio, medição e gate ADR-0127.

## 6. Ordem recomendada dos próximos passos

### P1141 — tipo público `path`

Começar por uma auditoria/nucleação, não por código. Entregáveis do primeiro
passo:

1. medir constructor, `repr`, igualdade, raízes, `.`/`..`, barras invertidas,
   escape de raiz e passagem entre arquivos/pacotes;
2. inventariar todos os consumidores que hoje aceitam apenas string;
3. escrever novos L0s de entidade, eval e consumers;
4. decidir fronteira L1/L3 e parar no gate ADR-0127.

Razão da prioridade: é o único binding global não feature-gated ainda ausente
e requer arquitetura própria.

### Depois de P1141

1. superfície estática de coleções: `array.all` + `str.clusters`;
2. namespace completo das cores, guiado por tabela;
3. `gradient.kind` e família medida;
4. accessors de `datetime` como família;
5. operações bitwise de `int` como família;
6. `counter.get` estático;
7. `content.fields` estático;
8. numbering por função;
9. evolução multi-codepoint de `Symbol` e `emoji.heart`;
10. `html` feature-gated.

A ordem após P1141 pode mudar por dependências medidas, mas cada unidade deve
permanecer no owner da feature. Rebaseline completo após cada cluster, não após
cada entrada de tabela.

## 7. Dívidas que não devem virar implementação automática

- `UNVERIFIED_METADATA` (178): presença sem metadados genéricos equivalentes;
  não prova falha funcional.
- `EXTRA_BINDING` (45): exige classificação língua/mecânica e uso real antes
  de remover; remoção pode quebrar compatibilidade.
- `MISSING_MEMBER` (1.155): inclui grandes tabelas de símbolos; a contagem não
  autoriza implementação indiscriminada.
- avisos do compilador/linter: só abrir frente quando houver violação ou efeito
  mensurável; warnings históricos não são automaticamente parte do próximo
  passo.

## 8. Instrução curta para abrir o novo chat

Usar como pedido inicial:

> Leia `AGENTS.md` e
> `00_nucleo/diagnosticos/typst-auditoria-handoff-pos-p1140.md`.
> Escreva o P1141 para auditar e nuclear o tipo público `path`. Meça primeiro
> contra o vanilla ratificado `a51e02804`, atualize os L0s necessários e pare
> no gate ADR-0127 antes de alterar contrato público.

Esse pedido fornece o único path histórico necessário e evita varrer
`materialization` no novo chat.
