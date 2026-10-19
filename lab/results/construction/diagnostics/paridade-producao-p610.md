# Paridade de Produção — P610

**Data do relatório:** 2026-07-08
**Passo:** 610
**Foco:** Testar `.otc` (OpenType Collection) directamente e varrer o corpus por outros documentos com sintoma de fonte não subsetada.

---

## Resumo executivo

P609 corrigiu fontes `.ttc` embutidas inteiras extraindo a face individual da coleção em `FontSlot::get()`. P610 confirma que a mesma correcção funciona para `.otc`, cujo cabeçalho de coleção (`ttcf`) é idêntico. Também varreu o corpus `lab/parity/corpus/` à procura de outros documentos que produzissem PDFs anómalamente grandes (>1 MB); nenhum foi encontrado.

**Conclusão:** `.otc` está coberto pela correcção de P609. Não há novos casos de fontes não subsetadas no corpus testado.

---

## Proveniência

- **Hash base:** `bd0077e6e9b48b4e9d97b882949512cacada85f9`
- **Data/hora:** 2026-07-08T01:30-03:00 (referência de sessão)
- **Binários usados:**
  - Cristalino: `./target/release/typst` (P609)
  - fontTools 4.63.0 (lab venv) para criar a fonte `.otc` de teste
- **Ferramentas auxiliares:** `mutool extract`, `fontTools`

---

## Parte 1 — Teste directo de `.otc`

### Fonte `.otc` de teste

O sistema não tinha fontes `.otc` instaladas. Criou-se uma coleção de teste a partir de duas fontes CFF existentes:

- `/usr/share/fonts/opentype/urw-base35/C059-Roman.otf`
- `/usr/share/fonts/opentype/urw-base35/NimbusRoman-Regular.otf`

Ficheiro criado: `/tmp/p610-test.otc` (189.880 bytes, 2 faces CFF).

### Teste com a primeira face (`C059`, índice 0)

Documento `/tmp/p610-otc.typ`:

```typst
#set text(font: "C059")
Hello world
```

Resultado:

- PDF: 24.037 bytes
- Fonte extraída: `font-0007.cid`, 21.985 bytes

### Teste com a segunda face (`Nimbus Roman`, índice 1)

Documento `/tmp/p610-otc2.typ`:

```typst
#set text(font: "Nimbus Roman")
Hello world
```

Resultado:

- PDF: 21.628 bytes
- Fonte extraída: `font-0007.cid`, 19.577 bytes

### Interpretação

A fonte `.otc` completa tem ~190 KB. A fonte embutida no PDF tem ~20-22 KB. Isto confirma que o subsetting funcionou: a face correcta foi extraída da coleção e depois reduzida aos glifos usados.

A correcção de P609 (`extract_collection_face` a partir do cabeçalho `ttcf`) é agnóstica ao conteúdo interno da face (TrueType `glyf` ou CFF), pelo que cobre `.otc` sem alterações adicionais.

---

## Parte 2 — Varrimento do corpus

### Método

Para cada ficheiro `.typ` em `lab/parity/corpus/`, compilou-se o PDF e registou-se o tamanho. Documentos com mais de 1 MB foram sinalizados para investigação.

```bash
for f in $(find lab/parity/corpus -name "*.typ"); do
  ./target/release/typst "$f" /tmp/p610-out.pdf 2>/dev/null
  if [ -f /tmp/p610-out.pdf ]; then
    size=$(stat -c%s /tmp/p610-out.pdf)
    if [ "$size" -gt 1000000 ]; then
      echo "GRANDE ($size bytes): $f"
    fi
    rm /tmp/p610-out.pdf
  fi
done
```

### Resultado

- **Total de documentos:** 90
- **PDFs > 1 MB:** 0
- **Maior PDF encontrado:** nenhum atingiu o limite de 1 MB

Não há indícios de outros documentos no corpus com fontes de fallback embutidas inteiras.

---

## Decisão

Não são necessárias alterações de código. A correcção de P609 cobre `.otc` pelo mesmo mecanismo. O corpus não revelou novos casos.

Se no futuro uma fonte `.otc` real falhar, o diagnóstico é o mesmo: verificar se `extract_collection_face` devolve a fatia correcta e se `oxifont_subset` consegue processar a face CFF resultante.

---

## Critérios de fecho do passo

- [x] `.otc` testado directamente com fonte real criada para o efeito.
- [x] Subsetting confirmado para ambas as faces da coleção `.otc`.
- [x] Corpus `lab/parity/corpus/` varrido (90 documentos).
- [x] Nenhum PDF anómalo (>1 MB) encontrado.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório escrito com proveniência.

---

## Ligações

- `00_nucleo/materialization/typst-passo-610.md` — passo que originou a verificação.
- `00_nucleo/diagnosticos/paridade-producao-p609.md` — passo cuja correcção foi validada para `.otc`.
- `03_infra/src/fonts.rs:67` — `extract_collection_face`.
