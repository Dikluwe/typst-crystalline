# Prompt L0 — `compiler/stdlib/text/lorem` — `lorem`
Hash do Código: 5e93e04d

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/text/lorem.rs`
**Prompt pai (hub)**: `00_nucleo/prompts/compiler/stdlib/text.md` — dono de `text/mod.rs`
e da história por marco (`6e29fbaba`, `c98ffc8ac`). Este L0 especifica **a superfície do
nó**.
**Convenções partilhadas**: `00_nucleo/prompts/compiler/stdlib/_comum.md`
**Vanilla**: `typst-library/src/text/lorem.rs` — ficheiro próprio; o corpo cristalino é
port directo do `lorem_impl` de lá (MIT, baseado na crate `lipsum`, © 2017 Martin
Geisler).

**Fronteira medida**: par `native_lorem` + `lorem_impl`, confirmado pelo critério 3 —
`c98ffc8ac` (2026-07-21) muda o corpo dos dois. Os restantes commits que tocam `lorem`
são lotes transversais (`87bc1c64d`, `04eda8179`, `0661aef91` `cargo fmt`, renames) ou o
commit de nascimento `6e29fbaba` (2026-06-22), que materializou `lorem` e `regex` no mesmo
lote sem os ligar mecanicamente. A fronteira vanilla e a co-mudança concordam.

---

## Contexto

`lorem(n)` devolve `n` palavras de texto dummy como `Value::Str`. É a única nativa deste
módulo que **não produz conteúdo** — a saída é uma string, e o chamador decide o que lhe
faz.

O ponto sensível deste nó é a paridade: a decisão original ("o texto exacto não precisa de
ser byte-identical", `6e29fbaba`, 2026-06-22) foi **revogada** em `c98ffc8ac`
(2026-07-21), depois de uma sonda medir divergência total a partir da palavra ~19 —
vírgulas e corpus diferentes, e ponto final em falta. O critério passou a ser
**byte-parity com o vanilla**, e a única forma de o garantir é usar o mesmo gerador: a
crate `lipsum` (whitelist `[l1_allowed_external.lipsum]`), com a mesma cadeia e a mesma
semente.

## Instrução

`lorem(n)` — 1 posicional `Int`; **zero** nomeados.

- `n < 0` → `lorem() não aceita números negativos`. `n == 0` → `Str("")`.
- Tipo errado → `lorem() espera um inteiro, recebeu {tipo}`; contagem errada de
  posicionais → `lorem() requer 1 argumento inteiro`; qualquer nomeado → erro de
  `expect_no_named`.
- Saída: `Value::Str(lorem_impl(n))`.

**`lorem_impl(n) -> String`** — port do vanilla, privado ao nó:

1. Markov chain de ordem 2 da crate `lipsum`, treinada com `LOREM_IPSUM` seguido de
   `LIBER_PRIMUS`, iterada a partir do par `("Lorem", "ipsum")` com o RNG determinístico
   interno da crate (ChaCha20Rng, semente 97). A ordem do treino faz parte do contrato:
   trocá-la muda o corpus e quebra a byte-parity.
2. Palavras separadas por um espaço, inserido **antes** de cada palavra a partir da
   segunda.
3. O token `--` não conta como palavra: acrescenta en-dash `U+2013` ao output e continua
   sem incrementar o contador.
4. Capitalização: a palavra seguinte a uma que termine em `.`, `!` ou `?` entra com a
   primeira letra em maiúscula (`char::to_uppercase`, resto do byte-offset intacto).
5. Fecho: se a frase não terminar em `.`/`!`/`?`, a pontuação ASCII final pendente é
   truncada (para não colar `.` depois de `,`) e acrescenta-se `.`.

## Restrições Estruturais

- L1 puro. `_ctx`/`_world`/`_current_file` são pass-through — nunca usados.
- A cadeia é construída **por chamada**: L1 proíbe estado global mutável (V13), enquanto o
  vanilla usa `LazyLock`. Divergência de mecânica deliberada (ADR-0107/ADR-0029), com
  custo aceite e registado como candidato a optimização se aparecer em perfil — não como
  débito de paridade, porque a saída é idêntica.
- `lipsum` só pode ser importada aqui dentro de `text/`; a whitelist
  `[l1_allowed_external.lipsum]` existe para este nó (V14).
- `lorem_impl` é privado: a superfície da linguagem é `lorem(n)`. Quem precisar do gerador
  para outro fim escreve L0 novo antes de o expor.

## Critérios de Verificação

Byte-parity medida contra o binário vanilla — a aceitação é a igualdade da string, não a
"semelhança" do texto:

```
#lorem(0)   → Str("")
#lorem(1)   → Str("Lorem.")
#lorem(10)  → Str("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do.")
#lorem(-1)  → Err "lorem() não aceita números negativos"
#lorem("x") → Err "lorem() espera um inteiro, recebeu string"
#lorem()    → Err "lorem() requer 1 argumento inteiro"
#lorem(5, foo: 1) → Err (nomeado inesperado)
```
