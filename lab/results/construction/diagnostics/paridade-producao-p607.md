# Paridade de Produção — P607

**Data do relatório:** 2026-07-08
**Passo:** 607
**Foco:** Medir o custo de desempenho da tag extra `HeadingForBookmarks` introduzida em P606.

---

## Resumo executivo

P606 introduziu uma segunda tag de introspecção por heading (`HeadingForBookmarks`, em paralelo com `HeadingForToc`). A contagem de tags por heading aumentou em alguns testes unitários (6→8, 8→10, 12→16, 18→24), mas nenhuma medição de desempenho tinha sido feita. Este passo mede o impacto real num documento com 300 headings, com e sem `#outline()` (que força o ciclo de fixpoint de numeração de página).

**Conclusão:** o custo existe mas é desprezável. A introspecção aumentou ~0,08 ms para 300 headings (~0,27 µs por heading), uma fração pequena do tempo total de compilação. O tempo total do processo não regrediu — P606 é ligeiramente mais rápido dentro do ruído de medição. Não se justifica optimizar (por exemplo, só emitir a tag quando `bookmarked` difere do defeito), porque o benefício da paridade correcta com o vanilla supera largamente o custo medido.

---

## Proveniência

- **Hash base (P606):** `439d4dbd6697ac1947d0e1351b69349e00670b6b`
- **Hash base comparativo (P605):** `e01e39f68413f33565825ada79d9737061bb2bbe`
- **Data/hora:** 2026-07-08T01:00-03:00 (referência de sessão)
- **Binários usados:**
  - Cristalino P605: `./target/release/typst` (checkout + rebuild do commit P605)
  - Cristalino P606: `./target/release/typst` (checkout + rebuild do commit P606)
- **Ferramentas auxiliares:** `hyperfine 1.19.0`, `--timings-json`

---

## Documentos de teste

### Com outline (força fixpoint)

`/tmp/p607-muitos-headings.typ`:

```typst
#outline()
= Secção 1
Texto da secção 1.
= Secção 2
Texto da secção 2.
...
= Secção 300
Texto da secção 300.
```

601 linhas, 300 headings.

### Sem outline

`/tmp/p607-muitos-headings-sem-outline.typ`:

```typst
= Secção 1
Texto da secção 1.
...
= Secção 300
Texto da secção 300.
```

600 linhas, 300 headings.

---

## Resultados

### Com `#outline()`

| Métrica | P605 (base) | P606 | Diferença |
|---------|-------------|------|-----------|
| `hyperfine` (tempo total processo) | 323,5 ± 4,7 ms | 317,4 ± 3,6 ms | −6,1 ms (−1,9 %) |
| `introspect_ms` (timings JSON) | 0,481 ms | 0,556 ms | +0,075 ms (+15,6 %) |
| `total_ms` (timings JSON) | 156,8 ms | 152,5 ms | −4,3 ms (−2,7 %) |

### Sem `#outline()`

| Métrica | P605 (base) | P606 | Diferença |
|---------|-------------|------|-----------|
| `hyperfine` (tempo total processo) | 262,1 ± 4,9 ms | 261,5 ± 7,5 ms | −0,6 ms (−0,2 %) |
| `introspect_ms` (timings JSON) | 0,487 ms | 0,570 ms | +0,083 ms (+17,0 %) |
| `total_ms` (timings JSON) | 107,4 ms | 112,6 ms | +5,2 ms (+4,8 %) |

---

## Análise

1. **Custo na introspecção:** existe e é pequeno. +0,075 ms / +0,083 ms para 300 headings = aproximadamente 0,25–0,28 µs por heading. A introdução da tag `HeadingForBookmarks` é a causa directa: o walk agora emite, no mínimo, dois pares Start/End adicionais por heading, e a fase `from_tags` popula uma sub-store extra.

2. **Custo no tempo total:** não é observável. O tempo de processo completo medido pelo `hyperfine` diminuiu ligeiramente em ambos os cenários, mas as diferenças estão dentro do ruído de medição (±4–7 ms). O trabalho extra de introspecção é absorvido por outras fases e pela variabilidade normal de I/O, fontes, etc.

3. **Impacto do `#outline()`:** o aumento relativo de `introspect_ms` é semelhante com e sem outline (~15–17 %), o que indica que o custo é proporcional ao número de headings, não específico do ciclo de fixpoint de numeração de página. O outline força uma segunda passagem de layout, mas a fase de introspecção em si é feita uma única vez.

4. **Variação em `total_ms`:** as diferenças em `layout_ms`, `shape_ms` e `render_ms` entre P605 e P606 são ruidosas e não correlacionadas com a mudança de introspecção. Por exemplo, com outline o `total_ms` diminuiu 4,3 ms; sem outline aumentou 5,2 ms. Atribuímos isto a variação de medição, não a P606.

---

## Decisão

**Não optimizar.** O custo adicional é ~0,08 ms para 300 headings — desprezável face ao tempo total de compilação (~260–320 ms). A optimização sugerida (só emitir `HeadingForBookmarks` quando `bookmarked` difere do valor por defeito) introduziria complexidade desnecessária:

- Lógica condicional extra no walk arm `Content::Heading`.
- Assimetria difícil de explicar: o utilizador não veria a tag extra, mas o código teria de saber quando ela é implícita.
- Ganho máximo teórico de ~0,08 ms no caso comum, dentro do ruído de medição.

Mantém-se a implementação de P606 tal como está. A paridade correcta com o vanilla 0.15.0 justifica o custo medido.

---

## Critérios de fecho do passo

- [x] Medição feita com `hyperfine` (5 runs, warmup 1) antes e depois de P606.
- [x] Fase `introspect_ms` isolada via `--timings-json`.
- [x] Documento sem `#outline()` também medido, para isolar o efeito do fixpoint.
- [x] Decisão registada com números: custo de ~0,08 ms / 300 headings, não significativo no tempo total.
- [x] Decisão: não optimizar; manter implementação P606.
- [x] Relatório escrito com proveniência.

---

## Ligações

- `00_nucleo/materialization/typst-passo-607.md` — passo que originou a medição.
- `00_nucleo/diagnosticos/paridade-producao-p606.md` — passo cujo custo foi medido.
- Commits comparados:
  - P605: `e01e39f68413f33565825ada79d9737061bb2bbe`
  - P606: `439d4dbd6697ac1947d0e1351b69349e00670b6b`
