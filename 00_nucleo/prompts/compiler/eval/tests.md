# Prompt L0 — `compiler/eval/tests`
Hash do Código: cdbdb24f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Camada:** L1 test-only
**Ficheiro alvo exclusivo:** `01_core/src/compiler/eval/tests.rs`

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
