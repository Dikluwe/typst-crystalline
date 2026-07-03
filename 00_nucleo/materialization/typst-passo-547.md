---
# P547 — Formatação CSL da bibliografia

> **Passo:** 547
> **Data:** 2026-07-03
> **Foco:** P539 testou quatro estilos de citação (ieee, apa, chicago-author-date, mla) e encontrou bugs em todos os quatro: ordem dos nomes de autor trocada, acentos corrompidos, hífens duplicados, espaços a mais antes de pontuação, dados em falta, sem ordenação alfabética. Este passo corrige, com sonda antes de código, para cada categoria de problema.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** L — cinco categorias de problema diferentes, cada uma pode ter causa própria.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR.**
> **Dependências:** P533 (correcção anterior de citações, não cobria formatação CSL a fundo), P539 (onde os dados desta investigação foram levantados).

---

## Contexto

P539 já fez o levantamento. Este passo não repete a investigação — usa a tabela já feita e corrige cada categoria.

Exemplo do problema, estilo IEEE:

- Cristalino: `[1] J. Silva Maria and Santos , "A Comprehensive Study..." vol. 12, p. 45––67, 2023.`
- Vanilla: `[1] M. Silva and J. Santos, "A Comprehensive Study..." vol. 12, pp. 45–67, 2023.`

Cinco categorias de problema, confirmadas em pelo menos um dos quatro estilos testados:

1. Ordem/composição dos nomes de autor trocada.
2. Acentos corrompidos (`João` → `JoÃ£o`) — mesma classe de bug já corrigida noutro sítio em P538b (metadados). Verificar se a bibliografia usa o mesmo caminho de codificação ou um diferente.
3. Hífens duplicados (`45––67` em vez de `45–67`).
4. Espaço a mais antes de pontuação.
5. Dados em falta (publisher ausente nalgumas entradas) e sem ordenação alfabética.

---

## Sonda

```bash
grep -rn "fn.*format.*author\|fn.*csl\|Bibliography.*style" 01_core/src/rules/eval/bibliography.rs 03_infra/src/bib/ --include="*.rs" 2>/dev/null
```

Para cada categoria, uma pergunta separada:

### Categoria 2 — acentos corrompidos

```bash
grep -n "escape_pdf_literal\|utf16be_hex_string" 03_infra/src/export/builder.rs | grep -i "bib\|citation"
```

Confirmar se a bibliografia usa a mesma função corrigida em P538b para `/Info`, ou se tem o seu próprio caminho de codificação, ainda por corrigir.

### Categoria 1 — ordem dos nomes

```bash
grep -n "fn.*parse.*author\|split.*author\|author.*split" 01_core/src/rules/eval/bibliography.rs 03_infra/src/bib/*.rs 2>/dev/null
```

Confirmar como o campo `author` do `.bib` (formato `Sobrenome, Nome and Sobrenome, Nome`) é interpretado — se a lógica de separar "and" e trocar nome/sobrenome está correcta para o formato do BibTeX.

### Categoria 3 — hífens duplicados

```bash
grep -n "en.dash\|–\|--" 01_core/src/rules/eval/bibliography.rs 2>/dev/null
```

O padrão `45––67` sugere que um hífen simples do `.bib` (`45-67`) está a ser convertido para travessão duas vezes, nalgum ponto duplicado da cadeia.

### Categoria 4 — espaço antes de pontuação

Provavelmente o mesmo tipo de causa já visto noutros dois casos de P539 Parte 2 (`Antes Forte : meio` em vez de `Antes Forte: meio`) — confirmar se é a mesma causa (relacionada com P544, espaçamento de layout) ou específica da bibliografia.

### Categoria 5 — dados em falta, sem ordenação

```bash
grep -n "publisher\|sort.*bib\|alphabetical" 01_core/src/rules/eval/bibliography.rs 2>/dev/null
```

### Critério de fecho da sonda

- [ ] Cada uma das cinco categorias tem causa localizada com `file:line`.
- [ ] Confirmado se a categoria 2 (acentos) é o mesmo bug de P538b não propagado à bibliografia, ou um bug diferente.
- [ ] Confirmado se a categoria 4 (espaço antes de pontuação) é o mesmo bug de P544, ou específico da bibliografia.

---

## Implementação

Uma sub-tarefa por categoria, cada uma com o seu próprio teste — não misturar as cinco num só fix.

### Critério de fecho da implementação

- [ ] Categoria 1: nomes de autor na ordem certa para os quatro estilos.
- [ ] Categoria 2: acentos correctos.
- [ ] Categoria 3: travessão simples, não duplicado.
- [ ] Categoria 4: sem espaço antes de pontuação.
- [ ] Categoria 5: publisher presente quando existe no `.bib`; entradas ordenadas alfabeticamente quando o estilo o exige.

---

## Validação

Reutilizar o mesmo `.bib` e os quatro estilos testados em P539.

```bash
cat > /tmp/refs-full.bib <<'EOF'
@article{silva2023,
  author = {Silva, Maria and Santos, João},
  title = {A Comprehensive Study of Modern Typography},
  journal = {Journal of Design Research},
  volume = {12},
  pages = {45--67},
  year = {2023}
}
@book{costa2021,
  author = {Costa, Ana},
  title = {The Art of Document Preparation},
  publisher = {Academic Press},
  year = {2021}
}
EOF

for style in ieee apa chicago-author-date mla; do
  cat > /tmp/p547-$style.typ <<EOF
#bibliography("refs-full.bib", style: "$style")
Ver @silva2023 e @costa2021.
EOF
  ./target/release/typst /tmp/p547-$style.typ /tmp/p547-$style.pdf
  echo "=== $style ==="
  pdftotext /tmp/p547-$style.pdf -
  /usr/local/bin/typst compile /tmp/p547-$style.typ /tmp/p547-$style-vanilla.pdf
  pdftotext /tmp/p547-$style-vanilla.pdf -
done
```

Comparar linha a linha, os quatro estilos, cristalino contra vanilla.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda completa para as cinco categorias, com `file:line`.
- [ ] As cinco categorias corrigidas, cada uma com teste próprio.
- [ ] Os quatro estilos (ieee, apa, chicago-author-date, mla) testados contra vanilla, com o mesmo `.bib` de P539.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p547.md`.

---

## Próximo passo

Retomar o inventário para os itens ainda sem passo: exportação de formatos (HTML/SVG/PNG/LSP), fontes de cor, PDF avançado, pacotes.
