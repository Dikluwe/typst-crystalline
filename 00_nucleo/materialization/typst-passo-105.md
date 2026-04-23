# Passo 105 — Auditoria global de DEBTs

**Série**: 105 (passo único; múltiplos sub-passos de inventário e
acção por DEBT).
**Precondição**: Passo 104 encerrado; Sink materializado; 803 L1 +
174 L3 + 6 ignorados; zero violations.
**ADR**: sem ADR nova. Auditoria puramente documental + fechos
oportunistas.

---

## Objectivo

Revisar sistematicamente **todos** os DEBTs registados em
`01_core/DEBT.md` e determinar, para cada um, uma das três
classificações:

- **F (Fechar)** — a dívida foi paga de facto por passos
  posteriores mas o DEBT.md não reflecte. Fechar imediatamente
  neste passo, sem código novo (apenas actualização do DEBT).
- **A (Actualizar)** — a dívida está activa mas o estado descrito
  no DEBT.md está obsoleto (números errados, referências a passos
  que já não se aplicam). Actualizar o texto; manter aberto.
- **M (Manter)** — a dívida continua válida e o estado
  registado está correcto. Sem acção.

O passo **não**:
- Resolve DEBTs que requerem trabalho novo (propagação, refactor,
  materialização). Esses continuam abertos.
- Cria DEBTs novos.
- Toca código de produção, excepto para remover comentários
  `// DEBT-XX` agora órfãos de DEBTs fechados.

---

## Contexto

Os últimos 5 passos (100–104) mostraram, três vezes, que o estado
dos DEBTs no documento divergiu do estado real do código:

- Passo 99: `LazyHash` listado como stub L1 contra ADR-0016 que
  dizia o oposto.
- Passo 102: `#set` listado como "não activo"; estava activo desde
  Passo 30.
- Passo 103: `#show` listado como "não activo"; estava activo
  desde Passo 70.

Este passo é manutenção preventiva. Fecha o desalinhamento
acumulado antes de continuar a empilhar trabalho novo.

---

## Lista conhecida (a confirmar em 105.A)

Do relatório de continuidade pós-96.10 e actualizações até 104:

| DEBT | Título | Estado registado |
|------|--------|-----------------|
| DEBT-1 | StyleChain — pendências residuais | PARCIALMENTE RESOLVIDO (pós-103) |
| DEBT-2 | Closures eager vs lazy | PARCIALMENTE RESOLVIDO (31) |
| DEBT-8 | Motor de equações | PARCIALMENTE RESOLVIDO |
| DEBT-9 | Cobertura de paridade | Tracking contínuo |
| DEBT-33 | ? | EM ABERTO (79–84) |
| DEBT-34d, 34e | ? | EM ABERTO (79–84) |
| DEBT-35b | ? | EM ABERTO (79–84) |
| DEBT-42 | scanner get_unchecked | Bloqueado por benchmark |
| DEBT-43 | linter whitelist crate-level | EM ABERTO (89) |
| DEBT-45 | check_*_depth não chamados | Parcialmente pago (93) |
| DEBT-49 | Propriedades `#set` silenciadas | Primeiro critério pago (104) |
| DEBT-50 | Show selector origin — dvida condicional | ABERTO com canário (103) |
| DEBT-51 | Canal Sink → L3 | ABERTO (104) |

Fechados já registados (ver se estão na Secção 2):
DEBT-39, DEBT-40, DEBT-41, DEBT-44, DEBT-46, DEBT-47, DEBT-48.

---

## Sub-passos

### 105.A — Inventário e leitura do DEBT.md real

Primeiro passo: **ler** `01_core/DEBT.md` na íntegra. A lista acima
é a memória do relatório; o ficheiro real pode ter mais ou menos.

Para cada DEBT encontrado:

1. Registar número, título, estado actual, secção (1 activos / 2
   encerrados).
2. Ler o critério de conclusão (se existe).
3. Identificar o último passo que o tocou (de comentários no
   ficheiro ou grep no workspace).

Escrever em
`00_nucleo/diagnosticos/auditoria-debts-passo-105.md` uma tabela:

```
| DEBT | Título | Estado actual | Critério | Último passo | Classificação |
|------|--------|--------------|----------|--------------|---------------|
| DEBT-1 | ... | PARCIALMENTE RES. | ... | 103 | ??? |
```

A coluna "Classificação" é preenchida em 105.B.

### 105.B — Classificação por DEBT

Para cada DEBT no inventário, aplicar a decisão F/A/M. Critérios:

**F — Fechar** se e só se:
- O critério de conclusão do DEBT está satisfeito pelo estado
  actual do código.
- Verificação empírica confirma (grep mostra que o que era
  pendente já lá está).
- Não é necessário código novo.

**A — Actualizar** se:
- O DEBT tem progresso parcial não registado.
- O estado descrito contradiz o código actual (ex: "aguarda X"
  quando X já foi feito num passo posterior).
- Números no critério (ex: "N sítios pendentes") estão
  desactualizados.

**M — Manter** se:
- O DEBT descreve correctamente o que falta e o critério de
  conclusão é claro.
- Requer trabalho novo para fechar.

**Gates objectivos** (em caso de dúvida, classificar como M):
- Se não tenho a certeza empírica de que o critério está
  satisfeito → M.
- Se o fecho exige escrever código → M.
- Se fechar iria remover referências que o código ainda usa → M.

### 105.C — Acção imediata (F e A)

Para cada DEBT classificado como **F**:

1. Em `01_core/DEBT.md`, mover da Secção 1 (activos) para a
   Secção 2 (encerrados).
2. Adicionar linha: `**ENCERRADO (Passo 105, auditoria)** — <razão empírica>`.
3. Grep por `// DEBT-XX` no workspace. Se houver comentários
   órfãos (DEBT fechado mas comentário persiste), remover os
   comentários. Um comentário pode permanecer se for nota
   histórica pertinente — decidir caso a caso.

Para cada DEBT classificado como **A**:

1. Actualizar o texto em `01_core/DEBT.md` Secção 1 com o estado
   correcto.
2. Adicionar cabeçalho `(actualizado Passo 105)` na entrada.
3. Actualizar critério de conclusão se números mudaram.

**Não** tocar DEBTs classificados como **M**.

### 105.D — Verificação

1. `cargo test --workspace`: contagem **igual** à linha de base
   (803 L1 + 174 L3 + 6 ignorados). Este passo não adiciona nem
   remove testes. Se a contagem mudar, algo foi tocado por
   engano.
2. `crystalline-lint .`: zero violations.
3. `grep -c "^## DEBT-" 01_core/DEBT.md` antes/depois —
   confirmar que o total de entradas na Secção 1 diminuiu pelo
   número de DEBTs fechados.
4. Para cada DEBT fechado: grep por `DEBT-<N>` no workspace —
   confirmar que referências activas no código (se existem) são
   comentários históricos ou foram removidas.

### 105.E — Encerramento

Relatório `typst-passo-105-relatorio.md` com:

- Tabela final: DEBT → classificação (F/A/M) → razão.
- Lista dos fechados com evidência empírica (grep, testes, ADR).
- Lista dos actualizados com diff textual do update.
- Lista dos mantidos com confirmação de que o estado registado
  está correcto.
- Total antes/depois: N DEBTs activos → M DEBTs activos.
- Nota sobre DEBTs cujo critério de conclusão é ambíguo — se
  houver, sugerir clarificação em passo futuro (não neste).

---

## Exemplos de raciocínio para classificação

Para calibrar o executante, quatro exemplos aplicados aos DEBTs
que conheço:

**DEBT-50** (show selector origin, ABERTO com canário):
- Critério: activa-se quando `#set text` migrar para wrapping.
- Estado actual: `#set text` continua em bake-in; o canário
  `debt_50_show_strong_nao_apanha_set_text_bold_porque_bake_in`
  passa.
- Classificação: **M**. Está correctamente aberto com canário a
  guardar. Sem acção.

**DEBT-48** (substituir TextStyle por StyleChain):
- Critério: zero `TextStyle` no Layouter e export.
- Estado actual: Passo 100 fechou-o; `TextStyle` foi redefinido
  como struct resolvido.
- Classificação: **F (já registado)** — confirmar que está na
  Secção 2. Se estiver, não fazer nada. Se ainda está na Secção
  1, mover.

**DEBT-1** (StyleChain — pendências residuais):
- Critério: "activar `#set`/`#show`, remover wrappers, propriedades
  adicionais".
- Estado actual: `#set` activo (102), `#show` activo (103), Strong/Emph
  removidos (101). Propriedades adicionais continuam bloqueadas.
- Classificação: possível **A** — três das quatro pendências
  pagas; actualizar o texto para reflectir que resta apenas
  "propriedades adicionais bloqueadas por tipos não
  materializados".

**DEBT-43** (linter whitelist crate-level):
- Estado registado: EM ABERTO desde Passo 89.
- Estado actual: depende do conteúdo real do DEBT (não tenho
  visibilidade).
- Classificação: exige leitura em 105.A. Se o critério é
  operacional do linter e nada mudou, **M**.

---

## Critério de conclusão

Todas em conjunto:

1. Inventário 105.A escrito com todos os DEBTs.
2. Cada DEBT classificado F/A/M com razão.
3. DEBTs F movidos para Secção 2 com linha de encerramento.
4. DEBTs A actualizados com texto correcto.
5. Comentários `// DEBT-XX` órfãos removidos (ou justificados).
6. `cargo test --workspace` com contagem inalterada.
7. `crystalline-lint` zero violations.
8. Relatório 105.E escrito.

---

## O que pode sair errado

- **Classificação demasiado optimista**. A tentação é fechar
  DEBTs que "parecem fechados" sem confirmar empiricamente. O
  gate "se em dúvida, M" existe precisamente para isto.
  Preferir M mais vezes do que F.
- **Remoção indevida de comentários `// DEBT-XX`**. Um comentário
  pode estar lá como aviso histórico legítimo mesmo depois do
  DEBT fechar. Ler o contexto antes de remover. Regra: remover
  **apenas** se o comentário diz "pendente" ou equivalente; se
  diz "nota: X foi decidido em DEBT-N (fechado)", deixar.
- **Mover DEBT fechado que ainda é referenciado em ADR**. Se uma
  ADR activa diz "ver DEBT-X", fechar DEBT-X não significa
  remover a referência — o DEBT fechado fica na Secção 2 com a
  razão de fecho, a ADR continua válida.
- **Contagem de testes muda**. Se algum teste é removido por
  engano (ex: teste-canário de DEBT fechado), a contagem baixa.
  Isto é bug — testes-canário ficam mesmo após o DEBT fechar,
  como documentação viva.
- **Críterios ambíguos**. Alguns DEBTs antigos (ex: DEBT-8 "motor
  de equações") podem ter critério vago. Se depois de ler o
  texto continua ambíguo, classificar M e registar no relatório
  105.E que o critério precisa de clarificação futura.

---

## Notas operacionais

- Este passo não toca código de produção excepto comentários
  órfãos.
- Não criar DEBTs novos neste passo. Se um DEBT precisa de
  subdivisão, é decisão para passo seguinte.
- Se o inventário 105.A revelar DEBTs duplicados (mesma dívida
  com dois números), **não fundir neste passo**. Registar e
  deixar para passo dedicado.
- DEBT-1 é provavelmente o mais complexo de classificar — tem
  múltiplas sub-pendências em estados diferentes. Se
  classificado como A, a actualização pode ser extensa. Se o
  texto actualizado ficar maior do que uma secção razoável,
  considerar subdividir em DEBT-1a, DEBT-1b, etc. — **mas isto
  é criar DEBTs novos**, portanto **parar** e reportar antes
  de fazer.
