# Passo 1431 — convergência vertical completa de `math.mat`

## Estado medido antes da decisão

P1430 fechou e certificou `math.cases`; restam 17 rotas `math.*` da coorte
nominal P1422. A próxima fatia coerente é `math.mat`: já existem carrier,
parser bare e layout dedicado, mas a função qualificada não está registrada,
o carrier não preserva toda a assinatura pública, o caminho bare aceita apenas
um subconjunto permissivo e `repr` reduz qualquer matriz à palavra `matrix`.

Medição executada em `2026-09-17T08:12:07-03:00`, sobre HEAD limpo
`fbf9542505875fce7702dd0276bff8773c5e8413`, tree
`4942241a8a49700f165ca4bd1c0450c1b3b3c418`. `git status --short` e
`git diff HEAD --stat` não produziram linhas. Entradas principais:

| artefato | SHA-256 |
|---|---|
| vanilla `typst-library/src/math/matrix.rs` | `fb6ed72f7fac4bb5489170059b9aa0f919beb0b1a23bf4e862d4cff52905120f` |
| vanilla `typst-library/src/math/ir/resolve.rs` | `115d775641509a755a1b7f4bd6da26cc8502a09e0e23e303d38d4a7cd76e0112` |
| cristalino `compiler/stdlib/structural/math.rs` | `c82b8ff8c8743eba08119cad737caa759934148a39d502a88b76867be8279756` |
| cristalino `compiler/eval/math.rs` | `a413b664e99485be2a77f9e3f2ea0e815661956d44a0a8c560c0fe12b7e0cff9` |
| cristalino `compiler/eval/repr.rs` | `6daff397e7ecbdd91f889e31b21fc0294aa191885fca04ef472168ccb3876930` |
| cristalino `compiler/eval/rules.rs` | `92472b1dacc052880ee5e3197c60027b78a79ac6f3a1272e29b3f4ab5816abfa` |
| cristalino `compiler/math/layout/matrix.rs` | `a7facbf2f648778bf4ab29e985deadb68177558685828e16b1f3ad3a54584c65` |
| cristalino `compiler/math/layout/mod.rs` | `f54e3017110632972f976439d2a092990cb6d71e17533af1949f37eaeb28bc6d` |
| cristalino `entities/content.rs` | `ad0f0ead17586b71a7f1c36079c36ca16ae65b60dd73ac00304faeb5b73e77a1` |
| cristalino `entities/elements/math_matrix.rs` | `b03d3149399f23ead6e33b4be53a3de1efd934d772cd0df3e976deea26fab65a` |

O alvo permanece o vanilla ratificado
`586e1bd43fae6c9a973218163d3165c53ab8d16d`. A fonte declara `MatElem` com
`delim`, `align`, `augment`, `row-gap`, `column-gap` e rows variádicas. `gap`
é shorthand externo: alimenta ambos os gaps, mas `row-gap`/`column-gap`
explícitos prevalecem. Se qualquer positional é array, todos devem ser arrays
e cada array vira uma linha; caso contrário, os posicionais formam uma única
linha. Linhas menores recebem células vazias até à largura máxima.

Sondas públicas diretas no binário ratificado mediram:

| expressão | resultado público |
|---|---|
| `repr(math.mat())` | `mat(rows: ((),))` |
| `repr(math.mat([x], [y]))` | `mat(rows: (([x], [y]),))` |
| `repr(math.mat((1, 2), (3,)))` | segunda linha é completada como `([3], [])` |
| `delim: none` | `delim: (none, none)` |
| `delim: ("[", none)` | preserva o par assimétrico |
| `align: start|left|center|right|end` | alinhamento horizontal preservado |
| `gap: 1em, row-gap: 2em` | `row-gap: 2em`, `column-gap: 1em` |
| `row-gap: 150%`, `column-gap: 2em` | ambos permanecem relativos |
| `augment: 2` | uma vline, `repr` compacto `augment: 2` |
| `augment: (hline: 1, vline: (1, 2))` | offsets canônicos e `stroke: auto` |
| `augment: none` explícito | campo permanece no `repr` |
| mistura de rows-array e content | `expected array, found content` |
| `delim: 1` | `expected array, none, symbol, or string, found integer` |
| align vertical | lista fechada `start|left|center|right|end` |
| gap inteiro | `expected relative length, found integer` |
| augment string | `expected integer, dictionary, or none, found string` |
| named desconhecido/`rows:`/duplicado | `unexpected argument` / `duplicate argument` |

O estado cristalino diverge em cinco pontos observáveis. `make_math_module`
não registra `mat`; o braço bare usa `if let Ok` e ignora erros; `MathMatrixElem`
não possui `align`, gaps relativos nem carrier completo de augment/presença;
`repr` imprime apenas `matrix`; o layout centra sempre, aceita somente uma
linha vertical positiva e usa gaps absolutos opcionais.

## Classificação e decisão proposta

Rows retangulares, delimitadores opcionais, alinhamento, gaps relativos,
augment, presença explícita, diagnósticos, `repr` e geometria visível são
semântica/sintaxe/morfologia da língua (ADR-0107). A forma Rust do carrier,
ordem dos loops e estrutura interna da grelha são mecânica. A declaração
`MatElem`, os parsers de fields e `resolve_mat` sustentam a intenção; esta
classificação seria refutada por outra definição no pin ou por sondas públicas
válidas com assinatura ou morfologia incompatível.

1. promover `MathMatrixElem` para rows retangulares, par opcional de
   delimitadores, `HAlign`, `Rel<Length>` independente para linhas/colunas,
   augment completo e bits de presença para os cinco campos materializados;
2. representar augment como offsets horizontais/verticais assinados e stroke
   opcional (`None = auto`), distinguindo augment omitido de `none` explícito;
3. registrar `math.mat` com posicionais variádicos e somente `delim`, `align`,
   `augment`, `gap`, `row-gap`, `column-gap`; `rows:` não é campo nomeado;
4. aplicar o algoritmo vanilla de rows: zero ou escalares formam uma linha,
   presença de qualquer array exige arrays em todos os posicionais, e rows
   desiguais são completadas com conteúdo vazio;
5. expandir `gap` causalmente para os dois eixos, deixando os named específicos
   prevalecerem independentemente, sem guardar um terceiro estado redundante;
6. fazer bare e qualificado delegarem ao mesmo parser/construtor; cada valor é
   avaliado uma vez, erros e duplicados propagam, e `&` permanece na célula;
7. ligar os cinco campos materializados à `StyleChain`, com precedência
   explícito > cascata > defaults: `()`, `center`, `none`, `0.2em`, `0.5em`;
   augment explícito dobra somente o stroke externo conforme a regra vanilla;
8. projetar `repr` na ordem `delim`, `align`, `augment`, `row-gap`,
   `column-gap`, `rows`; `gap` nunca aparece como campo próprio;
9. adaptar o layout para align horizontal, gaps relativos e todas as linhas de
   augment, incluindo offsets negativos/de borda, stroke customizado e
   diagnóstico de offset fora da matriz, preservando P825/P912/P919/P923/
   P945/P1042;
10. preservar todos os campos e bits em igualdade, hash, walkers,
    `apply_math_default`, `apply_math_style` e reconstruções.

## Limites explícitos

P1431 não fecha as outras 16 rotas residuais, não altera `math.vec` nem
`math.cases`, não generaliza `DelimiterPair`, não muda a gramática de `;`/`&`,
não promete toda a linguagem de stroke fora de augment e não reabre as
fórmulas geométricas já certificadas. A aceitação é por observáveis da língua,
não por igualdade de bytes de render ou de estrutura Rust.

Falha de build/execução, timeout, OOM, swap, stream truncado, observador opaco
ou identidade não verificável são `Unknown` e bloqueiam; nunca recebem crédito.

## Gate ADR-0127

O passo registra função pública, altera campos e constructors públicos do
carrier e amplia comportamento por default/cascata/layout. Incide nos itens
1, 2 e 4 do ADR-0127.

Fluxo obrigatório:

1. atualizar antes do código os oito L0 proprietários afetados;
2. commitar esta proposta e os L0s;
3. **PARAR sem alterar L1/L3 nem headers de linhagem**;
4. continuar somente após confirmação explícita do dono;
5. após confirmação, produzir manifesto, contrato, oráculos, ataques e testes
   segregados, selar, obter RED atribuível e só então implementar.

## Regime posterior ao gate

Aplicar o protocolo completo `tekt-materializacao-segregada`, executado sem
atestação de isolamento no workspace partilhado. O contrato deve cobrir
binding, rows escalares/arrays/zero, padding, casts, duplicados, delimitadores,
align, shorthand/precedência de gaps, cascata, augment inteiro/dict/none,
offsets/stroke, presença, carrier, walkers, repr, `&` e geometria. Mutation
score exigido: `1.0`; `Unknown` é fail-closed.

## Gates de fecho após confirmação

- RED atribuível do binding, rows retangulares e carrier completo;
- RED de casts, duplicados, presença e `gap` versus gaps específicos;
- RED de augment simples/dicionário/none, offsets negativos e inválidos;
- RED geométrico de align, gaps relativos, hline/vline/stroke e `&`;
- GREEN bilateral das rotas bare e qualificada;
- controles P825/P912/P919/P923/P945/P1042 preservados;
- três ordens determinísticas e hashes selados intactos;
- `cargo fmt --all -- --check`, testes focais, `cargo build`,
  `crystalline-lint .` e `git diff --check`.

## Confirmação do gate

**Pendente de confirmação explícita do dono.** Nenhuma alteração produtiva de
P1431 fica autorizada antes dessa confirmação.
