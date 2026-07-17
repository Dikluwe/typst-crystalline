---
# P772a (lote 3) — Varredura da stdlib: módulo seguinte por tamanho

> **Passo:** 772a (continuação da série P765a "lote 0", P765b "lote 1", P772 "lote 2")
> **Data:** 2026-07-16
> **Foco:** P772 classificou `typst_library::visualize::image::raster` (13 itens) — 11 confirmados como infra-estrutura sem efeito, 2 (DPI, rotação EXIF) reclassificados como bugs reais e corrigidos em P773/P774/P776. Os candidatos seguintes, já identificados pela recontagem de P772, são `typst_library::pdf::accessibility` (12 itens), `typst_syntax::span` (10 itens) e `typst_syntax::package` (8 itens) — todos com suspeita inicial de infra-estrutura Rust, mas nenhum verificado item a item ainda. Este passo reconfirma a contagem (pode ter mudado) e classifica o módulo seguinte.
> **Tipo:** Sonda + Implementação directa para achados confirmados como bugs reais.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.** **ADR-0107** — a lição de P772/P773 aplica-se aqui: um módulo pode ser maioritariamente infra-estrutura Rust e ainda assim conter 1-2 itens com efeito observável real (como DPI/rotação dentro de `image::raster`) — classificar item a item, não descartar o módulo inteiro pela primeira impressão.
> **Dependências:** P772 (lote 2, metodologia e lição sobre não descartar módulos inteiros sem verificar cada item).

---

## Sonda — reconfirmar contagem e escolher o módulo

```bash
awk -F'\t' '$1=="lacuna-inventario"' 00_nucleo/diagnosticos/lente-lista-B-2026-07-15.txt \
  | cut -f5 | sed 's/::[^:]*$//' | sort | uniq -c | sort -rn | head -20
```

Excluir os módulos já tratados (`foundations::calc`, `foundations::ops`, `diag`, `math::style`, `image::raster`) e confirmar o módulo seguinte. Se `pdf::accessibility` continuar a ser o maior por tamanho entre os não tratados, prosseguir com ele; caso a lista tenha mudado, seguir a nova ordem.

Para cada item do módulo escolhido, ler o código-fonte antes de classificar — **não assumir infra-estrutura pela categoria do módulo** (lição de P772/P773):

```bash
grep -n "<item>" 01_core/src/engine/stdlib/*.rs 01_core/src/entities/*.rs 01_core/src/engine/eval/*.rs 2>/dev/null
```

Para `pdf::accessibility` especificamente, atenção a itens que possam ter efeito observável mesmo sendo "infra-estrutura de exportação": tags de estrutura PDF, texto alternativo de imagens, idioma do documento, ordem de leitura — todos esses podem afectar o PDF final de forma verificável (abrir o PDF num leitor de acessibilidade ou extrair a árvore de estrutura), mesmo não sendo símbolos de língua Typst directamente expostos ao utilizador.

```bash
grep -n "fn.*structure\|StructTree\|alt_text\|lang\b" lab/typst-original/crates/typst-library/src/pdf/accessibility.rs 2>/dev/null | head -20
```

---

## Implementação

Só para achados classificados como `bug real` (comportamento/saída observável diverge do vanilla, com caso de teste mínimo). Corrigir um a um, com teste e comparação directa — mesmo padrão de P765a/P765b/P773.

Se o módulo revelar mais de ~5 bugs reais distintos, dividir em lotes, não forçar tudo num passo.

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Para cada bug corrigido: caso de teste mínimo comparando vanilla vs cristalino, usando a metodologia já padronizada (`mutool draw -r 300` + `compare -metric AE` para efeitos visuais; extracção de estrutura/texto para efeitos de acessibilidade/metadados, conforme o tipo de item).

---

## Critério de fecho do passo

- [ ] Contagem por módulo reconfirmada directamente.
- [ ] Módulo seguinte classificado item a item, sem descartar por categoria.
- [ ] Atenção especial a itens de `pdf::accessibility` com efeito observável no PDF final (estrutura, alt-text, idioma), não só símbolos de língua.
- [ ] Bugs reais corrigidos um a um, com teste e comparação directa.
- [ ] Volume dividido em lotes se exceder o razoável.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772a.md`, com a tabela de classificação completa.

---

## Próximo passo

P772b (lote 4): módulo seguinte por tamanho (`typst_syntax::span` ou `typst_syntax::package`, conforme a reconfirmação), mesma metodologia — continuar até a lista `lacuna-inventario` estar coberta ou restarem só módulos de infra-estrutura sem itens de efeito observável confirmado.
