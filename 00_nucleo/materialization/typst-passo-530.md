---
# P530 — Implementar fix real de Variation Fonts (instanciação estática)

> **Passo:** 530
> **Data:** 2026-07-01
> **Foco:** Implementar a pipeline validada em P529: subsetar a fonte VF com `oxifont-subset`, remover `GPOS`/`GSUB`/`GDEF` do subset, instanciar estaticamente com `fontTools` para cada combinação `(FontList, FontVariant)` usada no documento, e embutir cada instância como fonte separada no PDF.
> **Tipo:** Implementação.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR** — medir antes de decidir; medir depois de cada sub-tarefa.
> **ADR-0109 EM VIGOR** — atomização; sub-tarefas independentes onde possível.
> **ADR-0107 EM VIGOR** — `weight`/`stretch`/`style` são semântica de linguagem, não mecânica.
> **Dependências:** P525 (shaper aplica coordenadas de eixo correctamente), P527 (confirmação da regressão visual), P529 (pipeline de instanciação validada, 0.3s por combinação).

---

## Contexto

P529 confirmou que a instanciação estática é viável, com uma condição: tem de ser feita depois do subsetting (`oxifont-subset`), não antes. A ordem errada (instanciar a fonte completa e só depois subsetar) demorava cerca de 4 minutos por combinação de peso/estilo. A ordem correcta (subsetar primeiro, depois instanciar a fonte já pequena) demora cerca de 0,3 segundos.

A pipeline confirmada:

```
oxifont-subset (Rust)
  → fonte VF pequena, só com os glifos usados
  → remover GPOS/GSUB/GDEF (o shaper já aplicou kerning/ligatures antes; não são necessários depois)
  → fontTools.varLib.instancer (Python, rápido sobre fonte pequena)
  → fonte estática por combinação (FontList, FontVariant)
  → embutir cada uma no PDF como fonte separada
```

---

## Sub-tarefa 0 — Sonda: dependência de runtime em Python

Este passo introduz uma chamada a `fontTools` (Python) durante a compilação de qualquer documento que use uma fonte VF com peso ou estilo diferente do default. Isto é diferente das outras dependências do projecto, que são todas crates Rust compiladas junto com o binário. Antes de implementar, confirmar três coisas:

1. **O binário `typst-wiring` vai depender de um Python instalado na máquina onde corre**, não só na máquina de desenvolvimento. Confirmar se isto é aceitável para o projecto, ou se precisa de ficar registado como limitação conhecida.
2. **Custo fixo por chamada.** Cada `subprocess` para o Python tem um custo de arranque (iniciar o interpretador, importar `fontTools`). Medir esse custo isolado, sem contar a instanciação em si:

```bash
time lab/.venv/bin/python3 -c "from fontTools.ttLib import TTFont; from fontTools.varLib.instancer import instantiateVariableFont"
```

3. **Comportamento se o Python ou o `fontTools` não estiverem disponíveis.** O que deve acontecer: erro claro a dizer que falta a dependência, ou fallback silencioso para a instância default (o comportamento actual, antes deste passo)? Decidir antes de escrever código, não durante.

### Critério de fecho

- [ ] Custo de arranque do subprocess medido isoladamente.
- [ ] Decisão registada: Python como dependência de runtime obrigatória, ou com fallback.
- [ ] Se for dependência obrigatória: mensagem de erro a escrever, caso falte.

---

## Sub-tarefa 1 — Colectar combinações `(FontList, FontVariant)` do documento

### 1.1 Localização

`03_infra/src/pipeline.rs`, onde `collect_fonts_from_doc` agrupa fontes hoje (confirmado em P527/P528 que agrupa só por `FontList`, sem `weight`/`style`).

### 1.2 Mudança

Agrupar por `(FontList, FontVariant)` em vez de só `FontList`. Cada combinação distinta usada no documento (ex.: Ubuntu Sans + peso 400, Ubuntu Sans + peso 700, Ubuntu Sans + peso 400 + itálico) fica como uma entrada separada a processar.

### Critério de fecho

- [ ] Função de colecta devolve uma lista de `(FontList, FontVariant)` únicas.
- [ ] Teste unitário: documento com 3 pesos da mesma fonte produz 3 combinações, não 1.

---

## Sub-tarefa 2 — Subsetar, remover tabelas, instanciar

### 2.1 Para cada combinação da Sub-tarefa 1

1. Subsetar a fonte VF original com `oxifont-subset`, usando só os glifos que aparecem nessa combinação (não a união de todos os glifos do documento — cada combinação pode ter um conjunto de glifos diferente, mas normalmente vai ser o mesmo texto em pesos diferentes).
2. Remover `GPOS`, `GSUB`, `GDEF` do subset.
3. Chamar `fontTools.varLib.instancer.instantiateVariableFont` com as coordenadas de eixo da `FontVariant` (`wght`, `wdth`, `ital`/`slnt` conforme mapeamento já feito em P525).
4. Resultado: uma fonte estática pequena, específica para essa combinação.

### 2.2 Reaproveitamento entre combinações

Se duas combinações usam exactamente o mesmo conjunto de glifos e a mesma fonte, mas diferem noutra coisa (isto não deve acontecer se a chave é `(FontList, FontVariant)` — cada combinação já é única por definição), não há reaproveitamento a fazer aqui. Se o subset de glifos for igual entre combinações diferentes (ex.: peso 400 e peso 700 do mesmo texto), o subset de glifos pode ser partilhado antes da instanciação, para poupar uma chamada ao `oxifont-subset` — mas a instanciação em si tem de correr uma vez por combinação, porque o resultado é diferente.

### Critério de fecho

- [ ] Cada combinação produz uma fonte estática distinta.
- [ ] Tempo total medido para um documento com 4 combinações (o mesmo caso de teste de P525/P527: Regular, Bold, Thin, Italic).
- [ ] Sem erro ao remover `GPOS`/`GSUB`/`GDEF` mesmo quando alguma dessas tabelas não existir no subset.

---

## Sub-tarefa 3 — `export_pdf_multifont` indexado por combinação

### 3.1 Localização

`03_infra/src/export/builder.rs`, função `build_multifont` (recebe hoje `&[(FontList, Vec<u8>)]`, confirmado em P528).

### 3.2 Mudança

Passar a receber `&[((FontList, FontVariant), Vec<u8>)]` — uma entrada por combinação, não por fonte. Cada entrada gera uma fonte embutida separada no PDF, com o seu próprio identificador (`/F1`, `/F2`, etc.).

### Critério de fecho

- [ ] `build_multifont` aceita a nova assinatura.
- [ ] PDF resultante tem uma entrada de fonte por combinação usada no documento.
- [ ] Testes existentes de `build_multifont` (fontes estáticas, sem VF) continuam a passar sem alteração de comportamento.

---

## Sub-tarefa 4 — `emit_shaped_pdf` selecciona a fonte correcta

### 4.1 Localização

`03_infra/src/export/stream.rs`, `emit_shaped_pdf` (confirmado em P528 que selecciona hoje só por `style.font`, ignorando peso/estilo).

### 4.2 Mudança

Seleccionar o identificador de fonte (`/F{}`) com base na combinação `(style.font, style.variant)`, não só em `style.font`.

### Critério de fecho

- [ ] Cada trecho de texto no content stream referencia a fonte correcta para o seu peso/estilo.
- [ ] Texto com pesos diferentes no mesmo documento usa operadores `Tf` diferentes.

---

## Sub-tarefa 5 — Validação empírica

### 5.1 Documento de teste

Reutilizar o documento de P525/P527:

```typst
#set text(font: "Ubuntu Sans", size: 40pt)
Hello world.

#set text(weight: 700)
Bold hello.

#set text(weight: 100)
Thin hello.

#set text(weight: 400, style: "italic")
Italic hello.
```

### 5.2 Critérios de validação

| Critério | Método | Esperado |
|----------|--------|----------|
| Compilação sem erro | Exit code | 0 |
| Fontes embutidas | `pdffonts` | 4 entradas, uma por combinação |
| Peso 700 visivelmente mais grosso que 400 | Comparação visual | Diferença visível |
| Peso 100 visivelmente mais fino que 400 | Comparação visual | Diferença visível |
| Itálico visivelmente inclinado | Comparação visual | Diferença visível |
| Texto extraível | `pdftotext` | Todas as strings correctas |
| Tempo de compilação | `time` | Registar; comparar com o documento antes deste passo |

### 5.3 Comparação com vanilla

```bash
./lab/typst-original/target/release/typst compile /tmp/test-vf-p530.typ /tmp/vf-vanilla-p530.pdf
pdffonts /tmp/vf-vanilla-p530.pdf
```

Confirmar que o número de fontes embutidas é o mesmo (4), e que o resultado visual é equivalente.

### Critério de fecho

- [ ] Todos os critérios da tabela 5.2 confirmados.
- [ ] Comparação com vanilla feita.

---

## Sub-tarefa 6 — Bateria de paridade, benchmark, linter

```bash
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ lab/parity/corpus/p520/*.typ; do
  ./target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1 && echo "OK: $(basename $f)" || echo "FAIL: $(basename $f)"
done

./target/release/typst lab/parity/corpus/p523/test-cff-nimbus.typ /tmp/out.pdf >/dev/null 2>&1 && echo "OK: cff-nimbus" || echo "FAIL: cff-nimbus"

python3 tools/perf/benchmark-p507.py

crystalline-lint .
```

O benchmark é o ponto mais importante aqui: documentos sem VF não devem ficar mais lentos (a nova pipeline só deve activar quando houver combinação de peso/estilo diferente do default numa fonte VF). Confirmar isso explicitamente, não só olhar para a média geral.

### Critério de fecho

- [ ] Corpus sem VF: sem regressão de tempo.
- [ ] Corpus com VF (novo, desta sub-tarefa): tempo registado e considerado aceitável.
- [ ] Linter limpo.

---

## Sub-tarefa 7 — Documentação

Actualizar:

- `00_nucleo/diagnosticos/cristalino-contexto-handoff.md` — linha de Variation Fonts passa de "Parcial" para "Fechado em P530", com nota sobre a dependência de Python em runtime (resultado da Sub-tarefa 0).
- `00_nucleo/prompts/infra/export/font_subset.md` ou equivalente — pipeline de instanciação documentada.
- Registar a dependência de Python como algo a verificar em qualquer ambiente novo onde o projecto for instalado, não só como nota interna.

### Critério de fecho

- [ ] Handoff actualizado.
- [ ] Dependência de Python em runtime documentada de forma visível, não escondida numa nota de rodapé.

---

## Critério de fecho do passo

- [ ] Sub-tarefa 0: decisão sobre dependência de Python tomada e registada.
- [ ] Sub-tarefas 1–4: implementação completa.
- [ ] Sub-tarefa 5: validação empírica com os 7 critérios.
- [ ] Sub-tarefa 6: sem regressão em corpus, benchmark, linter.
- [ ] Sub-tarefa 7: documentação actualizada.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p530.md`.

---

## Próximo passo

Com Variation Fonts fechado, retomar as opções do handoff: Lookahead (P519), publicação, SVG/PNG/HTML export (sonda já feita em P526), ou optimização.
