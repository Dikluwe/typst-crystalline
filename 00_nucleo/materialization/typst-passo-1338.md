# Passo 1338 — diagnóstico de campo ausente em Array

## Recorte medido

A medição fresca `00_nucleo/diagnosticos/p1338-measurement.json`, SHA-256
`6bd65bffd487cf7103bbaa54e3655ff95467df557c852f6e5c7b44c96ea165e1`,
concluída em `2026-09-09T20:06:19.061817+00:00`, confirma que arrays vazios,
populados e aliases Unicode usam mensagem de campo inexistente e sublinhado
agregado no cristalino; vanilla ratificado a51e02804 usa
`cannot access fields on type array` somente no identificador.

Os acessos sintéticos len/first/last e o pré-despacho têm dívida própria:
preservá-los, inclusive a diferença entre lookup puro e AST antecipada.
Chamadas normais e wrappers estáticos continuam válidos. Valores-tipo e
ordem de argumentos são outros recortes. Não alegar paridade geral Array.

Baseline `p1338-baseline.json` SHA-256
`6d85aece9e323950a8f722b11e87eb125b4a342a7404f969f60ea69595790b3c`:
working tree não commitada sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, diff/stat e inventários exatos.
Nenhum passo histórico de materialization/context foi necessário à leitura.

## Execução autorizada

1. Atualizar primeiro o único L0 proprietário
   `00_nucleo/prompts/compiler/eval/bindings/field_access.md`.
2. Congelar baseline, L0, papéis e limites. Regime A/B, sem contrato de
   refinamento nem atestação técnica de isolamento. Reutilizar os três autores
   anteriores por limite total de agentes, declarando seu contexto herdado.
3. Autor de testes independente produz testes puro/AST, positivos e fronteiras
   derivados do L0; migra somente a sentinela Array do P1337 expressamente
   sucedida. Formatar antes do freeze. Não entregar novos testes ao implementador.
4. Adversário congela quatro famílias e perfil pareado antes de C. Revisor
   verifica o poder discriminatório e o RED real de assertions, antes do patch.
5. Implementar somente mensagem Array ausente e âncora AST no consumer
   `01_core/src/compiler/eval/bindings/field_access.rs`. Sem novos membros.
6. GREEN focal; build/suíte workspace release --locked; A/B CLI independente
   nos quatro perfis e três ordens, com dívida pré-classificada sem crédito.
7. Controle C e quatro mutantes reais em cópia exclusiva; comprovar recompilação,
   assertion específica e hash do executável retido por rodada. Sem falso kill
   por compile failure/cache. Perfil instrumental não substitui release normal.
8. fmt/diff-check, lint geral, V5/V15/V26 estritos e A/B recíprocos. Relatório
   substantivo em diagnósticos e veredito separado; closure só após aprovação.

Correção interna de paridade, fluxo contínuo ADR-0127; assinatura, entidade,
default, compatibilidade e fase não mudam. Se o recorte não bastar, reabrir
medição e classificação antes de ampliar código. Preservar todos os owners
alheios, arquivos herdados, evidências e temporários anteriores.

Budget: quatro famílias, 30s por CLI, 2700s por Cargo, até quatro workers CLI;
dois ajustes sem ganho na mesma causa exigem rever o método. Unknown obrigatório
bloqueia. Não repetir corpus geral enquanto houver falha focal.
Target exclusivo `/tmp/p1338-target.vlNAmp`, cache copiado sem hardlinks;
adversário usa workspace/target próprios. Sem commit, staging, push ou limpeza.
Não atualizar inventário global de paridade nem sincronizar lab.

## Execução

L0, testes e implementação realizados. A referência documental herdada ao
L0 de l-value foi corrigida por manifesto sucessor R1 antes de C, mantendo
o recorte Array inalterado. Resultados, proveniência e limites em
`00_nucleo/diagnosticos/p1338-final-report.md`; veredito e fechamento separados
em `p1338-review-final.json` e `p1338-closure.json` na pasta de diagnósticos.
Sem commit neste pedido.
