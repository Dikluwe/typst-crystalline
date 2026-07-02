---
# P537 — Notas de rodapé em layout de duas colunas

> **Passo:** 537
> **Data:** 2026-07-02
> **Foco:** P531 Grupo 8.1 confirmou que, com `#set page(columns: 2)`, a nota de rodapé aparece no fundo da página inteira, depois das duas colunas, em vez de no fundo da coluna onde foi referenciada, como o vanilla faz.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — sonda antes de qualquer edição.
> **Dependências:** Mecanismo de notas de rodapé (fechado para layout de uma coluna), mecanismo de colunas (fechado). P531 (Grupo 8.1, onde o problema foi confirmado). Este é o sexto e último dos itens de alto impacto da sequência — depois dele, P538 confirma o fecho de todos.

---

## Contexto

O mecanismo de notas de rodapé já existe e funciona para layout de uma coluna: as notas acumulam-se num buffer e são desenhadas no fundo da página quando a página fecha. O mecanismo de colunas também já existe, de forma independente.

O problema aparece quando os dois se combinam: o buffer de notas pendentes é pensado ao nível da página inteira, não ao nível de cada coluna. Quando a página tem duas colunas, o cálculo de "onde é o fundo" usa a altura da página completa, sem considerar que cada coluna tem a sua própria área e as suas próprias notas.

---

## Sonda

```bash
grep -rn "pending_footnote\|flush_pending_footnote\|footnote.*column\|column.*footnote" 01_core/src/rules/layout/ --include="*.rs"
```

Perguntas, com `file:line`:

1. O buffer de notas pendentes (`pending_footnote_bodies` ou equivalente) é um único buffer para a página inteira, ou já existe separação por coluna?
2. Quando uma coluna fecha (não a página inteira, só a coluna), existe algum evento equivalente a "fim de coluna" que possa disparar o desenho das notas dessa coluna, semelhante ao que já acontece no fim de página?
3. O cálculo da área disponível para as notas (`page_bottom - cursor_y` ou equivalente) usa a largura/altura da página inteira, ou já tem noção da coluna actual?

### Teste directo para confirmar o comportamento actual

```bash
cat > /tmp/test-footnote-cols.typ <<'EOF'
#set page(columns: 2)
#lorem(80)#footnote[Nota da primeira coluna.]
#colbreak()
#lorem(80)#footnote[Nota da segunda coluna.]
EOF
./target/release/typst /tmp/test-footnote-cols.typ /tmp/footnote-cols.pdf
```

Inspeccionar visualmente ou por posição no PDF: as duas notas aparecem juntas no fundo da página, ou cada uma no fundo da sua coluna?

### Critério de fecho da sonda

- [ ] Confirmado se o buffer de notas é por página ou por coluna.
- [ ] Confirmado se existe evento de "fim de coluna" disponível para reutilizar.
- [ ] Teste directo corrido, resultado registado.

---

## Implementação

Depende do resultado da sonda. Duas linhas gerais possíveis, a decidir depois da sonda:

**Se já existe evento de fim de coluna:** ligar o mesmo mecanismo que hoje desenha notas no fim de página a esse evento, mas com o buffer de notas separado por coluna, não partilhado.

**Se não existe evento de fim de coluna:** este passo cresce — é preciso criar essa noção antes de resolver o problema das notas. Se for esse o caso, medir a dimensão real antes de continuar a implementar às cegas; pode justificar dividir este passo em dois.

### Critério de fecho da implementação

- [ ] Cada nota de rodapé aparece no fundo da coluna onde foi referenciada, não no fundo da página inteira.
- [ ] Testado com notas em ambas as colunas do mesmo layout.
- [ ] Testado com uma nota grande que não caiba no espaço restante da coluna (comportamento de overflow, já resolvido para o caso de uma coluna — confirmar que continua a funcionar aqui).

---

## Validação

```bash
./target/release/typst /tmp/test-footnote-cols.typ /tmp/footnote-cols-pos-fix.pdf
```

Esperado: nota da primeira coluna no fundo da primeira coluna; nota da segunda coluna no fundo da segunda coluna. Comparar com vanilla 0.15.0 para o mesmo documento.

```bash
cargo test --workspace
crystalline-lint .
```

Confirmar também que o caso de uma coluna (já fechado antes) não regride — correr os testes existentes de notas de rodapé de página única.

---

## Critério de fecho do passo

- [ ] Sonda completa antes de código.
- [ ] Notas de rodapé posicionadas na coluna correcta.
- [ ] Caso de uma coluna sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p537.md`.

---

## Próximo passo

P538 — Confirmação: repetir a sondagem de P531 só para os seis itens (P532–P537), confirmar que ficaram fechados, incluindo os dois pontos deixados em aberto ao longo da sequência (verificação de `style.font` no texto de numeração de página, de P532; verificação de largura de quebra de linha em texto com fallback de fonte, de P534; verificação de codificação não-ASCII em título/autor, de P536). Só depois desta confirmação a reorganização (P539+) começa.
