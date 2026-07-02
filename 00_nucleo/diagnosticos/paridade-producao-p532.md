# Relatório de Paridade de Produção — Passo 532

| Campo | Valor |
|-------|-------|
| Passo | P532 |
| Foco | Numeração de página customizada (`#set page(numbering: ...)`) |
| Data | 2026-07-02 |
| Status | Concluído |

---

## Contexto

O Passo 531 (Grupo 8.2) confirmou que `#set page(numbering: "i")` compila sem erro, mas os números de página não aparecem no PDF. Este passo sondou a causa e implementou o fix.

## Sonda

### Perguntas e respostas

1. **A propriedade `numbering:` fica guardada depois de `#set page(...)`?**
   - Não. Em `01_core/src/rules/eval/rules.rs:701-729`, o target `"page"` só extrai `width`, `height` e `margin`. A propriedade `numbering` é lida mas descartada.

2. **Existe código no Layouter que lê essa propriedade?**
   - Não. `Content::SetPage` só transporta `width`, `height`, `margin` (`01_core/src/entities/content.rs:486`). `PageConfig` (`01_core/src/entities/layout_types.rs:432`) e `Page` (`01_core/src/entities/layout_types.rs:456`) também não a guardavam.

3. **Porque não aparecia?**
   - A cadeia estava partida em três sítios:
     1. eval não passava `numbering` para `Content::SetPage`;
     2. layout não guardava `numbering` em `PageConfig`/`Page`;
     3. o export PDF nunca desenhava o número.

4. **`counter(page).display()` em footer funciona?**
   - Não foi necessário testar isoladamente, porque o problema era a numeração automática por defeito, não o mecanismo de contagem. O fix ligou o padrão de numeração ao layout, não ao `counter`.

## Implementação

### Ficheiros alterados

- `01_core/src/entities/content.rs` — adicionado `numbering: Option<EcoString>` a `Content::SetPage`.
- `01_core/src/entities/layout_types.rs` — adicionado `numbering` a `PageConfig` e `Page`; actualizados testes internos.
- `01_core/src/entities/counter_format.rs` — adicionado token `'i'` para numerais romanos minúsculos.
- `01_core/src/rules/eval/rules.rs` — extrai `numbering` de `#set page(...)`.
- `01_core/src/rules/layout/set_page.rs` — propaga `numbering` para `layouter.page_config`.
- `01_core/src/rules/layout/mod.rs` — desenha o número na última página no `finish()`.
- `01_core/src/rules/layout/cursor.rs` — desenha o número em `new_page()` para páginas intermédias.
- `03_infra/src/export/mod.rs` — restaurados imports `cfg(test)` necessários a `export/tests.rs`.
- `03_infra/src/export/tests.rs` — actualizados 81+ construtores de `Page` para incluir `numbering: None`.
- `03_infra/src/pipeline.rs` — actualizado helper de teste `page_with`.
- `03_infra/src/shaper.rs` — actualizado helper de teste `doc_with`.
- `01_core/src/rules/layout/tests.rs` — actualizado construtor de `Content::SetPage`.

### Lógica do fix

Quando `Content::SetPage` muda o `numbering`, o `Layouter.page_config.numbering` é actualizado. Sempre que uma página é fechada (`new_page` ou `finish`), se houver um padrão activo:

1. Formata-se o número da página com `format_counter(&[page_number], pattern)`.
2. Adiciona-se um `FrameItem::Text` centrado horizontalmente no rodapé da página.

A posição vertical usa o sistema de coordenadas do layout (origem no canto superior-esquerdo, Y cresce para baixo), pelo que `y = height - margin/2` coloca o texto perto do fundo da página. O stream PDF inverte Y internamente.

## Validação

### Padrões testados

```typst
#set page(numbering: "i")
Página um.
#pagebreak()
Página dois.
```

| Padrão | Cristalino | Vanilla 0.15.0 |
|--------|------------|----------------|
| `"i"`  | `i`, `ii`  | `i`, `ii`      |
| `"1"`  | `1`, `2`   | `1`, `2`       |
| `"a"`  | `a`, `b`   | `a`, `b`       |

### Testes

| Comando | Resultado |
|---------|-----------|
| `cargo test -p typst-core counter_format` | 9 passed |
| `cargo test -p typst-core layout::tests::` | 564 passed |
| `cargo test -p typst-infra` | 565 passed |
| `crystalline-lint .` | zero violations |

## Limitações

- Apenas os tokens de numeração simples (`1`, `i`, `I`, `a`, `A`) foram testados. Padrões compostos (ex.: `"1 / 1"`, `"I-1"`) requerem a segunda parte do pattern (`counter(page).display()`), que continua como trabalho futuro.
- A posição é fixa (centrado no rodapé); vanilla permite `header`/`footer` customizados, que já existem como funcionalidade separada.
- O número é desenhado via `FrameItem::Text` (fallback), não `TextShaped`. Para documentos com fontes complexas, pode haver diferenças menores, mas para dígitos e letras romanas é suficiente.

## Decisão

Numeração de página customizada está **fechada em P532** para os padrões simples `"i"`, `"1"` e `"a"`.

---

## Reprodução

```bash
# Compilar
cargo build --release --bin typst

# Documento de teste
cat > /tmp/test-page-num.typ <<'EOF'
#set page(numbering: "i")
Página um.
#pagebreak()
Página dois.
EOF
./target/release/typst /tmp/test-page-num.typ /tmp/page-num.pdf
pdftotext /tmp/page-num.pdf -

# Testes e linter
cargo test -p typst-core counter_format
cargo test -p typst-infra
crystalline-lint .
```
