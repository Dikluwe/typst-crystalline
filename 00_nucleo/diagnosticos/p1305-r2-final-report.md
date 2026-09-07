# P1305 — módulos nomeados e arrays longos

A representação de módulos e arrays longos foi corrigida. O global continua acessível por `std`, mas apresenta o nome público `global`; arrays acima de 40 itens abreviam somente a representação, sem perder dados.

| Observável | Antes | Depois |
|---|---|---|
| `repr(std)` | `module(std)` | `<module global>` |
| `repr(color.map)` | `module(map)` | `<module map>` |
| Import ordinário de `std.typ` | `module(std)` | `<module std>`, sem confusão com o global |
| `repr(range(41))` | Representava os 41 itens | Representa 0–39 e acrescenta `.. (1 items omitted)` |

Os **12 caminhos selecionados** — `repr(std)`, `color.map` e os mapas cividis, coolwarm, crest, flare, icefire, mako, rainbow, rocket, turbo e vlag — passaram de `DIFFERENT_VALUE` para `MATCH_VALUE` nos quatro perfis. Foram exatamente 48 mudanças de resultado no corpus; não houve alteração de saída, diagnóstico ou exit code nas demais vias, nem no vanilla.

## Por que não bastava mudar o formatter

A hipótese repr-only foi refutada pelo import de um arquivo `std.typ` que reexporta `std: *`: nome e conteúdo do scope não identificam com segurança a origem do módulo. A ampliação autorizada resolveu isso na construção do objeto, nos caminhos de expressão, documento e arquivo importado. O formatter agora apenas projeta o nome armazenado.

O nome público também não pode substituir o nome lexical do import. `import std` continua ligando `std`; `import holder.saved` liga `saved`, sem introduzir uma variável `global`. Imports dinâmicos sem nome explícito recebem o erro/hint/span previstos; formas com `as`, items ou wildcard permanecem válidas.

A elisão ficou exclusivamente na projeção de `Value::Array`. Não foi aplicada ao helper genérico, nem às sequências de conteúdo, campos ou argumentos.

## Evidência que poderia refutar a correção

- RED independente: 38 testes executados, 22 falhas pelos defeitos previstos e 16 controles válidos. GREEN: os mesmos 38 passaram, sem adaptar expectativas à implementação.
- Fronteiras 0/1/39/40/41/42/81/256, nesting, pontuação, indentação, escaping e dados após o índice 40 foram exercitados; módulos ordinários, aliases e todas as rotas de construção também.
- Nos 15 mapas, a medição válida final teve 240 execuções completas e 120 repetições estáveis. Componentes completos, cardinalidade e digests RGBA8 permaneceram iguais ao P1304; a representação coincidiu com o vanilla.
- Os 627 probes foram repetidos nos quatro perfis, com inversão dos controles, divergências e amostra previstos. Sentinelas P1290/P1300/P1301/P1303 e gates PDF foram preservados.
- Os 27 mutantes válidos de código foram detectados semanticamente. A restauração da cópia temporária foi conferida e voltou a GREEN.
- Workspace: 6.577 testes passaram, zero falhas e três doctests ignorados. Build, formatação, checks arquiteturais estritos, dry-run de hashes e diff check passaram.

O lint geral não está livre de dívida: registrou 236 warnings e 1.133 mensagens informativas, sem erros. O conjunto e a multiplicidade das mensagens coincidiram com o registro P1304. Os checks V3/V4/V5/V13/V14/V15/V26 tiveram zero violações.

## Falhas da instrumentação, sem maquiar o resultado

A primeira medição auxiliar dos mapas reutilizou `float(x)`, que falha para ratio no cristalino. Seus 120 comandos cristalinos inválidos foram preservados e **não provaram igualdade de dados**. Recuperou-se a adaptação já calibrada no P1304, `x / 100%`, primeiro em dois mapas sobre o baseline pinado. Só após nova discriminação e selo foi feita a medição final dos 15 mapas. Não houve mudança do produto nem repetição do corpus completo por essa correção.

Na campanha de mutantes, duas tentativas aplicaram a alteração na rota expressão/documento trocada. Permanecem registradas como erros de aplicação; não foram contadas como detecções. Os ataques foram reaplicados nas rotas corretas e detectados. Nenhum erro de compilação contou como morte de mutante.

## Limites e proveniência

Continuam fora deste fechamento: anonimato de plugin, encoders JSON/TOML/YAML, membros ausentes e extensões intencionais. Também permanecem as dívidas já medidas de warnings de import sem efeito, forma de args/dict/conteúdo e diagnóstico nominal de campo ausente em um import ordinário chamado `std`. Não houve edição de `bindings/field_access` nesta execução; as alterações anteriores de P1303 foram preservadas. Permanecem 504 pares do corpus com diferenças históricas de disponibilidade: 168 `CRYSTALLINE_ONLY` e 336 `VANILLA_ONLY`. São observações por perfil, não 504 caminhos distintos. Não se afirma paridade geral da linguagem.

Todas as medições candidatas correspondem ao HEAD `8eb41b769eb840c7ab1063f981f98fdd4047952b` com working tree não commitado, diff SHA-256 `a6a964ac05ef32c26cca69e478af130ce3068a015182b034e1cc360bab5a8ae5`. A lista exata de arquivos e `git diff HEAD --stat` está no [recibo de implementação](p1305-r2-implementation-receipt.json); comandos, horários e canais completos estão nos [logs dos gates](p1305-r2-build.log) e nas medições pinadas pelo certificado. O binário candidato é `be51045f1df75ac42ee081801ddf7f738f709029e3694b9205473ba5fa8b31d4`; o vanilla ratificado é upstream/main `a51e02804`, binário `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

Houve um build release candidato em target exclusivo de RAM e uma execução do corpus completo, além da revisão auxiliar descrita acima. O passo original, as evidências anteriores e as entradas seladas permaneceram intactos. Não foi feito commit nesta etapa.

A skill Tekt separou autoria de contrato, testes, implementação e veredito; a medição inválida foi reaberta sem ajustar o produto ou os testes congelados. Regime: **executado sem atestação de isolamento técnico**.

O certificado aprova o fragmento exigido pelo passo (`P1305_PASS_REPR_MODULE_AND_LARGE_ARRAY`) sob o rótulo independente `P1305_R2_PASS_WITH_RECORDED_LIMITATIONS`. [Certificado](p1305-r2-certificate.json), SHA-256 `414479e20c93251488e1840f1298118fc35e3b4698ba73ae35b7e923c769dccc`. O certificado pina evidências e recibos; este relatório apenas referencia o certificado, sem ciclo de hashes.
