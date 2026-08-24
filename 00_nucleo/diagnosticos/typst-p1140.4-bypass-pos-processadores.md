# Diagnóstico P1140.4 — bypass de pós-processadores na pipeline

Data: 2026-08-23

## Proveniência

- Commit base: `ffd527c85dd7d547413d33cbc2d27a80e32a3f8c`.
- Estado medido: working tree não commitado; lista exata de ficheiros e
  magnitudes em `git diff HEAD --stat` deve acompanhar qualquer reprodução,
  pois a frente P1140.4-A ainda está em curso.
- Hora dos probes finais: 2026-08-23T23:08:28-03:00.
- Binário cristalino: `target/debug/typst`, reconstruído a partir desse estado.

## Erro encontrado

O callback de `math.equation(numbering: function)` funcionava nos testes do
fixpoint, mas não no PDF produzido pela CLI. A causa não estava no callback:
a pipeline paginada de produção chamava `introspect_with_introspector`, que
faz o walk estrutural, e nunca chamava os pós-processadores dependentes de
`Engine + EvalContext`.

Esta é uma classe de erro de **estágio implementado mas não alcançável pelo
entrypoint de produção**. Testes unitários do estágio dão GREEN e não provam
que ele pertence ao grafo real `CLI → eval → introspect → layout → export`.

## Inventário mecânico

A busca por `apply_*` chamados em `run_fixpoint` encontrou quatro estágios
dependentes de runtime:

1. `apply_state_funcs`;
2. `apply_state_displays`;
3. `apply_counter_displays`;
4. `apply_equation_numberings`.

Os quatro foram concentrados em `introspect_with_runtime`, agora chamado pela
pipeline paginada depois da expansão de `ContextBlock` e antes do layout. PDF,
PNG e SVG partilham essa pipeline. HTML continua fora deste fechamento porque
segue outro caminho e o seu contrato deve ser medido antes de integração.

## Provas ponta a ponta

- Equação: callback `n => "N" + str(n)` produziu `N1` tanto na equação como em
  `@eq-x`. Há teste de regressão em `03_infra/src/integration_tests.rs` usando
  `compile_to_pdf_bytes`, não o helper antigo que montava eval/layout à mão.
- State: `state_update("k", 1)`, seguido de
  `state_update_with("k", x => x + 4)` e
  `state_display("k", x => "S" + str(x))`, produziu `S5` no texto extraído.
  Isso prova em conjunto os dois pós-processadores de state.
- Counter callback: um heading seguido de
  `counter_display("heading", xs => "C" + str(xs.at(0)))` ainda produziu saída
  vazia. O estágio agora é alcançado, mas o probe indica uma divergência
  adicional, provavelmente de identidade da chave (`Str("heading")` versus
  selector de `ElementKind::Heading`). Não está fechado por P1140.4-A.

## Como encontrar a mesma classe no restante projeto

Aplicar três travas complementares:

1. **Inventário estático de alcançabilidade:** para cada função de fase
   (`apply_*`, `resolve_*`, `materialize_*`, `finalize_*`, `post_*`), registrar
   os callers e exigir ao menos um caminho vindo de cada entrypoint de
   produção relevante. Funções chamadas somente por `#[cfg(test)]`, fixtures
   ou orquestradores não usados em L3 são suspeitas.
2. **Sentinela semântica ponta a ponta:** usar callbacks que produzam tokens
   impossíveis de confundir, como `N1`, `S5` e `C1`, compilar pela API pública
   ou CLI e observar o artefato final. O teste deve falhar se o estágio for
   removido do pipeline, mesmo que seus testes unitários continuem verdes.
3. **Teste de composição por backend:** manter ao menos um caso para PDF e,
   quando o contrato existir, HTML; PNG/SVG podem compartilhar a prova da
   construção do `PagedDocument`, mas precisam de smoke tests que confirmem o
   entrypoint comum.

Uma regra mecânica útil para CI é gerar a lista de funções de fase com `rg` e
compará-la com callers fora de blocos de teste. Isso não prova semântica, mas
reduz o espaço de busca; a prova decisiva permanece o token sentinela no
artefato produzido pelo entrypoint real.

## Pendências

- abrir uma frente própria para o mismatch de chave de `counter_display`, com
  medição vanilla e L0 antes do código;
- auditar o caminho HTML separadamente;
- ampliar a sentinela para outras famílias encontradas por busca de nomes de
  fase, sem assumir que todo `apply_*` requer execução global.
