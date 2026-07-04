# Paridade de Produção — Passo 555

**Data:** 2026-07-03  
**Repositório:** `typst-crystalline`  
**Binários:**

- Cristalino: `./target/release/typst` (após P554)
- Vanilla 0.15.0: `lab/typst-original/target/release/typst`
- Ferramentas: `cargo`, `fc-list`, `pdfinfo`, `pdftotext`

---

## 1. Objetivo

P554 mudou a fonte por defeito do cristalino para `FreeSerif`. Este passo faz duas coisas:

1. Correr a **bateria completa de paridade** (não só `cargo test --workspace`) para detectar regressões causadas pela troca de fonte.
2. Garantir que, se `FreeSerif` não estiver disponível num ambiente, o fallback continue a preferir **fontes serif**, preservando a classe visual escolhida, em vez de saltar imediatamente para sans-serif.

---

## 2. Parte 1 — Bateria completa

### 2.1 `cargo test` no workspace cristalino

```bash
cargo test --workspace
```

Resultado: todos os testes passaram nas várias corridas de validação (incluindo a corrida final: 3568 + 577 + 24 + 2 + 21 + 2 passed; 0 failed). O teste `p307b_07_multi_feature` é flaky pré-existente; numa corrida falhou, noutras passou, e não foi introduzido nem agravado por P554/P555.

### 2.2 `cargo test` no pacote de paridade

```bash
cd lab/parity && cargo test
```

Resultado:

```text
11 passed   (consolidado_p206d)
50 passed   (eval_parity)
34 passed   (structural_parity)
2 passed    (vanilla_cli_smoke)
```

Total: **97 testes passados, 0 falhados**.

### 2.3 Compilação manual do corpus

```bash
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ \
         lab/parity/corpus/p520/*.typ lab/parity/corpus/p523/*.typ \
         lab/parity/corpus/p538i/*.typ; do
  ./target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1 && echo "OK" || echo "FAIL"
done
```

Resultado: **41/41 documentos compilaram com sucesso**.

### 2.4 Comparação com estado pré-P554

Não existe um relatório de paridade recente com comparação vanilla agregada (o `lab/parity/reports/latest.md` data de P150 e usa baseline cristalino-only). No entanto, as sentinelas de paridade do pacote `typst-parity` (p479–p488) comparam query outputs com o vanilla 0.15.0 e **passaram com 0 diffs**. A compilação manual do corpus também não revelou novos panics. Logo, **não houve regressão estrutural detectável** pela bateria existente.

---

## 3. Parte 2 — Fallback por classe (serif / sans)

### 3.1 Sonda

`03_infra/src/shaper.rs:83-89` e `03_infra/src/font_metrics.rs:294-300` definiam ambos a mesma lista única de fallback:

```rust
["DejaVu Sans", "Noto Sans", "Liberation Sans", "FreeSans", "Arial"]
```

**Conclusão:** não havia distinção de classe. Se `FreeSerif` faltasse, o sistema caía imediatamente em sans-serif, desfazendo o propósito de P554.

### 3.2 Implementação

Criado `03_infra/src/fallback_fonts.rs` com:

```rust
const DEFAULT_FALLBACK_FONTS_SERIF: &[&str] = &[
    "FreeSerif", "DejaVu Serif", "Liberation Serif", "Bitstream Vera Serif",
];
const DEFAULT_FALLBACK_FONTS_SANS: &[&str] = &[
    "DejaVu Sans", "Noto Sans", "Liberation Sans", "FreeSans", "Arial",
];
```

Heurística de classe a partir do nome da primeira família declarada:

- Contém `"serif"` (case-insensitive) → lista serif.
- Contém `"sans"` ou não é possível inferir → lista sans.

Módulos afectados:

- `03_infra/src/shaper.rs`: `try_shape` usa `fallback_font_list_for` quando as primárias não resolvem.
- `03_infra/src/font_metrics.rs`: `FallbackFontMetrics::resolve_primary` usa a mesma função para manter métricas alinhadas com a face efectiva.
- `03_infra/src/lib.rs`: adicionado `pub mod fallback_fonts`.

### 3.3 Teste do fallback

Documentos:

```typst
#set text(font: "NonExistent Serif")
#lorem(1200)
```

```typst
#set text(font: "NonExistent Sans")
#lorem(1200)
```

Resultado:

| Fonte pedida | Páginas | Classe efectiva |
|---|---|---|
| `"NonExistent Serif"` | **2** | serif (fallback por classe) |
| `"NonExistent Sans"` | **3** | sans-serif |

Confirmado: quando a primária é classificada como serif, o fallback oferece outras serifas primeiro.

---

## 4. Validação final

### 4.1 Paginação de P553/P554

```typst
#set page(columns: 2)
#lorem(1200)
```

| Versão | Páginas | Palavras |
|---|---|---|
| Cristalino (após P555) | **2** | 1200 |
| Vanilla 0.15.0 | **2** | 1213 |

### 4.2 Comandos de validação

```bash
cargo build --release        # ok
cd lab/parity && cargo test  # 97 passed, 0 failed
cargo test --workspace       # todos passaram na corrida final
crystalline-lint .           # 0 violations
```

---

## 5. Conclusão

- **Parte 1:** bateria completa corrida; nenhuma regressão estrutural detectada.
- **Parte 2:** fallback dividido por classe (serif/sans); `FreeSerif` ausente não força mais recair em sans-serif.
- **P553/P554 continuam fechados:** `#set page(columns: 2)\n#lorem(1200)` mantém 2 páginas no cristalino.
- Nenhuma actualização de estado no inventário foi necessária: os itens de P553/P554 já estavam marcados como fechados.

---

## 6. Ficheiros de verificação

- `/tmp/p553-cols-long.typ`
- `/tmp/p553-cristalino.pdf`
- `/tmp/p555-serif-fallback.typ`
- `/tmp/p555-serif-fallback.pdf`
- `/tmp/p555-sans-fallback.typ`
- `/tmp/p555-sans-fallback.pdf`

Temporários, não commitados.
