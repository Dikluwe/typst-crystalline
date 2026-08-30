# P1214 — paridade dos métodos públicos de `bytes`

**Estado:** EXECUTADO — PARCIAL (VALORES GREEN; SPANS DIAGNÓSTICOS ABERTOS)  
**Data da redação:** 2026-08-26  
**Origem:** primeira linha da fila missing-first do P1213.  
**Vanilla autoritativo:** `a51e02804`.  
**Relação no mapa:** `language-bytes`, atualmente `parcial`.

## 1. Objetivo

Fechar a superfície pública de métodos do valor Typst `bytes` demonstrada no
vanilla ratificado:

```text
bytes.len()
bytes.at(index, default: ...)
bytes.slice(start, end?, count: ...)
```

O passo não altera a representação Rust de `Bytes`, não toca o tipo de
transporte `world_types::Bytes` e não implementa capacidades que o vanilla não
expõe. O critério é equivalência de valores e diagnósticos observáveis, não
igualdade da mecânica interna.

## 2. Medição que autoriza o passo

P1213 executou:

| Expressão | vanilla | cristalino |
|---|---|---|
| `bytes((1,2,3)).len()` | `3` | tipo bytes não tem método `len` |
| `bytes((1,2,3)).at(1)` | `2` | tipo bytes não tem método `at` |
| `bytes((1,2,3)).slice(1)` | `bytes(2)` | tipo bytes não tem método `slice` |

Fonte vanilla:
`lab/typst-original/crates/typst-library/src/foundations/bytes.rs:229-302`.
A fonte declara exatamente `construct`, `len`, `at` e `slice` no scope do
tipo. `construct` já existe no cristalino; o RED atual são os três métodos.

O mapa P1213 também retirou a ambiguidade estrutural:

```text
typst_library::foundations::bytes::Bytes
  → typst_core::entities::bytes::Bytes
```

`typst_core::entities::world_types::Bytes` continua sendo transporte de
ficheiro/World e está fora deste contrato.

## 3. Regime e gate L0

Usar protocolo completo de materialização segregada porque há comportamento
de linguagem, matriz de diagnósticos e promoção posterior no mapa.

Classificação ADR-0127: correção de paridade vanilla em fluxo contínuo. Antes
do código, atualizar e ressellar os L0s proprietários, sem parada humana, desde
que a medição não descubra necessidade de novo tipo público, mudança de fase,
default geral ou quebra de compatibilidade. Se descobrir, parar no gate.

Owners a auditar integralmente e atualizar somente conforme responsabilidade:

- `00_nucleo/prompts/entities/bytes.md` — retirar `len/at/slice` do scope-out e
  especificar a semântica pública;
- `00_nucleo/prompts/compiler/stdlib/collections.md` — owner já usado pelo
  dispatch genérico de `array`, `dict` e `str`; a auditoria causal confirmou
  que `call_dispatch` já passa `Value`, método e `Args` avaliados a este owner.

`field_access`, `method_dispatch`, `value_methods` e `call_dispatch` foram
auditados, mas não recebem obrigação: nenhum precisa mudar para este contrato.

Não acrescentar a obrigação ao quarto owner por conveniência. A auditoria deve
preservar cardinalidade 1:1 e localizar a menor unidade dona antes do resselo.

## 4. Contrato vanilla a congelar

### 4.1 `len()`

- zero argumentos;
- devolve inteiro não-negativo igual ao número de bytes;
- vazio devolve `0`;
- comprimento conta bytes UTF-8, não caracteres Unicode;
- qualquer posicional ou named restante produz o diagnóstico vanilla medido.

### 4.2 `at(index, default:)`

- `index` é inteiro obrigatório;
- índice não-negativo conta do início;
- índice negativo conta do fim (`-1` é o último byte);
- retorno válido é inteiro `0..255`;
- fora de limites sem `default:` produz a mensagem vanilla;
- fora de limites com `default:` devolve o valor default sem coerção;
- em limites, `default:` não substitui o byte encontrado;
- `index == len` é fora de limites para acesso, embora seja posição válida de
  fronteira para `slice`;
- tipo errado, argumento ausente, posicional extra e named desconhecido devem
  preservar classe, ordem e mensagem observáveis.

### 4.3 `slice(start, end?, count:)`

- `start` inteiro obrigatório e inclusivo;
- `end` inteiro posicional opcional e exclusivo;
- omitido significa fim dos bytes;
- índices negativos contam a partir do fim;
- `start == len` e `end == len` são fronteiras válidas;
- `end < start` produz slice vazio, conforme `max(start)` vanilla;
- `count:` calcula `end = start_resolvido + count`;
- quando `end` e `count:` aparecem juntos, `end` prevalece e `count:` é
  consumido sem efeito (medido no vanilla; a hipótese inicial de exclusividade
  foi refutada durante a execução);
- resultado é novo valor `bytes`, com `repr` `bytes(N)`;
- início/fim fora de limites, overflow de `start + count`, tipos errados,
  argumentos ausentes/extras e named desconhecido usam diagnósticos medidos.

Não inferir a política de overflow ou de `count:` negativo apenas da leitura
Rust. Executá-la no binário ratificado antes de congelar o oráculo.

## 5. Matriz de sondas obrigatória

Criar `00_nucleo/diagnosticos/p1214-bytes-oraculos.tsv` antes de alterar L0 ou
código, contendo pelo menos:

### Valores positivos

1. `bytes(()).len()`;
2. `bytes((0, 127, 255)).len()`;
3. `bytes("é").len()` — confirma bytes UTF-8;
4. `.at(0)`, `.at(2)`, `.at(-1)`, `.at(-3)`;
5. `.at(3, default: 99)` e `.at(-4, default: none)`;
6. `.at(1, default: 99)` — default não mascara sucesso;
7. `.slice(0)`, `.slice(1)`, `.slice(1, 3)`;
8. `.slice(-2)`, `.slice(0, -1)`;
9. `.slice(3)`, `.slice(3, 3)`, `.slice(2, 1)`;
10. `.slice(1, count: 0)`, `.slice(1, count: 2)`;
11. slices cujo conteúdo é comparado com outro valor `bytes`, para não validar
    somente o comprimento do `repr`. O primeiro oráculo com `array(...)` foi
    invalidado e recongelado porque o cristalino ainda não expõe esse
    construtor; a igualdade de `bytes` isola o método sob teste.

### Diagnósticos e bordas

12. `at` sem índice;
13. `at` com string/float;
14. `at` em `index == len`, acima do fim e abaixo do início;
15. `at` com posicional extra e named desconhecido;
16. `len` com posicional e named;
17. `slice` sem start e com start de tipo errado;
18. `slice` com end de tipo errado;
19. `slice(end + count:)` simultâneos — congela precedência de `end`;
20. `slice` com início/fim fora dos limites;
21. `slice(count:)` negativo, muito grande e com overflow aritmético;
22. named desconhecido e posicional extra.

Para cada caso registrar expressão, stdout, stderr, exit status e classe. Usar
`repr(...)`, `array(...)`, JSON ou raw conforme o valor; não confundir erro de
serialização do CLI com erro do método.

## 6. Artefatos causais

Produzir em ordem:

1. `00_nucleo/diagnosticos/p1214-bytes-oraculos.tsv` — expectativas vanilla
   congeladas e hashadas;
2. alterações L0 e hashes ressellados;
3. testes RED co-localizados no owner correto;
4. recibo RED com comandos e falhas esperadas;
5. implementação mínima;
6. `00_nucleo/diagnosticos/p1214-bytes-resultados.tsv` — A/B final;
7. `00_nucleo/diagnosticos/typst-p1214-bytes-paridade.md` — laudo;
8. atualização da fila P1213 e do mapa somente após veredito.

Os oráculos são entradas protegidas. Se uma expectativa mudar após ler o
resultado cristalino, invalidar o selo e reiniciar desde a medição vanilla.

## 7. Testes RED

Os testes devem demonstrar primeiro que o dispatch público falha. Cobrir:

- descoberta dos três métodos no valor `Value::Bytes`;
- valores e índices negativos;
- `default:` de `at`;
- slicing por end e por count;
- conteúdo real do slice;
- vazio e fronteira `len`;
- diagnósticos nominais congelados;
- inexistência de método público inventado.

Confirmar RED antes de implementar. Um teste direto apenas sobre
`entities::Bytes::len()` não serve: esse helper Rust já existe e não reproduz
a lacuna de linguagem.

## 8. Implementação permitida

Implementar somente o necessário para o contrato congelado:

- descoberta/despacho estático dos métodos para `Value::Bytes`;
- avaliação de argumentos pelo caminho arquitetural proprietário;
- resolução de índices e wrapping `i64` de `start + count`, conforme o
  observável vanilla congelado;
- retorno de `Value::Int`, valor default ou `Value::Bytes`;
- diagnósticos vanilla nos mesmos casos observáveis.

Restrições:

- não reutilizar `world_types::Bytes`;
- não converter bytes em string para indexar;
- não expor o `Vec<u8>` como array interno;
- não adicionar reflexão, vtable, `dyn` ou tabela global mutável;
- não implementar concatenação, iteração, casts adicionais, `first`, `last`
  ou outros métodos neste passo sem nova medição da superfície vanilla;
- não alterar a representação para `Arc`/`EcoVec` em nome desta correção.

Se helpers puros `locate_opt`/`locate` forem colocados em
`entities/bytes.rs`, eles devem representar apenas invariantes do valor. Cast,
args, defaults e diagnósticos permanecem no owner da linguagem, não na
entidade.

## 9. Ataques obrigatórios

O adversário deve introduzir ou simular mutações que as sondas rejeitem:

1. contar caracteres em vez de bytes para `"é"`;
2. rejeitar todo índice negativo;
3. aceitar `index == len` em `at`;
4. devolver `default:` mesmo quando o índice é válido;
5. aplicar índice negativo duas vezes em `slice`;
6. tratar `end` como inclusivo;
7. rejeitar `end` e `count:` juntos ou deixar `count:` sobrepor `end`;
8. fazer `end < start` inverter ou falhar em vez de produzir vazio;
9. substituir o wrapping observado em `start + count` por soma verificada,
   saturada ou panic;
10. devolver array de inteiros em vez de `bytes`;
11. aceitar `first`/`last` por reutilização acidental do dispatch genérico;
12. produzir mensagem genérica diferente nos erros auditados.

O contrato só pode ser selado se todas as mutações válidas forem rejeitadas.
Construção deliberadamente não medida permanece `Unknown`; não vira GREEN.

## 10. Separação de autoridades

- **A — autor do contrato:** lê fonte/binário vanilla e publica os oráculos;
  não lê patch candidato;
- **B — adversário:** cria mutações contra a matriz congelada; não corrige
  implementação;
- **C — implementador:** recebe L0 e oráculos selados; não os edita;
- **D — testador A/B:** executa os dois binários e publica recibos;
- **E — verificador:** decide promoção no mapa e fechamento da fila; não
  escreve a implementação.

Registrar entradas, capacidades, hashes e predecessor causal de cada papel.
Se uma única sessão tiver acesso a tudo, classificar como
`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO` e não alegar independência.

## 11. Promoção no mapa e fila

A relação existente:

```toml
id = "language-bytes"
alegacao = "parcial"
```

só pode virar `declarada-fechada` se:

- `len`, `at` e `slice` passarem integralmente nos valores e diagnósticos
  congelados;
- nenhuma responsabilidade vanilla de `Bytes` incluída no fragmento tiver
  sido omitida;
- os ataques forem rejeitados;
- a lente aceitar o mapa sem diagnóstico;
- o laudo limitar expressamente o fechamento à superfície medida.

Se qualquer caso continuar diferente, manter `parcial` e listar a lacuna
nominal. A linha `bytes-methods` da fila P1213 só recebe `RESOLVED` depois da
mesma decisão; não apagar o registro histórico.

## 12. Proveniência e gates finais

Registrar HEAD, working tree, horário, hashes do vanilla/cristalino, L0s,
oráculos, mapa e binário da lente.

Executar no mínimo:

```text
cargo test -p typst-core <testes focais bytes>
cargo test --workspace
cargo build --workspace --quiet
crystalline-lint .
git diff --check
lente --comparar ... --mapa-correspondencia ...
```

Repetir a matriz A/B e a lente final para verificar determinismo. Nenhum RED
não relacionado pode ser ocultado; falha temporal deve ser registrada e
repetida isoladamente, sem apagar o primeiro resultado.

## 13. Estado terminal esperado

Sucesso integral:

```text
BYTES METHODS GREEN — MAP RELATION DECLARED-CLOSED
```

Sucesso parcial:

```text
BYTES METHODS PARTIAL — MAP RELATION REMAINS PARTIAL
```

O passo seguinte retorna à fila P1213 e abre `operator-diagnostics`.
