# Prompt — typst-passo-846: `syntax::span` — span de `#eval` diverge (#56) e call trace ausente (#57)

**Origem**: achados #56 e #57 de P831 (lote 5)
**Estado**: aguardando execução

---

## Achado #56 (S1) — span de erros dentro de `#eval` diverge mais do que o já documentado

Cristalino: ancora ao span da lista de argumentos (`args.span`, `stdlib/eval.rs:135`); vanilla: ancora ao span do literal string (`SpanMode::Uniform`, `foundations/mod.rs:267,318`). Exemplo medido: `span7.typ` — cristalino `3:5`, vanilla `4:2`. Isso já está **parcialmente documentado** no L0 `00_nucleo/prompts/engine/stdlib/eval.md` §3 (como "nuance de uma coluna", de P814) — mas a medição de P831 mostra uma divergência maior (diferença de linha, não só de coluna). Causa estrutural: `Args` no cristalino não guarda span por item individual (débito conhecido, P772s).

### Sonda
1. Reproduzir o caso exato `span7.typ` de P831 e confirmar a divergência de linha (não só coluna).
2. Revisar a nota existente no L0 `stdlib/eval.md` §3 — ela subestima a divergência real. Antes de corrigir, confirmar se dá para resolver sem o débito estrutural completo (span por argumento, P772s) ou se esse é de fato um pré-requisito.

### Implementação
Depende do que a sonda encontrar. Se for possível ancorar ao span do literal string sem precisar de span por argumento em geral (por exemplo, guardando o span do argumento posicional específico de `eval` só nesse ponto, sem generalizar para toda a stdlib), fazer isso. Se depender do débito estrutural maior, registrar como decisão de escopo — não implementar uma correção parcial que deixe a documentação do L0 ainda mais incorreta do que já está.

## Achado #57 (S2) — call trace ausente

Vanilla emite `while calling \`boom\` at ...` (rastro de chamadas até o ponto do erro); cristalino omite completamente. A estrutura já existe (`Tracepoint`, `entities/source_result.rs:22`) mas o campo `trace` nunca é populado. Vanilla: `typst-eval/src/call.rs:168`, `diag.rs:446`.

### Sonda
Reproduzir um erro que ocorre dentro de uma cadeia de chamadas de função (função A chama B chama C, erro em C) nos dois binários — confirmar o formato exato do trace do vanilla (quantos níveis mostra, como formata cada linha).

### Implementação
Popular `Tracepoint` durante a avaliação de chamadas de função, empilhando um nível por chamada, e propagar isso no relatório de erro no formato do vanilla.

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino) para cada achado. Contagem de testes de `typst-core` a bater com os testes novos.

## Validação (comum aos dois achados)
1. Recompilar. Repetir os casos de sonda, batendo com o vanilla (ou decisão de escopo formal registrada para #56, se for o caso).
2. Suíte completa, comando + contagem antes/depois.

## Relatório

`00_nucleo/diagnosticos/typst-passo-846-relatorio.md`, uma seção por achado (#56, #57). Para #56, atualizar explicitamente a nota do L0 `stdlib/eval.md` §3 que subestimava a divergência, independentemente do resultado (corrigido ou scope-out formal).
