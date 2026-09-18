# Prompt L0 — `compiler/eval/tests`
Hash do Código: bb43110c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Camada:** L1 test-only
**Ficheiro alvo exclusivo:** `01_core/src/compiler/eval/tests.rs`

## P1447 — oráculo bare de `math.abs`

O teste P1447 avalia `$ abs(x) $` pelo pipeline real e exige
`Content::MathLr` com body de três nós `|`, `x`, `|` e `size_explicit=false`.
Ele rejeita a aproximação histórica `MathDelimited` e não concede crédito por
texto visual semelhante. A assinatura, os casts e a presença causal de
`size` são cobertos pela suíte estrutural independente P1447.

## Propriedade

Este prompt possui a suíte unitária do eval. O consumer fornece Worlds puros,
helpers test-only e regressões de linguagem; não possui implementação
produtiva. Fixtures não fazem I/O real de L1.

Asserções observam semântica, sintaxe, morfologia, diagnósticos e spans. Ordem
de mapas Rust, endereços, formato Debug e estrutura privada não são paridade.

## Avaliação básica

A suíte cobre literais, expressões, arrays, dictionaries, blocks, escopos,
bindings, closures, operadores, controle de fluxo, imports, módulos, field
access, chamadas estáticas/ligadas e construção de Content. Casos positivos
preservam valores e efeitos; negativos preservam mensagem, severidade, hints,
trace e range quando publicamente observáveis.

## Perfis

Contratos afetados por feature são repetidos nos perfis `default`, `html`,
`a11y` e `html+a11y`. Uma feature não habilita outra e uma correção não pode
existir somente num perfil quando a linguagem a define globalmente.

## Imports e módulos

- bare imports sem efeito emitem uma vez `this import has no effect` no
  identificador fonte, inclusive antes de erro posterior;
- alias, field, items, wildcard, literal, dinâmico e resolução de fonte mantêm
  seus comportamentos próprios e não herdam o warning por generalização;
- módulo nomeado sem field usa o nome guardado do módulo, zero hints e span
  apenas sobre o field;
- aliases de `std` conservam identidade do módulo, sem hard-code pelo spelling
  local;
- lookups existentes preservam valor, kind e ausência de diagnóstico lateral;
- fontes importadas ancoram o diagnóstico na Source real.

## Superfície global e namespaces

`hsl`, `hsv` e `linear_rgb` bare e sob `std` permanecem ausentes, com os
diagnósticos do vanilla. `color.hsl`, `color.hsv` e `color.linear-rgb`
permanecem chamáveis. `rgb`, `luma`, `cmyk`, `oklab` e `oklch` continuam
funções bare e sob `std` nos quatro perfis.

Os membros `pdf.*` respeitam `a11y-extras`: membros ungated permanecem
acessíveis; membros gated falham sem a feature e funcionam com ela. Dict e
float conservam seus contratos distintos de field access e span.

## Arrays, Content e representação

Arrays preservam todos os elementos. A forma curta/multiline respeita os
limites contratados, inclusive 0, 1, 39, 40, 41, 42, 81 e 256 itens, strings
escapadas, nesting e Unicode. Elision não pode apagar payload ou vazar para
`Content::Sequence` e `Args`.

Módulos nomeados, aliases, colisões, imports e reexports preservam seus nomes e
payloads. `repr` de arrays, módulos, funções `with` e conteúdo não pode ser
calculado por Debug ou pelo expected produzido pelo próprio candidato.

## Encoders e `Args`

JSON, TOML e YAML preservam forma default/pretty/compact, escaping, ordem
linguística, classes públicas, tratamento de `none`, não finitos e `-0.0`.
CBOR e decoders/read mantêm seus diagnósticos próprios.

Chamadas diretas, aliases, `with`, sinks e métodos ligados preservam:

- agregado da chamada e spans individuais;
- origem distinta de factories iguais;
- sequência conjunta de posicionais e named, inclusive repetições;
- ordem de callbacks em `filter`, `map`, `join` e sink;
- arg-span original e value-span destacado quando o resultado é novo;
- trace causal sem apagar mensagem, hints ou span primário.

`arguments.filter` exige bool ancorado na origem da função; erros internos do
callback mantêm sua própria origem. Args sintético conserva ausência legítima
de Source. Nenhum teste normaliza traces ou warnings genericamente.

## Snapshots de conteúdo

Query preserva campos completos e ordenados do elemento realizado. Igualdade
de linguagem considera função e campos, ignora label e location quando o
contrato assim define, e atravessa arrays/closures sem perder o snapshot.
Mudança de numbering ou campo linguístico relevante altera igualdade; mudança
apenas de label não.

## Constructors e domínios complementares

A suíte mantém regressões de bytes, math attach, gradients, datetime, int,
counter e content scope. Cada família preserva defaults, named, casts,
mensagens e pureza dos Worlds. Esses testes não transferem ownership dos
respectivos módulos para este prompt test-only.

## Controles

Cada correção focal conserva controles positivos e negativos adjacentes:

- sucesso não pode surgir de erro de parser, fixture, import ou build;
- erro posterior não substitui o diagnóstico que o caso pretendia observar;
- `Unknown` ou timeout não contam como sucesso;
- ordem direta, repetida e inversa devem produzir o mesmo mapa observável;
- comparadores não removem warning, hint ou trace para obter igualdade;
- expectativas vêm do contrato e do vanilla ratificado, nunca da saída do
  candidato em teste.

## Aceitação

Todos os testes passam nos perfis aplicáveis e continuam a distinguir as
mudanças negativas descritas por suas testemunhas.

## P1401 — provas de construção contextual

### Medição anterior à decisão

Antes de P1401 não há teste que construa `selector.before/after` pela linguagem.

### Aceitação

Testes focais avaliam as duas formas vinculadas, o default inclusive e
`inclusive: false`, distinguindo método ausente, named ignorado e árvore
incorreta. O teste contextual com introspector é propriedade de
`compiler/introspect.md`; este owner cobre somente a construção por eval.

## P1438 — regressão bare de `norm` acompanha o carrier público

### Medição anterior à decisão

O teste histórico P899 agrupava `norm` com wrappers cristalinos ainda
materializados como `MathDelimited`. P1438 mediu no vanilla ratificado que
`norm` retorna `LrElem` e o gate aprovado substitui essa aproximação. Manter a
expectativa antiga transformaria a divergência conhecida em falso contrato.

### Aceitação

Separar `norm` do loop legado sem alterar as expectativas de
`abs`/`floor`/`ceil`/`round`/`bar`. A regressão P1438 exige `Content::MathLr`
com body de três nós `‖`, conteúdo, `‖`, tamanho default não explícito e
nenhum `MathDelimited` exterior. O teste não duplica layout nem concede
paridade às demais rotas residuais.

## P1439 — regressão bare de `overbrace` preserva identidade

### Medição anterior à decisão

O teste histórico não detecta que o braço agregado baixa `overbrace` para
`MathUnderover`, ignora argumentos e perde presença. O vanilla conserva o
elemento público na sintaxe matemática.

### Aceitação

Avaliar `$ overbrace(x, n) $` e exigir `Content::MathOverbrace` com body `x`,
annotation `n` e presença verdadeira. Nenhum `MathUnderover` exterior pode
substituir a identidade; validação detalhada da assinatura pertence ao owner
test-only P1439.

## P1440 — regressão bare de `overbracket` preserva identidade

### Medição anterior à decisão

O teste histórico D2 exige diretamente o carrier mecânico `MathUnderover` e o
braço agregado baixa `overbracket` para esse carrier, ignora argumentos e
perde presença. O vanilla conserva o elemento público na sintaxe matemática.

### Aceitação

Avaliar `$ overbracket(x, n) $` e exigir `Content::MathOverbracket` com body
`x`, annotation `n` e presença verdadeira. Sincronizar D2 e o controle dos
quatro codepoints para inspecionarem U+23B4 através da identidade pública; não
exigir que a redução mecânica exista antes do layout. Validação detalhada da
assinatura pertence ao owner test-only P1440.

## P1441 — regressão bare de `overline` preserva identidade matemática

### Medição anterior à decisão

A sintaxe `$ overline(x) $` cai hoje no binding global de decoração textual e
produz `Content::Overline`; no vanilla ratificado ela produz o mesmo elemento
matemático body-only exposto por `math.overline`.

### Aceitação

Avaliar `$ overline(x) $` e exigir `Content::MathOverline` com body `x`, sem
aceitar `Content::Overline`, `MathUnderline` ou normalização para conteúdo
textual. Validação detalhada de assinatura, `repr` e geometria pertence ao
owner test-only P1441.

## P1442 — regressão bare de `overparen` preserva identidade e annotation

### Medição anterior à decisão

A sintaxe `$ overparen(x, n) $` projeta hoje uma forma opaca e concatena a
annotation sem carrier público; o vanilla conserva `OverparenElem`.

### Aceitação

Avaliar `$ overparen(x, n) $` e exigir `Content::MathOverparen` com body `x`,
annotation `n` e presença verdadeira. A forma opaca histórica e qualquer
redução permanente para `MathUnderover` falham o teste.

## P1443 — regressão bare de `overshell` preserva identidade e annotation

### Medição anterior à decisão

A sintaxe `$ overshell(x, n) $` projeta hoje uma forma opaca e concatena a
annotation sem carrier público; o vanilla conserva `OvershellElem`.

### Aceitação proposta

Avaliar `$ overshell(x, n) $` e exigir `Content::MathOvershell` com body `x`,
annotation `n` e presença verdadeira. A forma opaca histórica e qualquer
redução permanente para `MathUnderover` falham o teste.
## P1444 — oráculos sintáticos de primes (APROVADO; GATE ADR-0127 CONFIRMADO)

### Medição anterior à decisão

O cristalino funde primes em texto com o sobrescrito e usa `tr`/`br` para
hat/underscore. O vanilla conserva `MathPrimes(count)` separado em `tr`, usa
`t`/`b` para hat/underscore e deixa a precedência da AST determinar nesting.

### Decisão

Adicionar oráculos que extraem a árvore avaliada e distinguem `t` de `tr`:
`x'''^2` deve preservar `t: [2]` e `tr: primes(count: 3)`; `x^'''` deve
preservar o carrier em `t`; `x^2'''` deve preservá-lo aninhado no conteúdo de
`t`. Não aceitar equivalência por `plain_text`, glyph ou concatenação.

## P1445 — regressão bare de `round` acompanha o carrier público

### Medição anterior à decisão

O teste histórico P899 ainda agrupa `round` com wrappers cristalinos
materializados como `MathDelimited`. P1445 mediu no vanilla ratificado que
`round` retorna `LrElem` e o gate aprovado substitui essa aproximação. Manter
a expectativa antiga transformaria a divergência conhecida em falso contrato.

### Aceitação

Separar `round` do loop legado sem alterar as expectativas de
`abs`/`floor`/`ceil`/`bar`. A regressão P1445 exige `Content::MathLr` com body
de três nós `⌊`, conteúdo, `⌉`, tamanho default não explícito e nenhum
`MathDelimited` exterior. O teste não duplica layout nem concede paridade às
demais rotas residuais.

## P1448 — regressão bare de `underbrace` preserva identidade pública

### Medição anterior à decisão

O braço P906 atual produz `MathUnderover` diretamente e aceita silenciosamente
named/excedentes. O vanilla bare produz o mesmo `UnderbraceElem` da função
qualificada, com body, annotation e presença causal observáveis.

### Aceitação

Avaliar `$ underbrace(x, n) $` deve produzir `Content::MathUnderbrace` com body
`x`, annotation `n` e `annotation_explicit = true`. O teste procura a variante
na árvore avaliada; não aceita equivalência por `plain_text`, glifo U+23DF ou
estrutura `MathUnderover` semelhante.

Os oráculos legados P906 deixam de exigir `MathUnderover` diretamente para
`underbrace`: passam a exigir o carrier público e sua projeção U+23DF. As
expectativas de `underbracket` e dos spreaders superiores permanecem próprias;
nenhum teste recebe crédito por igualdade apenas visual.

## P1449 — regressão bare e reflexão de `underbracket`

### Medição anterior à decisão

O braço P906 produz `MathUnderover` diretamente e aceita silenciosamente
named/excedentes. O vanilla bare produz o mesmo `UnderbracketElem` da função
qualificada; fields distinguem annotation omitida de `none` explícito.

### Aceitação

Avaliar `$ underbracket(x, n) $` deve produzir `Content::MathUnderbracket` com
body `x`, annotation `n` e presença verdadeira. `fields`, `has`, `at` e
`func()` preservam identidade e presença causal. Os testes procuram a variante
na árvore; não aceitam equivalência por `plain_text`, U+23B5 ou estrutura
`MathUnderover` semelhante.

## P1450 — oráculos `math.underparen`

Após aprovação ADR-0127, testar que a forma bare preserva
`Content::MathUnderparen`, body, annotation e presença causal; testar também
`fields`, `has`, `at` e `func` nas formas qualificada e com `none` explícito.
Esses oráculos são congelados antes do candidato produtivo.

## P1451 — oráculos `math.undershell`

Após aprovação ADR-0127, testar que a forma bare preserva
`Content::MathUndershell`, body, annotation e presença causal; testar também
`fields`, `has`, `at` e `func` nas formas qualificada e com `none` explícito.
Esses oráculos são congelados antes do candidato produtivo.
