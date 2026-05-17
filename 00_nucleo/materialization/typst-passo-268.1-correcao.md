# typst-passo-268.1-correção — Correcção factual ADR-0090 pré-commit

**Magnitude**: XS (cap 0 LOC L1/L3/stdlib; só edições documentais a ADR-0090 + relatório).
**Cluster**: Visualize / Gradient / PDF export (documental).
**Tipo**: correcção pré-commit. **Não é sub-passo materializado** — é edição directa do output P268.1 antes de commit final. Sem numeração .N ordinal.
**Origem**: pesquisa Kimi (corroborada via web_search literal blog Typst 2023 + W3C Workshop 2021 + Cairo/Igalia) revelou três afirmações factuais erradas em ADR-0090 EM VIGOR.
**Pré-condição**: ADR-0090 ainda **não commitada** (utilizador confirmou). Permite edição literal sem necessidade de anotação cumulativa nem revogação.
**Sequência**: P268 → P268.1 (ADR-0090 criada com erros) → **correcção pré-commit** → P268.2 (já executado; preservado literal) → próximo passo.

---

## §0 — Princípios vinculativos

1. **Regra de Ouro CLAUDE.md preservada literal** — sem código L1/L3/stdlib alterado. Apenas edição documental.

2. **ADR-0034 + ADR-0085** (diagnóstico imutável). **Sem Fase A nova** — diagnóstico empírico é a pesquisa Kimi + verificação literal via web_search blog Typst 2023, W3C Workshop 2021, ISO 19005-1 §6.2.7. Registado em §"Pesquisa empírica industry" actualizada da própria ADR-0090.

3. **Não cria sub-padrão novo "correcção ADR pré-commit"** — anti-pattern; normalizar isto incentivaria editar ADRs livremente após criação. Este caso é excepção justificada por ADR ainda não commitada + achado factual externo.

4. **Sub-padrão "Descoberta empírica que recalibra spec autor" N=1 → N=2 cumulativo** — P268.2 inaugurou (factor_delta CIELab → Oklab); esta correcção estende (Kimi externa → cristalino). Diferença factual: P268.2 foi recalibração de constante numa spec; esta é correcção de afirmações justificativas numa ADR EM VIGOR pré-commit.

5. **Sub-padrão "Diagnóstico empírico web em vez de filesystem" N=2 → N=3** — terceira aplicação (P268.1 web; P268.2 web + cristalino; **esta correcção web verificação Kimi**).

6. **Crystalline-lint zero violations** obrigatório (sem hash drift; ADR documento puro).

7. **Status ADR-0090 preservado `EM VIGOR`** — correcções factuais não alteram decisão de fundo (cristalino Type 4 Gouraud).

8. **ADR-0089 anotação P268.1 preservada literal** — referencia ADR-0090 mas não cita afirmações erradas; não precisa correcção.

9. **ADR-0054 anotação P268.1 preservada literal** — idem.

10. **L0 `entities/gradient.md` anotação P268.1 preservada literal** — idem.

---

## §1 — Achados factuais consolidados

### §1.A — Verificação literal via web_search Claude web

| Afirmação ADR-0090 actual | Status factual | Fonte verificada |
|---|---|---|
| "Krilla (typst usa): PostScript Function-Based Shading /ShadingType 1 (vendor-specific)" | **ERRADA / não verificável** | Blog Typst 2023 confirma que Typst original usou Coons Type 6. Krilla actual é opaco (sem doc pública sobre estratégia interna). |
| "PDF/A-1 proíbe functions (PostScript não permitido)" | **IMPRECISA** | ISO 19005-1:2005 §6.2.7 proíbe PostScript XObjects (streams PostScript embutidas como XObjects), não Type 4 calculator functions usadas em shading dictionaries. Type 4 functions são subset restrito (sem loops/variáveis/subrotinas). Suporte reader inconsistente é o argumento prático real. |
| "Type 1 Function-Based usado por krilla/Chrome é vendor-specific" | **PARCIALMENTE ERRADA** | Chrome aspect: verificado pdf.js issue #19233. Krilla aspect: não verificável. Typst original aspect: usou Coons Type 6 explicitamente. |

### §1.B — Citação literal blog Typst 2023 (achado central Kimi)

Blog Typst "Color gradients and my gradual descent into madness" (typst.app/blog/2023/color-gradients/) lista as três técnicas testadas pelo autor original (Typst pré-krilla):

1. **Sampled pattern** (imagem pré-renderizada): rejeitada por bloat e pixelização.

2. **PostScript function**: rejeitada porque "several readers do not support this feature".

3. **Coons patch method (Type 6)**: a que acabaram por adotar. Citação literal:
   > "we can still use Coons patches, but we need to create at least as many patches as there are stops in the gradient."

Devido a Apple PDF reader não suportar shading function em Coons patches.

### §1.C — Citação W3C CSS-Color-4 Workshop 2021

W3C Workshop "CSS-Color-4 for Print" (Mike Bremford, bfo):

> "These are implemented in PDF as Coons Patch shading. The concept isn't directly a part of CSS, but they're the only way we can render conic gradients."

Reforça que Coons é industry-standard para conic em PDF.

### §1.D — Transição typst → krilla

Tracking issue typst/typst #2282 (Part 7) regista "Switch PDF backend to krilla #5420". Sem documentação pública sobre estratégia interna krilla para conic. `lab/typst-original/crates/typst-pdf/src/paint.rs:255` mostra `krilla::SweepGradient` sendo inicializada para `Gradient::Conic` — mas SweepGradient é abstracção; o PDF emitido pode usar internamente Type 4/6/7 ou outra estratégia. **Não verificável literal sem ler código krilla**.

### §1.E — Cairo confirmado mesh-based

Igalia/Cairo blog: "A Coons patch comes very handy to paint a conical gradient." Cairo usa Type 6/7 mesh patches, não Type 4 Gouraud puro.

---

## §2 — Edições literais a ADR-0090

Ficheiro: `00_nucleo/adr/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md`.

### §2.A — Título e contexto

**Edição título** — substituir literal:

```
# ADR-0090 — Gradient Conic PDF strategy: Type 4 Gouraud (cristalino) vs Type 1 PostScript (vanilla)
```

por:

```
# ADR-0090 — Gradient Conic PDF strategy: Type 4 Gouraud (cristalino) divergência industry mesh-based variants
```

**Justificativa**: título antigo assume incorrectamente que vanilla usa Type 1. Título novo é factualmente neutro sobre vanilla actual.

**Edição §"Contexto"** — substituir literal o parágrafo que diz "P268.1.PRE revelou que vanilla typst via krilla::SweepGradient usa /ShadingType 1 Function-Based Shading com PostScript function atan2":

```
P268.1.PRE revelou que vanilla typst via krilla::SweepGradient inicializa
uma abstracção de gradient cónico (lab/typst-original/.../typst-pdf/src/paint.rs:255).
A estratégia PDF interna do krilla actual não está publicamente documentada
e não foi verificada literal.

Verificação histórica via blog Typst "Color gradients and my gradual descent
into madness" (2023) revela que Typst original (pré-krilla, antes da
transição Part 7 #5420) usou /ShadingType 6 Coons Patches com tantos patches
quantos stops do gradiente, após rejeitar PostScript functions porque
"several readers do not support this feature".

P268.1 pesquisa industry revelou que **não há estratégia canónica única**
para conic em PDF — cada projecto inventou a sua, mas a família mesh-based
(Type 4 Gouraud / Type 6 Coons / Type 7 Tensor) é amplamente adoptada.
Cristalino escolha Type 4 é alinhamento com industry mesh-based standard;
Typst original (Type 6 Coons), Cairo (Type 6/7), Inkscape (Type 7), e
cristalino (Type 4) são todos variantes mesh-based.
```

### §2.B — Tabela §"Pesquisa empírica industry"

**Edição tabela** — substituir linhas literais:

```
| Krilla (typst usa) | PostScript Function-Based Shading | /ShadingType 1 (vendor-specific) |
```

por:

```
| Krilla (typst actual via Part 7 #5420) | SweepGradient abstracção; estratégia PDF interna não documentada publicamente | desconhecido |
| Typst original (pré-krilla, blog 2023) | Coons Patches (1 patch por stop após rejeitar PS functions) | /ShadingType 6 |
```

E remover linha:

```
| pdf.js (consumidor) | Não suporta /ShadingType 1 (renderiza pink fallback) | — |
```

substituindo por:

```
| pdf.js (consumidor) | Não suporta /ShadingType 1 conic (pink fallback) — relevante para qualquer estratégia Type 1 embutida via Chrome puppeteer-style; não estabelecido se afecta krilla output actual | parcial |
```

### §2.C — §"Conclusões factuais"

**Edição ponto 1** — preservar literal (correcto).

**Edição ponto 2** — substituir literal:

```
2. Type 1 Function-Based Shading usado por krilla/Chrome é tecnicamente
extensão vendor-specific quando aplicado a conic — pdf.js (Firefox PDF
viewer) regista isto explicitamente como "Unsupported ShadingType: 1" e
renderiza pink fallback.
```

por:

```
2. Type 1 Function-Based Shading aplicado a conic é tecnicamente extensão
vendor-specific quando o renderer mapeia (x,y) → angle → cor via
PostScript function — pdf.js (Firefox PDF viewer) regista isto como
"Unsupported ShadingType: 1" e renderiza pink fallback. Chrome via
Puppeteer produz PDFs Type 1 conic. Krilla actual produce strategy
desconhecida; afirmação anterior "krilla usa Type 1" foi inferência
não verificada e é retraída neste passo de correcção.
```

**Edição ponto 3** — substituir literal:

```
3. Industry mesh-based abordagem (Type 4/6/7) é a mais comum em projectos
PDF maduros:
- Cairo (Type 6/7) — 20+ anos maturidade.
- Inkscape (Type 7) — segue Cairo.
- Cristalino (Type 4) — escolha mais simples da família mesh.
```

por:

```
3. Industry mesh-based abordagem (Type 4/6/7) é a mais comum em projectos
PDF maduros:
- Cairo (Type 6/7) — 20+ anos maturidade; Igalia blog confirma.
- Inkscape (Type 7) — segue Cairo.
- **Typst original (Type 6 Coons)** — blog 2023 documenta o processo de
  decisão literal (sampled rejeitado, PostScript function rejeitado, Coons
  adoptado).
- W3C CSS-Color-4 Workshop 2021 (bfo): "the only way we can render conic
  gradients" em PDF é Coons Patch shading.
- Cristalino (Type 4) — escolha mais simples da família mesh; divergência
  intra-família mesh vs Typst original Type 6 / Cairo Type 6-7.
```

**Edição ponto 5** — substituir literal:

```
5. Krilla/vanilla typst é o outlier no industry, não cristalino.
Cristalino está alinhado com Cairo/Inkscape industry standard mesh-based.
```

por:

```
5. Cristalino está alinhado com industry mesh-based standard (Cairo,
Inkscape, Typst original). Krilla actual estratégia interna desconhecida
publicamente; pode ou não estar alinhada. Cristalino divergência vs
Typst original é **intra-família mesh** (Type 4 vs Type 6), não entre
famílias.
```

### §2.D — §"Decisão" justificativa 3

**Edição justificativa 3** — substituir literal:

```
3. **Compatibilidade PDF/A standards restritivos**: Type 1 Function-Based
proibido em PDF/A-1 (PostScript não permitido); Type 4 permitido. Vanilla
typst `convert.rs:514` emite warning "conic gradients are not supported in
this PDF standard" precisamente porque Type 1 falha em PDF/A — Type 4
funcionaria.
```

por:

```
3. **Compatibilidade PDF/A standards restritivos**: ISO 19005-1:2005 §6.2.7
proíbe PostScript XObjects (streams PostScript embutidas como XObjects
independentes). Type 4 PostScript calculator functions usadas em shading
dictionaries são tecnicamente subset restrito (sem loops/variáveis/
subrotinas) e formalmente permitidas, mas suporte reader inconsistente é
o argumento prático. Vanilla typst `convert.rs:514` emite warning "conic
gradients are not supported in this PDF standard" — argumento prático
"reader-support inconsistente" cobre o que cristalino Type 4 Gouraud evita
ao não usar functions em shading.
```

### §2.E — §"Industry precedent" justificativa 4

**Edição justificativa 4** — substituir literal:

```
4. **Industry precedent**: Cairo (Type 6/7), Inkscape (Type 7), cristalino
(Type 4) — todos família mesh-based; krilla Type 1 é outlier.
```

por:

```
4. **Industry precedent**: Cairo (Type 6/7), Inkscape (Type 7), Typst
original pré-krilla (Type 6 Coons; blog 2023 documenta), cristalino
(Type 4) — todos família mesh-based. Krilla actual é opaco. Cristalino
divergência é intra-família mesh.
```

### §2.F — §"Scope-outs preserved"

**Edição scope-out Type 1** — substituir literal:

```
- **Type 1 PostScript Function**: scope-out permanente; vanilla outlier;
cristalino industry-aligned.
```

por:

```
- **Type 1 PostScript Function**: scope-out permanente. Rejeitada
historicamente também pelo Typst original (blog 2023: "several readers do
not support this feature"). Cristalino segue precedente, não diverge.
```

**Edição scope-out Type 6/7** — substituir literal:

```
- **Type 6/7 Coons/Tensor patches**: scope-out actual; cristalino escolha
Type 4 por simplicidade implementação; candidato refino futuro se Type 4
banding for problema real (improvável dado hybrid adaptive N P268.2).
```

por:

```
- **Type 6/7 Coons/Tensor patches**: scope-out actual; cristalino escolha
Type 4 por simplicidade implementação. Typst original usa Type 6 Coons (1
patch por stop); cristalino diverge intra-família. P268.2 adaptive N
hybrid mitiga banding sem mudar estratégia. Candidato refino futuro
(P-Gradient-Coons-Patch) se Type 4 banding for problema real.
```

### §2.G — §"Pesquisa empírica industry" — Nota metodológica

**Adicionar secção nova** após tabela §"Pesquisa empírica industry":

```
## Nota metodológica de proveniência

**Verificáveis literal no filesystem cristalino**:
- `lab/typst-original/crates/typst-pdf/src/paint.rs:255` — SweepGradient
  inicializado para Gradient::Conic (krilla actual).
- `lab/typst-original/crates/typst-pdf/src/convert.rs:514` — warning
  "conic gradients are not supported in this PDF standard".

**Verificáveis via web (não filesystem)**:
- Blog Typst "Color gradients" (2023) — Typst original usou Coons Type 6
  com 1 patch por stop após rejeitar PostScript functions.
- W3C CSS-Color-4 Workshop (2021, Mike Bremford bfo) — Coons como única
  forma render conic em PDF.
- Igalia blog conic gradients (2020) — Cairo Coons patches.
- pdf.js issue #19233 — Unsupported ShadingType: 1.
- ISO 19005-1:2005 §6.2.7 — proíbe PostScript XObjects, não Type 4
  calculator functions em shading dictionaries.

**Não verificáveis literal sem ler código privado**:
- Krilla actual estratégia PDF interna para conic.
- Chrome PDF generator estratégia interna para conic.

**Correcção pré-commit P268.1**: três afirmações originais erradas
(krilla=Type 1; PDF/A-1=functions proibidas; vanilla=outlier total)
identificadas via achado externo (Kimi) e corrigidas literal antes do
commit final, mantendo ADR-0090 estado EM VIGOR e decisão de fundo
(Type 4 Gouraud) preservada.
```

### §2.H — §"Referências"

**Adicionar entradas**:

```
- Blog Typst "Color gradients and my gradual descent into madness"
  (typst.app/blog/2023/color-gradients/) — Typst original Coons Type 6.
- W3C Workshop CSS-Color-4 Mike Bremford (2021) — Coons como única forma
  render conic em PDF.
- ISO 19005-1:2005 §6.2.7 — PDF/A-1 PostScript XObjects restriction.
- typst/typst issue #2282 Part 7 / PR #5420 — transição para krilla.
- Igalia blog "Renderization of Conic gradients" (2020) — Cairo Coons.
```

---

## §3 — Sub-passo .C — Verificação + relatório

### Ordem literal

1. Edições §2.A a §2.H aplicadas literal a ADR-0090 (sem commit antes).
2. `crystalline-lint` (sem `--fix-hashes` necessário; ADR puro documento).
3. Verificar que ADR-0089 / ADR-0054 / L0 `entities/gradient.md` anotações P268.1 ainda fazem sentido factual pós-correcção (nenhuma cita as afirmações erradas; verificar literal).
4. Relatório curto.

### Critério verificação anotações dependentes

- **ADR-0089 §"Anotação cumulativa P268.1"**: cita "divergência arquitectural Type 4 cristalino vs Type 1 vanilla". Após correcção, **deve mudar** para "divergência arquitectural Type 4 cristalino vs estratégia vanilla actual desconhecida; Typst original era Type 6 Coons".
- **ADR-0054 §"Anotação cumulativa P268.1"**: cita "Conic PDF Type 4 vs Type 1 vanilla". Mesma correcção.
- **L0 `entities/gradient.md` anotação P268.1**: cita "Type 4 cristalino vs Type 1 vanilla". Mesma correcção.

**Decisão**: estas três anotações também precisam edição literal pré-commit (mesma janela; ainda não commitadas; verificar status git).

Se anotações já foram commitadas, escopo expande para anotação cumulativa "P268.1-correção" nessas três localizações. **§política condição 1** dispara para confirmar.

### Cap LOC

- L1/L3/stdlib: 0 (cap exacto).
- Documental: ~50-80 linhas adicionadas/modificadas em ADR-0090 + ~10 linhas em cada uma de 3 anotações se necessário.

---

## §4 — Sub-passo .D — README + relatório

1. **ADR-0090** editada literal §2.
2. **ADR-0089 anotação P268.1** corrigida se ainda não commitada.
3. **ADR-0054 anotação P268.1** corrigida se ainda não commitada.
4. **L0 anotação P268.1** corrigida se ainda não commitada.
5. **README ADRs**: entrada P268.1 nos "passos-chave" pode precisar pequena edição literal (~2-3 linhas) para refletir título ADR-0090 actualizado. Sem mudança distribuição (77 preservado).
6. **Relatório** `00_nucleo/materialization/typst-passo-268-1-correção-relatorio.md`:
   - §1 Sumário executivo (correcção pré-commit; 3 afirmações factuais erradas; ADR-0090 EM VIGOR preservada; decisão Type 4 inalterada).
   - §2 Diff afirmações antes/depois ADR-0090.
   - §3 Verificação anotações dependentes (status commit).
   - §4 Sub-padrões + N cumulativo.
   - §5 Métricas (zero LOC; 1 ADR editada; 0-3 anotações editadas; 2428 tests preservados).
   - §6 Pesquisa Kimi consolidada.
   - §7 Referências.

---

## §política de paragem

1. **Anotações P268.1 já commitadas** — se ADR-0089 / ADR-0054 / L0 `entities/gradient.md` anotações já foram commitadas (verificar `git log`), correcção directa não é possível; escopo expande para anotação cumulativa "P268.1-correção" nessas três localizações. Parar e confirmar com utilizador.

2. **ADR-0090 já commitada** — se contra a expectativa do utilizador a ADR-0090 já foi commitada, parar imediatamente. Escopo muda para criar anotação cumulativa em vez de edição directa.

3. **Edições §2 ambíguas** — qualquer trecho onde a correcção exige interpretação além de substituição literal, parar e confirmar.

4. **Crystalline-lint reporta violations** — ADR é documento puro; não deveria haver hash drift. Se houver, indica que ADR-0090 está referenciada por L1/L3/stdlib (não esperado para uma ADR-0090 documental nova).

5. **Tests workspace regressão** — 2428 verdes preservados obrigatório. Sem código alterado; regressão indica build cache stale ou outro problema.

6. **README distribuição inconsistente** — total ADRs 77 preservado obrigatório.

7. **Cap LOC ameaçado** — passo é XS por definição; qualquer alteração de código indica scope creep.

---

## §notas estratégicas

### Subpadrões aplicados neste passo

| Subpadrão | N pós-correcção | Nota |
|---|---|---|
| Diagnóstico empírico web em vez de filesystem | **N=2 → N=3** | + correcção P268.1 (verificação Kimi via web_search blog Typst + W3C + ISO 19005-1) |
| Descoberta empírica que recalibra spec autor | **N=1 → N=2** | + correcção P268.1 (factor_delta P268.2 inaugurou; correcção factual P268.1 estende a ADRs) |
| Auto-aplicação ADR-0065 inline | N=9 preservado | correcção documental |

### Sub-padrões NÃO inaugurados deliberadamente

- "Correcção ADR pré-commit" — **não formalizado**. Anti-pattern; normalizar incentivaria editar ADRs livremente. Este caso é excepção justificada (ADR ainda não commitada + achado factual externo verificado), não pattern reutilizável.

### Marco

**Primeira correcção factual de ADR EM VIGOR pré-commit baseada em verificação empírica externa**. Estabelece prática (não pattern formal) de que ADRs ainda não commitadas podem ser editadas literal quando achados factuais novos invalidam justificativas, preservando decisão de fundo se intacta.

### Sequência pós-correcção

Decisão humana fica em aberto entre as pendências preservadas relatório P268.2 §9:

- **P-Gradient-Focal** (M; revoga ADR-0088 §focal).
- **ADR-0055bis variant-aware fonts** (M; refino Text).
- **P-Footnote-N** (M; Model pendência).
- **DEBT-33** + outros Visualize.
- **Tiling** (Paint::Tiling activação).

---

## §referências cross-passos

- **P268** — PDF Conic Type 4 Gouraud materializado.
- **P268.1** — ADR-0090 criada (com erros factuais a corrigir por este passo).
- **P268.2** — Refino adaptive N hybrid (já executado; preservado literal; não afectado por correcção factual).
- ADR-0090 — editada literal por este passo.
- ADR-0089 anotação P268.1 — possível edição literal (verificar status commit).
- ADR-0054 anotação P268.1 — possível edição literal (verificar status commit).
- L0 `entities/gradient.md` anotação P268.1 — possível edição literal.

---

## §0.1 — Notas de execução para Claude Code

- **Verificar status commit antes de editar**: `git log --oneline 00_nucleo/adr/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md` deve mostrar zero commits (ADR ainda não commitada per utilizador). Se mostrar commits, §política condição 2 dispara.
- **Verificar status commit anotações dependentes**: `git log --oneline 00_nucleo/adr/typst-adr-0089-gradient-conic-only.md` (verifica se anotação P268.1 está commitada; se sim, condição 1 dispara para essa anotação).
- **Edições literais §2.A a §2.H**: aplicar como substituição directa de texto. Não criar ADR-0090-v2 nem revogar.
- **Sem hash propagation esperado** — ADR-0090 é documento puro, não referenciado por L1/L3/stdlib code hashes.
- **Tests workspace 2428 preservados** — sem código alterado; cap 0 LOC.
- **Distribuição ADRs 77 preservado** — sem ADR nova; sem revogação.
- **Status ADR-0090 preservado `EM VIGOR`** — só afirmações justificativas corrigidas; decisão de fundo (Type 4 Gouraud) inalterada.
- **Relatório final esperado**: 2428 testes verdes preservados; hash drift 0; lint zero; ADRs 77 preservado; 1 ADR editada (ADR-0090); 0-3 anotações editadas (verificar status commit); zero LOC L1/L3/stdlib.
