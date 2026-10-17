from pathlib import Path
import json,hashlib,re
out=Path('00_nucleo/diagnosticos/block-auto-alignment-2026-10-14'); owners=json.loads((out/'owners.json').read_text())
nucleus=Path('00_nucleo/prompts/_nuclei/layout/block-auto-alignment.toml')
nucleus.write_text('''tekt = 1
kind = "nucleus"
id = "block-auto-alignment"
title = "Alinhamento geométrico na largura final de Block Auto"

[[claims]]
id = "constraint-versus-allocation"
level = "must"
statement = "A largura disponível limita o layout do corpo, mas não é a alocação horizontal final de Block automático. Align geométrico em Block Auto conserva posição intrínseca e adia seu deslocamento horizontal até conhecer a largura causal final com inset. Não executar o corpo duas vezes."

[[claims]]
id = "scope"
level = "must"
statement = "O estado privado de largura Auto pertence ao Block corrente. Subframe isolado não o herda; Block filho substitui/restaura em LIFO; checkpoint conserva e configuração de página limpa. Fixups locais são resolvidos uma vez antes do recorte e da raiz de pintura na mesma região."

[[claims]]
id = "limits"
level = "must"
statement = "Esta etapa não certifica alinhamento por linha textual, matemática centrada, Place horizontal, decorações externas consumidas antes do fechamento nem Block fragmentado. A correção geométrica não permite declarar esses refutadores resolvidos; eles precisam de transporte causal e validação próprios."
''')
digest=hashlib.sha256(nucleus.read_bytes()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest()
roles={
'mod':'Armazenar block_auto_width:bool privado, inicial false; configuração nova limpa. Não modificar API pública.',
'checkpoint':'Capturar e restaurar block_auto_width junto do estado de fluxo.',
'sub_frame':'No caminho isolado, salvar/limpar block_auto_width antes do corpo e restaurar no retorno. Não mudar a semântica do caminho inline.',
'placement':'Para Align geométrico (fora da rota textual/grid flow_aligned), tratar largura de Block Auto como ainda não resolvida: posicionar inicialmente usando content_w e registrar fixup X existente. Preservar transferência de subframe, metadados e largura causal. Alinhamentos de linhas textuais/Grid mantêm a rota anterior nesta etapa.',
'block':'Instalar block_auto_width segundo resolved_width e restaurar após o corpo. Na mesma região, resolver fixups X produzidos pelo corpo contra o limite interno final, antes de clip e paint_root. Deslocar somente os paths registrados por helpers existentes; não alterar largura causal nem executar corpo novamente. A fragmentação mantém pendência explícita.',
 'tests':'Adicionar refutadores de posição do filho em Auto com clip, left/center/right, irmã maior, nested/fixed Block, Link e isolamento de estado; não trocar dimensão esperada para acomodar o defeito. Manter controles de texto e decoração como medição separada.'}
for src,prompt in owners.items():
 p=Path(prompt);s=p.read_text(); entry=f'- {nucleus} sha256:{digest}'
 a=s.index('Núcleos Tekt:\n')+len('Núcleos Tekt:\n');b=s.index('\n\n',a)
 s=s[:a]+'\n'.join(sorted(s[a:b].splitlines()+[entry]))+s[b:]
 name=Path(src).stem
 s+='''\n\n## Alinhamento geométrico em Block Auto — 2026-10-14

### Medição anterior à decisão

`00_nucleo/diagnosticos/block-auto-alignment-2026-10-14/before/results.json.gz`
conserva HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, árvore não commitada,
UTC/diffstat/status/fontes e SHA dos binários. right/center/link com clip
perdem todo o retângulo vermelho no Tekt (zero pixels), contra 3840 no pin
9f2b6e8715237cb086899a42873660fe744622e8, bbox [48,40,128,88] em 2px/pt.
Com irmã de 70pt, right precisa bbox [108,40,188,88]. placement.rs:241 usa
avail_w finito como dimensão final; block.rs:287–303 só conhece a largura
causal depois do corpo. Fonte pinada flow/block.rs:278–287 distingue
constraint de expansão. Posição e clipping são observáveis de linguagem;
filas/flags são mecânica interna. Refutadores: inset, irmãos maiores,
Link, Block fixo/aninhado e escopo após subframe/rollback.

### Responsabilidade exclusiva

'''+roles[name]+'''

### Classificação e limites

Correção interna/de paridade ADR-0127 em fluxo contínuo, sem contrato público,
modo/default ou fase novos. L0 primeiro, RED→GREEN, resselo restrito e suíte.
Esta etapa completa o refutador geométrico da largura causal; não completa
alinhamento textual/matemático, decoração externa diferida, Place horizontal,
altura Auto ou fragmentação. São pendências de B2b/T3, sem crédito de paridade.
'''
 p.write_text(s)
print(digest)
