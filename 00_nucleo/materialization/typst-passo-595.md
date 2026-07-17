---
# P595 — Nota de rodapé descartada silenciosamente em colunas

> **Passo:** 595
> **Data:** 2026-07-05
> **Foco:** Registado em P537 como "scope-out" sem razão escrita: uma nota de rodapé maior do que o espaço restante numa coluna é descartada, sem aviso nenhum ao utilizador. Isto é perda de conteúdo do documento, não uma diferença de aparência. Este passo confirma o comportamento actual e corrige — a nota tem de aparecer nalgum sítio (mesmo que não seja no lugar perfeito), ou o utilizador tem de ser avisado, nunca as duas coisas em silêncio ao mesmo tempo.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** Perda de conteúdo sem aviso não é uma decisão aceitável, mesmo que documentada como scope-out.
> **Dependências:** P537 (onde o comportamento foi observado e aceite sem correcção).

---

## Sonda

### Confirmar o comportamento actual

```bash
cat > /tmp/p595-nota-grande.typ <<'EOF'
#set page(columns: 2, height: 200pt)
#lorem(30)#footnote[
  Esta é uma nota de rodapé muito longa, com texto suficiente para não caber no espaço restante de uma coluna pequena, testando o que acontece quando isto excede o espaço disponível na página.
]
#lorem(30)
EOF
./target/release/typst /tmp/p595-nota-grande.typ /tmp/p595.pdf
pdftotext /tmp/p595.pdf -
```

Confirmar: o texto da nota aparece nalgum sítio do PDF (mesmo que cortado, ou empurrado para outra página), ou desaparece por completo?

```bash
grep -n "footnote\|pending_footnote" 01_core/src/engine/layout/cursor.rs | head -20
```

Localizar, com `file:line`, o ponto exacto onde a nota é descartada quando não cabe.

### Confirmar o que o vanilla faz no mesmo caso

```bash
lab/typst-original/target/release/typst compile /tmp/p595-nota-grande.typ /tmp/p595-vanilla.pdf
pdftotext /tmp/p595-vanilla.pdf -
```

### Critério de fecho da sonda

- [ ] Confirmado, com teste directo, se a nota desaparece por completo ou fica visível de alguma forma.
- [ ] Localizado o ponto exacto onde isso acontece.
- [ ] Confirmado o que o vanilla faz no mesmo documento.

---

## Implementação

Depende da sonda. Duas linhas possíveis:

1. **Se o vanilla continua a nota na página seguinte** (ou na coluna seguinte): fazer o mesmo — a nota que não cabe numa coluna continua na próxima, em vez de desaparecer.
2. **Se isto for mais complexo do que cabe neste passo**: pelo menos garantir que a nota nunca desaparece sem aviso — por exemplo, um aviso no `Sink` do compilador (`engine.sink.warn`) a dizer que uma nota de rodapé não coube e foi truncada, para o utilizador saber que há um problema, em vez de descobrir sozinho que falta texto.

### Critério de fecho da implementação

- [ ] A nota nunca desaparece em silêncio — ou continua visível nalgum sítio, ou o utilizador recebe aviso claro.
- [ ] Testado com uma nota ainda maior, que não caiba nem numa página inteira, para confirmar o comportamento em caso extremo.
- [ ] Texto normal, com notas pequenas que já cabem, sem regressão.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Comportamento actual confirmado com teste directo.
- [ ] Corrigido — nota visível, ou aviso claro, nunca silêncio.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p595.md`, com hash do commit.
- [ ] Actualizar a entrada da lista de disparidades — deixa de ser "scope-out sem razão", passa a "corrigido" ou "scope-out com razão real", nunca fica como estava.
