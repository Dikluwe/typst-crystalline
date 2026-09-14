# Passo 1358 — Paridade ratificada do protocolo de argumentos de `luma`

## Estado medido

Este passo deriva de `diagnosticos/p1358-luma-preflight.md`. A medição foi
feita antes da decisão contra o único vanilla ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d`.

O L0 e o consumer cristalinos ainda materializam o comportamento do snapshot
histórico: `luma()` devolve branco, entradas inválidas caem em branco e alpha é
nomeado. O vanilla ratificado exige lightness, propaga erros de tipo/range e
aceita alpha como segundo positional `Component`.

## Decisão e escopo

Atualizar o protocolo público de `luma` para:

```text
luma(lightness)
luma(lightness, alpha)
luma(color)
```

- `lightness` é o primeiro positional obrigatório na forma numérica;
- `alpha` é o segundo positional opcional, default 100%;
- ambos usam `Component`: `Int` inclusivo 0–255 ou `Ratio` inclusivo
  0%–100%;
- Float, tipo errado e componentes fora do range são diagnósticos, nunca
  fallback branco;
- `luma()` é diagnóstico com hint para `white` ou `luma(100%)`;
- argumentos nomeados, inclusive `alpha:`, são inesperados;
- um terceiro positional é inesperado;
- `luma(color)` continua aceito e não pode ser combinado com componente.

### Scope-out obrigatório

Este passo não altera a transformação de `luma(color)`: o cristalino continua
preservando alpha conforme o L0 vigente, embora o vanilla medido produza alpha
100% para a cor não-Luma usada no preflight. Essa divergência terá passo e
decisão próprios. O fecho de P1358 significa paridade do protocolo de
argumentos numéricos e diagnósticos, não paridade total da função.

Não alterar `Color`, `ColorSpace`, representação, conversões, render PDF/HTML,
gradientes ou outros construtores.

## Classificação ADR-0127 e fluxo

A mudança rejeita chamadas que o cristalino hoje aceita (`luma()` e
`luma(..., alpha: ...)`) e converte fallbacks brancos em erros. A intenção é
convergência mensurada com o vanilla, não nova política do produto nem
alteração de API Rust. Classificação: correção de paridade geral,
ADR-0127 §2(3), em fluxo contínuo.

Fluxo obrigatório:

1. atualizar primeiro `00_nucleo/prompts/compiler/stdlib/foundations/color.md`;
2. ressellar o hash do L0 e registrar o novo hash;
3. escrever testes RED independentes;
4. implementar e revalidar sem nova paragem.

## Regime segregado

Usar protocolo completo porque diagnósticos são observáveis mecânicos e o
risco principal é adaptar testes ao parser candidato.

- autor do contrato recebe L0 atualizado, este passo e a matriz do preflight;
- autor dos oráculos recebe somente contrato, vanilla e observáveis públicos;
- implementador recebe L0/contrato/oráculos selados e não altera expected;
- adversário injeta os fallbacks e a forma nomeada antigos;
- verificador não corrige implementação nem testes.

Se não houver isolamento material, declarar “executado sem atestação de
isolamento”. Budget: duas revisões focais, uma execução final. `Unknown` não
conta como sucesso.

## Contrato RED

### Valores

```text
repr(luma(0))                    == "luma(0%)"
repr(luma(128))                  == "luma(50.2%)"
repr(luma(255))                  == "luma(100%)"
repr(luma(40, 50%))              == "luma(15.69%, 50%)"
repr(luma(40, 127))              == "luma(15.69%, 49.8%)"
repr(luma(40, 0))                == "luma(15.69%, 0%)"
repr(luma(40, 255))              == "luma(15.69%)"
luma(128).components()           == (50.2%, 100%)
luma(128, 0).components()        == (50.2%, 0%)
```

### Diagnósticos públicos

Cobrir isoladamente:

- `luma()` → `missing argument: lightness component` e hint
  “try writing `white` or `luma(100%)`”;
- `luma("invalid")` e `luma(10%, "invalid")` →
  `expected integer or ratio, found string`;
- `luma(-1)`, `luma(256)`, `luma(10%, -1)`, `luma(10%, 256)` →
  `number must be between 0 and 255`;
- `luma(-1%)`, `luma(101%)`, `luma(10%, -1%)`, `luma(10%, 101%)` →
  `ratio must be between 0% and 100%`;
- `luma(10%, 0.5)` → `expected integer or ratio, found float`;
- `luma(128, alpha: 40%)` e `luma(255, key: "val")` → argumento nomeado
  inesperado;
- `luma(25%, 50%, "val")` → terceiro positional inesperado;
- `luma(red, 50%)` → segundo positional inesperado.

Testar mensagem sem exigir igualdade de estrutura interna Rust. Onde o span é
observável na CLI, congelar o span do argumento causador, não `Span::detached`.

### Preservação

- `luma(color)` continua aceito;
- o alpha cristalino atual de `luma(rgb(..., 40%))` permanece 40% neste passo;
- `luma(128)` permanece idêntico nos quatro perfis;
- nenhum outro construtor de cor muda;
- aliases/scope e kind público de `luma` permanecem function.

## Mutantes mínimos

O contrato deve rejeitar:

1. restaurar `luma()` → branco;
2. restaurar fallback branco para tipo inválido;
3. restaurar fallback branco para range inválido;
4. manter alpha exclusivamente nomeado;
5. aceitar `alpha:` além do segundo positional;
6. aceitar alpha ratio mas rejeitar alpha inteiro;
7. aceitar alpha inteiro fora de 0–255;
8. omitir o hint do caso vazio;
9. alterar `luma(color)` ou seu alpha neste lote;
10. usar igualdade Rust/bytes de render como oráculo.

Exigir `mutation_score=1.0`, positivos `Preserved`, negativos `Violated` e
opacos somente `Unknown` explícito.

## Implementação prevista

1. escrever testes RED independentes no módulo dono;
2. fazer `native_luma` rejeitar named args;
3. separar a alternativa `[Value::Color(_)]` da forma numérica;
4. exigir primeiro `Component` e aceitar segundo `Component` opcional;
5. reutilizar/adequar cast de componente com diagnósticos vanilla, sem
   duplicar regras de range;
6. produzir o hint somente para ausência de lightness;
7. manter a conversão de cor inalterada;
8. atualizar header `@prompt-hash` e resselo de linhagem.

## Gates

Executar RED documentado, GREEN, testes A/B, mutantes e:

```sh
cargo fmt --all -- --check
cargo build --workspace --release --locked
cargo test --workspace --release --locked --no-fail-fast
crystalline-lint .
git diff --check
```

Exigir zero V5/V15/V26, hashes protegidos sem drift, quatro perfis bilaterais
determinísticos e regressão explícita de `luma(color)` preservada.

## Critério de fecho

P1358 fecha quando o protocolo numérico e seus diagnósticos coincidem com o
vanilla ratificado no nível público, os mutantes morrem e o scope-out de
`luma(color)` permanece observavelmente inalterado. O relatório final não pode
generalizar o resultado para toda a família de cores nem para toda `luma`.
