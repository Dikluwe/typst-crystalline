# Roteiro para completar a refatoração — typst-cristalino

Base: inventário `typst-cobertura-vanilla-vs-cristalino.md` (snapshot P447/P453),
scope-outs acumulados dos passos P388–P456, e DEBT.md atual (DEBT-2 aberta;
restantes fechadas/dissolvidas).

Objetivo deste documento: substituir o menu solto de "próximos passos" que
aparece no fim de cada spec por uma lista fixa de tópicos, com dependências e
ordem. Não é um cronograma; é a lista do que falta e em que ordem faz sentido.

---

## Regras do roteiro (gates a aplicar em cada passo)

Estas regras existem porque a deriva recorrente nesta série foi escrever a spec
antes de medir o estado real. Cada passo deste roteiro cumpre:

1. **Sonda A.0 real antes da spec (ADR-0114).** A sonda é `grep`/diagnóstico
   executado, com `file:line` + commit anexados. Não é preenchida de memória.
   Casos que violaram isto: P388, P409, P413, P416, P421, P447, P451, P452.
2. **Verificar fronteiras/ADR vigentes antes de propor estrutura (ADR-0117
   Cláusula 4).** Antes de propor campo novo num elemento ou módulo novo,
   confirmar a decisão que fixou a forma atual. Casos: P427 (L0 `stream.md`),
   P454 (P365 chain vs campo), P456 (aplicou a regra corretamente).
3. **Declarar divergência de paridade de saída (ADR-0107).** Se o texto
   renderizado diverge do vanilla por default (ex.: prefixo "Figura" vs
   "Figure"), registar no inventário de cobertura, não só como scope-out.
4. **Confirmar o gate da ADR-0117 no linter.** O gate "spec propõe ficheiro que
   já existe" só fecha o ciclo se estiver implementado no `crystalline-lint`. Se
   ainda é proposta, a disciplina continua manual.

---

## Trilhas

Cada trilha tem um objetivo, os passos-tópico que a compõem, a dependência, o
tamanho estimado e o estado. "Pronto" = substrato existe, pode começar.
"Bloqueado" = falta infraestrutura.

### Trilha 1 — Numeração restante

Fecha a família de contadores iniciada em P451 (heading) e P454 (figure).

| Passo-tópico | Dependência | Tamanho | Estado |
|---|---|---|---|
| Equation numbering (`math.equation.numbering` na chain, número à direita) | P451 `format_counter` | S | Pronto — spec P456 escrita, falta executar |
| Table of contents / `outline()` refinado (usa numbering de heading) | P451 | S-M | Pronto |
| Numeração de tabela (contador `"table"`, prefixo de caption) | P454 padrão | S | Pronto |

Nota: P456 já está specado e aplica corretamente o padrão da chain. Falta só o
relatório de execução.

### Trilha 2 — Referências cruzadas e navegação interna

`label`/`ref` foi scope-out desde P452. É a maior lacuna estrutural restante.

| Passo-tópico | Dependência | Tamanho | Estado |
|---|---|---|---|
| `label<x>` — tabela de destinos nomeados no documento | introspecção | M | Pronto |
| `ref<x>` / `@x` — resolução de destino + texto da referência | numbering (T1) | M | Depende de T1 |
| PDF `/Dests` + links internos `/GoTo` | export annotations (P424) | M | Pronto |
| List of figures / list of tables | label/ref + T1 | S | Depende de ref |

Ordem interna: label primeiro, depois ref (precisa dos números de T1), depois
/Dests no PDF, depois LoF/LoT.

### Trilha 3 — Selectors completos em show rules

Fecha o `#show ...where(...)` e o split do trecho casado do regex.

| Passo-tópico | Dependência | Tamanho | Estado |
|---|---|---|---|
| `Selector::Where` materializado no consumer (se ainda parcial) | — | S-M | Confirmar estado real com sonda |
| `#show elem.where(field: value): ...` | `Selector::Where` | S | Depende do acima |
| `#show regex(...)` — split do trecho casado (P393 scope-out) | — | M | Pronto |

Nota: o inventário marca `.where(field:)` como `ausente` (precisa
`Selector::Where`) na linha 111, mas P417/P423 mexeram em `Selector`. A primeira
ação desta trilha é uma sonda que resolva essa contradição entre o inventário e
o código.

### Trilha 4 — Tipos visuais: gradientes, espaços de cor, tiling

Cluster `Visualize`: 5 `ausente` + tiling com fallback. Abre pelo tipo (gate
ADR-0017), depois consumer, depois render PDF.

| Passo-tópico | Dependência | Tamanho | Estado |
|---|---|---|---|
| `Value::Gradient` tipo real (hoje placeholder em `TilingBody`) | gate ADR-0017 | M | Pronto |
| `gradient.linear/radial/conic` consumers | `Value::Gradient` | M | Depende do tipo |
| Render PDF de gradiente (`/Sh` shading operators) | consumers | L | Depende dos consumers |
| Espaços de cor: `cmyk`, `oklab`, `oklch`, `linear_rgb`, `hsl`, `hsv` | `Value::Color` estende | M | Pronto |
| Render PDF de tiling/pattern fill (P395 deixou Color fallback) | `Value::Tiling` (existe) | M | Pronto |

Esta trilha é grande no total. Pode ser fatiada e intercalada com trilhas mais
pequenas.

### Trilha 5 — Shaping / rustybuzz (DEBT-53)

Cluster de texto que depende de integração rustybuzz. É o maior bloco de
`ausente` em Text features.

| Passo-tópico | Dependência | Tamanho | Estado |
|---|---|---|---|
| smallcaps OpenType `smcp`/`c2sc` nativo (P446 fez fallback por scaling) | rustybuzz | M | Bloqueado por shaping |
| `text.dir` / RTL / bidi | rustybuzz + bidi | L | Bloqueado |
| `text.region` (variantes regionais) | rustybuzz | M | Bloqueado |
| `text.script` | rustybuzz | M | Bloqueado |
| Soft hyphen `\u{00AD}` na quebra de linha | hyphenation | S | Parcialmente bloqueado |

Esta trilha é candidata a ser tratada como um épico próprio, com uma sonda de
viabilidade de rustybuzz a abrir (mesmo padrão da sonda de P388 para o runtime
de introspecção). Não começar pelas folhas; sondar o substrato primeiro.

### Trilha 6 — Bibliografia Fase 2

DEBT-55 fechou a Fase 1 (autor-data + alfabética). A Fase 2 foi explicitamente
diferida em P388.

| Passo-tópico | Dependência | Tamanho | Estado |
|---|---|---|---|
| Estilos numéricos (`[1]`, `[2]`) | runtime introspecção 2-pass | M | Pronto (substrato confirmado em P388) |
| Numeração por ordem de aparição | introspecção ordenada | M | Pronto |
| Back-references ("ver [3]") | resolução de location estável | M | Pronto |
| `ibid` / `op. cit.` | acima | S | Depende dos anteriores |
| Múltiplas bibliografias num documento | P420 deixou scope-out | M | Pronto |

A sonda de P388 confirmou que `query_by_kind`, `position_of` e
`layout_with_introspector` existem. O substrato 2-pass está lá; falta a
materialização.

### Trilha 7 — Layout multi-região

Os refactors pesados do Layouter, diferidos por ADR.

| Passo-tópico | Dependência | Tamanho | Estado |
|---|---|---|---|
| `columns`/`colbreak` fluxo multi-região real (hoje width reduzida, single-render) | refactor Layouter | L | Pronto, pesado |
| `measure()` queries de runtime genuínas (ADR-0066) | introspecção | L | Pronto, pesado |
| Elemento `table` real (hoje só `grid`) | grid existente | M | Pronto |

### Trilha 8 — Refinos de stdlib e tipos (preenchimento de baixo risco)

Itens `parcial` pequenos, bons para intercalar entre trilhas grandes.

| Passo-tópico | Tamanho | Estado |
|---|---|---|
| `repr()` completo (hoje subset) | S | Pronto |
| Métodos restantes de `array`/`dict`/`str` | S | Pronto |
| Tipos `Value`: `Relative` (Rel<Length>), `Symbol`, `Dyn` | S cada | Pronto |
| `pad`/`corners`/`sides` inset modeling | S | Pronto |
| Parâmetros configuráveis de `sub`/`super`/`highlight`/decorações (offset, extent, size) | S cada | Pronto |
| Marcadores configuráveis de `list`/`enum` | S | Pronto |
| Prefixo i18n de caption ("Figure" vs "Figura") — divergência declarada de P454 | S | Pronto |

### Trilha 9 — DEBT-2 (encerrada em P458)

| Passo-tópico | Dependência | Tamanho | Estado |
|---|---|---|---|
| Captura lazy de closures (semântica reativa) | `comemo`/`TrackedWorld` | — | **Encerrado** |

A suposta divergência foi refutada no Passo 458. O teste de paridade empírico
mostrou que o Typst vanilla **também** devolve `1` para
`#let x=1; #let f()=x; #let x=2; f()` — isto é, o vanilla é eager para esta
construção, tal como o cristalino. Não há débito a corrigir.

A infraestrutura de introspecção (oráculo de duas passagens) continua a ser
necessária para outras funcionalidades (TOC, `ref`, bibliography Fase 2,
counters), mas essas têm os seus próprios rastreadores nas Trilhas 1, 2 e 6.
O DEBT-2 não bloqueia mais nenhuma trilha.

---

## Ordem sugerida

A ordem considera dependências e custo. Não é obrigatória.

1. **Fechar a Trilha 1** (executar P456 equation numbering; depois TOC e tabela).
   Pequena, desbloqueia referências.
2. **Trilha 2** (label/ref + /Dests). É a maior lacuna estrutural e depende de T1.
3. **Trilha 3** começa com a sonda que resolve a contradição do inventário sobre
   `Selector::Where`. Conforme o resultado, fecha rápido ou vira S-M.
4. **Escolher uma trilha grande** entre 4 (visuais), 6 (bibliografia Fase 2) e 7
   (multi-região), por prioridade do dono. As três estão "prontas" no substrato;
   a escolha é de valor, não de bloqueio.
5. **Trilha 5 (shaping)** tratada à parte, aberta por sonda de viabilidade de
   rustybuzz. Não intercalar com as outras até saber o tamanho real.
6. **Trilha 8** intercalada como preenchimento entre as trilhas grandes.
7. **Trilha 9** fica para quando (se) a infraestrutura reativa existir.

---

## O que confirmar antes de começar

Três verificações que o roteiro assume e que precisam de uma sonda inicial:

- O gate da ADR-0117 (spec propõe ficheiro que já existe) está implementado no
  `crystalline-lint`, ou ainda é proposta? Se proposta, as Trilhas continuam a
  depender de disciplina manual.
- `Selector::Where` está mesmo `ausente` no consumer, ou o inventário está
  desatualizado face a P417/P423? (abre a Trilha 3)
- A linha do inventário que marca `link` como "sem render visual" (`parcial`,
  linha 213/342) está desatualizada — P422–P424 e P452 materializaram o render e
  a annotation PDF. Atualizar o inventário para `implementado` antes de planear
  qualquer coisa sobre links.
