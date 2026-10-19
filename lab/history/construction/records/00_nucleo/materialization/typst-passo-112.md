# Passo 112 — Análise para CLI real em `04_wiring`: inventário e decisão

**Série**: 112 (passo de **análise**, não de construção).
**Precondição**: Passo 111 encerrado (formato rico de diagnósticos);
811 L1 + 189 L3 + 6 ignorados; zero violations.
**ADRs aplicáveis**: ADR-0033 (paridade funcional), ADR-0043
(canal Sink), ADR-0045 (formato de diagnósticos).
**ADR nova**: **não** neste passo. Este passo produz enquadramento
para a ADR que virá no passo de construção seguinte.

---

## Natureza deste passo

Três decisões estruturais para a CLI foram diferidas ao inventário:

- **Escopo** (micro / mínimo / vanilla subset).
- **Argparsing** (manual / clap / argh).
- **File loading** (single-file / World real em L3).

Cada uma depende de factos empíricos: o que já existe em
`04_wiring/`, que API PDF está estável, se algum `World` está em
L3, que deps são autorizadas em L4. Ir cegamente seria antecipação.

O Passo 108 (análise Introspection) estabeleceu o padrão para
passos deste tipo: ficheiros de diagnóstico + recomendação, zero
código de produção.

**Este passo não produz código.** Produz diagnósticos em
`00_nucleo/diagnosticos/` e recomendação num relatório.

---

## Objectivo

Ao fim deste passo, estarão disponíveis:

1. **Inventário do `04_wiring/` actual** — o que existe, o que é stub.
2. **Inventário da infra L3 que a CLI vai consumir** — eval, layout,
   PDF export, Sink drain, formatters.
3. **Inventário do que o vanilla CLI faz** — lista de subcomandos,
   flags, comportamento. Não precisa cobrir tudo; só identificar o
   perímetro.
4. **Candidatos de escopo** (micro / mínimo / subset), cada um com:
   - Tipos/APIs exactas a invocar.
   - Deps externas necessárias.
   - Tamanho estimado.
   - O que desbloqueia em termos de uso externo real.
5. **Recomendação** para o Passo 113 de construção.

A escolha de escopo, argparsing e loading é **decidida depois da
análise**, com base nos factos. Pode ser que o inventário revele
que "single file" não é viável porque todos os testes actuais
usam World real, ou que "clap" já é dep do workspace, ou que o
export PDF tem API não documentada que complica qualquer forma.

---

## Escopo

**Dentro**:
- `view`/`grep` em `04_wiring/`, `03_infra/`, `01_core/`.
- `view`/`grep` em `lab/typst-original/crates/typst-cli/`.
- Leitura do `Cargo.toml` workspace para descobrir deps autorizadas.
- Produção de ficheiros de diagnóstico.

**Fora**:
- Qualquer código de produção.
- ADR nova.
- Alteração de `04_wiring/main.rs`, Cargo.toml, ou outros ficheiros
  (excepto `00_nucleo/diagnosticos/`).
- Testes novos.

---

## Sub-passos

### 112.A — Inventário do `04_wiring/` actual

1. `view` em `04_wiring/src/`. Listar todos os ficheiros.
2. `view` em `04_wiring/src/main.rs`. Registar:
   - Linhas de código reais (excluindo header).
   - Se ainda é stub `println!` ou se já tem estrutura.
   - Que imports existem (se algum).
3. `view` em `04_wiring/Cargo.toml`. Registar:
   - Dependências declaradas.
   - Se há deps externas (clap, anyhow, etc.) já presentes.
4. `view` em `Cargo.toml` raiz (workspace). Registar:
   - `[workspace.dependencies]` — quais crates externas estão
     declaradas com versões.
   - Quais estão disponíveis mas não usadas ainda.

Escrever em `00_nucleo/diagnosticos/inventario-wiring-passo-112.md`:

```
04_wiring/src/main.rs:
  linhas: N
  estado: stub | parcial | completo
  imports: [...]

Cargo.toml de 04_wiring:
  deps: [...]

Cargo.toml workspace:
  deps disponíveis: [...]
  deps usadas por 04_wiring: [...]
```

### 112.B — Inventário da infra L3 consumível

1. Grep por `pub fn` em `03_infra/src/`. Listar APIs públicas.
2. Identificar especificamente:
   - Helper de eval (ex: `do_eval_with_sink` ou equivalente).
   - Função de layout (se L3 orquestra layout).
   - Export PDF (função e assinatura).
   - Carregamento de ficheiros / `World` real se existe.
   - `drain_diagnostics_to_stderr` (Passo 111) — confirmar API.
3. Para cada uma, registar:
   - Assinatura.
   - Visibilidade (`pub` vs `pub(crate)`).
   - Se é orientada a testes (usa paths hard-coded, MockWorld) ou
     a produção.

**Parte crítica**: verificar se existe `World` implementado em L3
capaz de ler ficheiros reais do filesystem. Se **não existe**, o
candidato "single file" passa a ser a única opção viável para a
CLI sem materializar mais L3 antes.

Escrever em `00_nucleo/diagnosticos/inventario-l3-apis-passo-112.md`.

### 112.C — Inventário do vanilla CLI

1. `view` em `lab/typst-original/crates/typst-cli/src/main.rs`
   (ou equivalente).
2. `view` em `lab/typst-original/crates/typst-cli/src/args.rs` (se
   existe) — argumentos e subcomandos.
3. Listar:
   - Subcomandos (`compile`, `watch`, `query`, `fonts`, `init`, ...).
   - Flags principais (`--root`, `--font-path`, `--input`, ...).
   - Comportamento default (se corre sem args, que acontece).
4. Para cada subcomando, **breve** descrição (1 linha). Não
   transcrever código.

O objectivo não é paridade completa. É ter visão do perímetro para
decidir quais são mínimos essenciais e quais são adiáveis.

Escrever em `00_nucleo/diagnosticos/vanilla-cli-perimetro-passo-112.md`.

### 112.D — Candidatos de escopo

Produzir 3-4 candidatos ranqueados. Para cada:

- **Nome** (ex: "Micro", "Mínimo com warnings", "Subset compile+query").
- **Subcomandos cobertos**.
- **Flags cobertas**.
- **APIs L1/L3 invocadas**.
- **Deps externas necessárias** (ex: clap, anyhow).
- **File loading**: single-file ou World real.
- **Tamanho estimado** (comparar com passos anteriores: 104 médio,
  109 grande, 111 médio).
- **O que o utilizador externo consegue fazer com esta CLI**.
- **O que fica por fora** (explicitamente).

Pelo menos 1 candidato deve qualificar como "passo ≤ 109". Se
nenhum qualifica, recomendar divisão em sub-passos (ex: "CLI micro
primeiro, depois adicionar flags").

Escrever em `00_nucleo/diagnosticos/candidatos-cli-passo-112.md`.

### 112.E — Recomendação

Relatório `typst-passo-112-relatorio.md` agregando:

- Resumo do estado actual de `04_wiring/` (3-5 linhas).
- Resumo do que L3 oferece para a CLI (3-5 linhas).
- Resumo do perímetro vanilla (3-5 linhas).
- Lista ranqueada de candidatos (de 112.D), com recomendação
  primária.
- Decisão sobre argparsing: manual / clap / argh, com razão factual.
- Decisão sobre file loading: single-file / World real, com razão.
- Avisos sobre o que pode dar errado em cada candidato.
- Ficheiros de diagnóstico produzidos.

O relatório **não toma decisão definitiva**. Serve como input para
a conversa onde o escopo é escolhido antes do Passo 113 de
construção.

---

## Critério de conclusão

Todas em conjunto:

1. Ficheiros de diagnóstico escritos em
   `00_nucleo/diagnosticos/`:
   - `inventario-wiring-passo-112.md`
   - `inventario-l3-apis-passo-112.md`
   - `vanilla-cli-perimetro-passo-112.md`
   - `candidatos-cli-passo-112.md`
2. Relatório `typst-passo-112-relatorio.md` escrito.
3. **Zero** código de produção alterado.
4. **Zero** ADRs novas.
5. **Zero** testes novos ou removidos.
6. `cargo test --workspace` com contagem inalterada (811 L1 + 189
   L3 + 6 ignorados).
7. `crystalline-lint` zero violations.
8. Pelo menos 3 candidatos de escopo documentados, com 1
   recomendação primária.

---

## O que pode sair errado

- **L3 não tem `World` real**. Se o único `World` é o `MockWorld`
  de testes, "World real em L3" não é opção — ou é passo separado
  primeiro. Documentar em 112.B.
- **Export PDF é instável ou depende de tipos em lab/**. Se a API
  PDF cristalina ainda passa por `lab/typst-original/`, o candidato
  "micro" pode não ser viável sem completar essa migração.
  Documentar.
- **Vanilla CLI é gigante e difícil de resumir**. Resistir a
  transcrever. Focar no que vai ao relatório: lista de
  subcomandos + 1 linha cada. Se algum é subtil, deixar nota em
  vez de detalhar.
- **Tentação de fazer algo pequeno "já que está aqui"**. Gate
  explícito: este passo **não constrói**. Se aparece correcção
  trivial (ex: remover `println!` do stub), **registar como
  candidato**, não executar.
- **Candidatos todos grandes**. Se nenhum candidato fica em
  tamanho ≤ Passo 109, isto é informação: CLI não se materializa
  pequena. Recomendação vira "dividir em sub-passos N passos" ou
  "adiar até L3 crescer mais".

---

## Notas operacionais

- Este passo não toca código. Se `cargo test` regride, algo foi
  tocado por engano.
- Ler vanilla CLI **superficialmente**. Não é projecto paralelo.
  Focar no perímetro, não no comportamento exacto.
- Se 112.B revelar que `do_eval_with_sink` e `drain_diagnostics_to_stderr`
  são `pub(crate)` ou privados a testes (em vez de `pub`), isto é
  bloqueio para CLI — registar. Pode significar que o Passo 113
  tem de tornar essas APIs públicas.
- O Cargo.toml workspace pode revelar deps disponíveis mas não
  usadas. `clap` em `[workspace.dependencies]` sem consumidor
  activo simplifica a decisão de argparsing.
- Se `lab/typst-original/crates/typst-cli/` for grande (provável),
  ler apenas `main.rs` e `args.rs` (ou equivalente). Não varrer
  recursivamente.
- O relatório 112.E faz **recomendação**, não decisão final.
  Conversa seguinte decide com base nele antes do Passo 113.
