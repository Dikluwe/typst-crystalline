---
# P711 — Blocos `context` não herdam o `StyleChain` activo (valor silenciosamente errado)

> **Passo:** 711
> **Data:** 2026-07-10
> **Foco:** P710 encontrou, como achado lateral não corrigido, que `#context [...]` parece resolver com um `StyleChain` "por defeito" em vez do estilo realmente activo no ponto onde o bloco está escrito — `#set text(size: 12pt); #context [ #(10em).to-absolute() ]` deu 116pt (11pt por defeito) em vez de 126pt (12pt definido). Isto não é um erro que pare a compilação — é um número silenciosamente errado, num mecanismo central (`context`) usado em qualquer documento que precise de resolver valores dependentes de estilo. Prioridade alta, dado o alcance potencial.
> **Tipo:** Sonda + Implementação. Prioridade alta.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — bug de mecanismo central que produz valores errados sem erro; sonda obrigatória e cuidado redobrado.

---

## Contexto

Reprodução exacta de P710:

```typst
#set text(size: 12pt)
#context [
  #(10em).to-absolute()
]
```

Esperado (vanilla, e logicamente correcto): `120pt` (10 × 12pt, o tamanho definido). Cristalino: `110pt`ish (10 × 11pt, o tamanho por defeito de `text`, ignorando o `#set` anterior).

---

## Sonda

### Confirmar o alcance do bug, com mais casos

```bash
cat > /tmp/p711-context.typ <<'EOF'
#set text(size: 12pt)
#context [#(10em).to-absolute()]

#set text(size: 20pt)
#context [#(2em).to-absolute()]

#set page(width: 300pt)
#context [#(50%).to-absolute()]
EOF
lab/typst-original/target/release/typst compile /tmp/p711-context.typ /tmp/p711-vanilla.pdf
pdftotext /tmp/p711-vanilla.pdf -
./target/release/typst /tmp/p711-context.typ /tmp/p711-cristalino.pdf
pdftotext /tmp/p711-cristalino.pdf -
```

Confirmar se o problema é só com `text(size:)`, ou também com outras propriedades resolvidas via estilo (`page(width:)` para percentagens, por exemplo).

### Localizar exactamente onde `ContextBlockElem` é resolvido

```bash
grep -rn "ContextBlock\|fn resolve.*context\|Content::ContextBlock" 01_core/src/rules/ 01_core/src/entities/ | head -20
```

Confirmar onde o `StyleChain` usado dentro do bloco `context` é obtido — se está a usar um `StyleChain::default()` ou equivalente, em vez do `StyleChain` realmente activo no ponto de avaliação (`engine.styles` no momento em que o `ContextBlockElem` é encontrado).

### Confirmar se isto afecta outros usos de `context`, não só `.to-absolute()`

```bash
cat > /tmp/p711-medir.typ <<'EOF'
#set text(size: 20pt)
#context [
  #let s = measure[Texto de teste]
  #s.width
]
EOF
lab/typst-original/target/release/typst compile /tmp/p711-medir.typ /tmp/p711-medir-vanilla.pdf
./target/release/typst /tmp/p711-medir.typ /tmp/p711-medir-cristalino.pdf
```

Confirmar se `measure` dentro de `context` também é afectado pelo mesmo problema (mediria com o tamanho errado) — isto seria uma confirmação de que o alcance é mais amplo do que só `.to-absolute()`.

### Critério de fecho da sonda

- [ ] Alcance do bug confirmado com múltiplos casos (`text`, `page`, outras propriedades).
- [ ] Localização exacta de onde o `StyleChain` errado é introduzido, com `file:line`.
- [ ] Confirmado se `measure` e outros usos de `context` são também afectados.

---

## Implementação

Corrigir a resolução de `ContextBlockElem` para usar o `StyleChain` realmente activo no ponto de avaliação (`engine.styles` no momento do encontro do bloco), não um `StyleChain` por defeito.

### Critério de fecho da implementação

- [ ] `context` dentro de blocos com `#set` anterior usa o estilo correcto, testado com múltiplos casos.
- [ ] `measure`/outros usos de `context` corrigidos, se confirmado que também eram afectados.
- [ ] Casos sem `#set` (estilo por defeito) continuam correctos, sem regressão.

---

## Validação

```bash
./target/release/typst /tmp/p711-context.typ /tmp/p711-depois.pdf
pdftotext /tmp/p711-depois.pdf -
```

Comparar com o vanilla já obtido na sonda.

```bash
cargo test --workspace
crystalline-lint .
```

Dado o alcance potencial (qualquer documento que combine `#set` com `context`), correr o corpus de testes já existente e confirmar que nenhum documento anterior desta conversa foi silenciosamente afectado.

### Repetir a reprodução de `cetz` (P700-710)

```bash
cat > /tmp/p711-cetz.typ <<'EOF'
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
EOF
./target/release/typst /tmp/p711-cetz.typ /tmp/p711-cetz.pdf
echo "Exit code: $?"
```

---

## Critério de fecho do passo

- [ ] Sonda completa, alcance e causa exacta confirmados.
- [ ] Corrigido, testado com múltiplos casos.
- [ ] Varredura ampla do corpus existente, sem regressão silenciosa.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] `cetz` re-testado, estado registado.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p711.md`, com hash do commit.
