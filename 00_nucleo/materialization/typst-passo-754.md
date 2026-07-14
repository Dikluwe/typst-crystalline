---
# P754 — Confirmar que a nova fonte por defeito não quebra cobertura de glifos (árabe, devanágari, e outros)

> **Passo:** 754
> **Data:** 2026-07-14
> **Foco:** P753 trocou a fonte por defeito de `Liberation Serif` para `Libertinus Serif`, mas nunca confirmou visualmente uma amostra de documentos já testados nesta conversa — só `cargo test --workspace`, que verifica texto extraído, não cobertura de glifos nem a fonte de facto usada. `Libertinus Serif` é uma fonte latina/grega/cirílica; documentos com árabe/devanágari (P590-627) dependem da cadeia de fallback para chegar a uma fonte com cobertura adequada, e essa cadeia pode ter mudado de comportamento com a nova fonte por defeito.
> **Tipo:** Verificação directa. Correcção se confirmada uma regressão.
> **Tamanho:** S-M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P753 (onde a mudança de fonte por defeito foi feita sem esta verificação).

---

## Verificação

### Confirmar a fonte de facto usada para árabe/devanágari, antes e depois de P753

```bash
cat > /tmp/p754-arabe.typ <<'EOF'
#set text(dir: rtl, lang: "ar")
الكتاب على الطاولة
EOF
./target/release/typst /tmp/p754-arabe.typ /tmp/p754-arabe.pdf
mutool show /tmp/p754-arabe.pdf | grep -i "BaseFont"

cat > /tmp/p754-devanagari.typ <<'EOF'
नमस्ते संसार
EOF
./target/release/typst /tmp/p754-devanagari.typ /tmp/p754-devanagari.pdf
mutool show /tmp/p754-devanagari.pdf | grep -i "BaseFont"
```

Confirmar qual fonte é de facto usada para estes scripts hoje, e se é diferente da usada antes de P753 (fazer checkout do commit anterior a P753 para comparar, se necessário).

### Confirmar visualmente que os glifos continuam correctos

```bash
mutool draw -o /tmp/p754-arabe.png -r 150 /tmp/p754-arabe.pdf
mutool draw -o /tmp/p754-devanagari.png -r 150 /tmp/p754-devanagari.pdf
```

Inspeccionar as imagens — confirmar que o texto árabe/devanágari renderiza com glifos reconhecíveis, não caixas vazias (`.notdef`) nem substituição incorrecta.

### Repetir uma amostra de documentos de teste já usados nas sequências P590-627 (RTL/árabe/devanágari) e outras secções significativas desta conversa

Reconstruir 2-3 dos documentos de teste mais representativos dessas sequências (a partir da descrição nos relatórios, já que os ficheiros `.typ` originais não persistem entre sessões) e confirmar visualmente que continuam correctos.

### Confirmar também scripts CJK, se algum documento os tiver testado

```bash
cat > /tmp/p754-cjk.typ <<'EOF'
你好世界
EOF
./target/release/typst /tmp/p754-cjk.typ /tmp/p754-cjk.pdf
mutool show /tmp/p754-cjk.pdf | grep -i "BaseFont"
mutool draw -o /tmp/p754-cjk.png -r 150 /tmp/p754-cjk.pdf
```

### Critério de fecho da verificação

- [ ] Fonte de facto usada para árabe/devanágari confirmada, antes e depois de P753.
- [ ] Glifos confirmados visualmente correctos para árabe, devanágari, e CJK (se aplicável).
- [ ] Amostra de documentos representativos das sequências P590-627 re-verificada.

---

## Decisão

Se nenhuma regressão for encontrada: confirmar e documentar que a mudança de fonte por defeito não afectou a cadeia de fallback para outros scripts — fechar com confiança, não só assumir.

Se houver regressão (fonte errada, glifos incorrectos): corrigir a ordem/composição da cadeia de fallback, garantindo que `Libertinus Serif` como primeira escolha não interfere com a resolução correcta de fallback para scripts que não cobre.

---

## Critério de fecho do passo

- [ ] Verificação completa, com confirmação visual, não só testes automatizados de texto.
- [ ] Se confirmada regressão: corrigida e testada.
- [ ] Se sem regressão: confirmação registada com evidência (imagens, nomes de fonte).
- [ ] `cargo test --workspace` sem regressão, se houver mudança de código.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p754.md`.
