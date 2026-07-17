---
# P763g — Checklist de sub-layouts + investigação/correcção: AE=2730 em `line`+`circle` nativo

> **Passo:** 763g
> **Data:** 2026-07-15
> **Foco:** P763f corrigiu o deslocamento do canvas do `cetz` (origem de `place` dentro de sub-frames), com coordenadas confirmadas a 0.01pt. Duas lacunas ficaram em aberto: (1) a correcção mexeu em `layout_place` sob `is_sub_frame`, mas a validação só cobriu `rect`/`place`/`block+align` — falta `grid` e `columns`, os outros dois sub-layouts que a regra 5 do handoff exige verificar sempre que um mecanismo de posicionamento é corrigido; (2) o documento nativo `#line(...)` + `#circle(...)` mantém AE=2730, mas a explicação anterior ("deslocamento próprio do círculo, ~13pt") foi contradita pela própria medição de P763f — `#circle(radius: 10pt)` isolado deu AE=271, praticamente baseline. A causa real do 2730 ainda não foi identificada.
> **Tipo:** Verificação (checklist) + Investigação com correcção condicional.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — medir antes de fechar; não repetir uma explicação já contradita pelos próprios dados.
> **Dependências:** P763f (correcção de `layout_place`, commit `b33706d208f2518acd8bdf032430e50437bd0e07`).

---

## Parte A — Checklist de sub-layouts (regra 5)

Testar a correcção de `layout_place` dentro dos dois sub-layouts que faltaram, com o mesmo mecanismo que expôs o bug original (`place` dentro de sub-frame):

```bash
cat > /tmp/p763g-grid.typ <<'EOF'
#set page(width: 8cm, height: 6cm)
#grid(
  columns: 2, gutter: 5pt,
  block(width: 3cm, height: 2cm, align(top, place(top+left, dx: 5pt, dy: 5pt, circle(radius: 10pt)))),
  block(width: 3cm, height: 2cm, align(top, place(top+left, dx: 5pt, dy: 5pt, circle(radius: 10pt)))),
)
EOF

cat > /tmp/p763g-columns.typ <<'EOF'
#set page(width: 8cm, height: 6cm)
#columns(2)[
  #block(width: 3cm, height: 2cm, align(top, place(top+left, dx: 5pt, dy: 5pt, circle(radius: 10pt))))
  #colbreak()
  #block(width: 3cm, height: 2cm, align(top, place(top+left, dx: 5pt, dy: 5pt, circle(radius: 10pt))))
]
EOF

for doc in grid columns; do
  lab/typst-original/target/release/typst compile /tmp/p763g-$doc.typ /tmp/p763g-$doc-vanilla.pdf
  ./target/release/typst compile /tmp/p763g-$doc.typ /tmp/p763g-$doc-cristalino.pdf
  mutool draw -o /tmp/p763g-$doc-vanilla.png -r 300 /tmp/p763g-$doc-vanilla.pdf
  mutool draw -o /tmp/p763g-$doc-cristalino.png -r 300 /tmp/p763g-$doc-cristalino.pdf
  echo "=== $doc ==="
  compare -metric AE /tmp/p763g-$doc-vanilla.png /tmp/p763g-$doc-cristalino.png /tmp/p763g-$doc-diff.png
done
```

Regra de medição obrigatória (herdada de P763e/P763f): sempre rasterizar com `mutool draw -r 300` antes de `compare`; comparação directa de PDF nunca é critério de fecho.

Se algum dos dois casos falhar (AE fora do baseline ≈241-300), a correcção de P763f está incompleta para esses sub-layouts — não fechar o checklist, tratar como achado a corrigir dentro deste mesmo passo (mesma função `layout_place`, mesma disciplina de leitura de código antes de alterar).

---

## Parte B — Investigar o AE=2730 do nativo `line`+`circle`

### Isolar de novo, com mais granularidade que P763c/P763f

```bash
cat > /tmp/p763g-line-so.typ <<'EOF'
#set page(width: 8cm, height: 4cm)
#line(start: (0pt, 0pt), end: (56pt, 28pt))
EOF

cat > /tmp/p763g-circle-so.typ <<'EOF'
#set page(width: 8cm, height: 4cm)
#circle(radius: 10pt)
EOF

cat > /tmp/p763g-line-circle.typ <<'EOF'
#set page(width: 8cm, height: 4cm)
#line(start: (0pt, 0pt), end: (56pt, 28pt))
#circle(radius: 10pt)
EOF
```

Medir os três (rasterizado) e comparar:
- `line` sozinho vs baseline.
- `circle` sozinho vs baseline (já confirmado 271 por P763f — reproduzir para garantir consistência).
- Os dois juntos, em fluxo normal do documento (um bloco depois do outro, sem `place`).

Se `line` sozinho e `circle` sozinho baterem, mas a combinação não, a causa está na **interacção entre dois elementos consecutivos no fluxo do documento** — não numa forma isolada. Hipóteses a confirmar por leitura de código, não por suposição:

```bash
grep -rn "fn.*flow\|fn.*stack\|cursor_y\|advance" 01_core/src/engine/layout/cursor.rs 2>/dev/null | head -30
```

1. O espaçamento entre blocos consecutivos de conteúdo de desenho (`line`, `circle`) pode estar a usar uma métrica de avanço diferente da do vanilla (a mesma família de bug de P745-762, mas para elementos de desenho, não texto).
2. A bounding box declarada por `line`/`circle` para efeitos de avanço do cursor pode estar incorrecta para um dos dois, mesmo que a própria forma renderize correctamente quando isolada.

### Coordenadas exactas

```bash
mutool trace /tmp/p763g-line-circle-vanilla.pdf > /tmp/p763g-trace-vanilla.txt
mutool trace /tmp/p763g-line-circle-cristalino.pdf > /tmp/p763g-trace-cristalino.txt
diff /tmp/p763g-trace-vanilla.txt /tmp/p763g-trace-cristalino.txt
```

Confirmar exactamente qual dos dois elementos (ou os dois) está deslocado, e por quanto — não repetir a hipótese de P763c ("deslocamento próprio do círculo") sem voltar a confirmá-la com este documento específico, já que P763f mostrou que ela não se sustenta para o círculo isolado.

### Implementação (só se a causa for confirmada e for um bug real, não diferença aceitável)

Corrigir conforme a causa raiz encontrada — avanço de cursor entre elementos de desenho, ou bounding box declarada, conforme o que a leitura de código confirmar.

---

## Critério de fecho do passo

- [ ] `grid` e `columns` testados com o mecanismo de `place`-em-sub-frame corrigido por P763f; AE no baseline em ambos, ou achado corrigido dentro deste passo.
- [ ] `line` e `circle` isolados remedidos, confirmando consistência com P763f (271 para `circle`).
- [ ] Causa real do AE=2730 na combinação identificada por leitura de código e confirmada por coordenadas (`mutool trace`), não assumida por analogia com o achado de P763c.
- [ ] Se bug real: corrigido, com AE final e coordenadas registadas antes/depois.
- [ ] Se diferença aceitável (ex: espaçamento por defeito entre blocos que o vanilla também aplica, confirmado por leitura do código do vanilla): registado como tal, com evidência, não suposição.
- [ ] Toda medição de AE feita via `mutool draw -r 300` → `compare`; comparação directa de PDF nunca usada como critério de fecho.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p763g.md`.

---

## Próximo passo

Se tudo fechar: a linha de trabalho `cetz`/download de pacotes (P763–P763g) fecha definitivamente, com checklist de sub-layouts completo e sem contradições em aberto entre relatórios.
Se a Parte B revelar uma causa estrutural maior (ex: avanço de cursor entre elementos de desenho, afectando mais do que só `line`+`circle`): abrir passo dedicado fora da numeração P763, dado que deixaria de ser específico de pacotes/`cetz`.
