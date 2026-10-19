from pathlib import Path
import hashlib,json,datetime
r=Path.cwd();p=r/'00_nucleo/diagnosticos/bibliography-margins-2026-10-18'
n=r/'00_nucleo/prompts/_nuclei/layout/block-margin-priority.toml'
n.write_text('''tekt = 1
kind = "nucleus"
id = "block-margin-priority"
title = "Prioridade causal de margens relativas entre blocos"

[[claims]]
id = "presence-before-magnitude"
level = "must"
statement = "Entre duas margens relativas vizinhas do protocolo de blocos, uma explícita domina uma automática, mesmo com valor zero ou menor. Entre margens da mesma classe, conservar o máximo. A classe vem da presença causal de above/below/spacing, nunca de comparação de valores."

[[claims]]
id = "heading-strength"
level = "must"
statement = "As margens próprias de Heading são explícitas para o protocolo existente. A margem automática anterior não aumenta seu above, e seu below não é aumentado pelo above automático do bloco seguinte."

[[claims]]
id = "scope-limit"
level = "must-not"
statement = "Este contrato não certifica espaçamento fracionário, vspace forte, sticky de títulos, hanging-indent nem métricas de runs mistos. Não alterar as fórmulas de origem/fim causal, top-edge, regiões ou resolução de Length por causa desta prioridade."
''')
h=hashlib.sha256(n.read_bytes()).hexdigest()
evidence='''
## Prioridade causal das margens — 2026-10-18

### Medição anterior à decisão

00_nucleo/diagnosticos/bibliography-margins-2026-10-18/before.json.gz e
before-sources.json.gz preservam HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c,
working tree não commitado, diffstat/status/UTC/fontes/argv/binários. Os 78 pares
foram medidos contra vanilla pinado bd5ebec285505889e8ff1d4f68804ab147a699f3
(binário 9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9).
Com par.spacing 2em, Block below0 seguido de Block above auto posiciona BODY
em y37.064 no Tekt contra13.064; below3 resulta37.064 contra16.063996.
Heading→Block auto produz BODY40.004 contra25.004. Block auto→Heading produz
TITLE35.7128 contra33.312804. Na bibliografia2em, título35.7128 contra33.31281
e primeiro corpo71.9 contra54.5. Os controles explícito/explícito e auto/auto
coincidem, assim como auto→explícito. O desvio em duas direções refuta um offset
especial de bibliografia e confirma prioridade antes da magnitude.

Fontes locais: block.rs:170–176 só trata a entrada forte depois de margem
fraca; heading.rs:62 usa max incondicional. No vanilla pinado,
typst-layout/src/flow/collect.rs:253–260 atribui fraquezas diferentes a Auto
versus margem relativa explícita; distribute.rs:244–259 compara prioridade
antes da magnitude. typst-library/src/model/heading.rs:305–312 configura
above/below explicitamente. Posições são linguagem; copiar enums/algoritmo
Rust não é requisito e não se infere intenção da fonte. Refutação: regressão
em zero explícito, ordens inversas, margens iguais, wrappers, par.spacing,
continuação, fracionários ou terminal; medir e não compensar com constantes.

### Classificação

Correção interna de paridade em fluxo contínuo ADR-0127: sem campo, trait,
assinatura pública, default/flag novo ou transferência de fase. A regra de
colapso anterior por max fica supersedida somente na presença de prioridades
diferentes do protocolo existente. Não amplia o significado privado de
prev_margin_is_parbreak nem encerra hanging, vspace/sticky ou métricas mistas.
'''
for name in ['block','heading']:
 f=r/f'00_nucleo/prompts/compiler/layout/{name}.md';s=f.read_text();s=s.replace('Núcleos Tekt:\n','Núcleos Tekt:\n- 00_nucleo/prompts/_nuclei/layout/block-margin-priority.toml sha256:'+h+'\n')
 spec=('''
### Responsabilidade exclusiva

Oferecer helper descendente puro collapse_margin(previous, previous_is_auto,
incoming, incoming_is_auto). Aplicar o núcleo: mesma classe usa máximo;
classes diferentes escolhem a explícita. No início de Block com chain ativa,
resolver a margem acima existente e chamar esse helper com o flag anterior
e com a ausência causal de above e spacing. Manter os ramos had_line e
origem inicial, medidas/reserva fracionária, fim causal, restauro e publicação
de below/flag existentes. Heading chama esse mesmo helper para sua entrada.
''') if name=='block' else ('''
### Responsabilidade exclusiva

Ao entrar após fronteira de bloco ativa, colapsar above próprio como explícito
com a margem anterior através do helper privado do owner
00_nucleo/prompts/compiler/layout/block.md. Manter cálculo de prev_bottom,
escala/bold, top-edge, numeração, scope lexical, flush e below explícito
existentes. Margem anterior automática cede, mas margem explícita maior
continua a prevalecer. Nenhuma alteração para título no topo da região.
''')
 tests=('''
### Aceitação

Testes RED→GREEN no próprio consumer: margem explícita anterior inclusive zero
vence above auto maior; above explícito vence below auto; classes iguais
conservam max; spacing explícito participa da precedência. Usar corpo real,
marcadores e comparação causal de posições, não só helper aritmético.
''') if name=='block' else ('''
### Aceitação

Testes RED→GREEN no próprio consumer: título após Block automático não muda
quando par.spacing fraco supera seu above; below explícito maior ainda move
título pelo delta legítimo. Cobrir níveis1/2/3, título inicial e invariância
da restauração de estilo. Revalidar título→Block no corpus bilateral.
''')
 s+=evidence+spec+tests+'''\nRepetir os 78 pares, controles nested/regionais, bibliografia sem/com título,
cores e corpus anterior; build, suíte workspace/doctests, Clippy estrito,
Rustdoc/fmt, V1/V5/V15/V26/reversa e diferencial Tekt. Limite12GiB/High8GiB,
swap0/jobs1/debug0/incremental0. Não apagar notas nem certificar todo o objetivo.
''';f.write_text(s)
(p/'prescription.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'owners':['compiler/layout/block.md','compiler/layout/heading.md'],'nucleus':str(n.relative_to(r)),'nucleus_sha256':h,'class':'ADR0127 continuous internal parity; no public API or phase change'},indent=2)+'\n')
