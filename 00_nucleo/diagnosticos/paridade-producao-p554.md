# Paridade de Produção — Passo 554

**Data:** 2026-07-03  
**Repositório:** `typst-crystalline`  
**Binários:**

- Cristalino: `./target/release/typst` (após P553)
- Vanilla 0.15.0: `lab/typst-original/target/release/typst`
- Ferramentas: `grep`, `fc-list`, `pdfinfo`, `pdftotext`, `mutool` 1.23.10

---

## 1. Objetivo

P553 corrigiu a geometria de colunas e reduziu a diferença de paginação de 5 vs 2 para 3 vs 2 páginas. A diferença restante era da fonte por defeito: o cristalino usava uma fonte sem serifa, enquanto o vanilla usa `Libertinus Serif`. Este passo sonda a questão, decide o caminho e implementa.

---

## 2. Sonda

### 2.1 Fonte por defeito no vanilla

`lab/typst-original/crates/typst-library/src/text/mod.rs:180`:

```rust
#[default(FontList(vec![FontFamily::new("Libertinus Serif")]))]
pub font: FontList,
```

Confirmação: vanilla 0.15.0 renderiza `#lorem(1200)` em `#set page(columns: 2)` com a fonte `FGESCD+LibertinusSerif-Regular-Identity-H`.

### 2.2 Fonte por defeito no cristalino

`01_core/src/entities/style_chain.rs:612-614`:

```rust
font: Some(chain.font().unwrap_or_else(|| {
    FontList::single(EcoString::from("Helvetica"))
})),
```

A família declarada era `Helvetica`, mas como o cristalino não embute fontes e `Helvetica` não estava disponível no sistema, o shaper fazia fallback pelas fontes sans-serif de `DEFAULT_FALLBACK_FONTS` (`03_infra/src/font_metrics.rs:294-300`):

```rust
["DejaVu Sans", "Noto Sans", "Liberation Sans", "FreeSans", "Arial"]
```

O resultado efectivo era uma fonte sans-serif (observado pelo avanço horizontal maior).

### 2.3 Disponibilidade de Libertinus Serif

```bash
fc-list : family | grep -i libertinus
```

Resultado: nenhuma família `Libertinus` instalada no ambiente de testes.

### 2.4 Testes com fontes serif disponíveis

Documento de teste:

```typst
#set page(columns: 2)
#set text(font: "<nome>")
#lorem(1200)
```

| Fonte | Páginas cristalino | Observação |
|---|---|---|
| (default anterior) | 3 | sans-serif via fallback |
| DejaVu Serif | 3 | serif, avanço maior que Libertinus |
| Bitstream Vera Serif | 3 | serif, avanço semelhante a DejaVu |
| FreeSerif | **2** | serif, avanço próximo de Libertinus Serif |

`FreeSerif` (pacote `fonts-freefont-ttf`, comum em sistemas Linux) foi a única fonte serif disponível no ambiente que reproduziu as 2 páginas do vanilla.

---

## 3. Decisão

**Caminho escolhido: 3 — Fonte por defeito diferente mas da mesma classe.**

Razão:

1. **Igualar o vanilla exactamente** (`Libertinus Serif`) não é viável sem embutir a fonte no binário; a família não está instalada no sistema de testes e o cristalino não a inclui.
2. **Manter a fonte actual** deixaria o gap de paginação (3 vs 2) sem justificativa escrita.
3. **`FreeSerif`** é uma serif amplamente disponível, atinge paridade de paginação no caso de teste de P553 e mantém a classe visual do vanilla (serif vs sem serifa).

Se `FreeSerif` não estiver disponível num ambiente específico, o shaper regressa automaticamente ao fallback sans-serif (`DEFAULT_FALLBACK_FONTS`), preservando o comportamento anterior de degradado.

---

## 4. Alterações implementadas

### 4.1 `01_core/src/entities/style_chain.rs`

- Fonte por defeito do bridge `From<&StyleChain> for TextStyle` alterada de `Helvetica` para `FreeSerif`.
- Comentários actualizados para refletir a decisão P554.
- Teste `p483_textstyle_from_chain_font_nunca_none` actualizado: espera `freeserif` em vez de `helvetica`.
- `@prompt-hash` actualizado para `e6d8ee13` (`entities/style_chain.md`).

### 4.2 `00_nucleo/prompts/entities/style_chain.md`

- Adicionada secção "Fonte por defeito (P554)" com a decisão e justificativa.
- Actualizado `Hash do Código` para `e6d8ee13`.

---

## 5. Validação

### 5.1 Paginação de P553

```typst
#set page(columns: 2)
#lorem(1200)
```

| Versão | Páginas | Palavras |
|---|---|---|
| Cristalino (após P554) | **2** | 1200 |
| Vanilla 0.15.0 | **2** | 1213 |

A diferença de paginação desapareceu.

### 5.2 Documento simples sem fonte definida

```typst
Texto sem fonte definida.
```

Renderiza sem erros; a fonte efectiva é `FreeSerif` (paridade visual de classe serif com o vanilla).

### 5.3 Comandos de validação

```bash
cargo build --release        # ok
cargo test --workspace       # 3568 + 572 + 24 + 2 + 21 + 2 passed; 1 flaky pré-existente (p307b_07_multi_feature)
crystalline-lint .           # 0 violations
```

O teste `p307b_07_multi_feature` continua flaky (ordem não-determinística no dicionário de bookmarks); não foi introduzido nem agravado por P554.

---

## 6. Conclusão

- **Sonda completa:** fonte por defeito confirmada em ambos os lados; `Libertinus Serif` não disponível no ambiente cristalino.
- **Decisão tomada:** caminho 3 — `FreeSerif` como fonte por defeito do cristalino, uma serif amplamente disponível que mantém a classe visual do vanilla e atinge paridade de paginação.
- **Implementada e validada:** `#set page(columns: 2)\n#lorem(1200)` passou de 3 para 2 páginas, igualando o vanilla 0.15.0.
- **Inventário actualizado:** item de fonte por defeito registado separadamente do item de fallback ausente (P538e).

---

## 7. Ficheiros de verificação

- `/tmp/p554-default.typ`
- `/tmp/p554-cristalino-default.pdf`
- `/tmp/p553-cols-long.typ`
- `/tmp/p553-cristalino.pdf`
- `/tmp/p553-dejavu-serif.typ`
- `/tmp/p553-dejavu-serif.pdf`
- `/tmp/p553-test.typ` (usado para FreeSerif/Bitstream Vera Serif)

Temporários, não commitados.
