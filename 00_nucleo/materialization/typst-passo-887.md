# Passo 887 — `table()` sem linhas de grelha (stroke default ausente)

**Precede este passo**: `typst-passo-885.md` (achados originais),
`typst-passo-885-relatorio.md` (confirmação visual — achado 3, secção 4), e
`typst-passo-886.md` (achado 2 — ler o veredicto da Fase A desse passo sobre
causa comum antes de começar este).

**Se P886 concluiu que os achados 2 e 3 partilham causa raiz**: confirmar
primeiro se este sintoma já desapareceu depois da correcção de P886, antes
de investigar de novo do zero. Se já estiver corrigido, este passo passa a
ser só verificação + fecho, não implementação nova — registar isso no
relatório e não duplicar trabalho.

**Pré-condição de árvore**: mesma nota de P886 — confirmar `git status`
antes de tocar em qualquer ficheiro; decidir e registar explicitamente como
lidar com trabalho pendente não commitado, se ainda existir nesta altura.

---

## Sintoma confirmado

Fonte de teste: `table(columns: 5, rows: 10, ..range(50).map(str))`, **sem**
`stroke:` explícito (`05-tables.typ`). Em Typst, `table()` desenha
`1pt + black` por omissão quando `stroke` não é passado — diferente de
`grid()`, que não desenha nada por omissão.

- **Vanilla**: grelha completa, borda visível em todas as células, nas 4
  páginas.
- **Cristalino**: os mesmos números (`0`–`49`, mesma disposição), mas sem
  nenhuma linha visível. Contagem de operadores `S` (stroke) no content
  stream: 371 no vanilla, 3 no cristalino, para o mesmo documento (4
  páginas, 20 tabelas, 5×10 células cada).

## Fase A — Diagnóstico (obrigatória antes de qualquer edição)

1. Localizar onde o cristalino resolve o `stroke` de `table()` quando não é
   passado explicitamente pelo utilizador — isto é, onde deveria existir o
   default `1pt + black`. Não presumir a localização — procurar pelo
   consumer de `table()` no layouter e no exportador PDF (candidatos
   razoáveis, não confirmados: onde `Content::Grid`/tabela é tratado no
   layout, e a parte do exportador que emite `S`/`re` para bordas de
   células — mencionada como cluster central em `03_infra/src/export/`
   pelo handoff pós-P884, mas tabela pode estar num módulo separado dos três
   listados lá).
2. Determinar se o default `1pt + black`:
   - nunca foi implementado (o valor por omissão de `stroke` é `none` no
     cristalino, divergindo do vanilla desde a origem do módulo `table`), ou
   - foi implementado mas o valor está a ser calculado e depois descartado
     na exportação (mesmo padrão "computado mas não emitido" suspeitado no
     achado 2), ou
   - foi implementado correctamente antes e regrediu nalgum passo posterior
     (nesse caso, tentar localizar em qual passo — `git log` / `git blame`
     no ficheiro relevante, cruzando com a lista de passos P872–P884 que
     mexeram em export).
3. Confirmar no vanilla (`lab/typst-original/`) qual é exactamente o default
   e onde é aplicado, para ter uma referência concreta do que replicar — não
   assumir "1pt + black" sem confirmar no código-fonte do vanilla.

**Não avançar para a Fase B sem a Fase A escrita no relatório**, incluindo
qual das três hipóteses acima se confirmou.

## Fase B — Implementação (TDD, per `CLAUDE.md`)

1. Escrever teste(s) que falhem primeiro — teste de unidade sobre a função
   que resolve o `stroke` default de uma tabela (deve devolver `1pt + black`
   quando `stroke` não é especificado), mais um teste ao nível do
   exportador confirmando que o content stream produzido para uma tabela
   sem `stroke` explícito contém operadores de desenho de borda. **Verificar
   que os testes falham antes de escrever código de produção.**
2. Implementar a correcção no ponto identificado pela Fase A.
3. Verificar que os testes novos passam e que a suíte completa continua
   verde, discriminada por crate (mesma exigência de P886).
4. Recompilar `05-tables.typ` (fonte actual) nos dois binários e confirmar
   visualmente que o cristalino agora desenha as linhas da tabela, com
   contagem de operadores `S` comparável à do vanilla (não precisa ser
   idêntica — mas não pode ficar na casa de unidades quando o vanilla está
   na casa das centenas).
5. `cargo run -- .` — zero violations.

## Fase C — Regressão

Mesma exigência de P886 — este achado também foi descoberto na frente de
performance P872–P884. Correr o benchmark completo (7 cenários) e reportar
os números antes de fechar. Se P886 já correu o benchmark completo depois
da sua própria correcção e nada neste passo tocar fontes/export além do
ponto identificado na Fase A, pode reaproveitar-se essa medição como
baseline e correr só a medição final pós-P887 — mas registar explicitamente
que essa é a decisão tomada, não presumir que é óbvio.

## Resultado esperado

- Header de linhagem actualizado no(s) ficheiro(s) tocado(s), apontando para
  este prompt.
- Testes novos cobrindo o default de stroke (unidade + confirmação E2E via
  PDF).
- Relatório do passo com: veredicto da Fase A (qual das três hipóteses),
  diff resumido da Fase B, números do benchmark completo da Fase C, e nota
  sobre a árvore de trabalho.
