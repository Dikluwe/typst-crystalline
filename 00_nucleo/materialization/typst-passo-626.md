---
# P626 — Direcção de preenchimento de colunas em RTL

> **Passo:** 626
> **Data:** 2026-07-05
> **Foco:** P625 encontrou e disse com clareza, sem esconder, que `#set page(columns: 2)` com texto árabe enche a coluna esquerda primeiro no cristalino, enquanto o vanilla enche a coluna direita primeiro. Isto não é um detalhe visual — é a ordem de leitura do documento inteiro. Um leitor de árabe, a seguir o fluxo natural do texto, começaria pela direita; no cristalino, o conteúdo que devia estar lá está à esquerda. Este passo corrige a direcção de preenchimento.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P625 (onde o problema foi encontrado e disclosed), P552/P553 (mecanismo de colunas já corrigido antes, por outras razões), P562-P592 (sequência RTL completa, mecanismo de direcção já disponível).

---

## Contexto

O mecanismo de colunas (`columns.rs`) decide em que coluna cada bloco de conteúdo entra, tipicamente enchendo a primeira coluna (a mais à esquerda) até esgotar o espaço, depois passando à segunda. Para um documento RTL, isto está invertido: a "primeira" coluna, na ordem de leitura, é a mais à direita.

---

## Sonda

### Confirmar o comportamento exacto do vanilla com mais de duas colunas

```bash
cat > /tmp/p626-tres-colunas.typ <<'EOF'
#set page(columns: 3)
#set text(lang: "ar", dir: rtl, size: 20pt)
#lorem(200)
EOF
lab/typst-original/target/release/typst compile /tmp/p626-tres-colunas.typ /tmp/p626-vanilla.pdf
mutool draw -o /tmp/p626-vanilla.png -r 100 /tmp/p626-vanilla.pdf
```

Confirmar: com três colunas, o vanilla enche da direita para a esquerda (terceira, segunda, primeira posição física), ou só inverte a primeira e a última?

### Localizar onde a ordem de preenchimento é decidida

```bash
grep -n "column_index\|fill.*column\|next_column\|start_column" 01_core/src/rules/layout/columns.rs 01_core/src/rules/layout/cursor.rs
```

Confirmar se a lógica de "próxima coluna" é um incremento simples (esquerda para direita sempre), ou já tem alguma noção de direcção que pode ser invertida.

### Critério de fecho da sonda

- [ ] Comportamento do vanilla confirmado com três colunas, não só duas.
- [ ] Ponto exacto de decisão de ordem de coluna localizado, com `file:line`.

---

## Implementação

Quando a direcção do documento (ou do texto predominante na página) for RTL, inverter a ordem de preenchimento de colunas: a primeira coluna a encher é a mais à direita, avançando para a esquerda.

### Critério de fecho da implementação

- [ ] Duas colunas: primeira posição preenchida é a da direita.
- [ ] Três colunas: ordem completa confirmada contra o vanilla (não assumir que é só inverter a primeira/última).
- [ ] Documentos LTR sem regressão — continuam a encher da esquerda para a direita.
- [ ] Documento misto (algumas secções LTR, outras RTL, cada `#set page(columns:)` a aplicar-se à sua secção) testado, para confirmar que a direcção de preenchimento muda correctamente consoante o contexto.

---

## Validação

```bash
cat > /tmp/p626-duas-colunas.typ <<'EOF'
#set page(columns: 2)
#set text(lang: "ar", dir: rtl, size: 20pt)
#lorem(200)
EOF
./target/release/typst /tmp/p626-duas-colunas.typ /tmp/p626-depois.pdf
mutool draw -o /tmp/p626-depois.png -r 100 /tmp/p626-depois.pdf
```

Comparar directamente com a imagem do vanilla já obtida na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

Repetir os testes de colunas já existentes (P552, P553, P595) para confirmar que a estrutura interna de cada coluna, já corrigida, não regride com esta mudança de ordem.

---

## Critério de fecho do passo

- [ ] Sonda completa, comportamento do vanilla confirmado com duas e três colunas.
- [ ] Ordem de preenchimento corrigida para RTL.
- [ ] LTR sem regressão.
- [ ] Documento misto testado.
- [ ] Sem regressão nos testes de colunas já existentes.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p626.md`, com hash do commit.
