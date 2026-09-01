# Passo 1292 — fecho dos quatro resíduos acionáveis da superfície padrão

**Estado:** pronto para execução, sujeito ao gate L0/ADR-0127 deste documento  
**Data de autoria:** 2026-08-31  
**Alvo de paridade:** vanilla ratificado `a51e02804`  
**Regime:** materialização Tekt segregada completa, com quatro obrigações independentes  
**Resultado pretendido:** materializar `math.cancel`, `math.underline`, `math.vec` e `place.flush` sem regressão dos membros já equivalentes

---

## 1. Natureza e limite deste passo

Este é um **Passo de Execução**, não um Prompt L0. Ele coordena a última milha
dos quatro resíduos ainda acionáveis encontrados pela sonda padrão depois de
P1288–P1291.

O passo não autoriza código antes da atualização e do selo humano dos Prompts
L0 afetados. Em particular, `math.underline`, `math.vec` e `place.flush`
introduzem entidades públicas e comportamento de layout; portanto atravessam
o gate obrigatório de ADR-0127.

O fecho é dividido em quatro lotes rejeitáveis separadamente:

| Lote | Obrigação pública | Estado inicial | Fecho mínimo |
|---|---|---|---|
| A | `math.cancel` | entidade, layout e callback já materializados; binding ausente | registrar o construtor correto e provar a morfologia/semântica completa |
| B | `math.underline` | dois L0s em rascunho, sem consumers | materializar entidade, construtor e layout matemático próprio |
| C | `math.vec` | dois L0s em rascunho, sem consumers | materializar entidade, sintaxe/construtor e layout com `gap` contextual |
| D | `place.flush` | membro, elemento e efeito no ponto do fluxo ausentes | expor o submembro da função `place` e esvaziar floats pendentes naquele ponto |

Nenhum lote recebe crédito pela mera presença do nome. O binding só pode ser
exposto quando o caminho que ele torna alcançável cumpre o contrato público do
lote.

---

## 2. Medição preliminar anterior à decisão

### 2.1 Proveniência

Medição de autoria, ainda não utilizável como selo final:

- instante: `2026-08-31T20:52:42-03:00`;
- `HEAD`: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`;
- árvore não commitada;
- lista exata dos caminhos e `git diff HEAD --stat`:
  `00_nucleo/diagnosticos/p1292-authoring-baseline-status.txt`;
- SHA-256 desse `git status --short`:
  `c364ff8b1d84fb5512171eea890b75580ae6e2258deeda163ffc1f9c97124067`;
- SHA-256 do `git diff HEAD --stat`:
  `0576e0347624f80c525a73140debeebd880091e34510c8ef16fd8ce77c7e2d48`.

Binários e resultado usados:

| Artefato | SHA-256 |
|---|---|
| `target/release/typst` | `06314ba817d3ad85bf015b326f78b1018742c6cb1fcb732f7a041c44db5c0aa9` |
| `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |
| `/tmp/p1291-post-close-probes-default.json` | `61df7e72f0fed09e7ec3b47eab1cf1b6f2a30a3dd2705f020b60a41473303dc6` |

A execução registrou 111 sondas: 95 `MATCH` e 16
`DIFFERENCE_OR_DISABLED`. Do conjunto não equivalente, doze já estavam
classificadas fora destes lotes — duas corretamente desligadas no perfil e dez
extras cristalinos — e quatro eram membros realmente ausentes:

```text
math.cancel    -> module 'math' does not contain field "cancel"
math.underline -> module 'math' does not contain field "underline"
math.vec       -> module 'math' does not contain field "vec"
place.flush    -> cannot access fields on type function
```

O vanilla devolveu, respectivamente:

```text
(function, "cancel")
(function, "underline")
(function, "vec")
(function, "flush")
```

Estes números são somente o ponto de partida. A primeira ação da execução é
capturar de novo a árvore, os binários e as sondas; qualquer divergência invalida
o baseline acima e exige reclassificação antes de editar L0 ou código.

### 2.2 Medições públicas adicionais já reproduzidas

No vanilla ratificado:

```text
repr(math.cancel([x]))        == "cancel(body: [x])"
repr(math.underline([x]))     == "underline(body: [x])"
repr(math.vec([a], [b]))      == "vec(children: ([a], [b]))"
repr(math.vec())              == "vec(children: ())"
repr(place.flush())           == "flush()"
repr(place.with(dx: 1pt).flush)   == "flush"
repr(place.with(dx: 1pt).flush()) == "flush()"
```

`place.flush` rejeita qualquer posicional e qualquer named desconhecido.
`math.underline` rejeita body ausente. `math.vec` aceita zero filhos. A forma
com `delim: "[", align: left, gap: 1em` normaliza publicamente o delimitador
para `("[", "]")` e o gap para `0% + 1em`.

Essas strings são observáveis da linguagem e podem ser oráculo. Igualdade de
bytes internos, passos de algoritmo ou `PartialEq` Rust não é critério.

---

## 3. Escopo e não-escopo

### 3.1 Dentro do escopo

1. Os quatro membros e os seus contratos completos de chamada, erro, `repr`,
   conteúdo e layout.
2. A convergência entre a sintaxe matemática `vec` e `math.vec` no construtor
   canônico, sem degradar para matriz.
3. A resolução percentual de `math.vec.gap` contra a altura efetiva da região,
   inclusive região de altura infinita, conforme o L0 vigente em rascunho.
4. O efeito de `place.flush()` no ponto exato do fluxo: floats pendentes antes
   do marcador são colocados antes de o conteúdo posterior continuar; floats
   criados depois do marcador não são antecipados.
5. A remoção dos quatro warnings V7 correspondentes aos L0s de underline/vec
   pela criação dos consumers proprietários corretos.
6. Regressão positiva de todos os 95 membros que já eram `MATCH` no baseline.

### 3.2 Fora do escopo

1. Fazer as dez extensões cristalinas desaparecerem para imitar inventário
   vanilla.
2. Ligar `html` ou `pdf.data-cell` no perfil em que devem permanecer ocultos.
3. Resolver as 13 violações e 10 `Unknown` históricos do gate global P1287.
4. Tornar retroativamente independente uma certificação histórica cuja ordem
   causal já foi perdida. P1288 pode receber uma nova auditoria prospectiva de
   preservação, mas o registro histórico `NOT REFINED` não deve ser reescrito.
5. Igualdade pixel a pixel de PNG, nome interno de fonte PDF, bytes de PDF ou
   estrutura Rust.
6. Refatorar o mecanismo geral de namespaces de `Func`: ele já suporta
   `native_with_namespace` e `Func::With` já preserva o namespace.
7. Criar despacho dinâmico, remover o `match` exaustivo ou deslocar lógica de
   render para arquivos de entidade.

---

## 4. Obrigações formais

### O-A — `math.cancel`

O módulo `math` deve expor `cancel` como a função que constrói o
`MathCancelElem` já legitimado, e não como alias superficial do `cancel`
histórico nem como wrapper que descarte campos.

Devem ser preservados body, length, inverted, cross, angle, stroke,
background, span e a realização runtime de callback de ângulo certificada em
P1291. O `repr` público deve refletir defaults/explicitude conforme o vanilla.

### O-B — `math.underline`

`math.underline` deve ser uma função distinta da função textual global
`underline`. Ela constrói `MathUnderlineElem { body }`, exige body e usa uma
free function `compiler/math/layout/underline.rs::layout`.

O traço matemático deve derivar gap, espessura e descida extra das constantes
MATH da fonte. Não é permitido aliasar o elemento textual, assar valores em pt
ou mover lógica de layout para o arquivo da entidade.

### O-C — `math.vec`

`math.vec` recebe filhos variádicos e os named `delim`, `align` e `gap`, com os
defaults já descritos nos L0s em rascunho. Zero filhos é válido. Cada filho é
uma linha em estilo de denominador, alinhada dentro da largura comum, com
delimitadores esticados quando presentes.

O percentual de `gap` é resolvido contra `Regions::effective().height` recebido
do caller de equação. Altura infinita precisa de decisão explícita medida e
testada; não pode virar base de tamanho de fonte, grid, `em` ou zero por
conveniência.

A sintaxe matemática e a chamada `math.vec` devem convergir para a mesma forma
canônica da linguagem. Compartilhar construtor é permitido; duplicar semântica
em dois caminhos não é.

### O-D — `place.flush`

`place` continua função chamável e passa a carregar namespace com o membro
`flush`. `place.with(...).flush` deve continuar resolvendo para o mesmo membro,
como no vanilla.

`place.flush()` constrói um elemento/sentinela público de zero campos, com
`repr` `flush()`. O elemento deve ter owner próprio — por exemplo
`entities/elements/flush.rs` — e layout próprio na camada de render, em forma B
da ADR-0109. Ele não deve ser comprimido em `Content::Empty`, porque isso perde
o efeito de fase no fluxo.

Ao encontrar o marcador, o layouter esvazia somente o conjunto de floats já
pendentes, respeitando colocação top/bottom, clearance, paginação e ordem
vigentes. O marcador não desenha item, não avança o cursor por si, não duplica
nem descarta floats e não antecipa floats posteriores.

A implementação deve medir primeiro se a chamada direta a
`flush_pending_floats()` no dispatcher preserva esses invariantes em fluxo
principal, sub-frame e mudança de página. Se a medição refutar essa hipótese,
o L0 deve ser corrigido antes de escolher outro mecanismo; este passo não
autoriza aceitar um no-op.

---

## 5. Nucleação obrigatória e gate humano

### 5.1 Auditoria de ownership antes de redigir

O autor de contrato deve executar V15/V26 e construir uma matriz
Prompt L0 ↔ consumer. Não se cria prompt agregador para vários consumers e
nenhum consumer pode apontar para Núcleo Tekt.

Owners existentes a revisar, quando afetados:

- `00_nucleo/prompts/entities/elements/math_cancel.md`;
- `00_nucleo/prompts/compiler/math/layout/cancel.md`;
- `00_nucleo/prompts/entities/elements/math_underline.md`;
- `00_nucleo/prompts/compiler/math/layout/underline.md`;
- `00_nucleo/prompts/entities/elements/math_vec.md`;
- `00_nucleo/prompts/compiler/math/layout/vec.md`;
- `00_nucleo/prompts/compiler/stdlib/structural/math.md`;
- `00_nucleo/prompts/compiler/eval/math.md`;
- `00_nucleo/prompts/compiler/math/layout/_comum.md`;
- `00_nucleo/prompts/entities/content.md`;
- `00_nucleo/prompts/entities/elements/_comum.md`;
- `00_nucleo/prompts/compiler/eval/repr.md`, se a medição demonstrar delta de
  morfologia;
- `00_nucleo/prompts/compiler/stdlib/layout.md`;
- `00_nucleo/prompts/compiler/eval.md`;
- `00_nucleo/prompts/compiler/layout.md`.

Para `place.flush`, a forma recomendada — sujeita à auditoria 1:1 — é criar:

```text
00_nucleo/prompts/entities/elements/flush.md
  -> 01_core/src/entities/elements/flush.rs

00_nucleo/prompts/compiler/layout/flush.md
  -> 01_core/src/compiler/layout/flush.rs
```

O L0 e o consumer vigentes de `place` continuam donos somente de `PlaceElem` e
do seu layout. O construtor `native_flush` pertence ao owner de stdlib/layout;
o namespace anexado a `place` pertence ao owner do scope base em compiler/eval.

Só criar ou alterar Núcleo Tekt se houver uma invariante normativa realmente
compartilhada por dois ou mais prompts. Nesse caso, TOML 1.0, DAG, consumidores
e pins SHA-256 completos devem passar V26 antes do resselo.

### 5.2 Conteúdo mínimo dos L0s

Antes de testes ou código, os L0s precisam fixar:

1. assinatura e defaults de cada função;
2. presença versus explicitude de campos observáveis em `repr`;
3. validação de posicionais, named e tipos;
4. identidade distinta de `math.underline` e `underline`;
5. contrato de região de `math.vec.gap` e altura infinita;
6. ponto do pipeline em que `place.flush` atua;
7. forma B da atomização e dispatcher estático exaustivo;
8. mapeamento 1:1 dos novos consumers;
9. escopos futuros ainda não materializados, sem aceitar argumentos ignorados.

### 5.3 Paragem obrigatória

Depois de redigir/atualizar os L0s, gerar
`00_nucleo/diagnosticos/p1292-pre-gate-l0-receipt.md` com hashes e matriz de
ownership e **PARAR**.

O código só começa após confirmação humana explícita do novo gate P1292. Uma
confirmação usada por P1291 não é automaticamente transferida para o contrato
novo de `place.flush`.

---

## 6. Segregação de autoria e capacidades

Cada lote usa os papéis abaixo. A mesma pessoa/agente não pode ser, no mesmo
lote, autora do código produtivo e autora/adjudicadora dos ataques. O autor de
testes/oráculos trabalha a partir do L0 selado, do vanilla pinado e dos recibos
de medição, sem copiar decisões do patch candidato.

| Papel | Pode escrever | Não pode escrever/decidir |
|---|---|---|
| Medidor | recibos de baseline e medições | L0, código produtivo, veredito |
| Autor de contrato | L0 e recibo de contrato | código produtivo e ataques |
| Autor de testes/oráculos | testes, fixtures e oráculos do lote | código produtivo |
| Implementador | consumers produtivos e seus headers de linhagem | oráculos selados, veredito |
| Atacante | plano/recibo de mutações e worktree temporária | candidato integrado, contrato |
| Verificador | recibos reprodutíveis | código produtivo |
| Adjudicador | certificado final | contrato, implementação e ataques |

Antes da primeira escrita de cada papel, criar ou atualizar
`00_nucleo/diagnosticos/p1292-manifest.json` com:

- identidade do papel e lote;
- allowlist de caminhos;
- hash do L0/contrato recebido;
- hashes dos binários de referência;
- estado exato da árvore;
- declaração de artefatos que o papel pode ler;
- sequência temporal das entregas.

Escrita fora da allowlist invalida o sub-selo do lote. Edição concorrente de
`entities/content.rs`, `compiler/eval/mod.rs`, `structural/math.rs` ou dos
dispatchers de layout é proibida. Lotes podem preparar contratos e testes em
paralelo, mas a integração produtiva nesses arquivos compartilhados é serial:
A → B → C → D, com rebase e novo hash entre lotes.

---

## 7. Contrato selado e testes RED

### 7.1 Selo

Após a confirmação humana, o autor de contrato gera
`p1292-contract-seal.json`. O selo contém, por lote:

- hashes de todos os L0s;
- hashes das fontes vanilla medidas, com path e linha;
- vetor de casos positivos, negativos e limítrofes;
- forma de comparação no nível da linguagem;
- política de `Unknown`;
- allowlists e exclusões;
- hash do próprio contrato.

Qualquer mudança posterior em L0, oráculo ou caso normativo invalida o selo e
obriga resselo antes de continuar.

### 7.2 RED mínimo por lote

Os testes devem falhar no candidato inicial pela razão correta:

**A — cancel**

- presença/tipo/`repr` de `math.cancel`;
- defaults e todos os named legitimados;
- body obrigatório e tipos/named inválidos;
- callback de ângulo, inclusive erro e span;
- `repr` do conteúdo e equivalência de layout com a forma sintática aplicável.

**B — underline**

- presença/tipo/`repr`;
- body obrigatório;
- identidade distinta do underline textual;
- conteúdo canônico e layout derivado de constantes MATH em estilos
  display/text/script/cramped relevantes.

**C — vec**

- zero, um e vários filhos;
- defaults, delimitadores pareados, alinhamentos e gap absoluto/relativo;
- named/posicionais inválidos;
- convergência entre sintaxe e função;
- altura finita e infinita de região;
- `repr` multilinha quando os named explícitos o exigirem.

**D — flush**

- `place` continua função e `place.flush` é função;
- `place.with(...).flush` preserva namespace;
- `repr(place.flush()) == "flush()"`;
- zero argumentos somente;
- nenhum float pendente;
- top float, bottom float e mistura de ambos;
- conteúdo antes/depois do marcador;
- novo float depois do marcador;
- quebra de página, clearance e caso aninhado medido como suportado pelo L0.

O recibo `p1292-red-tests-receipt.md` precisa registrar comando, saída, códigos
de retorno, hashes e motivo semântico de cada falha. Teste que falha por erro de
compilação alheio ao contrato não satisfaz RED.

---

## 8. Implementação autorizada após RED

### 8.1 Lote A

Registrar o construtor canônico no namespace `math` somente depois de provar
que o caminho já materializado de cancel está completo. Atualizar headers e
hashes dos owners tocados. Não reimplementar callback já certificado nem criar
um segundo `MathCancelElem`.

### 8.2 Lote B

Criar consumer de entidade e consumer de layout próprios, integrar a variante
em `Content`, traversal, `repr`, módulos e dispatcher. O corpo do braço do
dispatcher deve delegar à free function de underline.

### 8.3 Lote C

Criar consumers próprios, integrar `Content`, traversal, `repr`, módulos,
dispatcher, stdlib e parser/eval matemático. Transportar a altura efetiva de
região até o layout sem assar um default de outra unidade.

### 8.4 Lote D

Criar o elemento zero-campo e o layout atomizado; adicionar a variante
distinguível em `Content`; criar `native_flush`; registrar `place` por
`Func::native_with_namespace` com `flush`; preservar o namespace por
`place.with`. Integrar o efeito no ponto do fluxo definido pelo L0.

Em todos os lotes:

- `Content` permanece enum fechado;
- o `match` de layout permanece exaustivo e estático;
- L1 não faz I/O nem usa estado global mutável;
- argumentos reconhecidos nunca são silenciosamente ignorados;
- novos arquivos recebem `@prompt` e `@prompt-hash` corretos;
- não tocar `lab/typst-original` para “corrigir” o oráculo.

---

## 9. Campanha adversarial obrigatória

O atacante recebe o contrato selado e o candidato, mas não altera a integração.
Cada mutante não equivalente deve ser morto por teste/oráculo. Mínimos:

### A — cancel

1. membro aponta para função histórica errada;
2. um named é aceito e descartado;
3. callback é tratado como ângulo fixo;
4. `repr` reduz o conteúdo a body;
5. erro de callback perde o span de chamada.

### B — underline

1. alias para underline textual;
2. gap fixo em pt;
3. espessura ignora constante MATH;
4. body ausente vira vazio;
5. braço do dispatcher não delega ao owner.

### C — vec

1. degrada para matriz;
2. zero filhos vira erro;
3. `align` ou `delim` é ignorado;
4. percentual de gap usa `em` ou largura da região;
5. altura infinita produz NaN/pânico;
6. sintaxe e função constroem formas diferentes.

### D — flush

1. membro existe, mas devolve `Content::Empty`;
2. marcador só é efetivo em `finish()`;
3. flush antecipa floats posteriores;
4. top/bottom são emitidos na ordem errada;
5. marcador duplica ou descarta float;
6. `place.with(...).flush` perde namespace;
7. argumentos extras são ignorados.

Meta: `mutation score = 1.0` em cada lote e no agregado. Mutante equivalente
só sai do denominador mediante justificativa do atacante e aceitação do
adjudicador antes do cálculo. Denominador zero, mutante não executado, timeout,
erro de harness ou resultado ambíguo é `Unknown`, nunca “morto”. Qualquer
`Unknown` ou mutante sobrevivente bloqueia o lote.

---

## 10. Verificação integrada

Depois dos quatro lotes e sem alterar os artefatos selados:

1. rebuild limpo do binário release;
2. recaptura de `HEAD`, hora, `git status --short`, `git diff HEAD --stat` e
   hashes dos binários;
3. execução das sondas de superfície nos perfis default e HTML;
4. execução dos oráculos funcionais de A–D;
5. regressão prospectiva dos oráculos P1288, P1289, P1290 e do runtime P1291;
6. execução dos gates arquiteturais e da suíte completa.

Comandos mínimos:

```text
cargo build --release
cargo test --workspace -q
cargo fmt --all -- --check
crystalline-lint .
crystalline-lint --quiet --fail-on-warning=V5 .
crystalline-lint --quiet --fail-on-warning=V7 .
crystalline-lint --quiet --fail-on-warning=V15 .
crystalline-lint --quiet --fail-on-warning=V26 .
git diff --check
```

As sondas padrão devem terminar, no perfil default, com:

```text
111 total
99 MATCH
12 DIFFERENCE_OR_DISABLED já justificadas
0 dos 95 MATCH anteriores regredidos
0 membros acionáveis desta lista ausentes
```

Se o inventário mudar legitimamente durante o passo, não ajustar o denominador
à mão para obter 99. Regerar os dois lados, documentar o delta e adjudicar pela
obrigação nominal dos quatro membros mais a preservação do baseline.

V7 deve deixar de apontar como órfãos exatamente os quatro prompts de
underline/vec. V5/V15/V26 devem ficar em zero. Um warning ou `Unknown` não é
aceitável como fecho parcial disfarçado.

---

## 11. Artefatos de evidência

Produzir em `00_nucleo/diagnosticos/`:

```text
p1292-manifest.json
p1292-baseline-status.txt
p1292-vanilla-measurement-receipt.md
p1292-pre-gate-l0-receipt.md
p1292-contract-seal.json
p1292-contract-receipt.md
p1292-red-tests-receipt.md
p1292-implementation-receipt-a.md
p1292-implementation-receipt-b.md
p1292-implementation-receipt-c.md
p1292-implementation-receipt-d.md
p1292-adversarial-plan.md
p1292-adversarial-receipt.json
p1292-surface-default.json
p1292-surface-html.json
p1292-preservation-receipt.md
p1292-verification-receipt.md
p1292-final-certificate.json
typst-passo-1292-relatorio.md
```

Cada recibo numérico registra o estado exato da árvore que o produziu. O
certificado final referencia todos os hashes; não copia apenas conclusões.

---

## 12. Critério de veredito

O adjudicador pode emitir `APPROVED` para um lote individual, mas
`whole_step_closed: true` somente quando:

1. A, B, C e D estiverem `APPROVED`;
2. o gate humano pós-L0 estiver registrado;
3. RED preceder a implementação de cada lote;
4. todos os mutantes não equivalentes estiverem mortos;
5. não houver `Unknown`;
6. as quatro superfícies e seus comportamentos funcionais estiverem
   preservados nos perfis aplicáveis;
7. os 95 `MATCH` anteriores não tiverem regressão;
8. build, testes, fmt, lint, V5, V7, V15, V26 e diff-check estiverem verdes;
9. os hashes de consumers e L0s coincidirem com o estado verificado.

Falha de qualquer item produz `NOT APPROVED` para o passo inteiro, com o lote e
a obrigação exata ainda abertos. Não usar “maioria”, presença superficial,
ausência de crash ou resultado visual plausível como substituto.

---

## 13. Condições de paragem imediata

Parar e devolver ao dono se ocorrer qualquer um destes casos:

- o baseline fresco não reproduzir os quatro resíduos;
- houver conflito entre L0 vigente e a fonte vanilla pinada;
- a solução exigir mudar contrato público além dos campos já propostos;
- a altura infinita de `math.vec` não tiver semântica medida suficiente;
- `place.flush` exigir mudança de fase distinta da descrita no L0;
- V15/V26 revelar ownership inválido;
- um teste só puder ser tornado verde relaxando o oráculo;
- um mutante sobreviver ou ficar `Unknown`;
- outro trabalho modificar L0, consumer, harness ou binário durante um selo.

O passo fecha resíduos reais da superfície padrão; não autoriza alegar paridade
global do compilador.
