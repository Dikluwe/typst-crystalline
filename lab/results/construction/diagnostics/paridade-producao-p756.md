# Relatório de Paridade — P756

**Passo:** 756  
**Data:** 2026-07-14  
**Foco:** Integrar `icu_segmenter` para segmentação de linha de scripts sem espaços (CJK, Thai, Lao, Myanmar, Khmer).  
**Hash base:** `0a17e16a53f5f96ed10f4cf2adab0c0b7878ca6e`  
**Hash do commit com as alterações de código:** `877528b5a611cdf5fdb736259289b2bd459663dc`.  
(Nota: o commit final que inclui este relatório terá um hash diferente, pois o relatório faz parte do conteúdo commitado.)

---

## 1. O que foi implementado

### 1.1 Sonda L1/L3

Confirmado directamente que `icu_segmenter 2.2.0` com a feature `compiled_data` embute as tabelas Unicode e o modelo LSTM como constantes estáticas em tempo de compilação. Não há `std::fs`, `include_bytes` em runtime nem outro I/O. O mecanismo é análogo ao blob `typst_assets::icu::ICU_CJ_SEGMENT` do vanilla (também estático). Decisão: fica em L1, declarado em `[l1_allowed_external]`.

### 1.2 Dependências

- `Cargo.toml` (workspace): adicionado `icu_segmenter = { version = "2.2.0", features = ["compiled_data", "lstm"] }`.
- `01_core/Cargo.toml`: adicionada a dependência.
- `crystalline.toml`: adicionado `icu_segmenter` a `[l1_allowed_external]`.
- `Cargo.lock`: actualizado automaticamente.

### 1.3 Código

Ficheiro `01_core/src/engine/layout/cursor.rs`:

- `layout_word` detecta runs que contenham scripts sem espaços (`Han`, `Hiragana`, `Katakana`, `Thai`, `Lao`, `Myanmar`, `Khmer`) e delega a `layout_segmented_word`.
- `layout_segmented_word` obtém breakpoints via `icu_segmenter::LineSegmenter::new_lstm` e emite cada fragmento via `layout_chunk`.
- Tailoring de aspas CJK (`U+201C`/`U+201D`) implementado por pós-processamento dos breakpoints quando `lang` for `zh`/`ja` ou o run contiver contexto CJK, replicando o efeito do `CJ_SEGMENTER` do vanilla ao nível dos pontos de quebra permitidos.
- O segmentador é reconstruído a cada invocação para evitar estado global mutável (`LazyLock` rejeitado pelo linter V13).

### 1.4 L0

`00_nucleo/prompts/engine/layout.md` foi actualizado com a secção "Segmentação de linha para scripts sem espaços (P756)". Hash do código no L0: `849ec756`.

### 1.5 Testes

Adicionados em `01_core/src/engine/layout/tests.rs`:

- `p756_cjk_segmenta_sem_espacos`
- `p756_thai_segmenta_sem_espacos`
- `p756_cjk_aspas_renderiza_sem_panic`

## 2. Validação

```text
cargo test --workspace
  4119 passed (typst-core)
   635 passed (typst-infra)
    33 passed (typst-shell)
     2 passed (typst-wiring)
    27 passed (cli integration)
     2 passed (crystalline-lint integration)
     0 failed
crystalline-lint .
  ✓ No violations found
```

## 3. Resultados de paridade observáveis

Testes manuais com a CLI de release (`./target/release/typst`) e comparação com Typst vanilla (`/usr/local/bin/typst`):

### 3.1 CJK sem aspas (`/tmp/p756-cjk-plain.typ`)

Cristalino e vanilla quebram ambos o texto longo em várias linhas. A posição exacta das quebras difere porque o vanilla usa Knuth-Plass com penalidades globais, enquanto o cristalino mantém o algoritmo greedy existente. Não há truncamento nem fugas para além da margem.

### 3.2 CJK com aspas (`/tmp/p756-cjk.typ`)

O cristalino evita truncamento, mas numa página muito estreita (`100 pt`) a aspa de abertura ainda pode cair no início de uma linha visual quando o fragmento que a contém não cabe na linha anterior. O vanilla consegue evitar isso graças ao Knuth-Plass + `CJ_SEGMENTER` customizado, que optimiza globalmente.

### 3.3 Thai (`/tmp/p756-thai.typ`)

A segmentação ocorre (o texto Thai longo é dividido), mas o rendering com fontes reais apresenta diferenças visuais significativas face ao vanilla. A análise indica que o problema principal está no shaping/fallback de fontes (fora do scope de P756), não na decisão de quebra propriamente dita.

## 4. Limitações e scope-out

1. **Algoritmo greedy vs Knuth-Plass:** o cristalino não implementa Knuth-Plass. Em casos apertados, o layout CJK/Thai pode não coincidir visualmente com o vanilla, mesmo quando a segmentação é semanticamente válida. Melhorar o algoritmo de quebra de linha é scope-out.
2. **Aspas CJK em espaço apertado:** o tailoring de breakpoints evita quebras proibidas, mas o greedy layout pode ainda colocar aspas no início de linha quando o fragmento que as contém é maior que o espaço restante. Uma heurística de kinsoku mais sofisticada pode ser abordada num passo futuro (P758).
3. **Shaping/fallback Thai:** diferenças de posicionamento de glifos/marcas não são corrigidas por este passo.
4. **Largura em `em`:** o documento de validação `p756-cjk-aspas.typ` usa `#set page(width: 7em)`; o cristalino actualmente resolve este valor como `0 pt` (problema pré-existente de resolução de lengths, fora do scope de P756). Os testes manuais usaram `pt` explicitamente.

## 5. Decisão

P756 está fechado na sua fronteira: segmentação de linha para scripts sem espaços integrada em L1, testada, sem regressões em `cargo test --workspace` e `crystalline-lint .` limpo. As diferenças visuais residuais face ao vanilla são atribuídas ao algoritmo greedy pré-existente e ao shaping/fallback de fontes, documentadas como limitações.
