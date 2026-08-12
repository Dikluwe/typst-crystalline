# Passo 999 — Auditoria de todos os prompts (Fase 1): atomização, referências a passo, completude, ambiguidade

**Tipo**: Auditoria — **catalogar, não corrigir** neste passo. O volume é grande demais
para corrigir tudo numa só tacada sem risco de escolher o mais fácil em vez do mais
importante (ver Passo 995/discussão de prioridade).
**Regra nova, efectiva a partir de agora** (confirmada pelo dono): **nenhum prompt em
`00_nucleo/prompts/` pode referenciar um número de passo, em nenhuma secção, incluindo
`Criado em`/`Histórico de Revisões`.** Proveniência de passo só existe no relatório desse
passo — os prompts são o produto permanente e agnóstico do mecanismo que os gerou.
**Âmbito**: só `00_nucleo/prompts/`. ADRs ficam fora — decisão adiada para depois da
paridade estar fechada (algumas ADRs vão desaparecer, outras vão ser reordenadas; essa
decisão precisa da paridade concluída primeiro).

---

## Espinha dorsal a verificar (a confirmar, não a repetir de cor)

1. Prompts geram código — direcção sempre prompt → código.
2. Prompts são a inteligência do sistema — o conhecimento vive neles.
3. Passos são mecanismo, descartável — não deixam resíduo na coisa que produzem.
4. Prompts são agnósticos do mecanismo — têm de fazer sentido sozinhos.
5. Prompts ditam a atomização — 1 prompt : 1 ficheiro de código.

Cada achado deste passo deve dizer contra qual destes cinco pontos o prompt falha.

---

## Fase A — Inventário (obrigatório antes de qualquer avaliação, per ADR-0065)

```
find 00_nucleo/prompts -name '*.md' | sort
```

Registar a contagem total antes de decidir tamanho do swarm. Particionar por
subdirectório (`entities/`, `engine/`, `rules/`, `infra/`, `math/` ou equivalente
existente) — um agente por partição, mesma lógica do P998 (19 agentes por página de
documentação).

## Fase B — Critérios de auditoria (aplicar os 4, por prompt)

### Critério A — Atomização (ponto 5 da espinha dorsal)

- Confirmar 1:1 entre o prompt e o ficheiro de código que ele gera. Já há violações
  conhecidas e catalogadas em `ADR-0104` — **começar por confirmar se continuam por
  resolver**: `rules/eval.md`, `rules/parse.md`, `engine/layout.md` foram listados como
  candidatos a fatiar, não fatiados até à data desse ADR. Verificar estado actual antes de
  assumir que ainda se aplica.
- Qualquer prompt novo (não listado em ADR-0104) que cubra mais do que um ficheiro de
  código gerado — registar como achado novo de atomização.

### Critério B — Zero referência a passo (ponto 3/4 da espinha dorsal)

`grep -rniE '\b[Pp]asso[[:space:]]*[0-9]|\bP[0-9]{2,4}[A-Za-z]?\b' 00_nucleo/prompts/`

Para cada ocorrência:
- Registar ficheiro, linha, secção onde aparece (Contexto/Restrições/Instrução/Critérios/
  Resultado Esperado/Criado em/Histórico de Revisões — **todas** contam agora, sem
  excepção).
- Classificar gravidade:
  - **Leve**: só no campo `Criado em`/`Histórico de Revisões`, sem afectar a leitura do
    resto do prompt — remoção mecânica e segura.
  - **Grave**: a secção substantiva (Contexto/Instrução) depende do número de passo para
    ser compreendida — precisa de reescrita, não só remoção (exemplo já confirmado:
    `entities/content.md`, secção "Instrução" organizada inteiramente por intervalos de
    passo).

### Critério C — Completude estrutural (per `template-prompts.md`)

Confirmar presença das secções obrigatórias: Contexto, Restrições Estruturais, Instrução,
Critérios de Verificação, Resultado Esperado. (Histórico de Revisões fica **descontinuado**
como secção — decisão desta auditoria, per a regra nova; não confirmar a presença, marcar
para remoção onde existir.) Falta de qualquer uma das 5 = incompletude objectiva, registar.

### Critério D — Ambiguidade/omissão de conteúdo

Ler `.agents/workflows/auditar-spec.md` **primeiro**, na íntegra, e aplicar os critérios
que lá estiverem definidos — não presumir nem inventar critérios próprios aqui. Se o
workflow estiver, ele próprio, incompleto ou desactualizado, registar isso também como
achado (meta-achado, reportar separadamente).

## Fase C — Cruzamento com o corpus de documentação (só prompts de `math/`)

Para os prompts que cobrem elementos/funções já presentes no corpus `P998`
(`00_nucleo/corpus-docs/math/*.typ`), cruzar o conteúdo do prompt com as citações verbatim
já extraídas da documentação oficial. Registar:
- Onde o prompt descreve um comportamento que contradiz a citação da documentação.
- Onde o prompt é omisso sobre algo que a documentação define explicitamente (ex.: os três
  usos distintos de `\` identificados na pesquisa do Passo 998 — confirmar se algum prompt
  de math os documenta todos, ou só um).
- Isto **não substitui** o Critério D — é um cruzamento adicional, só possível porque o
  corpus já existe para `math/`. Não expandir a outras áreas neste passo (o corpus só
  cobre `math/` até agora).

## Fase D — Catálogo final (output deste passo)

Um relatório, sem código alterado, com:
1. Tabela por prompt: ficheiro, Critério A (violação sim/não), Critério B (nº ocorrências,
   leve/grave), Critério C (secções em falta), Critério D (achados do workflow), Critério
   E/cruzamento doc (só math, achados se houver).
2. Ordenação por gravidade combinada, não por ordem alfabética — os prompts com violação
   grave de B ou C ficam no topo.
3. **Nenhuma correcção aplicada neste passo.** Decisão de que corrigir primeiro fica para
   depois deste catálogo existir — mesma disciplina já usada em P998 (Fase D: achados,
   não fixes).

---

## Fora de âmbito deste passo

- ADRs — adiado, decisão do dono, depende da paridade estar fechada primeiro.
- Corrigir qualquer prompt — só catalogar.
- Expandir o corpus de documentação a outras secções além de `math/` — já coberto pelo
  âmbito do Passo 998, não repetir aqui.
