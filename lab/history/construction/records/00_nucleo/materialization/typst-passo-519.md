---

# P519 — Sonda ampla de regressão pós-P515–P518 (subsetting + fontdb)

> **Passo:** 519
> **Data:** 2026-07-01
> **Foco:** Verificar se a introdução de fontdb, subsetting TrueType e system fonts (P515–P518) introduziu regressões funcionais que não aparecem no benchmark de performance nem na bateria de paridade P490–P514. A bateria P490–P514 testou paridade de linguagem; este passo testa paridade de produção pós-subsetting.
> **Tipo:** Diagnóstico empírico. Zero código de produção a menos que uma regressão XS seja encontrada.
> **Tamanho:** M (~40 min).
> **ADR-0108 EM VIGOR** — medir antes de decidir. **ADR-0114 EM VIGOR** — sonda antes de spec.
> **Gatilho:** suspeita levantada durante handoff — secção 5.1 do documento de contexto marca "Kerning no subset" como scope-out ("GPOS/GSUB removidas"), o que pode ser uma regressão silenciosa não coberta pelos testes P490–P518.

---

## Contexto

A bateria P490–P514 mediu paridade de **linguagem** (sintaxe, semântica, morfologia) contra o vanilla via `typst query`. Isso não detecta regressões de **produção** — coisas que continuam a compilar sem erro mas que produzem um PDF visualmente ou estruturalmente diferente do que produziam antes de P515–P518.

P515–P518 tocaram a cadeia de fontes de ponta a ponta: descoberta de fontes do sistema (`fontdb`), fallback por carácter, subsetting TrueType (559 KB → 30 KB), e marcação de subset (`AAAAAA+`). Qualquer uma destas mudanças pode ter alterado o comportamento de features que dependiam da fonte completa — em particular, tabelas GPOS/GSUB (kerning, ligatures, features OpenType) que o shaper (Trilha 5, P482–P486) usa.

A hipótese concreta: se o subsetting remove GPOS/GSUB da fonte antes do PDF ser gerado, o rustybuzz deixa de ter essas tabelas disponíveis para consultar durante o shaping — mesmo que o código do shaper esteja intacto. O sintoma não aparece como erro; aparece como texto sem kerning, sem quebrar nenhum teste de paridade de linguagem.

Este passo expande a busca para cobrir esse tipo de regressão silenciosa, e não só o caso do kerning.

---

## Grupo 1 — Ordem do pipeline: subsetting antes ou depois do shaping?

Esta é a pergunta que decide se há regressão real ou não.

```bash
grep -n "shape_document\|subset\|oxifont" 03_infra/src/pipeline.rs
```

**Duas ordens possíveis:**

- **Shaping → subsetting:** o rustybuzz vê a fonte completa (com GPOS/GSUB), o kerning acontece correctamente, e só depois a fonte é reduzida ao conjunto de glifos usados para ir para o PDF. **Sem regressão.**
- **Subsetting → shaping:** a fonte já está reduzida antes do shaper correr. Se o subsetter remove GPOS/GSUB, o shaper perde a informação de kerning. **Regressão real.**

Confirmar com `file:line` qual dos dois é o caso actual.

---

## Grupo 2 — O que o subsetter (`oxifont-subset`, P516) realmente remove

```bash
grep -rn "GPOS\|GSUB\|remove_table\|keep_table\|drop_table" 03_infra/src/ --include="*.rs" | grep -i subset
```

Perguntas:
- O subsetter tem uma lista explícita de tabelas a preservar (`glyf`, `loca`, `hmtx`, `cmap`, ...) e tudo o resto é descartado por omissão? Se sim, GPOS/GSUB caem nessa omissão salvo excepção explícita.
- Existe algum teste que compara o número de tabelas da fonte antes e depois do subsetting?

---

## Grupo 3 — Teste empírico directo: kerning visível no PDF final

Criar um documento com um par clássico de kerning (`"AV"`, `"WA"`, ou `"To"`) e comparar a distância entre os dois glifos no PDF gerado, antes e depois de reverter para fonte completa (sem subsetting).

```bash
cat > /tmp/test-kern.typ << 'EOF'
#set text(font: "Noto Sans", size: 48pt)
AVAWATo
EOF

# Compilar com subsetting (comportamento actual por defeito)
target/release/typst compile /tmp/test-kern.typ /tmp/kern-subset.pdf

# Compilar com flag que desactive subsetting, se existir
target/release/typst compile /tmp/test-kern.typ /tmp/kern-full.pdf --no-subset
# (se a flag não existir, confirmar isso também — é um dado relevante)
```

Extrair as posições dos glifos de cada PDF (`pdftotext -bbox` ou inspecção directa do content stream `TJ`) e comparar os espaçamentos entre "A" e "V".

**Critério:** se os dois PDFs têm o mesmo espaçamento entre "A" e "V", o subsetting preserva kerning (falso alarme). Se o PDF com subsetting tem espaçamento uniforme (sem ajuste) e o PDF sem subsetting tem o "V" mais próximo do "A", confirma-se a regressão.

---

## Grupo 4 — Verificar se P490–P518 tinham algum teste que cobriria isto

```bash
grep -rln "kern\|GPOS\|x_offset" lab/parity/corpus/p490/ lab/parity/corpus/p500/ 2>/dev/null
grep -rn "kern\|x_offset" 00_nucleo/diagnosticos/*.md | grep -i "p51[5-8]"
```

Se nenhum ficheiro do corpus testa kerning visualmente, isso confirma que a bateria P490–P518 não tinha cobertura para este tipo de regressão — não porque o sistema estivesse correcto, mas porque o teste nunca existiu.

---

## Grupo 5 — Expandir a busca: outras features que dependem de tabelas de fonte completas

O kerning não é a única feature potencialmente afectada por subsetting agressivo. Verificar cada uma:

| Feature | Tabela OpenType | Sonda |
|---------|-----------------|-------|
| Kerning | GPOS | Grupo 3 acima |
| Ligatures (`fi`, `fl`) | GSUB | `grep -n "liga\|GSUB" oxifont-subset ou equivalente` |
| Small caps reais | GSUB (`smcp`) | já era scope-out declarado antes de P515 — confirmar que continua assim, não regrediu para pior |
| Contextual alternates | GSUB (`calt`) | idem kerning |
| Hinting (renderização em baixa resolução) | `fpgm`, `prep`, `cvt` | provavelmente removidas pelo subsetter; menos crítico em PDF vectorial mas vale confirmar |
| Variable font axes | `fvar`, `gvar` | já é scope-out declarado (Trilha 7 — VF) — não deveria ter regredido porque nunca funcionou |
| Vertical metrics / line height | `hhea`, `OS/2` | verificar se o subsetter preserva estas tabelas (afecta layout, não só glifos) |

Para cada linha "provavelmente removida" ou "verificar", correr o `grep` correspondente e registar resultado.

---

## Grupo 6 — Usar a lente de migração, se ajudar a cruzar referência

O projecto tem uma ferramenta de comparação estrutural vanilla-vs-cristalino (`lab/mapa-migracao/gerar.py`), usada em passos anteriores para detectar módulos `não-iniciado` ou discordâncias entre o que está declarado como feito e o que o código realmente tem. Não é obrigatório usá-la aqui, mas pode ajudar a confirmar rapidamente se o mapa já tinha sinalizado esta área antes de P515:

```bash
python3 lab/mapa-migracao/gerar.py <json-fonte> 2>/dev/null | grep -i "gpos\|gsub\|kern\|shap"
```

Se a lente já tinha uma linha sobre `typst_text::shape` ou tabelas OpenType marcada como `pendente`/`parcial` antes de P515, isso é confirmação adicional independente do `grep` manual. Se a lente não cobre este nível de detalhe (é provável que não — ela opera ao nível de módulo, não de tabela de fonte), essa ausência também é um dado: significa que este tipo de regressão fica fora do alcance normal da lente e só é detectável por teste directo como o Grupo 3.

---

## Classificação do resultado

Para cada item testado (kerning, ligatures, calt, hinting, vertical metrics):

- **REGRESSÃO_CONFIRMADA** — funcionava antes de P516 (subsetting) e não funciona depois; verificado por comparação directa (Grupo 3).
- **NUNCA_FUNCIONOU** — já era scope-out ou não implementado antes de P515; subsetting não mudou nada.
- **PRESERVADO** — subsetter mantém a tabela relevante; sem impacto.
- **NÃO_TESTÁVEL** — sem forma prática de verificar neste passo (ex.: hinting em PDF vectorial tem efeito mínimo observável).

---

## Critério de fecho

- [ ] Grupo 1: ordem do pipeline (shaping antes ou depois de subsetting) confirmada com `file:line`.
- [ ] Grupo 2: lista de tabelas preservadas/removidas pelo subsetter confirmada com `file:line`.
- [ ] Grupo 3: teste empírico de kerning (com e sem subsetting) executado; resultado registado.
- [ ] Grupo 4: confirmado que corpus P490–P518 não tinha cobertura de kerning visual (ou encontrada se existir).
- [ ] Grupo 5: cada linha da tabela (kerning, ligatures, smcp, calt, hinting, VF, vertical metrics) classificada.
- [ ] Grupo 6: lente consultada; resultado registado (mesmo que "não aplicável a este nível").
- [ ] Tabela final com classificação por item.
- [ ] Se alguma REGRESSÃO_CONFIRMADA for encontrada e o fix for XS (ex.: adicionar `GPOS`/`GSUB` à lista de tabelas preservadas pelo subsetter): aplicar e verificar com o mesmo teste do Grupo 3.
- [ ] Se o fix for maior que XS: não aplicar neste passo — documentar como P520 candidato com prioridade.
- [ ] Relatório produzido em `00_nucleo/diagnosticos/regressao-subsetting-p519.md`.

---

## Próximo passo

- **Se REGRESSÃO_CONFIRMADA em kerning/ligatures:** P520 é o fix (provavelmente S — adicionar tabelas à whitelist do subsetter).
- **Se tudo NUNCA_FUNCIONOU ou PRESERVADO:** falso alarme confirmado; retomar escolha entre as opções A–G do handoff (Lookahead, publicação, CFF subsetting, VF, optimização, etc.).
