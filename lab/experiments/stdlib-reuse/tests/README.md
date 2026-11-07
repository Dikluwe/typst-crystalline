# Controles independentes

`reuse_observables.rs` usa somente APIs públicas reais de `typst_core`,
`comemo` e `ecow`, além de std. Compile o mesmo arquivo contra baseline e
candidato. Nenhum teste depende do patch do implementador. Há 17 controles
públicos, dois controles do portal sob `--cfg stdlib_reuse_candidate` e uma
microbancada ignorada. A contagem final é a lista do binário de testes.

O portal diagnóstico especificado é `compiler::eval::__lab_resolve_stdlib`.
Ele existe apenas na cópia RAM, não é contrato proposto para o produto. Os
controles cfg isolam mudanças de features dentro do helper, situação que não
é um caminho normal da CLI. Os controles públicos intercalam roots/Worlds e
features/targets, preservando as fronteiras da linguagem.

A microbancada `measure_eval_with_preparsed_sources` usa Sources já parseadas,
World sem logs por consulta, inputs estáveis, reset de EvalContext em cada
entrada pública e descarte do resultado por iteração. O relógio inclui eval,
realização pura, resultado e verificações pequenas; não inclui parse, fontes,
layout ou exportação. `TEKT_STDLIB_ITERATIONS` controla as iterações (default 4).
Execute com `--ignored --exact measure_eval_with_preparsed_sources --nocapture`
em release, separado de alocações/profilers/builds. Contagem de chamadas World
é checada fora do relógio; não equivale a contagem de chamadas do builder.

Copie `fixtures/` inteiro para RAM antes de executar a CLI/watch. `imports_NN`
é fan-out de N arquivos; `repeated_NN` repete o mesmo arquivo N vezes;
`nested_NN` é cadeia de N arquivos. Todos mostram `Total N` e `Epoch 0`.
A primeira alteração de main é `Epoch 0` → `Epoch 1` sem mudar Total. Alterar
`modules/m00.typ` de answer 1 para 2 eleva Total em 1 no fan-out. Alterar
`modules/shared.typ` de 1 para 2 muda Total para 2N no repeated. Alterar a
folha `nestedNN/n(N-1).typ` de 1 para 2 eleva Total em 1 no nested. Regravar
os mesmos bytes deve publicar os mesmos valores. O benchmark precisa copiar
uma árvore por processo, para que as alterações não contaminem outro lado.

`inputs.typ` deve ser usado com pares `--input` explícitos. Compare nomes,
ordem e valores de sys.inputs; uma string de representação não substitui os
controles estruturais no teste Rust. `shadowing.typ` imprime as escolhas
root-local/child-local. Não se exige PDF binariamente idêntico: compare texto,
semântica/diagnósticos e render quando aplicável.
