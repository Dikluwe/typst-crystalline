# Relatório P924 — flake isolado em `typst-infra` (742/1 failed registado em P920)

**Precede este passo:** `typst-passo-920-relatorio.md`, "Achados em aberto" — uma execução de
`cargo test --workspace` mostrou `typst-infra: 742 passed, 1 failed`; três execuções seguintes
deram `743/0 failed`. Suspeita registada: testes sensíveis a descoberta de fontes do sistema.

**Commit base:** `c9df6fdee` (P921/P922/P923 integrados; working tree com alterações destes passos
ainda não commitadas).

**Data:** 2026-07-28.

---

## Fase A — tentar reproduzir

### A.1 — 12 execuções sequenciais (`--test-threads=1`)

Comando: `cargo test -p typst-infra -- --test-threads=1`

| Run | Resultado | Tempo |
|-----|-----------|-------|
| 1 | 743 passed; 0 failed | 31.82s |
| 2 | 743 passed; 0 failed | 31.74s |
| 3 | 743 passed; 0 failed | 31.55s |
| 4 | 743 passed; 0 failed | 31.71s |
| 5 | 743 passed; 0 failed | 31.40s |
| 6 | 743 passed; 0 failed | 31.27s |
| 7 | 743 passed; 0 failed | 31.49s |
| 8 | 743 passed; 0 failed | 31.48s |
| 9 | 743 passed; 0 failed | 31.57s |
| 10 | 743 passed; 0 failed | 31.60s |
| 11 | 743 passed; 0 failed | 31.71s |
| 12 | 743 passed; 0 failed | 31.46s |

**Subtotal:** 12/12 verdes.

### A.2 — execuções com paralelismo default

Comando: `cargo test -p typst-infra`

| Run | Resultado | Tempo |
|-----|-----------|-------|
| 1 | 743 passed; 0 failed | 17.76s |
| 2 | 743 passed; 0 failed | 18.58s |
| 3 | 743 passed; 0 failed | 17.65s |
| 4 | 743 passed; 0 failed | 17.63s |
| 5 | 743 passed; 0 failed | 17.57s |
| 6 | 743 passed; 0 failed | 17.88s |
| 7 | 743 passed; 0 failed | 17.61s |
| 8 | 743 passed; 0 failed | 17.54s |
| 9 | 743 passed; 0 failed | 17.45s |
| 10 | 743 passed; 0 failed | 17.59s |

**Subtotal:** 10/10 verdes.

**Total Fase A:** 22 execuções consecutivas de `typst-infra`, todas com `743 passed; 0 failed`.
O flake **não se reproduziu** no ambiente actual.

---

## Fase A.1 — candidatos identificados (sem reprodução)

Embora o flake não tenha voltado a ocorrer, a varredura dos testes de `03_infra/src`
revelou os pontos de contacto com estado externo / não-determinismo:

1. **Descoberta de fontes do sistema (`fontdb`)**
   - `03_infra/src/fontdb.rs:104` — `load_system_fonts_nao_panic()`.
   - `03_infra/src/world.rs:806` — `system_world_with_system_fonts_nao_panic()`.
   - `03_infra/src/font_metrics.rs:2122,2160,2217,2610` e
     `03_infra/src/shaper.rs:1456,1554` — testes que constroem `SystemWorld` com
     `with_system_fonts()` e fazem `return` precoce se o book ficar vazio.
   - `03_infra/src/integration_tests.rs:2921` — `discover_any_system_fonts()` faz
     skip explícito quando não encontra directórios canónicos de fonts.

   A maioria destes testes já tem guardas de skip; `load_system_fonts_nao_panic`
   e `system_world_with_system_fonts_nao_panic` são os únicos que **não** fazem
   skip — limitam-se a verificar o invariante `book.len() <= slots.len()`. Estes
   testes correm sempre que `fontdb::Database::load_system_fonts()` consegue
   terminar sem panic; não deveriam produzir `1 failed` isolado a menos que a
   própria descoberta de fontes devolva estado inconsistente (improvável mas não
   impossível com `fontdb` a iterar ficheiros do sistema).

2. **Manipulação global de `XDG_DATA_HOME` (`world.rs`)**
   - `03_infra/src/world.rs:982-993` — `system_world_include_source_resolve_de_package`
     faz `std::env::set_var("XDG_DATA_HOME", dir.path())` durante o teste e
     restaura no fim.
   - `package_candidate_dirs()` (`world.rs:66-69`) lê `XDG_DATA_HOME` em runtime.

   Este é o **candidato mais forte a flakiness por corrida**: se outro teste
   paralelo instanciar `SystemWorld` (ou chamar `package_candidate_dirs`) durante
   a janela em que `XDG_DATA_HOME` aponta para um temp dir, pode ver caminhos
   errados. A falha original em P920 aconteceu durante `cargo test --workspace`,
   que corre múltiplos crates em paralelo — embora os testes dentro de cada crate
   também corram em paralelo, a contenção com outros crates aumenta a superfície
   de corrida em variáveis de ambiente de processo.

3. **Ficheiros temporários com nomes baseados em `SystemTime::now()`**
   - Vários testes criam dirs em `/tmp` com timestamp de nanosegundos. A
     probabilidade de colisão é baixa, mas não nula; nenhum deles é crítico
     porque cada teste escreve no seu próprio dir.

**Conclusão da Fase A.1:** sem reprodução, não é possível confirmar a causa. O
 candidato mecânico mais provável permanece o estado partilhado via variável de
 ambiente (`XDG_DATA_HOME`), seguido da descoberta de fontes do sistema.

---

## Fase B — não executada

O flake não foi reproduzido; não houve causa confirmada a corrigir.

---

## Resultado final

| Item | Estado |
|---|---|
| Reproduzir flake em `typst-infra` | ❌ Não reproduzido (22 execuções consecutivas, 743/0) |
| Identificar teste exacto que falhou em P920 | ❌ Informação não disponível no relatório P920 |
| Confirmar causa (fontes do sistema vs outro estado partilhado) | ❌ Não confirmada |
| Candidato mais provável registado | ✅ `XDG_DATA_HOME` mutável em `world.rs:982-993` |
| Código alterado | ❌ Nenhum |

### Achado em aberto

- **Flake isolado em `typst-infra`** permanece em aberto. A próxima ocorrência
  deve capturar: (a) o nome exacto do teste que falhou, (b) a mensagem de erro
  completa, e (c) o comando exacto e flags (`--workspace`, `--test-threads`,
  etc.) usados na execução.
- **Recomendação futura:** se voltar a aparecer, correr imediatamente o teste
  isolado (`cargo test -p typst-infra <nome_do_teste>`) e, se passar, investigar
  primeiro o candidato `XDG_DATA_HOME` (serializar o teste ofensor ou isolar a
  variável de ambiente sem mutação global).

### Proveniência

- Commit base: `c9df6fdee`.
- Working tree: alterações não commitadas de P921/P922/P923 presentes durante as
  medições.
- Logs brutos das 22 execuções em:
  - `/home/dikluwe/.kimi-code/sessions/wd_typst-crystalline_f3e78e7af03c/session_ce334fab-1800-4e10-ab38-ac24a3a6a8dc/agents/main/tasks/bash-box4fcnf/output.log`
  - `/home/dikluwe/.kimi-code/sessions/wd_typst-crystalline_f3e78e7af03c/session_ce334fab-1800-4e10-ab38-ac24a3a6a8dc/agents/main/tasks/bash-r8264h3m/output.log`
