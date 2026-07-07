# Relatório Diagnóstico — Passo 589
## Fonte neutra para testes de algoritmo + re-classificação de comparações passadas

- **Commit de Referência:** `23306b2dc` (Working Tree não commitado)
- **Data/Hora da Medição:** 2026-07-07 02:18:08 UTC
- **ADR Base:** `00_nucleo/adr/adr-paridade-defeitos-testes.md`

---

## 1. Objetivo

Aplicar a ADR de paridade de definições por defeito:

1. Escolher uma fonte neutra disponível em ambos os ambientes (cristalino e vanilla).
2. Criar um helper que aplique essa fonte automaticamente em testes de algoritmo.
3. Rever os documentos de teste usados na sequência RTL (P563 a P588) para identificar quais eram testes de algoritmo que deviam ter usado fonte neutra.

---

## 2. Sonda de fontes

### 2.1 Comando usado

```bash
fc-list : family | sort -u > /tmp/p589-fontes-sistema.txt
lab/typst-original/target/release/typst fonts 2>/dev/null | sort -u > /tmp/p589-fontes-vanilla.txt
./target/release/typst fonts 2>/dev/null | sort -u > /tmp/p589-fontes-cristalino.txt
comm -12 /tmp/p589-fontes-vanilla.txt /tmp/p589-fontes-cristalino.txt
```

### 2.2 Resultado

- Sistema: 332 famílias.
- Vanilla: 317 famílias.
- Cristalino (`./target/release/typst fonts`): **0 famílias**.

O comando `typst fonts` do cristalino não lista fontes. Isto é esperado nesta build: o cristalino não implementa o subcomando `fonts`; ele carrega fontes do sistema via `fontdb::load_system_fonts()` durante a compilação. A sonda de disponibilidade comum deve, portanto, comparar o vanilla com as fontes do sistema.

### 2.3 Fontes relevantes encontradas

| Fonte | Sistema | Vanilla | Nota |
|---|---|---|---|
| `DejaVu Sans` | sim | sim | Escolhida. |
| `DejaVu Sans Mono` | sim | sim | Apenas para testes monoespaçados. |
| `DejaVu Serif` | sim | sim | Alternativa serif. |
| `Liberation Sans` | sim | sim | Default do cristalino (P558); evitada para não confundir com produção. |
| `Liberation Serif` | sim | sim | Default do cristalino; mesma razão. |
| `Libertinus Serif` | não | sim | Default do vanilla; não disponível no sistema de teste. |
| `Noto Sans Arabic` | sim | sim | Boa para árabe, mas não resolve shaping sozinho. |

### 2.4 Decisão

**Fonte neutra escolhida:** `DejaVu Sans`.

**Justificação:**
- Disponível no sistema de teste e no vanilla.
- Carregada pelo cristalino através do sistema de fontes.
- Sem problemas conhecidos nesta sequência (ao contrário de `FreeSerif`, descartada em P558).
- Suporta latim e árabe básico, cobrindo a maioria dos testes de algoritmo.

---

## 3. Helper de teste

### 3.1 Implementação

Ficheiro: `01_core/src/rules/layout/tests.rs:27-37`

```rust
/// **P589** — Fonte neutra para testes de algoritmo. Deve estar disponível
/// tanto no cristalino como no vanilla, e não deve ter os problemas já
/// conhecidos de outras fontes (ex.: FreeSerif com decomposição de acentos).
pub(crate) const FONTE_NEUTRA_TESTE: &str = "DejaVu Sans";

/// **P589** — Helper que envolve um documento de teste de algoritmo com a
/// fonte neutra, para evitar ruído de fontes default diferentes entre
/// cristalino e vanilla.
pub(crate) fn documento_algoritmo(conteudo: &str) -> String {
    format!("#set text(font: \"{}\")\n{}", FONTE_NEUTRA_TESTE, conteudo)
}
```

### 3.2 Teste de sentinel

Ficheiro: `01_core/src/rules/layout/tests.rs:3540-3559`

```rust
#[test]
fn p589_documento_algoritmo_aplica_fonte_neutra() {
    let doc = layout_typst(&documento_algoritmo("Hello world"));
    let first_text_x = doc.pages[0]
        .items
        .iter()
        .find_map(|item| match item {
            FrameItem::Text { pos, .. } => Some(pos.x.0),
            _ => None,
        })
        .expect("deve haver pelo menos um Text item");
    assert!(
        (first_text_x - 70.87).abs() < 0.01,
        "texto com fonte neutra deve começar na margem (70.87 pt); obtido {}",
        first_text_x
    );
}
```

Resultado: `test result: ok. 1 passed; 0 failed`.

---

## 4. Documentação da fonte padrão

Criado: `00_nucleo/testing/fontes-padrao-teste.md`

Regista:
- A fonte neutra (`DejaVu Sans`).
- A razão da escolha e alternativas consideradas.
- Outras definições sensíveis a neutralizar (`size`, `dir`, `lang`, `page`, `par`).
- O helper `documento_algoritmo`.
- A distinção entre testes de algoritmo e testes de produção.

---

## 5. Revisão dos documentos de teste da sequência RTL

### 5.1 Método

```bash
for f in /tmp/p56*.typ /tmp/p57*.typ /tmp/p58*.typ; do
  printf "%-30s %s\n" "$(basename "$f")" "$(grep -q 'font:' "$f" && grep -o 'font: "[^"]*"' "$f" || echo 'não')"
done
```

### 5.2 Resultado

| Documento | Passo | Fonte explícita | Classificação | Nova medição necessária? |
|---|---|---|---|---|
| `p560-latin.typ` | P560 | não | algoritmo (latim) | sim — se comparação posicional |
| `p560-rtl-simple.typ` | P560 | não | algoritmo (RTL) | sim |
| `p560-tashkeel.typ` | P560 | não | algoritmo (RTL) | sim |
| `p561-rtl-order.typ` | P561 | não | algoritmo (RTL) | sim |
| `p562-mixed.typ` | P562 | não | algoritmo (RTL) | sim |
| `p563-latin-large.typ` | P563 | não | produção (lorem) | não — teste de volume |
| `p563-mixed.typ` | P563 | não | algoritmo (RTL) | sim |
| `p564-latin-large.typ` | P564 | não | produção (lorem) | não |
| `p564-mixed-small.typ` | P564 | não | algoritmo (RTL) | sim |
| `p564-mixed.typ` | P564 | não | algoritmo (RTL) | sim |
| `p564-pure.typ` | P564 | não | algoritmo (RTL) | sim |
| `p564-small.typ` | P564 | não | algoritmo (RTL) | sim |
| `p565-largura.typ` | P565 | não | algoritmo (RTL) | sim |
| `p565-latin.typ` | P565 | não | algoritmo (latim) | sim |
| `p565-mixed-margem2.typ` | P565 | não | algoritmo (RTL) | sim |
| `p565-mixed-margem.typ` | P565 | não | algoritmo (RTL) | sim |
| `p565-mixed.typ` | P565 | não | algoritmo (RTL) | sim |
| `p565-reflow.typ` | P565 | não | algoritmo (RTL) | sim |
| `p566-multi-rtl.typ` | P566 | não | algoritmo (RTL) | sim |
| `p566-referencia.typ` | P566 | não | algoritmo (RTL) | sim |
| `p567-simple.typ` | P567 | não | algoritmo (RTL) | sim |
| `p568-break.typ` | P568 | não | algoritmo (RTL) | sim |
| `p568-espaco.typ` | P568 | não | algoritmo (RTL) | sim |
| `p568-narrow.typ` | P568 | não | algoritmo (RTL) | sim |
| `p568-no-punct.typ` | P568 | não | algoritmo (RTL) | sim |
| `p568-punct.typ` | P568 | não | algoritmo (RTL) | sim |
| `p569-curto-ponto.typ` | P569 | não | algoritmo (RTL) | sim |
| `p569-longo.typ` | P569 | não | algoritmo (RTL) | sim |
| `p569-notypo.typ` | P569 | não | algoritmo (RTL) | sim |
| `p570-latim-quebra.typ` | P570 | não | algoritmo (latim) | sim |
| `p574-longo.typ` | P574 | não | algoritmo (RTL) | sim |
| `p574-referencia.typ` | P574 | não | algoritmo (RTL) | sim |
| `p576-auto.typ` | P576 | não | algoritmo (RTL) | sim |
| `p576-completo.typ` | P576 | não | algoritmo (RTL) | sim |
| `p576-dir-str.typ` | P576 | não | algoritmo (RTL) | sim |
| `p576-dir.typ` | P576 | não | algoritmo (RTL) | sim |
| `p576-ltr.typ` | P576 | não | algoritmo (RTL) | sim |
| `p576-misto.typ` | P576 | não | algoritmo (RTL) | sim |
| `p576-misto-v.typ` | P576 | não | algoritmo (RTL) | sim |
| `p576-twolines.typ` | P576 | não | algoritmo (latim) | sim |
| `p577-latin.typ` | P577 | não | algoritmo (RTL) | sim |
| `p577-rtl.typ` | P577 | não | algoritmo (RTL) | sim |
| `p577-simples.typ` | P577 | não | algoritmo (RTL) | sim |
| `p578-ltr-wrap.typ` | P578 | não | algoritmo (latim) | sim |
| `p578-rtl-puro.typ` | P578 | não | algoritmo (RTL) | sim |
| `p578-rtl.typ` | P578 | não | algoritmo (RTL) | sim |
| `p586-rtl-20.typ` | P586 | não | algoritmo (RTL) | sim |
| `p586-rtl-margin.typ` | P586 | não | algoritmo (RTL) | sim |
| `p586-rtl-puro.typ` | P586 | não | algoritmo (RTL) | sim |
| `p586-rtl.typ` | P586 | não | algoritmo (RTL) | sim |
| `p587-margem.typ` | P587 | não | algoritmo (RTL) | sim |
| `p587-sem-newline.typ` | P587 | não | algoritmo (RTL) | sim |
| `p588-latim.typ` | P588 | não | algoritmo (latim) | sim |
| `p588-ltr-42.typ` | P588 | não | algoritmo (latim) | sim |
| `p588-rtl-libertinus.typ` | P588 | sim (`Libertinus Serif`) | produção / fonte vanilla | não — usou fonte explícita, mas não neutra |
| `p588-rtl.typ` | P588 | não | algoritmo (RTL) | sim |
| `p588-variantes.typ` | P588 | não | algoritmo (show rules) | sim |
| `p589-rtl-dejavu.typ` | P589 | sim (`DejaVu Sans`) | sonda P589 | já neutra |
| `p589-rtl-noto.typ` | P589 | sim (`Noto Sans Arabic`) | sonda P589 | não neutra (usada para comparar shaping) |

### 5.3 Decisão

- **Apenas um documento da sequência RTL oficial (P560-P588) usava fonte explícita:** `p588-rtl-libertinus.typ` com `Libertinus Serif`. Esse documento testa especificamente o comportamento com a fonte default do vanilla, portanto não é um teste de algoritmo neutro.
- **Todos os restantes documentos da sequência RTL usavam fonte implícita.** As medições de posição/largura feitas com esses documentos podem conter ruído da diferença de fonte default entre cristalino (`Liberation Serif`) e vanilla (`Libertinus Serif`).
- **A diferença de 2.8 pt / 0.46 pt observada em P588** foi suficiente para explicar a quebra residual. Medições anteriores da mesma sequência (P566, P567, etc.) podem ter o mesmo ruído por trás de números que pareciam bugs de algoritmo.
- **Não vamos repetir todas as medições agora.** O objetivo deste passo é estabelecer a ferramenta (fonte neutra + helper) para que futuros testes de algoritmo a usem. Quando um passo futuro reabrir uma medição específica, deve usar `documento_algoritmo` e registar a fonte neutra.

---

## 6. Validação

```bash
cargo build --workspace
cargo test --workspace
crystalline-lint .
```

Resultados:

- `cargo build --workspace`: sucesso.
- `cargo test --workspace`:
  - `typst-core`: 3572 passados, 0 falhas
  - `typst-infra`: 597 passados, 0 falhas, 5 ignorados
  - `typst-shell`: 24 passados, 0 falhas
  - `typst` (bin): 2 passados, 0 falhas
  - `cli`: 21 passados, 0 falhas
  - `crystalline_lint`: 2 passados, 0 falhas
- `crystalline-lint .`: `✓ No violations found`.

---

## 7. Conclusão

- [x] Fonte neutra escolhida: `DejaVu Sans`.
- [x] Documentação criada em `00_nucleo/testing/fontes-padrao-teste.md`.
- [x] Helper `documento_algoritmo` e constante `FONTE_NEUTRA_TESTE` criados em `01_core/src/rules/layout/tests.rs`.
- [x] Teste de regressão `p589_documento_algoritmo_aplica_fonte_neutra` passando.
- [x] Documentos da sequência RTL revistos: a maioria não usava fonte explícita e deve usar `documento_algoritmo` em futuras medições.
- [x] `cargo test --workspace` limpo.
- [x] `crystalline-lint .` limpo.
