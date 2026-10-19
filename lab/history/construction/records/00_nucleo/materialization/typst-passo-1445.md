# Passo 1445 — Materializar `math.round` sobre o carrier público `math.lr`

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


**Estado:** PROPOSTA L0 — AGUARDA CONFIRMAÇÃO HUMANA (ADR-0127)  
**Data da medição:** 2026-09-18T13:28:07-03:00  
**Commit medido:** `a748c433e8aa135690e2f880455a5a75533bed86`  
**Árvore medida:** `8400e6f0d43db59405fefc42c63cf6f80e144ae0`, limpa (`git status --short` e `git diff HEAD --stat` sem saída)  
**Regime:** `executed_without_isolation_attestation`

## 1. Próxima unidade nominal reconciliada

O inventário residual retomado em P1444 lista, depois de `math.primes`,
`math.root` e `math.round`. A leitura do histórico refuta a escolha mecânica de
`root`: P1426 já materializou e certificou essa rota, com confirmação humana e
binding público. Portanto a próxima rota ainda aberta é `math.round`.
`math.abs`, deliberadamente não retomada pela sequência recente, continua no
backlog e não é declarada fechada.

## 2. Medição antes da decisão

### 2.1 Fonte vanilla ratificada

No pin `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
`crates/typst-library/src/math/lr.rs:91-105` declara `round` como função
pública com `size: Option<Rel<Length>>` nomeado e `body: Content` posicional
obrigatório. O helper comum constrói `LrElem` com body
`sequence(⌊, body, ⌉)` e só fixa `size` no elemento quando o argumento esteve
presente. `math/mod.rs:77` registra a função no módulo.

Hashes identificador administrativo das fontes medidas:

| Fonte | identificador administrativo |
|---|---|
| `lab/typst-original/crates/typst-library/src/math/lr.rs` | [dado administrativo removido] |
| `lab/typst-original/crates/typst-library/src/math/mod.rs` | [dado administrativo removido] |

O binário `/usr/local/bin/typst`, identificador administrativo
[dado administrativo removido],
coincide byte a byte com o build ratificado em
`lab/typst-original/target/release/typst`.

### 2.2 Observáveis públicos medidos no vanilla

- `repr(math.round)` produz `round`;
- `repr(math.round([x]))` produz
  `lr(body: sequence([⌊], [x], [⌉]))`;
- `size: 150%` e `size: 100%` explícitos permanecem no `repr` como tamanho
  explícito, enquanto a omissão não imprime o campo;
- `none` vira conteúdo vazio, string vira conteúdo e inteiro é rejeitado;
- nesting conserva um `lr` interior como conteúdo do `lr` exterior;
- `$ round(x) $` e `$ round(x, size: #150%) $` produzem o mesmo carrier das
  chamadas qualificadas, dentro de `equation`;
- ausência falha com `missing argument: body`; segundo positional com
  `unexpected argument`; `body:` com ``the argument `body` is positional``;
  named desconhecido, `size` duplicado e casts inválidos falham pelos
  diagnósticos canônicos.

As sondas foram executadas no commit e árvore acima, às 13:28:07-03:00, com
`/usr/local/bin/typst eval --jobs 1`; todas as positivas terminaram com exit
zero e todas as negativas com exit um.

### 2.3 Estado cristalino medido

`make_math_module` não registra `round`; `repr(math.round)` e a chamada
qualificada falham com `module math does not contain round`. A forma bare cai
no braço histórico agregado, ignora named e constrói
`MathDelimited('⌊', body, '⌉')`; a sonda devolve
`equation(block: true, body: ⌊x⌉)`, apagando identidade `lr`, presença de
`size`, cascata e nesting público.

Hashes identificador administrativo dos consumers antes da proposta:

| Consumer | identificador administrativo |
|---|---|
| `01_core/src/compiler/stdlib/structural/math.rs` | [dado administrativo removido] |
| `01_core/src/compiler/eval/math.rs` | [dado administrativo removido] |

## 3. Classificação anti-deriva

- Binding, assinatura, casts, diagnósticos, presença de `size`, nesting,
  identidade `lr` e as cercas assimétricas `⌊…⌉` são linguagem (ADR-0107).
- O nome do helper Rust, a montagem do vetor e a variante concreta da
  sequência são mecânica e não concedem crédito.
- Inferência: `math.round` deve reutilizar o carrier P1436, pois a fonte
  retorna `LrElem` e as sondas revelam `lr(...)`; seria refutada por outra
  declaração no pin ou por observável público incompatível.
- A aparência aproximada do `MathDelimited` atual não basta: ela perde
  `size`, cascata, identidade, nesting e `repr`. A fonte e as sondas refutam
  o enquadramento cômodo de que o wrapper já estaria implementado.

## 4. Decisão L0 proposta

1. Registrar `math.round` antes do espelho `sym → math` como função nativa.
2. Aceitar exatamente um `body` posicional convertível a conteúdo e somente
   `size: Rel<Length>` nomeado; preservar ocorrência explícita inclusive para
   `100%`, aceitar spread canônico e rejeitar duplicados ou extras.
3. Construir `Content::MathLr` com
   `MathSequence([⌊, body, ⌉])`, default `100%` não explícito ou o `size`
   explícito. Não usar `MathDelimited` nem executar layout no builder.
4. Fazer `round(...)` bare avaliar cada ocorrência uma vez e delegar ao mesmo
   parser/construtor da forma qualificada, preservando ordem, spread e erros.
5. Reutilizar `repr`, cascata, short-fall, layout, igualdade e walkers de
   `MathLr`; não criar variante, campo, constructor ou fase nova.
6. Manter `abs`, `stretch`, `underbrace`, `underbracket`, `underparen`,
   `undershell`, `floor`, `ceil` e `bar` fora deste recorte.

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
3. Implementar builder compartilhado e registro, sem novo carrier.
4. Atacar remoção/troca das cercas, retorno de `MathDelimited`, perda de
   presença, aceitação de extra/named desconhecido, descarte de erro e rota
   bare ainda legada. Exigir score `1.0` e zero `Unknown` promovido.
5. Resselar somente após V15/V26 válidos.
6. Validar `cargo fmt --all -- --check`, `git diff --check`, testes focais,
   `cargo build --workspace`, suíte integral e `crystalline-lint .`, separando
   as oito falhas históricas do baseline P1444 de qualquer regressão P1445.

## 7. Confirmação humana

O dono confirmou explicitamente o gate ADR-0127 em 2026-09-18 após o commit
de proposta `b90adbee7a315b420d51b393f882cacbb7f78579`. A cadeia RED→GREEN,
ataques, resselo e veredito fica autorizada somente no recorte `math.round`
descrito acima.

## 8. Materialização e veredito

Medição final em `2026-09-18T15:46:41-03:00`, sobre working tree não
commitado derivado do commit de proposta
`b90adbee7a315b420d51b393f882cacbb7f78579`. O estado exato é enumerado por
`git status --short`; `git diff HEAD --stat` registra nove ficheiros rastreados
com 119 inserções e 14 remoções, e os novos artefatos P1445 constam no mesmo
status.

- RED atribuível: cinco testes executaram e falharam somente por binding
  qualificado ausente e carrier bare ainda `MathDelimited`.
- GREEN focal: 5 passados, 0 falhados.
- Ataques: 30/30 execuções eficazes mortas nas ordens direta, repetida e
  inversa; score 1,0, zero `Unknown`, todos M01–M10 mortos 3/3.
- Paridade pública positiva: função, cast, `none`, string, nesting, presença
  de `size`, cercas `⌊…⌉` e convergência bare/qualificada preservados. A
  quebra multilinha do pretty-printer global difere para representações
  longas e permanece fora deste recorte; conteúdo semântico e morfologia são
  iguais.
- Nos sete negativos, status e mensagem primária coincidem com o vanilla.
- Gates: `cargo fmt --check`, `git diff --check`, `cargo build --workspace` e
  `crystalline-lint .` passaram; V5/V15/V26 = 0.
- Suíte integral: 5.884 passados e os mesmos oito falhados históricos do
  baseline, nenhuma falha atribuível a P1445.

Veredito limitado: `Preserved` para o fragmento `math.round` descrito; não
declara equivalência Typst geral, paridade integral do pretty-printer nem
fecha as rotas residuais de P1422.

O candidato produtivo e os recibos foram fixados no commit
`f2fa4aeb6ee954946be6412cc03360ca2b3d03bc`, árvore
`316bc7f6a71827fd38675472c98ae2dcc9e5f9b0`. O certificado limitado ao
fragmento P1445 é relatório consolidado da limpeza.
