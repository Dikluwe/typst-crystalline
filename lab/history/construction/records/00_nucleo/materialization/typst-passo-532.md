---
# P532 — Numeração de página customizada (`#set page(numbering: ...)`)

> **Passo:** 532
> **Data:** 2026-07-02
> **Foco:** `#set page(numbering: "i")` (e outros padrões) compila sem erro mas os números não aparecem no PDF final, confirmado em P531 Grupo 8.2. Sondar a causa exacta antes de corrigir.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — sonda antes de qualquer edição.
> **Dependências:** P531 (Grupo 8.2, onde o problema foi confirmado).

---

## Contexto

P531 confirmou: a propriedade `numbering:` de `#set page()` é reconhecida pelo parser (não dá erro de sintaxe), mas o número de página não aparece no PDF gerado. O vanilla mostra `i`, `ii`, `iii`. O cristalino não mostra nada.

Isto é diferente de "funcionalidade ausente" — a propriedade é aceite, o que sugere que existe algum código a tratar dela, mas algo entre aceitar a propriedade e desenhar o número no PDF está a falhar ou está incompleto.

---

## Sonda

```bash
grep -rn "numbering.*page\|page.*numbering" 01_core/src/engine/stdlib/ --include="*.rs"
grep -rn "PageNumbering\|page_numbering" 01_core/src/entities/ 01_core/src/engine/layout/ --include="*.rs"
```

Perguntas a responder, com `file:line`:

1. A propriedade `numbering:` fica guardada nalgum sítio (`TextStyle`, `PageStyle`, ou equivalente) depois de `#set page(numbering: "i")`?
2. Existe código no Layouter que lê essa propriedade e desenha um número no PDF?
3. Se existe, porque não aparece? Candidatos: o número é calculado mas nunca emitido como `Content::Text`; o número é emitido mas posicionado fora da página; a função de formatação do padrão (`"i"` → `"i", "ii", "iii"`) não está implementada e falha em silêncio.
4. `counter(page).display()` (usado em `header`/`footer` customizado) funciona? Se sim, o problema pode ser só na numeração automática por defeito, não no mecanismo de contagem em si.

### Teste directo para isolar

```bash
cat > /tmp/test-page-num-a.typ <<'EOF'
#set page(numbering: "i")
Página um.
#pagebreak()
Página dois.
EOF

cat > /tmp/test-page-num-b.typ <<'EOF'
#set page(footer: context [#counter(page).display("i")])
Página um.
#pagebreak()
Página dois.
EOF

./target/release/typst /tmp/test-page-num-a.typ /tmp/a.pdf
./target/release/typst /tmp/test-page-num-b.typ /tmp/b.pdf
pdftotext /tmp/a.pdf -
pdftotext /tmp/b.pdf -
```

Se o teste B funcionar e o A não, o problema está isolado à numeração automática por defeito de `#set page(numbering:)`, não ao mecanismo de contagem/formatação. Isso reduz bastante o que precisa de correcção.

### Critério de fecho da sonda

- [ ] Localização exacta de onde `numbering:` é guardada.
- [ ] Confirmado se o Layouter lê essa propriedade.
- [ ] Teste A vs teste B corrido, resultado registado.
- [ ] Causa exacta identificada antes de qualquer edição.

---

## Implementação

Depende do resultado da sonda. Não escrever código antes disso. Uma vez confirmada a causa, o fix deve ser cirúrgico — não reescrever o mecanismo de footer/header, só ligar a numeração automática por defeito ao mesmo mecanismo que já funciona via `counter(page).display()` (se for esse o caso).

### Critério de fecho da implementação

- [ ] `#set page(numbering: "i")` produz `i`, `ii`, `iii` no PDF.
- [ ] Testar também `numbering: "1"` (padrão default, arábico) e `numbering: "a"` (alfabético), para confirmar que o fix cobre os três casos, não só o romano.
- [ ] `cargo test --workspace` sem regressão.
- [ ] `crystalline-lint .` limpo.

---

## Validação

```bash
./target/release/typst /tmp/test-page-num-a.typ /tmp/a-pos-fix.pdf
pdftotext /tmp/a-pos-fix.pdf -
```

Esperado: `i` na primeira página, `ii` na segunda. Comparar com vanilla 0.15.0 para o mesmo documento.

---

## Critério de fecho do passo

- [ ] Sonda completa, causa identificada.
- [ ] Fix aplicado, três padrões de numeração testados.
- [ ] Sem regressão.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p532.md`.

---

## Próximo passo

P533 — Citações bibliográficas (`@key1` não resolve; formatação CSL com erros de pontuação).
