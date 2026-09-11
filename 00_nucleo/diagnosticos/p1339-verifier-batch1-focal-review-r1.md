# P1339 — lote focal 1 rejeitado; preparação do último lote

Verificador `/root/p1311_review`, 2026-09-10. Executado sem atestação de
isolamento. Auditoria do raw real e reexecução somente do predicado sobre
esse raw; nenhum novo processo Typst ou teste produtivo. Não alterei inputs.

## Evidência e custo preservados

- Manifesto operacional: `86742a540fc58ac9e4096aadf32541f520a7953af3e209df490f48f69cee3d08`.
- `p1339-ab-batch1-focal-runs.json`:
  `471b753ba19921b4d90e0393452ac251b5875dfc833b362d8d7146bfa171c7c8`.
- `p1339-ab-batch1-focal-evaluation.json`:
  `b129f6861852ff9c76c444a27ea3f2d5d751e4b4c0092b62e361c38841125067`.

Todos os caminhos acima estão em `00_nucleo/diagnosticos/`. O raw registra
HEAD, working tree/diff exatos e UTC 04:50:00.373853–04:50:13.192882+00:00.
Foram 384 processos: 372 Observed e 12 Unknown timeout vanilla designados;
12.819637848006096 segundos wall e 49.92340499785496 segundos-filho somados.
A receita adicional consumiu 0.10872717201709747 segundos e falhou. Não há
expected derivado. Esses custos/falhas não são descartados como transporte.

Reexecutei public-predicate-v2 sobre o raw imutável, sem executar compilador:
as 384 classificações coincidem integralmente com o recibo autoral:
324 Preserved, 48 Violated, 12 Unknown. **Isso não é PASS focal.**

## Causa 1 — efeito de show confundido com texto da fonte

As 48 violações são as 12 células baseline de cada static-miss Strong/Emph/
Text e text-direct. O tuple bruto coincide com sua referência designada;
falha a classificação adicional de efeito. O código procura o marcador em
todo stderr, que inclui a linha-fonte com `panic("P1339_SHOW_WITNESS_...")`.
Isso não significa que o callback executou.

Há também 36 falsos efeitos CallbackExecuted nas três famílias static-match
baseline, embora classificados Preserved pelo predicado atual. Em todas
essas células a mensagem primária real é:

`error: type function does not contain field "where"`

Logo, corrigir apenas as 48 violações seria insuficiente. Os seis casos
static match/miss baseline em C falham antes do callback. O candidato F
continua sujeito à execução/miss real exigidos pelo L0/vanilla; text-direct
preserva o erro legado baseline. O marcador precisa constar da mensagem
primária exata do panic, não de frame, source line ou trace. Nenhum byte
diagnóstico é removido ou normalizado na comparação integral.

## Causa 2 — receita presumiu frame que não existe

Nas 12 combinações perfil/ordem, bare-isolated e padded-control baseline
têm exatamente o mesmo tuple completo:

`exit=1, stdout="", stderr="error: only element functions can be used as selectors\n\n"`.

Não existe frame, localização ou linha-fonte nesse diagnóstico. A receita
original exigiu exatamente uma localização e falhou com `positions=[]`.
Essa falha fechada foi correta; não se deve fabricar span nem editar o raw.

## Julgamento de budget e hipótese do lote 2

Aceito **preparação** do segundo e último lote agregado do suplemento
`4bdd984d…`, limitada às duas causas já integrantes da lista fechada:

1. Classificar efeitos pela mensagem primária real, com distinção explícita
   produto/fase para todos os seis static controls e text-direct. Conservar
   as comparações integrais e controles de execução positiva/miss.
2. Receita sucessora com ramo spanless estrito: bare/padded devem ter tuples
   integrais exatamente iguais ao controle conhecido em todas as células,
   sem qualquer frame/contexto/linha-fonte, com zero substituição. Manter
   fontes originais, padding/offset de entrada e todas as conjunções vanilla
   where/heading. O ramo com span continua separado e estrito; formas
   inesperadas, ambíguas ou instáveis falham, sem regex de normalização.

São hipóteses causais novas sobre tradução do observável, não nova intenção
R4 nem relaxamento de obrigação. A opacidade assimétrica agora foi medida
corretamente e deve ser preservada como controle, não usada para justificar
calibração ilimitada. Os 36 efeitos falsos ficam registrados sem reescrever
as classificações históricas.

Antes de executar lote 2, publicar sucessores imutáveis dos artefatos
afetados, ledger de delta/custo e manifesto agregado com células/comandos/
predicados/pins. O verificador ainda deve aceitá-lo. Nenhum retry automático,
nenhuma nova causa fora da lista e nenhuma terceira tentativa agregada.
Se persistir ausência de ganho ou surgir insuficiência fora do recorte,
parar e diagnosticar; não ajustar iterativamente até verde.

## Estado dos gates

Lote 1 consumido/reprovado; lote 2 disponível somente sob o gate acima.
Matrizes completas C permanecem 0/2. O script preparatório
`p1339-verifier-full-c-r1.py` foi escrito mas não executado, e seu recibo
start não existe. Seus pins antigos deverão ser sucedidos antes de qualquer
futuro uso, depois do congelamento do lote 2. Nenhum futuro F, canonical
RED, candidato ou selo foi executado/autorizado. As definições F aceitas
permanecem sem crédito de runtime e suas obrigações não mudam.
