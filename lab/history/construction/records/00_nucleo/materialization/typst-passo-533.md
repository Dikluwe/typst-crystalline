---
# P533 — Citações bibliográficas (`@key1` não resolve; formatação CSL com erros)

> **Passo:** 533
> **Data:** 2026-07-02
> **Foco:** P531 Grupo 6 confirmou dois problemas: (A) `@key1` no corpo do texto não resolve — o PDF mostra "See ." em vez de "See [1]."; (B) a bibliografia final tem erros de formatação — aspas tipográficas viram `?`, e "Bibliography" aparece truncado para "Bibliograph". Sondar as duas causas antes de corrigir; são provavelmente independentes uma da outra.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — sonda antes de qualquer edição.
> **Dependências:** P531 (Grupo 6, onde os dois problemas foram confirmados), P532 (nota de fecho — confirmar se `style.font` fica preenchido para texto desenhado directamente pelo Layouter, já que este passo também desenha texto de citação e de bibliografia).

---

## Contexto

Dois problemas distintos, tratados como duas sub-tarefas separadas porque é provável que tenham causas diferentes:

**A — Citação inline não resolve.** `@key1` no corpo do documento devia produzir `[1]` (ou o formato do estilo escolhido). Em vez disso, produz um espaço vazio: "See ." em vez de "See [1]."

**B — Formatação CSL com corrupção de texto.** A bibliografia final (com `style: "ieee"`) tem aspas a aparecer como `?` em vez do carácter tipográfico correcto, e a palavra "Bibliography" aparece cortada para "Bibliograph" — falta a última letra.

O padrão do problema B (um carácter em falta no fim de uma palavra, e um carácter substituído por `?`) é o mesmo tipo de sintoma já visto antes neste projecto num contexto diferente: corte de string a meio de uma fronteira de carácter UTF-8, sem verificação. Vale testar essa hipótese primeiro, porque já haveria precedente de código com esse bug noutro sítio.

---

## Sub-tarefa A — Citação inline não resolve

### Sonda

```bash
grep -rn "@\|CiteRef\|resolve_citation\|BibStore" 01_core/src/engine/eval/ 01_core/src/engine/layout/ --include="*.rs" | grep -i cit
```

Perguntas, com `file:line`:

1. `@key1` é reconhecido pelo parser como referência de citação, ou como outra coisa (label, referência cruzada genérica)?
2. Se reconhecido, chega a fazer lookup na `BibStore`? O lookup encontra a entrada?
3. Se encontra, o resultado (`[1]`) é emitido como `Content`, ou é descartado nalgum sítio da cadeia eval → layout?
4. Testar isoladamente: uma citação sozinha, sem `bibliography()` no mesmo documento, e uma citação com `bibliography()` presente — para ver se o problema depende da bibliografia estar carregada primeiro (ordem de processamento).

```bash
cat > /tmp/test-cite-a.typ <<'EOF'
#bibliography("refs.bib", style: "ieee")
See @key1.
EOF
cat > /tmp/test-cite-b.typ <<'EOF'
See @key1.
#bibliography("refs.bib", style: "ieee")
EOF
```

Comparar os dois. Se um funcionar e o outro não, o problema é de ordem/dependência entre a citação e a leitura do `.bib`, não de resolução em si.

### Critério de fecho da sonda A

- [ ] Localização exacta de onde `@key1` é parseado.
- [ ] Confirmado se chega a fazer lookup, e se o lookup falha ou o resultado é descartado depois.
- [ ] Testes de ordem (citação antes/depois de `bibliography()`) corridos.

### Implementação

Depende da causa. Não escrever antes de confirmar.

### Critério de fecho da implementação A

- [ ] `@key1` produz `[1]` no PDF, no formato do estilo escolhido.
- [ ] Testado com citação antes e depois de `bibliography()` no documento.
- [ ] Testado com mais de uma citação da mesma entrada (`@key1` duas vezes) e com citações de entradas diferentes (`@key1`, `@key2`).

---

## Sub-tarefa B — Formatação CSL corrompida

### Sonda

```bash
grep -rn "fn.*format.*csl\|fn.*ieee\|Bibliography\|smart.*quote\|curly.*quote" 01_core/src/engine/stdlib/ 01_core/src/entities/ --include="*.rs" | grep -i "csl\|bibliograph"
```

Hipótese a testar primeiro: corte de string sem respeitar fronteira de carácter UTF-8.

```bash
grep -n "Bibliography\|\"Bibliograph\"" 01_core/src/engine/stdlib/*.rs 01_core/src/entities/*.rs
```

Se a palavra "Bibliography" está escrita como constante fixa no código (não construída dinamicamente), a truncagem não pode ser um corte de string por byte — seria antes um erro de índice fixo (ex.: `&s[..12]` num texto com mais de 12 caracteres, ou um limite de buffer). Confirmar qual dos dois é o caso antes de assumir.

Para o problema das aspas: procurar onde a citação/entrada bibliográfica é formatada com aspas tipográficas (`"`, `"`) e confirmar se o texto de entrada (título do artigo, vindo do `.bib`) está a ser processado byte a byte em vez de carácter a carácter nalgum ponto — aspas tipográficas `"`/`"` ocupam 3 bytes em UTF-8; um corte a meio de um desses caracteres produzia exactamente o sintoma de um `?` (carácter de substituição usado quando bytes UTF-8 inválidos são encontrados).

```bash
cat /tmp/refs.bib 2>/dev/null || cat > /tmp/refs.bib <<'EOF'
@article{key1,
  title = {A Sample Paper},
  author = {Author, Test},
  year = {2024},
}
EOF
./target/release/typst /tmp/test-cite-a.typ /tmp/bib-debug.pdf
pdftotext /tmp/bib-debug.pdf - | grep -i "bibliograph\|sample paper"
```

### Critério de fecho da sonda B

- [ ] Causa da truncagem de "Bibliography" identificada (constante fixa com erro, ou corte dinâmico).
- [ ] Causa dos `?` no lugar de aspas identificada — confirmar ou refutar a hipótese de corte a meio de carácter UTF-8.

### Implementação

Depende da causa. Se for corte a meio de carácter, o fix segue o mesmo princípio já usado antes neste projecto noutro contexto: nunca cortar uma string por índice de byte sem confirmar que esse índice cai numa fronteira válida de carácter.

### Critério de fecho da implementação B

- [ ] "Bibliography" aparece completo no PDF.
- [ ] Aspas tipográficas aparecem correctas, não `?`.
- [ ] Testado com um título de artigo que tenha acentos ou outros caracteres multi-byte, não só ASCII — para confirmar que o fix cobre o caso geral, não só o exemplo específico.

---

## Validação conjunta

```bash
./target/release/typst /tmp/test-cite-a.typ /tmp/final.pdf
pdftotext /tmp/final.pdf -
```

Esperado: "See [1]." no corpo, e a entrada bibliográfica completa e correctamente formatada no fim, incluindo o cabeçalho "Bibliography" por inteiro.

Comparar com vanilla 0.15.0 para o mesmo documento e o mesmo `.bib`.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sub-tarefa A: citação inline resolve correctamente.
- [ ] Sub-tarefa B: formatação sem corrupção de texto.
- [ ] Testado com caracteres multi-byte no `.bib`, não só ASCII.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p533.md`.

---

## Próximo passo

P534 — Fallback de fonte por carácter (multi-script/emoji). É o maior dos seis itens da sequência; precisa de sonda própria, incluindo a correcção da linha do handoff que descreve P515 como tendo fechado isto quando P531 mostrou que não está completo.
