# Passo 995 — Reconciliação `(n \ k)` secção 7: medição isolada fresca

**Tipo**: Verificação/reconciliação — sem fix previsto (a menos que a medição confirme
regressão real, caso em que este passo pára e devolve diagnóstico, não corrige).
**Motivo**: fechar a dúvida em aberto desde a mensagem "Correcção de rumo" — uma
"investigação paralela" (noutra conversa) reportou `(n \ k)` com folga zero em "n", ainda
por corrigir, contradizendo P991 (que reportou correcção, revalidada de forma isolada por
`b59db7f16` com sha256 confirmado). Não tenho, e o dono não tem, o commit/binário exacto
usado na medição paralela.
**Âmbito**: só esta reconciliação. **Não** cobre o achado 4/secção 36 (transformações
`rotate`/`scale` no documento estendido) — fica registado como pendente, próximo passo
separado, per indicação do dono ("vamos resolver um de cada vez").

---

## Antes de começar

`git status` — confirmar HEAD em `f59278441` (P994) ou posterior conhecido. Se houver
divergência, registar o hash real encontrado antes de prosseguir.

---

## Passo 1 — Tentar obter os dados em falta (não bloqueante)

Pedir ao dono, mais uma vez e de forma directa, os três dados que fecham a comparação sem
ambiguidade, caso ainda estejam acessíveis na conversa paralela:
1. Commit ou hash de binário usado na medição paralela.
2. Ficheiro `.typ` exacto medido — é `typst-math-comprehensive-test.typ` secção 7 (o mesmo
   documento de P991), ou outro ficheiro (por exemplo o estendido, `secção 32+`)?
3. Comando exacto usado para medir (assumir `pdftotext -bbox` salvo indicação em
   contrário, mas confirmar).

**Se estes dados chegarem antes do Passo 2 ser executado**: comparar directamente contra
esses valores, sem precisar do resto deste passo além da confirmação de identidade de
binário (P934). **Se não chegarem**: prosseguir de qualquer forma com o Passo 2 — a
ausência de proveniência do lado paralelo não pode bloquear indefinidamente a
reconciliação; a medição fresca abaixo é o desempate independente do que a outra
conversa mediu.

## Passo 2 — Medição isolada fresca, a partir do HEAD actual

Objectivo: eliminar qualquer dúvida sobre "qual binário/estado" produzindo uma medição
nova, a partir do estado actual do repositório (que já inclui P991, P992, P993, P994),
com proveniência completa e verificável por qualquer pessoa depois.

1. Confirmar identidade do binário cristalino a usar — build a partir do HEAD actual
   (`f59278441`), **não** reaproveitar nenhum binário antigo em `target/release/` que
   possa ter sido reconstruído por passos posteriores sem ser exactamente este commit
   (mesma lição já aplicada em P991 — o `target/release/typst` da working tree não é
   garantidamente o artefacto exacto de um passo anterior).
   ```
   git worktree add --detach /tmp/p995-cristalino f59278441
   cd /tmp/p995-cristalino && cargo build --release
   sha256sum target/release/typst
   ./target/release/typst --version   # confirmar string distintiva "crystalline"
   ```
2. Confirmar identidade do binário vanilla — usar o baseline já ratificado
   (`typst-retificacao-p990-p992-lab-sync.md`, upstream/main `a51e02804`, pinado). Build
   isolado da mesma forma, não reaproveitar o binário do sistema sem confirmar:
   ```
   sha256sum /usr/local/bin/typst   # ou o binário usado como referência actual
   /usr/local/bin/typst --version   # registar a string completa, incluindo o hash
   ```
   Registar explicitamente no relatório: esta string **parece** "0.15.1" mas é
   main+93 com hash do repo local (per a retificação) — não confundir com o 0.15.1
   oficial. Se por algum motivo o binário vanilla disponível não for este baseline
   ratificado, parar e reportar antes de medir (não presumir qual está instalado).
3. Compilar `typst-math-comprehensive-test.typ` (o mesmo documento de P991) com os dois
   binários isolados acima.
4. `pdftotext -bbox` sobre a secção 7 nos dois PDFs resultantes. Extrair as coordenadas de
   "n" e "k" em cada um.
5. Comparar com os dois pontos de referência já existentes:
   - P991 original: "n" flush (0.00pt/0.00pt), "k" com 0.352pt/0.517pt.
   - Reconstrução isolada anterior (commit `b59db7f16`, sha256 `7301ee12…`): coordenadas
     idênticas, byte a byte, às de P991 original.

## Passo 3 — Interpretar o resultado

- **Se a medição fresca do Passo 2 bater com P991/reconstrução anterior**: a correcção
  está confirmada, reprodutível, e estável através de P992/993/994 (nenhum passo
  posterior reintroduziu a regressão). Fechar a reconciliação com esta evidência como
  desempate — a "investigação paralela" não forneceu proveniência verificável, e três
  medições independentes e reprodutíveis (P991 original, reconstrução isolada, esta) já
  convergem. Registar isto explicitamente no relatório, sem acusar a investigação paralela
  de erro (não há dados suficientes para essa afirmação) — só registar que a nossa cadeia
  de evidência está fechada e consistente.
- **Se a medição fresca divergir** (por exemplo "n" deixa de estar flush): **isto é uma
  regressão real introduzida por P992, P993 ou P994** — parar, não corrigir neste passo.
  Fazer `git bisect` isolado (mesma técnica de worktree, um binário por commit candidato:
  `b59db7f16`, e depois cada passo entre esse e `f59278441`) para identificar exactamente
  qual commit reintroduziu o problema. Reportar como achado novo, com o commit exacto
  identificado — não como "confirmação do achado paralelo" (continuamos sem saber se é a
  mesma coisa que a investigação paralela mediu, só que existe uma regressão real, medida
  por nós).

---

## Resultado esperado

Um destes dois:
1. Reconciliação fechada — cadeia de evidência consistente (P991 + reconstrução isolada +
   esta medição), registada com hashes completos, sem ambiguidade residual.
2. Regressão real encontrada e localizada por `git bisect` isolado — novo achado, passo de
   correcção a escrever a seguir (fora do âmbito deste passo).

Em qualquer dos dois casos, o relatório final **fecha com o hash do commit** (nenhum
placeholder "commit final no fim" — regra já reforçada duas vezes nesta frente).
