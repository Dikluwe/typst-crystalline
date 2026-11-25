# Proposta de pacote: resultados numéricos de `calc`

**24 de novembro de 2026 · aprovado pelo dono em 24/11/2026 (“Aprovo A–F”, no chat) sob a [ADR-0127](../adr/adr-0127-l0-gate-stop-vs-continuous-flow.md) (§2.1, §2.4 e pacotes de paridade)**

A aprovação cobre o escopo abaixo. Os L0s são a fonte do código.

## Medição

HEAD `f2c737125edd822ed6a8a1e4552607b2eee5db19`, árvore limpa, 4.580 expressões por `typst eval`, de 21:19:24 a 21:19:45 de Brasília. Vanilla `e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b`, igual ao manifesto. Cristalino: `lab/target/product/debug/typst`, SHA-256 `c9be6406c93bdd681ff1952a8474079246480e4e5a5b4d5955a2b42f5c0ae5f2`, compilado às 20:15, antes do merge do HEAD. O commit exato desse build não foi identificado. As fontes de `calc` só mudaram selos desde `958aa51a1`. O RED deve repetir a medição em um build do HEAD.

Script e saídas brutas em `lab/results/calc-package-2026-11-24/` (`calc.py`, `calc.json` SHA-256 `77055f8c1096962e372f28b3136bc0600dac94b6f74535264f0040470fe08686`). O diretório é local, não versionado.

| Resultado | Casos |
|---|---:|
| Iguais, inclusive o erro | 623 |
| Ambos erram, mensagem diferente | 3.644 |
| Cristalino rejeita, vanilla aceita | 166 |
| Cristalino aceita, vanilla rejeita | 56 |
| Valor ou texto diferente | 76 |
| Diferença relativa até 1e-12, ou só de grafia | 15 |

As contagens descrevem a grade escolhida, não a frequência em documentos.

## Causas

1. **Conversão saturante.** `floor`, `ceil` e `trunc` usam `as i64` (`01_core/src/compiler/stdlib/calc/round.rs:18`, `:45`, `:189`). Devolvem um inteiro errado em silêncio. A ADR-0101 (§3) afirma que o vanilla satura. No pin ele responde `the result is too large`.
2. **Extremos próprios.** `min` e `max` comparam com braços locais e coerção (`calc/extrema.rs`). Perdem o tipo, perdem precisão acima de 2^53, ignoram NaN e recusam tipos que `eval/operators/ordering.rs` já compara.
3. **Admissão sem decimal.** `quo`, `rem`, `rem-euclid`, `div-euclid` e `clamp` convertem tudo a float.
4. **`guard_float`.** Recusa todo resultado infinito ou NaN. O vanilla devolve infinito e recusa NaN só em `pow`, `exp` e `log`.
5. **`norm`.** Com `p: float.inf` devolve `1.0` para `(1, 2)`; o correto é `2.0`. Transborda em `norm(1e300)` e aceita `p` negativo.
6. **`erf`.** A aproximação difere na oitava casa: `erf(2.5)` é `0.999592964536606`, contra `0.999593047982555`.

## Escopo proposto

| Parte | Mudança | Efeito |
|---|---|---|
| A | `floor`, `ceil`, `trunc`, `quo`: erro `the result is too large` fora de i64 | valor errado vira erro |
| B | `min`, `max`: comparação da linguagem, devolvendo o argumento original | aceita mais tipos; `max(1, 0.5)` passa de `1.0` a `1`; NaN comparado com outro valor vira erro |
| C | `quo`, `rem`, `rem-euclid`, `div-euclid`, `clamp` com decimal e inteiro; decimal com float recusado | aceita mais entradas |
| D | `guard_float` sai das funções reais e de `round`; infinito e NaN propagam como nos operadores. `pow`, `exp` e `log` recusam NaN com `the result is not a real number` | erro vira valor |
| E | `norm`: `p` infinito, escala sem overflow, `p` não positivo recusado | corrige valor; `p < 0` vira erro (`p = 0` já era) |
| F | `erf`: `libm::erf`, já fixado em 0.2.11; NaN propaga | dígitos mudam |

**Compatibilidade.** A, B (NaN e tipo do resultado) e E (`p < 0`) alteram entradas que hoje compilam. D retira erros. Em código, os donos são `calc/{round,extrema,integer,power,logarithms,trig,hyperbolic,norm,erf,numeric}`. `Decimal` ganha operações verificadas (`checked_rem`, `checked_div` e afins), de uso interno em L1. Não há campo, variante, trait, flag, fase ou dependência nova; a permissão de `libm` para `erf` exige atualizar a lista em `crystalline.toml`.

**Mensagens.** Os erros que o pacote cria ou reescreve usam o texto do vanilla. As demais 3.644 diferenças, quase todas português contra inglês, ficam para a decisão de idioma.

## Divergências mantidas

- `norm` com entrada infinita devolve `float.inf`. O vanilla devolve NaN por dividir infinito por infinito, defeito que a ADR-0107 não obriga a copiar.
- `asin` e `acos` de NaN continuam erro. O vanilla devolve `0deg`.
- `exp(-float.inf)` continua `0.0`; `floor(float.nan)`, `ceil` e `trunc` continuam `0`, igual ao vanilla. Recusar NaN seria decisão própria.
- `clamp` com limite NaN continua erro comum; o vanilla entra em pânico.
- `floor`, `ceil` e `trunc` de `-9223372036854775808.0` continuam aceitos: o valor cabe em i64. O vanilla recusa.
- `quo` com NaN continua erro. O vanilla devolve `0`.
- `exp` com inteiro acima de i32 passa a seguir a regra geral e devolve `float.inf`. O vanilla responde `exponent is too large`, limite sem razão numérica.
- O segundo argumento posicional de `log`, `log10`, `deg` e `rad` ficam: são extensões existentes.

## Fora

Sonda complementar após a grade, no mesmo HEAD e com os mesmos binários, fora da grade: `round` com infinito, NaN e `digits`, `quo` com NaN, limites exatos de i64 e decimal com float. Ela sustenta as regras acima e entra na grade do RED.

Fora deste pacote: grafia de floats grandes (`1e300`), ângulo NaN (`atan`, `atan2`), idioma das mensagens, cor, construtores e operadores.

## ADR e donos

- **ADR-0101:** substituir a restrição categorial. Regra nova: o resultado segue a linguagem no pin; cada divergência é declarada no dono da função com sua medição. As listas acima são as exceções iniciais. A decisão anterior escolheu o status quo por custo zero; o registro histórico fica.
- **Dono único:** tipos, diagnósticos e limites passam ao filho de cada função. `calc.md` conserva a composição do módulo e perde a tabela de regras. Isso desfaz a autoridade circular em que os filhos remetem a `calc.md` e ele remete aos filhos.
- **Formato piloto:** contrato vigente, aceitação e uma linha de evidência com commit. Só nestes donos. Conferir cada obrigação antiga antes de remover texto.

## Aceitação

1. L0 e ADR primeiro, selos e linhagem limpos.
2. RED por parte A–F no argumento público, com valor, tipo, span e origem. Controles: zero com sinal, limites de i64, decimais, `em` incomparável, aridade e argumento nomeado.
3. Ajustar os testes que hoje fixam a política antiga, um a um e com justificativa. Entre eles `calc_exp_overflow_retorna_err`, `calc_erf_nan_err` e `existing_nonfinite_and_overflow_policy_is_preserved`.
4. Infinito e NaN de `calc` em comprimento, ângulo e página: saída e término iguais ao controle `float.inf`, sem pânico.
5. Repetir a grade. Toda diferença restante deve estar nas listas acima.
6. Suíte completa, lint estrito e linhagem reversa.

## Decisão pedida

Aprovar A–F, a nova redação da ADR-0101 e o dono único como um só pacote. A alternativa é aprovar A, B, C e E agora, e deixar D e F com a 0101 vigente.

## Execução

Árvore não commitada sobre `f2c737125`; `git diff HEAD --stat`: 35 arquivos, mais este diagnóstico, não rastreado. Os arquivos são a ADR-0101, `ORDER.md`, os L0s de `calc`, `tests/calc`, `decimal` e do contrato de CLI, seus consumidores em Rust e `crystalline.toml`.

- **RED:** seis testes novos de CLI falharam com o build do HEAD (`8e78403b6a9c943ec98879ef90d9ad073ad5e25c2d00e6faa041e164e0b922d3`) e passam agora.
- **Suíte:** 10.928 testes passaram, zero falhas, sete ignorados, com `TMPDIR` em `/dev/shm` e `TYPST_VANILLA` definidos. Lint sem erros nem avisos em uma cópia dos arquivos do repositório; no checkout ele também lê `.claude/worktrees/`, onde outra sessão tem cópias antigas. Linhagem reversa: 1.030 consumidores, zero erros.
- **Grade repetida**, 21:41:03 a 21:41:22, binário `8a82db8bdc7de8723261ffab5cee5c8639cc2f250677b5e6ea4ccc4b46245d77`, saída `calc-after.json` (SHA-256 `1f116e4e797e3bdf4ec6ad803317884f60542f085d5faaec57cb0b935639b84e`):

| Resultado | Antes | Depois |
|---|---:|---:|
| Iguais | 623 | 2.056 |
| Ambos erram, mensagem diferente | 3.644 | 2.430 |
| Cristalino rejeita, vanilla aceita | 166 | 21 |
| Cristalino aceita, vanilla rejeita | 56 | 10 |
| Valor ou texto diferente | 76 | 48 |
| Relativa até 1e-12 ou só grafia | 15 | 15 |

As 79 diferenças de aceitação ou valor são as divergências mantidas, a grafia de floats grandes e o ângulo NaN, mais um caso descoberto na execução: `pow(float.nan, 3)` dá `0.0` no vanilla e `pow(float.nan, -2)` dá infinito. O cristalino recusa, e `power.md` declara isso. Em 1e-12 ficam `pow(2.5, 2.5)` e `root`, que usam outra potência e não faziam parte do pacote.

**Ajustes internos.** O operador `<` do cristalino devolve `false` com NaN, e o vanilla erra. Operadores estavam fora do pacote; `extrema` trata o caso e o declara. `rem(-9223372036854775808, -1)` passou de erro a `0`, como no vanilla. `exp(+inf)` e `exp(NaN)` continuam recusados, com a mensagem do vanilla. Erros de tipo e de divisor zero das funções da parte C usam texto e posição do vanilla. A posição dos erros no span da chamada (decimal com float, resultado grande demais) continua a dos argumentos.

**Pendências.** Comparação de NaN nos operadores, potência por `libm` em `pow` e `root`, grafia de floats, ângulo NaN e idioma das mensagens.
