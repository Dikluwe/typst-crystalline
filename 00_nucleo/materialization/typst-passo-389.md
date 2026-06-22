# Passo 389 — Sonda em lote: confirmar e ordenar os ausentes da Lista A

**Tipo:** Sonda de viabilidade (read-only; ADR-0108). **Não materializa código.**
**Data:** 2026-06-22.
**Padrão:** medir-antes-de-decidir (ADR-0108); a lição directa do Passo 388 (sonda refutou o rótulo "parcial XL" do `bibliography`).
**Estratégia (decisão do dono):** parciais ficam para sob-demanda; materializar os **ausentes**. Esta sonda precede a materialização e fixa a ordem.

> **Numeração.** Um passo só (decisão do dono: não numerar à frente — previsão de múltiplos passos sempre derivou). Confirmar 389 como próximo livre ao gravar (precedente P160A/P376).

---

## 1. Contexto — por que uma sonda em lote antes de qualquer materialização

O Passo 388 mostrou que um rótulo do Inventário 148 pode estar errado em dois sentidos: "parcial" significava "modelo-próprio quase completo", e a ausência do parser externo era **adiamento deliberado** (ADR-0062/DEBT-55, com gatilho de consumer nomeado), não dívida. Materializar contra o rótulo teria revertido uma decisão documentada.

Os 16 ausentes da Lista A têm o mesmo risco, 16 vezes. Em vez de uma sonda por feature, esta sonda mede o lote de uma vez e devolve a **ordem aberta** ancorada em código — não no rótulo. É a aplicação directa de ADR-0108: confirmar comportamento, distinguir intenção de dívida, desconfiar do enquadramento.

---

## 2. Objectivo

Para cada ausente da Lista A (e os tipos `Value::*` ausentes da Tabela C que os habilitam), produzir uma linha factual com `file:line`:

1. **Estado real:** genuinamente-ausente / parcial-mal-rotulado / já-feito-mal-rotulado.
2. **Substrato:** o que falta para materializar (helper stdlib / variant `Content` / tipo `Value` / shaping-rustybuzz / regex / runtime). Marcar dependência de `Value::*` (ADR-0017 — variant proíbida sem tipo).
3. **Natureza da ausência:** acidental (dívida) **ou** deliberada (scope-out/adiamento com ADR/DEBT e gatilho de consumer, como hayagriva).
4. **Custo medido:** XS/S/M/XL, ancorado no que falta de facto, não no rótulo.

Saída única: `typst-sonda-ausentes-ordem-passo-389.md`, terminando numa **ordem aberta** (revisável) de materialização. Zero código.

---

## 3. Método (read-only)

Por feature, no HEAD actual:

1. **Grep de existência** — procurar o símbolo, o variant `Content`, a função stdlib, o tipo `Value`. Haiku/Explore lista os hits `file:line`. (Contagem/listagem é busca, não julgamento.)
2. **Classificar estado** (§2.1) a partir dos hits: ausente real vs já presente sob outro nome/forma.
3. **Conferir adiamento deliberado** (§2.3) — varrer ADRs e DEBT.md pelo nome da feature: existe decisão que a difere com gatilho de consumer? Se sim, **não é dívida**: é scope-out à espera de trigger (marcar, não materializar). Este é o passo que faltou no 388. (Julgamento — humano/Sonnet confirma.)
4. **Mapear substrato e dependência de tipo** (§2.2) — se a feature exige `Value::Gradient`/`Tiling`/color-space não-RGB/`Region`/`Script`, registar a dependência; o tipo vem antes (ADR-0017).
5. **Custo** (§2.4) — do substrato medido.

Regra de saída: o que tiver adiamento deliberado com gatilho sai da fila de dívida e vira nota; o que for genuinamente-ausente e acidental entra na ordem, ranqueado por (custo crescente × risco-de-substrato crescente × dependências resolvidas primeiro).

---

## 4. Escopo da sonda — os ausentes a medir

Da Lista A (16) + Tabela C (tipos habilitadores):

**Model:** `asset`, `title`, `document(...)`.
**Text:** `lorem`, `smallcaps`, `text.dir`, `text.region`, `text.script`.
**Visualize:** `gradient(...)`, `tiling(...)`, `cmyk`/`oklab`, `square(...)`.
**Layout:** `pad`/`corners`/`sides` (inset modeling).
**Foundations:** `eval(string)`, `panic(msg)`.
**`#show`:** `#show <selector>` (regex/where).
**Markup:** soft hyphen (`\u{00AD}`).
**Tabela C (tipos habilitadores):** `Value::Decimal`, `Value::Duration`, `Value::Version` (e `Value::Bytes` — já em DEBT-62, P387). Marcar quais features acima dependem deles.

> Conferir cada um contra ADRs de adiamento conhecidas: shaping/rustybuzz (DEBT-53) cobre `smallcaps`/`text.dir`/`region`/`script`/soft-hyphen — provável scope-out graded, não dívida. `#show <selector>` depende de `regex` em L1 (ADR-0054bis condicional). Estes são os candidatos a "falso-ausente por adiamento", o paralelo do hayagriva.

---

## 5. O que produzir

1. `typst-sonda-ausentes-ordem-passo-389.md`: tabela por feature (estado · substrato · natureza · custo · `file:line`) + a **ordem aberta** ranqueada + a lista separada dos "adiados-deliberados" (fora da fila de dívida).
2. Nada mais. Sem L0, sem ADR, sem código.

---

## 6. O que NÃO fazer

- **Não** materializar nenhuma feature. Esta sonda só ordena.
- **Não** numerar os passos de materialização seguintes — um de cada vez, decidido após cada sonda/relatório (decisão do dono).
- **Não** tratar "ausente" como dívida sem conferir adiamento deliberado (a falha que o 388 evitou por pouco).
- **Não** pôr LLM no loop de contagem; grep lista, humano confirma a natureza da ausência.

---

## 7. Critérios de aceitação

| # | Critério |
|---|----------|
| 1 | Todo ausente da §4 recebe linha com `file:line` e os quatro campos (estado/substrato/natureza/custo). |
| 2 | Adiamentos deliberados (com ADR/DEBT + gatilho) separados da fila de dívida, explicitamente. |
| 3 | Dependências `Value::*` mapeadas; tipo habilitador ordenado antes da feature que o exige. |
| 4 | Saída termina numa ordem aberta ranqueada, marcada revisável. |
| 5 | Zero código/L0/ADR; a sonda é o único artefacto. |

---

## 8. O que pode sair errado

- **Feature já materializada e mal-rotulada (o caso 388).** É o resultado esperado para algumas; registar como "já-feito" e remover da fila — é ganho, não erro.
- **Custo subestimado por dependência de tipo.** `gradient` parece M mas arrasta `Value::Gradient` + render PDF; o custo real é do conjunto. Mitigação: custo é do substrato completo, não da função isolada.
- **Adiamento sem ADR escrita.** Uma feature pode estar diferida na prática (shaping) sem DEBT explícito. Mitigação: marcar como "adiamento provável — confirmar com dono" em vez de empurrar para dívida.
- **A sonda virar materialização.** Se um ausente for XS óbvio (`square` derivável de Rect w=h), a tentação é já fazer. Não: a sonda ordena; o passo de materialização vem a seguir, mesmo que curto.

---

## 9. Referências

- `typst-falta-migrar-lista-A-passo-386.md` (os 16 ausentes) e `-lista-B-` (achados bottom-up).
- `typst-sonda-bibliography-passo-388.md` — a lição: rótulo ≠ código; conferir adiamento.
- ADR-0108 (medir antes de decidir), ADR-0017 (sem variant sem tipo), ADR-0054 (graded), DEBT-53 (shaping), DEBT-62 (`Value::Bytes`).

---

## 10. Nota sobre o Tekt

Duas sondas seguidas (388, 389) refutando ou ordenando rótulos do inventário sugerem um padrão: quando o oráculo de cobertura (inventário) e o código divergem, **o código ganha, e a sonda em lote é mais barata que confiar no rótulo feature-a-feature**. Candidato a lição: a sonda de ausência deve preceder qualquer série de materialização guiada por inventário. Registar; não materializar no Tekt aqui.
