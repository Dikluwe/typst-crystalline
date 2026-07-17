---
# P661 — `table.numbering` responde a `#show figure.where(kind: table): ...`?

> **Passo:** 661
> **Data:** 2026-07-09
> **Foco:** A explicação recebida sobre o design do vanilla confirma que `figure` existe para dar estilo consistente a qualquer conteúdo (tabela, imagem, código) através de `#show figure.where(kind: table): set figure.caption(...)`. `table.numbering` (extensão P459 do cristalino) não passa pelo `figure` — confirmar se isso significa que fica fora desse mecanismo de estilo unificado, o que seria uma limitação real da extensão, não só uma diferença de sintaxe.
> **Tipo:** Verificação directa.
> **Tamanho:** XS.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P459 (extensão original), P639 (confirmação de que é extensão, não paridade).

---

## Verificação

```bash
cat > /tmp/p661-show-figure.typ <<'EOF'
#show figure.where(kind: table): set figure.caption(position: top)

#set table(numbering: "1")
#table(
  columns: 2,
  caption: [Tabela via extensão P459],
  [A], [B],
)

#figure(
  table(columns: 2, [C], [D]),
  caption: [Tabela via figure normal],
)
EOF
./target/release/typst /tmp/p661-show-figure.typ /tmp/p661.pdf
pdftotext /tmp/p661.pdf -
```

Confirmar se a legenda da primeira tabela (via `table.numbering`/`caption`) respeita a regra `position: top`, ou se só a segunda (via `figure` normal) responde à regra.

---

## Decisão

Se `table.numbering` não responder à regra: registar como limitação conhecida da extensão P459, com a razão explicada (não passa pelo mecanismo de `figure`), e decidir se vale a pena estender a extensão para também disparar o `show` de `figure`, ou se fica documentado como está.

Se responder: confirmar como, e documentar o mecanismo.

---

## Critério de fecho do passo

- [ ] Testado directamente, resultado confirmado.
- [ ] Decisão registada — limitação aceite com razão, ou passo de extensão futuro proposto.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p661.md`.
- [ ] Documentação da extensão P459 (onde estiver registada) actualizada com esta limitação, se confirmada.
