---
# P666 — Retomar pendência de P525: fontes variáveis embutem só a instância default

> **Passo:** 666
> **Data:** 2026-07-10
> **Foco:** P525 (há muitos passos atrás) já descobriu e documentou, com clareza, que o export PDF de fontes variáveis embute sempre a instância default dos contornos, independentemente do `weight` pedido. `#set text(weight: 700)` produz o espaçamento certo de um negrito, mas desenha as letras finas — visualmente indistinguível de `weight: 400`. Foi classificado como "regressão de linguagem" pelo próprio P525, listado como pendência, e nunca retomado desde então. Este passo confirma se ainda se aplica, e corrige.
> **Tipo:** Sonda + Implementação.
> **Tamanho:** L. Envolve instanciação estática de fontes variáveis por combinação peso/estilo, e mudança na chave de recolha de fontes para export.
> **ADR-0108 EM VIGOR.** **ADR-0114 EM VIGOR** — uma pendência antiga, esquecida, precisa da mesma sonda obrigatória antes de qualquer código, para confirmar que o estado ainda é o descrito por P525 e não mudou por acidente entretanto.

---

## Sonda

### Confirmar se o problema ainda existe

Repetir exactamente o teste de P525:

```bash
cat > /tmp/p666-vf-pesos.typ <<'EOF'
#set text(font: "Ubuntu Sans", size: 40pt)
Hello world.

#set text(weight: 700)
Bold hello.

#set text(weight: 100)
Thin hello.
EOF
./target/release/typst /tmp/p666-vf-pesos.typ /tmp/p666.pdf
pdffonts /tmp/p666.pdf
mutool draw -o /tmp/p666.png -r 150 /tmp/p666.pdf
```

Confirmar visualmente: as três linhas têm pesos visuais diferentes, ou parecem todas iguais apesar do espaçamento diferente?

```bash
grep -n "fn resolve_font\|FontVariant::default()" 03_infra/src/pipeline.rs | head -10
```

Confirmar se `resolve_font` ainda usa `FontVariant::default()` independentemente do peso pedido, como P525 documentou.

### Confirmar o que mudou entretanto (P659/P660/P662)

P659 corrigiu a chave de cache de `shaped_width` para incluir eixos de variação — mas isso é sobre a **medição** de largura, não sobre o **export** de contornos. P660/P662 introduziram e depois reverteram `variant: (eixo: valor)` — isso também não tocou no problema de export identificado por P525. Confirmar que nenhum destes passos resolveu, por acidente, o problema de P525, e que a pendência continua exactamente onde estava.

### Critério de fecho da sonda

- [ ] Confirmado, visualmente e com `pdffonts`, se o problema de P525 ainda existe.
- [ ] Confirmado que `resolve_font`/`collect_fonts_from_doc` ainda agrupam por `FontList` sem weight/style.
- [ ] Confirmado que nenhum passo posterior resolveu isto por acidente.

---

## Implementação

Seguindo a recomendação já escrita por P525: instanciar estaticamente a fonte variável para cada combinação peso/estilo efectivamente usada no documento (o que o vanilla já faz), e embutir cada instância como uma fonte separada, em vez de uma só instância default partilhada por todos os pesos.

### Critério de fecho da implementação

- [ ] `collect_fonts_from_doc` (ou equivalente) agrupa por `(FontList, FontVariant)`, não só `FontList`.
- [ ] Cada combinação peso/estilo usada no documento gera uma instância estática separada, embutida com os contornos correctos para esse peso.
- [ ] Testado com o documento de P525/P666: as três linhas (regular, bold, thin) mostram pesos visuais diferentes no PDF final.
- [ ] Confirmado que o subsetting (já correcto desde P609/P610 para colecções de fonte) continua a funcionar correctamente com múltiplas instâncias da mesma VF.

---

## Validação

```bash
./target/release/typst /tmp/p666-vf-pesos.typ /tmp/p666-depois.pdf
pdffonts /tmp/p666-depois.pdf
mutool draw -o /tmp/p666-depois.png -r 150 /tmp/p666-depois.pdf
```

Confirmar visualmente a diferença de peso, e `pdffonts` a mostrar múltiplas instâncias (não uma só `CID TrueType` genérica como P525 relatou).

Comparar com o vanilla no mesmo documento (ajustando a sintaxe, já que `italic: true` foi revertido em P665 — usar `style: "italic"`):

```bash
cat > /tmp/p666-vf-vanilla.typ <<'EOF'
#set text(font: "Ubuntu Sans", size: 40pt)
Hello world.

#set text(weight: 700)
Bold hello.

#set text(weight: 100)
Thin hello.
EOF
lab/typst-original/target/release/typst compile /tmp/p666-vf-vanilla.typ /tmp/p666-vanilla.pdf
pdffonts /tmp/p666-vanilla.pdf
```

```bash
cargo test --workspace
crystalline-lint .
```

Verificar impacto no tamanho de ficheiro — múltiplas instâncias embutidas podem aumentar o tamanho do PDF; confirmar que o subsetting mantém isso razoável, seguindo a disciplina já estabelecida em P609/P610.

---

## Critério de fecho do passo

- [ ] Sonda completa, confirmando que a pendência de P525 continua válida.
- [ ] Instanciação estática por peso/estilo implementada.
- [ ] Testado visualmente, pesos diferentes agora visíveis.
- [ ] Subsetting continua eficaz com múltiplas instâncias.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p666.md`, com hash do commit.
- [ ] Pendência de P525 marcada como fechada, com ligação a este passo.
