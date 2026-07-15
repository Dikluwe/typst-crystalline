---
# P765a (lote 0) — Sonda profunda + correcção: `#title()` e `symbol()`

> **Passo:** 765a (lote 0)
> **Data:** 2026-07-15
> **Foco:** P765 confirmou três bugs reais de linguagem por comparação directa com o vanilla: `#title()` ausente (`unknown variable: title`), `symbol(...)` sem construtor, e modificadores de símbolo via field access (`sym.arrow.r.filled`) não suportados. Este passo faz a sonda de implementação (API exacta do vanilla, não só o sintoma) e corrige os três, um a um.
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** M — três correcções relacionadas mas com âmbitos distintos (elemento de markup vs sistema de tipo `Symbol`).
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — `#title()` e construtor/modificadores de `symbol` nunca foram tocados no cristalino; sonda obrigatória contra o código-fonte do vanilla antes de qualquer implementação, não só contra o binário.
> **Dependências:** P765 (L0 e sonda de amostra fechados, hash `0683fad7`; achados originais em `lente-falta-migrar-2026-07-15.md`, secção 4.1).

---

## Sonda — `#title()`

```bash
grep -n "TitleElem\|fn title\b" lab/typst-original/crates/typst-library/src/model/title.rs
grep -rn "\"title\"" lab/typst-original/crates/typst-library/src/model/mod.rs
```

Confirmar:
1. `#title()` é um elemento de markup normal (como `#heading()`) ou tem regra de show especial?
2. Que campos aceita (`level`, texto simples, conteúdo rico)?
3. Como se relaciona com `document(title: ...)` — são independentes, ou `#title()` é o que popula o metadado?

```bash
cat > /tmp/p765a-title-vanilla.typ <<'EOF'
#set document(title: "Metadado")
#title("Corpo")
EOF
lab/typst-original/target/release/typst compile /tmp/p765a-title-vanilla.typ /tmp/p765a-title-vanilla.pdf
```

Comparar se o metadado do PDF e o conteúdo renderizado divergem (confirma se são independentes ou ligados).

## Sonda — `symbol()` e modificadores

```bash
grep -n "fn construct\|pub fn symbol\|impl.*Symbol" lab/typst-original/crates/typst-library/src/foundations/symbol.rs | head -30
grep -n "field(" lab/typst-original/crates/typst-library/src/foundations/symbol.rs | head -30
```

Confirmar:
1. Assinatura exacta do construtor (`symbol(base, ...variantes)` — como são passadas as variantes: pares `("bold", "α")`?).
2. Como o field access (`sym.arrow.r.filled`) é resolvido — é um mecanismo genérico de `Symbol` (árvore de variantes) ou hardcoded por símbolo?
3. Como `repr()` de um `Symbol` com variantes deve formatar a saída (já sinalizado como incompleto por `lente-falta-migrar-2026-07-15.md`, secção 4.1) — corrigir também aqui, é a mesma família de bug.

```bash
cat > /tmp/p765a-symbol-vanilla.typ <<'EOF'
#repr(sym.arrow.r)
#repr(sym.arrow.r.filled)
#repr(symbol(("bold", "α"), ("italic", "α")))
EOF
lab/typst-original/target/release/typst compile /tmp/p765a-symbol-vanilla.typ /tmp/p765a-symbol-vanilla.pdf
mutool draw -F txt /tmp/p765a-symbol-vanilla.pdf
```

Registar a saída exacta de `repr()` para cada caso — é o alvo da correcção.

---

## Implementação — `#title()`

1. Adicionar `Content::Title` (ou equivalente, conforme a sonda confirmar se é elemento próprio ou variante) em `01_core/src/entities/content.rs`.
2. Registar `title` como função nativa de markup em `01_core/src/rules/stdlib/` (mesmo padrão de `heading`, confirmar por leitura directa do stdlib actual).
3. Regra de layout: renderizar o corpo (confirmar com a sonda se tem estilo próprio ou herda de `heading(level: 1)` ou similar).
4. Confirmar que `document(title: ...)` continua a funcionar independentemente (não quebrar o que já está fechado).

## Implementação — `symbol()` e modificadores

1. Construtor nativo `symbol(...)` em `01_core/src/rules/stdlib/foundations.rs` (ou caminho real), aceitando a assinatura confirmada pela sonda.
2. Field access em valores `Symbol`: estender `01_core/src/rules/eval/` (ponto exacto de field access a confirmar) para reconhecer modificadores sobre `Value::Symbol`, devolvendo um novo `Symbol` com a variante seleccionada — não hardcoded por nome, replicando o mecanismo genérico do vanilla.
3. `repr()` de `Symbol` com variantes: corrigir `01_core/src/rules/eval/repr.rs:95` para formatar `symbol("α")` / `symbol(("bold","α"),...)`, conforme a saída exacta registada na sonda.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .

# Reproduzir os três documentos de sonda no cristalino e comparar saída
./target/release/typst compile /tmp/p765a-title-vanilla.typ /tmp/p765a-title-cristalino.pdf
./target/release/typst compile /tmp/p765a-symbol-vanilla.typ /tmp/p765a-symbol-cristalino.pdf
```

Comparar `repr()` e renderização byte a byte (ou `compare -metric AE`, conforme metodologia já usada em P745-762) contra os PDFs vanilla gerados na sonda.

---

## Critério de fecho do passo

- [ ] Sonda de `#title()` e `symbol()` contra o código-fonte do vanilla, não só contra o binário.
- [ ] `#title()` implementado; `document(title:...)` confirmado sem regressão.
- [ ] `symbol(...)` construtor implementado com a assinatura real do vanilla.
- [ ] Field access de modificadores (`sym.arrow.r.filled`) implementado de forma genérica, não hardcoded.
- [ ] `repr()` de `Symbol` corrigido para variantes/modificadores.
- [ ] Saída comparada directamente com o vanilla nos três casos de sonda (AE=0 ou divergência justificada).
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p765a.md`.

---

## Próximo passo

P765b (lote 1): próximo grupo de itens do resíduo `divida-confirmada`/`lacuna-inventario` da lente de 2026-07-15 — dado que os 4 itens `divida-confirmada` originais já ficam todos cobertos por este lote (2 reais aqui + 2 falsos-positivos já explicados em `lente-falta-migrar-2026-07-15.md`, secção 4.1), o lote 1 deve amostrar o próximo módulo por tamanho dentro de `lacuna-inventario` (400 itens), com a mesma disciplina de leitura de código antes de classificar.
