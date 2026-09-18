# Passo 1438 — Materializar `math.norm` sobre o carrier público `math.lr`

**Estado:** PROPOSTA L0 — AGUARDA CONFIRMAÇÃO HUMANA (ADR-0127)  
**Data da medição:** 2026-09-17T21:53:17-03:00  
**Commit medido:** `c8071e1004c4ec5ee80cc1b87aeb234d6a5c7e14`  
**Árvore medida:** `a7dd58494001347ca5bdd3dd4eaf1f74a6476e1a`, limpa (`git status --short` e `git diff HEAD --stat` sem saída)  
**Regime:** `executed_without_isolation_attestation`

## 1. Próxima unidade nominal

O inventário residual P1422 ordenou as rotas matemáticas abertas como
`abs, accent, cases, frac, limits, lr, mat, mid, norm, ...`. P1429–P1437
fecharam `accent`, `cases`, `frac`, `limits`, `lr`, `mat` e `mid`; o Passo
1437 identifica `norm` como a próxima rota nominal. `abs` permanece no
backlog, mas já não é a próxima unidade na sequência explicitamente retomada
por P1437. P1436 deixou tanto `math.norm` como os demais wrappers fora do seu
escopo, embora tenha materializado o carrier de que eles dependem.

## 2. Medição antes da decisão

### 2.1 Fonte vanilla ratificada

No pin `586e1bd43`, `crates/typst-library/src/math/lr.rs` declara `norm` como
função pública com `size: Option<Rel<Length>>` nomeado e `body: Content`
posicional obrigatório. O helper `delimited` constrói um `LrElem` cujo body é
a sequência `‖`, conteúdo, `‖`; só marca `size` quando o argumento está
presente. O módulo `math` registra a função antes dos wrappers de estilo.

SHA-256 da fonte medida:

| Fonte | SHA-256 |
|---|---|
| `lab/typst-original/crates/typst-library/src/math/lr.rs` | `2a762699e7d3eed9a0b4b2d1afbbf5b58649339ec47ec12a2a82907519c107e0` |

### 2.2 Observáveis públicos medidos no vanilla

Com `/usr/local/bin/typst` ratificado (`typst 0.15.1 (586e1bd4)`):

- `repr(math.norm)` produz `norm`;
- `repr(math.norm([x]))` produz `lr(body: sequence([‖], [x], [‖]))`;
- `repr(math.norm(size: 150%, [x]))` conserva
  `size: 150% + 0pt` e a mesma sequência;
- `none` vira conteúdo vazio, string vira conteúdo e inteiro é rejeitado;
- nesting preserva dois carriers `lr`, cada qual com suas cercas;
- `$ norm(x) $` produz o mesmo `lr` dentro da equação;
- spread posicional canônico de um item é aceito;
- ausência falha com `missing argument: body`; segundo positional falha com
  `unexpected argument`; `body:` falha como positional-only; named desconhecido,
  `size` duplicado e tipo inválido falham pelos diagnósticos canônicos.

### 2.3 Estado cristalino medido

`make_math_module` não registra `norm`; todas as sondas qualificadas falham
com `module math does not contain norm`. A forma bare cai no braço histórico
comum de wrappers, descarta todos os named e constrói `MathDelimited`; assim
`repr($ norm(x) $)` produz `equation(block: true, body: ‖x‖)`, sem identidade
`lr`, presença de `size` ou convergência com a função qualificada.

Hashes SHA-256 dos consumers antes da proposta:

| Consumer | SHA-256 |
|---|---|
| `01_core/src/compiler/stdlib/structural/math.rs` | `8fc188b1152e6b30196f8242dbf12f8544740883ccb730582ad06d4e6355c8bb` |
| `01_core/src/compiler/eval/math.rs` | `6cc4031c9bde18d3fec0be8d448c93a4ac716605a12fdf6cd9f3bac1f4d8b364` |

## 3. Classificação anti-deriva

- Binding `math.norm`, assinatura, casts, erros, presença de `size`, nesting,
  identidade `lr` e morfologia `‖body‖` são linguagem (ADR-0107).
- O nome do helper Rust, a montagem de `Vec` e a representação interna da
  sequência são mecânica e não concedem paridade.
- Inferência: a chamada deve reutilizar exatamente o carrier público `MathLr`
  de P1436, porque a fonte vanilla retorna `LrElem` e a sonda pública revela
  `lr(...)`. Seria refutada por fonte pinada que retornasse outro elemento ou
  por sonda pública incompatível.
- A igualdade visual aproximada de `MathDelimited('‖', ..., '‖')` não basta:
  ela apaga `size`, cascata, identidade, nesting e `repr` observáveis.

## 4. Decisão L0 proposta

1. Registrar `math.norm` antes do espelho `sym → math` como função nativa.
2. Aceitar exatamente um `body` posicional convertível a conteúdo e somente
   `size: Rel<Length>` nomeado; preservar ocorrência explícita inclusive para
   `100%`, aceitar spread canônico e rejeitar duplicados ou extras.
3. Construir `Content::MathLr` com body `MathSequence([‖, body, ‖])`, default
   `100%` não explícito ou o `size` explícito. Não usar `MathDelimited` nem
   executar layout no builder.
4. Fazer `norm(...)` bare avaliar cada ocorrência uma vez e delegar ao mesmo
   parser/construtor da função qualificada, preservando ordem, spread e erros.
5. Reutilizar integralmente `repr`, cascata, short-fall, layout, igualdade e
   walkers de `MathLr`; não criar nova variante ou duplicar política P1436.
6. Manter `abs`, `round`, `floor`, `ceil`, `bar` e as outras treze rotas
   residuais fora deste recorte.

## 5. Gate ADR-0127

A proposta adiciona função pública e muda o comportamento por defeito da forma
bare de `MathDelimited` sem identidade para `MathLr`. Incide nos itens 1 e 2
da ADR-0127. Este commit contém somente passo e L0 e **PARA**; código, testes,
headers e resselo começam apenas após confirmação humana explícita.

## 6. Execução após confirmação

1. Selar contrato/oráculos RED para binding, assinatura, casts, diagnósticos,
   presença, spread, convergência bare/qualificada, nesting e `repr`.
2. Confirmar RED atribuível no cristalino e GREEN bilateral no vanilla onde
   aplicável.
3. Implementar builder compartilhado e o registro, sem novo carrier.
4. Atacar mutantes que removem cercas, trocam `MathLr` por `MathDelimited`,
   apagam presença, aceitam extra/named desconhecido, ignoram erro ou deixam a
   rota bare no braço antigo. Exigir score `1.0`, zero `Unknown` promovido.
5. Resselar apenas após V15/V26 válidos.
6. Validar `cargo fmt --all -- --check`, `git diff --check`, testes focais,
   `cargo build --workspace` e `crystalline-lint .`, distinguindo as oito
   falhas históricas da suíte completa já registradas em P1437 de qualquer
   regressão atribuível a P1438.

## 7. Confirmação e materialização

O dono confirmou explicitamente o gate ADR-0127 após o commit de proposta
`0a1d813f1243f04d36435632dc364919aec4ce15`.

P1438 foi materializado em 2026-09-17 sob o regime
`executed_without_isolation_attestation`. O RED atribuível compilou o harness
e fez os quatro oráculos congelados falharem somente em
`math.norm callable: None`. O candidato registrou a função, extraiu a política
compartilhada de `size`, construiu `MathLr` com body `‖`, conteúdo, `‖` e fez a
rota bare delegar ao mesmo builder. Depois do alinhamento do teste histórico
P899, o focal terminou com cinco testes verdes.

A primeira tentativa do runner parou antes de executar mutantes por conflito
de encoding Ruby entre bytes e `‖`; não recebeu crédito. A primeira campanha
física válida matou 27/30 porque M09 selecionou um braço `Value::None`
homônimo fora do conversor P1438. Após ancoragem no owner correto, duas
campanhas integrais sucessivas — a última já com a regressão histórica
sincronizada — mataram 30/30 mutantes nas ordens direta, direta repetida e
reversa, score `1.0`, zero `Unknown`.

As sondas públicas qualificadas e bare reproduziram função, body vazio,
string, nesting, `size: 150%` e seis diagnósticos negativos do vanilla.
`cargo fmt --all -- --check`, `git diff --check`, `cargo build --workspace` e
`crystalline-lint .` passaram; V5/V15/V26 ficaram limpos. A suíte completa
executou 5.862 testes: 5.854 verdes e as mesmas oito falhas históricas já
registradas em P1437, nenhuma atribuível a P1438. O certificado final abaixo
fixa o commit e a árvore candidatos sem ampliar o recorte.

O candidato produtivo e os recibos foram fixados no commit
`a38af56fd041328cb0b600168fec6a7e4f89ba4d`, árvore
`ff90bd3103af2fc3bddb210bed29fc03d642dbfa`. O certificado limitado ao
fragmento P1438 é `00_nucleo/diagnosticos/p1438-verifier-certificate.json`.
