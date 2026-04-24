# Passo 118 — Auditoria de atribuição de camadas (L1/L3/L4 → L2?)

**Série**: 118 (passo de **auditoria** pura; zero código de
produção).
**Precondição**: Passo 117 encerrado (CLI migrada para L2); 811 L1
+ 6 L2 + 201 L3 + 5 L4 + 6 ignorados; zero violations. Total 1023.
**ADRs aplicáveis**: ADR-0049 (CLI em L2), definição fundacional
em `typst-migracao-estado.md`.
**ADR nova**: **não** neste passo. Este passo produz diagnóstico
e recomendação para passos de correcção futuros.

---

## Natureza deste passo

**Auditoria pura.** O Passo 117 descobriu que 4 passos consecutivos
puseram CLI em camadas erradas. A pergunta natural: **quantas outras
responsabilidades estão na camada errada?** L2 ficou adormecido
durante toda a migração — outros elementos que "cheiram a
apresentação" podem ter ido para L3 por inércia.

Este passo faz travessia sistemática dos 3 crates cristalinos
procurando **inconsistências de camada** conforme a definição
fundacional:

- **L1**: domínio puro, zero I/O, zero contexto.
- **L2**: interface com utilizador — argparsing, formatação
  para apresentação, tradução de mundo sujo para intenção pura.
- **L3**: I/O braçal — filesystem, rede, fonts, bytes.
- **L4**: composição thin — maestro.

O critério prático (derivado do Passo 117):

| Cheiro | Pertence |
|--------|----------|
| Produz/consome bytes, toca filesystem/rede | L3 |
| Decide formato user-facing, cores, palavras | L2 |
| Traduz tipos internos para tipos user-facing | L2 |
| Orquestra L1+L3 | L4 (ou L3 como "pipeline braçal" se L4 cresce) |
| Não faz I/O nem decide apresentação | L1 |

Este passo **não produz código**. Produz ficheiros de diagnóstico
e uma **lista ranqueada de candidatos a migração**. A decisão do
que migrar e quando é para conversas seguintes.

---

## Objectivo

Ao fim do passo, estarão disponíveis:

1. **Inventário de L3 completo** — cada módulo/função pública com
   classificação (L3 correcto / candidato L2 / candidato L4 /
   fronteiriço).
2. **Inventário de L1** — verificar que não há contaminação
   "apresentacional" (improvável mas completude exige).
3. **Inventário de L4** — confirmar que L4 continua thin (pós-117)
   e que nenhuma lógica recuperou para lá.
4. **Lista ranqueada de candidatos a migração** (se algum),
   com:
   - Origem (ficheiro:linha).
   - Destino sugerido.
   - Razão factual.
   - Tamanho estimado do refactor.
   - Interdependências.
5. **Recomendação** para passo(s) de correcção.

---

## Escopo

**Dentro**:
- `view`/`grep` em `01_core/src/`, `02_shell/src/`,
  `03_infra/src/`, `04_wiring/src/`.
- Leitura de `Cargo.toml` de cada crate.
- Produção de ficheiros de diagnóstico.

**Fora**:
- Qualquer código de produção.
- ADR nova.
- Alteração a ficheiros fora de `00_nucleo/diagnosticos/`.
- Testes novos.

---

## Sub-passos

### 118.A — Inventário de L3

Mais crítico — onde o erro do Passo 116 aconteceu, provável
sítio de outros erros similares.

**Parte 1 — Listagem de módulos**:

1. `view` em `03_infra/src/`. Listar todos os ficheiros `.rs`.
2. `view` em `03_infra/src/lib.rs`. Listar módulos públicos.
3. Grep por `pub fn` em cada ficheiro — registar assinatura.

**Parte 2 — Classificação por função**:

Para cada função pública de L3, classificar:

| Classificação | Critério |
|---------------|----------|
| **L3 correcto** | Faz I/O real (filesystem, bytes, network, fonts). |
| **Candidato L2** | Decide formato user-facing, cores, tradução. |
| **Candidato L4** | É composição de L1+L3 (orquestração). |
| **Fronteiriço** | Mistura I/O e decisão — merece análise dedicada. |

Registar em tabela:

```
| Ficheiro | Função | Classificação | Razão |
|----------|--------|---------------|-------|
| diagnostic_format.rs | format_diagnostic | Candidato L2 | Produz string legível com escapes ANSI. |
| diagnostic_format.rs | drain_diagnostics_to_stderr | Fronteiriço | eprint! é IO trivial; strings vêm de função candidata a L2. |
| pipeline.rs | compile_to_pdf_bytes | Candidato L4 | Orquestra eval+layout+export_pdf. |
| pipeline.rs | eval_to_module_with_sink | Candidato L4 | Orquestra eval+sink. |
| system_world.rs | SystemWorld | L3 correcto | Lê filesystem, gere FileId↔path, descobre fontes. |
| export/pdf.rs | export_pdf | L3 correcto | Produz bytes PDF. |
| ...
```

Escrever em
`00_nucleo/diagnosticos/auditoria-l3-passo-118.md`.

### 118.B — Inventário de L1

Verificação de completude. Improvável mas necessária para sentir
que a auditoria cobre tudo.

**Parte 1 — Procurar sinais de apresentação**:

1. `grep` por palavras-chave típicas em `01_core/src/`:
   - `"warning:"`, `"error:"`, `"hint:"` (strings literais).
   - `"\x1b["`, `ANSI`, `color` (escapes ANSI).
   - `Display::fmt` que produza formatos user-facing
     (distinguir de `Debug`).
   - Referências a `stderr`, `stdout`, `print`.
2. Cada match: classificar.
   - Provavelmente todos são legítimos (`Display` em `Value`
     para erros compile-time, strings de diagnóstico do eval,
     etc.).
   - Se algum parece "formatação para CLI", marcar para análise.

**Parte 2 — Excluir falsos positivos**:

- `Display` em tipos de domínio (`Value`, `Content`) é L1 — é
  representação textual do tipo, não formatação CLI.
- Strings literais em `SourceDiagnostic::warning(...)` são
  conteúdo da mensagem, não formatação.

**Parte 3 — Registo**:

Se nada encontrado, registar "L1 limpo — zero candidatos de
migração". Isto é resultado válido.

Escrever em
`00_nucleo/diagnosticos/auditoria-l1-passo-118.md`.

### 118.C — Inventário de L4

Confirmar que pós-117 L4 é thin. O Passo 117 terminou com ~75
linhas em `main.rs` — verificar que ainda é assim.

**Parte 1 — Contagem**:

1. `view` em `04_wiring/src/main.rs`. Contar linhas úteis
   (excluir header, `use`, vazias).
2. Registar.

**Parte 2 — Classificação linha-a-linha**:

Para cada grupo lógico em `main()`:

- Parsing de args → chamada a L2 (correcto).
- Construção de SystemWorld → orquestração (aceitável em L4).
- Chamada ao pipeline → orquestração (aceitável).
- Drain de diagnósticos → chamada a L3 (correcto).
- Decisão de exit code → orquestração (aceitável).
- **Qualquer lógica de formatação ou decisão não-estrutural**:
  migrar para L2.

Se algum grupo falha a classificação, candidato.

**Parte 3 — Verificar imports**:

1. `grep` por `use` em `04_wiring/src/main.rs`.
2. Confirmar: nenhum `use clap::`, `use std::env::` (excepto
   `temp_dir` em testes), nenhum `use std::io::` directo para
   escrever.
3. Se algum estiver, L4 tem lógica que devia migrar.

Escrever em
`00_nucleo/diagnosticos/auditoria-l4-passo-118.md`.

### 118.D — L2 sanity check

L2 foi criado no Passo 117. Confirmar que o que lá está é
apropriado e que não há contaminação inversa.

**Parte 1 — Verificar que L2 não faz I/O**:

1. `grep` por `std::fs`, `std::env` (excepto `var_os` para
   env vars — permitido em L2), `std::io::Write` em
   `02_shell/src/`.
2. Nenhum destes deve estar. L2 lê env vars, não escreve
   ficheiros.

**Parte 2 — Verificar que L2 não importa L3**:

1. `grep` por `typst_infra` em `02_shell/src/`.
2. Não deve haver — L2 conhece L1 (domínio) mas não L3 (I/O).

Escrever em
`00_nucleo/diagnosticos/auditoria-l2-passo-118.md`.

### 118.E — Ranking e relatório

Produzir ranking de candidatos em
`00_nucleo/diagnosticos/candidatos-migracao-camadas-passo-118.md`.

Formato:

```
Candidato N — <título>
  Origem: <crate>/src/<ficheiro>.rs:<linha(s)>
  Destino: L<N>
  Razão: <factual>
  Tamanho: XS / S / M / L (referência: 117 foi M)
  Interdependências: [... outros candidatos]
  Bloqueios: [... o que tem de ser feito antes]
  Passo sugerido: corrigir no mesmo passo / separado / pode ficar
```

Ranking por:

1. **Clareza do erro** — "candidato óbvio" primeiro.
2. **Tamanho** — XS/S primeiro.
3. **Independência** — sem deps de outros candidatos primeiro.

Relatório final `typst-passo-118-relatorio.md` agrega:

- Resumo executivo (quantos candidatos em cada categoria).
- Ranking completo.
- Recomendação primária: qual candidato migrar primeiro (ou
  "auditoria limpa, zero candidatos").
- Ficheiros de diagnóstico produzidos.

---

## Critério de conclusão

Todas em conjunto:

1. Ficheiros de diagnóstico em `00_nucleo/diagnosticos/`:
   - `auditoria-l3-passo-118.md`
   - `auditoria-l1-passo-118.md`
   - `auditoria-l4-passo-118.md`
   - `auditoria-l2-passo-118.md`
   - `candidatos-migracao-camadas-passo-118.md`
2. Relatório `typst-passo-118-relatorio.md` escrito.
3. **Zero** código de produção alterado.
4. **Zero** ADRs novas.
5. **Zero** testes novos ou removidos.
6. `cargo test --workspace` com contagem inalterada (1023).
7. `crystalline-lint` zero violations.

---

## Candidatos já antecipados (pré-inventário)

Baseado no contexto desta conversa, espero encontrar pelo menos
estes candidatos. O inventário confirma ou corrige.

1. **`format_diagnostic`** (L3 → L2). Decide cores, palavras
   ("warning:"), indentação. É formatação para apresentação
   humana. A única razão para ficar em L3 era proximidade ao
   `drain_*`. Separável.

2. **`drain_diagnostics_to_stderr`** (L3 → possível L4 ou
   desaparecer). É `for diag in diagnostics { eprint!(format_diagnostic(...)) }`.
   Triviais. Se `format_diagnostic` migrar para L2, o drain
   fica em L4 (3 linhas de loop) ou em L2 (mas L2 ideal não
   faz I/O).

3. **`compile_to_pdf_bytes`** (L3 → L4 ou fica). Orquestra
   `eval`, `layout`, `export_pdf`. É composição. Fica em L3
   como "pipeline braçal" ou move para L4 se L4 puder crescer.

4. **`eval_to_module_with_sink`** (L3 → L4 ou fica). Igual.

Candidatos possíveis se aparecerem:

5. Outros módulos em L3 com `format`, `display`, `render` no nome.
6. Qualquer `mod ... { pub fn ... }` que processe strings para
   output humano.

Espero L1 limpo. Espero L4 thin. Mas só o inventário confirma.

---

## O que pode sair errado

- **Auditoria revela candidato grande e bloqueante**. Se
  `compile_to_pdf_bytes` for classificado "deveria estar em L4",
  mover é refactor maior que o 117 — L4 cresce, precisa de
  validação. Aceitar e registar como grande. Decisão do que
  fazer fica para conversa seguinte.
- **Candidatos fronteiriços sem resposta clara**. Se o inventário
  produzir 5 "fronteiriços" sem recomendação, o relatório está
  mal feito. Cada fronteiriço tem de ter **recomendação explícita**
  mesmo se for "manter por falta de razão forte para mover".
- **Descoberta que um passo anterior fez outro erro**. Provável
  dado o padrão. Registar factualmente sem julgamento.
- **Número crescente de candidatos**. Se aparecer > 10 candidatos,
  o passo pode virar "cancelar a CLI do Passo 117" versão
  exagerada. Provável que não — 117 só tocou CLI; o resto de L3
  é infra legítima.
- **Tentação de corrigir "só um"**. Este passo **não constrói**.
  Se aparece candidato óbvio e pequeno (ex: renomear um ficheiro),
  resistir — registar como candidato, não executar.
- **Dependência cíclica detectada**. Improvável mas possível: se
  L2 precisaria de importar algo que está em L3, e esse algo
  ideal seria L2, há ciclo. Registar para análise.

---

## Notas operacionais

- Este passo não toca código. Se `cargo test` regride, algo foi
  tocado por engano.
- O candidato mais óbvio é `format_diagnostic`. Verificar
  primeiro em 118.A — deve aparecer como classe "Candidato L2"
  imediatamente.
- L4 pós-117 é thin. Se o inventário revelar contaminação,
  regressão silenciosa aconteceu entre Passos 117 e 118.
  Improvável (nenhum passo no meio) mas possível.
- Para cada candidato, avaliar **interdependência**: se
  `format_diagnostic` e `drain_*` são tratados como par, migrar
  um sem o outro cria fronteira estranha. Candidatos acoplados
  migram juntos.
- A arquitectura Cristalina permite que L3 tenha "braços"
  chamados por L2. Por exemplo, se L2 tiver `cli::drain_to_stderr`
  (decide cor, formato) que internamente use
  `typst_infra::io::write_to_stderr` (bytes para stderr),
  ambas as camadas fazem a sua parte. Mas se esse split adiciona
  cerimónia sem ganho, L4 fazer `eprint!` directamente é OK.
- `crystalline-lint` é a fonte de verdade. Este passo produz
  diagnóstico humano; o lint continua a ser quem valida.
