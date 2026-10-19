# Passo 89 — Extensão do ADR-0024 (EcoVec) e abertura do DEBT-43 (gap do linter)

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0024-*.md` — ADR pontual que autoriza
  `ecow::EcoString` para `Value::Str`. Secção "O que esta ADR não
  decide" (linhas 96-99) menciona explicitamente `EcoVec` como
  trabalho futuro.
- `00_nucleo/adr/typst-adr-0018-*.md` — critério de pureza
  funcional para externos em L1.
- `00_nucleo/adr/README.md` — índice canónico dos ADRs e regras
  de formato (vocabulário de status, campos canónicos, convenções
  de cabeçalho).
- `00_nucleo/diagnosticos/diagnostico-ecow-autorizacao-passo-87.md`
  — classificação A (pontual) e identificação do gap do linter
  (whitelist crate-level vs type-level).
- `00_nucleo/DEBT.md` — para adicionar DEBT-43 na Secção 1.
- Configuração do linter (`crystalline.toml` ou equivalente) —
  para inspeccionar o gap identificado.

Pré-condição: `cargo test` — 914 testes (740 L1 + 174 L3, 6
ignorados pré-existentes), zero violations. Passo 88 concluído
(`Traced` materializado).

---

## Natureza deste passo

Passo único de governança arquitectural. Duas tarefas
independentes:

1. **Tarefa A** — Produzir ADR que estende a autorização do
   `ecow` em L1 para cobrir `EcoVec` (e opcionalmente `EcoMap`
   se se justificar). Próximo número disponível no índice
   (após ADR-0034, seria ADR-0035 se não houver outro entre).
2. **Tarefa B** — Adicionar DEBT-43 ao `DEBT.md` (Secção 1)
   documentando o gap do linter: whitelist crate-level permite
   tipos não autorizados por ADRs type-específicos. Sem
   resolução imediata — é dívida de infraestrutura registada
   para passo futuro.

Regra absoluta: **este passo não materializa código**. Não toca
em `01_core/`, `02_shell/`, `03_infra/`, `04_wiring/`. Altera
apenas `00_nucleo/adr/` (novo ficheiro) e `00_nucleo/DEBT.md`
(nova entrada + actualização do índice ADRs).

---

## Decisões formalizadas neste passo

**Decisão 1 (Tarefa A)**: `ecow::EcoVec` é autorizado em L1 para
uso em colecções de L1 que satisfazem o critério da ADR-0018
(pureza funcional).

**Decisão 2 (Tarefa B)**: gap do linter é reconhecido como
dívida técnica (DEBT-43), não como bug crítico. Enforcement
arquitectural continua por disciplina humana + revisão até a
infra do linter ser expandida para whitelisting type-level.

---

## Tarefa A — ADR de extensão para EcoVec

### A.1 — Determinar o número do novo ADR

```bash
# Próximo número disponível:
ls 00_nucleo/adr/typst-adr-*.md | tail -5
```

O ADR mais alto actual é 0034. O novo ADR é **ADR-0035**
(confirmar com `ls` antes de começar).

### A.2 — Criar o ficheiro

Caminho: `00_nucleo/adr/typst-adr-0035-ecovec-autorizacao.md`.

Seguir o template canónico documentado em `00_nucleo/adr/README.md`:

- Cabeçalho com `**Status**: ` `` `EM VIGOR` ``.
- Campo `**Data**: 2026-04-22` (data actual).
- Sem campos de relação (este ADR não revoga nem é revisão).
- Secções: Contexto, Decisão, Alternativas Consideradas,
  Consequências, Referências.

### A.3 — Conteúdo mínimo do ADR-0035

**Secção Contexto**:
- `EcoString` foi autorizado pelo ADR-0024 para uso em
  `Value::Str` (autorização pontual).
- A secção "O que esta ADR não decide" do ADR-0024 antecipou
  que `EcoVec` exigiria ADR separada quando colecções migrassem.
- A série de materializações dos stubs `#[comemo::track]`
  (iniciada no Passo 88 com `Traced`) exige `EcoVec` para
  `Sink` (`EcoVec<Introspection>`, `EcoVec<SourceDiagnostic>`,
  `EcoVec<(Value, Styles)>`) e `Styles` (`EcoVec<LazyHash<Style>>`).
- Sem esta autorização, `Sink` e `Styles` não podem ser
  materializados fielmente ao vanilla.

**Secção Decisão**:
- `ecow::EcoVec<T>` é autorizado em L1 para colecções de tipos
  já autorizados em L1.
- Autorização **não** é automática para `EcoMap`, `EcoArc`, ou
  outros tipos da crate `ecow` — cada um exige extensão ou ADR
  próprio.
- A autorização cobre o tipo `EcoVec` em si, não todos os usos
  de `ecow::*` em L1.

**Secção Alternativas Consideradas** (tabela):

| Alternativa | Razão rejeitada |
|-------------|-----------------|
| `Vec<T>` da stdlib | Perde clone-on-write; clones em APIs ficam O(n). |
| `Arc<[T]>` | Imutável após criação; `EcoVec` suporta mutação com CoW. |
| Tipo custom de L1 | Reinventar roda; `EcoVec` já testado no vanilla. |
| Não autorizar; escrever ADR por cada uso | Processo ruidoso; escopo geral é justificado. |

**Secção Consequências**:
- Positivas: `Sink`/`Styles` podem replicar vanilla; clones O(1)
  em APIs; consistência com `EcoString`.
- Negativas: dependência adicional na crate `ecow` (já estava
  no `Cargo.toml`); enforcement automático **ainda não existe**
  (ver DEBT-43).
- Neutras: a crate `ecow` é mantida pela equipa Typst; licença
  compatível; satisfaz ADR-0018 (sem I/O, determinístico).

**Secção Referências**:
- ADR-0018 (critério de autorização externa).
- ADR-0024 (autorização pontual de `EcoString` — esta ADR
  estende em vez de revogar).
- ADR-0029 (pureza física de L1 — RAM é domínio).
- Passo 86 (diagnóstico que identificou necessidade).
- Passo 87 (classificação A da autorização actual).
- Issue `typst/comemo` ou documentação pública de `ecow`.

### A.4 — Actualizar o índice em `README.md`

O `00_nucleo/adr/README.md` tem uma tabela "Estado por ADR" e
contagens por status. Adicionar uma linha:

```
| 0035 | EcoVec autorizado | `EM VIGOR` |
```

Actualizar a contagem `EM VIGOR` (era 6, passa a 7) e o total
(era 35, passa a 36).

---

## Tarefa B — DEBT-43 no `DEBT.md`

### B.1 — Adicionar entrada na Secção 1 (em aberto)

Posição: após DEBT-42 (último em aberto). Texto sugerido:

```markdown
## DEBT-43 — Linter: whitelist crate-level em vez de type-level — EM ABERTO (Passo 89)

O `crystalline.toml` (ou configuração equivalente do
`crystalline-lint`) usa whitelist crate-level para externos
autorizados em L1. Isto significa que se uma crate tem ao menos
um tipo autorizado via ADR, **qualquer** tipo dessa crate passa
o linter — mesmo tipos cujo uso não foi autorizado.

Exemplo actual: ADR-0024 autorizou `ecow::EcoString` para
`Value::Str` (pontual). Mas `use ecow::EcoVec` passaria o linter
sem reportar violação. A disciplina de respeitar o escopo do
ADR é humana, não automática.

Este DEBT regista o gap. O enforcement arquitectural continua
efectivo via revisão manual de código e pela cultura de
materialização precedida por ADR.

### Proposta de resolução

Estender o formato do linter para aceitar whitelisting por tipo:

```toml
[l1_allowed_external.ecow]
types = ["EcoString", "EcoVec"]
```

Em vez do actual `ecow` estar apenas listado no array de crates
autorizadas sem granularidade.

**Nota sobre escopo da resolução**: esta resolução exige trabalho
em **dois repositórios**:

1. **`crystalline-lint` (projecto separado)** — alterar o parser
   de configuração e a lógica de verificação para ler e aplicar
   o novo formato type-level. Sem esta alteração, o novo formato
   no `crystalline.toml` é ignorado pelo binário actual.
2. **Typst cristalino (este repositório)** — migrar o
   `crystalline.toml` para o novo formato, tipo por tipo,
   partindo das autorizações já estabelecidas pelos ADRs
   (0010-0013 para crates unicode, 0018 para `rustc_hash`, 0023
   para `indexmap`, 0024 para `EcoString`, 0035 para `EcoVec`).

O trabalho no `crystalline-lint` é pré-requisito do trabalho
neste repositório — o `.toml` só tem efeito depois do binário
saber interpretá-lo.

### Dependências

- Trabalho no repositório `crystalline-lint`: alteração do parser
  de configuração e da lógica de verificação. Passo dedicado
  nesse projecto, não neste.
- Após o `crystalline-lint` aceitar type-level whitelisting:
  passo neste repositório para migrar `crystalline.toml` para o
  novo formato.

### Nota sobre escopo

Este DEBT não afecta correcção funcional do código Typst. Afecta
apenas enforcement automático de decisões arquitecturais. É
trabalho de infraestrutura, não de domínio.

### Critério de conclusão

- [ ] `crystalline.toml` aceita whitelist type-level para externos.
- [ ] Pelo menos uma crate (sugestão: `ecow`) migrada para o novo
      formato.
- [ ] Tipo não autorizado dessa crate é reportado como violação
      em teste.
- [ ] Documentação actualizada no README do `crystalline-lint`.

---
```

Ajustar o texto acima conforme necessário — formato deve seguir
o padrão dos outros DEBTs em aberto (DEBT-40, DEBT-41 encerrado,
DEBT-42).

### B.2 — Sem alterar a ordem das outras entradas

DEBT-40, DEBT-42 permanecem onde estão. DEBT-43 é acrescentado
no fim da Secção 1, antes da transição para Secção 2.

---

## Critérios de conclusão

- [ ] Ficheiro `00_nucleo/adr/typst-adr-0035-ecovec-autorizacao.md`
      existe com as 5 secções mínimas.
- [ ] `**Status**: ` `` `EM VIGOR` `` no cabeçalho do ADR-0035.
- [ ] `00_nucleo/adr/README.md` actualizado: nova linha na tabela,
      contagens actualizadas, total passa a 36.
- [ ] Entrada DEBT-43 adicionada no `00_nucleo/DEBT.md` Secção 1,
      após DEBT-42.
- [ ] Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/`,
      `04_wiring/` foi alterado.
- [ ] Nenhum outro ADR alterado (ADR-0024 mantém-se; este passo
      **estende**, não revisa).
- [ ] `cargo test` passa com os mesmos 914 testes (740 L1 + 174 L3
      + 6 ignorados). `cargo run --package crystalline-lint` → zero
      violations.

---

## Ao terminar, reportar

Tarefa A:
- Caminho do ficheiro ADR-0035 criado.
- Tamanho (linhas).
- Linhas actualizadas em `README.md`.

Tarefa B:
- Linha de início e fim da nova entrada DEBT-43 no `DEBT.md`.
- Tamanho da entrada (linhas).

Verificação:
- Contagem de testes (deve estar inalterada).
- Contagem de violações do linter (deve ser zero).

Go/No-Go para Passo 90:
- **Go** se ADR-0035 está em vigor, DEBT-43 registado, e
  pré-condição preservada. Passo 90 = materializar `Route`
  (desbloqueia DEBT-40 como efeito secundário).
- **No-Go** se:
  - O número ADR-0035 já estava ocupado (passar a próximo
    disponível).
  - O `00_nucleo/adr/README.md` tem formato que não aceita
    a linha directamente (reportar antes de alterar).
