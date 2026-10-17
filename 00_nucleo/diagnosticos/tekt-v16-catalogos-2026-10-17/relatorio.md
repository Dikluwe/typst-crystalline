# Relatório — V16 dos catálogos

Etapa concluída: **quatro V16 corrigidos, de 124 para 120**, sem mudança de API, default, fase ou comportamento pretendido. Foram alterados somente dois consumers e seus dois L0 desde o checkpoint de igualdade Styled. Não houve commit/push do Typst, merge ou release.

## Alteração e equilíbrio com V19

`type_fields.rs`: constantes e funções passam por uma única classificação exaustiva de Type. Nove famílias mantêm seus catálogos; as outras 29 ficam explicitamente fora deste catálogo, permitindo a continuação no caller. Um nome de campo continua sendo uma string aberta: desconhecido retorna None.

`collections.rs`: classificação exaustiva de Value precede o lookup de método. Array/Dict/Str/Bytes/Args preservam seus caminhos e as outras 34 variantes continuam para o caller. Bytes conserva integralmente a delegação e seus metadados de span. A tabela estática classifica as cinco famílias Type de coleção; as outras 33 têm ausência local de catálogo. Some(Err) de método reconhecido não vira None.

Os três grupos negativos são coesos por responsabilidade local e estão prescritos nos L0. Não significam ausência universal de membros na linguagem. Não houve supressão de V16/V19 ou mudança de política do linter. Os nomes e corpos de 47 entradas do primeiro owner, 71 da tabela estática e 71 do dispatcher de instância foram conferidos por tokens, preservando strings, callbacks e ABI. Essa prova estrutural complementa os observáveis; não substitui paridade de linguagem. As alternativas negativas não têm todas sondas funcionais isoladas; a coesão foi auditada e a exaustividade foi verificada pelo compilador. As notas V19 permanecem visíveis.

## Evidência e validação

O inventário mede 118 entradas: 115 exercitadas no CLI e três Location no módulo. **159 pares bilaterais foram repetidos após a mudança**, preservando exatamente código de saída, stdout e stderr completos dentro de cada backend. Entre backends, 141 pares coincidem no critério código/stdout/erro principal+hints. Permanecem **18 divergências anteriores de diagnóstico**, detalhadas em observables-result.json e pendencias.json: aspas/nomes de tipos, mensagem de cast e bounds e hint de self. Não foram escondidas nem declaradas corrigidas.

O RED foi estrutural: os quatro padrões antigos compilavam mesmo com variante futura. Depois, os três padrões produtivos novos compilam para os enums atuais e rejeitam a variante futura com E0004. As sondas extraem os padrões reais, apagam somente RHS e usam payloads stub; não modificam o workspace/target Cargo. Os quatro controles funcionais passam antes e depois. A primeira versão de um controle tinha uma premissa inválida: Func::PartialEq compara nomes, não callbacks; o teste foi corrigido para repr/ABI/fn_addr_eq antes da implementação. Essa falha não é alegada como RED do produto. O verificador auxiliar também foi corrigido para ler padrões que começam por string e tolerar vírgulas finais introduzidas pelo rustfmt; não houve mudança produtiva por causa disso. As tentativas estão em tooling-corrections.json.

**Suíte completa: 8820 aprovados, zero falhas e 7 ignorados.** Build workspace, fmt, Clippy all-targets com -D warnings e -D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices HTML, V1/V5/V15/V26 estrito, reverse lineage e diff check passaram. Linhagem foi resselada somente nos dois owners após validar V15/V26.

SARIF: **{'V16:error': 120, 'V19:note': 1072, 'V20:note': 865}**. Nenhum novo erro/warning; todos os diagnósticos fora dos dois consumers são idênticos ao checkpoint anterior. O gate global continua exit 1 pelos 120 V16 restantes. [Inventário atualizado](inventario-v16-restante.md).

## RAM e proveniência

Todos os comandos pesados foram sequenciais sob **MemoryMax 12 GiB, MemoryHigh 10 GiB, swap zero e Cargo jobs=1**. Pico: **10193424384 bytes (9.493 GiB)**; eventos agregados: {'high': 0, 'max': 0, 'oom': 0, 'oom_kill': 0}. Sem OOM/OOM-kill. O valor é memória do cgroup, incluindo cache; não é RSS nem prova de otimização.

HEAD **06848cdcb5450f2c555626bf116bc7acd82bd82b**, working tree não commitado. before/final-state.json.gz registram UTC, status/diffstat e hashes de fontes/L0/config. Logs de comandos, recursos, corpus antes/depois e [patch restrito](patch.diff) permitem reprodução. As alterações anteriores, inclusive TS1 e igualdade Styled, foram preservadas por hash. Manifestos TS1 e RG1 conferidos integralmente; RG1 permanece pendente e não foi materializado.

Oracle: pin **9f2b6e8715237cb086899a42873660fe744622e8**, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. Linter pin **12797073972e375d9ab078cf6750c8b30b71e2b9**, binário SHA-256 e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2. A classe foi correção estrutural interna em fluxo contínuo ADR-0127: L0, testes, implementação e selos.

## Continuidade

O objetivo global continua ativo. [Próximo lote triado](next-batch-triage.json): quatro casos remanescentes de figure_image/casting, medindo fallbacks e contratos antes de decidir. As 18 divergências de diagnóstico ficam em lote próprio. Permanecem também RG1 pendente, Paint textual, seletor show text, defaults de Heading substituído por função, scripts, presença custom-only fora da projeção de igualdade, Box1fr, SVG multipágina e literal inteiro mínimo. Este lote não certifica toda a linguagem.
