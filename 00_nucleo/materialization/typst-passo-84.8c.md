# Passo 84.8c — Lacunas de conteúdo em ADR-0029 e ADR-0030

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0029-pureza-fisica-revoga-adr-0028.md` —
  ADR alvo da Tarefa 2.
- `00_nucleo/adr/typst-adr-0030-performance-dominio-l1.md` — ADR
  alvo da Tarefa 3.
- `00_nucleo/relatorios/relatorio-auditoria-adrs-passo-84.7.md` —
  Secções 3.1, 3.2 e 6.2 (motivação destas correcções).
- `lab/typst-original/crates/typst-library/src/foundations/` —
  directório do vanilla onde a maior parte dos tipos-valor vive.
- `lab/typst-original/crates/typst-library/src/layout/` — directório
  onde os tipos de layout-valor vivem (`Length`, `Abs`, `Rel`,
  `Angle`, `Fr`, `Em`, `Alignment`, `Sides`, etc.).

Pré-condição: `cargo test` — 911 testes, zero violations. P84.8b
concluído, 6 status alinhados, convenção `**Revogado por**:`
introduzida.

---

## Natureza deste passo

**Passo de correcção de conteúdo em 2 ADRs existentes.** Edita o
corpo (não apenas o cabeçalho) de dois ADRs para preencher
lacunas identificadas no P84.7. Inclui **Tarefa 0 de diagnóstico
em linha** — grep ao vanilla para enumerar tipos tipográficos —
sem gerar relatório persistente.

Nenhum ADR novo, nenhum DEBT novo, nenhum código tocado.

Três produtos:

1. Tarefa 0 diagnóstico — listar tipos tipográficos do vanilla a
   partir de `lab/typst-original/`.
2. ADR-0029 recebe enumeração completa de tipos tipográficos (com
   `Alignment` incluído e os restantes candidatos identificados
   na Tarefa 0).
3. ADR-0030 recebe secção nova "Clone profundo vs `Arc::clone`".

Decisão já confirmada: ADR-0030 **absorve** a regra `Arc::clone`
completamente (Opção α). ADR-0033 (que era o plano B) **não será
criado**. O plano dos passos seguintes (84.8e) acomoda-se a esta
decisão.

---

## Tarefa 0 — Diagnóstico de tipos tipográficos do vanilla

**Propósito**: enumerar os tipos-valor tipográficos do Typst
vanilla que devem aparecer na lista do ADR-0029. Este diagnóstico
corre em linha e preenche a Tarefa 2. Não é relatório
persistente.

### Critério de inclusão

**Incluir**: tipos-valor tipográficos atómicos — tipos que
representam quantidade, cor, direcção, fracção, ângulo, etc.,
usados directamente como valores no DSL do Typst.

**Excluir**:
- Estruturas container genéricas (`Sides<T>`, `Axes<T>`,
  `Corner<T>`, `Smart<T>`) — são matéria estrutural tratada
  implicitamente por ADR-0026.
- Tipos que não são "tipográficos" no sentido de valor tipográfico
  (ex: `Func`, `Content`, `Module`, `Array`, `Dict`, `Bytes` — são
  tipos do DSL mas não são unidades tipográficas).
- Tipos efémeros de implementação (enums internos de layout que
  não são expostos ao DSL).

**Casos fronteiriços a discutir se aparecerem**: `Regex`, `Version`,
`Duration`, `Datetime`. São valores do DSL mas não tipográficos no
sentido estrito. **Decisão durante a Tarefa 0**: se o vanilla
tratar estes tipos no mesmo módulo que `Length`/`Angle`/`Color`,
incluir na lista; se estiverem em módulo separado (ex:
`foundations/` vs `layout/`), excluir. `Datetime` tem o seu próprio
ADR-0021 — **excluir** da lista do ADR-0029 (já tratado).

### Comandos de diagnóstico

```bash
# Enumerar tipos em layout/ (onde vive a maior parte)
ls lab/typst-original/crates/typst-library/src/layout/*.rs

# Identificar structs e enums públicos em layout/
grep -rn "^pub struct\|^pub enum" \
  lab/typst-original/crates/typst-library/src/layout/*.rs \
  | grep -v "_test\|::test"

# Identificar tipos em foundations/ que podem ser tipográficos
ls lab/typst-original/crates/typst-library/src/foundations/*.rs

grep -rn "^pub struct\|^pub enum" \
  lab/typst-original/crates/typst-library/src/foundations/*.rs \
  | grep -v "_test\|::test" \
  | head -50

# Verificar implementações de Cast<Value> ou IntoValue — sinal
# forte de "tipo tipográfico exposto ao DSL"
grep -rn "impl Cast for\|impl IntoValue for\|impl FromValue for" \
  lab/typst-original/crates/typst-library/src/layout/ \
  lab/typst-original/crates/typst-library/src/visualize/ \
  | head -30

# visualize/ é onde viveim tipos visuais (Color, Gradient, Paint,
# Stroke, Pattern/Tiling)
ls lab/typst-original/crates/typst-library/src/visualize/*.rs 2>/dev/null \
  || ls lab/typst-original/crates/typst-library/src/visualize/

grep -rn "^pub struct\|^pub enum" \
  lab/typst-original/crates/typst-library/src/visualize/
```

### Processamento dos resultados

A partir dos outputs, compor lista filtrada segundo o critério de
inclusão. Formato da lista final:

```
Tipos tipográficos identificados (a incluir em ADR-0029):
- Length
- Abs
- Rel
- Ratio
- Em
- Fr (Fraction)
- Angle
- Alignment
- HAlignment
- VAlignment
- Direction
- Color
- Gradient
- Paint
- Stroke
- Tiling (ou Pattern em versões antigas)
- [... outros identificados pelo grep ...]

Tipos excluídos (com razão):
- Sides<T>, Axes<T>, Corner<T> — containers genéricos
- Datetime — já coberto por ADR-0021
- Func, Array, Dict, Content, Module — tipos do DSL não tipográficos
- [... outros ...]
```

**Reportar esta lista antes de prosseguir para a Tarefa 2.** A lista
é input directo para a edição do ADR-0029.

---

## Tarefa 1 — Ler o corpo actual do ADR-0029 e ADR-0030

Antes de editar, ler o corpo completo de ambos:

```bash
cat 00_nucleo/adr/typst-adr-0029-pureza-fisica-revoga-adr-0028.md
cat 00_nucleo/adr/typst-adr-0030-performance-dominio-l1.md
```

**Em particular identificar**:

- ADR-0029: onde está a enumeração actual (`Length`, `Abs`, `Rel`,
  `Angle`, `Ratio`, `Color`). Pode estar na secção "Decisão" ou
  "Âmbito" ou similar. A edição da Tarefa 2 **preserva a secção
  exacta**, só expande a lista.

- ADR-0030: onde se fala de `Arc<T>` em campos struct. A edição da
  Tarefa 3 adiciona secção nova **imediatamente após** essa
  secção existente, de modo que a continuidade narrativa do ADR
  fique preservada.

---

## Tarefa 2 — Expandir lista de tipos tipográficos em ADR-0029

### Edição

Na secção do ADR-0029 que enumera tipos tipográficos, substituir a
lista actual (`Length`, `Abs`, `Rel`, `Angle`, `Ratio`, `Color`)
pela lista completa identificada na Tarefa 0, mantendo ordem
lógica (tipos de dimensão, depois de razão/fracção, depois de
ângulo, depois de alinhamento, depois de cor/visual).

**Adicionar nota explícita** imediatamente após a lista:

```markdown
Esta enumeração lista os tipos tipográficos reconhecidos no Typst
vanilla no momento desta ADR, independentemente de estarem ou não
materializados em L1. A arquitectura cristalina mantém paridade
de tipos com o vanilla — cada tipo aparece em L1 com forma
eventualmente divergente (ex: `Align2D` em cristalino vs
`Alignment` em vanilla) mas com semântica observável idêntica.

Tipos futuros que o Typst vanilla venha a adicionar seguirão a
mesma arquitectura e serão adicionados a esta enumeração conforme
identificados em novos ADRs de materialização.
```

**Justificação da nota** (para o contexto do passo, não entra no
ADR): esta formulação previne que daqui a seis meses, quando um
tipo novo do vanilla seja materializado, a enumeração do ADR-0029
fique novamente desalinhada. A nota tem o efeito de duas coisas
simultâneas: documenta a lista actual como snapshot temporal, e
reconhece que a lista cresce.

### Verificar após editar

```bash
# Alignment agora aparece
grep "Alignment" 00_nucleo/adr/typst-adr-0029-*.md

# Outros tipos identificados na Tarefa 0 também aparecem — substituir
# <TIPO> pelos tipos concretos identificados
grep -E "Gradient|Paint|Stroke|Fr\b|Em\b" \
  00_nucleo/adr/typst-adr-0029-*.md

# A nota sobre "tipos futuros" está presente
grep "tipos futuros\|vanilla venha a adicionar" \
  00_nucleo/adr/typst-adr-0029-*.md
```

---

## Tarefa 3 — Adicionar secção "Clone profundo vs `Arc::clone`" em ADR-0030

### Localização da edição

A secção nova deve ser adicionada **imediatamente após** a secção
actual do ADR-0030 que discute `Arc<T>` em campos struct. Se essa
secção não existir como cabeçalho próprio (pode estar embutida na
"Decisão" ou "Consequências"), a secção nova vai para o fim da
"Decisão" antes das "Consequências".

### Conteúdo da secção nova

```markdown
## Clone profundo vs `Arc::clone`

A palavra `clone` em Rust é polissémica. Duas semânticas distintas
partilham o mesmo nome de método:

- **`Vec<T>::clone()`** (e clones de structs que contêm `Vec<T>`,
  `HashMap<K, V>`, `String`, etc.) — **cópia profunda de bytes**.
  Custo O(n) sobre o tamanho do dado. Cada clone aloca memória
  nova e duplica os conteúdos.

- **`Arc<T>::clone()`** (e clones de structs que só contêm `Arc<T>`,
  `Rc<T>`, `EcoString` que internamente usa refcounting) —
  **incremento de um contador atómico**. Custo O(1) independente
  do tamanho do dado. Nenhuma memória nova é alocada; o dado
  apontado é partilhado.

Esta ADR estabelece duas regras que derivam desta distinção:

1. **Proibido no hot path de L1**: `Vec<T>::clone()`, `String::clone()`,
   `HashMap<K, V>::clone()` e qualquer clone profundo semelhante.
   Se um campo tem semântica de "partilha" (múltiplos donos,
   imutável após criação), o tipo do campo deve ser `Arc<T>`,
   `Arc<[T]>`, `Arc<str>` ou `EcoString`.

2. **Obrigatório em L1 quando partilha é semântica**: quando um
   valor precisa de ser replicado para múltiplos donos sem
   duplicar bytes, usar `Arc::clone` explicitamente. Exemplos no
   código actual: `Source` contém `Arc<str>` (ADR-0031),
   `Content::Sequence(Arc<[Content]>)` (ADR-0026-revisao),
   `Arc<[ShowRule]>` (Passo 84.4 / DEBT-22).

**Observação sobre detecção**: a distinção entre clone profundo e
`Arc::clone` não é visível no código — ambos se escrevem
`x.clone()`. O critério é o **tipo de `x`**. Revisões de código
e passos de materialização devem verificar que campos em hot-path
têm tipos que tornam `clone` O(1), não O(n).

**Relação com ADR-0026-revisao**: a decisão de usar `Arc<[Content]>`
em `Content::Sequence` é instância directa desta regra.

**Relação com DEBT-22** (encerrado no Passo 84.4): a conversão de
`Vec<ShowRule>` para `Arc<[ShowRule]>` é instância directa desta
regra.
```

### Verificar após editar

```bash
# A nova secção tem o cabeçalho esperado
grep "^## Clone profundo" 00_nucleo/adr/typst-adr-0030-*.md

# Menciona Vec::clone e Arc::clone explicitamente
grep -E "Vec<T>::clone|Arc<T>::clone" 00_nucleo/adr/typst-adr-0030-*.md

# Menciona refs a ADRs relacionados
grep -E "ADR-0026|ADR-0031|DEBT-22" 00_nucleo/adr/typst-adr-0030-*.md

# A secção "Consequências" continua existir e vem depois (ou seja,
# a nova secção foi inserida antes de "Consequências", não depois)
awk '/^## Clone profundo/,/^## Consequências/' \
  00_nucleo/adr/typst-adr-0030-*.md \
  | head -40
```

---

## Tarefa 4 — Verificação final

```bash
# Conteúdo do ADR-0029 cresceu (lista expandida + nota)
wc -l 00_nucleo/adr/typst-adr-0029-*.md
# (Comparar mentalmente com o tamanho anterior — deve ter ~10-15
# linhas adicionais)

# Conteúdo do ADR-0030 cresceu (secção nova ~25-30 linhas)
wc -l 00_nucleo/adr/typst-adr-0030-*.md

# Status não foi alterado (só o corpo)
grep "^\*\*Status\*\*:" 00_nucleo/adr/typst-adr-0029-*.md \
                       00_nucleo/adr/typst-adr-0030-*.md

# Código intacto
git status 01_core/ 02_shell/ 03_infra/ 04_wiring/

# DEBT.md intacto
git diff --stat 00_nucleo/DEBT.md

# Outros ADRs intactos (só os dois alvo deste passo foram tocados)
git diff --stat 00_nucleo/adr/ | head -20
# Esperado: só os dois ficheiros alvo modificados. Se houver outros,
# reverter.

# Testes
cargo test

# Linter
crystalline-lint .
```

---

## Critérios de conclusão

- [ ] Tarefa 0 executada: lista de tipos tipográficos do vanilla
  identificada e reportada antes da Tarefa 2.
- [ ] ADR-0029 recebe lista expandida com todos os tipos
  identificados na Tarefa 0 (mínimo: `Alignment` explícito; real:
  todos os tipos vanilla relevantes).
- [ ] ADR-0029 recebe nota "tipos futuros serão adicionados" no
  formato canónico proposto na Tarefa 2.
- [ ] ADR-0030 recebe secção nova "Clone profundo vs `Arc::clone`"
  com a estrutura completa proposta na Tarefa 3 (duas regras,
  exemplos concretos de ADR-0026/0031 e DEBT-22).
- [ ] Status de ambos ADRs permanece `ACCEPTED` (não alterado pelo
  passo; alvo de uniformização em passo futuro 84.8f).
- [ ] Nenhum outro ADR tocado.
- [ ] Nenhum código-fonte tocado.
- [ ] Nenhum DEBT aberto ou fechado.
- [ ] `cargo test` mantém 911 testes.
- [ ] `crystalline-lint .` zero violations.

---

## Ao terminar, reportar

- Lista final de tipos tipográficos identificados na Tarefa 0,
  com distinção entre "incluídos" e "excluídos com razão".
- Tamanho antes/depois do ADR-0029 (linhas).
- Tamanho antes/depois do ADR-0030 (linhas).
- Confirmação de que a secção "Clone profundo" do ADR-0030 foi
  inserida antes de "Consequências" (não depois).
- Confirmação de que nenhum outro ADR foi modificado.
- Qualquer caso fronteiriço da Tarefa 0 onde a decisão de incluir
  ou excluir foi ambígua (ex: `Regex`, `Version`, `Duration`).

**Go/No-Go para P84.8d** (refactor do anti-padrão "Diagnóstico
obrigatório" em ADR-0022/0023/0025):

- **GO**: Os dois ADRs alvo têm conteúdo expandido, ADR-0030
  absorveu a regra Arc::clone (ADR-0033 não será criado no
  84.8e), código e DEBT.md intactos.
- **NO-GO — lista vanilla muito pequena**: se a Tarefa 0
  identificou só os 6 tipos que já estavam na lista original +
  `Alignment`, sem outros, reportar. Pode ser que o grep não
  tenha apanhado os restantes, ou que a estrutura de directórios
  do vanilla não corresponda ao que o enunciado assume.
- **NO-GO — secção "Clone profundo" colocada mal**: se a secção
  foi inserida depois de "Consequências" em vez de antes,
  reverter e reposicionar.

---

## Nota sobre caminho de ficheiro do relatório

Este passo **não produz relatório persistente**. A Tarefa 0 é
diagnóstico em linha cuja saída vai para a conversa (input da
Tarefa 2) mas não se persiste como ficheiro separado. O produto
são as duas edições nos ADRs alvo.

Justificação para não persistir: a lista da Tarefa 0 é
imediatamente consumida pelo ADR-0029 (Tarefa 2) que passa a ser
a versão canónica. Guardar a lista num relatório intermédio
criaria duplicação entre "lista em relatório do 84.8c" e "lista
em ADR-0029" — quando divergissem, qual seria a canónica? O ADR
ganha esse papel. A conversa do 84.8c fica como registo
histórico da decisão.
