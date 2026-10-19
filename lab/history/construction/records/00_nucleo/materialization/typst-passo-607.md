---
# P607 — Custo de desempenho da tag extra por heading (P606)

> **Passo:** 607
> **Data:** 2026-07-05
> **Foco:** P606 introduziu uma segunda tag de introspecção por heading (`HeadingForBookmarks`, ao lado de `HeadingForToc`), emitida para qualquer heading normal, não só para quem usa os novos parâmetros. A contagem de tags por heading quase duplicou nalguns testes (6→8, 8→10, 12→16, 18→24). Nenhuma medição de desempenho foi feita. Estas tags alimentam os ciclos de fixpoint já usados para numeração de página em índices — o sítio onde um custo pequeno por heading tende a multiplicar-se. Este passo mede, antes de aceitar como sem custo.
> **Tipo:** Verificação directa.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.** Uma mudança na estrutura de dados de introspecção, usada por todos os documentos com headings, precisa de medição, não de suposição.

---

## Verificação

### Documento com muitos headings

```bash
python3 -c "
for i in range(1, 301):
    print(f'= Secção {i}')
    print(f'Texto da secção {i}.')
" > /tmp/p607-muitos-headings.typ
cat >> /tmp/p607-muitos-headings.typ <<'EOF'
EOF
sed -i '1i #outline()' /tmp/p607-muitos-headings.typ
```

Documento com 300 headings e um índice no início (força o ciclo de fixpoint para resolver números de página).

### Medir antes e depois do commit de P606

```bash
git log --oneline | grep -i "P605\|P606" | tail -5
```

Seguir o método já estabelecido (checkout do commit antes de P606, rebuild, medir; voltar ao commit depois, medir de novo), com `hyperfine`, 3 a 5 execuções de cada lado:

```bash
hyperfine --warmup 1 --runs 5 './target/release/typst /tmp/p607-muitos-headings.typ /tmp/p607-out.pdf'
```

Com o `--timings-json` já usado noutros passos desta sequência, para ver especificamente o tempo de `introspect_ms`.

### Critério de fecho

- [x] Tempo medido antes e depois de P606, para o documento de 300 headings.
- [x] Fase `introspect_ms` isolada, para confirmar se o custo (se existir) está mesmo ligado às tags novas.
- [x] Comparar também com um documento sem `#outline()` nenhum (sem ciclo de fixpoint), para isolar se o custo é geral ou específico do fixpoint.

---

## Decisão

Se o custo for desprezável (dentro da variação normal de medição): registar como confirmado sem regressão, com os números.

Se houver custo real: decidir se é aceitável face ao benefício (paridade correcta de `outlined`/`bookmarked`), ou se vale a pena optimizar — por exemplo, só emitir `HeadingForBookmarks` quando o valor de `bookmarked` for diferente do valor por defeito que já seria implícito, evitando a tag extra no caso comum.

---

## Critério de fecho do passo

- [x] Medição feita com o método já estabelecido (hyperfine, antes/depois, várias execuções).
- [x] Fase de introspecção isolada.
- [x] Decisão registada com números, não com suposição.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p607.md`, com hash do commit.
