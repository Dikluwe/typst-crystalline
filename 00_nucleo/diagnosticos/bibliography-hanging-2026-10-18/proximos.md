# Pendências após o lote de recuo francês

O objetivo integral continua ativo. Este relatório não reabre nem substitui as aprovações anteriores (inclusive R2-S), nem publica código.

1. **Métricas mistas**: 31 casos da matriz atual e dez controles de margens mantêm ~0.156pt nas caixas verticais de runs com estilos mistos. As posições horizontais, palavras, linhas, páginas e dimensões passam na prova específica dos31; a paridade completa continua aberta.
2. **Plain + optimized em CSL custom**: sete casos atuais ainda põem `A` na primeira linha do corpo onde o vanilla o envia à seguinte. O corpo relativo à sua origem é igual ao anterior; a alteração da citação com set par3em é o efeito genérico correto e foi separada por prova. Não impor simple para ocultar esta divergência.
3. **Containers**: dez casos de Box/Par em Box, Raw, List e Quote continuam com resíduos de tamanho/recorte, espaçamento terminal ou quebra. As fixtures h0/h10 e o baseline estão preservados para distinguir o defeito do container do recuo. Os testes puros aprovados não fecham o caminho CLI recortado.
4. **Alias de Content e realização textual R3-T1**: um caso `bib-content-alias` permanece; não antecipar a mudança de realização ainda pendente de aprovação. Ver a proposta textual-realization-2026-10-18 e overlay da etapa bibliography-indent.
5. **Checkpoint com prefixo ainda em current_line**: investigar elegibilidade no caminho produtivo de Flush/float. A tentativa direta inicial perdeu DDDD e conservou FFFF após flush/restore; debug.log preserva a evidência. O contrato por comprimentos antecede o lote. O teste aprovado cobre fronteira já drenada, não resolve essa suspeita.
6. **Tekt**: os105 V16 do pin do CI continuam abertos; notas V19/V20 permanecem visíveis. Os nove achados adicionais do executável global não pinado seguem no relatório da etapa de margens. Não alterar o pin ou suprimir achados para fechar o gate.
7. **Demais frentes de paridade**: retomar o inventário de paridade-novo-vanilla-2026-10-17 e os relatórios antecedentes (RM1/RG/FG/Paint/Symbol e casos E01–E20); não inferir fechamento deles deste lote geométrico.

8. **Geometria completa de decorações**: o início horizontal LTR com recuo e a ausência de traço fantasma foram corrigidos no refinamento final. Y/espessura/cauda já divergem em h0 e continuam abertos; RTL completo dessa família requer medição própria. Não extrapolar a prova de x para paridade de todas as decorações.
