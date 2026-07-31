# Passo 942 — resolver o mistério de P939: código "equivalente" mas ~3× mais lento em tempo de parede

**Precede este passo**: `typst-passo-939-relatorio.md` — comparou `Coverage::from_codepoints`/
`contains`/extração de `cmap` linha a linha com o vanilla, concluiu "equivalente", mas mediu
`layout_ms` do cristalino (~740-780ms) contra `scan system fonts` do vanilla (~256ms) para o
mesmo trabalho — ~3× de diferença nunca explicada. A duplicação de I/O foi testada e não
explicava (revertida). Com o `render_ms` já resolvido (P941), este é o último fator desconhecido
antes da distância final ao vanilla.

**Este passo faz o que P938/939 pediram mas não chegaram a fazer por completo: perfilar de
verdade, não só comparar tempos totais.**

**Pré-condição de árvore**: `git status`. Confirmar estado P941 presente.

---

## Fase A — verificar diferenças de build antes de perfilar código (o mais barato de descartar primeiro)

Isto nunca foi verificado nesta frente inteira — comparações de tempo sempre assumiram que os dois
binários eram compilados de forma equivalente, sem confirmar.

1. Comparar o perfil de release do cristalino (`Cargo.toml`, `[profile.release]`) com o do vanilla
   (`lab/typst-original/Cargo.toml`) — `opt-level`, `lto`, `codegen-units`, `panic`,
   `debug-assertions`, `overflow-checks`. Uma diferença aqui (por exemplo, `debug-assertions`
   ligado por engano, ou `codegen-units` mais alto reduzindo otimização entre módulos) pode
   sozinha explicar um factor de 2-3× em código CPU-bound, sem nenhuma diferença de algoritmo.
2. Confirmar o alocador usado em cada binário — se o vanilla usa um alocador diferente (`jemalloc`,
   `mimalloc`) e o cristalino usa o alocador padrão do sistema, isso pode importar bastante para
   uma carga de trabalho que aloca muitas estruturas pequenas repetidamente (construir `Coverage`
   para ~1086 fontes).
3. Confirmar se algum dos dois usa paralelismo (`rayon` ou equivalente) na fase de descoberta/
   extração de fontes que o outro não usa — isso mudaria a natureza da comparação por completo
   (não seria "mesmo algoritmo, tempo diferente", seria "um é sequencial, o outro paralelo").
4. Se alguma diferença for encontrada aqui: corrigir e remedir **antes** de ir para a Fase B — pode
   ser que isto sozinho feche a distância, tornando o resto do passo desnecessário.

## Fase B — perfilar de verdade, com `perf record`/`perf report` ou `flamegraph`

Só se a Fase A não explicar a distância inteira.

1. Rodar `perf record -g` (ou equivalente) no cristalino P941 compilando `utf8-cjk.typ`,
   isolando a fase de descoberta/extração de coverage de fontes (não o documento inteiro).
2. Gerar um flamegraph ou relatório de `perf report` mostrando onde o tempo de CPU é gasto,
   função a função, dentro dessa fase.
3. Repetir para o vanilla real, mesma fase, mesmo documento.
4. Comparar os dois perfis lado a lado — identificar a função ou grupo de funções que domina o
   tempo do cristalino e não tem equivalente no perfil do vanilla (ou tem, mas com peso muito
   menor). Isto é a resposta concreta que P939 não chegou a produzir.
5. Confirmar se a causa é: alocação (muitas pequenas alocações vs. poucas grandes), uma função
   específica sendo chamada mais vezes que o necessário, alguma conversão de tipo/cópia de dados
   desnecessária, ou outra coisa que só o perfil revela.

## Fase C — corrigir a causa encontrada

TDD directo ou protocolo de dois agentes, conforme a causa revelar.

1. Teste com medição real confirmando o problema antes da correção.
2. Implementar.
3. Suíte completa verde, discriminada por crate.

## Fase D — medição final

7 cenários canônicos (`depois/antes`, zero regressão) + 5 casos UTF-8 (`depois/antes` e
`cristalino/vanilla-real`), `--min-runs 10`+, vanilla confirmado por string distintiva.
`layout_ms` isolado, confirmando quanto da distância de ~4.4-4.7× foi fechada.

## Resultado esperado

- Causa exacta da diferença de ~3× confirmada por perfil real (flags de build, alocador, ou
  função específica) — não mais "código parece equivalente, tempo diverge, sem explicação".
- Correção implementada e medida.
- Distância final ao vanilla real, com número, não estimativa.
- Se ainda sobrar distância inexplicada depois deste passo: registar como limitação aceite, com o
  que já foi investigado e descartado, para não repetir a mesma investigação de novo no futuro.
