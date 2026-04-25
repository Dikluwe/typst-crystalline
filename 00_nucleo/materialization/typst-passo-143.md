# Passo 143 — Correcção do `00_nucleo/adr/README.md` (inventário empírico)

**Série**: 143 (passo **S-M** L0-puro; correcção de índice
canónico desactualizado).
**Precondição**: Passo 142 encerrado; DEBT-1 e DEBT-52 fechados;
10 DEBTs abertos; 55 ADRs existentes; README do directório
`00_nucleo/adr/` lista apenas até ADR-0037.

**Natureza**: passo **L0-puro**. **Zero código**. **Zero testes**.
**Zero ADRs novas**. Único output: `00_nucleo/adr/README.md`
actualizado, baseado em inventário empírico do directório
`00_nucleo/adr/`.

**Conflito de numeração**: o relatório 142 §8 reservou "Passo
143" como candidato para gap 7 (lang hyphenation). Este passo
de correcção documental ocupa 143; o candidato de hyphenation
passa a **144** quando priorizado. Registar no relatório deste
passo.

**Tamanho estimado**: **S-M**, ~1h-1h30. Trabalho é
mecânico mas volumoso: 18 ADRs em falta (0038–0055) + revisão
dos 37 já listados + eventual revisão de meta-regras e cadeias
de revogação/revisão.

**ADRs aplicáveis**:
- Nenhuma directamente. O passo **executa** a convenção
  canónica documentada no próprio README actual (vocabulário
  de status, cadeias de revogação/revisão, campos de
  cabeçalho). Não cria, revê, nem revoga ADRs.

---

## Contexto

O `00_nucleo/adr/README.md` é o índice canónico dos ADRs do
projecto. Declara as meta-regras em vigor, o vocabulário
canónico de status (6 valores), cadeias de revogação e
revisão, convenções estruturais (nome de ficheiros, cabeçalho,
corpo) e, explicitamente, **"Estado por ADR"** em tabela.

O último estado documentado na tabela é **ADR-0037** ("Coesão
por domínio"). O total declarado no README é **38 ADRs (37
números únicos; ADR-0026 tem variante -R1 por revisão)**.

Desde então, o projecto continuou a produzir ADRs até pelo
menos **ADR-0055** (`IMPLEMENTADO` após Passo 141). O
relatório 141 confirma explicitamente o gap:

> `00_nucleo/adr/README.md` não tocado: a tabela de status nesse
> ficheiro só lista até ADR-0037; ADR-0055 nunca foi lá
> adicionada (consistente com o estado dos restantes ADRs
> 0038-0054).

ADR-0019 foi **anotada** no Passo 140A com nota factual (sem
revogação). A anotação não muda o status (`IMPLEMENTADO`), mas
a convenção do README ("Observações por ADR", se existir secção
dedicada) pode justificar referência.

Este passo fecha o buraco, produzindo README coerente com o
directório `00_nucleo/adr/` real. Trabalho é **empírico**:
cada ADR é lido, status transcrito, campos de relação
(Revoga/Revogado por/Revê/Revisto por) registados.

---

## Objectivo

Ao fim do passo:

1. **`00_nucleo/adr/README.md` actualizado** com:
   - **Tabela "Estado por ADR"** contendo **todos** os ADRs
     presentes em `00_nucleo/adr/`, não apenas 0001–0037.
   - **Distribuição de status** (contagens por valor canónico)
     recalculada.
   - **Cadeia de revogações** com entradas novas se ADRs
     posteriores a 0028 introduziram revogações.
   - **Cadeia de revisões** com entradas novas se ADRs
     posteriores a 0026 introduziram variantes `-RN`.
   - **Meta-regras em vigor** revista: ADRs posteriores a
     0037 podem ter introduzido meta-regras novas (ADR-0038 é
     "Sistema de estilos em L1" segundo relatório 100; pode ou
     não ser meta-regra em vigor). Verificação empírica em
     143.3.
   - **Total** actualizado.

2. **Passos-chave da história dos ADRs** estendida se aplicável
   (o README actual termina em P84.8h; passos posteriores que
   introduziram ADRs novas podem merecer entrada).

3. **Aviso sobre vocabulário em documentos históricos** revisto
   se necessário (actualmente refere P84.8g como corte — o
   corte continua válido).

4. **Relatório do passo** em
   `00_nucleo/materialization/typst-passo-143-relatorio.md`
   documentando:
   - Método (grep/view de cada ADR).
   - Resumo de cada ADR 0038–0055 (título + status + campos de
     relação) e revisão dos 0001–0037.
   - Divergências encontradas entre README antigo e realidade
     (se houver).
   - Alterações aplicadas.

Este passo **não**:

- Toca código em L1, L2, L3, L4.
- Toca testes.
- Toca ADRs em si (não corrige erros de status em ADR
  individual; apenas transcreve).
- Corrige ficheiros em `00_nucleo/relatorios/`,
  `00_nucleo/materialization/`, `00_nucleo/context/`
  (imutáveis, ver README secção "Aviso").
- Renomeia ficheiros ADR.
- Altera cabeçalho canónico ou convenções estruturais.

---

## Decisões já tomadas

1. **Escopo completo**: inventário empírico de todos os ADRs,
   não apenas os posteriores a 0037. Permite apanhar
   divergências que possam existir entre 0001–0037 (ex: ADR
   que foi revogado entretanto).
2. **Sem criar ADRs novas**. Se inventário revelar que algum
   ADR existente está em estado inconsistente (ex: cabeçalho
   sem `**Status**:`), registar no relatório mas **não
   corrigir** neste passo — correcção de ADR individual é
   passo separado.
3. **Sem mover ficheiros**. Se nome de ficheiro não seguir a
   convenção (`typst-adr-NNNN-<slug>.md`), registar mas não
   renomear.
4. **Meta-regras em vigor**: um ADR é "meta-regra em vigor" se
   o seu `**Status**:` é `EM VIGOR` **e** a sua natureza é de
   regra/política (em vez de decisão técnica concreta). Dúvida
   resolve-se pela secção "Decisão" do ADR: se escreve "todo o
   código presente e futuro", é meta-regra; se escreve "fazer X
   na função Y", é decisão técnica. Em caso ambíguo, seguir
   precedente: ADRs 0018, 0029, 0030, 0032–0037 são listadas
   como meta-regras no README actual; ADRs 0016, 0017, 0021–0027
   não.

## Decisões diferidas (resolvidas neste passo)

5. **Total exacto de ADRs**: a contar em 143.1. Esperado
   ~55–56 (54 activas + ADR-0026-R1 + possíveis outras
   variantes `-RN` introduzidas entre 0026-R1 e 0055).
6. **Status de cada ADR 0038–0055**: empírico em 143.2. Status
   de contexto conhecido (pelos relatórios):
   - ADR-0038 (Sistema de estilos em L1) — `EM VIGOR`.
   - ADR-0039 — relacionada com DEBT-48 (Passo 100).
   - ADR-0040 (`#set text(...)` + fill) — Passo 102.
   - ADR-0041 (show heading/strong/emph) — Passo 103.
   - ADR-0052 (Lang tipo semântico) — Passo 131B.
   - ADR-0053 (FontList) — Passo 132B.
   - ADR-0054 (critério fecho DEBT-1) — Passo 135, `EM VIGOR`.
   - ADR-0055 (font consumer CIDFont) — `IMPLEMENTADO` após
     Passo 141.
   - 0042–0051 — contexto incerto; empírico resolve.
7. **Quantidade de ADRs `-RN` adicionais além de 0026-R1**:
   desconhecido até 143.1. Se existirem, adicionar à cadeia
   de revisões.
8. **Quantidade de revogações novas além de 0007 e 0028**:
   desconhecido até 143.1.

---

## Escopo

**Dentro**:

- Leitura de todos os ficheiros em `00_nucleo/adr/` (excluindo
  o próprio README).
- Extracção de: número, título curto, status,
  `Revoga`/`Revogado por`/`Revê`/`Revisto por`.
- Reescrita de `00_nucleo/adr/README.md` — **apenas secções
  afectadas** (ver 143.4). Secções invariantes preservadas
  literalmente.
- Escrita do relatório do passo.
- Actualização de contagens e totais.

**Fora**:

- Modificação de qualquer ficheiro ADR.
- Criação/revogação de ADRs.
- Modificação de prompts L0 em `00_nucleo/prompts/`.
- Modificação de `DEBT.md`.
- Correcção de convenções estruturais (cabeçalho, nome de
  ficheiro) — se aparecerem divergências, regista-se.
- Renumeração.

---

## Sub-passos

### 143.1 — Inventário de ficheiros ADR

**A.1.1 — Listar**:

```
ls 00_nucleo/adr/typst-adr-*.md | sort
```

Registar lista completa. Esperado: 55+ entradas.

**A.1.2 — Detectar anomalias de nome**:

Padrão canónico: `typst-adr-NNNN-<slug>.md` ou
`typst-adr-NNNN-RN-<slug>.md`.

```
ls 00_nucleo/adr/ | grep -vE "^typst-adr-[0-9]{4}(-R[0-9]+)?-[a-z0-9-]+\.md$" | grep -v "^README\.md$"
```

Qualquer saída não-vazia é **anomalia**. Registar no relatório
mas não corrigir.

**A.1.3 — Agrupar**:

- ADRs "base" (sem `-RN`): lista principal.
- ADRs `-RN`: lista separada para cadeia de revisões.
- Detectar se algum número base tem `-RN` e `-R2` (improvável
  mas possível).

### 143.2 — Extracção de metadados por ADR

Para cada ficheiro ADR, executar:

```bash
grep -E "^\*\*Status\*\*:|^\*\*Revoga\*\*:|^\*\*Revogado por\*\*:|^\*\*Revê\*\*:|^\*\*Revisto por\*\*:|^\*\*Data\*\*:|^# " <file>
```

Alternativa (mais segura — ver cabeçalho inteiro):

```bash
head -12 <file>
```

Registar em tabela de trabalho (não é output final — é
scratchpad para construir o README):

| Número | Título curto (do `# ⚖️ ADR-NNNN: ...`) | Status | Revoga | Revogado por | Revê | Revisto por |
|--------|----------------------------------------|--------|--------|--------------|------|-------------|
| 0001 | ... | ... | — | — | — | — |
| ... | ... | ... | ... | ... | ... | ... |

**A.2.1 — Normalização de status**:

Se algum ADR usar valor fora dos 6 canónicos (`PROPOSTO`,
`IDEIA`, `EM VIGOR`, `IMPLEMENTADO`, `REVOGADO`, `ADIADO`),
**não corrigir**. Registar como anomalia no relatório. O
README canónico usa apenas valores canónicos; uma linha de
tabela com status fora-do-canónico fica como **nota**
(ex: `status irregular — ver §X do relatório`).

**A.2.2 — Títulos curtos**:

Extrair de `# ⚖️ ADR-NNNN: <título>`. Se não houver emoji
`⚖️`, tolerar. Se o título do README antigo for mais curto
que o actual, preservar o curto (consistência de largura de
tabela). Ver exemplos no README antigo: "Estratégia de
migração (Opção C)", "rustc_hash substituído".

**A.2.3 — Detecção de cadeias**:

Revogação: procurar pares `ADR-X --Revoga→ ADR-Y` **e** `ADR-Y
--Revogado por→ ADR-X`. Se só um lado existe, é assimetria —
registar no relatório.

Revisão: procurar pares `ADR-NNNN-RN --Revê→ ADR-NNNN` **e**
`ADR-NNNN --Revisto por→ ADR-NNNN-RN`. Mesma regra de
simetria.

### 143.3 — Identificação de meta-regras novas

Para cada ADR 0038–0055 com status `EM VIGOR`, decidir se é
meta-regra:

**Critério**: ler secção "Decisão" do ADR. Se escreve regra
geral aplicável a código presente e futuro → meta-regra. Se
escreve decisão técnica sobre parte específica → não.

Resultado esperado (baseado em contexto):

- **ADR-0038** (Sistema de estilos em L1): `EM VIGOR` segundo
  DEBT.md; provavelmente meta-regra. Confirmar em 143.3.
- **ADR-0054** (critério fecho DEBT-1): `EM VIGOR`; estabelece
  regra sobre fecho de dívida — meta-regra.
- Outros candidatos 0040–0053: contexto incerto. Confirmar.

**A.3.1 — Tabela de meta-regras**:

Montar lista de meta-regras finais (incluindo as 7 actuais
preservadas). Numeração na secção "Meta-regras em vigor" do
README é sequencial — manter ordem cronológica por número de
ADR. Item novo entra no fim.

### 143.4 — Reescrita do README

**Secções a reescrever** (todas as outras são preservadas
literalmente):

1. **"Meta-regras em vigor"**: adicionar itens se 143.3
   identificou meta-regras novas. Ordem cronológica
   preservada.

2. **"Estado por ADR"** (tabela): substituir pela tabela
   completa construída em 143.2. Ordem: ascendente por número,
   com `-RN` imediatamente após `-R0`.

3. **"Distribuição de status"**: recalcular contagens por
   valor canónico. Preservar formato `PROPOSTO: N ADRs
   (comentário)`.

4. **"Cadeia de revogações"**: adicionar pares novos
   detectados. Preservar formato ASCII-art existente.

5. **"Cadeia de revisões"**: idem.

6. **Total** (final da secção "Estado por ADR"):
   `**Total**: N ADRs (M números únicos; K têm variante -RN)`.

7. **"Passos-chave da história dos ADRs"** (final do README):
   adicionar 1–N entradas para passos posteriores a P84.8h
   que introduziram ADRs. Mínimo: entradas para introdução
   de ADR-0038 (Passo 99), ADR-0054 (Passo 135), ADR-0055
   (Passo 140A). Evitar entrada por ADR se a história é
   longa — agrupar por fases coerentes.

**Secções preservadas literalmente**:

- Cabeçalho ("Este documento é o índice canónico...").
- "Vocabulário canónico de status" (tabela de 6 valores).
- "Convenções estruturais" (ficheiros, cabeçalho, corpo).
- "Directórios relacionados" (tabela).
- "Distinção operacional".
- "Aviso sobre vocabulário em documentos históricos"
  (excepto se 143.2.A.1 detectou anomalias — nesse caso
  adicionar nota ao parágrafo final explicando que anomalias
  detectadas no passo 143 estão documentadas no relatório).

### 143.5 — Verificação de coerência

**A.5.1** — Contagem manual da tabela "Estado por ADR" bate
com o total declarado.

**A.5.2** — Soma das contagens em "Distribuição de status"
bate com o total.

**A.5.3** — Cada entrada de "Cadeia de revogações" tem
contraparte na tabela (status `REVOGADO` no revogado; ADR
que revoga existe na tabela).

**A.5.4** — Cada entrada de "Cadeia de revisões" tem
contraparte na tabela.

**A.5.5** — `grep -n "ADR-00[0-9]\{2\}" 00_nucleo/adr/README.md`
não produz órfãos (referências a números que não existem na
tabela).

### 143.6 — Relatório

Ficheiro:
`00_nucleo/materialization/typst-passo-143-relatorio.md`.

Secções:

1. Sumário executivo.
2. Método (inventário empírico).
3. Lista completa dos ADRs (tabela idêntica à do README
   actualizado, para preservação histórica).
4. Divergências detectadas (anomalias de nome, assimetrias
   de cadeia, status fora-do-canónico, cabeçalho
   irregular). Se lista é vazia, declarar explicitamente.
5. Meta-regras novas identificadas.
6. Passos-chave novos adicionados ao README.
7. Actualização de contagens (antes/depois).
8. Verificação de coerência (checklist A.5).
9. Conflito de numeração registado: este passo 143 ocupa o
   número antes reservado para lang hyphenation, que passa
   a 144.

---

## Verificação

1. ✅ `00_nucleo/adr/README.md` contém todos os ADRs do
   directório.
2. ✅ Tabela "Estado por ADR" sem buracos entre 0001 e último
   (excepto variantes `-RN` intercaladas).
3. ✅ "Distribuição de status" bate com contagem da tabela.
4. ✅ Cadeias de revogação e revisão simétricas.
5. ✅ Meta-regras ampliadas se aplicável.
6. ✅ Nenhum ficheiro ADR tocado.
7. ✅ Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/`,
   `04_wiring/` tocado.
8. ✅ Nenhum ficheiro em `prompts/`, `relatorios/`,
   `materialization/`, `context/` tocado (excepto
   relatório 143 criado em `materialization/`).
9. ✅ `cargo test --workspace --lib`: inalterado face a 142.
10. ✅ `crystalline-lint .`: zero violations.
11. ✅ Relatório 143 escrito.
12. ✅ Anomalias registadas (mesmo que a lista seja vazia —
    declarar explicitamente).

---

## Critério de conclusão

1. README do directório `00_nucleo/adr/` é índice canónico
   fiel ao conteúdo do directório.
2. Qualquer ADR novo (0038–0055 e, no futuro, posteriores)
   tem entrada na tabela.
3. Contagens coerentes.
4. Cadeias simétricas.
5. Anomalias (se houver) documentadas para passo futuro de
   correcção.
6. Relatório do passo 143 escrito.
7. Zero código tocado.
8. Numeração: 143 ocupado por este passo; 144 reservado
   para candidato de lang hyphenation.

---

## O que pode sair errado

- **Ficheiro ADR sem `**Status**:` canónico**: cabeçalho pode
  usar `**Estado**:` (forma antiga), `**Status:**` (dois
  pontos interiores), ou omitir. Registar como anomalia.
  Para efeito de tabela, usar inferência conservadora (última
  linha que pareça status). **Não corrigir o ADR.**

- **ADR com status `ACCEPTED` ou `UPDATED` (inglês)**: o
  README canónico declara que o vocabulário actual é só em
  português. Se aparecer inglês em ADR actual (não histórico),
  é violação da convenção. Registar como anomalia; mapear
  mentalmente (`ACCEPTED` ≈ `EM VIGOR` ou `IMPLEMENTADO`;
  `UPDATED` ≈ revisão mas não é status). **Não colocar
  inglês na tabela** — colocar `(status irregular)` e
  referenciar o relatório.

- **ADR com cadeia assimétrica**: ex: ADR-X diz "Revogado
  por ADR-Y" mas ADR-Y não diz "Revoga ADR-X". Registar
  assimetria. Colocar na cadeia **apenas** se a relação é
  plausível (mencionada em relatório de passo) — usar
  versão conservadora: se só um lado declara, não adicionar
  à cadeia; adicionar à lista de anomalias.

- **ADR com variante `-RN` não documentada**: ex:
  `typst-adr-0019-R1-...md` se existir. A anotação do 140A
  é nota factual sem revogação — não cria `-R1`. Confirmar:
  se só há `typst-adr-0019-ttf-rustybuzz.md`, a anotação não
  gera entrada nova na cadeia de revisões.

- **README antigo tem ADRs listados que não existem no
  directório** (ex: ADR-0008–0013 como `(inlining)` no
  README actual — pode ser convenção de "não implementado,
  não tem ficheiro"): registar explicitamente. Se não há
  ficheiro, marcar linha da tabela com nota `(sem ficheiro)`
  ou remover a linha e explicar no relatório.

- **Inventário revela >70 ADRs** (mais do que esperado):
  aceite. O passo dimensionou-se para S-M; se crescer para
  M-L, registar e continuar.

- **Passos-chave da história**: se muitos ADRs entraram entre
  P84.8h e 143, a secção pode crescer. Limite prático:
  **10 entradas novas**. Além desse limite, agrupar por
  "Passos 99–110: fundação de styles tipados" ou similar.

- **Conflito de numeração (143 hyphenation vs 143
  README)**: se alguém estiver a redigir passo de hyphenation
  em paralelo, há colisão. Mitigação: registar no relatório
  que este passo ocupa 143 e o candidato de hyphenation
  passa a 144. Se o outro passo já começou, negociar fora do
  fluxo.

---

## Notas operacionais

- **Passo administrativo**, análogo ao 84.8h (onde este README
  foi criado). O README é documento vivo — como cresce o
  corpo de ADRs, cresce o índice. Trabalho de actualização é
  **contínuo**, não excepcional; este passo fecha 18
  meses de atraso acumulado.

- **Empírico significa empírico**. Não inferir status a partir
  de DEBT.md, relatórios, ou memória. Cada ADR é aberto e
  lido. Contexto dos relatórios é usado só para priorização
  (quais ADRs esperar) e para validação cruzada (se
  relatório diz `IMPLEMENTADO` e ADR diz `PROPOSTO`, há
  anomalia — registar).

- **Sem correcção em cadeia**. Se inventário revelar que
  ADR-X deve estar `REVOGADO` mas está `EM VIGOR`, o README
  transcreve o estado real (`EM VIGOR` na tabela) e regista a
  inconsistência no relatório. Correcção do ADR em si é passo
  futuro.

- **Numeração 143**: conforme decisão operacional acima. O
  candidato de hyphenation passa a 144. Revogação futura não
  é necessária — os passos não são ADRs; só os ADRs têm
  numeração canónica.

- **Após este passo**: próximo momento natural para actualizar
  README é quando o próximo ADR for criado. Se política de
  "actualizar README no mesmo passo que cria ADR" fosse
  introduzida, este atraso de 18 ADRs não recorreria — mas
  essa política é meta-decisão, não matéria deste passo.

- **Volume expectável**: 18 ADRs novos + revisão de 37 ADRs
  existentes + recálculo de 4 tabelas. O grosso do tempo é
  em 143.2 (extracção) — pode ser paralelizado via um único
  comando shell que processa todos os ficheiros de uma vez.

- **Conflito com convenção README actual**: o README declara
  `### Ficheiros de ADR` com regra "sufixo `-RN` após o
  número". Se inventário encontrar ADR com sufixo antes do
  número (ex: `typst-adr-R1-0026-...md`), é anomalia. Precedente
  histórico (P84.8f): `ADR-0026-revisao` foi renomeado para
  `ADR-0026-R1` — conformidade actual esperada.
