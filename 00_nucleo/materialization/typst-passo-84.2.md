# Passo 84.2 — Cache de sub-frames no Grid Auto (DEBT-38)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/layout/mod.rs` — braço `Content::Grid`, onde o
  cache vai viver. Linha aproximada 684 (fase de medição Auto) e a
  fase de emissão de células.
- `00_nucleo/DEBT.md` — entrada DEBT-38 criada no Passo 83.
- ADR-0029, ADR-0030 — definem que este trabalho é domínio de L1,
  não "optimização prematura".

Pré-condição: `cargo test` — 901 testes, zero violations. Passo 84.1
concluído (DEBT.md limpo). DEBT-38 está em Secção 1 do DEBT.md.

---

## Restrições arquiteturais

Três regras do projecto que este passo respeita por construção:

1. **ADR-0029 — Pureza física de L1**: alocações de RAM são domínio,
   não I/O. `std::collections::HashMap` é permitida em L1 sem entrar
   em `[l1_allowed_external]` (stdlib isenta, como `std::*`, `core::*`,
   `alloc::*` por regra do V14).

2. **ADR-0030 — Performance de RAM é domínio**: evitar trabalho
   duplicado no percurso do Grid é **comportamento correcto**, não
   refactor cosmético. A justificativa para este passo é directamente
   a da ADR-0030:
   > "Um compilador que copia árvores O(n) quando podia partilhá-las
   > via Arc não é mais puro — é incorrectamente lento."

   Aqui, um compilador que executa `layout_sub_frame_with_width` duas
   vezes por célula Auto (medir + emitir) não é mais puro — é
   incorrectamente lento.

3. **Sem `unsafe` em L1** (convenção cristalina, consistente com
   ADR-0004, ADR-0005, ADR-0019, ADR-0029). `ptr::addr_of(cell) as
   usize` como chave de cache seria `unsafe` via `as` cast de
   ponteiro — excluído por esta regra. A chave é `usize` derivado
   do índice da célula na iteração, não do endereço.

---

## Contexto

O Passo 83 implementou `rows` e alinhamento vertical no Grid. A fase
de medição Auto chama `layout_sub_frame_with_width` por cada célula
para descobrir a altura intrínseca da linha:

```rust
TrackSizing::Auto => {
    let mut max_h: f64 = 0.0;
    for (col_idx, item) in row_items.iter().enumerate() {
        let cell_w = column_widths[col_idx];
        let (sub_h, _sub_items) = self.layout_sub_frame_with_width(
            item,
            cell_w,
        );
        // _sub_items DESCARTADO aqui
        if sub_h > max_h {
            max_h = sub_h;
        }
    }
    // ...
}
```

Os `_sub_items` são descartados. Minutos depois, na fase de emissão
de células, `layout_sub_frame_with_width` é chamada **de novo** para
o mesmo item com a mesma largura — reproduzindo exactamente os mesmos
`FrameItem`.

Para um Grid com N células Auto, o custo de layout é 2N em vez de N.
Em documentos com grelhas grandes (tabelas financeiras, listas de
figuras, caixas compostas) o custo dobra sem benefício.

---

## Decisão: cache local com `HashMap<usize, (f64, Vec<FrameItem>)>`

Uma variável local ao braço `Content::Grid` que cacheia o resultado
da medição. A chave é o índice sequencial da célula na iteração do
Grid. O valor é o par `(altura, items)` já calculado.

### Porquê chave `usize` (índice da célula) e não `&Content` ou ponteiro

Duas alternativas foram consideradas:

- **`ptr::addr_of(cell) as usize`**: endereço do `&Content`. Requer
  `unsafe` via `as` cast — excluído pela convenção cristalina.
- **Hash do `Content`**: o Passo 83 não exige `Hash` em `Content` e
  ADR-0031 só formaliza early hashing para `Source`. Calcular hash
  por célula contradiria o objectivo (evitar trabalho extra).

O índice da célula na iteração é estável dentro do braço `Content::Grid`
(a matriz de items não é reordenada entre medição e emissão), é
`usize` puro sem `unsafe`, e é derivado do iterador que já existe.

### Porquê variável local e não campo do Layouter

- **Escopo natural**: o cache sai de escopo quando o braço termina.
  Sem invalidação manual, sem risco de estado residual entre Grids.
- **Grids aninhados**: cada braço cria o seu próprio cache local.
  A recursão acontece dentro de `layout_sub_frame_with_width`, que
  chama `layout_content`, que volta a entrar no braço `Content::Grid`
  com o seu próprio cache. Zero conflitos.
- **Facilita atomização**: se um dia este cache for movido para outro
  sítio (por exemplo, partilhado entre passagens de um fixpoint),
  a mudança é cirúrgica — variável local → campo. Começar como
  variável local é o caminho de menor acoplamento.

### Porquê `Vec<FrameItem>` no valor e não `Arc<[FrameItem]>`

O cache é transitório e o `Vec` é transferido uma única vez (do cache
para o frame final) — nunca partilhado entre múltiplos proprietários.
`Arc<[FrameItem]>` seria prematuro aqui. A revisão da ADR-0026
aplica-se a `Content::Sequence` (partilhado entre ramos de eval), não
a caches locais.

---

## Tarefa 1 — Estrutura do cache

Dentro do braço `Content::Grid` em `layout_content`, após a extracção
dos campos `columns`, `rows`, `items` e antes da fase 1 de medição:

```rust
// Cache local para DEBT-38 (Passo 84.2).
// Chave: índice sequencial da célula na iteração (row_idx * num_cols + col_idx).
// Valor: (altura intrínseca, items produzidos).
// Motivação: evitar segunda chamada a layout_sub_frame_with_width
//            na fase de emissão. Sai de escopo no fim deste braço.
use std::collections::HashMap;
let mut cell_cache: HashMap<usize, (f64, Vec<FrameItem>)> = HashMap::new();
```

O uso de `use` local em vez de no topo do ficheiro é intencional —
confina a importação ao braço e sinaliza a quem lê o código que
`HashMap` é usada apenas aqui, não no resto do Layouter.

Se já existir `use std::collections::HashMap;` no topo do ficheiro
(por outras estruturas do Layouter), omitir o `use` local e apenas
declarar `cell_cache`.

---

## Tarefa 2 — Popular o cache na fase 1 (medição)

Alterar apenas o braço `TrackSizing::Auto` da fase 1. Outros braços
(`Fixed`, `Fraction`) não medem, portanto não tocam o cache.

**Antes:**

```rust
TrackSizing::Auto => {
    let mut max_h: f64 = 0.0;
    for (col_idx, item) in row_items.iter().enumerate() {
        let cell_w = column_widths[col_idx];
        let (sub_h, _sub_items) = self.layout_sub_frame_with_width(
            item,
            cell_w,
        );
        if sub_h > max_h {
            max_h = sub_h;
        }
    }
    total_fixed_and_auto += max_h;
    max_h
},
```

**Depois:**

```rust
TrackSizing::Auto => {
    let mut max_h: f64 = 0.0;
    for (col_idx, item) in row_items.iter().enumerate() {
        let cell_w = column_widths[col_idx];
        let (sub_h, sub_items) = self.layout_sub_frame_with_width(
            item,
            cell_w,
        );
        // DEBT-38 (Passo 84.2): guardar o resultado para a fase de emissão.
        // Chave: índice linear da célula na iteração.
        let cell_idx = row_idx * num_cols + col_idx;
        cell_cache.insert(cell_idx, (sub_h, sub_items));

        if sub_h > max_h {
            max_h = sub_h;
        }
    }
    total_fixed_and_auto += max_h;
    max_h
},
```

Três alterações específicas:

1. `_sub_items` (descartado) → `sub_items` (retido).
2. Cálculo do `cell_idx` usando o índice linear da célula.
3. `cell_cache.insert(cell_idx, (sub_h, sub_items))`.

**Importante**: `cell_idx` usa as mesmas fórmulas que a fase de
emissão precisa de usar para consultar o cache. Se a fase 1 calcula
o índice de uma forma e a fase 2 de outra, o cache falha silenciosamente
(cache miss em todas as consultas, comportamento idêntico ao estado
antes deste passo). A fórmula `row_idx * num_cols + col_idx` é a
convenção. Mantê-la consistente nas duas fases.

---

## Tarefa 3 — Consultar o cache na fase de emissão

Localizar a fase de emissão de células no braço `Content::Grid`.
Esta é a fase que efectivamente desenha cada célula no frame final —
a segunda chamada a `layout_sub_frame_with_width` descrita no
DEBT-38.

A alteração é: **antes** de chamar `layout_sub_frame_with_width`,
verificar se o resultado já está em `cell_cache`. Se estiver,
reutilizar. Se não, cair para a chamada original (mantém compatibilidade
com células de linhas `Fixed` e `Fraction`, que não foram medidas na
fase 1 e portanto não estão no cache).

**Esqueleto do padrão a aplicar:**

```rust
// No loop de emissão de células do Grid, para cada (row_idx, col_idx):

let cell_idx = row_idx * num_cols + col_idx;

// DEBT-38 (Passo 84.2): consultar cache antes de relayoutar.
// Cache hit: reutilizar resultado da fase de medição.
// Cache miss: célula de linha Fixed ou Fraction — medir agora.
let (sub_h, sub_items) = match cell_cache.remove(&cell_idx) {
    Some(cached) => cached,
    None => self.layout_sub_frame_with_width(item, cell_w),
};

// Resto da lógica de emissão (posicionamento, VAlign, etc.) inalterada.
// ...
```

Duas notas sobre o padrão:

1. **`cell_cache.remove(&cell_idx)` em vez de `cell_cache.get(&cell_idx)`**:
   remover consome o `Vec<FrameItem>` do cache em vez de o clonar.
   Cada célula é emitida exactamente uma vez, portanto o cache nunca
   precisa de ser consultado duas vezes para a mesma chave. `remove`
   evita uma cópia O(n) dos `FrameItem`.

2. **Cache miss é comportamento correcto, não erro**: células de linhas
   `Fixed` ou `Fraction` nunca entraram no cache (porque a fase 1 não
   as mediu — a altura era conhecida directamente). O `match` com braço
   `None` cai para a chamada de layout tal como antes deste passo.
   Sem `unwrap`, sem `panic`.

---

## Tarefa 4 — Verificar que o `use` não contamina L1

`std::collections::HashMap` é stdlib — não dispara V14 (whitelist de
externos). V3 (imports de camadas internas) não se aplica a `std::*`.
V13 (estado global mutável) não se aplica — a `HashMap` é local,
não `static`.

```bash
# Confirmar que nenhuma violação aparece.
crystalline-lint .

# Especificamente, o ficheiro alterado.
crystalline-lint 01_core/src/rules/layout/mod.rs

# Confirmar que std::collections::HashMap não aparece em lugares
# inadequados (deveria ser apenas dentro do braço Grid ou no topo
# do ficheiro se já lá estava).
grep -n "HashMap" 01_core/src/rules/layout/mod.rs
```

Resultado esperado:
- Zero violations.
- `HashMap` aparece uma vez no braço `Content::Grid` (ou no topo do
  ficheiro, se o `use` foi colocado lá por convenção local — ver
  Tarefa 1).

---

## Tarefa 5 — Testes de regressão

O cache é uma optimização interna. O comportamento observável (posições
X/Y dos items no frame final, tamanhos, paginação) deve ser **idêntico**
ao do Passo 83. Os três testes do Passo 83 devem continuar a passar
sem alteração:

- `grid_rows_fixed_coordenadas_y_correctas` — linhas `Fixed`, não toca
  o cache.
- `grid_valign_bottom_ancora_ao_limite_inferior_da_celula` — linha
  `Fixed(100pt)`, não toca o cache.
- `grid_rows_auto_e_fraction_coexistem` — **toca o cache** (primeira
  linha é `Auto`). É o teste chave de regressão para este passo.

Adicionalmente, adicionar um teste que exercita múltiplas células Auto
para confirmar que o cache funciona com mais de uma célula:

```rust
#[test]
fn grid_auto_com_multiplas_celulas_reutiliza_cache() {
    // Grid 2x2 com todas as linhas Auto. Cada célula contém um rect
    // com altura distinta para forçar a medição a importar.
    // Verifica que as 4 células aparecem no frame e que as coordenadas
    // Y correspondem ao máximo de cada linha.
    //
    // Linha 0: rects de 30pt e 50pt → altura da linha = 50pt.
    // Linha 1: rects de 20pt e 40pt → altura da linha = 40pt.
    //
    // Esperado:
    // - Item 0 (linha 0, col 0, altura 30pt) em y = 20pt (margem).
    // - Item 1 (linha 0, col 1, altura 50pt) em y = 20pt.
    // - Item 2 (linha 1, col 0, altura 20pt) em y = 20 + 50 = 70pt.
    // - Item 3 (linha 1, col 1, altura 40pt) em y = 70pt.
    let root = criar_dir_temporario();
    std::fs::write(root.join("main.typ"), "\
        #set page(width: 400pt, height: 400pt, margin: 20pt)\n\
        #grid(columns: 2,\n\
          rect(width: 100pt, height: 30pt),\n\
          rect(width: 100pt, height: 50pt),\n\
          rect(width: 100pt, height: 20pt),\n\
          rect(width: 100pt, height: 40pt),\n\
        )\n\
    ").unwrap();

    let world  = SystemWorld::new(&root, "main.typ", &[]);
    let src    = world.source(world.main()).unwrap();
    let module = eval(&world, &src).unwrap();
    let state  = introspect(module.content());
    let doc    = layout(module.content(), state);

    let items = &doc.pages[0].items;
    assert_eq!(items.len(), 4, "Deve haver 4 FrameItems");

    let y0 = frame_item_pos(&items[0]).y.0;
    let y1 = frame_item_pos(&items[1]).y.0;
    let y2 = frame_item_pos(&items[2]).y.0;
    let y3 = frame_item_pos(&items[3]).y.0;

    // Primeira linha: todos em y = 20pt.
    assert!((y0 - 20.0).abs() < 0.5, "Item 0 em y=20, obteve {:.1}", y0);
    assert!((y1 - 20.0).abs() < 0.5, "Item 1 em y=20, obteve {:.1}", y1);

    // Segunda linha: y = 20 + altura_linha_0 = 20 + 50 = 70pt.
    assert!((y2 - 70.0).abs() < 0.5, "Item 2 em y=70, obteve {:.1}", y2);
    assert!((y3 - 70.0).abs() < 0.5, "Item 3 em y=70, obteve {:.1}", y3);
}
```

Este teste não verifica directamente que o cache foi usado (isso seria
white-box, acoplado à implementação interna). Verifica que o
comportamento com múltiplas células Auto continua correcto — o tipo
de regressão que um cache mal implementado poderia introduzir (por
exemplo, `cell_idx` trocado entre fase 1 e fase 2 daria coordenadas
erradas em células cruzadas).

---

## Tarefa 6 — Encerrar DEBT-38 no DEBT.md

Mover DEBT-38 da Secção 1 (abertos) para a Secção 2 (encerrados).
Entrada actualizada:

```markdown
## DEBT-38 — Cache de sub-frames no Grid Auto — **ENCERRADO (Passo 84.2)** ✓

**Registado no Passo 83.**

A resolução de altura de linhas Auto chamava `layout_sub_frame_with_width`
para medir a altura intrínseca de cada item, descartando os FrameItems
produzidos. Quando a célula era emitida no documento, a mesma função
era chamada de novo para o mesmo item com a mesma largura, duplicando
o trabalho de layout em todas as células Auto.

**Resolvido no Passo 84.2.** Cache local `HashMap<usize, (f64, Vec<FrameItem>)>`
no braço `Content::Grid`, populado na fase de medição Auto e consumido
(via `remove`) na fase de emissão. Chave: `row_idx * num_cols + col_idx`.
Cache sai de escopo no fim do braço — sem invalidação manual.

Não usa `ptr::addr_of` nem cast de ponteiro (`unsafe` excluído pela
convenção cristalina). Não usa `Arc<[FrameItem]>` no valor (transferência
única do cache para o frame, sem partilha). `std::collections::HashMap`
é permitida em L1 por ADR-0029 (pureza física — RAM é domínio, não I/O).
```

---

## Critérios de conclusão

- [ ] Variável local `cell_cache: HashMap<usize, (f64, Vec<FrameItem>)>`
  declarada no braço `Content::Grid` antes da fase 1.
- [ ] Fase 1 (`TrackSizing::Auto`) popula o cache com `insert` usando
  chave `row_idx * num_cols + col_idx`. `sub_items` já não é descartado.
- [ ] Fase de emissão consulta o cache com `remove` antes de chamar
  `layout_sub_frame_with_width`. Cache miss cai silenciosamente para
  a chamada original (comportamento correcto para linhas `Fixed`/`Fraction`).
- [ ] Nenhum uso de `unsafe`, `ptr::addr_of`, ou cast de ponteiro.
- [ ] `std::collections::HashMap` usada sem `[l1_allowed_external]`
  (stdlib isenta).
- [ ] Cache sai de escopo no fim do braço — sem campo novo no Layouter,
  sem `static`, sem `Arc`.
- [ ] Os 3 testes de Grid do Passo 83 passam sem alteração.
- [ ] Novo teste `grid_auto_com_multiplas_celulas_reutiliza_cache` passa.
- [ ] DEBT-38 movido para Secção 2 do DEBT.md com nota de resolução.
- [ ] `cargo test` mantém o número de testes do Passo 84.1 +1 novo
  teste L1 = 733 L1 + 169 L3.
- [ ] `crystalline-lint .` zero violations.

---

## Ao terminar, reportar

- Confirmação de que o cache é variável local (não campo do Layouter).
- Confirmação de que a chave é `usize` derivado do índice da célula
  (sem `ptr::addr_of`, sem hash).
- Confirmação de que `remove` é usado na fase de emissão (não `get`
  que clonaria).
- Coordenadas Y validadas no teste novo: y0=20, y1=20, y2=70, y3=70.
- Número total de testes após o passo e zero violations.

**Go/No-Go para o Passo 84.3** (DEBT-21, `fn_addr_eq` em selectors):

- **GO — cache funciona transparentemente**: os 3 testes do Passo 83
  passam sem alteração + o teste novo passa. O comportamento observável
  é idêntico, o ganho é interno.
- **NO-GO — cache troca de coordenadas entre células**: se o teste
  novo falha com coordenadas cruzadas (ex: item 0 aparece onde item 1
  devia), a fórmula `row_idx * num_cols + col_idx` não está consistente
  entre a fase 1 e a fase de emissão. Verificar que as duas fases
  usam literalmente a mesma expressão.
- **NO-GO — panic em `cell_cache.remove(...).unwrap()`**: se o código
  da Tarefa 3 foi implementado com `unwrap` em vez de `match`, células
  de linhas `Fixed`/`Fraction` causam panic (nunca entraram no cache).
  Voltar ao `match` com braço `None` caindo para `layout_sub_frame_with_width`.
