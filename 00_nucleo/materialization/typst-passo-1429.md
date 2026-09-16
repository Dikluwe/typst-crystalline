# Passo 1429 — convergência vertical completa de `math.accent`

## Estado medido antes da decisão

P1428 certificou os observadores de `math.frac`; a fila residual P1422 mantém
`math.accent` como a próxima rota coerente. Ela já possui carrier e layout
dedicados, mas o estado cristalino ainda é o recorte P296: a nativa não está
registrada no módulo `math`, aceita conteúdo arbitrário como acento, rejeita
`size`/`dotless`, não conserva presença explícita, e o layout só realiza
acentos superiores no tamanho default.

Medição executada em `2026-09-16T20:25:19-03:00`, sobre HEAD limpo
`de2fa7be8e7ad06105e8cd4adb8297ad9947c46c`. Entradas principais:

| artefato | SHA-256 |
|---|---|
| vanilla `typst-library/src/math/accent.rs` | `52c898c9bfa5f80a9ea72a212f44955e7df5a1e858224f038e9a06d8bc55d0e8` |
| vanilla `typst-layout/src/math/accent.rs` | `3c8e7260ce50cae8d24d62acedec307bd5c87464e5031caa98bb96ffde15fc0b` |
| cristalino `compiler/stdlib/structural/math.rs` | `85aaed3f51fdac91562a1641a79a02118eb349ebfa0ddc416dccf7912f166e42` |
| cristalino `entities/elements/math_accent.rs` | `f1d7c7c1a35cea279ff6c24592ed9723f02ea7d96abb0bf0fc8eb6bc5c767015` |
| cristalino `compiler/math/layout/accent.rs` | `8c6311089ed117e972d886b0d945a648630ea638d3c75d356bd9ed90dc81d3b0` |

O alvo continua o vanilla ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d`. A fonte mede quatro campos:
`base`, `accent`, `size: Rel<Length> = 100% + 0pt` e `dotless: bool = true`.
O acento é um único codepoint normalizado pela tabela `ACCENTS`; por exemplo,
`"^"` vira U+0302 e `"→"` vira U+20D7. Mais de um carácter falha. A classe
Unicode `Below`, mais `⏟`, `⎵`, `⏝` e `⏡`, determina acento inferior.

Sondas públicas no binário ratificado mediram:

| expressão | resultado público |
|---|---|
| `repr(math.accent([x], "^"))` | `accent(base: [x], accent: "\\u{302}")` |
| `repr(math.accent("x", "→"))` | base convertida para `[x]`; accent U+20D7 |
| `math.accent()` | `missing argument: base` |
| `math.accent([x])` | `missing argument: accent` |
| terceiro posicional | `unexpected argument` |
| `base:`/`accent:` | o argumento correspondente é posicional |
| acento string com dois chars | `expected exactly one character` |
| acento content não-símbolo | `expected a single-codepoint symbol` |
| `size: 150%` | `size: 150% + 0pt` permanece em `repr` |
| `size: 2em` | `size: 0% + 2em` permanece em `repr` |
| `size: 1` | `expected relative length, found integer` |
| `dotless: false` | campo permanece em `repr` |
| `size: 100%`/`dotless: true` explícitos | ambos permanecem em `repr` |
| named desconhecido/duplicado | `unexpected argument: <nome>` / `duplicate argument: <nome>` |

A fonte de resolução e layout mede ainda: acento superior usa base cramped e,
quando `dotless` está ativo, remove o ponto de `i`/`j`; acento inferior não
usa cramped nem dotless; `size` resolve relativamente à largura da base e
altera o alvo de stretch; a posição superior usa `AccentBaseHeight`, enquanto
a inferior fica sob a base. `#set math.accent(size:, dotless:)` participa da
cascata já reconhecida pelo validador cristalino, mas ainda não chega ao
carrier.

## Decisão proposta

1. promover o carrier a `base: Content`, `accent: char`, `size: Rel<Length>`,
   `dotless: bool` e bits explícitos independentes para `size` e `dotless`;
2. substituir o wrapper Rust por uma forma estreita que recebe codepoint já
   normalizado, migrar os callsites internos default e acrescentar constructor
   completo; normalização/cast pertencem à fronteira de avaliação, não ao
   carrier;
3. registrar `math.accent` com dois posicionais obrigatórios e apenas
   `size:`/`dotless:` nomeados, preservando ordem, duplicados, tipos e
   diagnósticos medidos;
4. fazer a chamada bare `accent(...)`, a chamada qualificada e os wrappers
   simbólicos (`hat`, `tilde`, `dot`, `dot.double`) convergirem no mesmo
   carrier; wrappers simbólicos continuam fixando o codepoint mas aceitam os
   named públicos do wrapper vanilla;
5. ligar `math.accent.size` e `math.accent.dotless` à `StyleChain`, com
   precedência argumento explícito > cascata > defaults `100%`/`true`;
6. projetar `repr` como `accent(base: ..., accent: ...)`, acrescentando
   `size` e `dotless` somente quando explicitamente presentes no elemento;
   valores vindos apenas da cascata não fabricam presença morfológica;
7. realizar acentos superiores e inferiores; o alvo horizontal é
   `size.resolve(base.width)`, sujeito ao short-fall de `0.5em`; somente o
   ramo superior usa base cramped e remoção de ponto;
8. realizar dotless como transformação semântica dos glifos matemáticos
   `i`/`j` na base antes do layout. A implementação pode usar codepoint
   dotless ou substituição de fonte, desde que a morfologia/render observável
   coincida e o conteúdo público do carrier permaneça a base original;
9. preservar todos os campos e bits em walkers, hash, igualdade e
   `apply_math_default`; normalização Unicode e posição são derivadas do
   `accent`, nunca guardadas como estado redundante.

A aceitação é semântica, sintática e morfológica. Estrutura Rust, GSUB versus
codepoint dotless, sequência de operações e bytes de render são mecânica;
diagnósticos, `repr`, orientação, tamanho e ponto visível são observáveis.

## Limites explícitos

P1429 não fecha as outras 19 rotas residuais, não generaliza feature OpenType
para texto comum, não altera a tabela pública de símbolos e não reabre a
geometria já certificada de frac/root. Um fallback de fonte pode escolher
glifo diferente, mas não pode ignorar `dotless` silenciosamente quando a base
é `i`/`j` e o acento é superior.

Falha de build/execução, timeout, OOM, swap, stream truncado, observador opaco
ou identidade não verificável são `Unknown` e bloqueiam; nunca recebem crédito.

## Gate ADR-0127

O passo altera campos públicos, assinatura/construction path do carrier,
registra função pública, liga propriedades em cascata e acrescenta o ramo de
layout inferior. Incide nos itens 1, 2 e 3 do ADR-0127.

Fluxo obrigatório:

1. atualizar antes do código os L0 proprietários afetados;
2. commitar esta proposta e os L0s;
3. **PARAR sem alterar L1/L3 nem headers de linhagem**;
4. continuar somente após confirmação explícita do dono;
5. depois da confirmação, produzir manifesto, contrato, oráculos, ataques e
   testes segregados, selar, obter RED atribuível e só então implementar.

## Regime posterior ao gate

Aplicar o protocolo completo `tekt-materializacao-segregada`, executado sem
atestação de isolamento no workspace partilhado. O contrato deve cobrir
binding, aridade, casts, normalização, named/duplicados, presença explícita,
cascata, wrappers simbólicos, carrier, walkers, repr, orientação, size,
dotless e regressão da geometria superior. Mutation score exigido: `1.0`;
`Unknown` é fail-closed.

## Gates de fecho após confirmação

- RED atribuível do binding e do carrier completo;
- RED de normalização, aridade, casts, duplicados e presença explícita;
- RED de cascata e precedência explícito > cascata > default;
- RED geométrico discriminando superior/inferior, 100%/150% e dotless on/off;
- GREEN bilateral das três rotas de construção;
- três ordens determinísticas e hashes selados intactos;
- `cargo fmt --all -- --check`, testes focais, `cargo build`,
  `crystalline-lint .` e `git diff --check`.

## Confirmação do gate

**Confirmado explicitamente pelo dono em 2026-09-16.** A materialização fica
autorizada somente dentro do recorte completo de `math.accent` e dos oito L0
proprietários atualizados; as outras 19 rotas math permanecem fora de escopo.
