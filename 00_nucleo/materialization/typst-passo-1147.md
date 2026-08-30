# P1147 — auditoria integral da superfície pública de `int`

**Data:** 2026-08-24  
**Estado:** `CONCLUÍDO — scope int 11/11 + constructor`  
**Baseline de linguagem:** vanilla ratificado `a51e02804`  
**Sentinela de origem:** `int.bit-and`  
**Dependência:** P1145 e P1146 materializados na working tree, ainda não
commitados

## 1. Objetivo

Auditar e nuclear a superfície pública completa de `int`, tomando
`int.bit-and` apenas como sentinela. Materializar como família as operações
bitwise sem omitir os membros adjacentes do mesmo scope e sem confundir a
semântica pública de inteiros signed de 64 bits com o comportamento acidental
de operadores Rust em debug/release.

Antes de código, o passo deve decidir separadamente:

1. constantes `min`/`max`, já existentes no cristalino;
2. constructor e casts;
3. `signum`;
4. seis operações bitwise;
5. conversões `from-bytes`/`to-bytes`.

Se o lote for dividido, o novo L0 deve declarar explicitamente o subconjunto
incompleto e nomear o passo que fecha cada membro adiado.

## 2. Proveniência inicial e preservação da working tree

- Medição em `2026-08-24T20:31:42-03:00`.
- HEAD: `c65c839ff5544a66683b7e81edba923acb49760e`.
- Baseline vanilla: `a51e02804`.
- Working tree não commitada: 50 ficheiros rastreados alterados, 1.006
  inserções e 99 remoções, além dos novos L0/owner de datetime e dos passos
  P1145/P1146 não rastreados.
- Esse lote contém materializações aprovadas de gradient e datetime, mudança
  do contrato `World::today`, Cargo lock/features, snapshots e hashes L0.

P1147 deve preservar integralmente esse estado. Antes de executar, registrar
novamente:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
git diff HEAD --stat
```

Se o estado diferir, substituir estes números pela nova proveniência. Não
reformatar nem ressellar ficheiros dos lotes anteriores salvo quando uma
dependência P1147 comprovadamente os afetar.

## 3. Inventário estático já estabelecido

A fonte ratificada
`a51e02804:crates/typst-library/src/foundations/int.rs:68-428` define o tipo
signed de 64 bits e um scope com doze membros públicos, além da chamabilidade
do próprio tipo:

```text
min, max,
signum,
bit-not, bit-and, bit-or, bit-xor, bit-lshift, bit-rshift,
from-bytes, to-bytes
```

O constructor é marcado `#[func(constructor)]`: é a chamada `int(...)`, não um
field adicional. `min` e `max` são constantes; os nove restantes são funções.

No cristalino:

- `Type::Int` já expõe `min` e `max` em
  `compiler/eval/bindings/field_access.rs`;
- `native_int` vive em `compiler/stdlib/foundations/cast.rs`;
- nenhum dos nove fields funcionais foi encontrado;
- o L0 atual afirma que `native_int` rejeita float, enquanto a fonte
  ratificada aceita bool, int, float truncado, decimal truncado e string;
- `Value::Bytes` e `Value::Decimal` já existem, logo não presumir bloqueio de
  domínio para as conversões sem medir os casts reais.

Essas observações são inventário inicial, não autorização para copiar a
mecânica upstream.

## 4. Fase A — inventário literal e classificação

Reler o scope inteiro até o fecho literal e produzir tabela:

```text
membro | constante/função | assinatura Typst | static | instância |
posicional/nomeado | default | retorno | erro | since | owner
```

Extrair antes de decidir:

- representação two's complement signed de 64 bits como semântica observável;
- casts do constructor, base 2–36, Unicode minus, truncamento e overflow;
- aridade e receiver de cada operação;
- conversão de `shift` para `u32`, inclusive negativo e acima de `u32::MAX`;
- overflow de left shift versus saturação definida do right shift;
- right shift aritmético versus lógico;
- endianness, signed extension, tamanho e truncamento de bytes;
- mensagens e hints quando forem o observável.

Helpers, macros de cast e passos algorítmicos posteriores ao scope são
mecânica. Não contar impls genéricos de Rust como fields da linguagem.

## 5. Fase B — sondas nos dois binários vanilla

Executar todas as sondas em
`lab/typst-original/target/release/typst` e `/usr/local/bin/typst`. Só usar como
baseline resultados idênticos.

### 5.1 Descoberta e formas de chamada

Para cada função, medir `type(int.member)`, chamada estática e, quando houver
receiver `self`, chamada de instância. Exemplos:

```typst
int.bit-and(128, 192)
128.bit-and(192)
int.from-bytes(bytes((255,)), signed: false)
10000.to-bytes(endian: "big", size: 4)
```

Medir também field sem chamada, field desconhecido, `self` ausente/de tipo
errado, argumentos excedentes, named em positional e named desconhecido.

Não assumir que a exposição segue datetime: o source documenta sintaxe de
instância para os métodos com `self`; a disponibilidade real deve vir das
sondas.

### 5.2 Bitwise

Matriz mínima:

- `bit-not`: 0, 1, -1, `int.min`, `int.max`;
- AND/OR/XOR: zero, iguais, complementares, positivo/negativo, min/max;
- left shift: valores 0, 1, -1 e limites; shifts 0, 1, 62, 63, 64, negativo e
  acima de 64; casos cujo resultado cabe e cujo resultado é grande demais;
- right shift: os mesmos shifts, operandos positivos e negativos,
  `logical: false/true`, inclusive shifts 63, 64, 65 e muito grandes;
- tipos não inteiros para operandos/shift/logical;
- igualdade entre forma estática e de instância quando ambas existirem.

Aceitação deve afirmar valores signed de 64 bits e erros públicos, não o uso
de `checked_shl`, casts `as u64` ou número de passos do algoritmo.

### 5.3 Constructor

Medir bool, int, float/decimal positivos e negativos com fracção, strings
decimais e bases 2/8/16/36, letras maiúsculas/minúsculas, `+`, `-`, Unicode
minus, `int.min` e `int.max`.

Medir separadamente:

- base em valor não-string;
- base 1, 2, 36 e 37;
- string vazia/dígitos inválidos;
- overflow positivo e negativo;
- NaN/infinito e floats nos limites representáveis;
- tipo não aceite;
- mensagens e hints integrais.

A divergência float/decimal já encontrada não pode ser escondida como
scope-out da família bitwise sem um passo sucessor nomeado.

### 5.4 Bytes

Para `from-bytes`, medir comprimentos 0–9, little/big endian, signed
false/true, high bit desligado/ligado e roundtrip de positivos/negativos.

Para `to-bytes`, medir size 0, 1, 4, 8, 9 e tamanho grande limitado pelo risco
de memória; little/big endian; truncamento; zero; positivos; negativos; min e
max. Confirmar tipo retornado e conteúdo via `array(bytes)` ou observável
equivalente, não por representação interna do wrapper.

## 6. Fase C — auditoria cristalina e nucleação L0

Produzir matriz com `file:line`:

```text
caso | vanilla medido | cristalino atual | língua/mecânica |
L0 afetado | contrato público necessário? | decisão
```

Auditar antes de propor owner:

- `foundations/cast.md` e `cast.rs` para o constructor;
- `foundations.md` e `foundations/mod.rs` para atomização/reexport;
- `eval/bindings/field_access.md` e seu match de `Type::Int`;
- `eval/call_dispatch.md` somente se existir intercepção de instância real;
- `entities/value.md` para chamabilidade e fields do valor-tipo;
- entidade `Bytes` e seus métodos, sem alterar sua representação por
  conveniência;
- helpers de `Args`, casts e mensagens já utilizados por owners vizinhos.

Owner recomendado apenas como hipótese a confirmar: novo nó coeso
`compiler/stdlib/foundations/int.rs`, deixando o constructor no `cast.rs` ou
movendo-o somente se o L0 decidir a transferência completa e atualizar todos
os reexports/headers. O hub não recebe lógica.

Não criar entidade wrapper para `i64`, registry reflexivo, despacho dinâmico
ou trait bitwise próprio. A representação `Value::Int(i64)` já expressa o
domínio ratificado.

## 7. Decisão ADR-0107/0108 e gate ADR-0127

São língua: nomes, presença dos fields, formas de chamada, signed 64-bit,
resultados, defaults, endianness, signed extension, saturação/overflow,
truncamento, casts, mensagens e hints. São mecânica livre: operadores Rust,
buffers temporários, loops, macros, alocação e organização interna.

Atualizar primeiro os L0s comprovadamente afetados e ressellar hashes. Depois
classificar:

- funções privadas/pub(crate) de stdlib e braços de tabela sobre
  `Value::Int`, sem mudar entidade/assinatura/default/fase: fluxo contínuo,
  L0 primeiro + RED→GREEN;
- alteração de `Value`, `Type`, `Bytes`, assinatura pública Rust,
  comportamento por defeito, compatibilidade ou fase do pipeline:
  **parar no gate ADR-0127**;
- restringir permissividade cristalina existente ou alterar mensagens fora do
  scope medido exige decisão própria; em dúvida, parar.

A expectativa inicial é que bitwise e bytes possam ser glue privado sobre
tipos existentes. Isso é inferência, refutada por qualquer necessidade medida
de ampliar contrato público.

## 8. Plano RED→GREEN após L0 e eventual gate

1. RED da descoberta dos onze fields, preservando `min`/`max`;
2. RED das formas estática e de instância realmente medidas;
3. RED da matriz bitwise e limites de shift;
4. RED de constructor float/decimal/base/overflow;
5. RED de `from-bytes`/`to-bytes`, endian, signed e sizes;
6. RED de aridade, tipos, named args, mensagens e hints;
7. implementar no owner medido com match fechado e funções puras;
8. ressellar headers de linhagem afetados;
9. repetir sondas cristalinas equivalentes às do vanilla;
10. executar regressões de operadores, bytes, decimal, eval e serialização;
11. executar `cargo check --workspace`, `cargo build --workspace`,
    `cargo fmt --all -- --check`, `git diff --check` e
    `crystalline-lint .` com zero violations.

Confirmar cada RED antes da implementação. Teste que já passa documenta
regressão, mas não prova o delta.

## 9. Limites e encerramento

Ficam fora, salvo dependência inevitável medida:

- novos operadores sintácticos bitwise;
- floats/decimals além do cast necessário de `int`;
- aritmética geral `+ - * /` e parser de literais;
- refactor geral de `Args`, `Bytes` ou mensagens da stdlib;
- `counter.get`, `content.fields`, numbering, Symbol/emoji e HTML da fila
  pós-P1140;
- limpeza, commit ou alteração incidental dos lotes P1145/P1146.

P1147 encerra quando o scope tiver contagem reproduzível, os onze fields e o
constructor estiverem classificados, os L0s coincidirem com a implementação e
a validação final estiver verde. Qualquer membro adiado deve ter scope-out e
passo sucessor nomeados; não declarar “int completo” enquanto constructor,
bitwise ou bytes permanecerem sem decisão.

---

## 10. Execução e rebaseline

Ao iniciar a execução, o estado descrito na secção 2 já havia sido commitado.
Rebaseline efetivo em `2026-08-24T20:36:58-03:00`:

- HEAD `2df42a6f6dbdf15b5104d5e7a6df82755f07c8c4`;
- working tree limpa exceto por `typst-passo-1147.md` não rastreado;
- baseline vanilla `a51e02804`.

Portanto, os números P1145/P1146 da redação inicial não foram usados como
prova da execução P1147.

A varredura literal confirmou onze fields: duas constantes e nove funções,
além do constructor. As sondas positivas dos dois binários vanilla foram
idênticas e confirmaram formas estática/instância, signed 64-bit, shifts,
constructor float/decimal e bytes. Um primeiro erro do harness aplicava
`array` ao inteiro retornado por `from-bytes`; foi corrigido antes de qualquer
decisão e não foi contado como comportamento vanilla.

## 11. Decisão e materialização

A auditoria não encontrou necessidade de mudar `Value::Int(i64)`, `Bytes`,
assinatura pública Rust, default do produto ou fase. A implementação seguiu em
fluxo contínuo após atualizar e ressellar os L0s.

Materialização:

- novo owner `foundations/int.rs` com match fechado dos onze fields;
- `min`/`max` preservados;
- `signum`, seis operações bitwise e `from-bytes`/`to-bytes`;
- dispatch de instância somente para os oito métodos com receiver;
- left shift verificado por contagem e right shift saturante/logical;
- little/big endian, extensão signed e sizes de bytes;
- constructor ampliado para float e decimal truncados em direção a zero;
- base explícita rejeitada para não-string e Unicode minus preservado.

RED confirmado: o teste de superfície falhou inicialmente em `int.signum`
com `type int does not contain field`. GREEN: os dois testes P1147 passaram,
incluindo shifts 63/64/65, bytes vazios/9 bytes, extensão e cinco casos
negativos.

## 12. Validação final

Sonda cristalina após rebuild:

```text
signum(-5)                     -> -1
128.bit-and(192)               -> 128
(-8).bit-rshift(2, logical:true) -> 4611686018427387902
int(2.7)                       -> 2
int(decimal("3.8"))            -> 3
repr(10000.to-bytes(big, 4))   -> "bytes(4)"
```

Validação executada no HEAD acima, working tree P1147 não commitada, até
`2026-08-24T20:44:49-03:00`:

- testes P1147: 2/2;
- `cargo check --workspace`: passou;
- `cargo build --workspace`: passou;
- `cargo fmt --all -- --check`: passou;
- `git diff --check`: passou;
- `crystalline-lint .`: exit 0, sem violations.

Warnings históricos permanecem fora do scope. O diff P1147 final contém 12
ficheiros rastreados alterados (178 inserções, 28 remoções) e três ficheiros
novos: L0, owner e este passo.
