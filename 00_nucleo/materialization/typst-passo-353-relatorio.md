# Passo 353 — Relatório de Fase A (F-5 de-bake) + Estágio 0 (rig de perf)

> **Veredito da Fase A.** O **Estágio 0 está feito** (rig de perf restaurado + baseline novo
> medido). A medição dos 4 pontos **contradiz o enquadramento do passo** em **dois pontos
> materiais** (ADR-0108 regra 5 — desconfiar do enquadramento cômodo, checar a fonte):
> (1) o de-bake dos 3 numbering **não é "religar consumidores"** — exige **infra NOVA de
> chain-threading no introspect** (o walk do introspect tem **zero** chain hoje); (2) o ponto 4
> (`TextStyle`) **não "completa o render do show-set"** — o render do show-set **já funciona**
> (P352) via o merge chain-wins do layout. Aplica-se a fonte (precedente P351/P352): regista-se,
> e a **re-escopagem é decisão do dono**. Nenhum código de de-bake escrito.

**HEAD**: 3367a548d (pós-P352). **Branch**: Tekt. **Suíte**: 2733/0. Caveat `RUST_MIN_STACK=33554432`.

---

## Estágio 0 — rig de perf restaurado (feito; medição, não produto)

- **Rig** (documentado em `medicao-pre-f-passo-330.md`): `/usr/bin/time -f "%e"`, 12 runs (1ª
  descartada), binário `./target/release/typst`, corpus 10× (70030 linhas = `medicao-pre-f-passo-318-corpus.typ` ×10).
- **Restaurado ao repo** como harness corrível: `tools/perf/bench.sh` + `tools/perf/corpus-10x.typ`
  (gerado do corpus 1× já versionado). Comando: `bash tools/perf/bench.sh`.
- **Novo baseline medido no HEAD atual (pós-P352, "antes" do de-bake)**: **1.1991 s ± 0.0742 s**
  (n=11; σ≈6.2%). **NÃO** é o `0.6518` do P330 — o próprio doc do rig avisa da **deriva de
  ambiente entre sessões**; a prova válida é o par antes/depois **na mesma sessão**. Este 1.1991 s
  é o "antes" para qualquer depois deste lote (ADR-0108: afirmar só o medido).

---

## Os 4 pontos — medição (`file:line`)

### Bakers (onde a decisão é assada, no EVAL, de `engine.styles.custom`)
- **heading**: `rules/eval/markup.rs:90` → `Content::heading_numbered` vs `heading`.
- **equation**: `rules/eval/mod.rs:558` → `equation_numbered` vs `equation` (só bloco).
- **figure**: `rules/eval/closures.rs:79` → `figure_numbering: Option<&str>` na `FigureElem`.
- **`Content::Text`**: `TextStyle::from(&engine.styles)` assado na criação (pervasivo: `mod.rs:344/354/371/401`, markup, etc.).

### Transporte (fatia 1, P339) — confirmado em produção
`rules/eval/mod.rs:426-436`: o `#set …(numbering:)` embrulha o tail léxico num
`Content::Styled(body, styles)` carregando os customs. No layout, `Content::Styled` empurra
`self.chain`/`self.style` **antes** de layoutar o filho (`layout/mod.rs:1248-1256`) — logo o
consumidor de layout do heading/equation/figure/text vê a chain certa.

### Consumidores — a CHAIN está disponível?
| Ponto | Layout | Introspect |
|---|---|---|
| heading numbering | **SIM** (`self.chain`, `layout/mod.rs:714`) | **NÃO** — sem chain; `numbering_active` **nem está no payload** (`to_payload` só depth/body_hash/counter_update) |
| equation numbering | **SIM** (`layout/mod.rs:812`→`equation.rs:28`) | **NÃO** — sem chain; lê o **payload** (`numbering_active` está no payload, `introspect.rs:657`) |
| figure numbering | **SIM** (`layout/mod.rs:860`) | **NÃO** — sem chain; payload só tem `is_counted`, não o pattern |
| `Content::Text` `TextStyle` | **SIM** (`self.style`, merge `layout/mod.rs:609-626`) | N/A (texto não é locatável) |

### Achado-chave 1 — o introspect NÃO tem chain (infra NOVA, não "rewire")
`rules/introspect.rs` walk (assinatura ~`:739`) **não tem parâmetro `StyleChain`**; o arm
`Content::Styled(body, _)` (~`:1204`) **descarta** os styles e desce sem chain. Os gates de
numbering do introspect leem o **campo/payload assado**. **De-bakar os numbering exige construir
chain-threading no introspect** (add `chain` ao walk, push/pop em `Styled`, recomputar os gates da
chain, e adicionar campos ao payload de heading/figure). Isto é **infra nova e arriscada** — o
passo chama-o "religar os consumidores", mas o introspect não tem o que religar. **Medido, não
inferido.** *O que refutaria:* um caminho em que o gate de numbering chegue ao introspect sem
chain — não existe (o gate é computado no eval, e o introspect corre depois, sem `engine.styles`).

### Achado-chave 2 — o render do show-set JÁ funciona (ponto 4 é moot)
O merge do layout (`layout/mod.rs:609-626`) dá **prioridade à chain** em **todos** os campos:
`bold/italic` = `node_style || self.style`; `size` = chain se maior; `fill/weight/tracking/
leading/lang/font` = `self.style.X.or(node_style.X)`. Logo `#show heading: set text(fill: …)`
→ `Content::Styled(fill)` → `self.style.fill` → **renderiza** com o fill. **O render do show-set
do P352 já alcança o frame** — o ponto 4 do passo ("o de-bake completa o render do show-set")
está **refutado pela fonte**: o de-bake do `TextStyle` **não habilita nada observável novo**;
apenas removeria o `node_style` redundante. *(Correção honesta: o relatório do P352 disse "render
pendente do F-5" sem checar este merge — a Fase A do P353 corrige.)*

### Achado-chave 3 — o ponto 4 também não é content-preserving trivial
O **bold do heading** é assado no `node_style` do `Content::Text` (`markup.rs:85-86`), **não**
numa chain (o heading não embrulha em `Content::Styled(bold)`). Se o de-bake remover o `node_style`
e ler só a chain, o texto do heading perde o bold (`self.chain.bold` é false) → **regressão**.
De-bakar o `TextStyle` exige **antes** de-bakar o bold do heading para um wrapper de chain — mais
superfície. Logo nenhum dos 4 pontos é um "ganho seguro" isolado.

---

## Limites duros — respeitados
Nada tocado: loop α / caso 2, caso 4, `morph_canon`/`==`, flag P350c, Marco G. Suíte 2733/0
inalterada. `is_numbering_active` continua morto (não religado).

---

## Decisão proposta ao dono (a fonte aplicada; o dono audita a substância — ADR-0108)

1. **Estágio 0 entregue** (rig + baseline 1.1991 s) — útil para F-5, F-6 e o caso 1, independente do resto.
2. **O de-bake é maior e de forma diferente do enunciado:** os 3 numbering exigem **infra de
   chain-threading no introspect** (não um rewire); o ponto 4 (`TextStyle`) **não completa render**
   (já funciona) e ainda arrasta o de-bake do bold do heading. Recomenda-se **não** materializar o
   de-bake sob a premissa atual do passo.
3. **Caminhos possíveis (decisão do dono):**
   - **(a)** Aceitar a infra de introspect chain-threading como o **núcleo** do F-5 e fatiar:
     P353 = Estágio 0 + introspect-chain + **1** numbering (ex.: equation, que já tem o gate no
     payload — menor delta) como prova; os outros numbering em lotes seguintes.
   - **(b)** Re-escopar: como o render do show-set **já** funciona e os campos assados são
     **content-preserving** (layout já lê a chain por merge), questionar **se o de-bake é
     necessário agora** — o "caminho duplo" existe mas é **paridade-testada** (chain ≡ assado), não
     um bug dormindo. O de-bake vira limpeza, não correção. Pode ceder lugar ao **caso 1** (P354) ou
     ao F-6.
   - **(c)** Manter o plano e construir tudo (introspect-chain + 4 pontos) — maior risco, ergue
     infra nova no introspect.

Estágio 0 commitável já (isolado, como o passo pede). O de-bake aguarda a tua escolha de (a)/(b)/(c).

---

## ADENDO — probe de confinamento léxico (a medição que decide (a) vs (b))

O dono apontou: o "caminho duplo é paridade-testada" foi **testado, não provado sob
confinamento léxico** — a pergunta eager-vs-léxico (a mesma que justificou a F-realização)
aplicada ao numbering. Probe read-only (vanilla 0.14.2 como oráculo; árvore limpa; suíte 2733):

```
= A
#[ #set heading(numbering: "1.")
   == B ]
== C
#outline()
```

**Os três números para o heading B** (medidos: PDF→`pdftotext`):

| | heading A | **heading B** | heading C |
|---|---|---|---|
| **vanilla 0.14.2** (corpo) | (sem nº) | **`0.1.`** | (sem nº) |
| **vanilla 0.14.2** (outline) | (sem nº) | **`0.1.`** | (sem nº) |
| **crystalline layout** (corpo) | (sem nº) | **`1.1.`** | (sem nº) |
| **crystalline introspect** (outline) | (sem nº) | **`1.1`** (+ "Secção") | (sem nº) |

**Leitura contra a régua do dono:**
1. **Vazamento eager (outline numera A/C)?** **NÃO** — A e C ficam sem número no corpo **e** no
   outline (crystalline **e** vanilla). O numbering confina-se a B. *Por quê:* o bake lê de
   `engine.styles` que **já é lexicalmente escopada** (`local_styles` no `ContentBlock`), logo a
   régua "eager" do introspect **também** é confinada — o vazamento hipotético não materializa.
2. **Layout discorda do introspect sobre B?** **NÃO** — ambos numeram **só B**, com o **mesmo**
   número (`1.1`). As duas réguas **concordam** sob confinamento. **Sem divergência de régua.**
3. **Diverge do vanilla?** **SIM, mas ORTOGONAL ao de-bake:** crystalline `1.1` vs vanilla `0.1`
   (o **contador** de heading — o stepping de nível-1 de A), e o outline acrescenta "Secção" que o
   vanilla não emite. **Presente nas DUAS réguas igualmente** (layout `1.1` = introspect `1.1`).

**Conclusão — substância:** o de-bake lê o **gate** (`heading.numbering` bool) da chain; o número
vem do **contador** (`introspector.formatted_counter_at`), que o de-bake **não toca**. Logo
de-bakar **não** mudaria `1.1`→`0.1` — a divergência vanilla é um **bug de contador/supplement
ortogonal**, não uma divergência de régua. As réguas layout↔introspect **não divergem** sob
confinamento → o caminho duplo é genuinamente paridade (não bug dormindo) → **rumo (b)**
(ADR-0107: não erguer a infra de introspect-chain à frente da demanda; Estágio 0 commitado; seguir
para o caso 1 / F-6). **Tensão honesta:** a régua literal do dono ("divergir do vanilla → (a)")
**dispara**, mas o seu *intento* ("as réguas divergem → de-bake é correção") **não se sustenta** —
a divergência não é de régua e o de-bake não a corrigiria. **Achado novo (débito separado, não
F-5):** contador de heading `1.1`≠`0.1` + supplement "Secção" no outline — candidato a
diagnóstico/lote próprio.

---

## FECHO (decisão do dono) — rumo (b)

Escolhido **(b)**: commitar o **Estágio 0** isolado (rig de perf + baseline 1.1991 s + este
relatório); **adiar o de-bake** (sem demanda medida — as réguas não divergem; ADR-0107); **logar o
bug do contador** como **DEBT-60** (`00_nucleo/DEBT.md`, ortogonal ao F-5). Próximo: caso 1 (P354)
ou F-6, à escolha do dono. Nenhum código de produto (de-bake) escrito; suíte **2733/0**; árvore
limpa fora de `tools/perf/` (rig) e docs.
