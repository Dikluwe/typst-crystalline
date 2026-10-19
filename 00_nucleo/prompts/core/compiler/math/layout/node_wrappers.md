# L0 — compiler/math/layout/node_wrappers
Hash do Código: 62f1a84d

## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), `compiler/math/layout/mod.rs:1070-1104,1119-1193` contém os dois braços não triviais de wrappers de nó. A evidência delimita a extração, sem medir desempenho.

## Owner e contrato

Camada L1. Consumer único: `01_core/src/compiler/math/layout/node_wrappers.rs`. O wrapper de classe preserva o avanço do glifo e a correção itálica; o wrapper de estilo resolve os eixos internos, aplica a transformação existente e dispõe o corpo resultante. O match central permanece exaustivo e estático. Preservar assinaturas públicas, defaults, fase do pipeline e pureza.

## Aceitação

Testes L1, build e crystalline-lint passam. Nenhum ganho de desempenho é inferido.


## Exaustividade nominal e origem matemática — 2026-10-17

### Medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; UTC, fontes/L0/hashes/configuração, status e git diff HEAD --stat exato em `lab/results/construction/diagnostics/tekt-v16-math-layout-2026-10-17/before.json.gz` (snapshot inicial preservado em initial-ten-owners.json.gz). AST/spans em contexts.json/all-contexts.json/wildcard-inventory.json, parser pinado do CI. A suíte anterior passou no mesmo código de entrada, revalidado por hashes.

source-probe-before.json mede limits inline com largura 7.2 pt versus 12.912 pt ao acrescentar Source. source-family-before.json preserva a primeira matriz de 12 pares (11 diferenças). source-family-supplement.json e sua análise preservam 23 pares com métricas atribuíveis (19 diferenças e 4 controles iguais): limites, kerns, Op.text, acentos/dotless, suportes, alinhamento de matriz, espaçamento, sequências, stretch e classe. source-lines-before.json mede a mesma equação com 2 linhas versus 1 e última baseline 16.2 versus 0 sob Source, apesar dos mesmos extents. São observáveis de geometria, identidade extraível e metadados da equação, não igualdade de bytes/pointers/Rust. A transparência já é obrigatória no núcleo content-source-origin; a inspeção mostra os helpers classificando o wrapper em vez do body. Não se infere intenção histórica ou paridade global a partir desses números. O erro de contagem do analisador complementar (24 em vez de 23 pares) foi corrigido usando o stdout original válido, sem refazer a medição nem contar o erro como RED funcional.

### Prescrição exclusiva

Manutenção somente do helper/teste ou do eixo MathSize descrito abaixo. Não ampliar travessia ou reescrever os cenários existentes deste owner.

Trocar os fallbacks enumerados pelo complemento nominal atual dos braços **incondicionais**, sem alterar corpo/guard/ordem; variantes guardadas continuam no complemento para quando a condição falha. A projeção Option de attach é a única exceção estrutural: None retorna por ?, Some passa ao match nominal de Content normalizado, sem padrão profundo.

- **L22** — `01_core/src/compiler/math/layout/node_wrappers.rs:102`, `layout_styled_node`, MathStyleKind: excluir somente `Display, Inline, Script, SScript`; guardados `(nenhum)`. Setters de glyph não alteram MathSize; somente Display/Inline/Script/SScript o fazem. O corpo vazio é o no-op desse eixo independente.

### Aceitação e limites

L0 antes de código. Correção interna/paridade ADR-0127 em fluxo contínuo: sem campo/assinatura pública, trait, dependência, flag/default novo, compatibilidade ou fase nova. Grupos V19 compartilham o corpo anterior explicitamente justificado por caso; as notas permanecem visíveis e não provam cobertura de execução por alternativa. Não adicionar allow/expect, elevar limiar, capturar @ ou esconder um match em if-let/matches!. Somente a transparência e as reestruturações rasas aqui nomeadas alteram o código funcional; não editar fórmulas/constantes ou relaxar expectativas anteriores. RED funcional das regressões Source e RED estrutural dos avisos/sondas antes do reparo. Após, regressões GREEN, zero V16 dos locais emitidos, revisão futura dos 36 locais (incluindo a nova projeção de Content sob Option) por sentinelas Content/Value/MathStyleKind/FrameItem em workspace descartável com cfg(test) compilado. Os scrutinees/funções atuais e a composição ganham só as alterações descritas; revalidar ast/replay, assinaturas e demais owners.

Gates: suíte workspace completa, build, fmt, Clippy workspace/all-targets -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito/reversa, SARIF diferencial completo e diff-check. V16 esperado 170→152; notas V19 novas somente dos grupos prescritos e do observador novo; V20 sem crescimento. As demais violações continuam pendentes. Fontes congeladas durante os gates; MemoryMax=12 GiB, swap zero, um job Cargo, comandos pesados sequenciais. Sem commit/push/release ou alteração do pin CI. Não alegar ganho de RAM, leitura de IA ou paridade integral por encerrar o lint.
