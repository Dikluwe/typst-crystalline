# Passo 1016 — Fechar `footnote_counter` e `enum_counter`

**Pedido**: fechar os dois contadores locais ao `Layouter` assinalados como achado
lateral no Passo 1014 (§Fase C-bis).
**Nota de numeração**: o pedido dizia "passo 1015", mas `typst-passo-1015.md` já existe e
é outra coisa — a dedup das três cópias de `long_type_name`, achado do P1013. Este
trabalho ficou como **1016** para não sobrepor. Renumerar é trivial se preferires.

**Resultado**:
- **`enum_counter` — fechado.** Não é candidato a migração; está correctamente local. Mas
  a medição expôs um **defeito de paridade real**, corrigido aqui em fluxo contínuo.
- **`footnote_counter` — medido, não fechado.** O gap é real e o caminho é conhecido, mas
  fechá-lo é mudança de comportamento por defeito → **gate ADR-0127**. L0 redigido abaixo,
  à espera da tua confirmação. **Nenhum código escrito para esta parte.**

**Proveniência**: medições a partir de `HEAD = 836a8979a`, 2026-08-12 ~19:05 −03, working
tree com as alterações da Parte 1 deste relatório (`git diff HEAD --stat`: 14 ficheiros,
dos quais 11 são só resselo de `@prompt-hash`). Binário vanilla de referência:
`lab/typst-original/target/release/typst`.

---

## Parte 1 — `enum_counter`

### 1.1 A questão de localização fecha: está correctamente local

`enum_counter` **não é um contador de documento**. É um contador de **posição dentro do
grupo contíguo de itens**, que reinicia quando a lista termina. Evidência:

- `01_core/src/compiler/layout/enum_item.rs:56-62` — lê e escreve o próximo número;
  `e.number` explícito sobrepõe-se.
- `01_core/src/compiler/layout/sequence.rs:104` — `enum_counter = None` quando aparece
  conteúdo que não pertence à lista.
- `01_core/src/entities/element_kind.rs:57-61` — `ElementKind::Enum` existe **só como
  selector** (P494): *"Cristalino não materializa `Content::Enum` (enum é `Sequence` de
  `EnumItem`); contagem em L3 por análise do `Content`"*. Não há `kind_index`, não há
  counter flat, e não faria sentido haver: a numeração de enum reinicia por lista, não
  acumula no documento.

Migrar isto para o `CounterRegistry` — que é monotónico e por kind, como o `"table"` de
P461 — seria **semanticamente errado**. A dúvida do P1001 não se aplica a este campo.
**Fechado.**

### 1.2 A medição expôs outra coisa: a numeração reiniciava onde não devia

Ao medir, o comportamento não bate com o vanilla. Entrada:

```typst
+ a
+ b

+ c

texto

+ d
+ e
```

| | Vanilla | Cristalino (antes) |
|---|---|---|
| saída | `1. 2. 3.` … `texto` … `1. 2.` | `1. 2. 1.` … `texto` … `1. 2.` |

Regra do vanilla, medida: **a linha em branco não termina a lista** — o `Parbreak` torna a
lista *solta* (muda o espaçamento) mas a numeração continua. Só **conteúdo real** termina
a lista e reinicia a numeração.

O cristalino reiniciava também no `Parbreak`, em
`01_core/src/compiler/layout/sequence.rs:45` (antes da correcção).

### 1.3 Não era divergência deliberada — era uma afirmação falsa no L0

`00_nucleo/prompts/compiler/layout.md` §P864 dizia:

> […] introduzindo o espaçamento de parágrafo entre grupos e **reiniciando contadores
> quando aplicável** — **paridade vanilla** com listas/enums/termos separados por linha em
> branco no markup.

E o teste `layout_enum_parbreak_separa_grupos_e_reinicia_numero`
(`layout/tests.rs:1311`) assertava `"1."`/`"1."` com o comentário *"segundo grupo reinicia
em 1"*. Ou seja: a crença errada estava codificada em três sítios (L0, código, teste) e
justificada como paridade. Não é uma divergência registada ao abrigo da ADR-0033 — é um
erro.

Por isso segue em **fluxo contínuo** (ADR-0127: *"correções de paridade com o vanilla
seguem em fluxo contínuo — L0 editado primeiro + resselo, sem paragem; o gate é o teste
RED→GREEN + revalidação"*), e não em gate.

### 1.4 Correcção aplicada

**Ordem seguida**: L0 primeiro, depois teste RED, depois código, depois resselo.

1. **L0** — `layout.md` §P864 reescrito: a afirmação de paridade foi substituída pela
   tabela de medição acima; a regra 1 passa a dizer explicitamente *"Não toca em
   `enum_counter`"*; a regra 3 passa a nomear-se como **o único ponto onde a numeração
   reinicia**.
2. **Teste RED** — `layout_enum_parbreak_separa_grupos_e_reinicia_numero` renomeado para
   `..._mas_continua_numero` e a expectativa corrigida para `1.`/`2.`; teste novo
   `layout_enum_conteudo_real_entre_grupos_reinicia_numero` cobre a forma completa
   (`1. 2. 1.`). Ambos falharam antes da correcção:
   ```
   layout_enum_conteudo_real_entre_grupos_reinicia_numero ... FAILED
     left: ["1.", "1.", "1."]   right: ["1.", "2.", "1."]
   layout_enum_parbreak_separa_grupos_mas_continua_numero ... FAILED
     left: "1."                 right: "2."
   ```
3. **Código** — removida a linha `layouter.enum_counter = None;` de `sequence.rs`, dentro
   do braço do `Parbreak`. **Só essa linha.** O espaçamento de grupo
   (`cursor_y += paragraph_advance`, `last_was_loose_item = false`) fica — é a parte que o
   P864 acertou, e continua coberta pelos testes de gap que já existiam.
4. **Resselo** — `crystalline-lint --fix-hashes .`.

### 1.5 Validação

| Verificação | Resultado |
|---|---|
| Testes de enum | 6 passed, 0 failed (2 RED → GREEN) |
| `cargo test --workspace` | **5833 passed, 0 failed, 3 ignored** |
| `#[test]` HEAD → working tree | 5832 → 5833 (+1: o teste novo) |
| `crystalline-lint .` | 0 errors; 3 × V7 órfãos, todos pré-existentes ou untracked do dono (`auditar-spec.md`, `auditar-fatiamento.md`, `infra/package_version_resolution.md`) |

Decalque end-to-end depois da correcção, mesmo ficheiro de entrada:

```
CRISTALINO          VANILLA
1. a                1. a
2. b                2. b
3. c                3. c
texto               texto
1. d                1. d
2. e                2. e
```

---

## Parte 2 — `footnote_counter`: medido, **não** fechado (gate)

### 2.1 O gap é real

Ao contrário do enum, `footnote_counter` **é** um contador de documento monotónico
(`layout/footnote.rs:23-24`: `layouter.footnote_counter += 1`), e vive fora da máquina de
counters. O que isso custa, medido:

Entrada: `A#footnote[uma] B#footnote[duas]` + `#context counter(footnote).get()`

| Consulta | Vanilla | Cristalino |
|---|---|---|
| marcadores no corpo | `A1 B2` | `A[1] B[2]` |
| notas no rodapé | `1uma` / `2duas` | `[1] uma` / `[2] duas` |
| `counter(footnote).get()` | **`(2,)`** | **erro**: `counter() requer string, selector ou função de elemento, recebeu function` |
| `counter("footnote").get()` | `(0,)` | `(0,)` |

Duas leituras que a medição corrigiu face ao que eu presumia:

1. **A chave certa é a função de elemento, não a string.** O vanilla também devolve `(0,)`
   para `counter("footnote")` — a string é um contador de utilizador, distinto. O alvo é
   `counter(footnote)`.
2. **O render funciona.** O contador local numera correctamente marcadores e notas,
   incluindo o caso de colunas (`p552_footnote_counter_avanca_em_set_page_columns`). O que
   falta é a **consulta**: o utilizador não consegue ler o contador nem, presumivelmente,
   referenciar uma nota.

### 2.2 Porque é que está assim, e o que a nota no código dizia

`01_core/src/compiler/layout/mod.rs:224-230`:

> **P295 (Footnote Fase 1)** — counter monotónico incrementado em cada `Content::Footnote`
> consumido. […] Walker counter simples (sem Counter/Introspector machinery) — magnitude
> reduzida para Fase 1. Sub-passos P295.1 + P295.2 migrarão para Counter machinery **se
> 2-pass layout for adoptado**.

A condição do 2-pass já não é necessária: o P461 fez exactamente esta migração para
`Content::Table` sem 2-pass, usando o fixpoint que já existe. `ElementKind::Footnote`
existe (`element_kind.rs:78`) mas **só como selector** (P494) — *"contagem por análise do
`Content` em L3"* — tal como `Table` antes do P461.

### 2.3 Porque é gate

Fechar isto é a categoria (2) da ADR-0127 — **mudança de comportamento por defeito do
produto** — e discutivelmente a (3):

- `counter(footnote)` passa de **erro** a valor. Programas que hoje falham passam a
  compilar.
- A população do contador muda da travessia de layout para a fase de introspecção.
- Promover `Content::Footnote` a locatable indexa-o em `kind_index`, o que abre
  `#ref`/label sobre notas — superfície nova.

Em caso de dúvida sobre a classe, a regra é parar. Paro.

---

## L0 proposto (não guardado — aguarda a tua confirmação)

> Destino sugerido: `00_nucleo/prompts/compiler/layout/footnote_counter.md`, ou uma secção
> nova em `compiler/layout.md`. Não o coloquei em `prompts/` para não criar um órfão V7
> antes de existir código — foi exactamente essa a dívida limpa no P1014.

```markdown
# Prompt L0 — `compiler/layout/footnote_counter` — contador de notas na máquina de counters

**Camada**: L1
**Ficheiros alvo**: `01_core/src/compiler/layout/footnote.rs`,
`01_core/src/compiler/introspect.rs`, `01_core/src/compiler/introspect/locatable.rs`,
`01_core/src/compiler/introspect/extract_payload.rs`,
`01_core/src/compiler/stdlib/counter.rs`
**ADRs**: ADR-0107 (paridade língua), ADR-0127 (gate — categoria 2)
**Precedente a espelhar**: P461 (`Content::Table` promovido a locatable)

## Contexto

`footnote_counter` é hoje um campo do `Layouter` (`layout/mod.rs:230`), incrementado na
travessia. Numera correctamente marcadores e notas, mas não é visível à máquina de
counters: `counter(footnote)` erra em vez de devolver o valor. O vanilla devolve `(2,)`
para dois footnotes. A condição registada em P295 ("se 2-pass layout for adoptado")
caducou — o P461 fez a mesma migração para `Table` sobre o fixpoint existente.

## Instrução

1. **Promover `Content::Footnote` a locatable**, espelhando P461:
   - `introspect/locatable.rs` — reconhecer `Content::Footnote` e atribuir `Location`.
   - `introspect/extract_payload.rs` — emitir o payload de counter.
   - `introspect.rs` — `apply_at("footnote", …)` por ocorrência; indexar em `kind_index`.
2. **`native_counter` aceita a função de elemento**: acrescentar `native_footnote` à
   tabela de `fn_addr_eq` em `stdlib/counter.rs:47-58`, mapeando para a chave
   `"footnote"`.
3. **`layout/footnote.rs` lê do introspector**, não do campo local:
   `flat_counter_at("footnote", current_location)`, com fallback ao comportamento actual
   na primeira iteração do fixpoint (como `table.rs:37-40` faz com `.unwrap_or(1)`).
4. **Decidir explicitamente o destino de `footnote_counter`**: removê-lo do `Layouter`,
   ou mantê-lo como caminho de render e usar o introspector só para a consulta. O P461
   escolheu o introspector como fonte única para a tabela; a simetria sugere o mesmo, mas
   é decisão a registar, não a presumir.

## Restrições Estruturais

- L1 puro; sem 2-pass novo — o fixpoint existente é o mecanismo.
- `pending_footnote_bodies` e o flush em `new_page`/`finish` **não** mudam: é o
  posicionamento das notas, ortogonal à numeração.

## Critérios de Verificação

```
A#footnote[uma] B#footnote[duas]
#context counter(footnote).get()        → (2,)      // hoje: erro
marcadores no corpo                     → [1] [2]   // inalterado
notas no rodapé                         → [1] uma / [2] duas   // inalterado
#set page(columns: 2) + footnotes       → p552 continua verde
counter("footnote").get()               → (0,)      // string continua contador de utilizador
```

## Não-regressão obrigatória

`p552_footnote_counter_avanca_em_set_page_columns` e todos os testes de posicionamento
de nota, sem alteração de expectativa.
```

---

## O que fica em aberto para ti

1. **Gate do footnote** — confirmas o L0 acima (e o ponto 4, que é a decisão real:
   `footnote_counter` desaparece do `Layouter` ou fica como caminho de render)?
2. **Numeração deste passo** — 1016, ou renumerar para não deixar buraco?
3. **Achado colateral não tocado**: `counter(footnote)` erra hoje com *"recebeu
   function"*, mensagem que não distingue "não é função de elemento" de "é função de
   elemento mas ainda não tem contador". Se o gate for dado, o ponto 2 do L0 resolve-o de
   passagem; se não for, a mensagem merece ser mais precisa por si só.
