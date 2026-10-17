# Relatório — projeções de Content e campos de equação

Etapa concluída. V16 **115 → 109**, V19 **1088 → 1094 notas**, V20 **873 → 873 notas**. Além dos seis complementos nominais, foi corrigida uma perda real de campos de Equation quando um carrier interno aparece dentro de Styled.

## Correção

styled_equation reconhece agora Source, Located e ShowRecipe pelo body e Realized pelo visual. As leituras de numbering, supplement, alt, number-align, body e block deixam de depender da posição do wrapper. Não executa receitas nem consulta original de Realized. O getter mantém sua precedência de estilos, incluindo fallback ao nível interno quando o externo omite a chave; None explícito continua diferente de ausência. Styled de outro elemento não ganha campos de equação.

Os seis V16 pertenciam a is_empty, styled_equation, get_field, morph_canon, Hash::hash e source_span em cinco consumers/L0. Os complementos têm respectivamente 63, 114, 94, 112, 114 e 117 variantes. Seus valores/delegações anteriores foram preservados; Styled com guard recusado permanece no complemento. String de campo continua domínio aberto. O helper causal source_body não mudou. Não houve campo, variante, assinatura pública, default ou fase nova.

O núcleo content/projection-exhaustiveness.toml está pinado nos cinco owners; source-origin conserva seu pin e obrigação anteriores. L0 foi salvo antes dos testes/código. As seis notas V19 mantêm grupos uniformes explícitos, com justificativa por owner e cobertura de cada variante no modelo. Não há supressão, registry ou despacho dinâmico novo. Não se alega ganho medido de leitura por IA.

## Provas e validação

- Medição real na rlib: **48 diferenças em 84 controles → zero em 84**. Programa, fonte, argv, UTC, hashes e saídas em before/after-field-probe.json e field-control.rs. Quatro carriers, três posições e sete nomes de campo distinguem override, body/block, None explícito e ausência. A rlib medida permaneceu idêntica até o fechamento.
- RED funcional: os dois testes novos falharam antes da implementação; reproduziram a matriz e a precedência. GREEN preserva também os negativos não-equação e Realized com original Equation/visual texto.
- Proteção estrutural: uma variante futura de Content no modelo descartável passou de zero para **seis E0004**, um por local. **17280 controles simbólicos** exercitam 120 variantes, 12 campos, dois estados de guard e seis decisões; todas as variantes de cada complemento são exercitadas. Apenas os 96 casos simbólicos do helper para os quatro carriers mudam de classe, conforme a correção real. O modelo usa payloads opacos/guard booleano: não é prova de semântica de payload nem comparação vanilla. Replay textual e testes reais completam essa limitação.
- Suíte workspace: **8923 aprovados, 0 falhas, 7 ignorados**. Passaram build, fmt, Clippy all-targets com -D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito/quatro índices, V1/V5/V15/V26, reverse lineage, replay e diff --check. O diferencial confirma somente as seis remoções V16 e seis notas V19 prescritas, sem V20 novo ou severidade relaxada.
- Recursos sequenciais: MemoryMax12GiB/High10GiB/swap0/Cargo jobs1. Pico **10.003464 GiB** (10741137408 bytes); eventos {'low': 0, 'high': 232, 'max': 0, 'oom': 0, 'oom_kill': 0, 'oom_group_kill': 0}. Sem OOM/OOM kill.

## Proveniência e tentativas

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; baseline `2026-10-17T18:57:24.990755+00:00`, fechamento `2026-10-17T19:17:03.054681+00:00`. before.json.gz registra textos/L0/status/diff HEAD --stat e os 2129 hashes conferidos da etapa ssty; final-state.json.gz congela 2130 arquivos e binários. validation-result.json preserva argv/UTC/source freeze e recursos. Content e todos os arquivos protegidos fora dos cinco pares permaneceram inalterados. Linter 12797073972e375d9ab078cf6750c8b30b71e2b9, SHA-256 e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2.

O contrato já ratificado exige transparência de carriers internos: a prova é metamórfica, contra a mesma equação sem wrapper. Não há syntax vanilla para esses carriers e não se inventa equivalência de representação Rust. O alvo de linguagem permanece o commit 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.

A primeira preparação escalada não foi executada porque a revisão automática de permissão expirou. Foi confirmada a ausência dos destinos e a preparação local mais execução mínima limitada funcionaram. O scanner inicial omitiu quatro variantes struct por whitespace; foi corrigido para 120 antes de escrever L0/código. Uma importação Python ausente interrompeu a montagem da sonda antes do rustc; não foi contada como RED funcional. Registros preservados em probe-assembly-attempt.

O gate global ainda tem 109 V16; inventario-v16-restante.md lista cada local. TS1 aprovado e RG1 pendente são conferidos pela entrega. As frentes restantes continuam em follow-ups.md. Objetivo global ativo; sem commit/push/merge/release.
