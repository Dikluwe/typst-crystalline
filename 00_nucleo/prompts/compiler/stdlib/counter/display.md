# L0 — exibição de counter
Hash do Código: 6be7e834

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/counter/display.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidência antes da decisão

`01_core/src/compiler/stdlib/counter.rs:666-845` reúne a política de `.display()` para localização, valores, `both`, numbering e callback. O wrapper nativo em `:304-315` preserva a entrada pública. Medição estrutural da árvore em `320c2149b`; não é comparação de desempenho ou inferência sobre intenção upstream.

## Contrato

Receber o mesmo `Counter`, argumentos, scopes, contexto, engine e span da fachada. Validar localização contextual e named args na ordem vigente; obter histórico filtrado pelo owner irmão; observar CounterAt/CounterTotal nos mesmos pontos; aplicar default numbering, pattern explícito ou callback na mesma ordem e produzir o mesmo Content ou diagnóstico. O wrapper e `counter_display` público permanecem na fachada sem mudança de assinatura. Não alterar defaults, casts ou fase de eval.

## Aceitação

Texto, conteúdo, spans, erros, avisos e ordem de chamadas iguais nos casos de padrão implícito/explicito, callback, `both` e counters filtrados. Build, testes focais e lint válidos.
