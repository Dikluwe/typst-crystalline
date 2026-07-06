---
# P576 — Alinhamento de parágrafo com `#set text(dir: rtl)`

> **Passo:** 576
> **Data:** 2026-07-05
> **Foco:** Última peça da sequência RTL, marcada como "scope-out, ainda sem passo" desde P565. `#set text(dir: rtl)` continua a emitir aviso de propriedade não suportada. Sem isto, um parágrafo árabe começa sempre à esquerda da página, mesmo com as palavras já na ordem certa dentro da linha (P562/P564/P567) e sem se colarem (P569). Este passo liga a propriedade `dir:` ao alinhamento do parágrafo.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — sonda antes de código, dado que esta área já teve mais do que uma hipótese errada (P566).
> **Dependências:** P484 (bidi_runs), P562/P564/P567 (ordem e posição dentro da linha), P569 (espaço entre palavras), P574/P575 (confirmação recente de que a sequência RTL está estável).

---

## Contexto

Toda a sequência RTL construída até agora trata da ordem das palavras dentro de uma linha, e do espaço entre elas. Nenhuma parte trata de onde a linha começa na página. Um parágrafo árabe, mesmo com tudo o resto correcto, hoje começa na margem esquerda, como um parágrafo latino. O vanilla, com `dir: rtl`, começa na margem direita e o texto flui para a esquerda.

---

## Sonda

### Confirmar onde o aviso é emitido

```bash
grep -n "\"dir\"\|dir.*rtl\|text.*dir" 01_core/src/rules/eval/rules.rs 01_core/src/rules/stdlib/text.rs
```

Perguntas, com `file:line`:

1. `dir:` chega a ser reconhecido como nome de propriedade válido de `#set text(...)`, ou é rejeitado ainda no parser?
2. Existe algum campo em `TextStyle` ou no `StyleChain` que já guarde a direcção, mesmo que nada o use ainda?
3. O alinhamento de parágrafo (margem esquerda vs direita) é decidido em que ponto do Layouter — o mesmo sítio que decide a margem para `#set align(...)`, ou um mecanismo diferente?

### Confirmar que a detecção automática de direcção (já usada por `bidi_runs`) não é suficiente por si só

O texto árabe já é detectado como RTL automaticamente, sem precisar de `dir: rtl`, para efeitos de ordenação de palavras (P484, `unicode_bidi::BidiInfo::new(text, None)` detecta a direcção base sem ajuda). A pergunta aqui é diferente: se essa detecção automática também já poderia decidir o alinhamento da margem sem precisar de `dir:` explícito, ou se o alinhamento tem de vir de outro lado.

```bash
cat > /tmp/p576-auto.typ <<'EOF'
مرحبا بالعالم
EOF
./target/release/typst /tmp/p576-auto.typ /tmp/p576-auto.pdf
mutool draw -o /tmp/p576-auto.png -r 150 /tmp/p576-auto.pdf
```

Ver se, mesmo sem `dir: rtl`, o texto já começa à direita (a detecção automática já chega até ao alinhamento), ou fica à esquerda (falta mesmo ligar isto).

### Critério de fecho da sonda

- [ ] Confirmado se `dir:` é reconhecido ou rejeitado no parser.
- [ ] Confirmado se a detecção automática de direcção já alcança o alinhamento de margem, ou só a ordem das palavras.
- [ ] Localizado o ponto exacto onde o alinhamento de parágrafo é decidido no Layouter.

---

## Implementação

Depende da sonda. Dois casos prováveis:

**Se a detecção automática já chegar ao alinhamento** (isto é, o teste do parágrafo árabe sem `dir:` já começar à direita): o trabalho é só reconhecer a sintaxe `dir: rtl` sem emitir aviso, e confirmar que produz o mesmo resultado que já acontece automaticamente — não muda comportamento, só remove o aviso.

**Se não chegar**: é preciso ligar a informação de direcção (já calculada por `bidi_runs` ou equivalente) ao ponto do Layouter que decide a margem inicial de cada linha, de forma parecida ao que já foi feito para a ordem das palavras (P562/P564), mas afectando a posição inicial da linha inteira, não a ordem interna.

### Critério de fecho da implementação

- [ ] `#set text(dir: rtl)` deixa de emitir aviso de propriedade não suportada.
- [ ] Um parágrafo árabe, com ou sem `dir: rtl` explícito, começa na margem direita.
- [ ] Testado com um documento que mistura um parágrafo árabe e um parágrafo latino, confirmando que cada um mantém o seu próprio alinhamento.
- [ ] `dir: ltr` explícito, se reconhecido pelo vanilla, testado também, para confirmar que força a margem esquerda mesmo em texto que seria detectado como RTL.

---

## Validação

```bash
cat > /tmp/p576-completo.typ <<'EOF'
#set text(dir: rtl, lang: "ar", size: 20pt)
الكتاب على الطاولة

#set text(dir: ltr, lang: "en", size: 20pt)
The book is on the table.
EOF
./target/release/typst /tmp/p576-completo.typ /tmp/p576.pdf
mutool draw -o /tmp/p576.png -r 150 /tmp/p576.pdf
lab/typst-original/target/release/typst compile /tmp/p576-completo.typ /tmp/p576-vanilla.pdf
mutool draw -o /tmp/p576-vanilla.png -r 150 /tmp/p576-vanilla.pdf
```

Comparar as duas imagens: o parágrafo árabe deve começar à direita nos dois casos, o parágrafo latino à esquerda nos dois casos.

```bash
cargo test --workspace
crystalline-lint .
```

Repetir os documentos de referência já usados em P567/P569/P574, confirmando que continuam com os mesmos resultados já medidos (registando o hash do commit usado nesta medição, seguindo a regra escrita depois de P575).

---

## Critério de fecho do passo

- [ ] Sonda completa.
- [ ] `dir: rtl` e `dir: ltr` reconhecidos e a produzir o alinhamento certo.
- [ ] Documento misto (árabe + latim) com cada parágrafo no seu alinhamento.
- [ ] Documentos de referência anteriores sem regressão, medidos com o hash do commit registado.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p576.md`, com o hash do commit da medição, seguindo a regra de proveniência.

---

## Estado da sequência de RTL

| Camada | Passo | Estado |
|--------|-------|--------|
| Shaping de glyphs | P484/P521/P543 | Fechado |
| Fonte embutida no PDF | P560 | Fechado |
| Posição das palavras dentro da linha | P562/P564/P567 | Fechado |
| Espaço entre palavras | P569 | Fechado |
| Quebra de linha com direcção RTL (caso de referência) | P565/P567 | Fechado |
| Alinhamento de parágrafo (`dir: rtl`) | Este passo | Em preparação |

Com este passo, se fechar, a sequência de RTL fica completa do início ao fim — desde a forma de cada letra até onde a página inteira começa a mostrar o texto.
