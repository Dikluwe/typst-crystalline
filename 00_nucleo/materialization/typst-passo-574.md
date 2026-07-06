---
# P574 — A correcção de codepoints de P568 interage com a sequência RTL?

> **Passo:** 574
> **Data:** 2026-07-05
> **Foco:** P573 manteve e comitou a correcção de P568 (`try_shape` preserva texto só de espaço; `collect_text_codepoints` inclui esses espaços no subset). Este trabalho toca no mesmo assunto — espaços a sobreviver do texto até ao PDF final — que a sequência de P566 a P572 tratou para RTL. A validação de P573 não repetiu a medição de posições do documento árabe de referência. Este passo confirma se há interacção.
> **Tipo:** Verificação directa.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P569 (correcção de espaço em RTL, em `layout_bidi.rs`), P573 (correcção de codepoints, em `shaper.rs`/`builder.rs`/`fonts.rs`).

---

## Verificação

### Repetir o documento de referência da sequência RTL

```bash
cat > /tmp/p574-referencia.typ <<'EOF'
#set text(lang: "ar", size: 40pt)
الكتاب 42 على الطاولة
EOF
./target/release/typst /tmp/p574-referencia.typ /tmp/p574.pdf
pdftotext -tsv /tmp/p574.pdf /tmp/p574.tsv
```

Comparar com a tabela de posições já obtida em P567/P569 para o mesmo documento. Confirmar se as posições continuam iguais, ou se mudaram depois de P573.

### Repetir o documento longo

```bash
cat > /tmp/p574-longo.typ <<'EOF'
#set text(lang: "ar", size: 14pt)
هذا نص طويل باللغة العربية. يحتوي على معلومات قيمة. نأمل أن يعمل بشكل صحيح.
EOF
./target/release/typst /tmp/p574-longo.typ /tmp/p574-longo.pdf
pdftotext /tmp/p574-longo.pdf -
```

Comparar com o resultado já obtido em P569 para o mesmo documento (palavras separadas, sem colagem).

### Critério de fecho

- [ ] Documento de referência: posições comparadas com a medição já feita em P567/P569.
- [ ] Documento longo: confirmado que as palavras continuam separadas.
- [ ] Se algo mudou: decidir se é melhoria, regressão, ou coincidência sem relação com P573.
- [ ] Se nada mudou: confirmado por medição, não por suposição, que os dois blocos de trabalho (P568/P573 e P569) são independentes.

---

## Relatório de execução

`00_nucleo/diagnosticos/paridade-producao-p574.md` — as duas medições, comparadas directamente com os números já existentes dos passos anteriores, não descritas de novo sem esse ponto de comparação.
