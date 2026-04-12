# Passo 53 — Kern Diferenciado para Left-Scripts (Tipografia Fina)

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/math/layout.rs` — Função `layout_attach` e o cálculo de `left_col_width`
- `03_infra/src/font_metrics.rs` — Método `math_kern` para extracção de `TopLeft` e `BottomLeft`

Pré-condição: `cargo test` — 571 L1 + 107 L3 + 50 parity, zero violations.

---

## Contexto

A dívida técnica do Passo 46 reside na simplificação geométrica dos scripts à
esquerda (pre-scripts). Actualmente, o X da base e os X dos scripts `tl`
(top-left) e `bl` (bottom-left) são calculados assumindo uma coluna esquerda
rígida, resultando no alinhamento à esquerda dos scripts entre si.

O problema tipográfico: se um símbolo base tiver um contorno inclinado (como
um V itálico ou um J), o kern do topo-esquerdo é radicalmente diferente do
kern do fundo-esquerdo. Ao aplicar `max()` globalmente, os scripts à esquerda
ficam alinhados entre si pela esquerda, ignorando o contorno da base.

A tipografia matemática OpenType exige que os scripts se aproximem da base de
forma independente. A fórmula correcta é:

1. `tl_push = tl_width + tl_kern`
2. `bl_push = bl_width + bl_kern`
3. `base_x = max(tl_push, bl_push)`
4. `tl_x = base_x - tl_push`
5. `bl_x = base_x - bl_push`

Isto garante que os scripts "abraçam" a silhueta da base em vez de se
alinharem entre si pela esquerda.

---

## Diagnósticos obrigatórios antes de codificar

```bash
# 1. Localizar o cálculo de left_col_width e a atribuição de X para tl/bl
grep -A 15 "left_col_width" 01_core/src/rules/math/layout.rs
```

Reportar o output antes de continuar.

---

## Tarefa 1 — Refactoração geométrica (L1)

Em `01_core/src/rules/math/layout.rs`, dentro da função `layout_attach`:

Remover o conceito de `left_col_width` como largura única e opaca.

Calcular a distância de afastamento para cada script individualmente:

```rust
// 1. Alturas de conexão (em Design Units)
// tl liga-se pela sua parte inferior (descent); bl liga-se pela sua parte superior (ascent)
let tl_conn_du = tl_box.descent.val() * self.constants.upem / self.size.val();
let bl_conn_du = bl_box.ascent.val() * self.constants.upem / self.size.val();

// 2. Extrair kerns (em Design Units)
let tl_kern_du = base_kern.top_left.kern_at(tl_conn_du);
let bl_kern_du = base_kern.bottom_left.kern_at(bl_conn_du);

// 3. Converter para Pt — mesma fórmula do Passo 44
let tl_kern_pt = Pt(tl_kern_du * self.size.val() / self.constants.upem);
let bl_kern_pt = Pt(bl_kern_du * self.size.val() / self.constants.upem);

// 4. Calcular afastamento (largura do script + kern)
let tl_push = tl_width + tl_kern_pt;
let bl_push = bl_width + bl_kern_pt;

// 5. Determinar a origem X da base e dos left-scripts
let base_offset_x = tl_push.max(bl_push);
let tl_x = base_offset_x - tl_push;
let bl_x = base_offset_x - bl_push;

// ATENÇÃO: Substituir todas as referências antigas a `left_col_width` por
// `base_offset_x` ao posicionar os limites verticais (Under/Over do Passo 49)
// e os scripts da direita (`sup`/`sub`). Sem isto, os limites do ∑ ficam
// desalinhados do símbolo base.
```

As alturas de conexão determinam em que ponto da silhueta da base o kern é
avaliado: o `tl` aproxima-se pela parte de baixo (descent), o `bl` pela
parte de cima (ascent) — mesma lógica dos quadrantes direitos no Passo 44.

Se `tl` ou `bl` estiverem ausentes (`None`), os respectivos `push` são zero
e `base_offset_x` é determinado apenas pelo script presente.

---

## Tarefa 2 — Testes de geometria (L1)

```rust
#[test]
fn left_scripts_tem_posicoes_x_independentes() {
    // Notação padrão Typst: $_y^x A$ — tl=x (sup à esquerda), bl=y (sub à esquerda)
    // Com a fonte standard, confirmar que a lógica não panics com ambos presentes.
    // Se os kerns TopLeft e BottomLeft forem iguais na fonte de teste,
    // tl_x == bl_x é esperado — o teste verifica ausência de panic e PDF não vazio.
    let doc = layout_test("$ _y^x A $");
    assert!(!doc.pages.is_empty());
}

#[test]
fn left_scripts_sem_bl_nao_panica() {
    let doc = layout_test("$ ^x A $");
    assert!(!doc.pages.is_empty());
}

#[test]
fn left_scripts_sem_tl_nao_panica() {
    let doc = layout_test("$ _y A $");
    assert!(!doc.pages.is_empty());
}

// Regressão — testes do Passo 46
#[test]
fn left_scripts_passo46_nao_regride() {
    let doc = layout_test("$ _0^n sum $");
    let text = doc.plain_text();
    assert!(text.contains('n') || text.contains('0'));
}
```

---

## Verificação final

```bash
cargo clippy --fix --allow-dirty --allow-no-vcs
cargo test
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Critérios de conclusão:

- [ ] `layout_attach` aplica coordenadas X independentes a `tl` e `bl` via kerns por quadrante
- [ ] `left_col_width` como valor único está removido ou substituído
- [ ] O teste unitário confirma que a lógica não panics com `tl` e `bl` em simultâneo
- [ ] Todos os testes de regressão dos Passos 46–52 passam
- [ ] Não há regressões no PDF (testes L3 continuam a passar)
- [ ] Zero violations no linter e no clippy

---

## Ao terminar, reportar

**Do diagnóstico:**
- Como `left_col_width` estava escrito (uma variável única ou duas separadas)
- Se os kerns `TopLeft`/`BottomLeft` estavam a ser extraídos ou ignorados no Passo 46

**Da implementação:**
- Se `tl_x != bl_x` foi observável com a fonte de teste actual ou se os kerns são iguais nos dois quadrantes
- Se foi necessário tratar o caso em que `tl` ou `bl` estão ausentes de forma diferente do previsto

**Número total de testes e zero violations.**

**Go/No-Go para Passo 54:**
- **GO — Matrizes matemáticas (`mat(...)`)**: a base de layout em grelha do Passo 51 está disponível; Passo 54 pode implementar `mat` sobre ela
- **NO-GO**: se a remoção de `left_col_width` causou regressões nos testes do Passo 46 que não foram resolvidas
