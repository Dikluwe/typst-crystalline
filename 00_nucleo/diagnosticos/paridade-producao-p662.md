# Relatório de Paridade — P662

**Passo:** 662  
**Data:** 2026-07-09  
**Foco:** Reverter `variant: (eixo: valor)` porque é uma extensão de linguagem não suportada pelo vanilla.  
**Dependências:** P660 (onde a sintaxe foi introduzida por engano), P659 (correcção de cache de eixos internos).  
**Hash do commit com as alterações:** `PENDING`

---

## 1. Contexto

P660 implementou `font: (family: "...", variant: (eixo: valor))` com o objectivo de expor eixos de fontes variáveis. Durante a sonda de P660 verificou-se que o vanilla em quarentena (`lab/typst-original/target/release/typst`) rejeita essa construção:

```text
error: unexpected key 'variant', in dict</text>
```

Isto transforma a funcionalidade numa **extensão de linguagem**: um documento `.typ` que use `variant: (...)` compila no cristalino mas falha no Typst real. O projecto só aceita diferenças de *implementação* (caches, algoritmos, estruturas internas), não diferenças de *linguagem*, porque estas quebram a portabilidade dos documentos. Decidiu-se reverter.

---

## 2. Implementação

### 2.1 Reversão de P660

Reverteu-se o commit de implementação de P660 (`2187dabeb`) com `git revert --no-commit 2187dabeb`. Isto removeu:

- O tipo `FontAxisValue` e o campo `axes` de `FontFamily` (`01_core/src/entities/font_list.rs`).
- O campo `font_axes` de `TextStyle` (`01_core/src/entities/layout_types.rs`).
- A inicialização de `font_axes: None` na conversão `StyleChain → TextStyle`.
- O reconhecimento de `variant` como `Dict` no parser de dicionário de fonte (`01_core/src/rules/eval/rules.rs`).
- A decodificação de `"axes"` e a propagação de `font_axes` no layout de texto.
- A passagem de eixos explícitos em `axis_variations_for_font_variant` (`03_infra/src/font_variant.rs`, `03_infra/src/shaper.rs`).
- A expansão da chave de coleta/embed de fontes de `(FontList, FontVariant)` para `(FontList, FontVariant, Vec<(EcoString, FontAxisValue)>)`.
- Os testes específicos de P660 em `01_core/src/rules/eval/tests.rs`, `01_core/src/rules/layout/tests.rs` e `03_infra/src/shaper.rs`.

### 2.2 Correcções mantidas

Apesar da reversão, duas correcções independentes foram preservadas:

1. **Merge de `font` em `layout/text.rs`**: a linha `font: ns_font.or(layouter.style.font.clone())` foi re-aplicada após o revert. Esta correcção faz com que `#set text(font: (...))` substitua a fonte default da chain (`Liberation Serif`). Não depende da sintaxe `variant: (...)`. Caminho: `01_core/src/rules/layout/text.rs`.

2. **Correcção de P659 na cache de `shaped_width`**: a chave continua a incluir `axis_hash`, calculado a partir das variações de eixo derivadas de `weight`/`style`/`stretch`. Esta correcção continua necessária mesmo sem eixos explícitos, porque `weight`/`style`/`stretch` já produzem eixos internamente. Caminho: `03_infra/src/font_metrics.rs`.

### 2.3 Actualização do histórico de P660

Adicionou-se uma nota póstuma em `00_nucleo/diagnosticos/paridade-producao-p660.md` a explicar que a sintaxe `variant: (eixo: valor)` foi criada por erro de verificação e revertida em P662.

---

## 3. Validação

### 3.1 Testes

```bash
cargo test --workspace --lib
```

Resultado: `606 passed; 0 failed; 5 ignored`.

### 3.2 Build

```bash
cargo build --workspace
```

Resultado: sucesso (apenas warnings preexistentes).

### 3.3 Linter

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

### 3.4 Confirmação de que a sintaxe foi removida

Documento de teste:

```typst
#set text(font: (family: "Cantarell", variant: (wght: 100)))
Hello
```

Compilação com o cristalino:

```text
error: unknown font dict field: variant
```

Comportamento idêntico ao vanilla de referência (que reporta `unexpected key 'variant'`).

---

## 4. Decisão

- Reverteu-se a sintaxe `variant: (eixo: valor)` introduzida em P660.
- Manteve-se a correcção de merge de `font` em `layout/text.rs`.
- Manteve-se a correcção de P659 na chave de cache de `shaped_width`.
- A linguagem Typst suportada pelo cristalino volta a estar alinhada com o vanilla de referência no que toca a dicionários de fonte.
