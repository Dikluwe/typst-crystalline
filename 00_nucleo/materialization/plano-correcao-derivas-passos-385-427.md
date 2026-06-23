# Plano de correção das derivas — passos 385–427

Documento de remediação. Para cada problema identificado em
`problemas-passos-385-427.md`, define uma correção concreta: tipo,
ação, onde aplicar, e critério de fecho.

Cada entrada é autossuficiente. Pode ser elevada a um passo de
correção próprio (ex.: "P4XX — correção P393") ou agrupada num único
passo administrativo, conforme a decisão de quem executa.

---

## Nota de escopo

Este plano foi escrito sem acesso ao repositório. Por isso:

- Correções de tipo **documental** trazem o texto a inserir ou a regra
  a aplicar. Podem ser executadas a partir deste documento.
- Correções de tipo **medição** e **artefacto ausente** trazem o
  comando exato a executar. O resultado tem de ser obtido no
  repositório antes de a correção fechar.
- Correções de tipo **processo** referem-se a erros de ordem já
  consumados. Não se desfazem; a correção é anotar o que ocorreu e
  capturar a lição numa ADR.

**Verificação prévia obrigatória (DEBT-52):** o `00_nucleo/DEBT.md`
regista DEBT-52 como encerrado em P142 (rastreador de consumer de
`StyleDelta` em layout). O documento de problemas associa P407/P414 a
"DEBT-52" com forma dict de strong/emph. Antes de editar qualquer
coisa em P407/P414, confirmar o número real da DEBT. Há três
possibilidades: (a) o documento de problemas usou o número errado;
(b) DEBT-52 foi reaberto sem registo no histórico do `DEBT.md` —
nesse caso a ausência de registo é uma deriva adicional a corrigir;
(c) o número 52 foi reutilizado para outro assunto. A correção de
P407/P414 abaixo assume que esta verificação foi feita primeiro.

---

## Tabela-resumo

| Problema | Tipo | Precisa do código? | Artefacto afetado |
|----------|------|--------------------|-------------------|
| P393 | documental + decisão | não (decisão), sim (verificar comportamento) | relatório P393; inventário; possível ADR scope-out ou DEBT |
| P394 | documental | sim (grep da assinatura) | relatório P394 |
| P395 | medição | sim (grep `enum Fill`) | spec P395 §2.3 e §7 |
| P389 | artefacto ausente | sim (re-rodar sonda) | `00_nucleo/diagnosticos/typst-sonda-ausentes-ordem-passo-389.md` |
| P388 | processo + medição | sim (re-rodar sonda viabilidade) | spec P388 §2 e §4; ADR metodológica |
| P403/P405 | documental | não | relatório P403; L0 `primitives-constructors.md` |
| P407/P414 | documental + DEBT | sim (verificar número) | `00_nucleo/DEBT.md`; relatórios P407 e P414 |
| P409/P413 | processo + documental | não | specs P409 e P413; ADR metodológica |
| P416 | processo + documental | não | spec P416; ADR metodológica |
| P420 | documental | sim (verificar `PartialEq`/`Hash`) | relatório P420; possível DEBT |
| P421 | processo | não | relatório P421; ADR metodológica |
| P423 | medição | sim (grep tokens `\|` `&`) | spec P423 §A.1.3 |
| P424 | documental + teste | sim (adicionar teste) | relatório P424; `export/tests.rs` |
| P427 | processo + documental | não | spec P427 §5; checklist de spec |
| Padrão geral | metodológico | não | ADR-0094 (anotação) ou ADR nova |

---

## P393 — tabela de paridade do `#show regex(...)` declara resultado errado

**Tipo:** documental + decisão arquitetural.

**Situação:** a tabela de paridade do relatório P393 afirma que
`"abc123def"` com `#show regex('\d+'): it => strong(it)` produz strong
nos dígitos. O comportamento real do cristalino transforma o nó de
texto inteiro: produz `strong("abc123def")`. A tabela está, portanto,
a declarar paridade que não existe.

**Decisão exigida primeiro:** o comportamento "nó inteiro" é (a) um
scope-out aceite ou (b) um defeito. A tabela errada esconde esta
pergunta. É preciso respondê-la antes de corrigir o texto.

- Se (a) scope-out aceite: registar a divergência numa ADR de
  scope-out (paridade ADR-0054 graded) ou numa DEBT, e reclassificar o
  item no inventário de `implementado` para `parcial`.
- Se (b) defeito: abrir DEBT com o objetivo de fazer o regex aplicar
  só ao trecho casado, e reclassificar o item para `parcial` até lá.

**Ação documental (independente de a/b):** na tabela de paridade do
relatório P393, substituir a linha enganosa por uma linha que mostra o
resultado real do cristalino. Texto a inserir:

```
| Entrada       | Regra                                  | Vanilla esperado            | Cristalino real            | Estado  |
|---------------|----------------------------------------|-----------------------------|----------------------------|---------|
| "abc123def"   | #show regex('\d+'): it => strong(it)   | "abc" + strong("123") + "def" | strong("abc123def")      | parcial |
```

**Onde:** relatório P393, tabela de paridade (primeira linha);
inventário de cobertura do passo.

**Critério de fecho:** a tabela mostra a divergência; o item está
marcado `parcial` no inventário; existe uma ADR de scope-out ou uma
DEBT que regista a decisão (a) ou (b).

---

## P394 — relatório não diz se `apply_func` mudou de assinatura

**Tipo:** documental (resolução de ambiguidade por verificação).

**Situação:** o relatório P394 afirma duas coisas que não se
reconciliam: que `apply_func` passou a receber `&mut Scopes<'_>` (logo,
a assinatura mudou e propagou-se a todos os callers) e que a intrusão
foi "mecânica". Se a assinatura mudou, callers que não precisam de
scopes passam um scope vazio — isso é um custo real que o relatório não
declara.

**Ação:** verificar a assinatura real e os callers no código, depois
reescrever a secção "Decisão de engenharia" sem ambiguidade. Comando
de verificação:

```bash
rg -n "fn apply_func" 01_core/src/
rg -n "apply_func\(" 01_core/src/ | wc -l
```

O resultado decide a redação:

- Se a assinatura mudou: declarar literal "a assinatura de `apply_func`
  passou a `&mut Scopes<'_>`; N callers foram adaptados; M desses
  callers não usam scopes e passam um scope vazio (custo aceite:
  parâmetro inerte nesses caminhos)". Substituir N e M pelos números
  reais.
- Se a assinatura não mudou (dispatch ficou interno à função):
  declarar literal "a assinatura de `apply_func` foi preservada; o
  dispatch para o caminho que usa scopes é interno à função; nenhum
  caller foi alterado".

**Onde:** relatório P394, secções "Decisão de engenharia" e "Protocolo
de Nucleação cumprido".

**Critério de fecho:** o relatório afirma exatamente um dos dois
caminhos, com os números reais de callers quando aplicável, e a
afirmação "mecânica" só permanece se for o segundo caminho.

---

## P395 — `Fill::Tiling` assumido sem confirmar que `Fill` é enum

**Tipo:** medição (sonda de substrato em falta).

**Situação:** a spec P395 propõe adicionar `Fill::Tiling(Tiling)` ao
enum `Fill` e lista como risco "se `Fill` não existe isolado, criar
agora". O risco foi escrito mas não foi medido. Se `Fill` não for um
enum separado (por exemplo, se o código usa `Option<Color>` direto), o
passo deixa de ser S e torna-se um refactor de layout não previsto no
custo.

**Ação:** rodar a sonda de substrato que devia ter precedido a spec,
com o mesmo método que a sonda P389 usa para os outros substratos.
Comando:

```bash
rg -n "enum Fill" 01_core/src/entities/
rg -n "\bFill\b" 01_core/src/entities/layout_types.rs
```

Registar o `file:line` do `enum Fill` na spec, secção 2.3. Se o `enum`
não existir, reclassificar o passo (S → M ou maior) e reescrever a
estimativa de custo antes de materializar.

**Onde:** spec P395, secção 2.3 (tabela de impacto cross-module) e
secção 7 (primeiro risco).

**Critério de fecho:** a spec contém o `file:line` que confirma o
substrato, ou contém a reclassificação de custo se o substrato não
existir como assumido.

**Nota:** o `Value::Tiling` de P395 já está materializado (referido em
ADR-0112 como precedente do `Value::Decimal`). A correção aqui é
documental e de método: a sonda devia ter existido antes da spec. Se o
trabalho já está feito, a correção é registar retroativamente o
`file:line` do substrato real que foi usado.

---

## P389 — ficheiro de output da sonda não está presente

**Tipo:** artefacto ausente.

**Situação:** P389 descreve que produz
`typst-sonda-ausentes-ordem-passo-389.md`. Os passos P390–P394 citam o
§2D desse ficheiro como fonte de autoridade. O ficheiro não está nos
materiais. As âncoras `file:line` que P390–P394 citam não são
verificáveis sem ele.

**Ação:** localizar o ficheiro em
`00_nucleo/diagnosticos/`. Se existir, está apenas fora dos materiais
fornecidos — referenciar o caminho nos passos que o citam. Se não
existir, reconstruí-lo rodando a sonda de novo e gravando o output como
ficheiro imutável (paridade ADR-0085, diagnóstico imutável).

```bash
ls 00_nucleo/diagnosticos/ | rg "passo-389"
```

**Onde:** passos P390–P394, campo "Sonda fonte" de cada cabeçalho.

**Critério de fecho:** o ficheiro existe em `00_nucleo/diagnosticos/`,
e cada `file:line` citado por P390–P394 corresponde a uma linha
presente nele.

---

## P388 — spec completa escrita antes de a sonda responder

**Tipo:** processo (consumado) + medição (reconciliação).

**Situação:** P388 diz para abrir com sonda de viabilidade do runtime
de introspecção antes de qualquer código. A spec completa (arquitetura,
fases, critérios) foi escrita antes de a sonda responder. A Fase 1
(autor-data + bibliografia alfabética) foi fixada sem ler o runtime.
Isto inverte a ordem que ADR-0084 e ADR-0065 exigem.

**Ação (consumado):** não se desfaz a ordem. A correção factual é
verificar agora se o escopo da Fase 1 corresponde ao que o runtime
suporta de verdade. Rodar a sonda de viabilidade que devia ter vindo
primeiro:

```bash
rg -n "fn layout_with_introspector|World|introspect" 01_core/src/
```

Se a sonda revelar que o runtime não suporta coleta cross-document, a
Fase 1 encolhe para autor-data puro, e a spec é reconciliada com o
resultado. Se suportar, registar que a coincidência foi confirmada a
posteriori.

**Ação (lição):** entra no padrão geral no fim deste documento (ADR de
sonda-antes-de-spec como gate).

**Onde:** spec P388, secções 2 ("Sonda de viabilidade") e 4
("Faseamento").

**Critério de fecho:** o escopo da Fase 1 está confirmado contra o
runtime real, com o output da sonda registado; qualquer divergência foi
aplicada à spec.

---

## P403 e P405 — `duration()` dividido em dois passos com L0 desatualizado

**Tipo:** documental.

**Situação:** P403 criou `duration("1h30m")` (só string posicional).
P405 adicionou a forma com named args. Entre os dois passos, o L0
`primitives-constructors.md` descrevia só a forma string. Quem lesse o
L0 depois de P403 e antes de P405 tinha uma descrição incompleta.

**Ação:** isto é passado e não se reescreve a história. A correção é
inserir uma nota retroativa no relatório P403 e adicionar uma regra
para o futuro.

- Nota no relatório P403, secção "Decisão de engenharia". Texto a
  inserir:

```
Nota retroativa: este passo entregou apenas a forma string de
duration(). A forma named args (duration(seconds:, hours:, ...)) foi
adicionada em P405. Entre P403 e P405 o L0 primitives-constructors.md
descreveu apenas a forma string. Um leitor do L0 nesse intervalo tinha
uma descrição incompleta do constructor.
```

- Regra para o futuro (entra como cláusula numa ADR de método, ou no
  CLAUDE.md): quando um constructor é dividido entre passos, o L0 do
  passo inicial declara explicitamente que está incompleto e nomeia o
  passo que completa.

**Onde:** relatório P403 ("Decisão de engenharia"); L0
`primitives-constructors.md` (já atualizado em P405 — confirmar que o
hash foi propagado com `--fix-hashes`).

**Critério de fecho:** o relatório P403 declara a divisão; existe uma
regra registada para divisões futuras de constructor.

---

## P407 e P414 — critério de fecho de DEBT-52 revisado sem declarar

**Tipo:** documental + DEBT. **Depende da verificação de número acima.**

**Situação:** P407 fechou "DEBT-52" com a forma dict legada. P414 diz
que a forma named fields ainda faltava. Ou o critério de fecho foi
amplo demais (fechou antes da paridade completa), ou foi revisado
depois sem declaração.

**Ação:** primeiro resolver a verificação de número (ver "Nota de
escopo"). Depois:

- Adicionar uma entrada no histórico do `00_nucleo/DEBT.md` que declara
  o que o fecho de P407 cobriu e o que ficou de fora. Texto base (ajustar
  o número da DEBT após verificação):

```
Passo 407: fecho de DEBT-XX cobrindo a forma dict legada
("Name": ("Bold")). A forma named fields do vanilla ficou fora do
escopo deste fecho. P414 clarificou a lacuna.
Decisão: [escolher uma] (i) abrir DEBT nova para named fields;
(ii) marcar como scope-out graded ADR-0054 com revisão futura.
```

- No relatório P407, na secção "Inventário / DEBT", declarar
  explicitamente que a paridade named fields estava fora do escopo do
  fecho. Não deixar o fecho registado como paridade completa.

**Onde:** `00_nucleo/DEBT.md` (histórico); relatório P407 ("Inventário
/ DEBT"); relatório P414 ("Inventário / DEBT") — confirmar que P414 já
aponta para a lacuna.

**Critério de fecho:** o histórico do `DEBT.md` regista o escopo real
do fecho de P407 e a decisão sobre named fields; o número da DEBT está
confirmado.

---

## P409 e P413 — specs escritas para trabalho já feito

**Tipo:** processo (consumado) + documental.

**Situação:** P409 (aritmética de `Duration`) e P413 (aritmética de
`Decimal`) foram escritas como specs de materialização. A sonda A.0
revelou, nos dois casos, que o código já existia (P405 e P404). As
specs foram escritas sem confirmar o estado.

**Ação:** as specs não se apagam. A correção é convertê-las, no
cabeçalho, de "spec de materialização" para "verificação retroativa", e
registar o que a sonda A.0 encontrou. Texto a inserir no topo de cada
spec:

```
Reclassificação retroativa: a sonda A.0 confirmou que o trabalho
descrito já existia (PXXX). Esta spec passa a documento de verificação
retroativa, não de materialização. Nenhum código novo foi produzido por
este passo além do que a verificação exigiu.
```

(Substituir PXXX por P405 em P409 e por P404 em P413.)

**Onde:** specs P409 e P413, secção de contexto.

**Critério de fecho:** ambas as specs declaram no topo que são
verificação retroativa, com referência ao passo que fez o trabalho.

**Lição:** entra no padrão geral (sonda A.0 antes da spec).

---

## P416 — spec escrita para trabalho já feito (mesmo padrão)

**Tipo:** processo (consumado) + documental.

**Situação:** P416 (footnote body no rodapé) foi escrita como spec de
materialização. A sonda revelou que P304/P305 já tinham implementado,
com 11 testes. O relatório diz que a verificação aconteceu, mas a spec
existe como documento completo de materialização, o que sugere que foi
escrita antes da sonda.

**Ação:** mesma correção de P409/P413. Inserir no topo da spec P416:

```
Reclassificação retroativa: a sonda confirmou que footnote body no
rodapé já existia (P304/P305, 11 testes). Esta spec passa a documento de
verificação retroativa.
```

**Onde:** spec P416 (topo); relatório P416 ("Resumo executivo").

**Critério de fecho:** a spec declara que é verificação retroativa, com
referência a P304/P305.

---

## P420 — campo de cache em struct de dados de domínio

**Tipo:** documental (com verificação de código).

**Situação:** P420 adicionou `resolved_style: Option<Arc<IndependentStyle>>`
a `BibliographyElem`. Um struct de domínio passou a guardar estado
computado. O relatório documenta a divergência face ao L0 ("struct
inalterado"), mas não diz como `resolved_style` se comporta em
`PartialEq` e `Hash`. `BibliographyElem` participa dessas comparações;
o campo tem de estar explicitamente excluído ou incluído.

**Ação:** verificar no código como `PartialEq` e `Hash` tratam o campo.
Comando:

```bash
rg -n "BibliographyElem" 01_core/src/ -A3 | rg -n "PartialEq|Hash|derive|resolved_style"
```

Depois declarar o comportamento literal no relatório P420, secção 6.
Uma das duas redações:

- Excluído: "`resolved_style` é excluído de `PartialEq` e `Hash`. Dois
  `BibliographyElem` com o mesmo path mas styles resolvidos diferentes
  são considerados iguais."
- Incluído: "`resolved_style` participa de `PartialEq` e `Hash`. Dois
  `BibliographyElem` com o mesmo path mas styles resolvidos diferentes
  são considerados diferentes."

Avaliar se o smell (cache em struct de domínio) justifica uma DEBT que
registe o custo futuro de manter este campo consistente.

**Onde:** relatório P420, secção 6 ("Notas epistêmicas").

**Critério de fecho:** o relatório declara o comportamento de
`resolved_style` em `PartialEq` e `Hash` sem ambiguidade; existe uma
DEBT se o smell foi julgado relevante.

---

## P421 — reclassificação S→M não foi antecipada pela sonda

**Tipo:** processo (consumado).

**Situação:** P421 foi planeado como S; a sonda A.0 revelou que
`native_repr` não existia e forçou reclassificação para M. A sonda
previu a situação (o critério de passagem (1) era confirmar que
`native_repr` existe). O problema não é a reclassificação — é que a
sonda obrigatória correu depois de o passo já estar marcado S.

**Ação:** consumado; não há texto a corrigir no relatório além de
confirmar que a reclassificação está registada com a razão. A correção
real é metodológica e entra no padrão geral: a sonda corre antes de a
spec fixar a classificação de tamanho.

**Onde:** relatório P421, secção 1 ("Sonda do substrato").

**Critério de fecho:** a reclassificação está registada com a razão; a
lição está na ADR do padrão geral.

---

## P423 — scope-out de `|`/`&` infixo sem custo medido

**Tipo:** medição.

**Situação:** a spec P423 escolhe a opção β (métodos `.or()`/`.and()`)
em vez de operadores infixos `|`/`&`, com a justificativa de que o
parser é mais caro (M). O custo M é uma estimativa, não uma medição. A
sonda A.0 verificou que o parser não suportava `|`/`&`, mas não mediu o
custo de adicioná-los.

**Ação:** estender a sonda para medir o custo real de adicionar tokens
`|` e `&` ao lexer. Comandos:

```bash
rg -n "Token::|fn lex|lexer" 02_*/src/ | rg -n "Pipe|Amp|\||&"
rg -n "[^|]\|[^|]|[^&]&[^&]" 02_*/src/parser* | wc -l
```

Medir: quantos ficheiros do parser seriam afetados; se há conflito com
usos existentes de `|` e `&`. Registar a medição na spec, secção
A.1.3, e só então fixar a decisão de opção β como baseada em facto.

**Onde:** spec P423, secção A.1.3 ("Estratégia de implementação").

**Critério de fecho:** a spec contém a medição do custo do caminho
infixo (ficheiros afetados + conflitos), e a escolha de β refere essa
medição em vez de uma estimativa.

---

## P424 — bbox aproximada para `Group` não documentada como limitação

**Tipo:** documental + teste.

**Situação:** P424 calcula a bbox de `FrameItem::Link` para emitir a
annotation URI no PDF. Para links que contêm `Group` (ex.: link com
texto em bold, que gera `FrameItem::Group` interno), o cálculo usa
`inner_width`/`inner_height` sem aplicar a transform do Group. O
relatório chama isto de "bbox aproximada" nos scope-outs, mas a
limitação não está nos critérios de aceitação nem nos testes. Os testes
E2E só cobrem links com texto simples.

**Ação:** adicionar um teste que regista o comportamento atual para
links com `Group` interno. O objetivo do teste não é forçar correção —
é registar o comportamento para que qualquer mudança futura seja
visível. Adicionar também a limitação aos critérios de aceitação como
scope-out explícito.

Esboço do teste (a colocar em `export/tests.rs`):

```rust
#[test]
fn p424_link_com_group_interno_bbox_aproximada() {
    // Link cujo conteúdo produz FrameItem::Group (ex.: texto bold).
    // Documenta que a bbox emitida usa inner_width/inner_height sem
    // aplicar a transform do Group. Asserção sobre o rect atual da
    // annotation, não sobre o rect "correto".
    // ... montar frame com Link > Group > Text ...
    // assert_eq!(annotation_rect, RECT_APROXIMADO_ATUAL);
}
```

**Onde:** relatório P424 ("Scope-out / bloqueadores"); `export/tests.rs`.

**Critério de fecho:** existe um teste que fixa o comportamento atual
para link com `Group` interno; a limitação está nos critérios de
aceitação como scope-out declarado.

---

## P427 — spec propõe arquitetura que contradiz o L0 vigente

**Tipo:** processo (consumado) + documental (regra).

**Situação:** a spec P427 propõe criar `shape_emit.rs` como módulo
separado (opção β). O L0 vigente de `stream.md` (hash `9acca994`) diz
para não subdividir. O Kimi seguiu o L0 e não criou o módulo — o
resultado foi correto. Mas a spec propôs uma arquitetura em conflito
com uma decisão já registada.

**Ação:** o resultado está correto, então não há código a corrigir. A
correção é registar, no relatório P427, que a opção β da spec estava em
conflito com o L0 vigente e foi por isso descartada. E adicionar uma
regra ao checklist de escrita de spec.

- Nota no relatório P427, secção "Decisão arquitetural". Texto:

```
Nota: a opção β da spec (criar shape_emit.rs como módulo separado)
estava em conflito com o L0 vigente de stream.md (hash 9acca994), que
determina não subdividir. A opção foi descartada por esse motivo. A spec
não devia ter proposto a subdivisão como opção preferida.
```

- Regra para o checklist de spec: antes de propor arquitetura, ler o L0
  vigente dos módulos afetados e confirmar o hash. Se o L0 já decide a
  questão, a spec não propõe o contrário como preferido.

**Onde:** spec P427, secção 5; relatório P427, secção "Decisão
arquitetural".

**Critério de fecho:** o relatório regista o conflito e o motivo do
descarte; a regra de "ler L0 vigente antes de propor arquitetura" está
no checklist de spec.

---

## Correção do padrão geral — specs escritas antes das sondas

**Tipo:** metodológico.

**Situação:** em P388, P409, P413, P416 e P421, a spec foi escrita
antes de a sonda de substrato correr. Em todos, a sonda revelou que a
situação real era diferente da assumida: trabalho já feito (P409, P413,
P416), infraestrutura ausente (P388), reclassificação necessária
(P421). O protocolo já exige sonda antes de spec (Protocolo de
Nucleação; ADR-0065 inventariar primeiro; ADR-0084 Fase A antes de
decisão). A regra existe mas não funcionou como gate.

**Ação:** formalizar "sonda A.0 antes da spec" como gate duro. Há duas
formas; escolher uma:

- Anotação cumulativa em ADR-0094 (meta-operacional de specs). Adicionar
  um padrão: "Sonda A.0 é pré-condição da spec. A spec não é redigida
  antes de o output da sonda existir como ficheiro. N=5 cumulativo:
  P388, P409, P413, P416, P421." Esta forma segue o padrão de anotação
  cumulativa que o projeto já usa e não cria ADR nova.
- ADR nova, se a decisão for que o gate merece documento próprio para
  citação precisa.

Em qualquer das formas, o gate operacional é concreto: o ficheiro de
output da sonda (em `00_nucleo/diagnosticos/`) tem de existir antes de a
spec ser escrita. Se a spec for escrita primeiro, é tratada como
verificação retroativa (como em P409/P413/P416), não como
materialização.

**Onde:** ADR-0094 (anotação) ou ADR nova; CLAUDE.md, secção do
Protocolo de Nucleação, se quiser reforço no guia operacional.

**Critério de fecho:** existe uma regra registada que torna o output da
sonda pré-condição da spec, com os 5 casos listados como base empírica.

---

## Ordem sugerida de execução

A ordem abaixo agrupa por dependência e por custo. Não é obrigatória.

1. **Verificação de número DEBT-52** (bloqueia P407/P414). Custo: XS.
2. **Medições** (P395, P423, P420, P394, P389): exigem o repositório;
   rodar os comandos e registar resultados. Custo: S no total.
3. **Decisão de P393** (scope-out ou defeito) seguida da correção
   documental. Custo: S.
4. **Correções documentais retroativas** (P403/P405, P407/P414, P409,
   P413, P416, P427): texto já definido neste plano. Custo: S no total.
5. **Teste de P424**: escrever o teste que fixa o comportamento atual.
   Custo: S.
6. **Reconciliação de P388**: rodar a sonda de viabilidade e ajustar o
   escopo da Fase 1 se preciso. Custo: S–M conforme o resultado.
7. **Correção metodológica do padrão geral**: anotar ADR-0094 ou criar
   ADR nova. Custo: XS. Fazer por último, com os 5 casos já corrigidos
   como base empírica citável.
