# Passo 84.8e — ADR-0033 (paridade vanilla) e ADR-0034 (diagnóstico de tipos vanilla)

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0032-unsafe-em-l1.md` — ADR mais recente,
  referência de formato.
- `00_nucleo/adr/typst-adr-0029-pureza-fisica-revoga-adr-0028.md` —
  ADR expandido no 84.8c (lista de 20 tipos tipográficos). É citado
  por ambos os novos ADRs.
- `00_nucleo/adr/typst-adr-0030-performance-dominio-l1.md` — ADR
  expandido no 84.8c (secção "Clone profundo vs Arc::clone"). É
  citado por ADR-0033.
- `00_nucleo/adr/typst-adr-0026-content-divergencia.md` e
  `typst-adr-0026-revisao-content-arc.md` — ambos citados por
  ADR-0033 como exemplo de divergência estrutural.
- `00_nucleo/diagnosticos/` — directório criado no 84.8d. É citado
  por ADR-0034 como destino canónico dos diagnósticos.
- `00_nucleo/relatorios/relatorio-auditoria-adrs-passo-84.7.md` —
  Secções 6.3 e 6.4 (motivação dos dois ADRs).

Pré-condição: `cargo test` — 911 testes, zero violations. P84.8d
concluído, 3 diagnósticos movidos para `00_nucleo/diagnosticos/`.

---

## Natureza deste passo

**Passo de criação de 2 ADRs novos.** Formaliza duas regras
implícitas identificadas no P84.7: paridade funcional vanilla
(Secção 6.3) e diagnóstico obrigatório para materialização de
tipos vanilla (Secção 6.4).

Nenhum ADR existente alterado. Nenhum DEBT novo. Nenhum código
tocado.

Dois produtos:

1. `00_nucleo/adr/typst-adr-0033-paridade-funcional-vanilla.md`.
2. `00_nucleo/adr/typst-adr-0034-diagnostico-tipos-vanilla.md`.

Contagem de ADRs: 33 → 35.

---

## Decisões confirmadas

- **Status**: ambos `ACCEPTED` (segue o padrão dos ADRs recentes
  0029-0032). Uniformização de vocabulário fica para o 84.8f.
- **ADR-0033 cita 3 exemplos concretos**: Align2D vs Alignment,
  Content enum vs vtable, `Arc<[ShowRule]>` vs `Vec<ShowRule>`.
- **ADR-0034 é exigência vinculativa** (não sugestão). Qualquer
  passo futuro que materialize tipo do vanilla **deve** produzir
  diagnóstico em `00_nucleo/diagnosticos/` antes.

---

## Tarefa 1 — Criar ADR-0033

Criar `00_nucleo/adr/typst-adr-0033-paridade-funcional-vanilla.md`
com o conteúdo abaixo.

```markdown
# ⚖️ ADR-0033: Paridade funcional com vanilla como invariante arquitectural

**Status**: `ACCEPTED`
**Data**: 2026-04-XX

---

## Contexto

A Arquitectura Cristalina é refactor do Typst vanilla com três
objectivos declarados: pureza física (ADR-0029), performance como
domínio de L1 (ADR-0030), e manutenção por LLMs (atomização,
ADRs, linter). Durante a série de passos 84.x — particularmente
84.5 (Alignment/Align2D) e 84.6 (Place/PlacementScope) — apareceu
implicitamente uma regra não escrita: **o comportamento observável
do Typst cristalino deve corresponder ao do vanilla, mesmo quando
a forma interna diverge**.

O Passo 84.5 é o caso paradigma: ao materializar `Alignment`, a
escolha foi `Align2D` (struct com dois campos) em vez de `Alignment`
(enum com dezenas de variantes). A forma diverge; a semântica de
`center + bottom` (combinação de alignments) é idêntica. Durante
o diagnóstico do passo, a decisão foi deliberada: preservar
semântica, divergir na forma quando há ganho arquitectural.

O Passo 84.7 (relatório de auditoria) identificou esta regra como
Input implícito e propôs formalização em ADR dedicado. Este é esse
ADR.

---

## Decisão

### Regra

**Para qualquer input, o output observável do Typst cristalino é
idêntico ao output do Typst vanilla.**

"Output observável" inclui:
- Resultado visual final (PDF, PNG, SVG) — bytes diferentes são
  aceitáveis apenas se a diferença for invisível (ex: ordenação
  interna de dicionários, comentários).
- Mensagens de erro — texto pode ser reescrito com mais clareza,
  mas **sentido** e **localização no código-fonte** devem ser
  idênticos.
- Ordem de avaliação observável pelo utilizador (side effects,
  outputs de `#let`, etc.).

### Divergências permitidas

Internas (não observáveis ao utilizador):
- **Forma estrutural**: struct vs enum, `Vec` vs `Arc<[T]>`,
  hierarquia vtable vs enum tagged.
- **Representação de tipos**: `Align2D { x, y }` em vez de enum
  com 9+ variantes para todas as combinações de alinhamento.
- **Optimizações de performance**: caching de sub-frames, hashing
  preventivo, partilha via `Arc`.
- **Organização de módulos**: ficheiros divididos em L0-L4, etc.

### Divergências proibidas

Observáveis ao utilizador:
- **Semântica de operadores** sobre tipos vanilla. Exemplo:
  `center + bottom` retorna `Align2D { x: Center, y: Bottom }`,
  combinando os dois alinhamentos — não retorna erro "can't add
  alignments". O operador `+` sobre `Alignment` vanilla tem esta
  semântica; cristalino preserva.
- **Mensagens de erro com sentido diferente**. Um erro de
  "expected content, found int" do vanilla não pode virar "type
  mismatch" no cristalino.
- **Regras de combinação**. Por exemplo, show rules aplicam-se por
  ordem de declaração no vanilla; cristalino preserva mesmo que
  a estrutura de dados seja diferente (`Arc<[ShowRule]>` em vez de
  `Vec<ShowRule>`).
- **Ordem visível** de operações. `#let x = { a(); b() }` avalia
  `a` antes de `b` no vanilla; cristalino preserva.

---

## Exemplos de aplicação (com divergências reais)

### Exemplo 1 — Passo 84.5 / ADR-0029: Alignment → Align2D

**Vanilla**: enum `Alignment` com ~18 variantes mais aliases
(`Center`, `Left`, `Right`, `Top`, `Bottom`, `Horizon`, combinações
especializadas, etc.). Operador `+` combina alinhamentos
unidimensionais em bidimensionais.

**Cristalino**: struct `Align2D { x: HAlign, y: VAlign }`. Operador
`+` implementado para preservar a mesma combinação (se ambos os
operandos têm o mesmo eixo, é erro; se têm eixos diferentes,
combinam-se nos dois campos).

**Observabilidade preservada**:
- `align(center)` produz o mesmo output visual.
- `align(center + bottom)` produz o mesmo output.
- `align(center + left)` produz o mesmo erro ("cannot add two
  horizontal alignments").

**Divergência**: a representação em memória é completamente
diferente. O enum vanilla tem ~18 variantes discriminadas; o struct
cristalino tem dois campos de enum mais pequenos.

### Exemplo 2 — ADR-0026 / ADR-0026-revisao: Content

**Vanilla**: `Content` é tipo opaco com vtable interna (trait
`NativeElement`). Elementos são structs distintos implementando
a trait.

**Cristalino**: `Content` é enum com variantes fixas (ex:
`Content::Sequence(Arc<[Content]>)`, `Content::Text(EcoString)`,
etc.).

**Observabilidade preservada**:
- `#show "foo": it => bar` funciona igual (matching pelo tipo de
  elemento).
- Concatenação via `+` produz a mesma sequência.
- Profundidade de aninhamento e ordem de elementos preservadas.

**Divergência**: o vanilla permite adicionar novos tipos de
elemento via trait externa (`impl NativeElement for MyElement`);
cristalino exige que o tipo esteja no enum. Esta é divergência
de **extensibilidade**, aceitável porque L1 cristalino não permite
extensão em runtime (ADR-0029, pureza física).

### Exemplo 3 — Passo 84.4 / DEBT-22: ShowRule

**Vanilla**: `Vec<ShowRule>` dentro de cada scope. Clone de scope
copia todos os show rules (O(n)).

**Cristalino**: `Arc<[ShowRule]>`. Clone é `Arc::clone` (O(1),
incremento de refcount). Ver ADR-0030 secção "Clone profundo vs
`Arc::clone`".

**Observabilidade preservada**:
- Ordem de aplicação dos show rules idêntica.
- Escopo de visibilidade idêntico (rules de escope exterior são
  visíveis no interior).
- Redefinição de regra (`#show` com mesmo seletor) sobreescreve
  igual.

**Divergência**: o vanilla permite que código do utilizador mute
o `Vec` interno em certas condições; cristalino torna isso
impossível porque `Arc<[T]>` é imutável. Esta divergência **não
é observável** porque o Typst vanilla nunca expõe a mutabilidade
ao utilizador — é detalhe de implementação.

---

## Relação com outros ADRs

- **ADR-0029** (pureza física): a paridade é definida **em relação
  ao vanilla**; o vanilla faz I/O de sistema em L2/L3 (mundo do
  compilador), e o cristalino mantém essa separação. A pureza de
  L1 cristalino corresponde à pureza computacional do typst-library
  vanilla.

- **ADR-0030** (performance é domínio): optimizações internas
  (hashing, caching, `Arc::clone`) são permitidas porque a
  paridade é sobre output, não sobre ciclos de CPU.

- **ADR-0026** e **ADR-0026-revisao** (Content): exemplo concreto
  de divergência estrutural permitida.

- **ADR-0034** (diagnóstico de tipos vanilla): complementar — o
  diagnóstico obrigatório garante que decisões de paridade são
  tomadas com conhecimento factual do comportamento vanilla, não
  por adivinhação.

---

## Como verificar paridade

Verificação formal é responsabilidade da suite `lab/parity/` (já
existente). Cada tipo/comportamento vanilla tem ou deve ter teste
de paridade que compare output cristalino com output vanilla para
input idêntico.

**Regra operacional**: quando um passo altera comportamento
relacionado com tipo vanilla, o passo deve:
1. Correr `lab/parity/` antes (baseline).
2. Aplicar mudança.
3. Correr `lab/parity/` depois.
4. Reportar zero regressões, ou justificar divergências aceitáveis.

DEBT-9 (cobertura incompleta de `lab/parity/`) continua em aberto
e é relevante para esta regra.

---

## Consequências

**Positivas**:
- Formaliza a regra que já estava a ser seguida implicitamente
  desde 84.5.
- Dá critério claro para decisões futuras: quando divergir do
  vanilla, é aceitável se e só se a divergência não é observável
  pelo utilizador.
- Protege contra "optimizações" que quebram comportamento sem
  intenção.

**Negativas**:
- Exige disciplina em materializações novas: qualquer mudança de
  tipo ou operador precisa de verificar se altera semântica
  observável.
- Pode bloquear refactors que seriam desejáveis por si mesmos mas
  alteram comportamento de erro (ex: mensagens mais claras).
  Resolução: refactors desse tipo requerem ADR próprio justificando
  a divergência.

**Neutras**:
- Não impede divergências puramente internas (forma de dados,
  organização de módulos, performance). A regra é sobre **output
  observável**.

---

## Alternativas Consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| "Fork total — comportamento pode divergir livremente" | Liberdade máxima | Cristalino vira projecto diferente; perde compatibilidade com ecossistema Typst |
| "Paridade bit-a-bit em todos os outputs" | Teste simples | Impossível — ordenação interna de dicionários, timestamps de PDF, etc., divergem legitimamente |
| **Decisão adoptada: paridade semântica com divergência estrutural permitida** | **Preserva compatibilidade sem amarrar implementação** | **Exige testes de paridade (`lab/parity/`) para garantir**  |
| "Paridade apenas para cenários listados no test suite" | Escopo bem definido | Paridade torna-se função do test suite — incompleto = ADR incompleto |

---

## Referências

- ADR-0029 — Pureza física em L1
- ADR-0030 — Performance é domínio de L1 (inclui secção Clone
  profundo vs Arc::clone)
- ADR-0026 + ADR-0026-revisao — Content como enum (exemplo 2)
- ADR-0034 — Diagnóstico obrigatório para tipos vanilla
  (complementar)
- Passo 84.5 — Materialização de Alignment como Align2D (exemplo 1)
- Passo 84.4 — Conversão de ShowRule para Arc<[T]> (exemplo 3)
- DEBT-9 — Cobertura incompleta de `lab/parity/` (relevante)
- `lab/parity/` — suite de testes de paridade (mecanismo de
  verificação)
```

### Verificar após criar

```bash
ls -la 00_nucleo/adr/typst-adr-0033-paridade-funcional-vanilla.md
grep "^\*\*Status\*\*" 00_nucleo/adr/typst-adr-0033-*.md
# Esperado: **Status**: `ACCEPTED`

# Três exemplos presentes
grep -c "^### Exemplo" 00_nucleo/adr/typst-adr-0033-*.md
# Esperado: 3

# Referências cruzadas presentes
grep -E "ADR-0026|ADR-0029|ADR-0030|ADR-0034" \
  00_nucleo/adr/typst-adr-0033-*.md
```

---

## Tarefa 2 — Criar ADR-0034

Criar `00_nucleo/adr/typst-adr-0034-diagnostico-tipos-vanilla.md`
com o conteúdo abaixo.

```markdown
# ⚖️ ADR-0034: Diagnóstico obrigatório antes de materializar tipo do vanilla

**Status**: `ACCEPTED`
**Data**: 2026-04-XX

---

## Contexto

ADR-0033 estabelece paridade funcional com vanilla como invariante
arquitectural. A decisão sobre o que **é** paridade, porém, exige
conhecimento factual do comportamento vanilla. Decisões
arquitecturais tomadas sem verificação produzem um de dois
resultados:

- Divergência acidental não detectada — o cristalino comporta-se
  diferentemente sem intenção.
- Medo de refactor — divergências potenciais são sempre recusadas
  "para segurança", mesmo quando seriam legítimas.

Durante a série 84.x, múltiplos passos incluíram secções de
diagnóstico antes da execução (84.2 para DEBT-38, 84.3 para
DEBT-21, 84.5 para DEBT-36, 84.6 para DEBT-37). Inicialmente,
três ADRs (0022, 0023, 0025) incluíram o diagnóstico **dentro**
do próprio ADR como anti-padrão — o Passo 84.8d refactorou-os,
movendo o diagnóstico para `00_nucleo/diagnosticos/`.

Este ADR formaliza a regra que emergiu: **diagnóstico antes, ADR
depois, diagnóstico persistido separado**.

---

## Decisão

### Regra

**Qualquer passo que materialize em L1 um tipo do vanilla deve,
antes de escrever código, produzir e persistir diagnóstico
estruturado em `00_nucleo/diagnosticos/`.**

Esta regra é **vinculativa**, não sugestão.

### Escopo

"Materializar tipo do vanilla" inclui:
- Criar em L1 um tipo novo correspondente a tipo existente no
  Typst vanilla (ex: criar `Align2D` para corresponder a
  `Alignment` vanilla).
- Adicionar operador ou método significativo a tipo vanilla
  materializado (ex: adicionar `impl Add for Align2D` para
  reproduzir `alignment + alignment`).
- Refactorar tipo materializado de forma que altera semântica
  (forma estrutural nova, novos campos observáveis).

**Não** está no escopo:
- Correcções pontuais de bugs (ex: erro de cálculo num método
  existente).
- Optimizações internas (ex: converter campo `Vec<T>` para
  `Arc<[T]>` — não altera semântica observável).
- Tipos novos sem correspondência no vanilla (ex: tipos auxiliares
  internos da arquitectura cristalina).

### Conteúdo mínimo do diagnóstico

Cada diagnóstico em `00_nucleo/diagnosticos/` deve conter:

1. **Localização do tipo vanilla**: caminho(s) em
   `lab/typst-original/` onde o tipo está definido.
2. **Campos e variantes**: enumeração completa. Se enum, todas as
   variantes; se struct, todos os campos com tipos.
3. **Operadores e métodos públicos**: lista com assinatura.
4. **Dependências**: outros tipos referenciados (para entender
   ordem de materialização).
5. **Semântica de operadores-chave**: comportamento de `+`, `-`,
   `==`, conversões, casts — com exemplos concretos do vanilla.
6. **Mensagens de erro**: texto exacto dos erros que o tipo pode
   produzir no vanilla.
7. **Divergências propostas**: se a materialização vai divergir
   estruturalmente, descrever forma cristalina proposta e
   justificar paridade semântica (link para ADR-0033).

### Convenção de nome do ficheiro

`00_nucleo/diagnosticos/diagnostico-<contexto>-<slug>.md`

Onde `<contexto>` é um de:
- `adr-NNNN` — diagnóstico associado a ADR específico
  (convenção estabelecida no 84.8d para diagnósticos históricos).
- `passo-N.M` — diagnóstico associado a passo de materialização
  (convenção sugerida para futuros).
- `tipo-<nome>` — diagnóstico standalone sobre tipo vanilla
  (quando o diagnóstico precede a decisão de ADR ou passo).

Exemplos:
- `diagnostico-adr-0022-fontbook.md` (histórico, 84.8d).
- `diagnostico-passo-84.5-alignment.md` (convenção sugerida).
- `diagnostico-tipo-gradient.md` (antes de decidir passo).

### Cabeçalho obrigatório

```markdown
# Diagnóstico: <título>

**Tipo vanilla**: <nome do tipo principal>
**Localização vanilla**: `lab/typst-original/<path>`
**Data do diagnóstico**: YYYY-MM-DD
**Contexto**: <ADR ou passo que motivou; "standalone" se nenhum>

**Natureza**: registo factual do estado do vanilla na data acima.
Decisões arquitecturais derivadas deste diagnóstico ficam em
ADR/passo separados. Este ficheiro não contém decisões.
```

---

## Relação com outros ADRs

- **ADR-0033** (paridade vanilla): este ADR é mecanismo que torna
  ADR-0033 operacional. Sem diagnóstico, "paridade" seria
  afirmação sem base factual.

- **Anti-padrão corrigido no 84.8d**: os 3 ADRs (0022, 0023, 0025)
  continham diagnóstico dentro do ADR — era violação desta regra
  (ainda não formalizada na altura). O refactor do 84.8d moveu
  os diagnósticos para o directório canónico e os ADRs agora
  apenas referenciam via linha `**Diagnóstico prévio**: ver ...`.

---

## Consequências

**Positivas**:
- Decisões de paridade baseiam-se em factos documentados, não em
  memória.
- Futuros LLMs que peguem no projecto têm registo claro do que foi
  verificado antes de cada decisão.
- Diagnósticos persistidos permitem auditoria retrospectiva
  (como o Passo 83.6 fez com o 83.5).

**Negativas**:
- Passos de materialização tornam-se mais longos (diagnóstico
  precede execução).
- Pode haver tentação de "saltar o diagnóstico" para tipos
  aparentemente triviais. Resposta: os tipos tipográficos do
  vanilla listados em ADR-0029 **não** são triviais; cada um tem
  semântica particular (combinações, conversões, mensagens de
  erro específicas) que exigem verificação.

---

## Alternativas Consideradas

| Alternativa | Prós | Contras |
|-------------|------|---------|
| "Diagnóstico sugerido, não obrigatório" | Menos overhead para casos simples | Abre porta a "é trivial, não preciso" — volta ao estado pré-84.8d |
| "Diagnóstico dentro do ADR (como estava em 0022/0023/0025)" | Informação no mesmo sítio | Mistura decisão com execução; anti-padrão identificado no 84.7 |
| "Diagnóstico apenas para tipos novos, não para refactors" | Simplifica critério | Refactors são precisamente onde erros de paridade aparecem |
| **Decisão adoptada: obrigatório, persistido em directório canónico, referenciado pelo ADR/passo** | **Força rigor; preserva rastreabilidade; permite auditoria** | **Overhead real em cada materialização** |

---

## Referências

- ADR-0033 — Paridade funcional vanilla (regra que este ADR torna
  operacional)
- Passo 84.8d — Refactor do anti-padrão (criou `00_nucleo/diagnosticos/`)
- `00_nucleo/diagnosticos/` — directório canónico
- `00_nucleo/diagnosticos/diagnostico-adr-0022-fontbook.md`
  (exemplo de diagnóstico histórico)
- `lab/typst-original/` — fonte canónica para verificar
  comportamento vanilla
```

### Verificar após criar

```bash
ls -la 00_nucleo/adr/typst-adr-0034-diagnostico-tipos-vanilla.md
grep "^\*\*Status\*\*" 00_nucleo/adr/typst-adr-0034-*.md
# Esperado: **Status**: `ACCEPTED`

# Secção de conteúdo mínimo tem os 7 itens
grep -c "^[0-9]\." 00_nucleo/adr/typst-adr-0034-*.md
# Esperado: >= 7 (pode ter mais listas numeradas; confirmar
# visualmente que a secção "Conteúdo mínimo" tem 7 itens)

# Regra explicitamente "vinculativa"
grep "vinculativa\|obrigatóri" 00_nucleo/adr/typst-adr-0034-*.md
```

---

## Tarefa 3 — Verificação global

```bash
# Contagem de ADRs passou de 33 para 35
ls 00_nucleo/adr/typst-adr-*.md | wc -l

# Ambos os novos têm status ACCEPTED
grep -m1 "^\*\*Status\*\*" \
  00_nucleo/adr/typst-adr-0033-*.md \
  00_nucleo/adr/typst-adr-0034-*.md
# Esperado: 2 linhas, ambas com ACCEPTED em backticks.

# Nenhum ADR existente alterado
git diff --stat 00_nucleo/adr/ | grep -v "typst-adr-003[34]"
# Esperado: vazio (só 0033 e 0034 adicionados, não modificados).

# Código intacto
git status 01_core/ 02_shell/ 03_infra/ 04_wiring/

# DEBT.md intacto
git diff --stat 00_nucleo/DEBT.md

# Diagnósticos intactos
git diff --stat 00_nucleo/diagnosticos/

# Testes
cargo test

# Linter
crystalline-lint .
```

---

## Critérios de conclusão

- [ ] `typst-adr-0033-paridade-funcional-vanilla.md` criado.
- [ ] `typst-adr-0034-diagnostico-tipos-vanilla.md` criado.
- [ ] Ambos com status `ACCEPTED` em formato canónico.
- [ ] ADR-0033 tem 3 exemplos concretos (Align2D, Content, ShowRule).
- [ ] ADR-0034 tem secção "Conteúdo mínimo do diagnóstico" com 7
  itens.
- [ ] ADR-0034 declara regra como "vinculativa" / "obrigatória"
  explicitamente.
- [ ] Contagem de ADRs: 33 → 35.
- [ ] Nenhum ADR existente modificado.
- [ ] Nenhum DEBT aberto ou fechado.
- [ ] Nenhum código-fonte tocado.
- [ ] `cargo test` mantém 911 testes.
- [ ] `crystalline-lint .` zero violations.

---

## Ao terminar, reportar

- Confirmação de criação dos dois ADRs.
- Tamanho de cada ADR (linhas).
- Contagem final de ADRs (esperado: 35).
- Confirmação de que nenhum ADR existente foi tocado.
- Confirmação de que os 3 exemplos no ADR-0033 correspondem aos
  combinados (Align2D, Content enum, Arc<[ShowRule]>).

**Go/No-Go para P84.8f** (estrutura final: README.md, ADR-0026
duplicado, uniformização de vocabulário, coexistência com
crystalline-lint):

- **GO**: ADR-0033 e ADR-0034 criados com formato canónico,
  referências cruzadas consistentes, código intacto.
- **NO-GO — paridade mal explicada**: se os 3 exemplos do ADR-0033
  ficaram abstractos sem identificar claramente forma divergente
  vs semântica preservada, o ADR não serve como referência
  operacional. Reescrever.
- **NO-GO — ADR-0034 permissivo**: se o texto do ADR-0034 ficou
  como "sugestão" ou "recomendado" em vez de "vinculativo" /
  "obrigatório", falha em capturar a decisão. Reforçar linguagem.

---

## Nota sobre o estado do plano pós-84.8e

Após este passo, faltará apenas o 84.8f para fechar a série 84.8
de correcções do relatório P84.7:

- **84.8f** — Estrutura final:
  - Criar `00_nucleo/adr/README.md` como índice de ADRs
    (proposta na Secção 7 do relatório 84.7).
  - Resolver ADR-0026 duplicado (renomear para `-R1` ou similar).
  - Decidir coexistência com ADRs do `crystalline-lint`
    (recomendação 84.7: subdirectório `00_nucleo/adr/lint/`).
  - Uniformizar vocabulário de status: introduzir `EM VIGOR` para
    regras/políticas; aplicar `IMPLEMENTADO` apenas a decisões
    técnicas materializadas. Afecta os ADRs em `ACCEPTED`
    (0029, 0030, 0031, 0032, 0033, 0034 — neste passo) e os em
    `IMPLEMENTADO` conforme caso-a-caso.

O 84.8f é o último da série. Após a sua execução, a série 84.8
fecha e o projecto retoma trabalho sobre DEBTs restantes.

---

## Nota sobre caminho de ficheiro do relatório

Este passo **não produz relatório**. O produto são os dois ADRs
novos, ambos em `00_nucleo/adr/`.
