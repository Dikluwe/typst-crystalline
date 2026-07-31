# Passo 939 — perfilar a distância residual de 4-6× ao vanilla, e investigar o flake de testes paralelos (ligação a P924)

**Precede este passo**: `typst-passo-938-relatorio.md` — caso comum recuperado, ganho de fallback
preservado, duplicação de I/O identificada (explica só a diferença P937→P938, não a diferença
maior ao vanilla). E `typst-passo-924-relatorio.md` — flake nunca reproduzido, candidato de causa
já registado (`XDG_DATA_HOME` mutável globalmente, `world.rs:982-993`).

**Duas investigações independentes, ambas pendentes de passos anteriores.**

**Pré-condição de árvore**: `git status`. Confirmar estado P938 presente.

---

## Parte A — perfilar a distância residual de 4.4×-6.1× ao vanilla (a investigação que P938 pulou)

Isto é o ponto 0 já pedido em P938 e não executado — repetir aqui, obrigatório desta vez, antes de
qualquer implementação nova.

1. Usar `perf record`/`perf report` (ou instrumentação manual com `std::time::Instant`, revertida
   depois) no cristalino P938 e no vanilla real, processando o **mesmo** conjunto de fontes no
   caminho de fallback (documento CJK ou emoji), medindo tempo de CPU gasto em cada etapa: parse
   inicial de metadados por fonte, construção de `Coverage::from_codepoints` por fonte, qualquer
   outro trabalho por fonte no caminho quente.
2. Confirmar se a descoberta inicial do cristalino (`fontdb.rs`, chamada a `fontdb::Database::
   load_system_fonts()`) usa parse parcial (`RawFace`) como o vanilla, ou se already faz mais
   trabalho que o necessário só para indexar — ler o código real, não presumir a partir de P925.
3. Comparar a implementação de `Coverage::from_codepoints`/iteração da `cmap` do cristalino,
   linha a linha, com a do vanilla (`typst-library/src/text/font/info.rs:117-156`, já citada em
   P936) — confirmar se são de facto equivalentes ou se há diferença de algoritmo/alocação que
   ainda não foi vista.
4. Resolver a duplicação de I/O identificada em P938 (`FontSlot` reabrindo o que `fontdb` já tinha
   mapeado) — partilhar o mmap entre a fase de descoberta e a extração lazy de coverage, em vez de
   `FontSlot` criar o seu próprio mmap independente.
5. Medir de novo, com as correções desta parte, contra o vanilla real — confirmar quanto da
   distância de 4.4×-6.1× foi de facto explicada e fechada, e quanto (se sobrar) continua sem
   causa conhecida.

## Parte B — investigar o flake de testes paralelos (ligação a P924)

1. Reproduzir o flake relatado em P938 (`p858_*`, `p534_*`, `p838_*` falhando em conjunto, passando
   isolados) — rodar `cargo test -p typst-infra --lib` repetidamente (20+ vezes) com paralelismo
   default, tentando capturar a falha de novo, com a mensagem de erro completa desta vez (P924 não
   conseguiu reproduzir; agora há um relato recente e mais específico para perseguir).
2. Testar directamente o candidato já registado por P924: `XDG_DATA_HOME` mutável globalmente em
   `system_world_include_source_resolve_de_package` (`world.rs:982-993`) — confirmar se algum dos
   quatro testes que falharam usa essa variável, ou instancia `SystemWorld`/chama
   `package_candidate_dirs()` durante a janela em que outro teste a está a mudar.
3. Se confirmado: corrigir (serializar o teste ofensor, ou isolar a variável de ambiente sem
   mutação global — mesmas opções já registadas por P924).
4. Se não for essa a causa: procurar outro estado partilhado introduzido por esta frente
   especificamente (`coverage_cache`, `OnceLock` em `FontSlot`) — são mudanças novas desde P924,
   candidatos que não existiam quando aquele passo investigou.
5. Se não reproduzir de novo: registar como P924 já fez, sem forçar correção sem causa confirmada.

## Resultado esperado

- Parte A: causa exacta da distância residual ao vanilla, com números de perfil, não suposição.
  Duplicação de I/O corrigida. Distância final medida contra vanilla real.
- Parte B: flake reproduzido e corrigido, ou não reproduzido e registado com mais detalhe que
  P924 (nome exacto do teste, mensagem de erro, condições).
- Attestation completa (`L11`) para ambas as partes.
