# Passo 1430 — convergência vertical completa de `math.cases`

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Estado medido antes da decisão

P1429 fechou e certificou `math.accent`. Recontando a lista nominal P1422 a
partir do estado presente, ficam 18 rotas `math.*` ainda parciais; a menção a
“19” no limite histórico de P1429 não descontava uma das quatro rotas já
fechadas (`limits`, `root`, `frac`, `accent`) e não é usada como prova. A
próxima fatia coerente é `math.cases`: já há carrier, cascata e layout
dedicados, mas o módulo público ainda não contém `cases`, o caminho bare
constrói uma representação pré-resolvida diferente da identidade vanilla e o
`repr` cristalino reduz qualquer instância à palavra `cases`.

Medição executada em `2026-09-16T22:46:24-03:00`, sobre HEAD limpo
`9f21b21802698c07dcf897b076aad429f51db251`. `git status --short` e
`git diff HEAD --stat` não produziram linhas. Entradas principais:

| artefato | identificador administrativo |
|---|---|
| vanilla `typst-library/src/math/matrix.rs` | [dado administrativo removido] |
| vanilla `typst-library/src/math/ir/resolve.rs` | [dado administrativo removido] |
| cristalino `compiler/stdlib/structural/math.rs` | [dado administrativo removido] |
| cristalino `compiler/eval/math.rs` | [dado administrativo removido] |
| cristalino `entities/elements/math_cases.rs` | [dado administrativo removido] |
| cristalino `compiler/eval/repr.rs` | [dado administrativo removido] |
| cristalino `compiler/math/layout/cases.rs` | [dado administrativo removido] |
| cristalino `entities/content.rs` | [dado administrativo removido] |
| cristalino `compiler/eval/rules.rs` | [dado administrativo removido] |

O alvo continua o vanilla ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d`. A fonte declara `CasesElem`
com `delim: DelimiterPair = ("{", "}")`, `reverse: bool = false`,
`gap: Rel<Length> = 0.2em` e `children: Vec<Content>` variádicos. O resolver
transforma cada child em um ramo, resolve pontos `&` dentro dele, usa estilo
de denominador e mostra somente o delimitador de abertura à esquerda ou, em
`reverse`, o delimitador de fecho à direita.

Sondas públicas diretas no binário ratificado mediram:

| expressão | resultado público |
|---|---|
| `repr(math.cases())` | `cases(children: ())` |
| `repr(math.cases([x], [y]))` | `cases(children: ([x], [y]))` |
| `delim: "[", reverse: true, gap: 1em` | par normalizado `("[", "]")`, `true`, `0% + 1em` e tuple de children |
| `delim: none` | `delim: (none, none)` |
| `delim: ("[", none)` | preserva o par assimétrico |
| `gap: 150%` | `gap: 150% + 0pt` |
| `reverse: false` explícito | o campo permanece no `repr` |
| positional inteiro | `expected content, found integer` |
| `delim: 1` | `expected array, none, symbol, or string, found integer` |
| `reverse: 1` | `expected boolean, found integer` |
| `gap: 1` | `expected relative length, found integer` |
| named desconhecido/duplicado | `unexpected argument: <nome>` / `duplicate argument: <nome>` |
| `children:` named | `unexpected argument: children` |

## Classificação e decisão proposta

Children, delimitadores, direção, gap, presença explícita, `repr`, casts e
geometria observável pertencem à semântica/morfologia da língua (ADR-0107).
O tipo Rust usado para o par, a fase exata em que `&` vira colunas e a ordem
interna das operações de grelha são mecânica. A declaração `#[elem]`, os
defaults explícitos e `resolve_cases` sustentam intenção; a inferência seria
refutada por outra definição no mesmo pin ou por sondas bilaterais válidas
com assinatura/morfologia diferente.

1. promover o carrier de linhas pré-resolvidas para `children: Vec<Content>`,
   par de delimitadores opcionais, `gap: Rel<Length>`, `reverse` e bits de
   presença independentes para `delim`, `reverse` e `gap`;
2. manter um wrapper compatível para callsites internos antigos, mas fazer a
   fronteira pública e a sintaxe bare usarem um constructor completo único;
3. registrar `math.cases` com children posicionais variádicos — inclusive
   zero — e somente `delim:`, `reverse:` e `gap:` nomeados; named podem
   anteceder children, duplicados e argumentos desconhecidos falham;
4. aplicar casts e normalização do par na fronteira: `none`, string/símbolo
   unitário inferível ou array de dois delimitadores opcionais; nenhum sentinel
   interno aparece no observável público;
5. ligar os três campos à `StyleChain`: o layout deriva valores efetivos
   temporários com precedência argumento explícito > cascata > defaults
   `("{", "}")`, `false`, `0.2em`, sem mutar o carrier nem fabricar presença;
6. fazer chamadas bare e qualificadas convergirem no mesmo carrier sem
   pré-resolver children em rows; cada child é avaliado uma vez e `&` continua
   dentro do ramo até o consumer de layout;
7. projetar `repr` como `cases(delim?, reverse?, gap?, children: tuple)`, nessa
   ordem, mostrando os três opcionais somente quando causalmente explícitos;
8. adaptar o layout para receber children, formar um ramo por child, separar
   pontos de alinhamento e preservar denominador, gap relativo, direção,
   delimiter stretch, eixo e limites externos já fechados;
9. preservar o carrier completo em hash, igualdade, walkers,
   `apply_math_default`, `apply_math_style` e toda reconstrução.

A aceitação é semântica, sintática e morfológica. Estrutura de vetores, uso de
`Option<char>` versus sentinel privado, momento da separação por `&` e bytes
de render são mecânica; diagnósticos, identidade `cases`, ordem dos children,
campos explícitos e geometria visível são observáveis.

## Limites explícitos

P1430 não fecha as outras 17 rotas residuais, não promove `math.mat` nem
`math.vec`, não altera a gramática de `&`, não generaliza `DelimiterPair` para
todo o sistema e não reabre as fórmulas geométricas já certificadas em
P919/P923/P945/P1042/P1132. O wrapper interno de compatibilidade não concede
nova superfície pública nem pode apagar boundaries de células existentes.

Falha de build/execução, timeout, OOM, swap, stream truncado, observador opaco
ou identidade não verificável são `Unknown` e bloqueiam; nunca recebem crédito.

## Gate ADR-0127

O passo altera campos públicos de `MathCasesElem`, acrescenta constructor e
função pública e muda assinaturas internas que transportam o carrier. Incide
nos itens 1 e 4 do ADR-0127.

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
binding, zero/um/muitos children, casts, normalização de delimitadores,
named/duplicados, presença explícita, cascata, convergência bare/qualificada,
carrier, walkers, repr, alinhamento por `&`, direção, gap relativo e regressão
da geometria vigente. Mutation score exigido: `1.0`; `Unknown` é fail-closed.

## Gates de fecho após confirmação

- RED atribuível do binding e do carrier completo;
- RED de casts, duplicados, par opcional e presença explícita;
- RED de cascata e precedência explícito > cascata > default;
- RED geométrico discriminando esquerda/direita, `&`, 0.2em/1em/percentual;
- GREEN bilateral das rotas bare e qualificada;
- três ordens determinísticas e hashes selados intactos;
- `cargo fmt --all -- --check`, testes focais, `cargo build`,
  `crystalline-lint .` e `git diff --check`.

## Confirmação do gate

**Confirmado explicitamente pelo dono em 2026-09-16.** A materialização fica
autorizada somente dentro do recorte completo de `math.cases` e dos oito L0
proprietários atualizados; as outras 17 rotas math permanecem fora de escopo.
